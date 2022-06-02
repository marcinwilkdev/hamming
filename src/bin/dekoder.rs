use hamming::hamming_decoder;

fn main() {
    let input_file = std::env::args().nth(1).expect("You have to provide input file.");
    let output_file = std::env::args().nth(2).expect("You have to provide output file.");

    hamming_decoder::perfrom_hamming_decoding(input_file, output_file);
}
