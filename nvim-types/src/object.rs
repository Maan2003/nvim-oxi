use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::string::String as StdString;

use crate::{
    Array,
    Boolean,
    Dictionary,
    Float,
    Integer,
    LuaRef,
    NonOwning,
    String as NvimString,
};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L115
#[repr(C)]
pub struct Object {
    pub r#type: ObjectType,
    pub data: ObjectData,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L100
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum ObjectType {
    kObjectTypeNil = 0,
    kObjectTypeBoolean,
    kObjectTypeInteger,
    kObjectTypeFloat,
    kObjectTypeString,
    kObjectTypeArray,
    kObjectTypeDictionary,
    kObjectTypeLuaRef,
}

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L117
#[repr(C)]
pub union ObjectData {
    pub boolean: Boolean,
    pub integer: Integer,
    pub float: Float,
    pub string: ManuallyDrop<NvimString>,
    pub array: ManuallyDrop<Array>,
    pub dictionary: ManuallyDrop<Dictionary>,
    pub luaref: LuaRef,
}

impl Object {
    /// Returns a new object with type `kObjectTypeNil`.
    #[inline]
    pub const fn nil() -> Self {
        Self {
            r#type: ObjectType::kObjectTypeNil,
            data: ObjectData { integer: 0 },
        }
    }

    #[inline]
    pub const fn is_nil(&self) -> bool {
        matches!(self.r#type, ObjectType::kObjectTypeNil)
    }

    #[inline]
    pub const fn is_some(&self) -> bool {
        !self.is_nil()
    }

    /// Make a non-owning version of this `Object`.
    #[inline]
    pub fn non_owning(&self) -> NonOwning<'_, Self> {
        // Using ptr::read, because can't copy the union.
        unsafe { NonOwning::new(std::ptr::read(self)) }
    }

    /// Extracts the inner `String` from the object, without checking that the
    /// object actually represents a `String`.
    #[inline]
    pub unsafe fn into_string_unchecked(self) -> NvimString {
        let str = ManuallyDrop::new(self);
        NvimString { ..*str.data.string }
    }

    /// Extracts the inner `Array` from the object, without checking that the
    /// object actually represents an `Array`.
    #[inline]
    pub unsafe fn into_array_unchecked(self) -> Array {
        let array = ManuallyDrop::new(self);
        Array { ..*array.data.array }
    }

    /// Extracts the inner `Dictionary` from the object, without checking that
    /// the object actually represents a `Dictionary`.
    #[inline]
    pub unsafe fn into_dict_unchecked(self) -> Dictionary {
        let dict = ManuallyDrop::new(self);
        Dictionary { ..*dict.data.dictionary }
    }
}

impl Default for Object {
    #[inline]
    fn default() -> Self {
        Self::nil()
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data: &dyn fmt::Debug = match self.r#type {
            kObjectTypeNil => &"nil",
            kObjectTypeBoolean => unsafe { &self.data.boolean },
            kObjectTypeInteger => unsafe { &self.data.integer },
            kObjectTypeFloat => unsafe { &self.data.float },
            kObjectTypeString => unsafe { &self.data.string },
            kObjectTypeArray => unsafe { &self.data.array },
            kObjectTypeDictionary => unsafe { &self.data.dictionary },
            kObjectTypeLuaRef => unsafe { &self.data.luaref },
        };

        f.debug_struct("Object")
            .field("type", &self.r#type)
            .field("data", data)
            .finish()
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner: &dyn fmt::Display = match self.r#type {
            kObjectTypeNil => &"()",
            kObjectTypeBoolean => unsafe { &self.data.boolean },
            kObjectTypeInteger => unsafe { &self.data.integer },
            kObjectTypeFloat => unsafe { &self.data.float },
            kObjectTypeString => unsafe { self.data.string.deref() },
            kObjectTypeArray => unsafe { self.data.array.deref() },
            kObjectTypeDictionary => unsafe { self.data.dictionary.deref() },
            kObjectTypeLuaRef => unsafe { &self.data.luaref },
        };

        f.debug_tuple("Object").field(&inner.to_string()).finish()
    }
}

macro_rules! clone_copy {
    ($self:expr, $field:ident) => {{
        Self {
            r#type: $self.r#type,
            data: ObjectData { $field: unsafe { $self.data.$field } },
        }
    }};
}

macro_rules! clone_drop {
    ($self:expr, $field:ident, $as_type:ident) => {{
        Self {
            r#type: $self.r#type,
            data: ObjectData {
                $field: ManuallyDrop::new(
                    unsafe { &$self.data.$field as &$as_type }.clone(),
                ),
            },
        }
    }};
}

