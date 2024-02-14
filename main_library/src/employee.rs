//! Library crate with definitions
//! Employee Library
//!

use super::person as Person_Lib;
use chrono::NaiveDate;

/// Department
#[derive(Debug)]
pub struct Department {
    name: vec![String],
}

// NOTE: change in designation will be handled by a seperate process
/// Designation per Department
#[derive(Debug)]
pub struct Designation {
    department: Department,
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
    designation: String,
    provident_fund_account: Option<String>,
    date_employment_start: Option<NaiveDate>,
    date_employment_end: Option<NaiveDate>,
}

impl Employee {
    pub fn new(
        person: Person_Lib::Person,
        department: Department,
        designation: &str,
        provident_fund_account: &str,
        date_employment_start: &str,
        date_employment_end: &str,
    ) -> Employee {
        Employee {
            person,
            department,
            designation.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_department() {
        let departments = Department {
        name: vec!["Information Technology", "Human Resources"] 
    };
        assert_eq!(departments.get(0).name, "Information Technology" );
    }

    //#[test]
    //fn test_employee() {
    //    let mut contact_number = vec![
    //        ContactNumber {
    //            category: ContactNumberCategory::Personal,
    //            number: "9123456789123".to_string(),
    //        },
    //        ContactNumber {
    //            category: ContactNumberCategory::Business,
    //            number: "9112345678912".to_string(),
    //        },
    //    ];

    //    let emp_1 = Person::new(
    //        "A",
    //        "B",
    //        "C",
    //        "27-06-1974",
    //        Gender::Male,
    //        BloodType::A,
    //        RHFactor::Positive,
    //        150.0,
    //        78.0,
    //        "IND".to_string(),
    //        MaritalStatus::Single,
    //        "ANC43522",
    //        Vec::new(),
    //        contact_number,
    //    );
    //    let mut type_of_certificate = TypeOfCertificate { name: Vec::new() };
    //    type_of_certificate
    //        .name
    //        .push("High School Certificate".to_string());
    //    type_of_certificate.name.push("Pre University".to_string());

    //    let mut field_of_study = FieldOfStudy { name: Vec::new() };
    //    field_of_study.name.push("Higher Education".to_string());

    //    let mut instiution = Institution { name: Vec::new() };
    //    instiution.name.push("St Edmunds School".to_string());

    //    let mut awarded_by = AwardedBy { name: Vec::new() };
    //    awarded_by
    //        .name
    //        .push("Indian Certificate of Secondary Education".to_string());

    //    let certificate_1 = Certificate {
    //        serial_number: "12d342d3".to_string(),
    //        full_name: "John Rambo".to_string(),
    //        type_of_certificate: type_of_certificate.name.get(0).cloned().unwrap_or_default(),
    //        field_of_study: field_of_study.name.get(0).cloned().unwrap_or_default(),
    //        instution: instiution.name.get(0).cloned().unwrap_or_default(),
    //        date_of_completion: NaiveDate::from_ymd_opt(2000, 2, 1).unwrap(),
    //        awarded_by: awarded_by.name.get(0).cloned().unwrap_or_default(),
    //        grade: 20,
    //    };

   // let department_list = vec!["Information Technology", "Human Resources"];

   //     let employee_1: Employee = {
   //         person: emp_1,
   //         department: 
   //         designation: 
   //     }

   // }
}
