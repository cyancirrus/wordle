use rusqlite;
use std::collections::HashMap;
use std::collections::HashSet;

type RSqlConn = rusqlite::Connection;

fn retrieve_neighs(bm:u32, conn:&RSqlConn) -> rusqlite::Result<Vec<u32>> {
    let mut stmt = conn.prepare("select y from d_neighs where x = $1 order by y")?;
    let neigh_iter = stmt.query_map([bm], |row| {
        row.get::<_,u32>(0)
    })?;
    Ok(
        neigh_iter.collect::<rusqlite::Result<Vec<u32>>>()?
    )
}

fn retrieve_bitmasks(conn:&RSqlConn) -> rusqlite::Result<Vec<u32>> {
    let mut stmt = conn.prepare("select bitmask from d_words")?;
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
    // let mut neigh_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut neigh_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for b in &words {
        // let neighs = retrieve_neighs(*b, conn).unwrap();
        let neighs = retrieve_neighs(*b, conn).unwrap().into_iter().collect::<HashSet<_>>();
        neigh_map.insert(*b, neighs);
    }
    for b in words {
        println!("Searching {:?}", b);
        let mut available = neigh_map[&b].clone();
        backtrack(
            b,
            &mut vec![b],
            &mut found, 
            &mut available,
            &neigh_map,
        )
    }
    found
}

fn backtrack(
    curr:u32,
    members:&mut Vec<u32>,
    found:&mut Vec<Vec<u32>>,
    // available:&mut Vec<u32>,
    available:&mut HashSet<u32>,
    // neigh_map: &HashMap<u32, Vec<u32>>
    neigh_map: &HashMap<u32, HashSet<u32>>
) {
    if members.len() == 5 {
        found.push(members.to_vec());
        return;
    }
    for n in &neigh_map[&curr] {
        if n & curr == 0 {
            members.push(*n);
            // let mut intersection = sort_intersect(available, &neigh_map[&n]);
            let mut intersection = available.intersection(&neigh_map[&n]).cloned().collect::<HashSet<u32>>();
            backtrack(
                *n, 
                members,
                found,
                &mut intersection,
               &neigh_map, 
            );
            members.pop();
        }
    }
}

// fn sort_intersect(base:&Vec<u32>, other:&Vec<u32>) -> Vec<u32> {
//     let mut i = 0;
//     let mut j = 0;
//     let mut inter = Vec::new();
//     while i < base.len() && j < other.len(){
//         let l = base[i];
//         let r = other[j];
//         if l == r {
//             inter.push(base[i]);
//             i+=1;
//             j+=1;
//         } else if l < r {
//             i+=1;
//         } else {
//             j+=1;
//         }
//     }
//     inter
// }

fn main() {
    // use wordle::initialize;
    // let _ = initialize::create_data();
    let conn = RSqlConn::open("dev.db").unwrap();
    _ = find_collections(&conn);

}
