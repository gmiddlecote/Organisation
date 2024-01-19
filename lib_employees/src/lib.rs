/// Employee
// use std::fmt;
// use std::fmt::Display;
use std::default;

// add uuid
use uuid::Uuid;

// add chrono
// use chrono::format::ParseError;
// use chrono::prelude::*;
use chrono::{Local, NaiveDate};

// Define an enum for Gender
#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

// set a fefault gender
impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

#[derive(Debug, Default)]
pub struct Person {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub emp_id: Option<Uuid>,
    pub dob: NaiveDate,
    pub gender: Gender,
}

impl Person {
    // Initalize a new person
    pub fn new(first: &str, middle: &str, last: &str, dob: &str, gender: Gender) -> Person {
        Person {
            first_name: first.to_string(),
            middle_name: Some(middle.to_string()),
            last_name: Some(last.to_string()),
            emp_id: Some(Person::generate_emp_id()),
            dob: NaiveDate::parse_from_str(dob, "%d-%m-%Y").unwrap(),
            gender,
        }
    }

    // Generate an Employee ID
    fn generate_emp_id() -> Uuid {
        let id = Uuid::new_v4();
        id
    }

    // Get current age
    pub fn get_current_age(&self) -> i64 {
        let today = Local::now().date_naive();
        // dereference the dob
        let date_of_birth = *(&self.dob);
        let duration = today.signed_duration_since(date_of_birth);

        //Calculate the difference in years
        let years_difference = duration.num_days() / 365;
        years_difference
    }
}
