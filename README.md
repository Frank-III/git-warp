# Git-Warp v0.1.0

**High-performance, UX-focused Git worktree manager combining Copy-on-Write speed with advanced features**

Git-Warp combines the instantaneous Copy-on-Write worktree creation of `coworktree` with the rich user experience, terminal integration, and advanced features of `autowt`. Built in Rust for maximum performance and reliability.

## 🚀 Vision

Git-Warp aims to be the fastest and most user-friendly Git worktree manager, providing:

- **⚡ Instant worktree creation** using Copy-on-Write (CoW) on supported filesystems
- **🖥️ Rich terminal integration** with automatic tab/window switching
- **🤖 AI agent monitoring** with live dashboards and Claude Code hooks
- **🧹 Intelligent cleanup** with process management and safety checks
- **⚙️ Extensive configuration** with both interactive and file-based options

## 📋 Current Status

**Phase 1: Foundation ✅ COMPLETED**
- ✅ Project structure and dependencies configured
- ✅ Complete CLI interface with clap (all commands defined)
- ✅ Module architecture established
- ✅ Error handling and logging framework
- ✅ Cross-platform terminal abstraction
- ✅ TUI framework integration (ratatui)

**Phase 2: Core Implementation ✅ COMPLETED**
- ✅ Copy-on-Write engine for macOS (APFS)
- ✅ Git operations using gix and git CLI
- ✅ Path rewriting for environment compatibility
- ✅ Worktree listing and management
- ✅ Intelligent cleanup with branch analysis
- ✅ Terminal integration (macOS)
- ⏳ Process management and cleanup
- ⏳ Configuration system with figment

## 🛠️ Current Features

### Available Commands

```bash
# Show help
warp --help

# List worktrees ✅ WORKING
warp ls
warp ls --debug

# Create/switch to worktree ✅ WORKING
warp feature-branch
warp switch feature-branch
warp switch --path /custom/path feature-branch

# Cleanup worktrees ✅ WORKING
warp cleanup --mode merged
warp cleanup --mode all --force
warp cleanup --mode remoteless

# Test with dry-run ✅ WORKING
warp --dry-run switch test-feature
warp --dry-run cleanup --mode merged

# Configuration (stub)
warp config --show
warp config --edit

# Agent monitoring (stub)
warp agents

# Claude Code hooks (stub)
warp hooks-install --user
warp hooks-status

# Shell integration (stub)
warp shell-config bash
```

**✅ Core functionality now working:** `ls`, `switch`, and `cleanup` commands are fully functional with Copy-on-Write support!

## 🏗️ Architecture

### Module Structure

```
src/
├── main.rs           # Application entry point
├── cli.rs           # Complete CLI interface ✅
├── config.rs        # Configuration management
├── cow.rs           # Copy-on-Write abstraction
├── error.rs         # Error types and handling ✅  
├── git.rs           # Git operations via gix
├── hooks.rs         # Claude Code integration
├── process.rs       # Process management
├── rewrite.rs       # Path rewriting engine
├── terminal.rs      # Terminal integration ✅
└── tui.rs           # Interactive dashboards ✅
```

### Technology Stack

| Component | Technology | Status |
|-----------|------------|---------|
| **CLI** | clap 4.5 | ✅ Complete |
| **Git** | gix + git CLI | ✅ Complete |
| **CoW** | cp + APFS | ✅ Complete |
| **Terminal** | AppleScript (macOS) | ✅ Complete |
| **TUI** | ratatui + crossterm | ✅ Framework Ready |
| **Config** | figment + toml | ⏳ Planned |
| **Async** | rayon (parallel processing) | ✅ Used in path rewriting |

## 🎯 Roadmap

### v0.1.0 - Foundation & Core Engine
- [x] **Project Setup**: Complete Rust project with all dependencies
- [x] **CLI Structure**: Full command interface with help system  
- [x] **Architecture**: Module structure and error handling
- [x] **CoW Engine**: macOS APFS copy-on-write implementation
- [x] **Git Integration**: Full worktree operations via git CLI
- [x] **Path Rewriting**: Intelligent path rewriting with parallel processing
- [x] **Core Commands**: `ls`, `switch`, and `cleanup` fully functional

### v0.2.0 - Advanced Features
- [ ] **Process Management**: Find and terminate processes in worktrees
- [ ] **Configuration**: Layered config system (file + env + CLI)
- [ ] **Terminal Integration**: macOS terminal automation
- [ ] **Agent Hooks**: Claude Code integration for AI monitoring

### v0.3.0 - Interactive Experience  
- [ ] **Live Dashboards**: Agent monitoring TUI
- [ ] **Interactive Cleanup**: Worktree selection interface
- [ ] **Config Editor**: Interactive settings management
- [ ] **Enhanced UX**: Progress bars, better error messages

### v0.4.0 - Platform Expansion
- [ ] **Linux Support**: overlayfs-based CoW implementation
- [ ] **Windows Support**: Basic worktree fallback
- [ ] **CI/CD**: Automated builds and releases
- [ ] **Documentation**: Comprehensive guides and API docs

## 🏃‍♂️ Getting Started

### Prerequisites

- **Rust**: 2024 edition (latest stable)
- **Git**: Modern git installation
- **macOS**: For Copy-on-Write support (APFS filesystem)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/git-warp
cd git-warp

# Build the project
cargo build --release

# The binary will be available at target/release/warp
```

### Usage

```bash
# Show all available commands
./target/release/warp --help

# Test current functionality
./target/release/warp ls
./target/release/warp switch feature-branch
./target/release/warp cleanup --mode merged
./target/release/warp --dry-run switch test-feature
```

## 🤝 Contributing

This project is in early development. Contributions are welcome!

1. **Issues**: Report bugs or suggest features
2. **Pull Requests**: Implement planned features or fix bugs
3. **Testing**: Try the tool and provide feedback
4. **Documentation**: Improve guides and examples

### Development

```bash
# Run with debug logging
RUST_LOG=debug cargo run -- ls --debug

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## 🚀 What's Working Now

### Core Worktree Management
- **Fast Listing**: `warp ls` shows all worktrees with branch info
- **Intelligent Switching**: `warp switch branch-name` creates/switches to worktrees
- **Copy-on-Write**: Instant worktree creation on APFS (macOS)
- **Smart Cleanup**: Analyze and remove merged/remoteless branches
- **Safe Operations**: Dry-run mode for all destructive operations
- **Terminal Integration**: Automatic tab/window switching on macOS

### Advanced Features
- **Branch Analysis**: Detects merged, remoteless, and uncommitted branches
- **Path Rewriting**: Fixes absolute paths in CoW copies
- **Process Safety**: Prevents cleanup of active worktrees
- **Multiple Modes**: Traditional Git fallback when CoW unavailable
- **Rich Output**: Emoji-enhanced CLI with clear status messages

## 🎨 Design Philosophy

### Performance First
- **Zero-copy operations** where possible
- **Parallel processing** for file operations  
- **Minimal overhead** in common workflows
- **Fast startup** times

### User Experience
- **Intuitive commands** that just work
- **Rich feedback** with progress indicators
- **Smart defaults** with full customization
- **Excellent error messages** with helpful suggestions

### Safety & Reliability  
- **Dry-run mode** for all destructive operations
- **Process detection** before cleanup
- **Git safety checks** to prevent data loss
- **Comprehensive testing** across platforms

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

This project combines concepts from:
- **[autowt](https://github.com/example/autowt)**: Advanced UX and agent integration
- **[coworktree](https://github.com/example/coworktree)**: High-performance CoW implementation

Built with ❤️ in Rust 🦀