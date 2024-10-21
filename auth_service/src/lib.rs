pub mod database;

pub mod auth_utils;

use database::{Status, Credentials};
use auth_utils:: {login,connect_to_db};

pub fn auth(cred:Credentials){
  if let Status::Connected = connect_to_db() { 
       login(cred);
  }
}