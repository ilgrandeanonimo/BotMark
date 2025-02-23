<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Current version)](https://img.shields.io/badge/current_version-1.21.4-blue)

A tool to stress test Minecraft servers by simulating numerous bot connections, helping identify performance bottlenecks and stability issues.
![Screenshot_20250223_154536](https://github.com/user-attachments/assets/b1379060-e375-44dc-95a3-91f1f2fb52dd)
</div>

### How to Run
1. **Disable Online Mode:** Ensure that `online-mode=false` in your server's `server.properties` file.  This is essential for Botmark to connect.

2. **Run Botmark:** Use the following command in your terminal:
`./botmark --ip <ip:port> --count <bot-count>`
**Example**
`./botmark --ip 0.0.0.0:25565 --count 50`
