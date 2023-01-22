use serde::Deserialize;
use reqwest::{self, ClientBuilder, header::{CONTENT_TYPE, AUTHORIZATION}};
use serde_json::{json, Value as JsonValue};
use crate::token::TOKEN;


#[derive(Deserialize, Debug)]
struct TaniumResponse {
    edges: Vec<TaniumPC>,
    has_next_page: bool,
    end_cursor: String,
   
}

#[derive(Deserialize, Debug)]
struct TaniumPC {
    id: u32,
    serial_number: String,
    name: String,
    model: String,
    manufacturer: String
}



#[tokio::main]
pub async fn tanium_api_call() {
    let base_url = "https://hm-api.cloud.tanium.com/plugin/products/gateway/graphql".to_string();
    let client = ClientBuilder::new().build().unwrap();
    let response = client
        .post(base_url)
        .header(CONTENT_TYPE, "application/json")
        .header("session", TOKEN)
        .body(QUERY)
        .send()
        .await
        .unwrap();


    let t_response: TaniumResponse  = response.json().await.unwrap();
    println!("{:?}", t_response);
}



const QUERY: &str = r#"{
  "query": "{ endpoints (filter: {path: \"manufacturer\", op: CONTAINS, value: \"Dell\" }) { edges { node { id serialNumber name model manufacturer } } pageInfo { hasNextPage endCursor } }}"
}"#;




/*
#[derive(Deserialize)]
struct CatFact {
    fact: String,
}


#[tokio::main]
pub async fn test_api_call() {
    let url = "https://catfact.ninja/fact";
    let mut facts: Vec<CatFact> = Vec::new();

    let mut i = 0; 
    while i < 10 {
        let response = reqwest::get(url).await.unwrap();
        let fact: CatFact = response.json().await.unwrap();
        facts.push(fact);
        i += 1;
    }

    println!("{:?}", facts.into_iter().map(|x| x.fact).collect::<Vec<String>>());
}
*/
