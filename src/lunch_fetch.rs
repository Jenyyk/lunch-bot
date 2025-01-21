// This file contains methods that interact with the "Na tácu" API

const API_URL: &str = "https://apiv2.natacu.cz/graphql";

use serde_json::{Value, json};
use reqwest::blocking::Client;

pub fn fetch_food() -> Value {
    let canteen_id: u8 = dotenv::var("CANTEEN_ID").unwrap_or("1".to_string()).parse().unwrap_or(1);
    // Multiplied by 1000, because the API takes values in milliseconds, not seconds
    let timestamp: i64 = chrono::Utc::now().timestamp() * 1000;
    // Parse the request body
    // We do this to modify the query variables
    let request_body_str = r#"{
        "operationName": "canteenOffersQuery",
        "variables": {
            "query": {
                "canteenId": 0,
                "from": "0",
                "to": "0",
                "order": "ASC"
            }
        },
        "query": "query canteenOffersQuery($query: GetOffersInput!) {\n  canteenOffers(query: $query) {\n    id\n    date\n    food {\n      id\n      name\n      averageRating\n      __typename\n    }\n    __typename\n  }\n}"
    }"#;
    let mut request_body: Value = serde_json::from_str(request_body_str).unwrap();

    // Modifying the request body
    request_body["variables"]["query"]["canteenId"] = json!(canteen_id);
    request_body["variables"]["query"]["from"] = json!(timestamp.to_string());
    request_body["variables"]["query"]["to"] = json!(timestamp.to_string());


    // Sending the request
    let client = Client::new();
    let food_response = client.post(API_URL)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_body).unwrap())
        .send().unwrap();

    serde_json::from_str(&food_response.text().unwrap()).unwrap()
}

pub fn fetch_food_image(id: u32) -> String {
    // Parse the request body
    // We do this to modify the query variables
    let request_body_str = r#"{
        "operationName":"foodQuery",
        "variables":{
            "id":0
        },
        "query":"query foodQuery($id: Int!) {\n  food(id: $id) {\n    id\n    name\n    description\n    canteenId\n    averageRating\n    similarNames {\n      alternateName\n      __typename\n    }\n    photos {\n      id\n      s3url\n      __typename\n    }\n    __typename\n  }\n}"
    }"#;
    let mut request_body: Value = serde_json::from_str(request_body_str).unwrap();
    // Modifying the request body
    request_body["variables"]["id"] = json!(id);

    // Sending the request
    let client = Client::new();
    let image_response = client.post(API_URL)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_body).unwrap())
        .send().unwrap();

    let serde_object: Value = serde_json::from_str(&image_response.text().unwrap()).unwrap();
    // JSON formatting magic to return the s3 url
    let photos = serde_object["data"]["food"]["photos"].as_array().unwrap();
    let first_photo = match photos.get(0) {
        Some(photo) => photo,
        _ => return "".to_string(),
    };
    first_photo["s3url"].to_string()
}
