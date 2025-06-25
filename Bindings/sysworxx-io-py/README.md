# Python bindings for sysworxx-io

## Build and use locally (x86_64)

```sh
cargo build --release -p sysworxx-io-py &&
  cp -v target/release/libsysworxx_io_py.so target/release/sysworxx_io_py.so &&
  python -ic "import sys; sys.path.append('$(pwd)/target/release'); import sysworxx_io_py as io"
```

## Cross compile for different target

```sh
# PC
cross --verbose build --release --target aarch64-unknown-linux-gnu --workspace &&
  scp target/aarch64-unknown-linux-gnu/release/libsysworxx_io_py.so root@TARGETIP:/tmp/sysworxx_io_py.so
# DUT
cd /tmp && python -ic "import sysworxx_io_py as io"
```

## Update stubs

```sh
cargo run --bin py-stub-gen && mv sysworxx-io-py.pyi sysworxx_io_py.pyi
```
