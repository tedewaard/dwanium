use database::add_computers_db;
use tanium::get_computers;

mod tanium;
mod database;
mod dell;
mod token;



fn main() {
    //tanium_api_call();
    //get_pages();
    //print_computers();
    let computers = get_computers();
    add_computers_db(computers);
}
