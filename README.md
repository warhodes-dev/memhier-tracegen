# memhier-tracegen

## Building

run `cargo build --release` in the project directory. The executable will be located here: `./target/release/tracegen`

## Usage

Run the executable with a provided config file. Create different configs to test various edge cases.

Example: Generate a simple trace file of 100 memory accesses:
```
./tracegen --config ./trace.config 100
```

Example: Generate a trace input with 100 memory accesses, 25 unique addresses, with 25% writes and 75% reads:
```
./tracegen --config ./my_configs/config1 -rrrw 100 25
```
