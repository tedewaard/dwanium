use dell::{dell_api_query, map_to_serial_and_enddate};
use database::{query_serialnum, update_computer_db, add_computers_db};
use tanium::get_computers;


mod tanium;
mod database;
mod dell;
mod token;


/* 
 * Function that breaks up into 100 chunks and then calls the dell dell api
 * Takes results of the api call and then adds to database
 */

fn main() {

    //Querying Tanium for all Dell Endpoints and add to DB
    let computers = get_computers();
    add_computers_db(computers);

    //Reading from DB and querying Dell
    let serial_nums = query_serialnum();
    let mut temp_serial = Vec::new();
    for serial in serial_nums {
        if temp_serial.len() == 100 {
            let dell_api_results = dell_api_query(temp_serial).unwrap();
            println!("{:?}", dell_api_results);
            let updates = map_to_serial_and_enddate(dell_api_results);
            update_computer_db(updates);
            temp_serial = Vec::new();
        }
        temp_serial.push(serial);
    }
}
