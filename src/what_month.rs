use std::io;

pub fn what_month_is_it() {
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read fail");
    let selection: usize = input.trim().parse().expect("convert fail");

    // assume user enters 1..=12, adjust to 0-based index and validate
    if selection == 0 || selection > months.len() {
        eprintln!("please enter a month number between 1 and {}", months.len());
        return;
    }

    let month = months[selection - 1];
    println!("the number month: {selection} is {month}");
}