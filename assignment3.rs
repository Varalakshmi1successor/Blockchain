use std::io;

#[derive(Debug)]
struct Employee {
    employee_name: String,
    employee_id: u32,
    email: String,
    age: u8,
    phone_number: String,
}

fn get_employee_by_id(employees: &[Employee], id: u32) -> Option<&Employee> {
    employees.iter().find(|&employee| employee.employee_id == id)
}

fn get_employees_by_age(employees: &[Employee], age: u8) -> Vec<&Employee> {
    employees.iter().filter(|&employee| employee.age == age).collect()
}

fn main() {
    let mut employees = Vec::new();

    loop {
        let mut employee_name = String::new();
        let mut employee_id = String::new();
        let mut email = String::new();
        let mut age = String::new();
        let mut phone_number = String::new();

        println!("Enter employee name (or 'done' to finish):");
        io::stdin().read_line(&mut employee_name).unwrap();
        let employee_name = employee_name.trim();
        if employee_name == "done" {
            break;
        }

        println!("Enter employee ID:");
        io::stdin().read_line(&mut employee_id).unwrap();
        let employee_id: u32 = employee_id.trim().parse().unwrap();

        println!("Enter email:");
        io::stdin().read_line(&mut email).unwrap();
        let email = email.trim().to_string();

        println!("Enter age:");
        io::stdin().read_line(&mut age).unwrap();
        let age: u8 = age.trim().parse().unwrap();

        println!("Enter phone number:");
        io::stdin().read_line(&mut phone_number).unwrap();
        let phone_number = phone_number.trim().to_string();

        employees.push(Employee {
            employee_name: employee_name.to_string(),
            employee_id,
            email,
            age,
            phone_number,
        });
    }

    // Example usage of the functions
    if let Some(employee) = get_employee_by_id(&employees, 1) {
        println!("Employee with ID 1: {:?}", employee);
    } else {
        println!("No employee found with ID 1");
    }

    let same_age_employees = get_employees_by_age(&employees, 30);
    println!("Employees with age 30: {:?}", same_age_employees);
}

