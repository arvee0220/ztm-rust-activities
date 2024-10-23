// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
struct Student {
    name: String,
    // * The locker assignment should use an Option<i32>
    locker: Option<i32>,
}
fn main() {
    let rv: Student = Student {
        name: String::from("Rowell"),
        locker: Some(5),
    };

    println!("Student name: {:}", rv.name);

    match rv.locker {
        Some(locker) => println!("Locker assignment number: {locker:}"),
        None => println!("No locker assigned"),
    }
}
