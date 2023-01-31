use serde::Deserialize;
use reqwest::{self, ClientBuilder, Error, header::{CONTENT_TYPE, AUTHORIZATION, ACCEPT}};
use crate::token::*;

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
    ship_date: String,
    entitlements: Vec<DellEntitlements>,
}

#[derive(Deserialize, Debug)]
pub struct DellEntitlements {
    #[serde(alias="startDate")]
    start_date: String,
    #[serde(alias="endDate")]
    end_date: String,
}

async fn get_dell_bearer_token() -> Result<BearerToken, Error> {

    let base_url = "https://apigtwb2c.us.dell.com/auth/oauth/v2/token".to_string();
    let body = [("client_id", DELL_ID),
        ("client_secret", DELL_SECRET),
        ("grant_type", "client_credentials")];
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
pub async fn dell_api_query(serial_number: String) -> Result<DellResult, Error> {

    let params = [("servicetags", serial_number)];
    let bearer_token = get_dell_bearer_token().await.unwrap().access_token;
    let token = format!("Bearer {}", bearer_token);
    let base_url = "https://apigtwb2c.us.dell.com/PROD/sbil/eapi/v5/asset-entitlements".to_string();
    let client = ClientBuilder::new().build().unwrap();
    let response = client
        .get(base_url)
        .header(AUTHORIZATION, token)
        .query(&params)
        .send()
        .await?;

    let dell_info: DellResult = response.json().await?;
    //print!("{:?}", response);

    Ok(dell_info)
}
