#[cfg(test)]
mod test_object{
    use std::vec;
    use crate::endpoints::job_wrapper::{Object, Data};

    #[test]
    fn key_val_str(){                
        let mut obj = Object::new_str("key", "val");
        println!("{}", obj.format());
        assert_eq!(obj.format(), r#""key": "val""#);
    }
    #[test]
    fn key_val_i32(){
        let mut obj = Object::new_i32("key", 20);
        println!("{}", obj.format());
        assert_eq!(obj.format(), r#""key": 20"#);
    }
    #[test]
    fn key_val_bool(){
        let mut obj = Object::new_bool("key", true);
        println!("{}", obj.format());
        assert_eq!(obj.format(), r#""key": true"#);
    }

    #[test]
    fn key_vec_str(){
        let mut obj = Object::new_vec_str("key", vec!["val1", "val2"]);
        println!("{}", obj.format());
        assert_eq!(obj.format(), r#""key": ["val1","val2"]"#);
    }
    #[test]
    fn key_vec_i32(){
        let mut obj = Object::new_vec_i32("key", vec![52, 4, -1]);
        println!("{}", obj.format());
        assert_eq!(obj.format(), r#""key": [52,4,-1]"#);
    }
    #[test]
    fn key_vec_bool(){
        let mut obj = Object::new_vec_bool("key", vec![true, false]);
        println!("{}", obj.format());
        assert_eq!(obj.format(), r#""key": [true,false]"#);
    }
    
    #[test]
    fn nested_object(){
        let mut node = Object::new("master");

        let bools = Object::new_vec_bool("bools", vec![true, false]);
        node.add_val(Box::new(bools));

        let ints = Object::new_vec_i32("ints", vec![52, 4, -1]);
        node.add_val(Box::new(ints));

        let strs = Object::new_vec_str("strs", vec!["val1", "val2"]);
        node.add_val(Box::new(strs));

        let bool = Object::new_bool("bool", true);
        node.add_val(Box::new(bool));

        let int = Object::new_i32("int", 42);
        node.add_val(Box::new(int));

        let str = Object::new_str("str", "val");
        node.add_val(Box::new(str));

        let mut sub_node = Object::new("person");
        sub_node.add_val(Box::new(Object::new_str("name", "joe")));
        sub_node.add_val(Box::new(Object::new_str("surname", "bloggs")));
        sub_node.add_val(Box::new(Object::new_vec_str("fav_colours", vec!["green", "red", "blue"])));
        sub_node.add_val(Box::new(Object::new_bool("wears_glasses", false)));
        node.add_val(Box::new(sub_node));


        println!("{}", node.format());
        assert_eq!(node.format(), r#""master": {"bools": [true,false],"ints": [52,4,-1],"strs": ["val1","val2"],"bool": true,"int": 42,"str": "val","person": {"name": "joe","surname": "bloggs","fav_colours": ["green","red","blue"],"wears_glasses": false}}"#);
    }
}
