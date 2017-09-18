extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::thread::{JoinHandle, spawn};

use std::sync::{Arc,Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::mpsc::TryRecvError::Disconnected;

use std::collections::HashMap;

fn start_hashing_threads(salt: &str, cycles: usize) -> (Sender<usize>, Receiver<(usize, String)>) {
    let salt = salt.to_owned();
    let (in_send, in_recv) = channel();
    let in_recv = Arc::new(Mutex::new( in_recv ));
    let (out_send, out_recv) = channel();
    for _ in 0..7 {
        let in_recv = in_recv.clone();
        let out_send = out_send.clone();
        let salt = salt.clone();
        spawn(move || {
            loop {
                match in_recv.lock().unwrap().recv() {
                    Ok(i) => {
                        let mut digest = Md5::new();
                        let result = (0..cycles).fold(format!("{}{}", salt, i), |acc, _| {
                            digest.reset();
                            digest.input_str(&acc);
                            digest.result_str()
                        });
                        out_send.send( (i, result) );
                    },
                    Err(_) => break,
                }
            }
        });
    }
    (in_send, out_recv)
}

fn run(salt: &str, cycles: usize, desired_key: usize) -> usize {
    let (hash_in, hash_out) = start_hashing_threads(salt, cycles);
    let mut to_file = Vec::new();
    let mut filed = Vec::new();
    let mut range = 0..;
    for _ in 0..1001 {
        hash_in.send(range.next().unwrap());
    }
    let mut possible = 0;
    let mut otps = Vec::new();
    loop {
        hash_in.send(range.next().unwrap());
        to_file.push(hash_out.recv().unwrap());
        to_file.sort_by_key(|i| i.0);
        let f = to_file.drain(..).collect::<Vec<_>>();
        for (i, h) in f {
            if filed.len() == i {
                filed.push(h);
            } else {
                to_file.push( (i, h) );
            }
        }
        if filed.len() > 1000 {
            let ref p = filed[possible];
            if let Some(ch) = p.as_bytes().windows(3).filter_map(|w| if w[0] == w[1] && w[1] == w[2] { Some(w[0]) } else { None } ).next() {
                println!("possible {} is a triple", possible);
                for i in possible+1..possible+1001 {
                    if filed[i].as_bytes().windows(5).any(|w| w.iter().all(|&c| c == ch)) {
                        otps.push( possible );
                        println!("found otp #{}: {} at {}", otps.len(), p, possible);
                    }
                }
            }
            possible += 1;
        }
        if otps.len() == desired_key {
            return otps[desired_key - 1]
        }
    }
}

fn main() {
    let salt = "ahsbgdzn";
    println!("testing sample key 1");
    assert_eq!(run("abc", 1, 1), 39);
    println!("testing sample key 2");
    assert_eq!(run("abc", 1, 2), 92);
    println!("testing sample key 64");
    assert_eq!(run("abc", 1, 64), 22728);
    println!("part one: {}", run(salt, 1, 64));

    println!("testing sample key 1");
    assert_eq!(run("abc", 2017, 1), 10);
    println!("testing sample key 64");
    assert_eq!(run("abc", 2017, 64), 22551);

    println!("part two: {}", run(salt, 2017, 64));
}
