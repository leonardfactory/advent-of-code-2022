use std::ops::{Index, IndexMut};

pub struct Message {
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, Copy)]
pub struct Link {
    pub value: i64,
    pub prev: i64,
    pub next: i64,
}

impl Message {
    pub fn parse(input: &str) -> Self {
        let numbers: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
        Self::from_vec(numbers)
    }

    pub fn from_vec(numbers: Vec<i64>) -> Self {
        let size = numbers.len();
        let links = numbers
            .iter()
            .enumerate()
            .map(|(i, &value)| Link {
                value,
                prev: ((i + size - 1) % size) as i64,
                next: ((i + 1) % size) as i64,
            })
            .collect();

        Message { links }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> i64 {
        self.links.len() as i64
    }

    pub fn to_vec(&self, rotate: i64) -> Vec<i64> {
        let mut numbers = Vec::with_capacity(self.links.len());
        let mut link = &self[rotate];
        numbers.push(link.value);
        for _ in 0..self.links.len() - 1 {
            link = &self[link.next];
            numbers.push(link.value);
        }
        numbers
    }

    pub fn find_index(&self, current: i64, delta: i64) -> i64 {
        let mut index = current;
        let mut link = &self[current];

        // This line is literally Part 2
        let delta = delta % (self.len() - 1);

        for _ in 0..delta.abs() {
            match delta.signum() {
                1 => {
                    index = link.next;
                    if index == current {
                        index = self[index].next;
                    }
                }
                -1 => {
                    index = link.prev;
                    if index == current {
                        index = self[index].prev;
                    }
                }
                _ => unreachable!(),
            }
            link = &self[index as i64];
        }
        index
    }
}

impl Index<i64> for Message {
    type Output = Link;

    fn index(&self, index: i64) -> &Self::Output {
        let len = self.links.len() as i64;
        &self.links[((index + len) % len) as usize]
    }
}

impl IndexMut<i64> for Message {
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        let len = self.links.len() as i64;
        &mut self.links[((index + len) % len) as usize]
    }
}

pub fn reorder_step(message: &mut Message, i: i64) {
    let link = message[i];
    // let delta = link.value % message.len();
    let swap_link_index = message.find_index(i, link.value);
    let swap = message[swap_link_index];

    let sign = link.value.signum();

    match sign {
        1 => {
            // 1, (2), 3, [4], 5 -->  1, 3, [4], (2), 5
            message[link.prev].next = link.next;
            message[link.next].prev = link.prev;

            message[swap_link_index].next = i;
            message[swap.next].prev = i;

            message[i].prev = swap_link_index;
            message[i].next = swap.next;
        }
        -1 => {
            // 1, [2], 3, (-2), 5 -->  1, (-2), [2], 3, 5
            message[link.prev].next = link.next;
            message[link.next].prev = link.prev;

            message[swap_link_index].prev = i;
            message[swap.prev].next = i;

            message[i].next = swap_link_index;
            message[i].prev = swap.prev;
        }
        _ => {}
    }
}

pub fn reorder(input: &str) -> Message {
    let mut message = Message::parse(input);

    for i in 0..message.len() {
        reorder_step(&mut message, i);
    }

    message
}

#[allow(dead_code)]
fn reorder_vec(input: Vec<i64>) -> Message {
    let mut message = Message::from_vec(input);

    for i in 0..message.len() {
        reorder_step(&mut message, i);
    }

    message
}

pub fn groove_coordinates(input: &str) -> i64 {
    let message = reorder(input);
    message_coordinates(&message)
}

pub fn message_coordinates(message: &Message) -> i64 {
    let zero_index = message
        .links
        .iter()
        .position(|link| link.value == 0)
        .unwrap();

    let numbers = message.to_vec(zero_index as i64);
    let coordinates: Vec<_> = (1..=3)
        .map(|x| numbers[(x * 1000) % numbers.len()])
        .collect();
    println!("coords={:?}", coordinates);

    coordinates.iter().sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_movements() {
        let input = "1\n2\n-3\n3\n-2\n0\n4";
        let mut message = Message::parse(input);
        assert_eq!(message.to_vec(0), vec![1, 2, -3, 3, -2, 0, 4]);

        reorder_step(&mut message, 0);
        assert_eq!(message.to_vec(1), vec![2, 1, -3, 3, -2, 0, 4]);

        reorder_step(&mut message, 1);
        assert_eq!(message.to_vec(0), vec![1, -3, 2, 3, -2, 0, 4]);

        reorder_step(&mut message, 2);
        assert_eq!(message.to_vec(0), vec![1, 2, 3, -2, -3, 0, 4]);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut message = Message::from_vec(vec![4, -2, 5, 6, 7, 8, 9]);
        reorder_step(&mut message, 1);
        assert_eq!(message.to_vec(0), vec![4, 5, 6, 7, 8, -2, 9]);

        // 3, 1, 0
        let mut message = Message::from_vec(vec![3, 1, 0]);
        reorder_step(&mut message, 0);
        assert_eq!(message.to_vec(1), vec![1, 3, 0]);
        reorder_step(&mut message, 1);
        assert_eq!(message.to_vec(0), vec![3, 1, 0]);
        reorder_step(&mut message, 2);
        assert_eq!(message.to_vec(0), vec![3, 1, 0]);

        assert_eq!(reorder_vec(vec![3, 1, 0]).to_vec(0), vec![3, 1, 0]); // (3), 1, 0 -> 1, (3), 0 -> 1, 0, (3) -> 1, (3), 0
    }

    #[test]
    fn test_mixing() {
        let input = "1\n2\n-3\n3\n-2\n0\n4";
        let message = reorder(input);
        assert_eq!(message.to_vec(0), vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(groove_coordinates(input), 3);
    }
}
