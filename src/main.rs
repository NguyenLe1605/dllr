use std::cell::Cell;

use list::List;

mod list;
fn main() {
    let mut list = List::new();

    list.insert_front("2".to_string());
    list.insert_front("1".to_string());

    list.insert_back("3".to_string());
    list.insert_back("4".to_string());

    println!("{}", list);
}
