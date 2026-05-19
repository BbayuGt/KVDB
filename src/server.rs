use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::database::{self, Crud, Database};
use crate::parser::{self};

pub struct Server;


impl Server {
    pub fn start() {
        // Arc is used to share ownership across thread, Mutex is used to
        // prevent race condition by adding a queue system when data is being used
        let db = Arc::new(Mutex::new(database::Database::new()));
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connected");
            let cloned_db = Arc::clone(&db);

            thread::spawn(|| handle_connection(stream, cloned_db));
        }
    }
}

/// Helper reply so I can easily send back text to client
fn reply(stream: &mut TcpStream, text: &str) {
    stream.write(text.as_bytes()).unwrap();
}

/// Main loop
fn handle_connection(stream: TcpStream, arc_db: Arc<Mutex<Database>>) {
    loop {
        let mut stre = stream.try_clone().unwrap();
        reply(&mut stre, "$ ");
        let mut text = String::new();
        let mut buf_reader = BufReader::new(&stream);
        let _ = buf_reader
            .read_line(&mut text);

        let text_stripped = text.trim();
        if text_stripped == "" {
            println!("No data provided, closing");
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            break
        }

        let parsed = parser::parse(String::from(text_stripped));

        if parsed.is_err() {
            println!("{}", parsed.unwrap_err());
        } else {
            let mut db = arc_db.lock().unwrap();
            let act = parsed.unwrap();
            let value = db.get(&act.key);

            match act.action {
                parser::ActionsType::GET => {       
                    if value.is_none() {
                        reply(&mut stre, "Key tidak valid\n");
                    }
                    else {
                        reply(&mut stre, format_args!("result: {}\n", value.unwrap() ).to_string().as_str() );
                    }
                },
                parser::ActionsType::SET => {
                    let act_value = act.value;
                    if act_value.clone().is_none_or(|e| e == "") {
                        reply(&mut stre, "Berikan value\n");
                    }
                    else {
                        let result = db.set(&act.key, &act_value.unwrap());
                        if result {
                            reply(&mut stre, "Success!\n");
                        } else {
                            reply(&mut stre, "Failed!\n");
                        }
                    }
                },
                parser::ActionsType::UPDATE => {
                    let act_value = act.value;
                    if act_value.clone().is_none_or(|e| e == "") {
                        reply(&mut stre, "Value tidak valid!\n");
                    }
                    else {
                        let result = db.update(&act.key, &act_value.unwrap());
                        if result {
                            reply(&mut stre, "Success!\n");
                        } else {
                            reply(&mut stre, "Failed!\n");
                        }
                    }
                },
                parser::ActionsType::DELETE => {
                    if value.is_none() {
                        reply(&mut stre, "Value tidak valid!\n");
                    }
                    else {
                        let result = db.delete(&act.key);
                        if result {
                            reply(&mut stre, "Success!\n");
                        } else {
                            reply(&mut stre, "Failed!\n");
                        }
                    }
                }
            }
        }
    }
    
}


