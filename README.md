## Getting started with Rust
- Youtube [tutorial series](https://www.youtube.com/watch?v=2hY7Uib2UDM)

### Setup with Rustup
- [rustup](https://www.rust-lang.org/tools/install) is an installer for the systems programming language Rust
- Don't forget to add Rust cargo Source in your Path. Installer should give you the command to add at the very end of installation 

### Cargo commands
Cargo is the Rust [package manager](https://doc.rust-lang.org/cargo/appendix/glossary.html#package-manager). Cargo downloads your Rust package's dependencies, compiles your packages, makes distributable packages, and uploads them to [crates.io](https://crates.io/), the Rust community’s package registry.

**Cargo.lock file should be included in the version control for binary project and should be ignored for library project**

```
cargo new hello-rust
cargo run

Some common cargo commands are (see all commands with --list):
    build, b    Compile the current package
    check, c    Analyze the current package and report errors, but don't build object files
    clean       Remove the target directory
    doc, d      Build this package's and its dependencies' documentation
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory
    run, r      Run a binary or example of the local package
    test, t     Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this package to the registry
    install     Install a Rust binary. Default location is $HOME/.cargo/bin
    uninstall   Uninstall a Rust binary

```

### Plugins to use in VS-Code for Rust
- [rust-analyzer](https://rust-analyzer.github.io/manual.html)
- codeLLDB (For debugging purpose)

### Difference between Rust executable and library project
- Should have main.rs to create a executable project
- Should have lib.rs to create a library project
- Create tests folder to add all the test cases 
- Create examples folder to add examples codes 

### Install bacon using: 
- `cargo install bacon`
- Run by running: `bacon`
- Hot reload check while the code changes get saved

### Install cargo add using: 
- `cargo install cargo-edit`
- cargo add <dependency_name> <dependency_name>

### Create a new lib package:
- `cargo new --lib quotesapi`