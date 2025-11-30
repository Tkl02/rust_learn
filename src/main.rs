mod gues_a_number;
use std::io;
use gues_a_number::guess_number;
mod  hour_to_sec;
use hour_to_sec::convert;
mod what_month;
use what_month::what_month_is_it;

fn main() {

    let mut selection = String::new();
    io::stdin().read_line(&mut selection).expect("falha ao ler valor");

    match selection.trim() {
        "1" => guess_number(),
        "2" => read_to_cover(),
        "3" => what_month_is_it(),
        _ => println!("Opção inválida"),
    };
}
//auxiliary function for converting hours 
fn read_to_cover(){


    let mut  number = String::new();
    io::stdin().read_line(&mut number).expect("falha ao ler valor");
    let number: i32 = number.trim().parse().expect("falha na converção");
    convert(number);
}
