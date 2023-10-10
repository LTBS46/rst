use std::{
    io::stdin,
    sync::{Arc, Mutex},
};

pub mod basic;
use crate::basic::{BasicServer, BasicWorld};

use engine::{Server, ServerHolder, World};

pub fn main() {
    println!("---------------------------------------------");
    let mut m = ServerHolder::new(BasicServer::new(None));
    let w: Arc<Mutex<(dyn World + 'static)>> = BasicWorld::new("myworld".to_string());
    m.auto_register(Arc::downgrade(&w));
    w.lock().unwrap().start();
    loop {
        let mut line_in = String::new();
        match stdin().read_line(&mut line_in) {
            Ok(_v) => match &(line_in)[..line_in.len() - 2] {
                "" => {}
                "exit" | "quit" | "q" => {
                    return;
                }
                _ => println!("got {}", line_in),
            },
            Err(e) => println!("{}", e),
        }
    }
}
