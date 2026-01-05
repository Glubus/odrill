mod compiler_result;
mod generate;
mod ops;
mod processor;
mod symbols;
mod verify;

pub use compiler_result::CompilerResult;
pub use symbols::SymbolTable;

use crate::parser::LuaParser;
use pkg::OdrillProject;

pub struct Compiler {
    pub(crate) project: OdrillProject,
    pub(crate) parser: LuaParser,
    pub(crate) symbols: SymbolTable,
}

impl Compiler {
    pub fn new(project: OdrillProject) -> Self {
        // Use manifest options or default?
        // let include = project.manifest.options.include_directive...;
        let parser = LuaParser::new("--"); // Default Lua comment

        Self {
            project,
            parser,
            symbols: SymbolTable::new(),
        }
    }
}
