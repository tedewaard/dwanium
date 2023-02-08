use dell::{dell_api_query, map_to_serial_and_enddate};
use database::{query_serialnum, update_computer_db, add_computers_db};
use tanium::get_computers;
use dotenv::dotenv;
use std::time::*;
use std::thread;



mod tanium;
mod database;
mod dell;



fn main() {
    let before = Instant::now();
    dotenv().ok();

    //Querying Tanium for all Dell Endpoints and add to DB
    println!("Querying Tanium...");
    let mut computers = get_computers();
    println!("Elapsed Time: {:.2?}", before.elapsed());

    let second = computers.split_off(computers.len()/2);
    let handle = thread::spawn(|| {
        add_computers_db(second);
    });
    add_computers_db(computers);
    handle.join().unwrap();
    println!("Elapsed Time: {:.2?}", before.elapsed());

    //Reading from DB and querying Dell
    let serial_nums = query_serialnum();
    let mut temp_serial = Vec::new();
    println!("Querying Dell...");
    for serial in serial_nums {
        if temp_serial.len() == 100 {
            let dell_api_results = dell_api_query(temp_serial).unwrap();
            //println!("{:?}", dell_api_results);
            let updates = map_to_serial_and_enddate(dell_api_results);
            update_computer_db(updates);
            temp_serial = Vec::new();
        }
        temp_serial.push(serial);
    }
    println!("Elapsed Time: {:.2?}", before.elapsed());
}
