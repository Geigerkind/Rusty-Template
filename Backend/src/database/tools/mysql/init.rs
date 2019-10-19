use mysql;

use crate::database::material::mysql_connection::MySQLConnection;

pub trait Init {
  fn new(db_name: &str) -> Self;
}

impl Init for MySQLConnection {
  fn new(db_name: &str) -> Self
  {
    let mut dns: String = "mysql://root@127.0.0.1/".to_owned();
    dns.push_str(db_name);
    MySQLConnection {
      con: mysql::Pool::new(dns).unwrap()
    }
  }
}