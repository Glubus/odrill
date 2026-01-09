//! Known globals configuration
//!
//! Provides configurable lists of known global variables.

use std::collections::HashSet;

/// Lua built-in globals
pub const LUA_BUILTINS: &[&str] = &[
    "_G",
    "_VERSION",
    "assert",
    "collectgarbage",
    "dofile",
    "error",
    "getmetatable",
    "ipairs",
    "load",
    "loadfile",
    "next",
    "pairs",
    "pcall",
    "print",
    "rawequal",
    "rawget",
    "rawlen",
    "rawset",
    "require",
    "select",
    "setmetatable",
    "tonumber",
    "tostring",
    "type",
    "unpack",
    "xpcall",
];

/// Lua standard library globals
pub const LUA_STDLIB: &[&str] = &[
    "coroutine",
    "debug",
    "io",
    "math",
    "os",
    "package",
    "string",
    "table",
    "utf8",
];

/// Lua keywords/literals
pub const LUA_KEYWORDS: &[&str] = &["true", "false", "nil", "self"];

/// Payday 2 / BLT specific globals
pub const PAYDAY2_GLOBALS: &[&str] = &[
    "BLT",
    "BLTMod",
    "Hooks",
    "MenuHelper",
    "DelayedCalls",
    "QuickMenu",
    "managers",
    "tweak_data",
    "Global",
    "Application",
    "World",
    "TimerManager",
    "Idstring",
    "Vector3",
    "Rotation",
    "Color",
    "callback",
    "alive",
    "CoreClass",
    "CoreUnit",
    "deep_clone",
    "clone",
    // Added by request/discovery
    "EXPOK",
    "MenuCallbackHandler",
    "PlayerManager",
    "ExperienceManager",
    "ModPath",
    "SavePath",
];

/// Container for known global variables
#[derive(Debug, Clone, Default)]
pub struct KnownGlobals {
    globals: HashSet<String>,
}

impl KnownGlobals {
    /// Create with default Lua + Payday 2 globals
    pub fn with_defaults() -> Self {
        let mut globals = HashSet::new();

        for &g in LUA_BUILTINS {
            globals.insert(g.to_string());
        }
        for &g in LUA_STDLIB {
            globals.insert(g.to_string());
        }
        for &g in LUA_KEYWORDS {
            globals.insert(g.to_string());
        }
        for &g in PAYDAY2_GLOBALS {
            globals.insert(g.to_string());
        }

        Self { globals }
    }

    /// Create empty (no globals)
    pub fn empty() -> Self {
        Self {
            globals: HashSet::new(),
        }
    }

    /// Create with only Lua builtins
    pub fn lua_only() -> Self {
        let mut globals = HashSet::new();
        for &g in LUA_BUILTINS {
            globals.insert(g.to_string());
        }
        for &g in LUA_STDLIB {
            globals.insert(g.to_string());
        }
        for &g in LUA_KEYWORDS {
            globals.insert(g.to_string());
        }
        Self { globals }
    }

    /// Add a single global
    pub fn add(&mut self, name: impl Into<String>) {
        self.globals.insert(name.into());
    }

    /// Add multiple globals
    pub fn extend(&mut self, names: impl IntoIterator<Item = String>) {
        self.globals.extend(names);
    }

    /// Check if a variable is a known global
    pub fn contains(&self, name: &str) -> bool {
        self.globals.contains(name)
    }

    /// Get reference to inner set
    pub fn inner(&self) -> &HashSet<String> {
        &self.globals
    }
}
