use crate::CONFIGURATION;
use postgres::{Client, NoTls, Row};
use log::{error, info};

pub trait SqlStatement{
    fn get_query_string() -> String;
    fn get_client() -> DbClient;
    fn process(&self) -> Result<Option<Vec<Row>>, String>;
}

pub struct DbClient{
    connection_string: String,
}
impl DbClient{
    pub fn new() -> Self {
        return Self {
            connection_string: Self::get_connection_string(),
        };
    }

    fn get_connection_string() -> String {
        //postgres://<Username>:<Password>@<Host>:<port>/<DatabaseName>
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            CONFIGURATION.database.connection.username,
            CONFIGURATION.database.connection.password,
            CONFIGURATION.database.connection.host,
            CONFIGURATION.database.connection.port,
            CONFIGURATION.database.connection.database_name
        );
        return connection_string;
    }

    // Used when you want to recieve data back from the database
    pub fn query(&self, query: &str, params: &[&(dyn postgres::types::ToSql + Sync)]) -> Result<Option<Vec<Row>>, String> {
        // Get a client
        let client_result = self.open_client();

        // Check if the client connection was successful
        if client_result.is_some() {
            // Unwrap to get the client
            let mut client = client_result.unwrap();

            // Perform the query and manage errors
            match client.query(query, params) {
                Ok(rows) => {
                    Self::close_client(client);
                    return Ok(Some(rows));
                },
                Err(err) => {
                    error!("The query \n{}\n failed \n{:?}", &query, err);
                    Self::close_client(client);
                    return Err(format!("Query failed: {:?}", &query));
                },
            }            
        }
        else{
            return Err("Failed to connect to DB".to_string());
        }
    }

    // Used when you want to put data into the database without returning anything
    pub fn execute(&self, query: &str, params: &[&(dyn postgres::types::ToSql + Sync)]) -> Result<Option<Vec<Row>>, String> {
        // Get a client
        let client_result = self.open_client();

        // Check if the client connection was successful
        if client_result.is_some() {
            // Unwrap to get the client
            let mut client = client_result.unwrap();

            // Perform the query and manage errors
            match client.execute(query, params) {
                Ok(_) => {
                    Self::close_client(client);
                    return Ok(None);
                },
                Err(err) => {
                    error!("The query \n{}\n failed \n{:?}", &query, err);
                    Self::close_client(client);
                    return Err(format!("Query failed: {:?}", &query));
                },
            }
        }
        else{
            return Err("Failed to connect to DB".to_string());
        }        
    }

    // Used when you want to execute multiple lines of sql without returning anything
    pub fn batch_execute(&self, query: &str) -> Result<Option<Vec<Row>>, String> {
        // Get a client
        let client_result = self.open_client();

        // Check if the client connection was successful
        if client_result.is_some() {
            // Unwrap to get the client
            let mut client = client_result.unwrap();

            // Perform the query and manage errors
            match client.batch_execute(query) {
                Ok(_) => {
                    Self::close_client(client);
                    return Ok(None);
                },
                Err(err) => {
                    error!("The query \n{}\n failed \n{:?}", &query, err);
                    Self::close_client(client);
                    return Err(format!("Query failed: {:?}", &query));
                },
            }
        }
        else{
            return Err("Failed to connect to DB".to_string());
        }        
    }
    
    fn open_client(&self) -> Option<Client> {
        // Create a new client connection
        let res = Client::connect(&self.connection_string, NoTls);
        
        // Check for an error and perform relevant logging
        if res.is_err(){
            error!("Failed to establish client connection {} \n{:?}", &self.connection_string, res.err());
            return None;
        }
        else{
            info!("Client connection successfully established");
            return Some(res.unwrap());
        }        
    }

    fn close_client(client: Client){
        // Close the client
        let res = client.close();

        // Check for an error and perform relevant logging
        if res.is_err(){
            error!("Client failed to close \n{:?}", res.err());
        }
        else {
            info!("The client was closed successfully");
        }
    }
}