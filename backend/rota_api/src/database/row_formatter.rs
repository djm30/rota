use log::error;
use postgres::Row;

use crate::endpoints::job_wrapper::{Object, JobWrapper};

pub fn add_rows_to_object(rows: Vec<Row>, wrapper: &mut JobWrapper){
    rows.iter().for_each(|row| {
        add_row_to_object(row, wrapper);
    });
}

fn add_row_to_object(row: &Row, wrapper: &mut JobWrapper) {
    for (col_index, col) in row.columns().iter().enumerate() {
        let col_type: &str = &col.type_().to_string();
        let col_name: &str = col.name();
        
        match col_type{
            "varchar" => wrapper.add_data(Box::new(Object::new_str(
                            col_name, 
                            row.get::<usize, &str>(col_index)))
            ),
            "int4" => wrapper.add_data(Box::new(Object::new_i32(
                            col_name, 
                            row.get::<usize, i32>(col_index)))
            ),
            "bool" => wrapper.add_data(Box::new(Object::new_bool(
                            col_name, 
                            row.get::<usize, bool>(col_index)))
            ),
            "_varchar" => wrapper.add_data(Box::new(Object::new_vec_str(
                            col_name, 
                            row.get::<usize, Vec<&str>>(col_index)))
            ),
            "_int4" => wrapper.add_data(Box::new(Object::new_vec_i32(
                            col_name, 
                            row.get::<usize, Vec<i32>>(col_index)))
            ),
            _ => {
                wrapper.add_err(&format!("Cannot convert db type, Implement the type: {}", col_type));
                error!("Implement the type: {}", col_type);
            },
        }
    };
}