#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate postgres;
extern crate dotenv;

use std::io;
use std::path::{Path, PathBuf};
use std::env;
use dotenv::dotenv;
use rocket::response::NamedFile;
use postgres::{Connection, TlsMode};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("../frontend/build/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("../frontend/build").join(file)).ok()
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL");

    println!("Connecting to {}", database_url);

    let pg = Connection::connect(database_url, TlsMode::None).unwrap();

    // rocket::ignite()
    //     .manage(pg)
    //     .mount("/", routes![index, files])
    //     .launch();
}




// struct Person {
//     id: i32,
//     name: String,
//     data: Option<Vec<u8>>
// }
// 
//     for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
//         let person = Person {
//             id: row.get(0),
//             name: row.get(1),
//             data: row.get(2)
//         };
//         println!("Found person {}", person.name);
//     }
// 
