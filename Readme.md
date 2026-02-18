### Doc: https://rust-lang.org/learn/get-started/
- Rust installation in Ubuntu
```bash
sudo snap install rustup --classic
```
- Set default rust toolchain
```bash
rustup default stable
```
- Generating a rust project
```bash
cargo new project_name
```

- Run code
```bash
cargo build && cargo run
```

```bash
cargo install cargo-edit
```

- Add dependency
```bash
caro add dependencyName
```

- Add WebAssembly support In Rust compiler .WASM standard library download.No program install.

```bash
rustup target add wasm32-unknown-unknown
```
- wasm32       → 32-bit WebAssembly
- unknown      → No Operating system 
- unknown      → ABI unspecified

- If want to run rust code in browser

```bash
cargo build --target wasm32-unknown-unknown
```

- Set up trunk

```bash
cargo install trunk && ls $HOME/.cargo/bin | grep trunk
```

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

- Confirmation of trunk

```bash
which trunk
trunk --version
```

- Check before build webassembly.

```bash
cargo check --target wasm32-unknown-unknown
```