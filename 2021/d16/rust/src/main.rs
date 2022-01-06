use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

use std::collections::VecDeque;

struct Bits {
    data: VecDeque<u8>,
    current: u8,
    cursor: u8,
}

impl Bits {
    fn from_str(s: &str) -> Self {
        let mut chars = s.chars();
        let mut data = VecDeque::new();

        loop {
            match (chars.next(), chars.next()) {
                // unwrap: assume all data coming in is valid hex
                (Some(a), Some(b)) => data.push_back(((a.to_digit(16).unwrap() << 4) | b.to_digit(16).unwrap()) as u8),
                (None, None) => break,
                _ => panic!("an incomplete byte was included in the message"),
            }
        }

        // unwrap: assume there's at least some data
        let current = data.pop_front().unwrap();

        Bits {
            data,
            current,
            cursor: 0,
        }
    }

    fn from_bytes<T: Into<VecDeque<u8>>>(b: T) -> Self {
        let mut data = b.into();
        let current = data.pop_front().unwrap();

        Bits {
            data,
            current,
            cursor: 0,
        }
    }
}

struct Counter {
    current: usize,
    limit: usize,
}

impl Counter {
    fn new(limit: usize) -> Self {
        Counter {
            current: 0,
            limit,
        }
    }

    fn expired(&self) -> bool {
        self.current >= self.limit
    }
}

struct Deserializer {
    bits: Bits,
    counter: Option<Counter>,
}

const MAX_BITS: u8 = std::mem::size_of::<usize>() as u8 * 8;

impl Deserializer {
    fn from_str(s: &str) -> Self {
        Deserializer {
            bits: Bits::from_str(s),
            counter: None
        }
    }

    fn from_bytes(b: Vec<u8>) -> Self {
        Deserializer {
            bits: Bits::from_bytes(b),
            counter: None
        }
    }

    fn start_counter(&mut self, limit: usize) {
        self.counter = Some(Counter::new(limit));
    }

    // panic: can only be called if start_counter has already been called.
    fn counter_expired(&self) -> bool {
        self.counter.as_ref().map(|c| c.expired()).expect("no counter started")
    }

    fn read_bits(&mut self, mut len: u8) -> usize {
        if len > MAX_BITS {
            panic!("requested {} bits, but only {} fit in a usize", len, MAX_BITS);
        }

        let mut out = 0;

        while len > 0 {
            if self.bits.cursor == 8 {
                self.bits.cursor = 0;
                self.bits.current = self.bits.data.pop_front().expect("read past eof");
            }

            let read_len = (8 - self.bits.cursor).min(len);
            out <<= read_len;
            let read_mask = if read_len == 8 { 0xFFu8 } else { (1u8 << read_len) - 1 };
            out |= ((self.bits.current >> (8 - self.bits.cursor - read_len)) & read_mask) as usize;

            self.bits.cursor += read_len;
            len -= read_len;

            if let Some(ref mut counter) = self.counter {
                counter.current += read_len as usize;
            }
        }

        out
    }

    fn read_to_bytes(&mut self, mut num_bits: usize) -> Vec<u8> {
        let mut bytes = Vec::new();

        while num_bits > 0 {
            if num_bits > 8 {
                // cast: safe because we're only reading 8 bits which by definition fits in a u8.
                bytes.push(self.read_bits(8) as u8);
                num_bits -= 8;
            } else {
                // cast: safe because we know num_bits is less than 8 and therefore fits in a u8.
                // cast: safe because we're only reading less than 8 bits which by definition fits in a u8.
                bytes.push((self.read_bits(num_bits as u8) << 8 - num_bits) as u8);
                num_bits = 0;
            }
        }

        bytes
    }

    fn deserialize_number(&mut self) -> usize {
        let mut out = 0;

        loop {
            let should_continue = self.read_bits(1) == 1;

            out <<= 4;
            out |= self.read_bits(4);

            if !should_continue { break }
        }

        out
    }

    fn deserialize_operator_length(&mut self) -> Vec<Packet> where Self: Sized {
        let content_bits = self.read_bits(15);

        let mut packets = Vec::new();

        let bytes = self.read_to_bytes(content_bits);
        let mut new_de = Deserializer::from_bytes(bytes);
        new_de.start_counter(content_bits);

        while !new_de.counter_expired() {
            packets.push(Packet::deserialize(&mut new_de));
        }

        packets
    }

    fn deserialize_operator_count(&mut self) -> Vec<Packet> where Self: Sized {
        let count = self.read_bits(11);

        let mut packets = Vec::with_capacity(count);

        for _ in 0..count {
            packets.push(Packet::deserialize(self));
        }

        packets
    }
}

#[derive(Debug)]
struct Packet {
    version: usize,
    ty: PacketType,
}

impl Packet {
    fn deserialize(de: &mut Deserializer) -> Self {
        let version = de.read_bits(3);
        let ty = PacketType::deserialize(de);

        Packet { version, ty }
    }

    fn sum_versions(&self) -> usize {
        self.version + self.ty.sum_versions()
    }

