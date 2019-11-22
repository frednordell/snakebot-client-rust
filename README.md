This snake-bot was build to compete in a competition with collaborated betweenen Cygni and D-sektionen.
It wheighs different tiles with BFS and picks the direction with the highest score.

# SNAKE CLIENT 
[![Build Status](https://travis-ci.org/cygni/snakebot-client-rust.svg?branch=master)](https://travis-ci.org/cygni/snakebot-client-rust)

Do you want the most annoying compiler ever?
Do you want to constantly think of what is owning what variable?
Do you want to stare angrily at the screen and wonder what the hell it means that some dumb value can't be moved?
Then here is the ultimate snake client for you, written for the beautiful language Rust.

## Requirements

- Rust (which should be installed via [rustup](https://github.com/rust-lang-nursery/rustup.rs))
- Snake server (local or remote)

## Setup

A. Clone the repository: `git clone https://github.com/cygni/snakebot-client-rust.git`

B. Open the repo: `cd snakebot-client-rust`

C. Run the snake: `cargo run`

D. Improve the snake: edit `src/snake.rs`, and more specifically `get_next_move`.
