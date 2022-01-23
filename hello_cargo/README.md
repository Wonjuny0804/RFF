# Hello Cargo

- TOML format : `(Tom's Obvious, Minimal Langauge) foramt`

The cargo's configuration format is `TOML`, so you should learn how to write one.

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2018"

[dependencies]
```

it's like the npm's `package.json` file

- The three lines that you read above are the configuartion information Cargo needs to complile the program.
- `[dependencies]` is the section where you write any kind of project's dependencies.
- In Rust packages of code are referred to as **crates**

- cargo expects all your source files to live inside the `src/` directory

### commands

```bash
$ cargo build
```

- if you run the command above you will see a message something like this
- also it will create some files that hasen't existed there for the past

  - such as
    - target/
    - Cargo.lock

- the Cargo.lock has the all dependencies and library versions locked in it so that when it can remain the version it was originally made

```bash
Compiling hello_cargo v0.1.0 (/Users/wonjunjang/codes/RustForFun/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
```

- this command creates an executable file in target/debug/hello_cargo
- you can run the executable with the command line that follows

```bash
$ ./target/debug/hello_cargo
```

```bash
$ cargo run
```

- if you run this command Cargo will run the program without building it.

```bash
$ cargo check
```

- This command checks if your code is compilable and doesn't create an Executable

### For release

- when your project is done and you want to make a release verion of it, you can use the command `cargo build` but with a `release` option

```bash
$ cargo build --release
```

- when you give a `release` option it will build a optimized verion of your program
- this command will create an executable in `target/release`

## Conclusion

- so whenever you are developing using rust, use Cargo :)
- Remember Cargo commands:
  - cargo new <Project_Name>
  - cargo build
  - cargo check
  - cargo run
  - cargo build --release
