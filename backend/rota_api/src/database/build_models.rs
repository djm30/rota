use crate::CONFIGURATION;
use crate::endpoints::job_wrapper::{JobWrapper, Object};
use super::db_client::{DbClient, SqlStatement};
use log::{error, warn, info};
use serde::{Serialize, Deserialize};
use std::{fs, io};

#[derive(Deserialize, Serialize)]
pub struct Build {
    pub drop: bool,
    pub build: bool,
    pub update: bool,
}
impl Build {
    fn get_files_contents(path: &str) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();

        Self::get_files_in_path(&mut files, path);

        let mut queries: Vec<String> = Vec::new();
        files.iter().for_each(|file|{
            queries.push(Self::get_file_contents(file));
        });

        return queries;
    }

    // this is fucking magic, no clue how I got this to work
    fn get_files_in_path(paths: &mut Vec<String>, path_to_check: &str){    
        let mut files: Vec<String> = Vec::new();

        let all_paths: Vec<String> = match fs::read_dir(path_to_check){
            Ok(val) => {
                val.map(|paths| paths.map(|path| path.path()            
                    .into_os_string()
                    .into_string()
                    .unwrap()))
                .collect::<Result<Vec<String>, io::Error>>().unwrap()
            }
            Err(err) => {
                error!("Problem with the path: {:?}\n{:?}", path_to_check, err);
                Vec::new()
            }
        };

        all_paths
        .iter()
        .for_each(|path|
            if path.contains('.'){
                files.push(path.clone());
            }else{
                Self::get_files_in_path(paths, path);            
            }
        );
        
        for file in files{
            info!("Found the file: {}", file);
            paths.push(file);
        }
    }

    fn get_file_contents(path: &str) -> String{

        match fs::read_to_string(path){
            Ok(val) => {
                info!("Reading from {}", path);
                return val;
            },
            Err(err) =>{
                error!("Error reading from {}: {:?}", path, err);
                panic!();
            },
        }
    }
}
impl SqlStatement for Build {
    fn get_query_string() -> String {
        todo!()
    }   

    fn get_client() -> DbClient {
        return DbClient::new();
    }

    fn process(&self, response: &mut JobWrapper){
        info!("Constructing database...\n\tdrop: {:?}\n\tbuild {:?}\n\tupdate {:?}", self.drop, self.build, self.update);        

        let mut queries: Vec<String>;
        let mut failed: bool;
        
        if self.drop {
            info!("Performing drop");
            queries = Self::get_files_contents(&CONFIGURATION.database.scripts.drop);

            failed = false;
            queries.iter().for_each(|query|  {
                if Self::get_client().batch_execute(query).is_err() {
                    failed = true;
                }
            });
            if failed {
                response.add_err("Drop failed");                
            }
        }        
        if self.build {
            info!("Performing build");
            queries = Self::get_files_contents(&format!("{}\\schemas", &CONFIGURATION.database.scripts.build));
            queries.append(&mut Self::get_files_contents(&format!("{}\\public", &CONFIGURATION.database.scripts.build)));

            queries.append(&mut Self::get_files_contents(&format!("{}\\person\\tables", &CONFIGURATION.database.scripts.build)));
            queries.append(&mut Self::get_files_contents(&format!("{}\\person\\functions", &CONFIGURATION.database.scripts.build)));

            queries.append(&mut Self::get_files_contents(&format!("{}\\task\\tables", &CONFIGURATION.database.scripts.build)));
            queries.append(&mut Self::get_files_contents(&format!("{}\\task\\functions", &CONFIGURATION.database.scripts.build)));
            queries.append(&mut Self::get_files_contents(&format!("{}\\task\\views", &CONFIGURATION.database.scripts.build)));

            failed = false;
            queries.iter().for_each(|query|  {
                if Self::get_client().batch_execute(query).is_err() {
                    failed = true;                    
                }
            });
            if failed {
                response.add_err("Build failed");
            }
        }
        if self.update {
            warn!("Updates should only be performed once then incorporated into the build scripts");
            queries = Self::get_files_contents(&CONFIGURATION.database.scripts.update);

            failed = false;
            queries.iter().for_each(|query|  {
                if Self::get_client().batch_execute(query).is_err() {
                    failed = true;
                }
            });
            if failed {
                response.add_err("Update failed");                
            }
        }
    }
}