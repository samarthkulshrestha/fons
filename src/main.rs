use std::env;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Word {
    _id: i32,
    word: String,
    pos: String,
    hyphenation: Option<String>,
    ipa: Option<String>,
    _etym_num: Option<i32>,
    etym_txt: String,
}

fn search(conn: &Connection, term: &str) -> Result<Word> {
    let mut recr = conn.prepare("SELECT id, word, pos, hyphenation, ipa, etymology_number, etymology_text FROM word WHERE word = ?1;")?;
    let word = recr.query_row(params![term], |r| {
        Ok(Word {
            _id: r.get(0)?,
            word: r.get(1)?,
            pos: r.get(2)?,
            hyphenation: r.get(3)?,
            ipa: r.get(4)?,
            _etym_num: r.get(5)?,
            etym_txt: r.get(6)?,
        })
    })?;

    return Ok(word);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = "/home/evil/code/fons/data/enwiktionary-etym.db";
    let conn = Connection::open(path)?;

    if args.len() < 2 {
        eprintln!("ERROR: not enough arguments!");
    } else {
        let word = search(&conn, &args[1])?;
        println!("word: {}", word.word);
        println!("part of speech: {}", word.pos);
        println!("hyphenation: {:?}", word.hyphenation);
        println!("ipa: {:?}", word.ipa);
        println!("etymology: {}", word.etym_txt);
    }

    Ok(())
}
