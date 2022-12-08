use md5::{Md5, Digest};

const INPUT: &'static str = "qljzarfv";
const TEST: &'static str = "ihgpwlah";

fn is_open(value: u8, x: isize, y: isize) -> bool {
    if x < 0 || x > 3 {
        return false;
    }

    if y < 0 || y > 3 {
        return false;
    }

    value >= 0xb
}

fn search(input: &str, x: isize, y: isize, is_min: bool) -> Option<String> {
    if x == 3 && y == 3 {
        return Some("".to_string());
    }

    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let room_hash = hasher.finalize();

    let up = room_hash[0] >> 4;
    let down = room_hash[0] & 0b1111;
    let left = room_hash[1] >> 4;
    let right = room_hash[1] & 0b1111;

    let directions = [(up, "U", 0, -1), (down, "D", 0, 1), (left, "L", -1, 0), (right, "R", 1, 0)];

    let iter = directions.into_iter().filter_map(|(d, ch, dx, dy)| {
        if is_open(d, x + dx, y + dy) {
            search(&format!("{}{}", input, ch), x + dx, y + dy, is_min)
                .map(|s| format!("{}{}", ch, s))
        } else {
            None
        }
    });

    if is_min {
        iter.min_by_key(|s| s.len())
    } else {
        iter.max_by_key(|s| s.len())
    }
}

fn main() {
    println!("part1 (test): {:?}", search(TEST, 0, 0, true));
    println!("part1 (actual): {:?}", search(INPUT, 0, 0, true));
    println!("part2 (test): {:?}", search(TEST, 0, 0, false).map(|s| s.len()));
    println!("part2 (actual): {:?}", search(INPUT, 0, 0, false).map(|s| s.len()));
}
