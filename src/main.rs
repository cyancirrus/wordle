#![allow(dead_code)]
use rusqlite;
use std::time::Instant;

type RSqlConn = rusqlite::Connection;

fn retrieve_neighs(bm:u32, pos:u32, conn:&RSqlConn) -> rusqlite::Result<Vec<u32>> {
    let mut stmt = conn.prepare("select y from d_neighs where x = $1 and y > $2")?;
    // let mut stmt = conn.prepare("select y from d_neighs where x = $1 and y > $2 order by y")?;
    let neigh_iter = stmt.query_map([bm, pos], |row| {
        row.get::<_,u32>(0)
    })?;
    Ok(
        neigh_iter.collect::<rusqlite::Result<Vec<u32>>>()?
    )
}

fn retrieve_neighs_cached(
    bm: u32,
    pos: u32,
    stmt: &mut rusqlite::Statement<'_>
) -> rusqlite::Result<Vec<u32>> {
    let neigh_iter = stmt.query_map([bm, pos], |row| row.get(0))?;
    neigh_iter.collect()
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
    let mut stmt = conn.prepare("select word from d_words where word = $1")?;
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

fn search_backtrack(conn:&RSqlConn) -> Vec<Vec<u32>> {
    let mut found:Vec<Vec<u32>> = vec![];
    let words = retrieve_bitmasks(&conn).unwrap();
    let mut start = Instant::now(); 
    let mut stmt = conn.prepare("select y from d_neighs where x = ?1 and y > ?2").unwrap();
    for (pos, &b) in words.iter().enumerate() {
        if pos % 100 == 0 && pos > 0 {
            println!("Searching [{}:{}] took {:?}", pos - 100, pos, start.elapsed());
            start = Instant::now();
        }

        let pos = pos as u32;
        // let mut available = retrieve_neighs(b, pos, conn).unwrap();
        let mut available = retrieve_neighs_cached(b, pos as u32, &mut stmt).unwrap();
        backtrack(
            pos,
            b,
            &mut vec![b],
            &mut found, 
            &mut available,
            conn,
        )
    }
    found
}

fn backtrack(pos:u32, curr:u32, members:&mut Vec<u32>, found:&mut Vec<Vec<u32>>, available:&mut Vec<u32>, conn:&RSqlConn) {
    if members.len() == 5 {
        found.push(members.to_vec());
        return;
    }

    for n in retrieve_neighs(curr, pos, conn).unwrap() {
        if n > curr && n & curr == 0 {
            members.push(n);
            let mut intersection = sort_intersect(available, &mut retrieve_neighs(n, pos, conn).unwrap());
            backtrack(
                pos,
                curr ^ n, 
                members,
                found,
                &mut intersection,
                conn
            );
            members.pop();
        }
    }
}

fn sort_intersect(base:&Vec<u32>, other:&Vec<u32>) -> Vec<u32> {
    let mut i = 0;
    let mut j = 0;
    let mut inter = Vec::new();
    while i < base.len() && j < other.len(){
        let l = base[i];
        let r = other[j];
        if l == r {
            inter.push(base[i]);
            i+=1;
            j+=1;
        } else if l < r {
            i+=1;
        } else {
            j+=1;
        }
    }
    inter
}

fn main() {
    // use wordle::initialize;
    // let _ = initialize::create_data();
    let conn = RSqlConn::open("dev.db").unwrap();
    _ = find_collections(&conn);

}
