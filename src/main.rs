mod lunch_fetch;
mod embed;

use serde_json::Value;
fn main() {
    food_loop(0_u8);
    food_loop(1_u8);
}

fn food_loop(days_forward: u8) {
    let food_struct: Value = lunch_fetch::fetch_food();

    // Getting todays Lunches
    let mut embed_array: Vec<embed::Embed> = Vec::new();
    let offer_array = food_struct["data"]["canteenOffers"].as_array().unwrap()
        .get(0).unwrap()
        ["food"].as_array().unwrap();

    let mut lunch_counter: u8 = 1_u8;
    // Construct JSON of each lunch
    for offer in offer_array {
        // Formats the image_url, as it can be missing for some foods
        let image_url = &lunch_fetch::fetch_food_image(offer["id"].as_u64().unwrap_or(0) as u32);
        let mut trimmed_image_url: Option<String> = None;
        if image_url != "" {
            trimmed_image_url = Some((&image_url[1..&image_url.len()-1]).to_string());
        }

        // Constructing the embed
        let new_embed = embed::Embed::new(
            lunch_counter,
            offer["name"].as_str().unwrap_or(""),
            trimmed_image_url,
            5_u32,
            offer["averageRating"].to_string(),
        );
        embed_array.push(new_embed);
        lunch_counter += 1;
    }

    let webhhook_url = dotenv::var("WEBHOOK_URL").unwrap();
    let discord_body = DiscordBody{embeds: embed_array};
    send_webhook(webhhook_url, discord_body);
}

// Function for sending webhooks to discord
use reqwest::blocking::Client;
fn send_webhook(url: String, body: DiscordBody) {
    let client = Client::new();

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .send().unwrap();

    // It is nice to do this, just to monitor discord
    // Status 204 is fine, it means embeds were sent without any extra message
    println!("Status: {}", &response.status());
    println!("Response: {}", &response.text().unwrap());
}
// This is here, because Embeds must be a Dict and not an array
// Dumbass decision by discord tbh
use serde::Serialize;
#[derive(Serialize)]
struct DiscordBody {
    embeds: Vec<embed::Embed>,
}
