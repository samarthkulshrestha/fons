use rusqlite::{Connection, Result};
use fons::*;
use std::env;

fn main() -> Result<()> {
    let path = cmd::get_db_path().unwrap();
    let conn = Connection::open(path)?;


    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        cmd::print_err("not enough arguments!");
        cmd::print_help();
    } else {
        let words = db::search(&conn, &args[1])?;
        cmd::print_def(&words[0]);
        // for w in words {
        //     print_def(&w);
        // }
    }


    Ok(())
}
