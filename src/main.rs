use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
 
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

type Clientes = Arc<Mutex<HashMap<usize,TcpStream>>>;

fn main() {
    std::process::Command::new("clear").status().unwrap();
    let mut clientes:Clientes = Arc::new(Mutex::new(HashMap::new()));
    
    // Clona a referencia atomica do dicionario de clientes, e abre o servidor em uma thread separada, para sempre ficar escutando novos clientes.
    let clientes_clone = Arc::clone(   &mut clientes);
      thread::spawn(move ||{
        server(clientes_clone);
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
        let clientes_clone = Arc::clone(   &mut clientes);
        interface(clientes_clone);
        
    }
    


  
}




fn interface(clientes: Clientes) 
{
    // std::process::Command::new("clear").status().unwrap();
    let mut buffer:String = String::new();
    
    print!("\n>");
    // O codigo espera uma quebra de linha para mostrar o buffer, caso não tenha ele fica travado no stdin e só mostra depois dele.
    // Assim é preciso dar um flush no stdout manualmente, fazendo com que ele apareça no terminal.
    io::stdout().flush().expect("Falha ao fazer flush do stdout");


    io::stdin().read_line(&mut buffer).expect("Erro ao ler input");

    let input = buffer.trim().to_lowercase();

    match input.as_str(){
        "clients" => {
            // std::process::Command::new("clear").status().unwrap();

            println!("Lista"); 
            let clientes_clone = Arc::clone(&clientes);
            handle_clients( clientes_clone);
        },
        "help" =>   println!("»help\n»clients"),
        _ => println!("Errado")
    };

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

fn server(vetor:Clientes){

 
    let mut id:usize = 0;

    let listener = TcpListener::bind("127.0.0.1:8080")
    .expect("Erro ao Iniciar o Listener.");


    
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {

                
                let stream_copy = stream.try_clone().expect("Erro ao clonar a stream");

                vetor.lock().unwrap().insert(id, stream_copy);
                id +=1;
               
            }
            Err(e) => println!("Stream Error! :{}",e),
        }
    }

}


fn list_clientes(clientes: Clientes){

    let lock_clientes = clientes.lock().unwrap();
    let ids:Vec<usize> = lock_clientes.keys().cloned().collect();


    for conectados in ids{
        match lock_clientes.get(&conectados){
            Some(ip) => println!("{}->{}",conectados,ip.peer_addr().unwrap().ip()),
            None => println!("List Error!")
        }

    }



}


fn handle_clients(clientes: Clientes){

    let clientes_clone = Arc::clone(&clientes);
    list_clientes(clientes_clone);

    print!(">");
    let mut buffer:String = String::new();
    io::stdout().flush().expect("Falha ao fazer flush do stdout");

    io::stdin().read_line(&mut buffer).expect("Erro ao ler input");
    let option:usize = buffer.trim().parse().expect("Erro na conversão do buffer para i32");

    
    let clientes_clone = Arc::clone(&clientes);
    let alvo = clientes_clone.lock().unwrap();

   match alvo.get(&option){
    Some(ip) => handle_tcp(ip),
    None => println!("Cliente Invalido")
   };


}

 

fn handle_tcp( stream: & TcpStream){
    std::process::Command::new("clear").status().unwrap();
    println!("Cliente: {}", stream.peer_addr().unwrap().ip());
    

    loop {
        let mut send:String = String::new();
        
        print!("{} >",stream.peer_addr().unwrap().ip());
        io::stdout().flush().expect("Falha ao fazer flush do stdout");

        io::stdin().read_line(&mut send).expect("Não foi possivel ler a mensagem");
        
        let input = send.trim().to_lowercase();

        match input.as_str(){
            "/voltar" => break,
            _=> println!("{}",interact(send,stream))
        }
         
        
    }
}