pub fn encode(input: &[u8]) -> String {
    let mut encoded = String::new();

    for chunk in input.chunks_exact(3) {
        let combined =
            (u32::from(chunk[0]) << 16) | (u32::from(chunk[1]) << 8) | u32::from(chunk[2]);

        let v1 = (combined >> 18) & 0b111111;
        let v2 = (combined >> 12) & 0b111111;
        let v3 = (combined >> 6) & 0b111111;
        let v4 = combined & 0b111111;

        let c1 = ALPHABET[v1 as usize];
        let c2 = ALPHABET[v2 as usize];
        let c3 = ALPHABET[v3 as usize];
        let c4 = ALPHABET[v4 as usize];

        encoded.push(c1);
        encoded.push(c2);
        encoded.push(c3);
        encoded.push(c4);
    }

    match input.len() % 3 {
        1 => {
            let combined = u32::from(input[input.len() - 1]) << 16;
            let v1 = (combined >> 18) & 0b111111;
            let v2 = (combined >> 12) & 0b111111;
            let c1 = ALPHABET[v1 as usize];
            let c2 = ALPHABET[v2 as usize];
            encoded.push(c1);
            encoded.push(c2);
            encoded.push('=');
            encoded.push('=');
        }
        2 => {
            let combined = (u32::from(input[input.len() - 2]) << 16)
                | (u32::from(input[input.len() - 1]) << 8);
            let v1 = (combined >> 18) & 0b111111;
            let v2 = (combined >> 12) & 0b111111;
            let v3 = (combined >> 6) & 0b111111;
            let c1 = ALPHABET[v1 as usize];
            let c2 = ALPHABET[v2 as usize];
            let c3 = ALPHABET[v3 as usize];
            encoded.push(c1);
            encoded.push(c2);
            encoded.push(c3);
            encoded.push('=');
        }
        _ => {}
    }

    encoded
}

const ALPHABET: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input = b"";
        assert_eq!(encode(input), "");

        let input = b"Hello, World!";
        assert_eq!(encode(input), "SGVsbG8sIFdvcmxkIQ==");
    }
}
