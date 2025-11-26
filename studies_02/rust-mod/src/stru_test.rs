
use crate::tra_test::Hello;

pub struct Person;


impl Hello for Person {
    fn hello(&self) {
        println!("trait Hello");
    }
}