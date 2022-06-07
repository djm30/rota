mod config;
mod endpoints;
mod database;

use actix_web::{App, HttpServer};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use chrono;
use log::{error, warn, info, trace};
use log::{LevelFilter};
use log::{Record, Level, Metadata};

#[macro_use]
extern crate lazy_static;


struct SimpleLogger;
impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let level: Level = match CONFIGURATION.logging.level.as_str() {
            "trace" => Level::Trace,
            "info" => Level::Info,
            "warn" => Level::Warn,
            "error" => Level::Error,            
            _ => panic!("{} {:?} - '{}' {}", Level::Error, chrono::offset::Local::now(), CONFIGURATION.logging.level, "is an invalid log level"),
        };
        metadata.level() <= level
    }
    fn log(&self, record: &Record) {      
        if self.enabled(record.metadata()) {
            println!("{} {:?} - {}", record.level(), chrono::offset::Local::now(), record.args());
            if CONFIGURATION.logging.log_to_file {
                let log_file: String = format!("{}{}.log", CONFIGURATION.logging.file_path,  chrono::offset::Utc::now().date().to_string());                
                
                let mut file = match OpenOptions::new()
                .read(false)
                .write(true)
                .append(CONFIGURATION.logging.append_file)                
                .create(true)
                .open(&log_file){
                    Ok(res) => res,
                    Err(err) => panic!("{}", err),
                };                
                
                match writeln!(file, "{} {:?} - {}", record.level(), chrono::offset::Utc::now(), record.args()){
                    Ok(_) => (),
                    Err(err) => panic!("{}", err),
                };
                
            }
        }
    }
    fn flush(&self) {}
}
static LOGGER: SimpleLogger = SimpleLogger;


lazy_static! {
    pub static ref CONFIGURATION: config::Config = {
        let config_path: &str =
            "cfg.json";

        let config: &str =
            &fs::read_to_string(config_path).expect("Something went wrong reading the file");

        println!("{}", config);

        let config: config::Config =
            serde_json::from_str(config).expect("JSON was not well-formatted");       

        return config;
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_log_file();

    // logger checks
    error!("This is an 'error' log");
    warn!("This is a 'warn' log");
    info!("This is an 'info' log");   
    trace!("This is an 'info' log");   

    let server = HttpServer::new(|| App::new().configure(endpoints::init_endpoints));
    
    server.bind((CONFIGURATION.address.clone(), CONFIGURATION.port.clone()))?.run().await
}

fn setup_log_file(){    
    // setting the logger
    match log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace)) {
        Ok(_) => info!("The logger was sucessfully set"),
        Err(err) => error!("The logger failed to be set {:?}", err),
    }
}