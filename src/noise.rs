use std::path::Path;

use rand::prelude::*;

pub fn apply_noise_to_file<P>(input_file: P, output_file: P, noise_chance: f64)
where
    P: AsRef<Path>,
{
    let mut file_content = std::fs::read(input_file).expect("File doesn't exist");

    for byte in &mut file_content {
        let mask = prepare_mask(noise_chance);

        *byte ^= mask;
    }

    std::fs::write(output_file, file_content).expect("Couldn't write to file");
}

fn prepare_mask(noise_chance: f64) -> u8 {
    let mut mask = 0;

    for i in 0..8 {
        if thread_rng().gen::<f64>() < noise_chance {
            mask += 1 << i;
        }
    }

    mask
}
