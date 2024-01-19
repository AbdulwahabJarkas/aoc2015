type Dimensions = (usize, usize, usize);

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Vec<Dimensions> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split('x');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(dims: &[Dimensions]) -> usize {
    dims.iter()
        .map(|&(l, w, h)| {
            let a = l * w;
            let b = w * h;
            let c = h * l;
            let slack = a.min(b).min(c);
            2 * (a + b + c) + slack
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(dims: &[Dimensions]) -> usize {
    dims.iter()
        .map(|&(l, w, h)| {
            let a = 2 * (l + w);
            let b = 2 * (w + h);
            let c = 2 * (l + h);
            l * w * h + a.min(b).min(c)
        })
        .sum()
}
