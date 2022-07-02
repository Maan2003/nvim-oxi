use std::cell::RefCell;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::result::Result as StdResult;
use std::{fmt, mem, ptr};

use libc::{c_char, c_int};
use nvim_types::{LuaRef, Object, ObjectData, ObjectType};
use serde::{de, ser};

use super::{ffi::*, LuaPoppable, LuaPushable};
use crate::object::ToObject;
use crate::Result;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct LuaFun<A, R>(pub(crate) LuaRef, PhantomData<A>, PhantomData<R>);

impl<A, R> fmt::Debug for LuaFun<A, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("LuaFun").field(&self.0).finish()
    }
}

impl<A, R> From<LuaFun<A, R>> for Object {
    fn from(fun: LuaFun<A, R>) -> Self {
        Self {
            r#type: ObjectType::kObjectTypeLuaRef,
            data: ObjectData { luaref: fun.0 },
        }
    }
}

impl<A, R> ToObject for LuaFun<A, R> {
    fn to_obj(self) -> Result<Object> {
        Ok(self.into())
    }
}

impl<'de, A, R> de::Deserialize<'de> for LuaFun<A, R> {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct LuaFunVisitor<A, R>(PhantomData<A>, PhantomData<R>);

        impl<'de, A, R> de::Visitor<'de> for LuaFunVisitor<A, R> {
            type Value = LuaFun<A, R>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an f32 representing a Lua reference")
            }

            fn visit_f32<E>(self, value: f32) -> StdResult<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(LuaFun(value as i32, PhantomData, PhantomData))
            }
        }

        deserializer.deserialize_f32(LuaFunVisitor(PhantomData, PhantomData))
    }
}

impl<A, R> ser::Serialize for LuaFun<A, R> {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_f32(self.0 as f32)
    }
}

impl<A, R> LuaFun<A, R> {
    pub fn from_fn<F>(fun: F) -> Self
    where
        A: LuaPoppable,
        R: LuaPushable,
        F: Fn(A) -> Result<R> + 'static,
    {
        type Cb = Box<dyn Fn(*mut lua_State) -> Result<c_int> + 'static>;

        unsafe extern "C" fn c_fun(lstate: *mut lua_State) -> c_int {
            let fun = {
                let idx = lua_upvalueindex(1);
                let upv = lua_touserdata(lstate, idx) as *mut Cb;
                &**upv
            };

            fun(lstate).unwrap_or_else(|err| handle_error(lstate, err))
        }

        let r#ref = super::with_state(move |lstate| unsafe {
            let fun = Box::new(move |l| fun(A::pop(l)?)?.push(l));
            let ud = lua_newuserdata(lstate, mem::size_of::<Cb>());
            ptr::write(ud as *mut Cb, fun);
            lua_pushcclosure(lstate, c_fun, 1);
            luaL_ref(lstate, LUA_REGISTRYINDEX)
        });

        Self(r#ref, PhantomData, PhantomData)
    }

    pub fn from_fn_mut<F>(fun: F) -> Self
    where
        A: LuaPoppable,
        R: LuaPushable,
        F: FnMut(A) -> Result<R> + 'static,
    {
        let fun = RefCell::new(fun);
        Self::from_fn(move |args| {
            fun.try_borrow_mut()
                .map_err(|_| crate::Error::LuaFunMutRecursiveCallback)?(
                args
            )
        })
    }

    pub fn from_fn_once<F>(fun: F) -> Self
    where
        A: LuaPoppable,
        R: LuaPushable,
        F: FnOnce(A) -> Result<R> + 'static,
    {
        let fun = RefCell::new(Some(fun));
        Self::from_fn(move |args| {
            fun.try_borrow_mut()
                .ok()
                .and_then(|mut fun| fun.take())
                .ok_or_else(|| crate::Error::LuaFunOnceMoreThanOnce)?(
                args
            )
        })
    }

    /// Calls the function, passing `args` as function arguments.
    pub fn call(&self, args: A) -> Result<R>
    where
        A: LuaPushable,
        R: LuaPoppable,
    {
        super::with_state(move |lstate| unsafe {
            lua_rawgeti(lstate, LUA_REGISTRYINDEX, self.0);
            let nargs = args.push(lstate)?;

            match lua_pcall(lstate, nargs, R::N, 0) {
                LUA_OK => R::pop(lstate),

                err_code => {
                    let msg = CStr::from_ptr(lua_tostring(lstate, -1))
                        .to_string_lossy()
                        .to_string();

                    lua_pop(lstate, 1);

                    match err_code {
                        LUA_ERRRUN => Err(crate::Error::LuaRuntimeError(msg)),

                        LUA_ERRMEM => Err(crate::Error::LuaMemoryError(msg)),

                        LUA_ERRERR => {
                            panic!("errorfunc is 0, this never happens!")
                        },

                        _ => unreachable!(),
                    }
                },
            }
        })
    }

    /// Consumes the `LuaFun`, removing the reference stored in the Lua
    /// registry.
    pub(crate) fn unref(self) {
        super::with_state(move |lstate| unsafe {
            luaL_unref(lstate, LUA_REGISTRYINDEX, self.0);
        })
    }
}

unsafe fn handle_error(lstate: *mut lua_State, err: crate::Error) -> ! {
    let msg = err.to_string();
    lua_pushlstring(lstate, msg.as_ptr() as *const c_char, msg.len());
    lua_error(lstate);
}
