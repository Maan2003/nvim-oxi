use derive_builder::Builder;
use nvim_types::Dictionary;

/// Options passed to `Buffer::delete`.
#[derive(Clone, Debug, Default, Builder)]
#[builder(default, build_fn(private, name = "fallible_build"))]
pub struct BufDeleteOpts {
    #[builder(setter(strip_option))]
    force: Option<bool>,

    #[builder(setter(strip_option))]
    unload: Option<bool>,
}

impl BufDeleteOpts {
    #[inline(always)]
    pub fn builder() -> BufDeleteOptsBuilder {
        BufDeleteOptsBuilder::default()
    }
}

impl BufDeleteOptsBuilder {
    pub fn build(&mut self) -> BufDeleteOpts {
        self.fallible_build().expect("never fails, all fields have defaults")
    }
}

impl From<&BufDeleteOpts> for Dictionary {
    fn from(opts: &BufDeleteOpts) -> Self {
        Self::from_iter([("force", opts.force), ("unload", opts.unload)])
    }
}
