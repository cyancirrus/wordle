# Wordle

Wordle is a Rust project focused on exploring the challenge of finding sets of **5 unique English words** that collectively use **25 unique letters** (almost the entire alphabet). The project leverages bitmask representations of words, graph-based neighbor relations, and efficient backtracking to identify these word sets.

## Features

- Uses a precomputed word bank stored in an SQLite database (`dev.db`)
- Each word is represented as a bitmask of letters for fast set operations
- Graph representation of "neighbors" to limit backtracking search space
- Parallelized search using Rayon for faster exploration
- Balances SQLite usage between on-disk and in-memory databases for performance

## How It Works

1. **Data Storage**:  
   The words and their bitmasks, as well as neighbor relationships between bitmask sets, are stored in an SQLite database.  
   
2. **Bitmask Representation**:  
   Each word is represented as a 32-bit integer where bits correspond to letters used in the word, allowing fast checks for letter overlaps.

3. **Graph of Neighbors**:  
   A graph is constructed where each node is a bitmask representing a word, and edges indicate compatible neighbors (words without letter overlap).

4. **Backtracking Search**:  
   The algorithm performs a parallelized backtracking search on this graph to find 5-word sets where all letters combined cover 25 unique characters with no overlaps.

5. **Retrieving Words**:  
   Once a candidate set of bitmasks is found, the corresponding English words are retrieved from the database for display.

## Usage

Run the project by ensuring you have the `dev.db` SQLite database in place with the required tables (`d_words`, `d_neighs`). Then execute:

```bash
cargo run --release

