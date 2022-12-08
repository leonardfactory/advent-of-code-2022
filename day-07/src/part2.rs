use itertools::Itertools;

use crate::part1::parse_tree;

const HDD_SIZE: usize = 70_000_000;
const NEEDED_SIZE: usize = 30_000_000;

pub fn find_deletable_directory_size(input: &str) -> usize {
    let (tree, root_id) = parse_tree(input);
    
    let unused_size = HDD_SIZE - tree.get(root_id).unwrap().get().get_dir_size();
    let needed_size = NEEDED_SIZE - unused_size;

    tree.iter()
        .map(|n| n.get().get_dir_size())
        .filter(|&size| size > needed_size)
        .sorted()
        .next()
        .unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(find_deletable_directory_size(input), 24933642);
    }
}
