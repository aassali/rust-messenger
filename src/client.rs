use std::net::TcpStream;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = &args[1];

    println!("Searching for {}", name);


    println!("Tentative de connexion au serveur...");
    match TcpStream::connect("127.0.0.1:1234") {
        Ok(_) => {
            println!("Connexion au serveur réussie !");
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}