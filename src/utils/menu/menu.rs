use std::io::prelude::*;
use std::io;
use std::sync::Arc;
 
use crate::utils::database_struct::Database;
use crate::utils::servers::server_tcp::{input,handle_clients,manage_alias};


pub fn spawn_menu(database:Arc<Database>){
    loop{
        let db_clone = Arc::clone(&database);
        interprete(db_clone);  
    }
}


pub fn banner(){
    std::process::Command::new("clear").status().unwrap(); 
    println!("                                                                                                         
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
                                                                                                         \n");
 println!("Your last words, will eventually rust.");
}

pub fn interprete(database: Arc<Database>){

    print!("\n>");
    io::stdout().flush().expect("Falha ao fazer flush do stdout");

    let input = &input();
    let comandos:Vec<&str> = input.split_whitespace().collect();

    if comandos.is_empty(){
        return;
    }

    match comandos[0].to_lowercase().as_str(){
        "connect" => {
            if comandos.len() != 2 {
                println!("Comando Incorreto.");
            }else {
                handle_clients(database, comandos[1]);
            }
        }
        "alias" => {
            if comandos.len() != 3 {
                println!("Comando Incorreto.");
            }else{
                 manage_alias(database, comandos[1], comandos[2]);
            }
        }
        "list" => {
            database.list_clientes();
        }
        "help" => {
            println!("connect <ID | Alias>");
            println!("alias <ID> <Alias>");
            println!("list");
        }
        _ => println!("Comando inexistente")
    }

}
