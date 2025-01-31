use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Word {
    pub _id: i32,
    pub word: String,
    pub pos: Option<String>,
    pub _hyphenation: Option<String>,
    pub ipa: Option<String>,
    pub _etym_num: Option<i32>,
    pub etym_txt_wiki: Option<String>,
    pub etym_txt_etym: Option<String>,
}

pub fn search(conn: &Connection, term: &str) -> Result<Vec<Word>> {
    let mut words: Vec<Word> = vec!();
    let mut recr = conn.prepare("SELECT DISTINCT id, word, pos,             \
        hyphenation, ipa, etym_num, etym_txt_wiki, etym_txt_etym FROM word  \
        WHERE word = ?1;")?;
    let rows = recr.query_map(params![term], |r| {
        Ok(Word {
            _id: r.get(0)?,
            word: r.get(1)?,
            pos: r.get(2)?,
            _hyphenation: r.get(3)?,
            ipa: r.get(4)?,
            _etym_num: r.get(5)?,
            etym_txt_wiki: r.get(6)?,
            etym_txt_etym: r.get(7)?,
        })
    })?;

    for word in rows {
        words.push(word.unwrap());
    }

    return Ok(words);
}
