use serde::Deserialize;

use super::{CommandAddr, CommandNArgs, CommandRange};
use crate::lua::LuaFn;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct CommandInfos {
    pub addr: Option<CommandAddr>,
    pub bang: bool,
    pub bar: bool,
    pub callback: Option<LuaFn<(), ()>>,
    pub complete: Option<String>,
    pub complete_arg: Option<String>,
    pub count: Option<String>,
    pub definition: Option<String>,
    pub keepscript: bool,
    pub name: String,
    pub nargs: Option<CommandNArgs>,
    pub range: Option<CommandRange>,
    pub register: bool,
    pub script_id: u32,
}
