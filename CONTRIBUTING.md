# Contributing Guide

Terima kasih sudah tertarik untuk berkontribusi! 🎉

## Setup Development

1. **Install Rust**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone & Build**

   ```bash
   git clone <your-repo>
   cd rust-websocket-server
   cargo build
   ```

3. **Run Development Server**

   ```bash
   RUST_LOG=debug cargo run
   ```

## Project Structure

```
rust-websocket-server/
├── src/
│   ├── main.rs       # Entry point & connection handling
│   ├── types.rs      # Data structures & enums
│   ├── room.rs       # Room management logic
│   └── handler.rs    # Message handling
├── examples/
│   └── card_game.rs  # Example game implementation
├── client.html       # Test client
└── Cargo.toml        # Dependencies
```

## Development Workflow

### 1. Testing Locally

```bash
# Terminal 1: Run server
cargo run

# Terminal 2: Test dengan wscat
wscat -c ws://localhost:8080

# Or open client.html in browser
```

### 2. Adding New Features

**Example: Menambah Game Baru**

1. Buat enum di `types.rs`:

```rust
pub enum RoomType {
    Chat,
    CardGame,
    YourNewGame,  // Add here
}
```

1. Update handler di `handler.rs`:

```rust
async fn handle_game_action(&self, action: &str, data: Value) {
    match action {
        "your_action" => {
            // Your logic here
        }
        _ => {}
    }
}
```

1. Test dengan client

### 3. Code Style

- Run formatter: `cargo fmt`
- Run linter: `cargo clippy`
- Run tests: `cargo test`

## Common Tasks

### Adding Dependencies

```bash
cargo add <package-name>
```

### Running with Logs

```bash
# Info level
RUST_LOG=info cargo run

# Debug level
RUST_LOG=debug cargo run

# Specific module
RUST_LOG=rust_websocket_server::handler=debug cargo run
```

### Testing WebSocket

**Create Room:**

```json
{ "type": "create_room", "room_type": "card_game", "max_players": 4 }
```

**Join Room:**

```json
{ "type": "join_room", "room_id": "<room-id>" }
```

**Send Message:**

```json
{ "type": "send_message", "content": "Hello!" }
```

## Pull Request Process

1. Fork repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Make changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Commit: `git commit -m 'Add amazing feature'`
7. Push: `git push origin feature/amazing-feature`
8. Create Pull Request

## Code Review Checklist

- [ ] Code compiles without warnings
- [ ] Tests pass
- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings
- [ ] Documentation updated (if needed)
- [ ] Examples updated (if needed)

## Need Help?

- Open an issue
- Check existing issues
- Read the README.md

## Tips

1. **Debugging**: Use `dbg!()` macro

   ```rust
   dbg!(&room_manager);
   ```

2. **Performance**: Use `cargo build --release` for production builds

3. **Testing**: Write tests in same file

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_something() {
           // Your test
       }
   }
   ```

Happy coding! 🚀
