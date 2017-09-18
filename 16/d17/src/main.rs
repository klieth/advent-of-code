extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let input = "qljzarfv";
    assert_eq!(part_one("ihgpwlah"), "DDRRRD");
    assert_eq!(part_one("kglvqrro"), "DDUDRLRRUDRD");
    assert_eq!(part_one("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    println!("")
}
