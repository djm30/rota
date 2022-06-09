use std::vec;
use actix_web::dev::HttpResponseBuilder;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;

pub struct Response{
    pub data: Object,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub status: StatusCode,
}
impl Response{
    pub fn set_data(&mut self, data: Object){
        self.data = data;
    }

    pub fn add_err(&mut self, err: &str){
        self.errors.push(err.to_string());
    }
    pub fn add_warn(&mut self, warn: &str){
        self.errors.push(warn.to_string());
    }

    pub fn set_status(&mut self, status: StatusCode){
        self.status = status;
    }

    
    pub fn get_http_response(&mut self) -> HttpResponse{
        let mut err: Object = Object::new_vec_str("errors", self.errors.iter().map(AsRef::as_ref).collect());
        let mut warn: Object = Object::new_vec_str("warnings", self.warnings.iter().map(AsRef::as_ref).collect());

        let json = format!("{{{},{},{}}}", self.data.format(), err.format(), warn.format());

        return HttpResponseBuilder::new(self.status).json(json);
    }
}


pub trait Data {
    fn format(&mut self) -> String;
}

pub struct Object{
    key: String,
    container: bool,
    val: Vec<Box<dyn Data>>,
}
impl Data for Object{
    fn format(&mut self) -> String { 
        if self.val.len() == 0{
            return "".to_string();
        }else{
            let s = &self.val.iter_mut().map(|obj|{
                obj.format()
            }).collect::<Vec<String>>()
            .join(",");

            if self.container{
                return format!("\"{}\": {{{}}}", self.key, s); 
            } else{
                return format!("\"{}\": {}", self.key, s); 
            }
        }
    }
}
impl Object {
    pub fn new(key: &str) -> Self{
        return Self {
            key: key.to_string(),
            container: true,
            val: vec![],
        };
    }

    pub fn new_str(key: &str, val: &str) -> Self{
        return Self { 
            key: key.to_string(),
            container: false,
            val: vec![Box::new(Value::new_str(val))],
        };
    }
    pub fn new_i32(key: &str, val: i32) -> Self{
        return Self { 
            key: key.to_string(),
            container: false,
            val: vec![Box::new(Value::new_i32(val))],
        };
    }
    pub fn new_bool(key: &str, val: bool) -> Self{
        return Self { 
            key: key.to_string(),
            container: false,
            val: vec![Box::new(Value::new_bool(val))],
        };
    }

    pub fn new_vec_str(key: &str, val: Vec<&str>) -> Self{        
        return Self { 
            key: key.to_string(),
            container: false,
            val: vec![Box::new(ValueArray::new_str(val))] 
        };
    }
    pub fn new_vec_i32(key: &str, val: Vec<i32>) -> Self{        
        return Self { 
            key: key.to_string(),
            container: false,
            val: vec![Box::new(ValueArray::new_i32(val))] 
        };
    }
    pub fn new_vec_bool(key: &str, val: Vec<bool>) -> Self{        
        return Self { 
            key: key.to_string(),
            container: false,
            val: vec![Box::new(ValueArray::new_bool(val))] 
        };
    }

    pub fn add_val(&mut self, val: Box<dyn Data>) {
        if !self.container{
            panic!("You cannot add to this object, make one with the container value set to true")
        }
        self.val.push(val);
    }
}

struct Value{
    val: String
}
impl Data for Value{
    fn format(&mut self) -> String {
        return self.val.clone();
    }
}
impl Value {
    pub fn new_str(val: &str) -> Self{
        return Value { val: format!("\"{}\"", val) };
    }
    pub fn new_i32(val: i32) -> Self{
        return Value { val:  val.to_string() };
    }
    pub fn new_bool(val: bool) -> Self{
        return Value { val:  val.to_string() };
    }
}

struct ValueArray{
    val: Vec<String>
}
impl Data for ValueArray{
    fn format(&mut self) -> String {        
        return format!("[{}]", self.val.join(","));
    }
}
impl ValueArray{    
    pub fn new_str(val: Vec<&str>) -> Self{        
        return ValueArray { 
            val: val.iter().map(|s| format!("\"{}\"", s)).collect()
        };
    }
    pub fn new_i32(val: Vec<i32>) -> Self{        
        return ValueArray { 
            val: val.iter().map(|s| s.to_string()).collect()
        };
    }
    pub fn new_bool(val: Vec<bool>) -> Self{        
        return ValueArray { 
            val: val.iter().map(|s| s.to_string()).collect()
        };
    }
}