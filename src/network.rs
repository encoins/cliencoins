use std::fs::read;
use std::net::{TcpStream,SocketAddr};
use std::io::{Write, Read, stdin, stdout};


use crate::input::{Input,read_input};

pub fn exchange_with_server(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let mut buf = [0; 1+4+4+4]; // the first to discrimine read/transfer then 3 element of type u32

    println!("Enter 'quit' when you want to leave");
    loop {
        //write!(io, "> ");
        // pour afficher de suite
        //io.flush();
        match read_input() {
            Input::Transfer { sender, recipient, amount } => {
                let a: u8 = (sender >> 24 ) as u8;
                let b: u8 = (sender >> 16) as u8;
                let c: u8 = (sender >> 8) as u8;
                let d: u8 = sender as u8;

                let e: u8 = (recipient >> 24) as u8;
                let f: u8 = (recipient >> 16) as u8;
                let g: u8 = (recipient >> 8) as u8;
                let h: u8 = recipient as u8;

                let i: u8 = (amount >> 24) as u8;
                let j: u8 = (amount >> 16) as u8;
                let k: u8 = (amount >> 8) as u8;
                let l: u8 = amount as u8;

                buf = [1, a, b, c, d, e, f, g, h, i, j, k, l];
                //buf = [1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8]
            }
            Input::Balance { user } => {
                let a: u8 = (user >> 24) as u8;
                let b: u8 = (user >> 16) as u8;
                let c: u8 = (user >> 8) as u8;
                let d: u8 = user as u8;

                buf = [0, a, b, c, d, 0, 0, 0, 0, 0, 0, 0, 0];
                //buf = [1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8,1 as u8]
            }
            /// Input to get transactions history of an account according to a given account
            Input::Help => {
                return;
            }
            /// Input to clear terminal from previous inputs
            Input::Clear => {
                return;
            }
            /// Input to quit program
            Input::Quit => {
                println!("bye !");
                return;
            }
        }
        stream.write(&mut buf);
        buf = [0; 1+4+4+4];
        match stream.read(&mut buf) {
            Ok(received) => {
                if received < 1 {
                    println!("Perte de la connexion avec le serveur");
                    return;
                }
            }
            Err(_) => {
                println!("Perte de la connexion avec le serveur");
                return;
            }
        }
        println!("Réponse du serveur : {:?}", buf);
    }
}

fn get_entry() -> String {
    let mut buf = String::new();

    stdin().read_line(&mut buf);
    buf.replace("\n", "").replace("\r", "")
}

pub fn connect_to_serv() {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    println!("Choisi l'adresse de connection");
    loop {
        write!(io, "Adresse :");
        io.flush();

        let addr = &*get_entry();
        println!("{}", addr);
        println!("Tentative de connexion au serveur...");

        match TcpStream::connect(addr) {
            Ok(stream) => {
                println!("Connexion au serveur réussie !");
                exchange_with_server(stream);
            }
            Err(e) => {
                println!("La connexion au serveur a échoué : {}", e);
            }
        }
    }
}
