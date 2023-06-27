use dell::{dell_api_query, map_to_serial_and_enddate};
use database::{query_all_serialnum_enddate, query_serialnum_missing_date, query_all_serialnum, bulk_add_serial_name, update_computer_db};
use tanium::{format_import_query, get_computers, Computer, push_end_date_to_tanium};
use dotenv::dotenv;
use std::time::*;
use std::collections::HashSet;
use tokio::time::{sleep, Duration};



mod tanium;
mod database;
mod dell;


#[tokio::main]
async fn main() {
    loop {
        let before = Instant::now();
        //Load environment variables
        dotenv().ok();
        //Querying Tanium for all Dell Endpoints and add to DB if missing
        println!("Querying Tanium...");
        add_missing_records().await;
        println!("Elapsed Time: {:.2?}", before.elapsed());

        //Reading from DB and querying Dell
        println!("Querying serial nums from db");
        let mut serial_nums1 = query_serialnum_missing_date().await;
        if serial_nums1.len() > 100 {
            let serial_nums2 = serial_nums1.split_off(serial_nums1.len()/2);
            println!("Querying Dell...");
            let task1 = tokio::spawn(async {
                chunck_dell_query(serial_nums1).await;
            });
            let task2 = tokio::spawn(async {
                chunck_dell_query(serial_nums2).await;
            });
            let _ = tokio::try_join!(task1, task2);
        } else {
            println!("Querying Dell...");
            let task1 = tokio::spawn(async {
                chunck_dell_query(serial_nums1).await;
            });
            let _ = task1.await;
        }
        println!("Elapsed Time: {:.2?}", before.elapsed());
        println!("Writing warranty end dates to Tanium...");
        let mut import_data = query_all_serialnum_enddate().await;
        if import_data.len() > 5000 {
            let import_data2 = import_data.split_off(5000);
            push_end_date_to_tanium(import_data2).await;
        }
        push_end_date_to_tanium(import_data).await;

        println!("Waiting for next run.");
        
        sleep(Duration::from_secs(1800)).await;
    }
}

async fn chunck_dell_query(serial_nums: Vec<String>) {
    let mut temp_serial = Vec::new();
    for (idx, serial) in serial_nums.iter().enumerate() {
        if temp_serial.len() == 100 || idx == serial_nums.len()-1 {
            let dell_api_results = dell_api_query(temp_serial).await.unwrap();
            //println!("Elapsed Time (api): {:.2?}", before.elapsed());
            //println!("{:?}", dell_api_results);
            let updates = map_to_serial_and_enddate(dell_api_results);
            update_computer_db(updates).await;
            //println!("Elapsed Time (db): {:.2?}", before.elapsed());
            temp_serial = Vec::new();
        }
        temp_serial.push(serial.to_string());
    }
}

async fn add_computers_db(computers: Vec<Computer>) { 
    println!("Adding computers to db...");
    let mut temp_computers = Vec::new();
    for (idx, computer) in computers.iter().enumerate() {
        if temp_computers.len() == 100 || idx == computers.len()-1 {
            bulk_add_serial_name(temp_computers).await;
            temp_computers = Vec::new();
        }
        temp_computers.push(computer.clone());
    }
}

async fn add_missing_records() {
    let mut db_hashset = HashSet::new();
    let mut missing_records = Vec::<Computer>::new();
    let computers_in_tanium = get_computers().await;
    let computers_in_db = query_all_serialnum().await;
    for computer in computers_in_db {
        db_hashset.insert(computer);
    }
    for computer in computers_in_tanium {
        if !db_hashset.contains(&computer.serial_number) {
            missing_records.push(computer); 
        }
    }
    println!("There are {} missing records.", missing_records.len());
    add_computers_db(missing_records).await;
}

