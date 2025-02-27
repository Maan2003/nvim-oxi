use nvim_types::{
    array::Array,
    dictionary::Dictionary,
    error::Error,
    object::Object,
    string::String,
    BufHandle,
    Integer,
    LuaRef,
};

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L145
    pub(super) fn nvim_buf_attach(
        channel_id: u64,
        buf: BufHandle,
        send_buffer: bool,
        opts: Dictionary,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1358
    pub(super) fn nvim_buf_call(
        buf: BufHandle,
        fun: LuaRef,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1383
    pub(super) fn nvim_buf_create_user_command(
        buf: BufHandle,
        name: String,
        command: Object,
        opts: *const Dictionary,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L969
    pub(super) fn nvim_buf_del_keymap(
        channel_id: u64,
        buf: BufHandle,
        mode: String,
        lhs: String,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1216
    pub(super) fn nvim_buf_del_mark(
        buf: BufHandle,
        name: String,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1406
    pub(super) fn nvim_buf_del_user_command(
        buf: BufHandle,
        name: String,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1030
    pub(super) fn nvim_buf_del_var(
        buf: BufHandle,
        name: String,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1150
    pub(super) fn nvim_buf_delete(
        buf: BufHandle,
        opts: Dictionary,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L921
    pub(super) fn nvim_buf_get_changedtick(
        buf: BufHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L983
    pub(super) fn nvim_buf_get_commands(
        buf: BufHandle,
        opts: *mut Dictionary,
        err: *mut Error,
    ) -> Dictionary;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L940
    pub(super) fn nvim_buf_get_keymap(
        channel_id: u64,
        buf: BufHandle,
        mode: String,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L269
    pub(super) fn nvim_buf_get_lines(
        channel_id: u64,
        buf: BufHandle,
        start: Integer,
        end: Integer,
        strict_indexing: bool,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1299
    pub(super) fn nvim_buf_get_mark(
        buf: BufHandle,
        name: String,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1086
    pub(super) fn nvim_buf_get_name(buf: BufHandle, err: *mut Error)
        -> String;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L876
    pub(super) fn nvim_buf_get_offset(
        buf: BufHandle,
        index: Integer,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1049
    pub(super) fn nvim_buf_get_option(
        buf: BufHandle,
        name: String,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L778
    pub(super) fn nvim_buf_get_text(
        channel_id: u64,
        buf: BufHandle,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        opts: Dictionary,
        err: *mut Error,
    ) -> Array;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1049
    pub(super) fn nvim_buf_get_var(
        buf: BufHandle,
        name: String,
        err: *mut Error,
    ) -> Object;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1135
    pub(super) fn nvim_buf_is_loaded(buf: BufHandle) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1198
    pub(super) fn nvim_buf_is_valid(buf: BufHandle) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1049
    pub(super) fn nvim_buf_line_count(
        buf: BufHandle,
        err: *mut Error,
    ) -> Integer;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L957
    pub(super) fn nvim_buf_set_keymap(
        channel_id: u64,
        buf: BufHandle,
        mode: String,
        lhs: String,
        rhs: String,
        opts: *const Dictionary,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L365
    pub(super) fn nvim_buf_set_lines(
        channel_id: u64,
        buf: BufHandle,
        start: Integer,
        end: Integer,
        strict_indexing: bool,
        replacement: Array,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1265
    pub(super) fn nvim_buf_set_mark(
        buf: BufHandle,
        name: String,
        line: Integer,
        col: Integer,
        opts: Dictionary,
        err: *mut Error,
    ) -> bool;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1104
    pub(super) fn nvim_buf_set_name(
        buf: BufHandle,
        name: String,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1069
    pub(super) fn nvim_buf_set_option(
        channel_id: u64,
        buf: BufHandle,
        name: String,
        value: Object,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L538
    pub(super) fn nvim_buf_set_text(
        channel_id: u64,
        buf: BufHandle,
        start_row: Integer,
        start_col: Integer,
        end_row: Integer,
        end_col: Integer,
        replacement: Array,
        err: *mut Error,
    );

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/buffer.c#L1013
    pub(super) fn nvim_buf_set_var(
        buf: BufHandle,
        name: String,
        value: Object,
        err: *mut Error,
    );
}
