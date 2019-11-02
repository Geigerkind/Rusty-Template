extern crate dotenv;
use dotenv::dotenv;
use std::env;

pub struct MySQLConnection {
  pub con: mysql::Pool
}

impl MySQLConnection {
  pub fn new(db_name: &str) -> Self
  {
    dotenv().ok();
    MySQLConnection {
      con: mysql::Pool::new([&env::var("MYSQL_DNS").unwrap(), db_name].concat()).unwrap()
    }
  }
}