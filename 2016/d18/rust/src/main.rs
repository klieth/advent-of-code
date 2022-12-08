
const TEST: &'static str = include_str!("../../test");
const INPUT: &'static str = include_str!("../../input");

fn compute(line: &[u8], rounds: usize) -> usize {
    let mut ret = line.iter().filter(|ch| **ch == b'.').count();

    let mut line = line.to_owned();

    for _ in 0..rounds - 1 {
        let mut new_line = vec![];

        for i in 0..line.len() {
            let (l, r) = (
                i == 0 || line[i - 1] == b'.',
                line.get(i + 1).map(|c| *c == b'.').unwrap_or(true),
            );

            if l ^ r {
                new_line.push(b'^');
            } else {
                ret += 1;
                new_line.push(b'.');
            }
        }

        line = new_line;
    }

    ret
}

fn main() {
    println!("part1 (test): {}", compute(TEST.trim().as_bytes(), 10));
    println!("part1 (actual): {}", compute(INPUT.trim().as_bytes(), 40));
    println!("part2 (actual): {}", compute(INPUT.trim().as_bytes(), 400000));
}
