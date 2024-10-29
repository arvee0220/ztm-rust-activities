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
#[derive(Debug)]
enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

enum Status {
    Employed,
    Terminated,
}

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
        _ => Err(String::from("Peasant! Begone!")),
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

    let prev_employee: Employee = Employee {
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
        status: Status::Employed,
    };

    match print_access(&spy) {
        Err(e) => println!("Access denied: {e:?}"),
        _ => (),
    }

    match print_access(&prev_employee) {
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
}
