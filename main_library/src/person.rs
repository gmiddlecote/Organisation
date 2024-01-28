//! Library crate with definitions
//! - Person
//!

// use std::fmt;

use chrono::{Local, NaiveDate};
use rust_iso3166::*;
use uuid::Uuid;

/// Define an enum for Gender
#[derive(Debug, Default)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

/// Define an enum for Blood Types
#[derive(Debug)]
pub enum BloodType {
    A,
    B,
    AB,
    O,
}

/// Define an enum for Blood RH Factor
#[derive(Debug)]
pub enum RHFactor {
    Positive,
    Negative,
}

/// Define an enum for Complete Blood Type
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

/// Define an enum for Marital Status
#[derive(Debug, Default)]
pub enum MaritalStatus {
    #[default]
    Single,
    Married,
    Divorced,
    Widowed,
    Separated,
    Annulled,
    DomesticPartnershipOrCivilUnion,
    CommonLawMarriage,
    Other(String),
}

// Define an Educational Certificate

/// Define the Types of Certificates
#[derive(Debug, Default)]
pub struct TypeOfCertificate {
    pub name: Vec<String>,
}

/// Define the Field of Study
#[derive(Debug)]
pub struct FieldOfStudy {
    pub name: Vec<String>,
}

/// Name of the educational instution or university
#[derive(Debug)]
pub struct Institution {
    pub name: Vec<String>,
}

/// Authority or Institution awarding the Certificate
#[derive(Debug)]
pub struct AwardedBy {
    pub name: Vec<String>,
}

#[derive(Debug)]
pub struct Certificate {
    /// Seriazal number ofza Certificate
    pub serial_number: String,
    /// Full name as on Certificate
    pub full_name: String,
    pub type_of_certificate: String,
    pub field_of_study: String,
    /// Name of Institution awarding Certificate
    pub instution: String,
    /// Completion date of Course
    pub date_of_completion: NaiveDate,
    /// Institution awarding the Certificate
    pub awarded_by: String,
    /// Grade acheived
    pub grade: u64,
}

#[derive(Debug, Default)]
pub enum ContactNumberCategory {
    #[default]
    Personal,
    Business,
    Emergency,
}

#[derive(Debug)]
pub struct ContactNumber {
    pub category: ContactNumberCategory,
    pub number: String,
}

/// Define a Person
#[derive(Debug)]
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
    pub citizenship_country: Option<CountryCode>,
    pub marital_status: MaritalStatus,
    pub government_issue_id: Option<String>,
    pub certificate: Vec<Certificate>,
    pub contact_number: Vec<ContactNumber>,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            first_name: String::new(),
            middle_name: None,
            last_name: None,
            emp_id: None,
            dob: NaiveDate::parse_from_str("1-1-2020", "%d-%m-%Y").unwrap(),
            gender: Gender::Male,
            complete_bloodtype: CompleteBloodType::BPositive,
            height: 0.0,
            weight: 0.0,
            citizenship_country: None,
            marital_status: MaritalStatus::Single,
            government_issue_id: None,
            certificate: Vec::new(),
            contact_number: Vec::new(),
        }
    }
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
        country_code: String,
        marital_status: MaritalStatus,
        government_issue_id: &str,
        certificate: Vec<Certificate>,
        contact_number: Vec<ContactNumber>,
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
            citizenship_country: rust_iso3166::from_alpha3(&country_code.to_string()),
            marital_status,
            government_issue_id: Some(government_issue_id.to_string().to_uppercase()),
            certificate,
            contact_number,
        }
    }

    // pub fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     // write!(f, "{} {}", self.first_name, self.middle_name)
    //
    //     let f_name = self.first_name.to_string();
    //     let m_name = String::new();
    //     match &self.middle_name {
    //         Some(value) => m_name = self.middle_name,
    //
    //         None => m_name = "",
    //     };
    //     write!("{} {}", f_name, m_name)
    // }

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn test_person() {
        let mut contact_number = vec![
            ContactNumber {
                category: ContactNumberCategory::Personal,
                number: "9123456789123".to_string(),
            },
            ContactNumber {
                category: ContactNumberCategory::Business,
                number: "9112345678912".to_string(),
            },
        ];

        let emp_1 = Person::new(
            "A",
            "B",
            "C",
            "27-06-1974",
            Gender::Male,
            BloodType::A,
            RHFactor::Positive,
            150.0,
            78.0,
            "IND".to_string(),
            MaritalStatus::Single,
            "ANC43522",
            Vec::new(),
            contact_number,
        );
        assert_eq!(emp_1.first_name, "A");
        assert_eq!(emp_1.middle_name.as_deref(), Some("B"));
        assert_eq!(emp_1.last_name.as_deref(), Some("C"));
        assert_eq!(emp_1.dob, NaiveDate::from_ymd_opt(1974, 6, 27).unwrap());
        assert!(matches!(emp_1.gender, Gender::Male));
        assert!(matches!(
            emp_1.complete_bloodtype,
            CompleteBloodType::APositive
        ));
        assert_eq!(emp_1.height, 150.0);
        assert_eq!(emp_1.weight, 78.0);
        assert_eq!(Some(emp_1.citizenship_country.unwrap().alpha3), Some("IND"));
        assert!(matches!(emp_1.marital_status, MaritalStatus::Single));
        assert_eq!(emp_1.government_issue_id, Some("ANC43522".to_string()));
        let mut type_of_certificate = TypeOfCertificate { name: Vec::new() };
        type_of_certificate
            .name
            .push("High School Certificate".to_string());
        type_of_certificate.name.push("Pre University".to_string());

        let mut field_of_study = FieldOfStudy { name: Vec::new() };
        field_of_study.name.push("Higher Education".to_string());

        let mut instiution = Institution { name: Vec::new() };
        instiution.name.push("St Edmunds School".to_string());

        let mut awarded_by = AwardedBy { name: Vec::new() };
        awarded_by
            .name
            .push("Indian Certificate of Secondary Education".to_string());

        let certificate_1 = Certificate {
            serial_number: "12d342d3".to_string(),
            full_name: "John Rambo".to_string(),
            type_of_certificate: type_of_certificate.name.get(0).cloned().unwrap_or_default(),
            field_of_study: field_of_study.name.get(0).cloned().unwrap_or_default(),
            instution: instiution.name.get(0).cloned().unwrap_or_default(),
            date_of_completion: NaiveDate::from_ymd_opt(2000, 2, 1).unwrap(),
            awarded_by: awarded_by.name.get(0).cloned().unwrap_or_default(),
            grade: 20,
        };

        assert_eq!(certificate_1.serial_number, "12d342d3");
        assert_eq!(
            certificate_1.type_of_certificate,
            type_of_certificate.name.get(0).unwrap().to_string()
        );
    }
}
