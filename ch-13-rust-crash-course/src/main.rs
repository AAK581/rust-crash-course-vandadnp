#![deny(clippy::all)]


// use std::ops::Deref;
//use std::rc::Rc;
//use std::cell::Cell;
//use std::cell::RefCell;

use std::ops::AddAssign;

// struct BoxedValue<T> {
//     value: T,
// }

// impl<T> BoxedValue<T> {
//     fn new(value: T) -> Self {
//         BoxedValue { value }
//     }
// }

// impl<T> Deref for BoxedValue<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }

// fn print_integer(value: &i32) {
//     println!("{value}");
// }

///////////////////

// struct Person {
//     name: String,
//     age: Cell<u8>, //NOTE: It allows mutability
// }

// impl Person {
//     fn increment_age(&self) -> u8 {
//         self.age.set(self.age.get() + 1); //Didn't need to declare it as mutable
//         self.age.get()
//     }
// }




// struct Point<T> {
//     x: T,
//     y: T,
// }

// // impl<T> Point<T> {
// //     fn move_offset(&mut self, x: T, y: T) {
// //         self.x += x; //binary assignment operation `+=` cannot be applied to type `T`
// //         self.y += y;
// //     }
// // }

// impl<T> Point<T> {
//     fn move_offset(&mut self, x: T, y: T) where T: AddAssign {
//         self.x += x;
//         self.y += y;
//     }
// }

// impl<T: AddAssign> AddAssign for Point<T> { //so we can do p1 += p2
//     fn add_assign(&mut self, other: Self) {
//         self.x += other.x;
//         self.y += other.y;
//     }
// }

// impl<T: PartialEq> PartialEq for Point<T> { //so we can do p1 == p2
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

trait CanRun {
    fn run(&self);
}

trait CanWalk {
    fn walk(&self);
}

impl<T: CanRun> CanRun for Vec<T> {
    fn run(&self) {
        for item in self {
            item.run();
        }
    }
}

impl<T: CanWalk> CanWalk for Vec<T> {
    fn walk(&self) {
        for item in self {
            item.walk();
        }
    }
}

struct Person {
    name: String,
}

impl CanWalk for Person {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}

impl CanRun for Person {
    fn run(&self) {
        println!("{} is running", self.name);
    }
}

struct Elephant {
    name: String,
}

impl CanWalk for Elephant {
    fn walk(&self) {
        println!("{} is walking", self.name);
    }
}

fn main() {
    // let age = BoxedValue::new(22);
    // //let _twice = *_age * 2;
    // //println!("{_twice}");
    // //println!("Value  is {_*age}"); //can't be referenced
    // println!("Value  is {}", *age); //i32
    // let actual_age = age.deref(); //&i32
    // println!("Actual value  is {}", *actual_age);
    // let _other = *(age.deref()); //i32

    // let value = BoxedValue::new(10);
    // print_integer(&value);
    /////////////////////
    // let array = vec!["John".to_string(), "Jane".to_string()];
    // let rc = Rc::new(array);
    // // let weak = Rc::downgrade(&rc); //as soon as rc drops what it's referencing, this weak thing will drop it too
    // // drop(rc);
    // // //let value = weak.upgrade().unwrap();
    // // //println!("{value:?}"); //panics, called `Option::unwrap()` on a `None` value
    // // match weak.upgrade() {  //prints None
    // //     Some(rc) => println!("{rc:?}"),
    // //     None => println!("None")
    // // }
    // ///////////
    // //let rc2 = rc.clone();
    // let rc2 = Rc::clone(&rc); //this and what's above it works
    // drop(rc);
    // println!("{rc2:?}");
    ////////////////////
    // let person = Person {
    //     name: "John".to_string(),
    //     age: Cell::new(20),
    // };
    // let new_age = person.increment_age();
    // let person_age = person.age.get();
    // println!("{new_age}");
    // println!("{person_age}");
    ////////////////////
    // let ref_cell = RefCell::new(vec![1, 2, 3]); //You can either borrow it immutably several times, or only once mutably
    // let mut mut_ref = ref_cell.borrow_mut();
    // //let len = ref_cell.borrow().len(); //panics, already mutably borrowed: BorrowError
    // mut_ref.push(100);

    ///////////////////////
    /////// GENERICS //////////////

    // let p1 = Point {x: 0, y: 7};
    // let p2 = Point {x: 0.0, y: 7.0};
    // let p3 = Point {x: "Foo", y: "Bar"};
    // let mut p1 = Point {x: 1.0, y: 2.0};
    // let p2 = Point {x: 3.0, y: 4.0};
    // p1 += p2;

    let people = vec![
        Person {
            name: "John".to_string(),
        },
                Person {
            name: "Jane".to_string(),
        },
                Person {
            name: "Joe".to_string(),
        }
    ];

    people.run();
    people.walk();

    let elephants = vec![
      Elephant {
        name: "Elephant1".to_string(),
      },
            Elephant {
        name: "Elephant2".to_string(),
      },
            Elephant {
        name: "Elephant3".to_string(),
      },
    ];

    elephants.walk();
}
