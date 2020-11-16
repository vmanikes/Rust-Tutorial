# Cargo

- Cargo is Rust's build and package manager
-  Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. 


## Check version
`cargo --version`

## Creating project with cargo
- `cargo new hello_cargo`
- It will generate 3 files `.gitignore`, `cargo.toml` and `src/main.rs`
- It also initializes git repository for us

## Conventions
- Cargo expects the files to live in `src` directory
- All the files above the src directories are supposed to be the file unrelated to code, such as license, readme etc

## Building a cargo project
- `cargo build`
- This command creates an executable file in `target/debug/hello_cargo`
- Run using `cargo run`
- Running the build also causes to create a new file at top level called `Cargo.lock`. This keeps track of exact version
of dependencies of the project

## Building for Release
- `cargo build --release` to compile it with optimizations. This command will create an executable in target/release 
instead of target/debug.