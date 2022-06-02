use hamming::checker;

fn main() {
    let first_file = std::env::args().nth(1).expect("You have to provide input file.");
    let second_file = std::env::args().nth(2).expect("You have to provide output file.");

    checker::check_files(first_file, second_file);
}
