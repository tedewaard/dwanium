use std::println;

use postgres::{Client, NoTls, Error};
use sqlx::{QueryBuilder, Postgres};
use crate::tanium::Computer;
use sqlx::PgConnection;
use sqlx::Connection;

//SQLX 
async fn setup_sqlx_connection() -> Result<PgConnection, sqlx::Error> {
    let conn = PgConnection::connect("postgresql://localhost?dbname=dwanium&user=tedewaard&password=test-password").await?;
    Ok(conn)
}

pub async fn sqlx_bulk_add_test_entrys(computers: Vec<Computer>) {
    let mut conn = setup_sqlx_connection().await.unwrap();
    let mut querybuilder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO computers(serial, name) "
    );

    querybuilder.push_values(computers.into_iter(), |mut b, computer| {
        b.push_bind(computer.serial_number)
            .push_bind(computer.name);
    });

    let query = querybuilder.build(); 
    //println!("{:?}", query.sql());
    let result = query.execute(&mut conn).await;
    match result {
        Err(error) => println!("Error inserting record: {}", error),
        _ => ()
    }
}

pub async fn sqlx_add_test_entrys() {
    let mut conn = setup_sqlx_connection().await.unwrap();
    let name = "test";
    let serial = "test";
    let end_date = "test";
    let query = sqlx::query("INSERT INTO computers (serial, name) VALUES ($1, $2)")
        .bind(serial).bind(name).execute(&mut conn).await;
    match query {
        Err(error) => println!("Error inserting record: {}", error),
        _ => ()
    }

}

pub async fn sqlx_query_serialnum() -> Vec<String>{
    let mut serial_numbers = Vec::new();
    let mut client = setup_sqlx_connection().await.unwrap();
    for row in client.query("SELECT serial, name, end_date FROM computers", &[]).unwrap() {
        let serial: &str = row.get(0);
        serial_numbers.push(serial.to_string());
    }
    return serial_numbers;
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

pub fn mass_add_computers_db(computers: Vec<Computer>) {
    let mut client = setup_client().unwrap();
    add_test_entrys(&mut client);
    let mut strings = Vec::new();
    for computer in computers {
        if computer.serial_number.len() == 7 {
            //Create String
            let temp_string = format!("('{}', '{}')", computer.serial_number, computer.name);
            strings.push(temp_string);
            //mass_add_record(&mut client, &insert_query);
        }
    }
    let insert_query: String = strings.join(", ");
    //println!("{}", insert_query);
    mass_add_record(&mut client, &insert_query);
}

fn mass_add_record(client: &mut Client, query: &String) { 
    println!("INSERT INTO computers (serial, name) VALUES {}", query);
    let execute = client.execute(
        "INSERT INTO computers (serial, name) VALUES $1",
        &[&query],
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

