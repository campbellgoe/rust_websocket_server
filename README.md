# Step-by-Step Instructions
Clone both https://github.com/campbellgoe/rust_websocket_client and this repository.

1. Running the WebSocket Server
First, ensure that you are in the directory of the server project (rust_websocket_server). If you are not, navigate to it using the command line:

```bash
cd path/to/rust_websocket_server
```
Then, run the server using Cargo:

```bash
cargo run
```
This will compile and start the WebSocket server. You should see output indicating that the server has started:

```bash
WebSocket server started on ws://127.0.0.1:8080
```
2. Running the WebSocket Client
Open a new terminal window or tab. Navigate to the client project directory (rust_websocket_client):

```bash
cd path/to/rust_websocket_client
```
Then, run the client using Cargo:

```bash
cargo run
```
This will compile and start the WebSocket client. You should see output indicating that the client has connected to the server:

```bash
WebSocket client connected
```
If everything is set up correctly, you should see messages being sent and received between the server and the client.

Testing Communication
To test communication between the server and the client, you can modify the client to send a message upon connection and the server to echo it back, as already set up in your code. When the client connects, it should send "Hello, Server!" and the server should echo it back, printing:

```bash
WebSocket client connected
Received message from server: Hello, Server!
```
### Additional Notes
Ensure Ports Are Not Blocked: Make sure that port 8080 (or whichever port you are using) is not blocked by any firewall or used by another application.

Check Dependencies: If you add or update dependencies, ensure you run cargo build to compile the project with the updated dependencies.

Running on Different Machines: If you want to run the server and client on different machines, ensure they are connected to the same network and use the server's IP address instead of `127.0.0.1`.

Debugging: If you encounter any issues, check the terminal output for error messages, which can provide hints on what might be wrong.

By following these steps, you should be able to run and test your WebSocket server and client written in Rust. If you have any further questions or run into issues, feel free to ask!