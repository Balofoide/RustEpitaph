use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
 

type Clientes = Arc<Mutex<HashMap<Uuid,ClientInfo>>>;
#[derive(Debug)]
pub struct ClientInfo {
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



pub struct Database{
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

    pub fn remove_client(&self, id:Uuid){
        let mut client = self.client.lock().unwrap();
        if let Some(_) = client.remove(&id){

        }
    }

    pub fn list_clientes(&self){
        let client = self.client.lock().unwrap();
        for(id,client) in client.iter(){
            if client.alias != "NULL"{
                println!("{} -> {} ", client.alias, client.stream.peer_addr().unwrap().ip());
            }else{
                println!("{} -> {} ", id, client.stream.peer_addr().unwrap().ip());
            }
        }
    }
 
    pub fn get_stream(&self,id:&Uuid) -> Option<TcpStream>{

        let client = self.client.lock().unwrap();

        return client.get(id).map(|client|client.stream.try_clone().unwrap());

    }


    pub fn alias_to_id(&self, input:&str) -> Option<Uuid>{

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