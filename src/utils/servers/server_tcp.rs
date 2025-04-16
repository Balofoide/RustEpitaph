use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use uuid::Uuid;

use crate::utils::servers::database_struct::ClientInfo;
use crate::utils::servers::database_struct::Database;

pub fn spawn_server(database: Arc<Database>) {
    thread::spawn(move || {
        server(database);
    });
}

pub fn server(database: Arc<Database>) {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Erro ao Iniciar o Listener.");

    for stream in listener.incoming() {
        let id = Uuid::new_v4();
        match stream {
            Ok(stream) => {
                let stream_copy = stream.try_clone().expect("Erro ao clonar a stream");
                let client = ClientInfo::new(stream_copy, "NULL");

                let stream_copy = stream.try_clone().expect("Erro ao clonar a stream");
                println!(
                    "Novo cliente conectado {}",
                    stream_copy.peer_addr().unwrap().ip()
                );

                // let stream_copy = stream.try_clone().expect("Erro ao clonar a stream");
                let database_clone = Arc::clone(&database);
                database_clone.add_client(id, client);

                // vetor.lock().unwrap().insert(id, stream_copy);
            }
            Err(e) => println!("Stream Error! :{}", e),
        }
    }
}

pub fn handle_clients(database: Arc<Database>, input: &str) {
    let database_clone = Arc::clone(&database);

    // Tenta buscar pelo alias
    if let Some(id) = database_clone.alias_to_id(input) {
        // Alias existe: usa o ID associado
        match database_clone.get_stream(&id) {
            Some(ip) => {
                let mut ip = ip.try_clone().expect("Falha ao clonar tcp");
                match ip.peer_addr() {
                    Ok(_) => handle_tcp(&mut ip, database, id),
                    Err(_) => {
                        println!("Client Disconected");
                    }
                }
            }
            None => println!("Client Offline"),
        }
    } else {
        // Se não é alias, tenta parsear como UUID
        match input.parse::<Uuid>() {
            Ok(uuid) => match database_clone.get_stream(&uuid) {
                Some(ip) => handle_tcp(&ip, database, uuid),
                None => println!("UUID não encontrado ou sem conexão."),
            },
            Err(_) => println!("Input inválido: não é um alias registrado nem um UUID."),
        }
    }
}

pub fn handle_tcp(stream: &TcpStream, database: Arc<Database>, id: Uuid) {
    // std::process::Command::new("clear").status().unwrap();

    loop {
        print!("{}>", stream.peer_addr().unwrap().ip());
        io::stdout()
            .flush()
            .expect("Falha ao fazer flush do stdout");

        if let Err(_) = stream.peer_addr() {
            println!("Cliente desconectado!");
            return; // Sai completamente da função
        }

        let input = user_input();

        match input.as_str() {
            "help" => {
                println!("-----Host-Commands----");
                println!("bg -> Backgrounds the session");
                println!("cmd -> Connect to remote shell terminal");
                println!("help -> Help message");
                println!("clear -> Clear the terminal history");
            }
            "bg" => break,
            "clear" => {
                std::process::Command::new("clear").status().unwrap();
            }
            "cmd" => loop {
                if let Err(_) = stream.peer_addr() {
                    database.remove_client(id);
                    println!("Client Disconected!");
                    return;
                }

                match interact("cd".to_string(), stream) {
                    Ok(dir) => {
                        let dir = dir.trim();
                        print!("{}", dir);
                        io::stdout().flush().unwrap();

                        let cmd_commands = user_input();

                        if cmd_commands == "/exit"{
                            break;
                        }else if cmd_commands == "/help"{
                        
                            println!("----Pseudo-Terminal----");
                            println!("/exit -> Exit from host Terminal");
                        }

                        else {

                        match interact(cmd_commands, stream) {

                            Ok(resposta) => {
                                println!("{}", resposta.replace(dir, ""));
                            }
                            Err(_) => {
                                println!("Client Timeout");
                            }
                        }
                    }
                    }
                    Err(_) => {
                        println!("Client Timeout");
                    }
                }
            },
            _ => println!("Commando Invalido"),
        }
    }
}

pub fn manage_alias(database: Arc<Database>, id: &str, alias: &str) {
    let id: Uuid = id
        .trim()
        .parse()
        .expect("Erro na conversão do buffer para Uuid");

    database.update_alias(&id, &alias);
}

fn interact(send: String, mut stream: &TcpStream) -> Result<String, io::Error> {
    match stream.peer_addr() {
        Ok(_) => {
            stream.write_all(send.as_bytes())?;

            stream.set_read_timeout(Some(std::time::Duration::from_secs(30)))?;

            let mut buffer = [0; 65535];
            let bytes_read = stream.read(&mut buffer)?;

            Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
        }
        Err(e) => Err(e),
    }

    // let send_bytes = send.as_bytes();

    //  let mut stream = stream;
    // stream.write(send_bytes)
    // .expect("Erro ao Enviar a mensagem");

    // let mut get_bytes:[u8;65535] = [0; 65535];

    // let temp_buffer = stream.read(&mut get_bytes)
    // .expect("Erro ao ler a resposta");

    // let resposta = String::from_utf8_lossy(&get_bytes[..temp_buffer]);

    // return resposta.to_string();
}

pub fn user_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Erro ao ler input");
    let input = buffer.trim().to_string(); // Input tratado (sem espaços extras)
    return input;
}
