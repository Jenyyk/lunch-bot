// This file creates structs for serde_json to serialize into JSON
use serde::Serialize;

#[derive(Serialize)]
pub struct Embed {
    author: Author,
    description: String,
    thumbnail: Thumbnail,
    color: u32,
    footer: Footer,
}

impl Embed {
    pub fn new(lunch_number: u8, lunch_name: &str, image_url: Option<String>, embed_color: u32, mut rating: String) -> Self {
        if rating == "null" { rating = "Bez hodnocení".to_string(); }
        else { rating = (&rating[..3]).to_string(); }
        Self {
            author: Author{
                name: ("Oběd ".to_string() + &lunch_number.to_string()).to_string(),
                icon_url: "https://www.gypce.cz/wp-content/uploads/2013/06/gypce-1.jpg".to_string(),
            },
            description: ("# ".to_string() + lunch_name),
            thumbnail: Thumbnail{ url: image_url, },
            color: embed_color,
            footer: Footer{
                text: rating,
                icon_url: "https://png.pngtree.com/png-vector/20230222/ourmid/pngtree-shiny-yellow-star-icon-clipart-png-image_6613580.png".to_string(),
            },
        }
    }
}

#[derive(Serialize)]
struct Author {
    name: String,
    icon_url: String,
}

#[derive(Serialize)]
struct Thumbnail {
    url: Option<String>
}

#[derive(Serialize)]
struct Footer {
    text: String,
    icon_url: String,
}
