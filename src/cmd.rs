use crate::db::Word;
use inline_colorization::*;
use std::env;
use anyhow::Result;

pub fn get_db_path() -> Result<String> {
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

    Ok(path)
}

pub fn print_def(word: &Word) {
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

pub fn print_help() {
    println!("{color_blue}USAGE:{color_reset} fons <search term>")
}

pub fn print_err(err: &str) {
    eprintln!("{color_red}ERROR:{color_reset} {}", err)
}
