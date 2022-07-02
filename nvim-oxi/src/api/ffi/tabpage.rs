use nvim_types::{
    Array,
    Error,
    Integer,
    NonOwning,
    Object,
    String,
    TabHandle,
    WinHandle,
};

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/tabpage.c#L85
    pub(crate) fn nvim_tabpage_del_var(
        tabpage: TabHandle,
        name: NonOwning<String>,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/tabpage.c#L129
    pub(crate) fn nvim_tabpage_get_number(
        tabpage: TabHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/tabpage.c#L50
    pub(crate) fn nvim_tabpage_get_var(
        tabpage: TabHandle,
        name: NonOwning<String>,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/tabpage.c#L102
    pub(crate) fn nvim_tabpage_get_win(
        tabpage: TabHandle,
        err: *mut Error,
    ) -> WinHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/tabpage.c#L145
    pub(crate) fn nvim_tabpage_is_valid(tabpage: TabHandle) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/tabpage.c#L20
    pub(crate) fn nvim_tabpage_list_wins(
        tabpage: TabHandle,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/tabpage.c#L68
    pub(crate) fn nvim_tabpage_set_var(
        tabpage: TabHandle,
        name: NonOwning<String>,
        value: NonOwning<Object>,
        err: *mut Error,
    );
}
