use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde_json::Value;
use std::{env, fs::read_to_string};

fn main() {
    let (artist, title) = parse_args();
    let client = create_client();
    let song_url = find_song_url(&client, artist, title);
    let document = get_lyrics_page(&client, song_url.as_str());
    println!(
        "Lyrics retrieved from {song_url}\n\n{}",
        get_lyrics_from_page(document)
    );
}

fn parse_args() -> (String, String) {
    let artist = env::args().nth(1).expect("missing artist & title argument");
    let title = env::args().nth(2).expect("missing title argument");

    (artist.trim().to_lowercase(), title.trim().to_lowercase())
}

fn create_client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .https_only(true)
        .build()
        .expect("failed to create HTTP client")
}

fn find_song_url(client: &Client, artist: String, title: String) -> String {
    client
        .get("https://api.genius.com/search")
        .query(&[
            ("q", format!("{artist} {title}")),
            ("per_page", "1".to_string()),
        ])
        .bearer_auth(read_token())
        .send()
        .expect("failed to parse URL")
        .json::<Value>()
        .expect("failed to read response of search query")["response"]["hits"][0]["result"]["url"]
        .to_string()
        .replace("\"", "")
}

fn get_lyrics_page(client: &Client, url: &str) -> Html {
    Html::parse_document(
        client
            .get(url)
            .send()
            .expect("failed to parse URL")
            .text()
            .expect("failed to decode song page response")
            .replace("<br/>", "\n")
            .as_str(),
    )
}

fn get_lyrics_from_page(document: Html) -> String {
    let selector = Selector::parse(r#"div[data-lyrics-container="true"]"#).unwrap();
    let verse_separator = ["\n"];
    document
        .select(&selector)
        .flat_map(|e| e.text().chain(verse_separator))
        .collect()
}

fn read_token() -> String {
    let token = read_to_string(env::var("XDG_CONFIG_HOME").unwrap() + "/lyrical/token")
        .expect("Failed to read token from $XDG_COFIG_HOME/lyrical/token");

    token
        .lines()
        .next()
        .expect("failed to load Genius API access token from file")
        .to_string()
}
