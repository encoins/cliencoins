use std::fs::read;
use std::net::{TcpStream,SocketAddr};
use std::io::{Write, Read, stdin, stdout};
use crate::client::Client;


use crate::input::{Input,read_input};
use crate::instructions::Instruction;
use crate::crypto::{SignedInstruction};

pub fn exchange_with_server(client : &Client, mut stream: TcpStream) { // je suis tenté de le faire en impl de Client
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

                let instruction = Instruction::Transfer {sender,recipient,amount};
                let serialized_instruction = &(bincode::serialize(&instruction).unwrap()[..]);
                println!("serialized = {:?}", serialized_instruction);

                let signed_instruction = instruction.sign_instruction(&client.secret_key);

                let buf = &(bincode::serialize(&signed_instruction).unwrap()[..]);

            }
            Input::Balance { user } => {
                let instruction = Instruction::Balance {user};
                let serialized_instruction = &(bincode::serialize(&instruction).unwrap()[..]);
                println!("serialized = {:?}", serialized_instruction);

                let signed_instruction = instruction.sign_instruction(&client.secret_key);

                let buf = &(bincode::serialize(&signed_instruction).unwrap()[..]);
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

pub fn connect_to_serv(client : &Client) {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    println!("Choisi l'adresse de connection");
    loop {
        write!(io, "Adresse :");
        io.flush();

        let addr = &*get_entry();
        println!("{}", addr);
        println!("Tentative de connexion au serveur...");

        if addr == "quit" {return; }

        match TcpStream::connect(addr) {
            Ok(stream) => {
                println!("Connexion au serveur réussie !");
                exchange_with_server(client,stream);
            }
            Err(e) => {
                println!("La connexion au serveur a échoué : {}", e);
            }
        }
    }
}
