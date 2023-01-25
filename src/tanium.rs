use serde::Deserialize;
use reqwest::{self, ClientBuilder, Error, header::{CONTENT_TYPE, AUTHORIZATION}};
use serde_json::{json, Value as JsonValue};
use crate::token::TOKEN;

#[derive(Deserialize, Debug)]
pub struct TaniumResponse {
    data: TaniumData,
}

#[derive(Deserialize, Debug)]
struct TaniumData {
    endpoints: TaniumEndpoints,
}

#[derive(Deserialize, Debug)]
struct TaniumEndpoints{
    edges: Vec<TaniumEdges>,
    #[serde(alias="pageInfo")]
    page_info: TaniumPageInfo, 
}

#[derive(Deserialize, Debug)]
struct TaniumEdges {
    node: TaniumNodes,
}

#[derive(Deserialize, Debug)]
struct TaniumNodes{
    id: String,
    #[serde(alias="serialNumber")]
    serial_number: String,
    name: String,
    model: String,
    manufacturer: String
}

#[derive(Deserialize, Debug)]
struct TaniumPageInfo {
    #[serde(alias="hasNextPage")]
    has_next_page: bool,
    #[serde(alias="endCursor")]
    end_cursor: String,
}

struct Computer {
    name: String,
    serial_number: String,
}


#[tokio::main]
async fn tanium_api_call(query: String) -> Result<TaniumResponse, Error> {

    let base_url = "https://hm-api.cloud.tanium.com/plugin/products/gateway/graphql".to_string();
    let client = ClientBuilder::new().build().unwrap();
    let response = client
        .post(base_url)
        .header(CONTENT_TYPE, "application/json")
        .header("session", TOKEN)
        .body(query)
        .send()
        .await?;

    let tanium_response: TaniumResponse = response.json().await?;

    //println!("{:?}", tanium_response.data.endpoints.edges);

    Ok(tanium_response)
}


pub fn get_pages() {
    let query_start: &str = r#"{
    "query": "{ endpoints ("#;
    let query_end: &str = r#"filter: {path: \"manufacturer\", op: CONTAINS, value: \"Dell\" }) { edges { node { id serialNumber name model manufacturer } } pageInfo { hasNextPage endCursor } }}"
    }"#;

    let base_query: String = r#"{
    "query": "{ endpoints (filter: {path: \"manufacturer\", op: CONTAINS, value: \"Dell\" }) { edges { node { id serialNumber name model manufacturer } } pageInfo { hasNextPage endCursor } }}" 
    }"#.to_string();
    let mut end_cursor = ""; 
    let end_cursor_string = format!("after: \"{}\" ", end_cursor); 
    let query = query_start.to_owned() + &end_cursor_string + query_end;
    println!("{}", base_query);
    println!("{}", query);

    let base_call = tanium_api_call(base_query.to_string());
    let base_result = match base_call {
        Ok(r) => r,
        Err(error) => panic!("Problem with the Tanium API response: {}", error)
    }; 

    println!("{:?}", base_result);

    end_cursor = &base_result.data.endpoints.page_info.end_cursor;
    let next_page = &base_result.data.endpoints.page_info.has_next_page;

    while *next_page {
        match tanium_api_call(query.clone()) {
            Ok(r) => println!("{:?}", r),
            Err(error) => panic!("{}", error)
        }
    }
    

    //next_page = tanium_response.data.endpoints.page_info.has_next_page;
    //let mut next_page = true;

}





fn parse_response(response: TaniumResponse) -> Vec<Computer> {
    let mut computers: Vec<Computer> = Vec::new(); 
    
    

    return computers
}



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
