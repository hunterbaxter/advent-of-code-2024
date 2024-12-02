use std::collections::HashMap;

// NOTE: we are not using a generator, and parsing will be separate from the actual logic
pub fn input_generator_v0(input: &str) -> (Vec<u32>, Vec<u32>) {
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

// We are going to improve our parsing
// - Each line is number separated by 3 spaces and a new line, which is at most 5 digits and at least one digit per the example, which means u32.
// - As a result, we will have at most 14 bytes per line, and they are all ASCII
// - We can therefore work with 4 lines at once per CPU core, which we know is two for a standard Ubuntu-latest on github actions
pub fn input_generator_v1(input: &str) -> (Vec<u32>, Vec<u32>) {
    // Pre-allocate vectors to prevent resizing operations
    let line_count = input.lines().count();
    let mut numbers1 = Vec::with_capacity(line_count);
    let mut numbers2 = Vec::with_capacity(line_count);

    let bytes = input.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        // Parse first number
        let mut num = 0u32;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            // Convert ASCII digit to number and accumulate
            num = (num * 10) + (bytes[pos] - b'0') as u32;
            pos += 1;
        }
        numbers1.push(num);

        // We know we have 3 spaces, so just add 3
        pos += 1;

        // Parse second number
        num = 0;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            // Convert ASCII digit to number and accumulate
            num = (num * 10) + (bytes[pos] - b'0') as u32;
            pos += 1;
        }
        numbers2.push(num);

        // Skip newline character
        pos += 1;
    }

    (numbers1, numbers2)
}

pub fn part1_v0(input: &str) -> u32 {
    let (mut numbers1, mut numbers2) = input_generator_v1(input);
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
    let (numbers1, numbers2) = input_generator_v1(input);
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

pub fn part2_v1(input: &str) -> u32 {
    let (numbers1, numbers2) = input_generator_v1(input);
    let rows = input.len();

    let n1: HashMap<u32, u32> =
        numbers1
            .into_iter()
            .fold(HashMap::with_capacity(rows), |mut map, n| {
                *map.entry(n).or_default() += 1;
                map
            });

    numbers2
        .into_iter()
        .filter_map(|n| n1.get(&n).map(|&count1| n * count1))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    part2_v1(input)
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
