use derive_builder::Builder;
use nvim_types::{self as nvim, NonOwning, Object};
use serde::Serialize;

use crate::api::{Buffer, Window};
use crate::object;

/// Options passed to `crate::api::create_user_command`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct OptionValueOpts {
    #[builder(setter(custom))]
    scope: Object,

    #[builder(setter(into, strip_option))]
    window: Option<Window>,

    #[builder(setter(into, strip_option))]
    buffer: Option<Buffer>,
}

impl OptionValueOpts {
    #[inline(always)]
    pub fn builder() -> OptionValueOptsBuilder {
        OptionValueOptsBuilder::default()
    }
}

impl OptionValueOptsBuilder {
    pub fn scope(&mut self, scope: OptionScope) -> &mut Self {
        self.scope = Some(nvim::String::from(scope).into());
        self
    }

    pub fn build(&mut self) -> OptionValueOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionScope {
    Global,
    Local,
}

impl From<OptionScope> for nvim::String {
    fn from(ctx: OptionScope) -> Self {
        ctx.serialize(object::Serializer)
            .expect("`OptionScope` is serializable")
            .try_into()
            .expect("`OptionScope` is serialized into a string")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) struct KeyDict_option<'a> {
    buf: Object,
    win: Object,
    scope: NonOwning<'a, Object>,
}

impl<'a> From<&'a OptionValueOpts> for KeyDict_option<'a> {
    fn from(opts: &'a OptionValueOpts) -> Self {
        Self {
            buf: opts.buffer.into(),
            win: opts.window.into(),
            scope: opts.scope.non_owning(),
        }
    }
}
