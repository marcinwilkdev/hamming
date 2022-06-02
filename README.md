# hamming
Hamming encoding and decoding with helpers.

## Setup
You need cargo to build and run this program.
You can install it using rustup: https://rustup.rs/

To run this project locally, compile it using cargo:
```
cargo build --release
````

## Code examples
Encode 'input_file' with Hamming code with additional parity bit and save result in 'output_file'.
```
./target/release/koder 'input_file' 'output_file'
```

Add noise to 'input_file' by flipping every bit with 'p' probability and save result in 'ouptut_file'.
```
./target/release/szum 'p' 'input_file' 'output_file'
```

Decode 'input_file' with Hamming code and additional parity bit, print out how many times two errors in one block
have been encountered and save result in 'output_file'.
```
./target/release/dekoder 'input_file' 'output_file'
```

Check how many 4-bit blocks are different between 'input_file' and 'output_file' and print result to screen.
```
./target/release/sprawdz 'input_file' 'output_file'
```
