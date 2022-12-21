use crate::part1::{message_coordinates, reorder_step, Message};

const DECRYPTION_KEY: i64 = 811_589_153;

pub fn decrypt_coordinates(input: &str) -> i64 {
    let mut message = Message::parse(input);
    message
        .links
        .iter_mut()
        .for_each(|link| link.value *= DECRYPTION_KEY);

    for _ in 0..10 {
        for i in 0..message.len() {
            reorder_step(&mut message, i);
        }
    }

    message_coordinates(&message)
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(decrypt_coordinates(input), 1623178306);
    }
}
