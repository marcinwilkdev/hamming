use std::path::Path;

use nalgebra::{SMatrix, SVector};

type Matrix3x7 = SMatrix<u8, 3, 7>;
type Vector7 = SVector<u8, 7>;

const REVERSE_HAMMING_CACHE: [u8; 256] = {
    let mut rev_hamming_cache = [0; 256];

    rev_hamming_cache[0b00000000] = 0;
    rev_hamming_cache[0b11100001] = 1;
    rev_hamming_cache[0b10011001] = 2;
    rev_hamming_cache[0b01111000] = 3;
    rev_hamming_cache[0b01010101] = 4;
    rev_hamming_cache[0b10110100] = 5;
    rev_hamming_cache[0b11001100] = 6;
    rev_hamming_cache[0b00101101] = 7;
    rev_hamming_cache[0b11010010] = 8;
    rev_hamming_cache[0b00110011] = 9;
    rev_hamming_cache[0b01001011] = 10;
    rev_hamming_cache[0b10101010] = 11;
    rev_hamming_cache[0b10000111] = 12;
    rev_hamming_cache[0b01100110] = 13;
    rev_hamming_cache[0b00011110] = 14;
    rev_hamming_cache[0b11111111] = 15;

    rev_hamming_cache
};

pub fn perfrom_hamming_decoding<P>(input: P, output: P)
where
    P: AsRef<Path>,
{
    let buff = std::fs::read(input).expect("File doesn't exist");

    let mut double_errors = 0;

    let mut parts = vec![];

    for byte in buff {
        match byte {
            0b00000000 | 0b11100001 | 0b10011001 | 0b01111000 | 0b01010101 | 0b10110100
            | 0b11001100 | 0b00101101 | 0b11010010 | 0b00110011 | 0b01001011 | 0b10101010
            | 0b10000111 | 0b01100110 | 0b00011110 | 0b11111111 => {
                parts.push(REVERSE_HAMMING_CACHE[byte as usize])
            }
            byte => {
                let parity_ok = check_parity(byte);
                let error_index = get_error_index(byte);

                if !parity_ok && error_index == 0 {
                    parts.push(REVERSE_HAMMING_CACHE[(byte ^ 0b00000001) as usize]);
                } else if !parity_ok && error_index > 0 {
                    let mask = 1 << (8 - error_index);

                    parts.push(REVERSE_HAMMING_CACHE[(byte ^ mask) as usize]);
                } else {
                    double_errors += 1;
                }
            }
        }
    }

    let merged_parts = merge_parts(&parts);

    println!("Double errors encountered: {}", double_errors);

    std::fs::write(output, merged_parts).expect("Couldn't write to file");
}

fn check_parity(byte: u8) -> bool {
    let mut num_ones = 0;

    for i in 0..8 {
        if (1 << i) & byte > 0 {
            num_ones += 1;
        }
    }

    num_ones % 2 == 0
}

fn get_error_index(byte: u8) -> usize {
    let matrix = Matrix3x7::from_row_slice(&[
        1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1,
    ]);

    let mut bits = [0; 7];

    for i in 0..7 {
        let mask = 7 - i;

        bits[i] = (byte & (1 << mask)) >> mask;
    }

    let vector = Vector7::from_row_slice(&bits);

    let mut result_vetor = matrix * vector;

    let mut index = 0;

    for i in 0..3 {
        result_vetor[(i, 0)] %= 2;

        index += result_vetor[(i, 0)] * (1 << i);
    }

    index.into()
}

fn merge_parts(parts: &[u8]) -> Vec<u8> {
    let mut parts_iter = parts.into_iter();

    let mut merged_parts = vec![];

    loop {
        let first_part = parts_iter.next();
        let second_iter = parts_iter.next();

        match (first_part, second_iter) {
            (None, None) => break,
            (Some(part), None) => {
                merged_parts.push(part << 4);
                break;
            }
            (Some(first_part), Some(second_part)) => {
                merged_parts.push((first_part << 4) + second_part);
            }
            _ => unreachable!(),
        }
    }

    merged_parts
}
