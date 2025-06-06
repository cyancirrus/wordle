use sqlite;
use std::fs::File;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{ self, BufRead, BufReader, Write };
use std::fmt::{ self, Display, Formatter};

# [derive(Eq, PartialEq, Debug)]
struct MinHeapNode<T>(T);

impl <T:Ord> Ord for MinHeapNode<T> {
    fn cmp(&self, other:&Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}


impl <T:Ord> PartialOrd for MinHeapNode<T> {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <T:Display> Display for MinHeapNode<T> {
    fn fmt(&self, f:&mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn process_word_bank() -> io::Result<usize> {
    let mut word_count = 0;
    let file = File::open("words/five.txt")?;
    let reader = BufReader::new(file);

    let mut dictionary:BinaryHeap<MinHeapNode<u32>> = BinaryHeap::new();
    for line in reader.lines() {
        let word = line?;
        if word.len() != 5 { continue; }
        if let Some(mask) =  convert(&word) {
            dictionary.push(MinHeapNode(mask));

        }
    }
    let mut out = File::create("words/bits.bm")?;
    let mut prev = 0;
    while let Some(mask) = dictionary.pop() {
        if mask.0 != prev {
            word_count += 1;
            writeln!(out, "{mask}")?;
        }
        prev = mask.0
    }
    Ok(word_count)
} 

pub fn convert(s:&str) -> Option<u32> {
    let mut bit_word = 0;
    for b in s.bytes() {
        let offset = b - b'a';
        let b = 1<< offset;
        if b & bit_word != 0 {
            return None;
        }
        bit_word |= b;
    }
    Some(bit_word)
}


fn main() {
    match process_word_bank() {
        Ok(n) => {
            println!("Success! Processed {} words", n);
        },
        Err(e) => {
            println!("Error found {:?}", e);
        }
    }
}
