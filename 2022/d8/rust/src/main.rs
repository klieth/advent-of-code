const TEST: &'static str = include_str!("../../test");
const INPUT: &'static str = include_str!("../../input");

fn parse(raw: &str) -> Vec<Vec<u8>> {
    raw.lines().map(|line| {
        line.chars().map(|ch| (ch as u8) - b'0').collect()
    }).collect()
}

fn is_visible(data: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let current = data[i][j];

    if ((i + 1)..data.len()).all(|x| data[x][j] < current) {
        return true;
    }

    if (0..i).all(|x| data[x][j] < current) {
        return true;
    }

    if ((j + 1)..data[0].len()).all(|y| data[i][y] < current) {
        return true;
    }

    if (0..j).all(|y| data[i][y] < current) {
        return true;
    }

    return false;
}

fn scenic_score(data: &Vec<Vec<u8>>, i: usize, j: usize) -> usize {
    let current = data[i][j];

    let mut product = 1;

    let mut count = 0;
    for x in (i + 1)..data.len() {
        let seen = data[x][j];
        count += 1;
        if seen >= current {
            break;
        }
    }
    product *= count;

    count = 0;
    for x in (0..i).rev() {
        let seen = data[x][j];
        count += 1;
        if seen >= current {
            break;
        }
    }
    product *= count;

    count = 0;
    for y in ((j + 1)..data[0].len()) {
        let seen = data[i][y];
        count += 1;
        if seen >= current {
            break;
        }
    }
    product *= count;

    count = 0;
    for y in (0..j).rev() {
        let seen = data[i][y];
        count += 1;
        if seen >= current {
            break;
        }
    }
    product *= count;

    product
}

fn part1(data: &Vec<Vec<u8>>) -> usize {
    let mut res = 0;

    for (i, row) in data.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_visible(data, i, j) {
                res += 1;
            }
        }
    }

    res
}

fn part2(data: &Vec<Vec<u8>>) -> usize {
    let mut max = 0;

    for (i, row) in data.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let score = scenic_score(data, i, j);

            if score > max {
                max = score;
            }
        }
    }

    max
}

fn main() {
    let test = parse(TEST);
    let input = parse(INPUT);
    println!("part1 (test): {}", part1(&test));
    println!("part1 (actual): {}", part1(&input));
    println!("part2 (test): {}", part2(&test));
    println!("part2 (actual): {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scenic_score_1_2() {
        let test = parse(TEST);
        assert_eq!(test[1][2], 5);
        assert_eq!(scenic_score(&test, 1, 2), 4);
    }

    #[test]
    fn scenic_score_3_2() {
        let test = parse(TEST);
        assert_eq!(test[3][2], 5);
        assert_eq!(scenic_score(&test, 3, 2), 8);
    }
}
