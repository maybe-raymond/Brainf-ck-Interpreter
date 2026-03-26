# Brainf\*ck Interpreter in Rust

Based on the [BrainFuck Programming Tutorial by: Katie](https://gist.github.com/roachhd/dce54bec8ba55fb17d3u) and the [Fireship video](https://www.youtube.com/watch?v=hdHjjBS4cs8). This project exists as a way to learn the Rust programming language.

Currently the memory array has a size of 16 bytes, this was done as a way to visualise the memory layout while debugging but can be increase manually within the `main.rs` file.

Note: This implementation is slightly different from the one found on [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck) because the Wikipedia version implements `[` and `]` differently compared to the Katie implementation and so the Wikipedia examples will not work with this version.

## Building

```bash
# Build the project
cargo build --release
```

## Usage

```bash
cargo run -- examples/hello_world.bf
```

Example files are given in the `examples` folder that show some of the programs that can be written in the language.

If you want to see something cool run the following command

```bash
cargo run -- examples/triangle.bf
```

## Future Fixes

- Error handling for array access
- Capacity size from the terminal
- Better Error handling for the stack and the rest of the program.
