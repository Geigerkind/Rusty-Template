use mysql;

use crate::database::material::mysql_connection::MySQLConnection;

pub trait Select {
  fn select<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Vec<T>;
  fn select_wparams<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Vec<T>;
  fn select_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Option<T>;
  fn select_wparams_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Option<T>;
}

impl Select for MySQLConnection {
  /**
  * Select
  * Return Vector
  **/

  fn select<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Vec<T>
  {
    self.con.prep_exec(query_str, ())
    .map(|result| {
      result.map(|x| x.unwrap())
      .map(|row| process_row(row))
      .collect()
    }).unwrap()
  }

  fn select_wparams<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Vec<T>
  {
    self.con.prep_exec(query_str, params)
    .map(|result| {
      result.map(|x| x.unwrap())
      .map(|row| process_row(row))
      .collect()
    }).unwrap()
  }

  /**
  * Return value directly
  * TODO:
  * Find out how it it done properly, for now using this hack
  **/

  fn select_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Option<T>
  {
    // Just return first row, see TODO
    for row in self.select(query_str, process_row) {
      return Some(row);
    }
    None
  }

  fn select_wparams_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Option<T>
  {
    // Just return first row, see TODO
    for row in self.select_wparams(query_str, process_row, params) {
      return Some(row);
    }
    None
  }
}