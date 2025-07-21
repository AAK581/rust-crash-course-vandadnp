#![deny(clippy::all)]
// #[derive(Debug)]

use std::fmt;

// // fn get_user_name() -> Result<String, ()> {
// //     //Err(()) //crashes with this error: I was expecting a username: ()
// //     Ok(String::from("John"))
// // }

// fn get_first_name() -> Result<String, String> {
//     Err("No first name".to_string())
// }

// fn get_last_name() -> Result<String, String> {
//     Ok("Doe".to_string())
//     //Err(()) //makes the match print Errorrr without crashing the application
// }

// fn get_full_name() -> Result<String, String> {
//     let first_name = get_first_name()?;
//     let last_name = get_last_name()?;
//     Ok(format!("{first_name} {last_name}"))
// }

///////////////
// fn _get_full_name() -> &'static str { //this causes it to live for the full app
//     "John"
// }

// fn get_random_name<'l1, 'l2>(a: &'l1 str, b: &'l2 str) -> &'l1 str { //two variables, one lives as long as l1 and the other lives as long as l2, the return lives as long as l1 (lifetime1)
//     a
// }

// fn get_random_name<'l>(a: &'l str, b: &'l str) -> &'l str { 
//     a
// }

// struct Person<'a> {
//     // name: &str, //ERROR: expected named lifetime parameter
//     name: &'a str, //the name has the same lifetime as the struct, duh
// }

// fn _get_first_name(full_name: &str) -> &str { //the compiler automatically assigns lifetime operators to both the parameter and the result
//     full_name
// }

// struct _Person<'a> { //those 
//     first_name: &'a str,
//     last_name: &'a str,
// }

// impl<'a> _Person<'a> {
//     fn _first_char_of_first_name(&self) -> &str {
//         &self.first_name[0..1]
//     }
//     fn _get_full_name(&self) -> String {
//         format!("{} {}", self.first_name, self.last_name)
//     }
// }

// enum _Animal<'a> {
//     Dog {name: &'a str},
//     Cat {name: &'a str}
// }

struct Person {
    first_name: String,
    last_name: String,
    age: u8
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} is {} years old", self.first_name, self.last_name, self.age)
    }
}

trait _HasFullName {
    fn full_name(&self) -> String;
}

impl _HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

trait _HasName {
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
}

impl _HasName for Person {
    fn first_name(&self) -> &str {
        &self.first_name
    }
    fn last_name(&self) -> &str {
        &self.last_name
    }
}

trait _HasFullName1 where Self: _HasName {
    fn full_name(&self) -> String;
}

impl<T> _HasFullName1 for T where T: _HasName {
    fn full_name(&self) -> String {
    format!("{} {}", self.first_name(), self.last_name())
}  
}

fn _print_full_name_and_age(value: &impl _HasFullName) {
    println!("{}", value.full_name())
}


// fn _print_details<T: _HasFullName + _CanRun>(value: &T) {
//     println!("{}", value.full_name());
//     value.run();
// }

fn _print_details<T>(value: &T) where T: _HasFullName + _CanRun { //better readability
    println!("{}", value.full_name());
    value.run();
}

trait _CanRun {
    fn run(&self);
}

impl _CanRun for Person {
    fn run(&self) {
        //todo
    }
}
trait CanInitializeWithFullName {
    fn new(full_name: &str) -> Self;
}

impl CanInitializeWithFullName for Person {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(' ').collect(); //turns string slices to vectors
        Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age: 23
        }
    }
}

fn main() {
    // let value: Result<&str, Box<dyn std::error::Error>> = Ok("Hello"); //&str, Box means either a string slice or a box in the heap which contains the Error shown
    // match value {
    //     Ok(value) => println!("{value}"),
    //     Err(error) => println!("{error}")
    // }
    // let value1: Result<&str, ()> = Err(()); //&str, () means either a string slice or nothing
    // match value1 {
    //     Ok(value1) => println!("{value1}"),
    //     Err(_) => println!("Void error")
    // }
    // let user_name = get_user_name();
    // let unwrapped = user_name.expect("I was expecting a username");
    // println!("{unwrapped}"); //prints John
    // get_user_name().expect_err("Trying to expect an error"); //panics and says this: Trying to expect an error: "John"
    // let is_ok = get_user_name().is_ok(); //true
    // let is_error = get_user_name().is_err(); //false
    //let full_name = get_full_name()?; //the ? is used so we don't need to manually unwrap it
    // match full_name {
    //     Ok(name) => println!("Hello, {name}!"),
    //     Err(_) => println!("Errorrr")
    // }
    // let length = full_name.map(|s| s.len()).unwrap_or_default(); //if it's okay it will go normally, uint length, otherwise it will give the default string length value, 0
    // println!("{length:?}"); //8
    // let error_length = full_name.map_err(|e| e.len());
    // println!("{error_length:?}"); //after changing the return types of the 3 functions (first, last, and full name) to <String, String>, this prints Err(13)
    /////////////////////////
    // let name = get_random_name("John", "Doe");
    // println!("{name}");
    // let person = Person {
    //     first_name: "John".to_string(),
    //     last_name: "Doe".to_string(),
    //     age: 30
    // };
    //println!("{person:?}");
    let person1 = Person::new("Adham Ahmed");
    //println!("{person1:?}");
    //println!("First name is {}, last name is {}, and age is {}", person1.first_name, person1.last_name, person1.age);
    //println!("{person1}");
    _print_full_name_and_age(&person1);
    let full_name = _HasFullName1::full_name(&person1);
    println!("{full_name}");
}