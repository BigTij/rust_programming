use std::collections::HashMap;
use std::io;

fn choose_department() -> String {
    println!("Please choose a department:");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("failed to read line.");
    department.trim().to_string()
}

fn list_department_names(h: &mut HashMap<&str, Vec<String>>, department: String){
    if let Some(value) = h.get_mut(department.as_str()){
        println!("{}: {:?}", department, value);
    }
}

fn list_departments(h: &HashMap<&str, Vec<String>>){
    for (key, value) in h{
        println!("{}", key);
    }
}
fn add_employee(h: &mut HashMap<&str,Vec<String>>, department: String) {
    if let Some(value) = h.get_mut(department.as_str()){
        println!("Enter a name to add to this department:");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        value.push(name.trim().to_string());
    }
}
fn main() {

    let departments = vec!["Engineering","Human_ressources","Janitors","Managers","Technicians"];
    let employees = vec![vec![String::from("");0];departments.len()];

    let mut department_management: HashMap<_, _> = departments.into_iter().zip(employees.into_iter()).collect();

    while true {
        println!("choose an option \n1: list departments\n2: list department employees\n3: add an employee to a department\n0: exit");
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line.");
        let option = option.trim();
        match &option[..] {
            "1" => list_departments(&department_management),
            "2" => {
                let department = choose_department();
                list_department_names(&mut department_management, department);
            },
            "3" => {
                let department = choose_department();
                add_employee(&mut department_management, department)
            },
            "0" => break,
            _ => println!("Wrong command please try again."),
        }
    }
}
