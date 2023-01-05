use std::env;
use crate::converter::converter::Book;

mod converter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let book = Book::from_file(&file);
    let export = book.export_to_post();
    println!("{}", export)
}
