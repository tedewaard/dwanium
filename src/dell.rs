
use serde::Deserialize;
use reqwest::{self, ClientBuilder, Error, header::{CONTENT_TYPE, AUTHORIZATION}};
use chrono::DateTime;

#[derive(Deserialize, Debug)]
struct BearerToken{
    access_token: String
}

pub type DellResult = Vec<DellObject>;

#[derive(Deserialize, Debug)]
pub struct DellObject {
    #[serde(alias="serviceTag")]
    service_tag: String,
    #[serde(alias="shipDate")]
    ship_date: Option<String>,
    entitlements: Option<Vec<DellEntitlements>>,
}

//There is a serviceLevelDescription field that I'm not grabbbing but describest the warranty
#[derive(Deserialize, Debug)]
pub struct DellEntitlements {
    #[serde(alias="startDate")]
    start_date: String,
    #[serde(alias="endDate")]
    end_date: String,
}

async fn get_dell_bearer_token() -> Result<BearerToken, Error> {
    let dell_id = dotenv::var("DELL_ID").expect("Error reading dell id env variable.");
    let dell_secret = dotenv::var("DELL_SECRET").expect("Error reading dell secret env variable.");
    let base_url = "https://apigtwb2c.us.dell.com/auth/oauth/v2/token".to_string();
    let body = [("client_id", dell_id),
        ("client_secret", dell_secret.to_string()),
        ("grant_type", "client_credentials".to_string())];
    let client = ClientBuilder::new().build().unwrap();
    let response = client
        .post(base_url)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&body)
        .send()
        .await?;

    let token: BearerToken = response.json().await?;

    Ok(token)
}


#[tokio::main]
pub async fn dell_api_query(serial_number: Vec<String>) -> Result<DellResult, Error>{

    let mut query_string = "servicetags=".to_string();
    let mut serial_string = String::new();
    if serial_number.len() == 1 {
        serial_string = serial_number.first().unwrap().to_string();
    } else if serial_number.len() > 1 {
        serial_string = serial_number.first().unwrap().to_string();
        for i in 1..serial_number.len() {
            serial_string = format!("{},{}", serial_string, serial_number[i]);
        }
    }
    query_string = format!("{}{}", query_string, serial_string);
    //println!("{}", query_string);
    let bearer_token = get_dell_bearer_token().await.unwrap().access_token;
    let token = format!("Bearer {}", bearer_token);
    let base_url = format!("https://apigtwb2c.us.dell.com/PROD/sbil/eapi/v5/asset-entitlements?{}", query_string);
    let client = ClientBuilder::new().build().unwrap();
    let response = client
        .get(base_url)
        .header(AUTHORIZATION, token)
        //.query(&params)
        //.build();
        .send()
        .await?;

    let dell_info: DellResult = response.json().await?;
    //println!("{:?}", response);

    Ok(dell_info)
}

pub fn map_to_serial_and_enddate(dell_result: DellResult) -> Vec<(String,String)> {
    let mut computers = Vec::new();
    for object in dell_result {
        //Going to skip objects that don't have entitlements and therefor no end data
        if object.entitlements.is_some() { 
            //Some computers have multiple entitlements. We are grabbing the date furthest out. 
            let entitlements = object.entitlements.expect("No Dell entitlements for object.");
            let mut dates = Vec::new();
            for entitlement in entitlements {
                let date = DateTime::parse_from_rfc3339(&entitlement.end_date);
                match date {
                    Ok(date) => dates.push(date),
                    Err(err) => println!("Failed to parse date. Err: {}", err),
                }
            }
            dates.sort();
            let latest_date = dates.pop(); 

            if latest_date.is_some() {
                let short_date = latest_date.expect("There is no latest date.").to_string();
                let computer = (object.service_tag.to_owned(),
                short_date[0..10].to_string());
                computers.push(computer);
            }
        }
    }
    return computers;
}
