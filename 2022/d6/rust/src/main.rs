const INPUT_DATA: &'static str = include_str!("../../input");

fn unique(size: usize, chars: &[char]) -> bool {
    for i in 0..(size - 1) {
        for j in (i + 1)..size {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }

    return true;
}

fn search(size: usize, signal: &str) -> usize {
    let chars: Vec<char> = signal.chars().collect();

    for (idx, window) in chars.windows(size).enumerate() {
        if unique(size, window) {
            return idx + size;
        }
    }

    unreachable!()
}

fn main() {
    println!("part1 (actual): {:?}", search(4, &INPUT_DATA));
    println!("part2 (actual): {:?}", search(14, &INPUT_DATA));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unique_true() {
        let data = ['a', 'b', 'c', 'd'];
        assert!(unique(data.len(), &data));
    }

    #[test]
    fn unique_false() {
        let data = ['a', 'a', 'a', 'a'];
        assert!(!unique(data.len(), &data));
    }

    macro_rules! gen_test {
        ($name:ident, $input:expr, $expected_part1:expr, $expected_part2:expr) => {
            mod $name {
                use super::*;

                #[test]
                fn part1() {
                    let result = search(4, $input);
                    assert!(result == $expected_part1);
                }

                #[test]
                fn part2() {
                    let result = search(14, $input);
                    assert!(result == $expected_part2);
                }
            }
        }
    }

    gen_test!(one, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19);
    gen_test!(two, "bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23);
    gen_test!(three, "nppdvjthqldpwncqszvftbrmjlhg", 6, 23);
    gen_test!(four, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29);
    gen_test!(five, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26);
}
