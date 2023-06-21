use dell::{dell_api_query, map_to_serial_and_enddate};
use database::{query_serialnum, bulk_add_serial_name, update_computer_db};
use tanium::get_computers;
use dotenv::dotenv;
use std::time::*;



mod tanium;
mod database;
mod dell;


//Was working on setting up concurrency for adding computers to DB
#[tokio::main]
async fn main() {
    let before = Instant::now();
    //Load environment variables
    dotenv().ok();

    //Querying Tanium for all Dell Endpoints and add to DB
    println!("Querying Tanium...");
    let computers = get_computers().await;
    println!("Elapsed Time: {:.2?}", before.elapsed());


    println!("Adding computers to db...");
    let mut temp_computers = Vec::new();
    for (idx, computer) in computers.iter().enumerate() {
        if temp_computers.len() == 100 || idx == computers.len()-1 {
            bulk_add_serial_name(temp_computers).await;
            temp_computers = Vec::new();
        }
        temp_computers.push(computer.clone());
    }
    println!("Elapsed Time: {:.2?}", before.elapsed());
    //Reading from DB and querying Dell
    println!("Querying serial nums from db");
    let serial_nums = query_serialnum().await;
    let mut temp_serial = Vec::new();
    println!("Querying Dell...");
    for (idx, serial) in serial_nums.iter().enumerate() {
        if temp_serial.len() == 100 || idx == serial_nums.len()-1 {
            let dell_api_results = dell_api_query(temp_serial).await.unwrap();
            println!("Elapsed Time (api): {:.2?}", before.elapsed());
            //println!("{:?}", dell_api_results);
            let updates = map_to_serial_and_enddate(dell_api_results);
            update_computer_db(updates).await;
            println!("Elapsed Time (db): {:.2?}", before.elapsed());
            temp_serial = Vec::new();
        }
        temp_serial.push(serial.to_string());
    }
    println!("Elapsed Time: {:.2?}", before.elapsed());
}
