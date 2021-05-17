# Rust TCP Server

### Running a Server with ngrok (OSX)
 - Download the binary
 - `chmod 755 {FILENAME}`
 - `./{FILENAME}` (Give permission in System Preferences -> Security & Privacy -> General
 - In a new terminal run: `./ngrok tcp 6000`


### Creating a Release
- `git tag -a vX.X.X -m "{description}"`
- `git push origin vX.X.X`

### Inspiration & Notes
- Rust TCP Chat: https://github.com/tensor-programming/Rust_client-server_chat
