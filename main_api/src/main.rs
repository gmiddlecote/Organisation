#[macro_use]
extern crate rocket;

use main_library::employee as Employee_Lib;
use main_library::person as Person_Lib;

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

fn create_person() {
    let contact_number = vec![
        Person_Lib::ContactNumber {
            category: Person_Lib::ContactNumberCategory::Personal,
            number: "9123456789123".to_string(),
        },
        Person_Lib::ContactNumber {
            category: Person_Lib::ContactNumberCategory::Business,
            number: "9112345678912".to_string(),
        },
    ];

    let emp_1 = Person_Lib::Person::new(
        "A",
        "B",
        "C",
        "27-06-1974",
        Person_Lib::Gender::Male,
        Person_Lib::BloodType::A,
        Person_Lib::RHFactor::Positive,
        150.0,
        78.0,
        "IND".to_string(),
        Person_Lib::MaritalStatus::Single,
        "ANC43522",
        Vec::new(),
        contact_number,
    );
}
