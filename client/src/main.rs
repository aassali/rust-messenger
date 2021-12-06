#[macro_use] extern crate magic_crypt;

use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use magic_crypt::MagicCryptTrait;
use std::str;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn encrypt(message: &str) -> String {
    let mc = new_magic_crypt!("magickey", 256);
    mc.encrypt_str_to_base64(message)
}

fn decrypt(message: &str) -> String {
    let mc = new_magic_crypt!("magickey", 256);
    mc.decrypt_base64_to_string(&message).unwrap()
}

struct Client {
    username: String,
    conn: TcpStream,
}

fn build_client(username: String, conn: TcpStream) -> Client {
    Client{
        username,
        conn,
    }
}

fn main() {
    let d = encrypt("blablabla");
    println!("{}", &d);
    let stream = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    let mut client = build_client("toto".to_string(), stream);
    //println!("Hello: {}", client.username);
    client.conn.set_nonblocking(true).expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.conn.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("message recv : {:?}", decrypt(str::from_utf8(&msg).unwrap()));
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.conn.write_all(&buff).expect("writing to socket failed");
            }, 
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a Message:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(encrypt(&msg)).is_err() {break}
    }
    println!("bye bye!");

}