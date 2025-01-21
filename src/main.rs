mod lunch_fetch;

use serde_json::Value;
fn main() {
    let food_struct: Value = serde_json::from_str(&lunch_fetch::fetch_food()).unwrap();
    println!("{}", food_struct.to_string());
}
