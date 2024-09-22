# DDD forum | Rust

## Run the project

- To build and run the project in one step, use:

```bash
cargo run
```

## Running database migrations

### Make sure the SQLX cli is installed

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### Migration

```bash
sqlx migrate
```

## Rust Installation

### 1. Installing Rust

#### macOS and Linux

To install Rust, you can use the `rustup` toolchain installer, which will set up the Rust compiler and the Cargo
package manager.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For Homebrew:

```bash
brew install rustup
```

Follow the on-screen instructions to complete the installation. After installation, make sure to configure your
environment:

```bash
source $HOME/.cargo/env
```

### 2. Verifying Installation

Check the installation by confirming the versions of `rustc`, `cargo`, and `rustup`:

```bash
rustc --version
cargo --version
rustup --version
```

## Updating Rust

To update the Rust toolchain to the latest stable version, run:

```bash
rustup update
```

For Homebrew:

```bash
brew upgrade rust
```

This command will update `rustc`, `cargo`, and `rustup` to the latest versions.

## Managing Dependencies

### 1. Installing Project Dependencies

Navigate to your Rust project\'s directory and run:

```bash
cargo build
```

This will compile the project and automatically download and install all dependencies specified in your `
Cargo.toml` file.

### 2. Updating Dependencies

To update the project's dependencies to the latest compatible versions, run:

```bash
cargo update
```

This updates the `Cargo.lock` file with the latest versions of the dependencies.

### 3. Adding a New Dependency

To add a new dependency to the project, use:

```bash
cargo add <dependency_name>
```

Replace `<dependency_name>` with the name of the crate you want to add. This will automatically update your `
Cargo.toml` file.

## Rust Watch mode

Make sure to install `cargo-watch` globally by running:

```bash
cargo install cargo-watch
```

### Watch mode without alias

To run the project in watch mode without using an alias, use:

```bash
cargo watch -x run
```

### Watch mode with alias

To simplify running the project in watch mode, you can add an alias to your `~/.cargo/config.toml` file.

Add the following alias:

```toml
[alias]
dev = "watch -x run"
```

If the file does not exist, create it with:

```bash
touch ~/.cargo/config.toml
```

Once the alias is set up, you can run the project in watch mode with:

```bash
cargo dev
```

## Additional Tips

- To build and run the project in one step, use:

```bash
cargo run
```

- To clean the build artifacts and cache:

```bash
cargo clean
```

- For running tests:

```bash
cargo test
```
