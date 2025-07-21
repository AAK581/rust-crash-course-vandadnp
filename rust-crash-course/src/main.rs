#![deny(clippy::all)]
//#[derive(Debug)]
//#[derive(PartialEq)] // to equate an enum with an instance of it
// use std::collections::HashMap;
// #[derive(Hash, Eq, PartialEq, Debug)]

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

// struct Point(f64, f64, f64);

// impl Point {
//     fn describe(&self) {
//         println!("x = {}, y = {}, z = {}", self.0, self.1, self.2);
//     }
//     fn twice(&self) -> Point {
//         Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
//     }
//     fn make_twice(&mut self) {
//         self.0 *= 2.0;
//         self.1 *= 2.0;
//         self.2 *= 2.0;
//     }
//     fn zero() -> Point {
//         Point(0.0, 0.0, 0.0)
//     }
// }

// enum AnimalType {
//     Dog,
//     Cat,
//     Rabbit,
// }

// struct Size {
//     width: f32,
//     height: f32,
// }

// enum Shapes {
//     Circle(f32, f32, f32),
//     Rectangle { width: f64, height: f64 },
//     Paralellogram(f32, f32, Size),
// }

// impl Shapes {
//     fn area(&self) -> f32 {
//         match self {
//             Shapes::Paralellogram(x, y, size) => size.width * size.height,
//             Shapes::Circle(x, y, radius) => 3.14 * radius * radius,
//             _ => 1.0

//         }
//     }
// }

// enum Pet {
//     Cat {name: String},
//     Dog {name: String},
// }

// fn get_values() -> (String, String, i32) {
//     ("hello".to_string(), "World".to_string(), 25)
// }

