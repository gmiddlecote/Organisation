// #[path = "./employee.rs"]
// mod organisation_employees;

fn main() {}

#[test]
fn check_employee() {
    let emp = Person::new("A", "B", "C", "20-01-1920");
    assert_eq!(emp.first_name, "A");
}
