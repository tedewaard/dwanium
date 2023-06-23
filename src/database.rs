use std::println;

use postgres::{Client, NoTls, Error};
use sqlx::Row;
use sqlx::{QueryBuilder, Postgres};
use crate::tanium::Computer;
use sqlx::PgConnection;
use sqlx::Connection;
use sqlx::{postgres::PgRow};

//SQLX 
async fn setup_sqlx_connection() -> Result<PgConnection, sqlx::Error> {
    let conn = PgConnection::connect("postgresql://localhost?dbname=dwanium&user=tedewaard&password=test-password").await?;
    Ok(conn)
}

pub async fn bulk_add_serial_name(computers: Vec<Computer>) {
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

/*
async fn sqlx_add_test_entrys() {
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
*/
pub async fn query_all_serialnum() -> Vec<String>{
    let mut serial_numbers = Vec::new();
    let mut client = setup_sqlx_connection().await.unwrap();

    let query = "SELECT serial FROM computers";
    let rows = sqlx::query(query)
        .fetch_all(&mut client)
        .await
        .unwrap();

    for row in rows {
        //println!("{:?}", row.get::<String, _>(0));
        serial_numbers.push(row.get::<String, _>(0));
    }
    return serial_numbers;
}

pub async fn query_serialnum_missing_date() -> Vec<String>{
    let mut serial_numbers = Vec::new();
    let mut client = setup_sqlx_connection().await.unwrap();

    let query = "SELECT serial FROM computers WHERE end_date IS NULL";
    let rows = sqlx::query(query)
        .fetch_all(&mut client)
        .await
        .unwrap();

    for row in rows {
        //println!("{:?}", row.get::<String, _>(0));
        serial_numbers.push(row.get::<String, _>(0));
    }
    return serial_numbers;
}

pub async fn update_computer_db(computers: Vec<(String, String)>) {
    let mut client = setup_sqlx_connection().await.unwrap();
    for computer in computers {
       update_record(&mut client, computer.0, computer.1).await;
    }
}

async fn update_record(client: &mut PgConnection, serial: String, end_date: String) {
    let query = "UPDATE computers SET end_date = $1 WHERE serial = $2";
    sqlx::query(query)
        .bind(end_date)
        .bind(serial)
        .execute(client)
        .await
        .unwrap();
}

