use serde::Deserialize;
use reqwest;



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

