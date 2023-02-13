## 설치
rustup을 사용해 설치

- command line tool for managing Rust versions and associated tools

```bash
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
//이게 안되면
--proto '=https' --tlsv1.3 제거하고 해보기
```

- Linker도 필요하기 때문에 C compiler를 설치해야 함
    - 어떤 Rust 패키지는 C 코드에 의존하고 그렇기 때문에 C compiler가 필요함
- rust update하려면

```bash
rustup update
```

- rust 지우려면

```bash
rustup self uninstall
```

## compile and run rust code
    rustc main.rs // compile rust code
    ./main // execute rust program

## meaning of !
! means that you're calling a macro instead of a normal function and macros don't always follow the same rules as functions.
ex. println!   
# cargo
Rust's build system and package manager

## creating project using cargo
    cargo new hello_cargo

Cargo.toml - TOML format(Tom's Obvious, Minimal Language)
- [package]
- [dependencies]
    - in Rust, packages of code are referred to as crates

## Building and Running a Cargo Project
    cargo build // creates an executable file in target/debug/hello_cargo

Cargo.lock - keeps track of the exact versions of dependencies in your project

    cargo run // compile the code and then run the resultant executable all in one command

    cargo check // quickly checks your code to make sure it compiles but doesn't produce an executable


    // compile with optimizations
    // create an executable in target/release
    cargo build --release 

