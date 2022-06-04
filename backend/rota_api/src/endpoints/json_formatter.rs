use log::error;
use postgres::Row;


fn row_to_key_value(row: &Row) -> Vec<String>{
    let mut object: Vec<String> = vec![];
    // Loop through each column in the row
    for (col_index, column) in row.columns().iter().enumerate() {
        // Get the column type
        let col_type: &str = &column.type_().to_string();

        // Get column properties
        let name = column.name();
        let value: String;
        
        // Convert the value into a valid string  
        match col_type{         
            "int4" => value = row.get::<usize, i32>(col_index).to_string(),
            "varchar" => value = row.get::<usize, &str>(col_index).to_string(),
            "bool" => value = row.get::<usize, bool>(col_index).to_string(),
            "_varchar" => {
                let mut varchars: String = "[".to_string();
                let varchar_array: Vec<String> = row.get::<usize, Vec<String>>(col_index)
                    .iter()
                    .map(|item| format!("\"{}\"", item))
                    .collect();                
                varchars.push_str(&varchar_array.join(","));
                varchars.push(']');
                value = varchars;
            },
            "_int4" => {
                let mut varchars: String = "[".to_string();
                let varchar_array: Vec<String> = row.get::<usize, Vec<i32>>(col_index)
                    .iter()
                    .map(|item| format!("{}", item))
                    .collect();                
                varchars.push_str(&varchar_array.join(","));
                varchars.push(']');
                value = varchars;
            }
            _ => {
                error!("Implement the type: {}", col_type);
                panic!();
            },
        }

        // Put the column into the object
        object.push(format!("\"{}\": {}", name, value));
    }
    return object;
}

pub fn rows_to_body(rows: Vec<Row>) -> String{
    let mut objects: Vec<Vec<String>> = vec![];

    // Loop through all the rows
    for row in rows {
        let object: Vec<String> = row_to_key_value(&row);

        // Put the object into the objects vector
        objects.push(object);
    }

    // Define the body to be returned
    let mut body: String = "{".to_string();

    // Check if the body should be a json array
    if objects.len() > 1 {
        body.push('[')
    }

    // Loop through each object
    for (obj_i, object) in objects.iter().enumerate(){
        // Set up the start of a json object
        //body.push('{');

        // Loop through each name value pair in the object
        for (i, name_value) in object.iter().enumerate(){
            // Check if the current name value pair is the last one and needs a comma at the end 
            if i == (object.len() - 1) {
                body.push_str(name_value);
            }
            else {
                body.push_str(&format!("{}{}", name_value, ","));
            }
        }

        // Check if the current object is the last one and needs a comma at the end
        if obj_i == (objects.len() - 1) {
            //body.push('}');
        }
        else {
            body.push_str(",");
        }
    }
    // If the body is a json array and therefore needs to be closed
    if objects.len() > 1 {
        body.push(']');
    }
    body.push('}');
    
    return body;
}