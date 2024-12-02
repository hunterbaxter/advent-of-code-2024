use std::collections::HashMap;

// NOTE: we are not using a generator, and parsing will be separate from the actual logic
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (numbers1, numbers2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .collect::<Vec<_>>()
        .into_iter()
        .unzip();

    (numbers1, numbers2)
}

pub fn part1_v0(input: &str) -> u32 {
    let (mut numbers1, mut numbers2) = input_generator(input);
    numbers1.sort_unstable();
    numbers2.sort_unstable();

    numbers1
        .into_iter()
        .zip(numbers2)
        .map(|(x, y)| y.abs_diff(x))
        .collect::<Vec<u32>>()
        .into_iter()
        .sum()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    part1_v0(input)
}

pub fn part2_v0(input: &str) -> u32 {
    let (numbers1, numbers2) = input_generator(input);
    let (numbers1, numbers2): (HashMap<u32, u32>, HashMap<u32, u32>) =
        numbers1.into_iter().zip(numbers2).fold(
            (HashMap::new(), HashMap::new()),
            |(mut map1, mut map2), (n1, n2)| {
                *map1.entry(n1).or_default() += 1;
                *map2.entry(n2).or_default() += 1;
                (map1, map2)
            },
        );

    numbers1
        .into_iter()
        .map(|(k, left_count)| {
            if let Some(right_count) = numbers2.get(&k) {
                k * right_count * left_count
            } else {
                0
            }
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    part2_v0(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part_one_example() {
        let result = part1(&crate::read_file("examples", 1));
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two_example() {
        let result = part2(&crate::read_file("examples", 1));
        assert_eq!(result, 31);
    }

    #[test]
    fn day1_part_one_answer() {
        let result = part1(&crate::read_file("input/2024", 1));
        assert_eq!(result, 2580760);
    }

    #[test]
    fn test_part_two_answer() {
        let result = part2(&crate::read_file("input/2024", 1));
        assert_eq!(result, 25358365);
    }
}
