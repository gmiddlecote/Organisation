//! Library crate with definitions
//! Employee Library
//!

use crate::person::*;
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
    person: Person,
    department: Department,
    designation: Designation,
    provident_fund_account: Option<String>,
    date_employment_start: Option<NaiveDate>,
    date_employment_end: Option<NaiveDate>,
}
