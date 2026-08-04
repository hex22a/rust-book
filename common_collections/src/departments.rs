use std::{
    collections::{
        HashMap,
        hash_map::Entry::{Occupied, Vacant},
    },
    io,
};

fn add_departments(departments: &mut HashMap<String, Vec<String>>) {
    loop {
        println!("Adding departments:");
        println!("1. Add");
        println!("0. Back");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            0 => {
                break;
            }
            1 => {
                println!("Enter a department name: ");
                let mut name = String::new();

                io::stdin()
                    .read_line(&mut name)
                    .expect("falied to read line");
                name = name.trim().to_string();

                match departments.entry(name) {
                    Vacant(e) => {
                        e.insert(Vec::new());
                    }
                    Occupied(_) => {
                        println!("Department already exists!");
                    }
                }
            }
            _ => {
                println!("Invalid option. Try again");
                println!();
            }
        }
    }
}

fn add_person(department: &String, departments: &mut HashMap<String, Vec<String>>) {
    loop {
        println!("Adding person to a department {}", &department);
        println!("1. Add");
        println!("0. Back");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            0 => {
                break;
            }
            1 => {
                println!("Enter a name: ");
                let mut name = String::new();

                io::stdin()
                    .read_line(&mut name)
                    .expect("falied to read line");

                departments.get_mut(department).unwrap().push(name);
                println!("Added!");
            }
            _ => {
                println!("Invalid option. Try again");
                println!();
            }
        }
    }
}

fn departments_menu(departments: &HashMap<String, Vec<String>>) -> HashMap<u8, String> {
    let mut result: HashMap<u8, String> = HashMap::new();
    for (i, (name, _)) in departments.iter().enumerate() {
        result.insert((i + 1) as u8, name.clone());
    }
    result
}

fn persons_in_department(department: &String, departments: &mut HashMap<String, Vec<String>>) {
    let persons = departments.get_mut(department).unwrap();
    persons.sort();
    for p in persons {
        println!("{p}");
    }
}

fn list_department(departments: &mut HashMap<String, Vec<String>>) {
    let departments_list: HashMap<u8, String> = departments_menu(departments);
    let dep_list_len = departments_list.len() as u8;
    loop {
        println!("Choose department:");
        for (i, d) in &departments_list {
            println!("{i}. {d}");
        }

        println!("0. Back");
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            0 => {
                break;
            }
            num => {
                if (num >= 1u8) && (num <= dep_list_len) {
                    let department_name = departments_list.get(&num).unwrap();
                    persons_in_department(department_name, departments);
                }
            }
        }
    }
}

fn choose_department(departments: &mut HashMap<String, Vec<String>>) {
    let departments_list: HashMap<u8, String> = departments_menu(departments);
    let dep_list_len = departments_list.len() as u8;
    loop {
        println!("Choose department:");
        for (i, d) in &departments_list {
            println!("{i}. {d}");
        }

        println!("0. Back");
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            0 => {
                break;
            }
            num => {
                if (num >= 1u8) && (num <= dep_list_len) {
                    let department_name = departments_list.get(&num).unwrap();
                    add_person(department_name, departments);
                }
            }
        }
    }
}

fn list_all(departments: &mut HashMap<String, Vec<String>>) {
    let mut all: Vec<String> = Vec::new();
    for persons in departments.values_mut() {
        all.append(persons);
    }
    all.sort();
    for p in all {
        println!("{p}");
    }
}

pub fn run() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Departments");
        println!("1. Add departments");
        println!("2. Add a person");
        println!("3. List department");
        println!("4. List all people");
        println!("0. Exit");
        println!("Choose an option: ");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            0 => {
                println!("Goodbye!");
                break;
            }
            1 => add_departments(&mut departments),
            2 => choose_department(&mut departments),
            3 => list_department(&mut departments),
            4 => list_all(&mut departments),
            _ => {
                println!("Invalid option. Try again");
                println!();
            }
        }
    }
}
