use std::io;
use chrono::Utc;
use chrono::Datelike;
pub fn guess_birth_year() {
    println!("Year of birth calculator!");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age : u32 = age.trim().parse().expect("Please type a number!");
    println!("{}", get_birth_year(age));
}

fn get_birth_year(age : u32) -> String {
    let year :u32 = Utc::now().year().try_into().unwrap();
    return format!("You were born in year {}",  &year - &age);
}