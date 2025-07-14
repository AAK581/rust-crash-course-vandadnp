#![deny(clippy::all)]
#[derive(Debug)]



//const MY_AGE: u8 = 23;

// fn greet(name: &String) {
//     println!("Hello, {name}");
// }

// fn empty_string(value: &mut String) {
//     value.clear();
// }

// fn get_name() -> &String { // Returns a reference to a string, not working. expected named lifetime parameter
//     &"John".to_string();
// }

// fn say_hello_world() -> String {
//     String::from("Hello, world!")
// }

// fn say_hello_world(message: &str) {
//     println!("{message}");
// }

// fn say_hello_world(to_person: String) -> String {
//     format!("Hello {to_person}")
// }

// fn process_name(name: &str, callback_test: fn(&str)) {  //functions can be passed as arguments
//     callback_test(name);
// }

//////////////////////////

// struct Person {
//     name: String,
//     age: u8,
// }

// fn create_person(name: String, age: u8) {
//     let person = Person { name, age };
// }

struct Point(f64, f64, f64);

impl Point {
    fn describe(&self) {
        println!("x = {}, y = {}, z = {}", self.0, self.1, self.2);
    }
    fn twice(&self) -> Point {
        Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
    }
    fn make_twice(&mut self) {
        self.0 *= 2.0;
        self.1 *= 2.0;
        self.2 *= 2.0;
    }
    fn zero() -> Point {
        Point(0.0, 0.0, 0.0)
    }
}

fn main() {
    // let mut name = "John";
    // println!("Hello, {name}!");
    // name = "Walker";
    // println!("Hello, {name}!");
    //////////////////////////
    // let data = "Foo";
    // {
    //     let data = data.to_string();
    // }
    //////////////////////////
    // println!("My age is {MY_AGE}.");
    // let personal_data = (22, "John");
    // let (age, name) = personal_data;
    // let age_1 = personal_data.0;
    //////////////////////////
    // let name1 = String::from("John");
    // let name2 = name1;
    // println!("Hello, {name1}"); //borrow of moved value: `name1`
    // println!("Hello, {name2}");
    //////////////////////////
    // {
    //     let name = "John".to_string();
    //     println!("Hello, {name}");
    // }
    // //name; //cannot find value `name` in this scope
    //////////////////////////
    // let s1 = String::from("John");
    // let s2 = &s1; //referencing
    // greet(&s1);
    // greet(s2);
    //////////////////////////
    // let mut name = String::from("Walker");
    // empty_string(&mut name);
    //////////////////////////
    // let mut name = String::from("John");
    // let mut name2 = &mut name;
    // let mut name3 = &mut name; //error: second mutable borrow occurs here (This is to prevent data races)
    // empty_string(&mut name2);
    //////////////////////////
    // let mut name1 = String::from("John");
    // let name2 = &name1;
    // let mut name3 = &mut name1; //No mutable references while immutable references are there
    // println!("{name1}");
    // println!("{name2}");
    /////////////////////////
    // let message = say_hello_world();
    // println!("{message}");
    /////////////////////////
    //say_hello_world("Helloooo");
    /////////////////////////
    // let message = say_hello_world(String::from("Doma"));
    // println!("{message}");
    /////////////////////////
    // let say_hello_to = |name: &str| format!("Hello, {name}");
    // println!("{}", say_hello_to("Doma"));
    /////////////////////////
    // let full_name = |first_name: &str, last_name: &str| format!("{first_name} {last_name}");
    // println!("{}", full_name("Adham", "Ahmed"));
    /////////////////////////
    // let multiply_by_2 = |x:i32| x * 2;
    // println!("{}", multiply_by_2(5));
    /////////////////////////
    // let ask_for_age = || {
    //     // ask for age
    //     // calculate age in 10 years
    // };
    /////////////////////////
    // let multiply_by_2 = |x:i32| x * 2;
    // let ptr = multiply_by_2; //pointing to a function
    // let result = ptr(10);
    // println!("{result}");
    /////////////////////////
    // let person = Person {
    //     name: "John".to_string(),
    //     age: 30,
    // };
    // //println!("{} is {} years old.", person.name, person.age);
    // // let person_2 = Person {
    // //     name: "Doe".to_string(),
    // //     age: person.age,
    // // };
    // let person_2 = Person {
    //     name: "Doe".to_string(),
    //     ..person
    // };
    /////////////////////
    // let point = Point(0.0, 0.1, 0.2);
    // println!("x = {}, y = {}, z = {}", point.0, point.1, point.2); //x = 0, y = 0.1, z = 0.2
    ////////////////////
    // let p = Point(1.0, 2.0, 3.0);
    // p.describe(); //x = 1, y = 2, z = 3
    // println!("{p:?}");   // Using the debug trait
    ////////////////////
    // let mut point = Point(1.0, 2.0, 3.0);
    // let twice = point.make_twice();
    ////////////////////
    let point = Point::zero();
}
