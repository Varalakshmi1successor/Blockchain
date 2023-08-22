use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    email: String,
    phno: String,
    id: u32,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    students.push(Student {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        phno: "1234567890".to_string(),
        id: 1,
    });
    students.push(Student {
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
        phno: "0987654321".to_string(),
        id: 2,
    });
    students.push(Student {
        name: "Charlie".to_string(),
        email: "charlie@example.com".to_string(),
        phno: "1357924680".to_string(),
        id: 3,
    });
    students.push(Student {
        name: "Dave".to_string(),
        email: "dave@example.com".to_string(),
        phno: "2468013579".to_string(),
        id: 4,
    });
    students.push(Student {
        name: "Eve".to_string(),
        email: "eve@example.com".to_string(),
        phno: "9876543210".to_string(),
        id: 5,
    });

    println!("Enter the index of the student you want to view (0-4):");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number between 0 and 4.");
            return;
        }
    };

    match students.get(index) {
        Some(student) => println!("{:#?}", student),
        None => println!("No student found at index {}.", index),
    }
}
