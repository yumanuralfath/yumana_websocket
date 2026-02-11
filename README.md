# Rust WebSocket Server

WebSocket server sederhana namun solid yang dibangun dengan Rust untuk game multiplayer dan chat real-time.

## ✨ Fitur

- 🎮 **Game Support** - Dirancang untuk game kartu dan game sederhana lainnya
- 💬 **Real-time Chat** - Sistem chat yang efisien
- 🏠 **Room System** - Manajemen room dengan kapasitas dinamis
- 🔒 **Type-safe** - Menggunakan Rust's type system untuk keamanan
- ⚡ **Asynchronous** - Built dengan Tokio untuk performa tinggi
- 📦 **Lightweight** - Dependency minimal, tidak overcomplicated

## 🏗️ Arsitektur

```
src/
├── main.rs          # Entry point & WebSocket connection handling
├── types.rs         # Data structures & message types
├── room.rs          # Room management logic
└── handler.rs       # Message handling logic
```

### Komponen Utama

1. **Message Types** - Type-safe communication dengan serde
2. **Room Manager** - Thread-safe room management dengan DashMap
3. **Message Handler** - Logic untuk handle berbagai tipe message
4. **Connection Handler** - WebSocket connection lifecycle

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ ([rustup.rs](https://rustup.rs/))

### Installation

```bash
# Clone repository
git clone <repo-url>
cd rust-websocket-server

# Build project
cargo build --release

# Run server
cargo run
```

Server akan jalan di `ws://127.0.0.1:8080`

## 📡 API Reference

### Client Messages

#### Create Room

```json
{
  "type": "create_room",
  "room_type": "card_game",
  "max_players": 4
}
```

#### Join Room

```json
{
  "type": "join_room",
  "room_id": "room-uuid"
}
```

#### Leave Room

```json
{
  "type": "leave_room"
}
```

#### Send Chat Message

```json
{
  "type": "send_message",
  "content": "Hello everyone!"
}
```

#### Game Action

```json
{
  "type": "game_action",
  "action": "play_card",
  "data": {
    "card_id": 5,
    "position": "table"
  }
}
```

### Server Messages

#### Connected

```json
{
  "type": "connected",
  "client_id": "client-uuid"
}
```

#### Room Created

```json
{
  "type": "room_created",
  "room_id": "room-uuid",
  "room_type": "card_game"
}
```

#### Room Joined

```json
{
  "type": "room_joined",
  "room_id": "room-uuid",
  "room_type": "card_game",
  "players": [...]
}
```

#### Chat Message

```json
{
  "type": "chat_message",
  "from": "client-uuid",
  "content": "Hello!",
  "timestamp": 1234567890
}
```

#### Game State/Action

```json
{
  "type": "game_action",
  "player_id": "client-uuid",
  "action": "play_card",
  "data": {...}
}
```

## 🎮 Contoh Implementasi Game

Project ini generic dan bisa di-extend untuk berbagai game:

### Game Kartu (Contoh: Uno, Poker)

```rust
// Di game_state
{
  "deck": [...],
  "players": {
    "player1": {"hand": [...], "score": 0},
    "player2": {"hand": [...], "score": 0}
  },
  "current_player": "player1",
  "table_cards": [...]
}
```

### Turn-based Game

```rust
// Game action example
{
  "action": "make_move",
  "data": {
    "from": [2, 3],
    "to": [4, 5]
  }
}
```

## 🔧 Kustomisasi

### Menambah Game Logic Baru

1. Extend `GameAction` di `handler.rs`
2. Update `game_state` structure di `types.rs`
3. Implement game rules di `handle_game_action`

### Menambah Room Type Baru

```rust
// Di types.rs
pub enum RoomType {
    Chat,
    CardGame,
    YourNewGameType, // Add here
}
```

## 🧪 Testing

```bash
# Run tests
cargo test

# Run dengan logging
RUST_LOG=debug cargo run
```

## 🌐 Production Considerations

Untuk production, tambahkan:

1. **Connection Manager** - Simpan WebSocket senders untuk broadcast
2. **Authentication** - Token-based auth
3. **Persistence** - Save game state ke database
4. **Load Balancing** - Redis pub/sub untuk multi-instance
5. **Rate Limiting** - Prevent spam/abuse
6. **SSL/TLS** - Secure WebSocket (wss://)

## 📚 Dependencies

- `tokio` - Async runtime
- `tokio-tungstenite` - WebSocket implementation
- `serde` - Serialization/deserialization
- `uuid` - Unique ID generation
- `dashmap` - Concurrent HashMap
- `tracing` - Logging

## 🤝 Contributing

1. Fork repository
2. Create feature branch
3. Commit changes
4. Push dan create PR

## 📝 License

MIT License - feel free to use untuk project kamu!

## 🎯 Roadmap

- [ ] Connection manager untuk broadcast
- [ ] Example client (JavaScript/HTML)
- [ ] Example card game implementation
- [ ] Authentication system
- [ ] Persistence layer
- [ ] Admin panel
- [ ] Metrics & monitoring

## 💡 Tips Development

### Testing dengan wscat

```bash
# Install wscat
npm install -g wscat

# Connect
wscat -c ws://localhost:8080

# Send message
{"type":"create_room","room_type":"card_game","max_players":4}
```

### Debug Logging

```bash
RUST_LOG=debug cargo run
```

---

Built with ❤️ using Rust
