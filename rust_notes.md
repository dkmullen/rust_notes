## Rust notes

- Rust files end in `.rs`
- Four spaces, not tab
- Compile with `rustc`, ie `rustc myfile.rs`
- Run with `./myfile`
- Every Rust program contains a function called `main`(the entry point into the program)
- A command like `println!("hello world")` calls a macro (println); If it called a function, there would be no '!'

### Cargo

- Rust's build system and package manager, similar to npm
- Installed along with Rust - `cargo --version`
- Start a new project with `cargo new hello_cargo`
  ..- Creates a new directory with `/src` and `/target` subdirectories
  ..- Initializes a git repo
  ..- Creates configuration file `Cargo.toml` (Tom's obvious minimal language)
  ..- Creates a hello, world program in `/src/main.rs`

Other commands:

- `cargo build` which creates a debug (or dev) version of the compiled program in `target/debug. Run with `./target/debug/hello_cargo`
- Or use `cargo run`, which builds and runs in one step
- `cargo check` - Checks code for compilability w/o actually doing a build
- `cargo build --release` which builds an optimized version of the program in `/target/release`
