const TEST: usize = 5;
const INPUT: usize = 3005290;

fn josephus(num_elves: usize) -> usize {
    let power = (num_elves as f64).log(2.0).floor() as u32;
    let l = num_elves - (2usize).pow(power);
    (l * 2) + 1
}

fn josephish(num_elves: usize) -> usize {
    let power = (num_elves as f64).log(3.0).floor() as u32;
    let power = (3usize).pow(power);
    let remainder = num_elves - power;

    if remainder == 0 {
        power
    } else if remainder <= power {
        remainder
    } else {
        power + (2 * (remainder - power))
    }
}

fn main() {
    println!("part1 (test): {}", josephus(TEST));
    println!("part1 (actual): {}", josephus(INPUT));
    println!("part2 (test): {}", josephish(TEST));
    println!("part2 (actual): {}", josephish(INPUT));
}