impl Clone for Object {
    fn clone(&self) -> Self {
        use ObjectType::*;
        match self.r#type {
            kObjectTypeNil => Self::nil(),
            kObjectTypeBoolean => clone_copy!(self, boolean),
            kObjectTypeInteger => clone_copy!(self, integer),
            kObjectTypeFloat => clone_copy!(self, float),
            kObjectTypeString => clone_drop!(self, string, NvimString),
            kObjectTypeArray => clone_drop!(self, array, Array),
            kObjectTypeDictionary => clone_drop!(self, dictionary, Dictionary),
            kObjectTypeLuaRef => clone_copy!(self, luaref),
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        use ObjectType::*;
        match self.r#type {
            kObjectTypeString => unsafe {
                ManuallyDrop::drop(&mut self.data.string)
            },

            kObjectTypeArray => unsafe {
                ManuallyDrop::drop(&mut self.data.array)
            },

            kObjectTypeDictionary => unsafe {
                ManuallyDrop::drop(&mut self.data.dictionary)
            },

            _ => {},
        }
    }
}

impl PartialEq<Self> for Object {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.r#type != other.r#type {
            return false;
        };

        let (lhs, rhs) = (&self.data, &other.data);

        unsafe {
            use ObjectType::*;
            match self.r#type {
                kObjectTypeNil => true,
                kObjectTypeBoolean => lhs.boolean == rhs.boolean,
                kObjectTypeInteger => lhs.boolean == rhs.boolean,
                kObjectTypeFloat => lhs.float == rhs.float,
                kObjectTypeString => lhs.string == rhs.string,
                kObjectTypeArray => lhs.array == rhs.array,
                kObjectTypeDictionary => lhs.dictionary == rhs.dictionary,
                kObjectTypeLuaRef => lhs.luaref == rhs.luaref,
            }
        }
    }
}

impl From<()> for Object {
    fn from(_: ()) -> Self {
        Self::nil()
    }
}

// Implements `From<..>` for primitive `Copy` types.
macro_rules! from_copy {
    ($type:ident, $variant:ident, $data:ident) => {
        impl From<$type> for Object {
            #[inline(always)]
            fn from($data: $type) -> Self {
                Object {
                    r#type: ObjectType::$variant,
                    data: ObjectData { $data },
                }
            }
        }
    };
}

from_copy!(Boolean, kObjectTypeBoolean, boolean);
from_copy!(Integer, kObjectTypeInteger, integer);
from_copy!(Float, kObjectTypeFloat, float);

/// Implements `From<..>` for primitive `ManuallyDrop` types.
macro_rules! from_man_drop {
    ($type:ident, $variant:ident, $data:ident) => {
        impl From<$type> for Object {
            #[inline(always)]
            fn from($data: $type) -> Self {
                Object {
                    r#type: ObjectType::$variant,
                    data: ObjectData { $data: ManuallyDrop::new($data) },
                }
            }
        }
    };
}

from_man_drop!(NvimString, kObjectTypeString, string);
from_man_drop!(Array, kObjectTypeArray, array);
from_man_drop!(Dictionary, kObjectTypeDictionary, dictionary);

/// Implements `From<..>` for integer types convertible to `Integer`.
macro_rules! from_int {
    ($type:ident) => {
        impl From<$type> for Object {
            #[inline(always)]
            fn from(i: $type) -> Self {
                Integer::from(i).into()
            }
        }
    };
}

from_int!(i8);
from_int!(u8);
from_int!(i16);
from_int!(u16);
from_int!(i32);
from_int!(u32);

impl From<f32> for Object {
    #[inline(always)]
    fn from(n: f32) -> Self {
        Float::from(n).into()
    }
}

impl From<StdString> for Object {
    #[inline(always)]
    fn from(s: StdString) -> Self {
        NvimString::from(s).into()
    }
}

impl From<&str> for Object {
    #[inline(always)]
    fn from(s: &str) -> Self {
        NvimString::from(s).into()
    }
}

impl From<char> for Object {
    #[inline(always)]
    fn from(ch: char) -> Self {
        NvimString::from(ch).into()
    }
}

impl<T> From<Option<T>> for Object
where
    Object: From<T>,
{
    #[inline(always)]
    fn from(maybe: Option<T>) -> Self {
        maybe.map(Into::into).unwrap_or_default()
    }
}

impl<T> From<Box<T>> for Object
where
    Object: From<T>,
{
    #[inline(always)]
    fn from(boxed: Box<T>) -> Self {
        (*boxed).into()
    }
}

impl<T> From<Cow<'_, T>> for Object
where
    T: Clone,
    Object: From<T>,
{
    #[inline(always)]
    fn from(moo: Cow<'_, T>) -> Self {
        moo.into_owned().into()
    }
}

impl<T> FromIterator<T> for Object
where
    T: Into<Object>,
{
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Array::from_iter(iter).into()
    }
}