/////////////
// struct Person {
//     name: String,
//     age: u8,
// }

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
    //let point = Point::zero();
    ////////////////////
    // let fluffy = AnimalType::Cat;
    // // if fluffy == AnimalType::Cat { //not efficient
    // //     println!("It's a cat!");
    // // }
    // match fluffy {
    //     AnimalType::Dog => println!("Woof!"),
    //     // AnimalType::Cat => println!("Meow!"),
    //     // AnimalType::Rabbit => println!("What's up doc!"),
    //     _ => println!("Something else"),
    // }
    /////////////////
    // let rectangle = Shapes::Rectangle {
    //     width: 3.0,
    //     height: 4.0,
    // };

    // if let Shapes::Rectangle { width, height } = rectangle {
    //     println!("Width = {width}, height = {height}");
    // }

    // match rectangle {
    //     Shapes::Rectangle { width, height } => {
    //         println!("Width = {width}, height = {height}");
    //     }
    //     _ => println!("Not a rectangle"),
    // }

    // let paralellogram = Shapes::Paralellogram(
    //     1.0,
    //     2.0,
    //     Size {
    //         width: 3.0,
    //         height: 4.0,
    //     },
    // );
    // if let Shapes::Paralellogram(x, y, Size{width, height}) = paralellogram {
    //     println!("{x} {y} {width} {height}")
    // }
    // match paralellogram {
    //     Shapes::Paralellogram(x, y, Size{width, height}) => {
    //         println!("A match with {x} {y} {width} {height}")
    //     }
    //     _ => println!("Not a match")
    // }

    // let area = paralellogram.area();
    // println!("Area is {area}");
    ///////////////////
    // let fluffy = Pet::Cat{name: "Fluffy".to_string()};
    // let name = match fluffy {
    //     Pet::Cat {name} => name,
    //     Pet::Dog {name} => name,
    // };
    // println!("{name}");
    /////////////
    // let values = ("Hello", "World", 30);
    // let hello = values.0;
    // let (hello1, world, age) = values;
    // let (_, _, age1) = values;
    // let (hello2, _, _) = get_values();
    /////////////// VECTORSSS
    // let values: [&str; 2] = ["foo", "bar"]; //fixed sized vector of string slices
    // for value in values.iter() {
    //     println!("{value}");
    // }
    // let foo1 = &values[0];
    // println!("Foo is {foo1}");
    // let length = values.len();
    // println!("Length is {length}");
    // //let values: [i32; 2] = [10, 20];
    // //let doubles = values.iter().map(|x: &i32| x*2);
    // let mut values = vec![100, 200]; //you can use an array directly
    // values.push(300);
    // //let threehnd = values.pop();
    // println!("Values are {values:?}");
    // //values.clear();
    // values.extend_from_slice(&[400, 500]); //pushing a whole vector, basically
    // println!("Values are {values:?}");
    // let mut values2 = vec![4, 5, 6];
    // values.append(&mut values2);
    // println!("Values = {values:?}");
    // println!("Values2 = {values2:?}");
    // if values.contains(&200) {
    //     println!("200 is contained");
    // }
    // else {
    //     println!("200 is not contained");
    // }
    // if values.is_empty() {
    //     println!("Values is empty");
    // }
    ////////////
    // let mut values: HashMap<&str, &str> = HashMap::new();
    // values.insert("foo", "bar");
    // println!("{values:?}");
    // if values.contains_key("name") {
    //     println!("Name exists");
    // } else {
    //     println!("Name doesn't exist");
    // }
    // let bar = values["foo"];
    // println!("Bar has {bar}");
    // //values.remove("foo");
    // //println!("{values:?}");
    // match values.get("foo") {
    //     Some(value) => println!("{value}"),
    //     None => println!("Not found"),
    // }
    // for (&k, &v) in &values {
    //     println!("{k} {v}");
    // }
    // let entry = values.entry("foo");
    // match entry {
    //     std::collections::hash_map::Entry::Occupied(value) => {
    //         println!("{value:?}");
    //     }
    //     _ => {
    //         println!("Noooot found");
    //     }
    // }
    // values.insert("husband", "John Doe");
    // values.entry("wife").or_insert("Jane Doe"); //if there's no name entry, create this one
    // println!("{values:?}");
    /////////////
    //let mut values: HashMap<Person, &str> = HashMap::new();
    /////////////
    // let mut values = vec![1, 2, 3, 4, 5];
    // values.push(6);
    // for value in values.iter() {
    //     println!("{value}");
    // }
    // let iter = values.iter();
    // let sum1: i32 = iter.sum();
    // println!("{sum1}");
    // let values_d: Vec<i32> = values.iter().map(|v| v*2).collect();
    // println!("{values_d:?}");

    // let mut names = vec!["John", "Jane", "Tom"];
    // names.push("Bro");
    // for name in names.into_iter() {
    //     if name.len() != 3 {
    //         break;
    //     }
    //     println!("{name}");
    // }
    // for name in names.into_iter().filter(|name| name.len() == 3) {
    //     println!("{name}");
    // }
    ////////////////
    // let value = Some(10);
    // let name: Option<&str> = Some("John");
    // let name: Option<&str> = None;
    // //let unwrapped_name = name.expect("Name was not provided"); thread 'main' panicked at src\main.rs:361:31: Name was not provided
    // //println!("Name is {unwrapped_name}");
    // match name {
    //     Some(name) => println!("Hello, {name}"),
    //     None => println!("None, no name")
    // }
    // let mut age: Option<i8> = Some(20);
    // match age.as_mut() {
    //     Some(age) => *age += 1,
    //     None => println!("No age")
    // }
    // println!("Age is {}", {age.unwrap()});
    //////////////////
    // let age1 = Some(20);
    // let age2 = Some(20);
    // let age3 = Some(20);
    // if let (Some(age_1), Some(age_2), Some(age_3)) = (age1, age2, age3) {
    //     println!("{}", age_1 + age_2 + age_3);
    // }

    // let name:Option<&str> = None;
    // // let unwrapped: &str = name.unwrap_or("Doe");
    // // println!("{unwrapped}");
    // // let unwrapped: &str = name.unwrap_or_else(|| {
    // //     "John"
    // // });
    // // println!("{unwrapped}");
    // if name.is_some() {
    //     println!("There is value in name");
    // } else {
    //     println!("There is no value in name");
    // }

    // if name.is_none() {
    //     println!("There is still no value in name");
    // } else {
    //     println!("There is actually some value in name");
    // }
    //////////////////
    // let age: Option<i32> = None;
    // let age = age.unwrap_or_default();
    // println!("{age}"); //prints 0 which is the default value for i32
    let age: Option<i32> = Some(20);
    let age_multiplied_by_2 = age.map(|age| age * 2);
    println!("{}", age_multiplied_by_2.unwrap_or_default()); //40
    //CHAPTER 9
}