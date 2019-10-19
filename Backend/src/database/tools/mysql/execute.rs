use mysql;

use crate::database::material::mysql_connection::MySQLConnection;

pub trait Execute {
  fn execute(&self, query_str: &str) -> bool;
  fn execute_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool;
}

impl Execute for MySQLConnection {
  /**
  * Normal execution, i.e. update and inserts
  **/

  fn execute(&self, query_str: &str) -> bool
  {
    self.con.prep_exec(query_str, ()).unwrap().affected_rows() > 0
  }

  fn execute_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool
  {
    self.con.prep_exec(query_str, params).unwrap().affected_rows() > 0
  }
}