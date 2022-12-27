use std::env;
use crate::converter::converter::Book;

mod converter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let book = Book::from_file(&file);
    let export = book.export_to_post();
    println!("{}", export)

   // let input = read_test_file("input.txt");
   // let book = converter::Book::from(&input);
   // //let export = book.export_to_post();
   // println!("{export:?}");
}
