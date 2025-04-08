use std::collections::HashMap;
 
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
 
use std::io;
 
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

type Clientes = Arc<Mutex<HashMap<Uuid,ClientInfo>>>;
#[derive(Debug)]
struct ClientInfo {
    stream: TcpStream,
    alias: String,
} 
impl ClientInfo {
    
    pub fn new(stream:TcpStream, alias:&str) -> Self{
        ClientInfo { stream, alias:alias.to_string() }
    }

    pub fn set_alias(&mut self, new_alias: &str) {
        self.alias = new_alias.to_string();
    }
}
struct Database{
    client:Clientes
}

impl Database { 

    pub fn new() -> Self {
        Database { client: Arc::new(Mutex::new(HashMap::new())), }
    }
    
    pub fn add_client(&self, id: Uuid, client_info:ClientInfo){
        let mut client = self.client.lock().unwrap();
        client.insert(id, client_info);

    }

    // pub fn remove_client(&self, id:Uuid){
    //     let mut client = self.client.lock().unwrap();
    //     if let Some(client) = client.remove(&id){

    //     }
    // }

    pub fn list_clientes(&self){
        let client = self.client.lock().unwrap();
        for(id,client) in client.iter(){
            if client.alias != "NULL"{
                println!("{} -> {} ", client.alias, client.stream.peer_addr().unwrap().ip());
            }else{
                println!("{}",client.alias);
                println!("{} -> {} ", id, client.stream.peer_addr().unwrap().ip());
            }
        }
    }

    pub fn get_stream(&self,id:&Uuid) -> Option<TcpStream>{

        let client = self.client.lock().unwrap();

        return client.get(id).map(|client|client.stream.try_clone().unwrap());

    }


    pub fn alias_to_id(&self, input:&String) -> Option<Uuid>{

        let client = self.client.lock().unwrap();

       let result =  client.iter()
        .find(|(_,client)| client.alias == input.to_string())
        .map(|(id,_)| *id);

        return result;
    }

    // pub fn get_stream_alias(&self,alias:&Uuid) -> Option<TcpStream>{

    //     let client = self.client.lock().unwrap();
        
    //     return client.get(alias).map(|client|client.stream.try_clone().unwrap());

    // }

    pub fn update_alias(&self, idi: &Uuid, new_alias: &str) {
        let mut clients = self.client.lock().unwrap();  // Notar o mut aqui
        
        // Usar get_mut para obter referência mutável
        if let Some(client) = clients.get_mut(idi) {
            client.set_alias(new_alias);
        }
    }

 
}



fn main() {
    let mut db = Arc::new(Database::new());

    std::process::Command::new("clear").status().unwrap();
    
    // let mut clientes:Clientes = Arc::new(Mutex::new(HashMap::new()));
   
    // Clona a referencia atomica do dicionario de clientes, e abre o servidor em uma thread separada, para sempre ficar escutando novos clientes.
    // let clientes_clone = Arc::clone(   &mut clientes);
   
     
    let db_clone = Arc::clone(&mut db);
      thread::spawn(move ||{
        server(db_clone);
    });

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
    
    // Loop do menu da aplicação
    loop{
        let db_clone = Arc::clone(&mut db);
        interface(db_clone);
        
    }
    
 
}

fn interface(database: Arc<Database>){
    // std::process::Command::new("clear").status().unwrap();
   
    
    print!("\n>");
    // O codigo espera uma quebra de linha para mostrar o buffer, caso não tenha ele fica travado no stdin e só mostra depois dele.
    // Assim é preciso dar um flush no stdout manualmente, fazendo com que ele apareça no terminal.
    io::stdout().flush().expect("Falha ao fazer flush do stdout");
 

    let input = input();

    match input.as_str(){
        "clients" => {
            let database_clone = Arc::clone(  &database);
            database_clone.list_clientes();
        },
        "connect" => {
            let database_clone = Arc::clone(  &database);
            handle_clients(database_clone);
        }
        "alias" => {
            manage_alias(database);
        }
        "help" =>   println!("
        »help\n
        »list\n
        »alias\n
        »connect"),

        _ => println!("Comando incorreto")
    };

}

fn server(database:Arc<Database>){

    
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

fn handle_clients(database: Arc<Database>){

     
   

    print!("Host para conectar: ");
    io::stdout().flush().expect("Falha ao fazer flush do stdout");

    let input = input();

    let database_clone = Arc::clone(&database);

    // Tenta buscar pelo alias
    if let Some(id) = database_clone.alias_to_id(&input) {
        // Alias existe: usa o ID associado
        match database_clone.get_stream(&id) {
            Some(ip) => handle_tcp(&ip),
            None => println!("Cliente encontrado, mas sem conexão ativa."),
        }
    } else {
        // Se não é alias, tenta parsear como UUID
        match input.parse::<Uuid>() {
            Ok(uuid) => {
                match database_clone.get_stream(&uuid) {
                    Some(ip) => handle_tcp(&ip),
                    None => println!("UUID não encontrado ou sem conexão."),
                }
            }
            Err(_) => println!("Input inválido: não é um alias registrado nem um UUID."),
        }
    }

}

fn handle_tcp( stream: & TcpStream){
    std::process::Command::new("clear").status().unwrap();
    println!("Cliente: {}", stream.peer_addr().unwrap().ip());
    

    loop {
         
        
        print!("{} >",stream.peer_addr().unwrap().ip());
        io::stdout().flush().expect("Falha ao fazer flush do stdout");


        
        let input = input();

        match input.as_str(){
            "/voltar" => break,
            _ => println!("{}",interact(input, stream))
            
        }
         
        
    }
}

fn manage_alias(database:Arc<Database> ){

    let mut send:String = String::new();
    let mut buffer:String = String::new();
        print!("Digite o ID: ");
        io::stdout().flush().expect("Falha ao fazer flush do stdout");

        io::stdin().read_line(&mut buffer).expect("Não foi possivel ler a mensagem");

        print!("Digite um alias:");
        io::stdout().flush().expect("Falha ao fazer flush do stdout");

        io::stdin().read_line(&mut send).expect("Não foi possivel ler a mensagem");
        
        let input = send.trim().to_lowercase();
        let id:Uuid = buffer.trim().parse().expect("Erro na conversão do buffer para Uuid");

    database.update_alias(&id,&input);

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

fn input() -> String{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erro ao ler input");
    let input = buffer.trim().to_string(); // Input tratado (sem espaços extras)
    return input;
}