    fn value(&self) -> usize {
        self.ty.value()
    }
}

#[derive(Debug)]
enum PacketType {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(usize),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

impl PacketType {
    fn deserialize(de: &mut Deserializer) -> Self {
        match de.read_bits(3) {
            4 => PacketType::Literal(de.deserialize_number()),
            op => {
                let packets = match de.read_bits(1) {
                    0 => de.deserialize_operator_length(),
                    1 => de.deserialize_operator_count(),
                    _ => unreachable!(),
                };

                match op {
                    0 => PacketType::Sum(packets),
                    1 => PacketType::Product(packets),
                    2 => PacketType::Minimum(packets),
                    3 => PacketType::Maximum(packets),
                    5 => PacketType::GreaterThan(packets),
                    6 => PacketType::LessThan(packets),
                    7 => PacketType::EqualTo(packets),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn sum_versions(&self) -> usize {
        match self {
            PacketType::Sum(packets) => packets.iter().map(|p| p.sum_versions()).sum(),
            PacketType::Product(packets) => packets.iter().map(|p| p.sum_versions()).sum(),
            PacketType::Minimum(packets) => packets.iter().map(|p| p.sum_versions()).sum(),
            PacketType::Maximum(packets) => packets.iter().map(|p| p.sum_versions()).sum(),
            PacketType::Literal(_) => 0,
            PacketType::GreaterThan(packets) => packets.iter().map(|p| p.sum_versions()).sum(),
            PacketType::LessThan(packets) => packets.iter().map(|p| p.sum_versions()).sum(),
            PacketType::EqualTo(packets) => packets.iter().map(|p| p.sum_versions()).sum(),
        }
    }

    fn value(&self) -> usize {
        match self {
            PacketType::Sum(packets) => packets.iter().map(|p| p.value()).sum(),
            PacketType::Product(packets) => packets.iter().map(|p| p.value()).product(),
            PacketType::Minimum(packets) => packets.iter().map(|p| p.value()).min().unwrap(),
            PacketType::Maximum(packets) => packets.iter().map(|p| p.value()).max().unwrap(),
            PacketType::Literal(v) => *v,
            PacketType::GreaterThan(packets) => if packets[0].value() > packets[1].value() { 1 } else { 0 },
            PacketType::LessThan(packets) => if packets[0].value() < packets[1].value() { 1 } else { 0 },
            PacketType::EqualTo(packets) => if packets[0].value() == packets[1].value() { 1 } else { 0 },
        }
    }
}

trait Operator {
    fn calculate(values: Vec<Packet>) -> usize;
}

struct Nop;
impl Operator for Nop {
    fn calculate(_values: Vec<Packet>) -> usize {
        unreachable!("this type of operator is a placeholder that should never be called")
    }
}

fn versions(input: &str) -> usize {
    let mut d = Deserializer::from_str(input);
    let p = Packet::deserialize(&mut d);
    dbg!(&p);

    p.sum_versions()
}

fn value(input: &str) -> usize {
    let mut d = Deserializer::from_str(input);
    let p = Packet::deserialize(&mut d);
    dbg!(&p);

    p.value()
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
    let input = input.trim();

    let now = Instant::now();
    println!("part 1: {}", versions(&input));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);

    let now = Instant::now();
    println!("part 2: {}", value(&input));
    println!("time: {} ms", now.elapsed().as_nanos() as f64 / 1000000000f64);
}

#[cfg(test)]
mod tests {
    use super::*;

    // PART 1

    #[test]
    fn p1t1() {
        let data = "8A004A801A8002F478";
        assert_eq!(versions(data), 16);
    }

    #[test]
    fn p1t2() {
        let data = "620080001611562C8802118E34";
        assert_eq!(versions(data), 12);
    }

    #[test]
    fn p1t3() {
        let data = "C0015000016115A2E0802F182340";
        assert_eq!(versions(data), 23);
    }

    #[test]
    fn p1t4() {
        let data = "A0016C880162017C3686B18A3D4780";
        assert_eq!(versions(data), 31);
    }

    // PART 2

    #[test]
    fn p2t1() {
        let data = "C200B40A82";
        assert_eq!(value(data), 3);
    }

    #[test]
    fn p2t2() {
        let data = "04005AC33890";
        assert_eq!(value(data), 54);
    }

    #[test]
    fn p2t3() {
        let data = "880086C3E88112";
        assert_eq!(value(data), 7);
    }

    #[test]
    fn p2t4() {
        let data = "CE00C43D881120";
        assert_eq!(value(data), 9);
    }

    #[test]
    fn p2t5() {
        let data = "D8005AC2A8F0";
        assert_eq!(value(data), 1);
    }

    #[test]
    fn p2t6() {
        let data = "F600BC2D8F";
        assert_eq!(value(data), 0);
    }

    #[test]
    fn p2t7() {
        let data = "9C005AC2F8F0";
        assert_eq!(value(data), 0);
    }

    #[test]
    fn p2t8() {
        let data = "9C0141080250320F1802104A08";
        assert_eq!(value(data), 1);
    }
}
