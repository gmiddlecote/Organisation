/// Employee
// add uuid
use uuid::Uuid;

use chrono::{Local, NaiveDate};

// Define an enum for Gender
#[derive(Debug, Default)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

// Define an enum for Blood Types
#[derive(Debug)]
pub enum BloodType {
    A,
    B,
    AB,
    O,
}

// Define an enum for Blood RH Factor
#[derive(Debug)]
pub enum RHFactor {
    Positive,
    Negative,
}

// Define an enum for complete blood type
#[derive(Debug, Default)]
pub enum CompleteBloodType {
    APositive,
    ANegative,
    #[default]
    BPositive,
    BNegative,
    ABPositive,
    ABNegative,
    OPositive,
    ONegative,
}

impl CompleteBloodType {
    pub fn from_components(abo: BloodType, rh: RHFactor) -> CompleteBloodType {
        match (abo, rh) {
            (BloodType::A, RHFactor::Positive) => CompleteBloodType::APositive,
            (BloodType::A, RHFactor::Negative) => CompleteBloodType::ANegative,
            (BloodType::B, RHFactor::Positive) => CompleteBloodType::BPositive,
            (BloodType::B, RHFactor::Negative) => CompleteBloodType::BNegative,
            (BloodType::AB, RHFactor::Positive) => CompleteBloodType::ABPositive,
            (BloodType::AB, RHFactor::Negative) => CompleteBloodType::ABNegative,
            (BloodType::O, RHFactor::Positive) => CompleteBloodType::OPositive,
            (BloodType::O, RHFactor::Negative) => CompleteBloodType::ONegative,
        }
    }
}

// Define a Person
#[derive(Debug, Default)]
pub struct Person {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub emp_id: Option<Uuid>,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub complete_bloodtype: CompleteBloodType,
    pub height: f64, // in cms
    pub weight: f64, // in kgs
}

impl Person {
    // Initalize a new person
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        first: &str,
        middle: &str,
        last: &str,
        dob: &str,
        gender: Gender,
        abo: BloodType,
        rh_factor: RHFactor,
        height: f64,
        weight: f64,
    ) -> Person {
        Person {
            first_name: first.to_string(),
            middle_name: Some(middle.to_string()),
            last_name: Some(last.to_string()),
            emp_id: Some(Person::generate_emp_id()),
            dob: NaiveDate::parse_from_str(dob, "%d-%m-%Y").unwrap(),
            gender,
            complete_bloodtype: CompleteBloodType::from_components(abo, rh_factor),
            height,
            weight,
        }
    }

    // Generate an Employee ID
    fn generate_emp_id() -> Uuid {
        Uuid::new_v4()
    }

    // Get current age
    pub fn get_current_age(&self) -> i64 {
        let today = Local::now().date_naive();
        // dereference the dob
        // let date_of_birth = *(&self.dob);
        let duration = today.signed_duration_since(*(&self.dob));

        //Calculate the difference in years
        duration.num_days() / 365
    }
}
