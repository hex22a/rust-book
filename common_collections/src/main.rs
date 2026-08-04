mod departments;
mod median_mode;
mod pig_latin;

fn main() {
    median_mode::run();
    println!();
    pig_latin::run();
    println!();
    println!("Departments:");
    departments::run();
}