impl<K, V> FromIterator<(K, V)> for Object
where
    NvimString: From<K>,
    Object: From<V>,
{
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Dictionary::from_iter(iter).into()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FromObjectError {
    /// Raised when implementing `TryFrom<Object>` for one of the "primitive"
    /// data types, i.e. a field of the `ObjectData` union.
    #[error("Was expecting a \"{expected:?}\", got \"{actual:?}\" instead")]
    Primitive { expected: ObjectType, actual: ObjectType },

    /// Raised when implementing `TryFrom<Object>` for a type that implements
    /// `TryFrom<{type}>`, where `{type}` is a primitive data type. For
    /// example, `TryFrom<StdString>` or `TryFrom<usize>`.
    #[error("Error converting {into} into {primitive:?}: {source}")]
    Secondary {
        primitive: ObjectType,
        into: &'static str,
        source: Box<dyn StdError>,
    },
}

impl PartialEq<Self> for FromObjectError {
    fn eq(&self, other: &Self) -> bool {
        use FromObjectError::*;
        match (self, other) {
            (
                Primitive { expected: e1, actual: a1 },
                Primitive { expected: e2, actual: a2 },
            ) => (e1 == e2) && (a1 == a2),

            (
                Secondary { primitive: p1, into: i1, source: _ },
                Secondary { primitive: p2, into: i2, source: _ },
            ) => (p1 == p2) && (i1 == i2),

            _ => false,
        }
    }
}

impl Eq for FromObjectError {}

impl FromObjectError {
    pub fn secondary<E, T>(primitive: ObjectType, err: E) -> Self
    where
        E: StdError + 'static,
    {
        Self::Secondary {
            primitive,
            into: std::any::type_name::<T>(),
            source: Box::new(err),
        }
    }
}

use FromObjectError::*;
use ObjectType::*;

impl TryFrom<Object> for () {
    type Error = FromObjectError;

    fn try_from(obj: Object) -> StdResult<Self, Self::Error> {
        (matches!(obj.r#type, kObjectTypeNil)).then_some(()).ok_or_else(|| {
            Primitive { expected: kObjectTypeNil, actual: obj.r#type }
        })
    }
}

/// Implements `TryFrom<Object>` for primitive `Copy` types.
macro_rules! try_from_copy {
    ($type:ident, $variant:ident, $data:ident) => {
        impl TryFrom<Object> for $type {
            type Error = FromObjectError;

            fn try_from(obj: Object) -> StdResult<Self, Self::Error> {
                (matches!(obj.r#type, $variant))
                    .then_some(unsafe { obj.data.$data })
                    .ok_or_else(|| Primitive {
                        expected: $variant,
                        actual: obj.r#type,
                    })
            }
        }
    };
}

try_from_copy!(Boolean, kObjectTypeBoolean, boolean);
try_from_copy!(Integer, kObjectTypeInteger, integer);
try_from_copy!(Float, kObjectTypeFloat, float);

/// Implements `TryFrom<Object>` for primitive `ManuallyDrop` types.
macro_rules! try_from_man_drop {
    ($type:ident, $variant:ident, $into_inner:ident) => {
        impl TryFrom<Object> for $type {
            type Error = FromObjectError;

            fn try_from(obj: Object) -> StdResult<Self, Self::Error> {
                let ty = obj.r#type;
                (matches!(ty, ObjectType::$variant))
                    .then_some(unsafe { obj.$into_inner() })
                    .ok_or_else(|| FromObjectError::Primitive {
                        expected: ObjectType::$variant,
                        actual: ty,
                    })
            }
        }
    };
}

try_from_man_drop!(NvimString, kObjectTypeString, into_string_unchecked);
try_from_man_drop!(Array, kObjectTypeArray, into_array_unchecked);
try_from_man_drop!(Dictionary, kObjectTypeDictionary, into_dict_unchecked);

/// Implements `TryFrom<Object>` for a type that implements `TryFrom<{prim}>`,
/// where `{prim}` is one of the primitive data types.
macro_rules! try_from_prim {
    ($orig:ident, $type:ty, $variant:ident) => {
        impl TryFrom<Object> for $type {
            type Error = FromObjectError;

            fn try_from(obj: Object) -> StdResult<Self, Self::Error> {
                $orig::try_from(obj)?.try_into().map_err(|err| {
                    FromObjectError::secondary::<_, $type>($variant, err)
                })
            }
        }
    };
}

try_from_prim!(Integer, i8, kObjectTypeInteger);
try_from_prim!(Integer, u8, kObjectTypeInteger);
try_from_prim!(Integer, i16, kObjectTypeInteger);
try_from_prim!(Integer, u16, kObjectTypeInteger);
try_from_prim!(Integer, i32, kObjectTypeInteger);
try_from_prim!(Integer, u32, kObjectTypeInteger);
try_from_prim!(Integer, u64, kObjectTypeInteger);
try_from_prim!(Integer, i128, kObjectTypeInteger);
try_from_prim!(Integer, u128, kObjectTypeInteger);
try_from_prim!(Integer, isize, kObjectTypeInteger);
try_from_prim!(Integer, usize, kObjectTypeInteger);

try_from_prim!(NvimString, StdString, kObjectTypeString);

#[cfg(test)]
mod tests {
    use super::{Object, StdString};

    #[test]
    fn std_string_to_obj_and_back() {
        let str = StdString::from("foo");
        let obj = Object::from(str.clone());
        let str_again = StdString::try_from(obj);
        assert!(str_again.is_ok());
        assert_eq!(str, str_again.unwrap());
    }
}
