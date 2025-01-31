use tiny_http::{Header, Method, Response, Server};
use rusqlite::Connection;
use crate::db;
// use std::fs::File;
// use std::path::Path;

pub fn launch(conn: &Connection) {
    let srv_ip = "0.0.0.0";
    let srv_port = "8000";
    let srv = Server::http(format!("{srv_ip}:{srv_port}")).unwrap();
    println!("INFO: launching web server on {srv_ip}:{srv_port}");

    for req in srv.incoming_requests() {
        if *req.method() == Method::Get {
            // if req.url() == "/" {
            //     let content_type_header = Header::from_bytes("Content-Type", "text/html; charset=utf-8").unwrap();
            //     let res = Response::from_file(File::open(&Path::new("src/index.html")).unwrap()).with_header(content_type_header);
            //     let _ = req.respond(res);
            // } else if req.url() == "/index.js" {
            //     let content_type_header = Header::from_bytes("Content-Type", "text/javascript; charset=utf-8").unwrap();
            //     let res = Response::from_file(File::open(&Path::new("src/index.js")).unwrap()).with_header(content_type_header);
            //     let _ = req.respond(res);
            // } else {
                let search_term = &req.url()[1..];
                // println!("{:?}\nlen: {:?}", &req.headers(), &req.body_length());
                let words = db::search(&conn, search_term).unwrap();
                // [0].clone();
                if words.len() > 0 {
                    // let word = words[0].clone();
                    let json = serde_json::to_string(&words).unwrap();
                    let content_type_header = Header::from_bytes("Content-Type", "application/json; charset=utf-8").unwrap();
                    let res = Response::from_string(json).with_header(content_type_header);
                    let _ = req.respond(res);
                }
            // }
        }
    }
}
