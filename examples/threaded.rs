/// A thread-based client + server example. It also demonstrates using a struct as a WebSocket
/// handler to implement more handler methods than a closure handler allows.

extern crate ws;
extern crate url;
extern crate env_logger;

use std::thread;
use std::thread::sleep_ms;

use ws::{connect, listen, CloseCode, Sender, Handler, Message, Result};

fn main () {

    // Setup logging
    env_logger::init().unwrap();

    // Server WebSocket handler
    struct Server {
        out: Sender,
    }

    impl Handler for Server {

        fn on_message(&mut self, msg: Message) -> Result<()> {
            println!("Server got message '{}'. ", msg);
            self.out.send(msg)
        }

        fn on_close(&mut self, code: CloseCode, reason: &str) {
            println!("WebSocket closing for ({:?}) {}", code, reason);
            println!("Shutting down server after first connection.");
            self.out.shutdown().unwrap();
        }
    }

    // Server thread
    let server = thread::spawn(move || {
        listen("127.0.0.1:3012", |out| {

            Server { out: out }

        }).unwrap()
    });

    // Give the server a little time to get going
    sleep_ms(10);

    // Client thread
    let client = thread::spawn(move || {

        let url = url::Url::parse("ws://127.0.0.1:3012").unwrap();

        connect(url, |out| {

            out.send("Hello WebSocket").unwrap();

            move |msg| {
                println!("Client got message '{}'. ", msg);
                out.close(CloseCode::Normal)
            }

        }).unwrap()

    });

    let _ = server.join();
    let _ = client.join();

    println!("All done.")
}
