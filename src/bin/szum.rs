use hamming::noise;

fn main() {
    let noise_chance = std::env::args()
        .nth(1)
        .expect("You have to specify noise chance")
        .parse::<f64>()
        .expect("Noise chance has to be float");

    let input_file = std::env::args()
        .nth(2)
        .expect("You have to specify input file");

    let output_file = std::env::args()
        .nth(3)
        .expect("You have to specify output file");

    noise::apply_noise_to_file(input_file, output_file, noise_chance);
}
