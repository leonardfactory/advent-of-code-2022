pub fn decode_snafu(input: &str) -> i64 {
    input
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            '2' => 2 * 5_i64.pow(i as u32),
            '1' => 5_i64.pow(i as u32),
            '0' => 0,
            '-' => -(5_i64.pow(i as u32)),
            '=' => -2 * 5_i64.pow(i as u32),
            _ => panic!("Invalid character"),
        })
        .sum()
}

pub fn encode_snafu(value: i64) -> String {
    let mut result = String::new();
    let mut value = value;
    let mut _i = 0;
    while value != 0 {
        let digit = value % 5;
        value /= 5;
        if digit >= 3 {
            value += 1;
        }
        // println!("i={}, value={}, digit={}", i, value, digit);
        match digit {
            2 => result.push('2'),
            1 => result.push('1'),
            0 => result.push('0'),
            3 => result.push('='),
            4 => result.push('-'),
            _ => panic!("Invalid digit"),
        }
        _i += 1;
    }
    result.chars().rev().collect()
}

pub fn count_fuel_snafu(input: &str) -> String {
    let sum = input.lines().map(decode_snafu).sum();
    encode_snafu(sum)
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_decode_snafu() {
        assert_eq!(decode_snafu("2=-01"), 976);
        assert_eq!(decode_snafu("2="), 8);
        assert_eq!(decode_snafu("20"), 10);
        assert_eq!(decode_snafu("1121-1110-1=0"), 314159265);
        assert_eq!(decode_snafu("1=-0-2"), 1747);
        assert_eq!(decode_snafu("20012"), 1257);
    }

    #[test]
    fn test_encode_snafu() {
        assert_eq!(encode_snafu(7), "12");
        assert_eq!(encode_snafu(8), "2=");
        assert_eq!(encode_snafu(10), "20");
        assert_eq!(encode_snafu(976), "2=-01");
        assert_eq!(encode_snafu(314159265), "1121-1110-1=0");
        assert_eq!(encode_snafu(1747), "1=-0-2");
        assert_eq!(encode_snafu(1257), "20012");
    }
}
