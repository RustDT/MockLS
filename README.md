# MockLS
A mock language server using the LSP protocol, the purpose of which, is to help test LSP clients. 

**WORK IN PROGRESS**

MockLS is compiled into an executable that communicates with LSP client via stdin/stdout or via TCP sockets. 
Command line:
 * `mock_ls`: communicate via stdin/stdout
 * `mock_ls <port number>`: listen on localhost, on given port number 
