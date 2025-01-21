mod lunch_fetch;

use serde_json::Value;
fn main() {
    let food_struct: Value = lunch_fetch::fetch_food();
    println!("{}", food_struct.to_string());
    let food_image = lunch_fetch::fetch_food_image(110_u32);
    println!("{}", food_image)
}
