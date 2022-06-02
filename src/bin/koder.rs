use hamming::hamming_encoder;

fn main() {
    let input_file = std::env::args().nth(1).expect("You have to provide input file.");
    let output_file = std::env::args().nth(2).expect("You have to provide output file.");

    hamming_encoder::perfrom_hamming_encoding(input_file, output_file);
}
