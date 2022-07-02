use nvim_types::{Array, Dictionary, Error, String as NvimString};

use super::ffi::vimscript::*;
use super::opts::*;
use super::types::*;
use crate::lua::LUA_INTERNAL_CALL;
use crate::object::{FromObject, ToObject};
use crate::Result;

/// Binding to `nvim_call_dict_function`.
///
/// Calls a VimL dictionary function with the given arguments, returning the
/// result of the funtion call.
pub fn call_dict_function<D, A, R>(dict: D, func: &str, args: A) -> Result<R>
where
    D: ToObject,
    A: Into<Array>,
    R: FromObject,
{
    let dict = dict.to_obj()?;
    let func = NvimString::from(func);
    let args = args.into();
    let mut err = Error::new();
    let res = unsafe {
        nvim_call_dict_function(
            dict.non_owning(),
            func.non_owning(),
            args.non_owning(),
            &mut err,
        )
    };
    err.into_err_or_flatten(|| R::from_obj(res))
}

/// Binding to `nvim_call_function`.
///
/// Calls a VimL function with the given arguments, returning the result of the
/// funtion call.
pub fn call_function<A, R>(func: &str, args: A) -> Result<R>
where
    A: Into<Array>,
    R: FromObject,
{
    let func = NvimString::from(func);
    let args = args.into();
    let mut err = Error::new();
    let res = unsafe {
        nvim_call_function(func.non_owning(), args.non_owning(), &mut err)
    };
    err.into_err_or_flatten(|| R::from_obj(res))
}

/// Binding to `nvim_cmd`.
///
/// Executes an Ex command. Unlike `crare::api::command` it takes a structured
/// `CmdInfos` object instead of a string.
pub fn cmd(infos: &CmdInfos, opts: &CmdOpts) -> Result<Option<String>> {
    let mut err = Error::new();
    let output = unsafe {
        nvim_cmd(LUA_INTERNAL_CALL, &infos.into(), &opts.into(), &mut err)
    };
    err.into_err_or_flatten(|| {
        output
            .into_string()
            .map_err(From::from)
            .map(|output| (!output.is_empty()).then_some(output))
    })
}

/// Binding to `nvim_command`.
///
/// Executes an Ex command.
pub fn command(command: &str) -> Result<()> {
    let command = NvimString::from(command);
    let mut err = Error::new();
    unsafe { nvim_command(command.non_owning(), &mut err) };
    err.into_err_or_else(|| ())
}

/// Binding to `nvim_eval`.
///
/// Evaluates a VimL expression.
pub fn eval<V>(expr: &str) -> Result<V>
where
    V: FromObject,
{
    let expr = NvimString::from(expr);
    let mut err = Error::new();
    let output = unsafe { nvim_eval(expr.non_owning(), &mut err) };
    err.into_err_or_flatten(|| V::from_obj(output))
}

/// Binding to `nvim_exec`.
///
/// Executes a multiline block of Ex commands. If `output` is true the
/// output is captured and returned.
pub fn exec(src: &str, output: bool) -> Result<Option<String>> {
    let src = NvimString::from(src);
    let mut err = Error::new();
    let output = unsafe {
        nvim_exec(LUA_INTERNAL_CALL, src.non_owning(), output.into(), &mut err)
    };
    err.into_err_or_flatten(|| {
        output
            .into_string()
            .map_err(From::from)
            .map(|output| (!output.is_empty()).then_some(output))
    })
}

/// Binding to `nvim_parse_cmd`.
///
/// Parses the command line.
pub fn parse_cmd(src: &str, opts: &ParseCmdOpts) -> Result<CmdInfos> {
    let src = NvimString::from(src);
    let opts = Dictionary::from(opts);
    let mut err = Error::new();
    let dict = unsafe {
        nvim_parse_cmd(src.non_owning(), opts.non_owning(), &mut err)
    };
    err.into_err_or_flatten(|| CmdInfos::from_obj(dict.into()))
}

/// Binding to `nvim_parse_expression`.
///
/// Parses a VimL expression.
pub fn parse_expression(
    expr: &str,
    flags: &str,
    include_highlight: bool,
) -> Result<ParsedVimLExpression> {
    let expr = NvimString::from(expr);
    let flags = NvimString::from(flags);
    let mut err = Error::new();
    let dict = unsafe {
        nvim_parse_expression(
            expr.non_owning(),
            flags.non_owning(),
            include_highlight,
            &mut err,
        )
    };
    // crate::print!("{dict:?}");
    err.into_err_or_flatten(|| ParsedVimLExpression::from_obj(dict.into()))
}
