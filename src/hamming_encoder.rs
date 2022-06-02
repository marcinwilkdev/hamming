use std::path::Path;

const HAMMING_CACHE: [u8; 16] = [
    0b00000000, 0b11100001, 0b10011001, 0b01111000, 0b01010101, 0b10110100, 0b11001100, 0b00101101,
    0b11010010, 0b00110011, 0b01001011, 0b10101010, 0b10000111, 0b01100110, 0b00011110, 0b11111111,
];

pub fn perfrom_hamming_encoding<P>(input: P, output: P)
where
    P: AsRef<Path>,
{
    let buff = std::fs::read(input).expect("File doesn't exist");

    let coded_chunks = buff
        .into_iter()
        .map(|byte| CodedChunk::from_byte(byte))
        .flat_map(|coded_chunk| coded_chunk.as_bytes())
        .collect::<Vec<_>>();

    std::fs::write(output, coded_chunks).expect("Couldn't write to file");
}

struct CodedChunk {
    chunk: [u8; 2],
}

impl CodedChunk {
    fn from_byte(byte: u8) -> CodedChunk {
        let first_part = (byte & 0b11110000) >> 4;
        let second_part = byte & 0b00001111;

        let coded_first_part = CodedChunk::encode_part(first_part);
        let coded_second_part = CodedChunk::encode_part(second_part);

        CodedChunk {
            chunk: [coded_first_part, coded_second_part],
        }
    }

    fn encode_part(byte: u8) -> u8 {
        HAMMING_CACHE[byte as usize]
    }

    fn as_bytes(self) -> [u8; 2] {
        self.chunk
    }
}
