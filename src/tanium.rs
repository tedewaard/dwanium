use serde::Deserialize;
use reqwest::{self, ClientBuilder, Error, header::CONTENT_TYPE};

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


#[tokio::main]
async fn tanium_api_call(query: String) -> Result<TaniumResponse, Error> {
    //dotenv().ok();
    let token = dotenv::var("TOKEN").expect("There was an issue reading the tanium token env variable.");
    let base_url = "https://hm-api.cloud.tanium.com/plugin/products/gateway/graphql".to_string();
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

fn format_query(end_cursor: String) -> String {
    let query_start: &str = r#"{
    "query": "{ endpoints (first: 1000, "#;
    let query_end: &str = r#"filter: {path: \"manufacturer\", op: CONTAINS, value: \"Dell\" }) { edges { node { id serialNumber name model manufacturer } } pageInfo { hasNextPage endCursor } }}"
    }"#;
    let end_cursor_string = format!("after: \\\"{}\\\" ", end_cursor); 
    let query = query_start.to_owned() + &end_cursor_string + query_end;
    return query
}


fn get_pages() -> Vec<TaniumResponse>{
    let mut all_responses: Vec<TaniumResponse> = Vec::new();
    let base_query: String = r#"{
    "query": "{ endpoints (first: 1000, filter: {path: \"manufacturer\", op: CONTAINS, value: \"Dell\" }) { edges { node { id serialNumber name model manufacturer } } pageInfo { hasNextPage endCursor } }}" 
    }"#.to_string();

    let base_call = tanium_api_call(base_query.to_string());
    let base_result = match base_call {
        Ok(r) => r,
        Err(error) => panic!("Problem with the Tanium API response. Check that the API token is valid: {}", error)
    }; 
    

    all_responses.push(base_result);

    let mut end_cursor = &all_responses[0].data.endpoints.page_info.end_cursor;
    let mut next_page = &all_responses[0].data.endpoints.page_info.has_next_page;

    while *next_page {
        let query = format_query(end_cursor.clone());
        match tanium_api_call(query.clone()) {
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

fn parse_responses(responses: Vec<TaniumResponse>) -> Vec<Computer> {
    let mut computers: Vec<Computer> = Vec::new(); 
    
    for response in responses {
        for edges in response.data.endpoints.edges {
            let comp = Computer {name: edges.node.name, serial_number: edges.node.serial_number};
            computers.push(comp);
        }
    }

    return computers
}

pub fn get_computers() -> Vec<Computer> {
    let responses = get_pages();
    let computers = parse_responses(responses);
    return computers
}
