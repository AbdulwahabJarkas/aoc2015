type Dimensions = (usize, usize, usize);

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Vec<Dimensions> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split('x');
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(dims: &[Dimensions]) -> usize {
    dims.iter()
        .map(|dim| {
            let (l, w, h) = dim;
            let a: usize = l * w;
            let b = w * h;
            let c = l * h;
            a.min(b).min(c) + 2 * (a + b + c)
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(dims: &[Dimensions]) -> usize {
    dims.iter()
        .map(|dim| {
            let (l, w, h) = dim;
            let a = 2 * (l + w);
            let b = 2 * (w + h);
            let c = 2 * (l + h);
            l * w * h + a.min(b).min(c)
        })
        .sum()
}