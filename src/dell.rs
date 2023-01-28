use serde::Deserialize;
use reqwest::{self, ClientBuilder, Error, header::{CONTENT_TYPE, AUTHORIZATION}};
use crate::token::*;

#[derive(Deserialize, Debug)]
pub struct BearerToken{
    access_token: String
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


pub async fn trial_run() {
    let token = get_dell_bearer_token().await.unwrap();
    println!("{:?}", token.access_token);
}

#[tokio::main]
pub async fn dell_api_query() -> Result<(), Error> {

    let params = [("servicetags", "btchnn3")];
    let bearer_token = get_dell_bearer_token().await.unwrap().access_token;
    let token = format!("Bearer {}", bearer_token);
    let base_url = "https://apigtwb2c.us.dell.com/PROD/sbil/eapi/v5/assets".to_string();
    let client = ClientBuilder::new().build().unwrap();
    let response = client
        .post(base_url)
        .header(AUTHORIZATION, token)
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    //let pc_info = response.json().await?;
    print!("{:?}", response);

    Ok(())
}
