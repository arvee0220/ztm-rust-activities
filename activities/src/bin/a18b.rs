// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
use std::fmt;

#[derive(Debug)]
enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

#[derive(Debug)]
enum Status {
    Employed,
    Terminated,
}

// Tested something
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Employed => write!(f, "Employed"),
            Status::Terminated => write!(f, "Terminated"),
        }
    }
}

#[derive(Debug)]
struct Employee {
    name: String,
    position: Position,
    status: Status,
}

fn access_allowed(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Employed => (),
        Status::Terminated => return Err(String::from("Terminated")),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        Position::AssemblyTechnician | Position::KitchenStaff | Position::LineSupervisor => Err(
            String::from("Your current position is restricted to access this establishment"),
        ),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    access_allowed(employee)?;
    let employee_name = &employee.name;
    println!("Access granted! Good morning {employee_name:?}");
    Ok(())
}

fn main() {
    let spy: Employee = Employee {
        name: "Mr.47.".to_owned(),
        position: Position::Manager,
        status: Status::Employed,
    };

    let new_employee: Employee = Employee {
        name: "Mista.. Should I call you Mistah? Mistah Julian Pepe Onzema".to_owned(),
        position: Position::Maintenance,
        status: Status::Employed,
    };

    let mkt_personnel: Employee = Employee {
        name: "Ms.Sally".to_owned(),
        position: Position::Marketing,
        status: Status::Employed,
    };

    let line_supervisor: Employee = Employee {
        name: "Michelle".to_owned(),
        position: Position::LineSupervisor,
        status: Status::Employed,
    };

    let kitchen_staff: Employee = Employee {
        name: "Gordon".to_owned(),
        position: Position::KitchenStaff,
        status: Status::Employed,
    };

    let assembly_staff: Employee = Employee {
        name: "Malupiton".to_owned(),
        position: Position::AssemblyTechnician,
        status: Status::Terminated,
    };

    match print_access(&spy) {
        Err(e) => println!("Access denied: {e:?}"),
        _ => (),
    }

    match print_access(&new_employee) {
        Err(e) => println!("Access denied: {e:?}"),
        _ => (),
    }

    match print_access(&mkt_personnel) {
        Err(e) => println!("Access denied: {e:?}"),
        _ => (),
    }

    match print_access(&line_supervisor) {
        Err(e) => println!("Access denied: {e:?}"),
        _ => (),
    }

    match print_access(&kitchen_staff) {
        Err(e) => println!("Access denied: {e:?}"),
        _ => (),
    }

    match print_access(&assembly_staff) {
        Err(e) => println!("Access denied: {e:?}"),
        _ => (),
    }

    // Tested something
    let user: Employee = Employee {
        name: "Alex".to_owned(),
        position: Position::KitchenStaff,
        status: Status::Employed,
    };

    let Employee {
        mut name,
        mut position,
        mut status,
    } = user;

    println!("{name:?}, {position:?}, {status:}");

    name = "Maloi".to_owned();
    position = Position::LineSupervisor;
    status = Status::Employed;

    let user = Employee {
        name,
        position,
        status,
    };

    println!("{user:?}");
}
