use nvim_types::object::Object;
use serde::ser::{self};

use crate::Result;

pub trait ToObject {
    fn to_obj(self) -> Result<Object>;
}

impl<T> ToObject for T
where
    T: ser::Serialize,
{
    fn to_obj(self) -> Result<Object> {
        self.serialize(super::Serializer)
    }
}
