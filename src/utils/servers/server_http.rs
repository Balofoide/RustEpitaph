 

use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;

use std::fs;
use uuid::Uuid;
use std::io::{self, Write};

use crate::utils::servers::database_struct::ClientInfo;
use crate::utils::servers::database_struct::Database;

pub fn spawn_http_server(database: Arc<Database>) {
    thread::spawn(move || {

        http_server(database);
    });
}

pub fn http_server(database: Arc<Database>) {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Erro ao Iniciar o Listener.");

    for stream in listener.incoming() {
        let id = Uuid::new_v4();
        match stream {
            Ok(stream) => {
                let stream_copy = stream.try_clone().expect("Erro ao clonar a stream");
                let client = ClientInfo::new("HTTP".to_string(),stream_copy, "NULL");

                let stream_copy = stream.try_clone().expect("Erro ao clonar a stream");
                println!(
                    "\n[+] Novo cliente conectado {}",
                    
                    stream_copy.peer_addr().unwrap().ip()
                );
                io::stdout().flush().expect("Falha ao fazer flush do stdout");

                // let stream_copy = stream.try_clone().expect("Erro ao clonar a stream");
                let database_clone = Arc::clone(&database);
                database_clone.add_client(id, client);

                // vetor.lock().unwrap().insert(id, stream_copy);
            }
            Err(e) => println!("Stream Error! :{}", e),
        }
    }
}


pub fn handle_http(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let http_request = buf_reader.lines().next().unwrap().unwrap();
    if http_request == "GET / HTTP/1.1"{

        let status_line = "HTTP/1.1 200 OK";
        let page = fs::read_to_string("teste.html").unwrap();
        let length = page.len();

        let response = format!("{status_line}\r\nContent:Length: {length}\r\n\r\n{page}");

        stream.write_all(response.as_bytes());

    }
    else{
         let status_line = "HTTP/1.1 404 NOT FOUND";
        let page = fs::read_to_string("404.html").unwrap();
        let length = page.len();

        let response = format!("{status_line}\r\nContent:Length: {length}\r\n\r\n{page}");

        stream.write_all(response.as_bytes());
    }


}

pub fn handle_clients_http(database: Arc<Database>, input: &str) {
    let database_clone = Arc::clone(&database);

    // Tenta buscar pelo alias
    if let Some(id) = database_clone.alias_to_id(input) {
        // Alias existe: usa o ID associado
        match database_clone.get_stream(&id) {
            Some(ip) => {
                let mut ip = ip.try_clone().expect("Falha ao clonar tcp");
                match ip.peer_addr() {
                    Ok(_) => handle_http(ip),
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
                Some(ip) => handle_http(ip),
                None => println!("UUID não encontrado ou sem conexão."),
            },
            Err(_) => println!("Input inválido: não é um alias registrado nem um UUID."),
        }
    }
}
 

 