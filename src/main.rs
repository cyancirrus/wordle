use sqlite;
use std::fs::File;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{ self, BufRead, BufReader, Write };
use std::fmt::{ self, Display, Formatter};

// # [derive(Eq, PartialEq, Debug)]
// struct MinHeapNode<T>(T);
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

#[derive(Eq, PartialEq, Debug)]
struct WordHeapNode<T> {
    bitmask:T,
    word:String,
}

impl <T:Ord> Ord for WordHeapNode<T> {
    fn cmp(&self, other:&Self) -> Ordering {
        other.bitmask.cmp(&self.bitmask)
    }
}

impl <T:Ord> PartialOrd for WordHeapNode<T> {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <T:Display> Display for WordHeapNode<T> {
    fn fmt(&self, f:&mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bitmask)
    }
}

pub fn load_word_database() -> io::Result<()> {
    let mut success = 0;
    let mut failure = 0;
    let file = File::open("words/five.txt")?;
    let connection = sqlite::open("words.db").unwrap();
    let mut dictionary:BinaryHeap<WordHeapNode<u32>> = BinaryHeap::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let word = line?;
        if word.len() != 5 { continue; }
        if let Some(bitmask) =  convert(&word) {
            dictionary.push(WordHeapNode { bitmask, word } );
        }
    }

    let create_query = " create table d_words (bitmask INTEGER, word TEXT); ";
    let insert_query = " insert into d_words values (?, ?); ";

    match connection.execute(create_query) {
        Ok(tcreate) => println!("Successfully created db {:?}",tcreate),
        Err(e) => println!("Error cannot create db {:?}", e),
    }
    
    while let Some(node) = dictionary.pop() {
        let mut statement = {
                connection.prepare(insert_query)
                .unwrap()
                .into_iter()
                .bind((1, node.bitmask as i64 ))
                .unwrap()
                .bind((2, node.word.as_str()))
                .unwrap()
        };

        match statement.next() {
            Some(Ok(_)) => success += 1,
            _ => failure += 1,
        }
    }
    println!("Dimension words completed {{ success : {}, failures: {} }}", success,failure);
    Ok(())
}

pub fn process_word_bank() -> io::Result<usize> {
    let mut word_count = 0;
    let file = File::open("words/five.txt")?;
    let reader = BufReader::new(file);

    let mut dictionary:BinaryHeap<WordHeapNode<u32>> = BinaryHeap::new();
    for line in reader.lines() {
        let word = line?;
        if word.len() != 5 { continue; }
        if let Some(bitmask) =  convert(&word) {
            dictionary.push(WordHeapNode { bitmask, word } );

        }
    }
    let mut out = File::create("words/bits.bm")?;
    let mut prev = 0;
    while let Some(mask) = dictionary.pop() {
        if mask.bitmask != prev {
            word_count += 1;
            writeln!(out, "{mask}")?;
        }
        prev = mask.bitmask
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
    match load_word_database() {
        Ok(_) => {
            println!("Success! Loaded all words");
        },
        Err(e) => {
            println!("Error found {:?}", e);
        }
    }
}
