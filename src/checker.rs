use std::path::Path;

pub fn check_files<P>(first_file: P, second_file: P)
where
    P: AsRef<Path>,
{
    let first_file_content = std::fs::read(first_file).expect("File couldn't be opened");
    let second_file_content = std::fs::read(second_file).expect("File couldn't be opened");

    let mut different_blocks = 0;

    for (first_byte, second_byte) in first_file_content.into_iter().zip(second_file_content.into_iter()) {
        let [first_byte_left, first_byte_right] = split_byte_into_two(first_byte);
        let [second_byte_left, second_byte_right] = split_byte_into_two(second_byte);

        if first_byte_left != second_byte_left {
            different_blocks += 1;
        }

        if first_byte_right != second_byte_right {
            different_blocks += 1;
        }
    }

    println!("There are {} different 4-bit blocks.", different_blocks);
}

#[inline]
fn split_byte_into_two(byte: u8) -> [u8; 2] {
    [(byte & 0b11110000) >> 4, byte & 0b00001111]
}
