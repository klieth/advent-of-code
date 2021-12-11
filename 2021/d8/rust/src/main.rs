#![feature(drain_filter)]

use std::path::Path;
use std::fs::File;
use std::io::Read;

/*
static N0: &'static [char] = &['a', 'b', 'c', 'e', 'f', 'g'];
static N1: &'static [char] = &['c', 'f'];
static N2: &'static [char] = &['a', 'c', 'd', 'e', 'g'];
static N3: &'static [char] = &['a', 'c', 'd', 'f', 'g'];
static N4: &'static [char] = &['b', 'c', 'd', 'f'];
static N5: &'static [char] = &['a', 'b', 'd', 'f', 'g'];
static N6: &'static [char] = &['a', 'b', 'd', 'e', 'f', 'g'];
static N7: &'static [char] = &['a', 'c', 'f'];
static N8: &'static [char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g'];
static N9: &'static [char] = &['a', 'b', 'c', 'd', 'f', 'g'];
*/

fn easy_digits(entries: &Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    entries.iter().fold(0, |acc, entry| {
        let (_, output_digits) = entry;

        acc + output_digits.iter().fold(0, |acc2, digit| {
            let len = digit.len();
            acc2 + if len == 2 || len == 3 || len == 4 || len == 7 {
                1
            } else {
                0
            }
        })
    })
}

fn find_digit<P>(input_digits: &mut Vec<&str>, predicate: P) -> String where P: FnMut(&mut &str) -> bool {
    let mut digit: String = input_digits.drain_filter(predicate).collect::<Vec<_>>().pop().unwrap().into();
    unsafe { digit.as_mut().as_bytes_mut() }.sort();
    digit
}

fn full_sum(entries: &Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    entries.iter().fold(0, |acc, (input_digits, output_digits)| {
        let mut input_digits = input_digits.clone();
        let mut digit_map: Vec<Option<String>> = vec![None; 10];

        // 1 4 7 8 = easy numbers
        digit_map[1] = Some(find_digit(&mut input_digits, |e| e.len() == 2));
        digit_map[4] = Some(find_digit(&mut input_digits, |e| e.len() == 4));
        digit_map[7]= Some(find_digit(&mut input_digits, |e| e.len() == 3));
        digit_map[8] = Some(find_digit(&mut input_digits, |e| e.len() == 7));

        let mut five_segment = input_digits.drain_filter(|e| e.len() == 5).collect::<Vec<_>>();
        let mut six_segment = input_digits.drain_filter(|e| e.len() == 6).collect::<Vec<_>>();

        // 3 = five segments that include both segments of 1
        digit_map[3] = Some(find_digit(&mut five_segment, |e| digit_map[1].as_ref().unwrap().chars().all(|c| e.contains(c))));

        // 2 = five segments that include two segments of 4
        digit_map[2] = Some(find_digit(&mut five_segment, |e| {
            e.chars().filter(|c| digit_map[4].as_ref().unwrap().contains(*c)).collect::<Vec<_>>().len() == 2
        }));

        // 5 = five segments that are not 2 and 3
        digit_map[5] = Some(find_digit(&mut five_segment, |_| true));

        // 9 = six segments that contain all segments of 4
        digit_map[9] = Some(find_digit(&mut six_segment, |e| {
            digit_map[4].as_ref().unwrap().chars().all(|c| e.contains(c))
        }));

        // 0 = six segments that are not 9 and contain all segments of 1
        digit_map[0] = Some(find_digit(&mut six_segment, |e| {
            digit_map[1].as_ref().unwrap().chars().all(|c| e.contains(c))
        }));

        // 6 = six segments that are not 9 or 0
        digit_map[6] = Some(find_digit(&mut six_segment, |_| true));

        let digit_map: Vec<String> = digit_map.into_iter().map(|d| d.unwrap()).collect();

        acc + output_digits.iter().fold(0, |acc, digit| {
            let mut digit: String = digit.to_string();
            unsafe { digit.as_mut().as_bytes_mut() }.sort();

            for (value, d) in digit_map.iter().enumerate() {
                if *d == digit {
                    return acc * 10 + value;
                }
            }

            unreachable!()
        })
    })
}

fn main() {
    let mut args = std::env::args();

    let input_filename = args.nth(1).unwrap_or_else(|| {
        eprintln!("No input file specified");
        eprintln!("Usage: ./run <input filename>");
        std::process::exit(1);
    });

    let mut input_file = File::open(Path::new(&input_filename)).expect("failed to open input file");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("failed to read from file");

    let lines = input.lines()
        .map(|l| {
            let mut parts = l.split(" | ").map(|p| p.split(' ').collect::<Vec<_>>());

            let input_digits = parts.next().unwrap();
            let output_digits = parts.next().unwrap();

            (input_digits, output_digits)
        })
        .collect();

    println!("part 1: {}", easy_digits(&lines));
    println!("part 2: {}", full_sum(&lines));
}
