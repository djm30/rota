use log::warn;
use postgres::Row;
use serde::{Serialize, Deserialize};
use super::db_client::*;

#[derive(Deserialize, Serialize)]
pub struct CompleteTask{
    pub id: i32,
}
impl SqlStatement for CompleteTask{
    fn get_query_string() -> String {
        return "SELECT \"task\".\"complete\"($1);".to_string();
    }

    fn get_client() -> DbClient {
        return DbClient::new();
    }

    fn process(&self) -> Result<Option<Vec<Row>>, String> {
        return Self::get_client().query(
            &Self::get_query_string(),
            &[&self.id],
        );
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateTask{
    pub name: String,
    pub description: String,
    pub assigned_person: i32,
    pub rotating: bool,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteTask{
    pub id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateTask{
    pub id: i32,
    pub name: String,
    pub description: String,
    pub assigned_person: i32,
    pub rotating: bool,
}

#[derive(Deserialize, Serialize)]
pub struct DownloadStats{
    pub person_name: String,
    pub task_name: String,
}
impl SqlStatement for DownloadStats{
    fn get_query_string() -> String {
        return "SELECT * FROM \"task\".\"download_stats\" WHERE \"person_name\" LIKE $1 AND \"task_name\" LIKE $2".to_string();
    }

    fn get_client() -> DbClient {
        return DbClient::new();
    }

    fn process(&self) -> Result<Option<Vec<Row>>, String> {
        return Self::get_client().query(
            &Self::get_query_string(),
            &[&self.person_name, &self.task_name],
        );
    }
}