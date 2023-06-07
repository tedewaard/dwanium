use postgres::{Client, NoTls, Error};
use crate::tanium::Computer;



fn setup_client() -> Result<Client, Error>{
let client = Client::connect("host=localhost dbname=dwanium port=5432 user=tedewaard password='test-password'", NoTls)?;
Ok(client)
}


fn add_test_entrys(client: &mut Client) {
    let name = "test";
    let serial = "test";
    let end_date = "test";
    let execute = client.execute(
        "INSERT INTO computers (serial, name, end_date) VALUES ($1, $2, $3)",
        &[&serial, &name, &end_date],
    );

    match execute {
        Err(error) => println!("Error inserting record: {}", error),
        _ => ()
    }
}

pub fn query_serialnum() -> Vec<String> {
    let mut serial_numbers = Vec::new();
    let mut client = setup_client().unwrap();
    for row in client.query("SELECT serial, name, end_date FROM computers", &[]).unwrap() {
        let serial: &str = row.get(0);
        serial_numbers.push(serial.to_string());
    }
    return serial_numbers;
}

pub fn add_computers_db(computers: Vec<Computer>) {
    let mut client = setup_client().unwrap();
    for computer in computers {
        if computer.serial_number.len() == 7 {
            add_record(&mut client, computer.serial_number, computer.name);
        }
    }
}

fn add_record(client: &mut Client, serial: String, name: String) { 
    let execute = client.execute(
        "INSERT INTO computers (serial, name) VALUES ($1, $2)",
        &[&serial, &name],
    );

    match execute {
        Err(error) => println!("Error inserting record: {}", error),
        _ => ()
    }
}

pub fn update_computer_db(computers: Vec<(String, String)>) {
    let mut client = setup_client().unwrap();
    for computer in computers {
       update_record(&mut client, computer.0, computer.1);
    }
}

fn update_record(client: &mut Client, serial: String, end_date: String) {
    let execute = client.execute(
        "UPDATE computers SET end_date = $1 WHERE serial = $2",
        &[&end_date, &serial],
        );
    match execute {
        Err(error) => println!("Error updating record: {}", error),
        _ => ()
    }
}

