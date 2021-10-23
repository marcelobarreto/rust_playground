use std::fmt;
use std::collections::HashMap;

pub struct Person {
  name: String,
  age: u8,
  city: String,
  status: Statuses,
}

pub enum Statuses {
  Active,
  Inactive,
}

impl fmt::Display for Statuses {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Statuses::Active => write!(f, "Active"),
      Statuses::Inactive => write!(f, "Inactive"),
    }
  }
}

impl Person {
  fn say(&self, msg: &str) {
    println!("{0} said: {1}", self.name, msg);
  }
}

pub fn create_person(name: String, city: String, age: u8, status: Statuses) -> Person {
  return Person { name, city, age, status };
}

fn main() {
  let mut users = HashMap::new();
  users.entry("1").or_insert(create_person("Marcelo".to_string(), "San Francisco".to_string(), 27, Statuses::Active));
  users.entry("2").or_insert(create_person("Marcelo".to_string(), "San Francisco".to_string(), 27, Statuses::Active));

  let person = &users["1"];
  // let person = USERS[1];
  println!("[USER FOUND] id=1 name={0} age={1} city={2} status={3}", person.name, person.city, person.age, person.status);
  println!("");
  person.say("Hello, there!");
}