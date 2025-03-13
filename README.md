# Crab Game

A simple 2D game where you control a crab by moving its legs. The game is built using Rust and the Macroquad game framework.

[WASM version - in development](https://yanfrimmel.github.io/crab-game/)

## Features
- Control the each of crab's legs to move and rotate.
- Gravity simulation for realistic movement.
- Collision detection.
- Jump.

## Prerequisites

Before running the game, ensure you have the following installed:

- **Rust + Cargo**.
- **[basic-http-server](https://github.com/brson/basic-http-server)**: For running WASM.
- **Macroquad OS specific dependencies:** see [https://github.com/not-fl3/macroquad](https://github.com/not-fl3/macroquad).

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/crab-game.git
   cd crab-game
   ```

2. Build the project:

   ```bash
   cargo build
   ```
3. Add WASM support:

   ```bash
   rustup target add wasm32-unknown-unknown
   ```
## Running the Game

To run the game, use the following command:

```bash
cargo run
```
To run in browser(requires basic-http-server):


```bash
sh runWasm.sh
```

## Controls

- **A / D**: Rotate the left leg.
- **Left Arrow / Right Arrow**: Rotate the right leg.
- **Up Arrow**: Move the crab upward.
- **Down Arrow**: Move the crab downward.
- **Space**: Jump.
