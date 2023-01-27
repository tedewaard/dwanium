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


fn query_table(client: &mut Client) {
    for row in client.query("SELECT serial, name, end_date FROM computers", &[]).unwrap() {
        let serial: &str = row.get(0);
        let name: &str = row.get(1);
        let end_date: &str = row.get(2);

        println!("found entry: {} {} {}", serial, name, end_date);
    }
}

pub fn add_computers_db(computers: Vec<Computer>) {
    let mut client = setup_client().unwrap();
    for computer in computers {
       add_record(&mut client, computer.serial_number, computer.name);
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

pub fn run() {
    let mut client = setup_client().unwrap();
    //add_test_entrys(&mut client);

    query_table(&mut client);
}
