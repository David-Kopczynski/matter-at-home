# 🏠️ Matter-at-Home
This is a collection of my private smart home devices using [rs-matter-embassy](https://github.com/sysgrok/rs-matter-embassy) based on [rs-matter](https://github.com/project-chip/rs-matter).

```bash
.
├── .cargo/
│   └── config.toml                # Cargo configuration for building ESP targets
├── .vscode/
│   ├── extensions.json            # VSCode recommended extensions for development
│   └── settings.json              # VSCode workspace settings
├── examples/
│   ├── empty.rs                   # Minimal example project
│   ├── thread.rs                  # Matter Ble+Thread example
│   └── wifi.rs                    # Matter Ble+WiFi example
├── src/
│   ├── bin/
│   │   ├── light/                 # Light device implementation
│   │   │   └── main.rs
│   │   ├── sensor-pack/           # Sensor-Pack device implementation
│   │   │   └── main.rs
│   │   └── thread-border-router/  # Thread-Border-Router implementation
│   │       └── main.rs
│   └── lib.rs                     # Common modules for examples and binaries
├── target/
├── .clippy.toml                   # Clippy configuration
├── .gitignore
├── Cargo.lock
├── Cargo.toml                     # Project dependencies and metadata
├── LICENSE.md
├── NOTICE
├── README.md
├── rust-toolchain.toml            # Rust toolchain configuration
└── shell.nix                      # Nix shell configuration for development environment
```

## 🛠️ Building
Setting up the development environment is as easy as running:

```bash
nix-shell
code .
```

*Hint: You may want to edit `.vscode/settings.json` -> `rust-analyzer.cargo.features` to include the required features for the current application.*
*Otherwise, the rust-analyzer will not work correctly.*

## 🤓 Development
To tinker with examples during development, you can use the following commands.

```bash
# Check build
cargo clippy --example empty
cargo clippy --example thread --features thread
cargo clippy --example wifi   --features wifi

# Build, flash and monitor example
cargo run --example empty                    --release -- --monitor --port /dev/ttyACM0
cargo run --example thread --features thread --release -- --monitor --port /dev/ttyACM0
cargo run --example wifi   --features wifi   --release -- --monitor --port /dev/ttyACM0
```

## 🏗️ Projects
At the current time, the following projects can be found.

### 💡 Light
TODO

```bash
# Check build
cargo clippy --bin light --features release

# Build, flash and monitor application
cargo run --bin light --features release --release -- --monitor --port /dev/ttyACM0
```

### 📊 Sensor Pack
TODO

```bash
# Check build
cargo clippy --bin sensor-pack --features release

# Build, flash and monitor application
cargo run --bin sensor-pack --features release --release -- --monitor --port /dev/ttyACM0
```

### ⚡️ Thread Border Router
TODO

```bash
# Check build
cargo clippy --bin thread-border-router --features release

# Build, flash and monitor application
cargo run --bin thread-border-router --features release --release -- --monitor --port /dev/ttyACM0
```
