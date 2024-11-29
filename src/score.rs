pub fn get_leading_zeroes(addr: &str) -> (usize, u32) {
    let mut leading_zeroes = 0;
    let mut non_zero_idx = 0;
    for (idx, char) in addr.chars().enumerate() {
        if char == '0' {
            leading_zeroes += 1;
        } else {
            non_zero_idx = idx;
            break;
        }
    }

    // return index of first non-zero byte and number of leading zeroes
    (non_zero_idx, leading_zeroes)
}

/// Rate an address based on the following criteria:
// - 10 points for every leading 0 nibble
// - 40 points if the first 4 is followed by 3 more 4s
// - 20 points if the first nibble after the 4 4s is NOT a 4
// - 20 points if the last 4 nibbles are 4s
// - 1 point for every 4
pub fn rate_address(addr: &str) -> u32 {
    let addr = String::from(addr)
        .trim_start_matches("0x")
        .to_ascii_lowercase();

    let magic_4 = "4444";
    let mut points: u32 = 0;

    let (idx, leading_zeroes) = get_leading_zeroes(&addr);

    points += leading_zeroes * 10;

    if addr.chars().nth(idx).unwrap() != '4' {
        return 0;
    }

    let first_four = &addr[idx..idx + 4];
    if first_four == magic_4 {
        points += 40;

        let next_char = addr.chars().nth(idx + 4).unwrap();
        if next_char != '4' {
            points += 20;
        }
    }

    let last_four = &addr[addr.len() - 4..];
    if last_four == magic_4 {
        points += 20;
    }

    let rest = &addr[idx..];
    for char in rest.chars() {
        if char == '4' {
            points += 1;
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let addr = "0x000000000004444c5dc75cB358380D2e3dE08A90";

        assert_eq!(rate_address(addr), 174);

        let addr = "0x00000000004444c4F1f4c11953bB3997274ea8B2";

        assert_eq!(rate_address(addr), 167);

        let addr = "0x00000004444Dc6335C3721F0dc7cF4340d344444";

        assert_eq!(rate_address(addr), 161);

        let addr = "0x00000004444eC61Aa00282943f1814b0400C10C0";

        assert_eq!(rate_address(addr), 137);

        let addr = "0x00000004444eC61Aa00282943f1814b0400C10C0";

        assert_eq!(rate_address(addr), 137);
    }
}
