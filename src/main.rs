use std::sync::Arc;
 
mod utils;

use crate::utils::servers::database_struct::Database;
use crate::utils::servers::server_tcp::spawn_server;
use crate::utils::menu::menu::{spawn_menu,banner};


fn main() {
    let db = Arc::new(Database::new());
    banner();

    let db_clone = Arc::clone(&db);
    spawn_server(db_clone);

    let db_clone = Arc::clone(&db);
    spawn_menu(db_clone);
}


