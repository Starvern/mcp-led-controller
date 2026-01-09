# mcp-led-controller

A system where an AI model can interface with an MCP server to control an LED on an Arduino microcontroller.

## /axum-socketio/
Contains the websocket server implementing Axum & Socketioxide. On Windows, run the `run-server.bat` file to startup.

## /mcp-ws-client/
Contains the MCP server. The server provides the agent with two tools: high and low. Each of these tools will emit a websocket event to the server. I recommend using `Claude Desktop`. The target `.exe` file can be found in `/mcp-ws-client/target/release/mcp-ws-client.exe` after building. This process is made easy on Windows by using `build-mcp-server.bat`.

## /arduino/websocket-mcp
The `.ino` file contains the Arduino code for a `SocketIO` client on `esp8266` hardware. This client will receive the websocket events `high` and `low` and will use `digitalWrtie` to toggle the LED as needed.

## /python
Contains a simple Python script to test the websocket server. This will send the same events as `mcp-ws-client`. On Windows, use `test-client.bat` for easy access.
