pub struct MySQLConnection {
  pub con: mysql::Pool
}

impl MySQLConnection {
  pub fn new(db_name: &str) -> Self
  {
    let mut dns: String = "mysql://root:vagrant@127.0.0.1:5555/".to_owned();
    dns.push_str(db_name);
    MySQLConnection {
      con: mysql::Pool::new(dns).unwrap()
    }
  }
}