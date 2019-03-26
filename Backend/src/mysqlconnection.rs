use mysql;

/**
* TODO:
* Shouldnt we maintain the connections here instead of passing it around?
**/

pub struct MySQLConnection {
    pub con: mysql::Pool
}

impl MySQLConnection {
    /**
    * Select
    * Return Vector
    **/

    pub fn select<T>(&self, query_str: &str, process_row: &Fn(mysql::Row) -> T) -> Vec<T>
    {
        self.con.prep_exec(query_str, ())
        .map(|result| {
            result.map(|x| x.unwrap())
            .map(|row| process_row(row))
            .collect()
        }).unwrap()
    }

    pub fn select_wparams<T>(&self, query_str: &str, process_row: &Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Vec<T>
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

    pub fn select_value<T>(&self, query_str: &str, process_row: &Fn(mysql::Row) -> T) -> Option<T>
    {
        // Just return first row, see TODO
        for row in self.select(query_str, process_row) {
            return Some(row);
        }
        None
    }

    pub fn select_wparams_value<T>(&self, query_str: &str, process_row: &Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Option<T>
    {
        // Just return first row, see TODO
        for row in self.select_wparams(query_str, process_row, params) {
            return Some(row);
        }
        None
    }

    pub fn new() -> Self
    {
        MySQLConnection {
            con: mysql::Pool::new("mysql://root@127.0.0.1/test").unwrap()
        }
    }
}

