# kintsugi-stack-rust
> kintsugi-stack-rust

## 1. Getting Started

## 1.1. Installation

```bash
sudo apt update
sudo apt install -y curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Rust Lang: Rust Install https://doc.rust-lang.org/book/ch01-01-installation.html
  - Windows
    - Install Linux, Just kidding !!
    - https://visualstudio.microsoft.com/downloads/
      - Install VS
      - Install VSC
      - Install Build Tools for Visual Studio
        - then Restart Computer
    - https://rust-lang.org/tools/install/
      - Install Rust
    - `rustup toolchain install stable-x86_64-pc-windows-gnu`
    - `rustup default stable-x86_64-pc-windows-gnu`
  - Linux: `$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
- Rust Server Dev: Rust Analyzer Install https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

## 1.2. Hello, World!

- rust code file extension `.rs`
```rs
// 1_2_hello_world.rs
fn main(){
    println!("Hello, World! ")
}
// rustc main.rs && ./main
```
- Compile command
```bash
rustc main.rs
```
- Rust Binary Run command
```bash
./main
```

## 1.3. Cargo

```bash
cargo --version # cargo version check
&& cargo new project_name # create proj.
&& cd project_name
&& cargo build # build executable
&& cargo run # run project
&& cargo check # check for err without any executable
&& cargo help # help

# .
# ├── Cargo.lock
# ├── Cargo.toml
# └── src
#     └── main.rs

# .
# ├── Cargo.lock
# ├── Cargo.toml
# ├── src
# │   └── main.rs
# └── target
#     ├── CACHEDIR.TAG
#     └── debug
#         ├── build
#         ├── deps
#         │   ├── libone_three_hello_cargo-f9884884092cd48a.rmeta
#         │   ├── one_three_hello_cargo-5885dd703046e3fc
#         │   ├── one_three_hello_cargo-5885dd703046e3fc.d
#         │   └── one_three_hello_cargo-f9884884092cd48a.d
#         ├── examples
#         ├── incremental
#         │   ├── one_three_hello_cargo-10ah7hvrv4gzi
#         │   │   ├── s-heho9i1rut-1ysdico-bwbrfy6ptxbomf5iwqz5vt3f0
#         │   │   │   ├── dep-graph.bin
#         │   │   │   ├── query-cache.bin
#         │   │   │   └── work-products.bin
#         │   │   └── s-heho9i1rut-1ysdico.lock
#         │   └── one_three_hello_cargo-3dgwin0zbxstr
#         │       ├── s-heho98ij63-039vdoh-143bk3qfw5zxnyx5otl9s0tja
#         │       │   ├── 00ylhni9avwle6wyqpyzm6par.o
#         │       │   ├── 19k6gm7hj98zo0jv2b5mu1std.o
#         │       │   ├── 1jqdhkz0e02p777bbobcmna2j.o
#         │       │   ├── 5fgvfmdk1vtvncsc4ze5a0wi9.o
#         │       │   ├── 8z1o97dthkm4wl9qy6anckmmy.o
#         │       │   ├── 9fwica1fdmiqw5oux5l4cedjc.o
#         │       │   ├── dep-graph.bin
#         │       │   ├── query-cache.bin
#         │       │   └── work-products.bin
#         │       └── s-heho98ij63-039vdoh.lock
#         ├── one_three_hello_cargo
#         └── one_three_hello_cargo.d
```

- Cargo: 
    - Rust's Build System
    - + Package manager 
    - + Builtin When we Install Rust ( Painpoint of other prog. lang.)
- Compile command
```bash
rustc main.rs
```
- Rust Binary Run command
```bash
./main
```
- Cargo version check
```bash
cargo --version
```
- Create New Cargo Project
```bash
cargo new one_three_hello_cargo
```
- File Organisation
  - `Cargo.toml`
    - package config file
  - `.gitignore`
    - default code ver. ignore file
    - ignore flags for git ver.
  - `\src`
    - contains actual code
    - `main.rs` 
      - Starter code
```bash
.
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```
- Build command 
  - Build 
  - + Create `Cargo.lock`
    - contain dependencies
  - + Create `\target`
    - contain `\debug`
      - contain our actual executable
      - + other supporting stuff
```bash
cargo build
```
- run command
```bash
cargo run
```
- help command
  - to view all commands
```bash
cargo help
```
- check command
  - check your prog. for err.
  - without producing any executable
  - faster than running the prog.
```bash
cargo check
```

```bash
bali-king@war-machine:~/BaliGit/kintsugi-stack-rust/one_three_hello_cargo$ cargo run
   Compiling one_three_hello_cargo v0.1.0 (/home/bali-king/BaliGit/kintsugi-stack-rust/one_three_hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/one_three_hello_cargo`
Hello, world!
bali-king@war-machine:~/BaliGit/kintsugi-stack-rust/one_three_hello_cargo$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
bali-king@war-machine:~/BaliGit/kintsugi-stack-rust/one_three_hello_cargo$ cargo check
    Checking one_three_hello_cargo v0.1.0 (/home/bali-king/BaliGit/kintsugi-stack-rust/one_three_hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
bali-king@war-machine:~/BaliGit/kintsugi-stack-rust/one_three_hello_cargo$ 
```
```bash
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        ├── deps
        │   ├── libone_three_hello_cargo-f9884884092cd48a.rmeta
        │   ├── one_three_hello_cargo-5885dd703046e3fc
        │   ├── one_three_hello_cargo-5885dd703046e3fc.d
        │   └── one_three_hello_cargo-f9884884092cd48a.d
        ├── examples
        ├── incremental
        │   ├── one_three_hello_cargo-10ah7hvrv4gzi
        │   │   ├── s-heho9i1rut-1ysdico-bwbrfy6ptxbomf5iwqz5vt3f0
        │   │   │   ├── dep-graph.bin
        │   │   │   ├── query-cache.bin
        │   │   │   └── work-products.bin
        │   │   └── s-heho9i1rut-1ysdico.lock
        │   └── one_three_hello_cargo-3dgwin0zbxstr
        │       ├── s-heho98ij63-039vdoh-143bk3qfw5zxnyx5otl9s0tja
        │       │   ├── 00ylhni9avwle6wyqpyzm6par.o
        │       │   ├── 19k6gm7hj98zo0jv2b5mu1std.o
        │       │   ├── 1jqdhkz0e02p777bbobcmna2j.o
        │       │   ├── 5fgvfmdk1vtvncsc4ze5a0wi9.o
        │       │   ├── 8z1o97dthkm4wl9qy6anckmmy.o
        │       │   ├── 9fwica1fdmiqw5oux5l4cedjc.o
        │       │   ├── dep-graph.bin
        │       │   ├── query-cache.bin
        │       │   └── work-products.bin
        │       └── s-heho98ij63-039vdoh.lock
        ├── one_three_hello_cargo
        └── one_three_hello_cargo.d
```