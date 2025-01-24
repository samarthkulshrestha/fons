use std::env;
use rusqlite::{params, Connection, Result};
use inline_colorization::*;

#[derive(Debug)]
struct Word {
    _id: i32,
    word: String,
    pos: Option<String>,
    _hyphenation: Option<String>,
    ipa: Option<String>,
    _etym_num: Option<i32>,
    etym_txt_wiki: Option<String>,
    etym_txt_etym: Option<String>,
}

fn search(conn: &Connection, term: &str) -> Result<Word> {
    let mut recr = conn.prepare("SELECT id, word, pos, hyphenation, ipa,    \
        etym_num, etym_txt_wiki, etym_txt_etym FROM word WHERE word = ?1;")?;
    let word = recr.query_row(params![term], |r| {
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

    return Ok(word);
}

fn print_def(word: &Word) {
    print!("+ {color_bright_blue}{}{color_reset} ",
        word.word);
    match &word.pos {
        Some(s) => print!("({color_yellow}{}{color_reset}) ", s),
        None => ()
    }
    match &word.ipa {
        Some(s) => println!("[{color_cyan}{}{color_reset}]", s),
        None => ()
    }
    match &word.etym_txt_wiki {
        Some(s) => println!("+ {color_bright_magenta}etymology (wiktionary):{color_reset} \n    {}", s),
        None => println!()
    }
    match &word.etym_txt_etym {
        Some(s) => println!("+ {color_bright_magenta}etymology (etymonline):{color_reset} \n    {}", s),
        None => println!()
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let envvar_db_path = env::var("FONS_DB");
    let envvar_xdg_data = env::var("XDG_DATA_HOME");
    let envvar_home = env::var("HOME");
    let path;

    if envvar_db_path .is_ok() {
        path = envvar_db_path.unwrap();
    } else if envvar_xdg_data.is_ok() {
        path = envvar_xdg_data.unwrap() + "/fons/fons.db";
    } else if envvar_home.is_ok() {
        path = envvar_home.unwrap() + "/.local/share/fons/fons.db";
    } else {
        path = "~/.local/share/fons/fons.db".to_string();
    }

    let conn = Connection::open(path)?;

    if args.len() < 2 {
        eprintln!("ERROR: not enough arguments!");
    } else {
        let word = search(&conn, &args[1])?;
        print_def(&word);
    }

    Ok(())
}
