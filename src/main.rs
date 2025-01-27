mod lunch_fetch;
mod embed;


use daemonize::Daemonize;
use std::fs::File;
use std::time::Duration;
use std::thread::sleep;
use chrono::{Local, NaiveTime};

fn main() {
    // Setting up daemon
    let daemon = Daemonize::new()
        .pid_file("daemon.pid")
        .stdout(File::create("stdout.log").unwrap())
        .stderr(File::create("stderr.log").unwrap())
        .working_directory(".");
    let _ = daemon.start();

    loop {
        food_loop(0);
        food_loop(1);
        let now = Local::now();
        let target_time = NaiveTime::from_hms(8, 0, 0); // 8:00 AM
        // Determine the target datetime
        let next_run = if now.time() < target_time {
            // If it's before 8:00 AM today, schedule for today at 8:00 AM
            now.date().and_time(target_time).unwrap()
        } else {
            // If it's past 8:00 AM, schedule for 8:00 AM the next day
            now.date().succ().and_time(target_time).unwrap()
        };
        // Calculate the duration until the next run
        let duration = next_run.signed_duration_since(now);

        sleep(Duration::from_secs(duration.num_seconds() as u64));
    }
}


use serde_json::Value;
use rand::Rng;
fn food_loop(days_forward: i16) {
    let food_struct: Value = lunch_fetch::fetch_food(days_forward as i64 * 86400_i64);

    // Getting todays Lunches
    let mut embed_array: Vec<embed::Embed> = Vec::new();
    // Must handle empty offers (weekends)
    let offer_array = match food_struct["data"]["canteenOffers"].as_array().unwrap().get(0) {
        Some(offer) => offer["food"].as_array().unwrap(),
        None => return,
    };

    // count which lunch i am on
    let mut lunch_counter: u8 = 1_u8;

    // Creating a nice color
    let color: u32 = rand::thread_rng().gen_range(0..=16777215);

    // Construct JSON of each lunch
    for offer in offer_array {
        // Formats the image_url, as it can be missing for some foods
        let image_url = &lunch_fetch::fetch_food_image(offer["id"].as_u64().unwrap_or(0) as u32);
        let mut trimmed_image_url: Option<String> = None;
        if image_url != "" {
            trimmed_image_url = Some((&image_url[1..&image_url.len()-1]).to_string());
        }
        // Formats the date (gotta love the czech language)
        let date: String;
        date = match days_forward {
            0 => "Dnes".to_string(),
            1 => "Zítra".to_string(),
            2 => format!("Za {days_forward} dny").to_string(),
            3 => format!("Za {days_forward} dny").to_string(),
            4 => format!("Za {days_forward} dny").to_string(),
            _ => format!("Za {days_forward} dnů").to_string(),
        };

        // Constructing the embed
        let new_embed = embed::Embed::new(
            lunch_counter,
            offer["name"].as_str().unwrap_or(""),
            trimmed_image_url,
            date,
            color,
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
