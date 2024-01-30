//! Library crate with definitions
//! Employee Library
//!

// use crate::person::*;
use super::person as Person_Lib;
use chrono::NaiveDate;

/// Department
#[derive(Debug)]
pub struct Department {
    name: String,
}

// NOTE: change in designation will be handled by a seperate process
/// Designation per Department
#[derive(Debug)]
pub struct Designation {
    name: String,
}

#[derive(Debug, Default)]
pub enum Status {
    #[default]
    Employed,
    Suspended,
    Transfered,
    Resigned,
    Terminated,
}

/// Employee
#[derive(Debug)]
pub struct Employee {
    person: Person_Lib::Person,
    department: Department,
    designation: Designation,
    provident_fund_account: Option<String>,
    date_employment_start: Option<NaiveDate>,
    date_employment_end: Option<NaiveDate>,
}

impl Employee {
    pub fn new(
        person: Person_Lib::Person,
        department: Department,
        designation: Designation,
        provident_fund_account: &str,
        date_employment_start: &str,
        date_employment_end: &str,
    ) -> Employee {
        Employee {
            person,
            department,
            designation,
            provident_fund_account: Some(provident_fund_account.to_string()),

            date_employment_start: Some(
                NaiveDate::parse_from_str(date_employment_start, "%d/%m/%Y").unwrap(),
            ),
            date_employment_end: Some(
                NaiveDate::parse_from_str(date_employment_end, "%d/%m/%Y").unwrap(),
            ),
        }
    }
}
