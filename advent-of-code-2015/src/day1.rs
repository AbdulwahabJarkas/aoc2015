#[aoc(day1, part1)]
pub fn solve_part1(input: &[u8]) -> i32 {
    input.iter().fold(0, |floor, b| match b {
        b'(' => floor + 1,
        b')' => floor - 1,
        _ => floor, // Ignore any other characters
    })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut floor = 0;

    for (i, b) in input.bytes().enumerate() {
        match b {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => {}
        }

        if floor == -1 {
            return i + 1; // +1 because position is 1-indexed, not 0-indexed
        }
    }

    0 // Return 0 if basement is never reached
}
