# Wrap up

- Create Project: `$ cargo new hello_cargo`
  + binary project: `$ cargo new hello_cargo --bin` [default]
  + library project: `$ cargo new hello_cargo --lib`
  + no version control: `--vcs` flag
- Build (target/debug): `$ cargo build`
  + ... and Execute: `$ ./target/debug/hello_cargo`
  + Release (target/release): `$ cargo build --release`
- Run: `$ cargo run`
- Check: `$ cargo check` no producing an executable
