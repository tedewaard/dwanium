use database::{add_computers_db, query_table};
use dell::{trial_run, dell_api_query};
use tanium::get_computers;

mod tanium;
mod database;
mod dell;
mod token;




fn main() {
    /* Below querys Tanium and adds computers to DB
    let computers = get_computers();
    add_computers_db(computers);
    */


    //query_table();
    dell_api_query().unwrap(); 

}
