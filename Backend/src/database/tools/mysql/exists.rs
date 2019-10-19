use mysql;

use crate::database::material::mysql_connection::MySQLConnection;
use crate::database::tools::mysql::select::Select;

pub trait Exists {
  fn exists(&self, query_str: &str) -> bool;
  fn exists_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool;
}

impl Exists for MySQLConnection {
  fn exists(&self, query_str: &str) -> bool
  {
    let mut exists: String = "SELECT EXISTS(".to_owned();
    exists.push_str(query_str);
    exists.push_str(")");
    self.select_value(&exists, &|row|{
      let res: bool = mysql::from_row(row);
      res
    }).unwrap()
  }

  fn exists_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool
  {
    let mut exists: String = "SELECT EXISTS(".to_owned();
    exists.push_str(query_str);
    exists.push_str(")");
    self.select_wparams_value(&exists, &|row|{
      let res: bool = mysql::from_row(row);
      res
    }, params).unwrap()
  }
}