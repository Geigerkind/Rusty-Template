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

    pub fn select<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Vec<T>
    {
        self.con.prep_exec(query_str, ())
        .map(|result| {
            result.map(|x| x.unwrap())
            .map(|row| process_row(row))
            .collect()
        }).unwrap()
    }

    pub fn select_wparams<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Vec<T>
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

    pub fn select_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T) -> Option<T>
    {
        // Just return first row, see TODO
        for row in self.select(query_str, process_row) {
            return Some(row);
        }
        None
    }

    pub fn select_wparams_value<T>(&self, query_str: &str, process_row: &dyn Fn(mysql::Row) -> T, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> Option<T>
    {
        // Just return first row, see TODO
        for row in self.select_wparams(query_str, process_row, params) {
            return Some(row);
        }
        None
    }

    /**
    * Normal execution, i.e. update and inserts
    **/

    pub fn execute(&self, query_str: &str) -> bool
    {
        self.con.prep_exec(query_str, ()).unwrap().affected_rows() > 0
    }
    pub fn execute_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool
    {
        self.con.prep_exec(query_str, params).unwrap().affected_rows() > 0
    }

    /**
    * Test if something exists
    **/

    pub fn exists(&self, query_str: &str) -> bool
    {
        let mut exists: String = "SELECT EXISTS(".to_owned();
        exists.push_str(query_str);
        exists.push_str(")");
        self.select_value(&exists, &|row|{
            let res: bool = mysql::from_row(row);
            res
        }).unwrap()
    }
    pub fn exists_wparams(&self, query_str: &str, params: std::vec::Vec<(std::string::String, mysql::Value)>) -> bool
    {
        let mut exists: String = "SELECT EXISTS(".to_owned();
        exists.push_str(query_str);
        exists.push_str(")");
        self.select_wparams_value(&exists, &|row|{
            let res: bool = mysql::from_row(row);
            res
        }, params).unwrap()
    }





    pub fn new(db_name: &str) -> Self
    {
        let mut dns: String = "mysql://root@127.0.0.1/".to_owned();
        dns.push_str(db_name);
        MySQLConnection {
            con: mysql::Pool::new(dns).unwrap()
        }
    }
}

