use rusqlite;
use std::fs::File;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{ self, BufRead, BufReader, Write };
use std::fmt::{ self, Display, Formatter};

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
    let file = File::open("words/five.txt")?;
    let connection = rusqlite::Connection::open("dev.db").unwrap();
    let mut dictionary:BinaryHeap<WordHeapNode<u32>> = BinaryHeap::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let word = line?;
        if word.len() != 5 { continue; }
        if let Some(bitmask) =  convert(&word) {
            dictionary.push(WordHeapNode { bitmask, word } );
        }
    }

    let create_query = " create table d_words (bitmask INTEGER, word TEXT PRIMARY_KEY); ";
    let mut insert_query = String::from(" insert into d_words values");
    let create_index = "create index idx_bitmask_words on d_words(bitmask); ";

    match connection.execute(create_query, ()) {
        Ok(tcreate) => println!("Successfully created db {:?}",tcreate),
        Err(e) => println!("Error cannot create db {:?}", e),
    }
    while let Some(node) = dictionary.pop() {
        let vals = format!("({}, '{}'),", node.bitmask, &node.word);
        insert_query.push_str(&vals);

    }
    insert_query.pop();
    insert_query.push_str(";");
    match connection.execute( &insert_query[0..insert_query.len()-1], () ){
        Ok(_) => println!("!! DimWords successfully created table inserted all words"),
        Err(e) => println!("DimWords failed with error {:?}", e),
    }
    match connection.execute(create_index, ()) {
        Ok(tcreate) => println!("!! DimWords successfully created index {:?}", tcreate),
        Err(e) => println!("DimWords Error for index {:?}", e),
    }
    Ok(())
}

pub fn load_word_neighbors() -> io::Result<usize> {
    let connection = rusqlite::Connection::open("dev.db").unwrap();
    let file = File::open("words/bits.bm")?;
    let bitmasks: Vec<u32> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .collect()
    ;
    let create_query = " create table d_neighs (x INTEGER, y INTEGER); ";
    let create_index = "create index idx_x_y on d_neighs(x, y); ";
    // let create_range= "create index idx_bitmask_neigh_range on d_neighs(y); ";
    // let create_range= "create index idx_bitmask_neigh_range on d_neighs(y); ";
    let mut insert_query = String::from(" insert into d_neighs values");

    for i in 0..bitmasks.len() {
        for j in 0..bitmasks.len() {
            if bitmasks[i] & bitmasks[j] == 0 {
                let vals = format!("({}, {}),", bitmasks[i], bitmasks[j]);
                insert_query.push_str(&vals);
            }
        }
    }
    insert_query.pop();
    insert_query.push_str(";");
    
    match connection.execute(create_query, ()) {
        Ok(_) => println!("Successfully created neighbors"),
        Err(e) => println!("Error for create neighbors {:?}", e),
    };
    match connection.execute(&insert_query, ()) {
        Ok(_) => println!("Successfully created neighbors"),
        Err(e) => println!("Error for create neighbors {:?}", e),
    }
    match connection.execute(create_index, ()) {
        Ok(tcreate) => println!("!! DimNeighs successfully created index {:?}", tcreate),
        Err(e) => println!("DimNeighs Error for index {:?}", e),
    }
    // match connection.execute(create_index, ()) {
    //     Ok(tcreate) => println!("!! DimNeighs successfully created index {:?}", tcreate),
    //     Err(e) => println!("DimNeighs Error for index {:?}", e),
    // }
    // match connection.execute(create_range, ()) {
    //     Ok(tcreate) => println!("!! DimNeighs successfully created range {:?}", tcreate),
    //     Err(e) => println!("DimNeighs Error for range {:?}", e),
    // }
    Ok(0)
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

pub fn create_data() {
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
    match load_word_neighbors() {
        Ok(_) => {
            println!("Success! Loaded all words");
        },
        Err(e) => {
            println!("Error found {:?}", e);
        }
    }
}
