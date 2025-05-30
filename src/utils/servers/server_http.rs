 

use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use uuid::Uuid;

use crate::utils::servers::database_struct::ClientInfo;
use crate::utils::servers::database_struct::Database;

pub fn spawn_http_server(database: Arc<Database>) {
    thread::spawn(move || {

        http_server(database);
    });
}

pub fn http_server(database: Arc<Database>) {
    let listener = TcpListener::bind("127.0.0.1:80").expect("Erro ao Iniciar o Listener.");

    for stream in listener.incoming() {
        let id = Uuid::new_v4();
        match stream {
            Ok(stream) => {
                let stream_copy = stream.try_clone().expect("Erro ao clonar a stream");
                let client = ClientInfo::new("HTTP".to_string(),stream_copy, "NULL");

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


// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&stream);
//     let http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();

//     let status_line = "HTTP/1.1 200 OK";
//     let contents = fs::read_to_string("hello.html").unwrap();
//     let length = contents.len();

//     let response =
//         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();
// }

 