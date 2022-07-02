use std::mem::ManuallyDrop;
use std::{fmt, ptr};

use super::{Collection, Object};

// https://github.com/neovim/neovim/blob/master/src/nvim/api/private/defs.h#L95
pub type Array = Collection<Object>;

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter().map(ToString::to_string)).finish()
    }
}

impl IntoIterator for Array {
    type IntoIter = ArrayIterator;
    type Item = <ArrayIterator as Iterator>::Item;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        // Wrap `self` in `ManuallyDrop` to avoid running destructor.
        let arr = ManuallyDrop::new(self);
        let start = arr.items;
        let end = unsafe { start.add(arr.len()) };

        ArrayIterator { start, end }
    }
}

impl<T> FromIterator<T> for Array
where
    T: Into<Object>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter()
            .map(Into::into)
            .filter(Object::is_some)
            .collect::<Vec<Object>>()
            .into()
    }
}

pub struct ArrayIterator {
    start: *const Object,
    end: *const Object,
}

impl Iterator for ArrayIterator {
    type Item = Object;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        (self.start != self.end).then(|| {
            let current = self.start;
            self.start = unsafe { self.start.offset(1) };
            unsafe { ptr::read(current) }
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.len();
        (exact, Some(exact))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl ExactSizeIterator for ArrayIterator {
    fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.start) as usize }
    }
}

impl Drop for ArrayIterator {
    fn drop(&mut self) {
        while self.start != self.end {
            unsafe {
                ptr::drop_in_place(self.start as *mut Object);
                self.start = self.start.offset(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Array, Object};

    #[test]
    fn iter_basic() {
        let array = Array::from_iter(["Foo", "Bar", "Baz"]);

        let mut iter = array.into_iter();
        assert_eq!(Some(Object::from("Foo")), iter.next());
        assert_eq!(Some(Object::from("Bar")), iter.next());
        assert_eq!(Some(Object::from("Baz")), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn drop_iter_halfway() {
        let array = Array::from_iter(["Foo", "Bar", "Baz"]);

        let mut iter = array.into_iter();
        assert_eq!(Some(Object::from("Foo")), iter.next());
    }
}

macro_rules! impl_from_tuple {
    ($($ty:ident),*) => {
        impl <$($ty: Into<Object>),*> From<($($ty,)*)> for Array {
            #[allow(non_snake_case)]
            fn from(($($ty,)*): ($($ty,)*)) -> Self {
                Self::from_iter([$($ty.into(),)*])
            }
        }
    };
}

impl_from_tuple!(A);
impl_from_tuple!(A, B);
impl_from_tuple!(A, B, C);
impl_from_tuple!(A, B, C, D);
impl_from_tuple!(A, B, C, D, E);
impl_from_tuple!(A, B, C, D, E, F);
impl_from_tuple!(A, B, C, D, E, F, G);
impl_from_tuple!(A, B, C, D, E, F, G, H);
impl_from_tuple!(A, B, C, D, E, F, G, H, I);
impl_from_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_from_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_from_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_from_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_from_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_from_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_from_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
