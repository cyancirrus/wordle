#![allow(dead_code)]
use rayon::prelude::*;
use rusqlite;
use std::time::Instant;
use std::collections::HashMap;

type RSqlConn = rusqlite::Connection;

fn retrieve_neighs(conn:&RSqlConn) -> HashMap<u32, Vec<u32>> {
    let mut stmt = conn.prepare("select x, y from d_neighs").unwrap();
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut rows = stmt.query([]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        let x:u32 = row.get(0).unwrap();
        let y:u32 = row.get(1).unwrap();
        map.entry(x).or_default().push(y);
    }
    map
}

fn retrieve_bitmasks(conn:&RSqlConn) -> rusqlite::Result<Vec<u32>> {
    let mut stmt = conn.prepare("select distinct bitmask from d_words")?;
    let word_iter = stmt.query_map([], |row| {
        row.get::<_,u32>(0)
    })?;
    Ok(
        word_iter.collect::<rusqlite::Result<Vec<u32>>>()?
    )
}

fn retrieve_word(mem:u32, conn:&RSqlConn) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare("select word from d_words where bitmask = $1")?;
    let word_iter = stmt.query_map([mem], |row| {
        row.get::<_,String>(0)
    })?;
    Ok(
        word_iter.collect::<rusqlite::Result<Vec<String>>>()?
    )
}

fn find_collections(conn:&RSqlConn) {
    let results = search_backtrack(&conn);

    println!("Found the following");
    for f in results {
        println!("-------------------");
        for m in f {
            let w = retrieve_word(m, conn);
            println!("[{:?}]", w);
        }
        println!("-------------------");
    }
    println!("Done!");
}


fn search_backtrack(conn: &RSqlConn) -> Vec<Vec<u32>> {
    let words = retrieve_bitmasks(&conn).unwrap();
    let neighs = retrieve_neighs(&conn);
    words.par_iter().enumerate().flat_map(|(pos, &b)| {
        let conn = RSqlConn::open("dev.db").unwrap();
        let pos = pos as u32;
        let mut local_found = vec![];
        backtrack(
            pos,
            b,
            b,
            &mut vec![b],
            &mut local_found,
            &neighs,
            &conn,
        );
        local_found
    }).collect()
}


// fn search_backtrack(conn:&RSqlConn) -> Vec<Vec<u32>> {
//     let mut found:Vec<Vec<u32>> = vec![];
//     let words = retrieve_bitmasks(&conn).unwrap();
//     let neighs =  retrieve_neighs(&conn);
//     for (pos, &b) in words.iter().enumerate() {
//         let pos = pos as u32;
//         backtrack(
//             pos,
//             b,
//             b,
//             &mut vec![b],
//             &mut found, 
//             &neighs,
//             conn,
//         )
//     }
//     found
// }

fn backtrack(
    pos:u32,
    bm:u32,
    curr:u32,
    members:&mut Vec<u32>,
    found:&mut Vec<Vec<u32>>,
    neighs:&HashMap<u32, Vec<u32>>,
    conn:&RSqlConn
) {
    if members.len() == 5 {
        found.push(members.to_vec());
        return;
    }
    for &n in &neighs[&curr] {
        if n > curr && n & bm == 0 {
            members.push(n);
            backtrack(
                pos,
                bm ^ n, 
                n,
                members,
                found,
                neighs,
                conn
            );
            members.pop();
        }
    }
}

fn main() {
    // use wordle::initialize;
    // let _ = initialize::create_data();


    // let disk_conn = RSqlConn::open("dev.db")?;
    // let mem_conn = RSqlConn::open_in_memory()?;
    // disk_conn.backup(rusqlite::DatabaseName::Main, &mem_conn, None)?;
    // // _ = find_collections(&mem_conn);


    let conn = RSqlConn::open("dev.db").unwrap();
    let start = Instant::now();
    _ = find_collections(&conn);
    println!("Took {:?}", start.elapsed());
}
