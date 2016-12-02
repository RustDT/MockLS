# MockLS
A mock language server using the LSP protocol, the purpose of which, is to help test LSP clients. 

**WORK IN PROGRESS**
Only initialization and shutdown is supported, no other LSP operations yet.

## Usage:
Needs __nightly__ Rust toolchain to compile/run.

MockLS is compiled into an executable that communicates with LSP client via stdin/stdout or via TCP sockets. 
Command line:
 * `mock_ls`: communicate via stdin/stdout.
 * `mock_ls <port number>`: listen on localhost, on given port number .
 
Note: if you want to compile and run in one invocation, you can do `cargo run <arguments>`, with Cargo invoked on the root directory.
