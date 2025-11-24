## Background
Example repo to test basic functionalities of [ofi-librabric-sys](github.com/kyle-yh-kim/ofi-libfabric-sys-example) crate.

## How to build & run
```
// To build the binary.
PKG_CONFIG_PATH={parent_directory_to_libfabric.pc} cargo build

// To invoke the main() function.
export LD_LIBRARY_PATH={path_to_libfabric.so}
PKG_CONFIG_PATH={parent_directory_to_libfabric.pc} cargo run
```
