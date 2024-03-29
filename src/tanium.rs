use serde::Deserialize;
use reqwest::{self, ClientBuilder, Error, header::CONTENT_TYPE};
use std::collections::HashSet;

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
    //id: String,
    #[serde(alias="serialNumber")]
    serial_number: String,
    name: String,
   // model: String,
    //manufacturer: String
}

#[derive(Deserialize, Debug)]
struct TaniumPageInfo {
    #[serde(alias="hasNextPage")]
    has_next_page: bool,
    #[serde(alias="endCursor")]
    end_cursor: String,
}

#[derive(Debug, Clone)]
pub struct Computer {
    pub name: String,
    pub serial_number: String,
}

fn get_base_url() -> String {
    let target = dotenv::var("API_TARGET").expect("There was an issue reading the tanium API_Target env variable.");
    let base_url = format!("https://{}-api.cloud.tanium.com/plugin/products/gateway/graphql", target);
    return base_url;
}


async fn tanium_api_call_mutation(query: String) -> Result<String, Error> {
    //dotenv().ok();
    let token = dotenv::var("TOKEN").expect("There was an issue reading the tanium token env variable.");
    let base_url = get_base_url();
    let client = ClientBuilder::new().build().unwrap();
    let response = client
        .post(base_url)
        .header(CONTENT_TYPE, "application/json")
        .header("session", token)
        .body(query)
        //.build()?;
        .send()
        .await?;

    //println!("{:?}", response.body().unwrap());
    let body = response.text().await?;

    //Ok("Test".to_string())
    Ok(body)
}

async fn tanium_api_call(query: String) -> Result<TaniumResponse, Error> {
    //dotenv().ok();
    let token = dotenv::var("TOKEN").expect("There was an issue reading the tanium token env variable.");
    let base_url = get_base_url();
    let client = ClientBuilder::new().build().unwrap();
    let response = client
        .post(base_url)
        .header(CONTENT_TYPE, "application/json")
        .header("session", token)
        .body(query)
        .send()
        .await?;

    let tanium_response: TaniumResponse = response.json().await?;

    //println!("{:?}", tanium_response.data.endpoints.edges);


    Ok(tanium_response)
}

fn format_request_query(end_cursor: String) -> String {
    let query_start: &str = r#"{
    "query": "{ endpoints (first: 1000, "#;
    let query_end: &str = r#"filter: {path: \"manufacturer\", op: CONTAINS, value: \"Dell\" }) { edges { node { id serialNumber name model manufacturer } } pageInfo { hasNextPage endCursor } }}"
    }"#;
    let end_cursor_string = format!("after: \\\"{}\\\" ", end_cursor); 
    let query = query_start.to_owned() + &end_cursor_string + query_end;
    return query
}


async fn get_pages() -> Vec<TaniumResponse>{
    let mut all_responses: Vec<TaniumResponse> = Vec::new();
    let base_query: String = r#"{
    "query": "{ endpoints (first: 1000, filter: {path: \"manufacturer\", op: CONTAINS, value: \"Dell\" }) { edges { node { id serialNumber name model manufacturer } } pageInfo { hasNextPage endCursor } }}" 
    }"#.to_string();

    let base_call = tanium_api_call(base_query.to_string()).await;
    let base_result = match base_call {
        Ok(r) => r,
        Err(error) => panic!("Problem with the Tanium API response. Check that the API token is valid: {}", error)
    }; 
    

    all_responses.push(base_result);

    let mut end_cursor = &all_responses[0].data.endpoints.page_info.end_cursor;
    let mut next_page = &all_responses[0].data.endpoints.page_info.has_next_page;

    while *next_page {
        let query = format_request_query(end_cursor.clone());
        match tanium_api_call(query.clone()).await {
            Ok(r) => {
                all_responses.push(r);
                end_cursor = &all_responses.last().unwrap().data.endpoints.page_info.end_cursor;
                next_page = &all_responses.last().unwrap().data.endpoints.page_info.has_next_page},
            Err(error) => panic!("{}", error)
        }
    }

    //next_page = tanium_response.data.endpoints.page_info.has_next_page;
    return all_responses;

}

//TODO: Can start with HashSet and avoid the first array
fn parse_responses(responses: Vec<TaniumResponse>) -> Vec<Computer> {
    let mut computers: Vec<Computer> = Vec::new(); 
    let mut unique_computers = HashSet::new();
    for response in responses {
        for edges in response.data.endpoints.edges {
            if edges.node.serial_number.len() == 7 {
                let comp = Computer {name: edges.node.name, serial_number: edges.node.serial_number};
                computers.push(comp);
            }
        }
    }

    let arr = computers.into_iter().filter(|c| unique_computers.insert(c.serial_number.clone())).collect();

    return arr;
}

pub async fn get_computers() -> Vec<Computer> {
    let responses = get_pages().await;
    let computers = parse_responses(responses);
    return computers
}

//This will handle pushing the enddates to tanium asset
pub async fn push_end_date_to_tanium(records: Vec<(String, String)>) {
    let base_query: String = r#"{
        "query": "mutation importAssets($source: String!, $json: String!) {assetsImport(input: {sourceName: $source, json: $json}) {assets {id index status } }}",
        "variables": {"source": "Dell Warranty End Date","#.to_string();
    let query = format_import_query(records);
    let final_query = format!("{}{}}}}}", base_query, query);
    let response = tanium_api_call_mutation(final_query).await;
    match response {
        Err(error) => println!("Error writing dell warranty end dates to Tanium. {}", error),
        _ => () 
    }
}

pub fn format_import_query(records: Vec<(String, String)>) -> String {
    let base: String = r#""json": "["#.to_string();
    let mut v = Vec::new();

    for record in records {
        let s = format!(
            r#"{{\"serial\": \"{}\", \"end_date\": \"{}\"}}"#,
            record.0, record.1
            ); 
        v.push(s);
    }
    
    let formated_query = format!("{}{}]\"", base, v.join(","));
    return formated_query;
}
