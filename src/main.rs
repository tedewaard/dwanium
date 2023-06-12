use dell::{dell_api_query, map_to_serial_and_enddate};
use database::{query_serialnum, update_computer_db, add_computers_db};
use tanium::get_computers;
use dotenv::dotenv;
use std::time::*;



mod tanium;
mod database;
mod dell;


//Was working on setting up concurrency for adding computers to DB
fn main() {
    let before = Instant::now();
    //Load environment variables
    dotenv().ok();

    /*
    //Querying Tanium for all Dell Endpoints and add to DB
    println!("Querying Tanium...");
    let computers = get_computers();
    println!("Elapsed Time: {:.2?}", before.elapsed());

    println!("Adding computers to db...");
    add_computers_db(computers);
    println!("Elapsed Time: {:.2?}", before.elapsed());
    */

    //Reading from DB and querying Dell
    let serial_nums = query_serialnum();
    let mut temp_serial = Vec::new();
    println!("Querying Dell...");
    for (idx, serial) in serial_nums.iter().enumerate() {
        if temp_serial.len() == 100 || idx == serial_nums.len()-1 {
            let dell_api_results = dell_api_query(temp_serial).unwrap();
            //println!("{:?}", dell_api_results);
            let updates = map_to_serial_and_enddate(dell_api_results);
            update_computer_db(updates);
            temp_serial = Vec::new();
        }
        temp_serial.push(serial.to_string());
    }
    println!("Elapsed Time: {:.2?}", before.elapsed());
}
