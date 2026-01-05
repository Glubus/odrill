# Odrill

Modern modding toolkit for **Payday 2** BLT/SuperBLT mods.

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

## Features

- **Package Management** - Install and publish Lua libraries
- **Selective Imports** - Rust-style `use module::function` syntax
- **Smart Bundling** - Only include what you use
- **Dev Launcher** - One command to test in-game
- **Code Formatting** - Stylua integration

## Quick Start

```bash
# Create new mod
odrill init my-mod
cd my-mod

# Build
odrill build

# Test in game
odrill run
```

## Installation

### From Source
```bash
git clone https://github.com/Glubus/odrill.git
cd odrill
cargo install --path apps/odrill
```

## Commands

| Command | Description |
|---------|-------------|
| `odrill init <name>` | Create new project |
| `odrill build` | Bundle Lua files |
| `odrill run` | Dev launcher |
| `odrill add <pkg>` | Add dependency |
| `odrill install` | Install dependencies |
| `odrill publish` | Publish to registry |
| `odrill fmt` | Format Lua code |
| `odrill login` | Authenticate |

## License

AGPL-3.0 - See [LICENSE](LICENSE) for details.
