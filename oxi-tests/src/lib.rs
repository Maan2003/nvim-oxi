mod api;
#[cfg(test)]
mod test_all;

use std::{panic, process};

use nvim_oxi as nvim;

#[nvim::module]
fn liboxi_tests() -> nvim::Result<()> {
    let result = panic::catch_unwind(|| {
        api::autocmd::clear_autocmds_buffer_n_patterns();
        api::autocmd::clear_autocmds_current_buf();
        api::autocmd::clear_autocmds_events();
        api::autocmd::create_augroup();
        api::autocmd::create_autocmd();
        api::autocmd::create_autocmd_buffer_n_patterns();
        api::autocmd::exec_autocmds();
        api::autocmd::get_autocmds();
        api::autocmd::set_del_augroup_by_id();
        api::autocmd::set_del_augroup_by_name();
        api::autocmd::set_exec_del_autocmd();

        api::buffer::attach();
        api::buffer::call();
        api::buffer::create_del_user_command();
        api::buffer::get_changedtick();
        api::buffer::loaded_n_valid();
        api::buffer::new_buf_delete();
        api::buffer::set_get_del_keymap();
        api::buffer::set_get_del_lines();
        api::buffer::set_get_del_mark();
        api::buffer::set_get_del_text();
        api::buffer::set_get_del_var();
        api::buffer::set_get_name();
        api::buffer::set_get_option();

        api::extmark::add_highlight();
        api::extmark::clear_namespace();
        api::extmark::get_extmarks();
        api::extmark::get_namespaces();
        api::extmark::set_decoration_provider();
        api::extmark::set_get_del_extmark();

        api::global::chan_send_fail();
        api::global::create_del_user_command();
        api::global::eval_statusline();
        api::global::get_chan_info();
        api::global::get_colors();
        api::global::get_context();
        api::global::get_highlights();
        api::global::get_mode();
        api::global::get_options();
        api::global::set_get_del_current_line();
        api::global::set_get_del_keymap();
        api::global::set_get_del_mark();
        api::global::set_get_del_var();
        api::global::set_get_option();
        api::global::strwidth();

        api::tabpage::get_list_wins();
        api::tabpage::get_number();
        api::tabpage::is_valid();
        api::tabpage::set_get_del_var();

        // api::vimscript::call_function();
        // api::vimscript::cmd_basic();
        api::vimscript::cmd_no_output();
        api::vimscript::command();
        api::vimscript::eval();
        api::vimscript::exec();
        api::vimscript::parse_cmd_basic();
        api::vimscript::parse_expression_basic();

        api::win_config::open_win_basic_config();
        api::win_config::open_win_empty_config();
        api::win_config::open_win_full_config();
        api::win_config::set_config();

        api::window::call();
        api::window::close_hide();
        api::window::get_number();
        api::window::get_position();
        api::window::get_set_buf();
        api::window::get_set_height_width(); // TODO: this should fail
        api::window::get_tabpage();
        api::window::set_get_cursor();
        api::window::set_get_del_var();
    });

    process::exit(match result {
        Ok(_) => 0,

        Err(err) => {
            eprintln!("{err:?}");
            1
        },
    })
}
