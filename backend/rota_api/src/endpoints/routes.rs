use actix_web::{delete, get, post, put, web, HttpResponse, HttpRequest};
use log::{error, warn, info};
use postgres::Row;
use serde_json::Value;


use crate::database::db_client::SqlStatement;
use crate::database::build_models::Build;
use super::json_formatter::rows_to_body;

#[get("/health")]
async fn get_health() -> HttpResponse{
    return HttpResponse::Ok().finish();
}

#[post("/construct_database")]
async fn construct_database(payload: web::Json<Build>) -> HttpResponse{
    let response = format_db_response(payload.process());
    return response;
}

fn format_db_response(db_response: Result<Option<Vec<Row>>, String>) -> HttpResponse{
    // Check if the response has an error
    match db_response {
        Ok(response) => {
            // Check if json needs to be returned
            match response{
                Some(rows) => {
                    // Format body from rows
                    let body: String = rows_to_body(rows);
                    println!("{}", body);                    
                    let json_body: Result<Value, serde_json::error::Error> = serde_json::from_str(&body);
                    match json_body {
                        Ok(body) => return HttpResponse::Ok().json(body),
                        Err(err) => {
                            let msg = format!("Problem parsing following json: \n{}\n{:?}", &body, err);
                            error!("{}", msg);
                            return HttpResponse::InternalServerError().json(msg);
                        }                        
                    }
                },
                None => return HttpResponse::Ok().finish(),
            }            
        },
        Err(message) => return HttpResponse::UnprocessableEntity().json(message)
    }
}

pub fn init_endpoints(service_config: &mut web::ServiceConfig) {
    service_config.service(get_health);
    service_config.service(construct_database);
}