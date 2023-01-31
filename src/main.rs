use dell::dell_api_query;
use database::query_serialnum;

mod tanium;
mod database;
mod dell;
mod token;




fn main() {
    let serial_nums = query_serialnum();
    for serial in serial_nums {
        let pc_info = dell_api_query(serial).unwrap();        
        println!("{:?}", pc_info)
    }

    
}
