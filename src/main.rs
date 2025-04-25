use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde_json::Value;
use std::{env, fs::read_to_string};

fn main() {
    let (artist, title) = parse_args();
    let client = create_client();
    let results = search(&client, artist, title);
    let song_url = results.first().expect("No search results found");
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

fn search(client: &Client, artist: String, title: String) -> Vec<String> {
    let matches = query(client, format!("{artist} {title}"));

    let mut results: Vec<String> = Vec::with_capacity(matches.len() / 2);

    for result in matches.iter() {
        match result["type"].as_str() {
            Some("song") => (),
            _ => continue,
        }

        if !artist.is_empty() {
            match result["result"]["artist_names"].as_str() {
                Some(names) if names.to_lowercase().contains(&artist) => (),
                _ => continue,
            }
        }

        match result["result"]["title_with_featured"].as_str() {
            Some(song_title) if song_title.to_lowercase().contains(&title) => (),
            _ => continue,
        }

        results.push(
            result["result"]["url"]
                .as_str()
                .expect("failed to deserialize url from results")
                .to_string(),
        );
    }

    results
}

fn query(client: &Client, query: String) -> Vec<Value> {
    let matches = client
        .get("https://api.genius.com/search")
        .query(&[("q", query), ("per_page", "20".to_string())])
        .bearer_auth(read_token())
        .send()
        .expect("failed to parse URL")
        .json::<Value>()
        .expect("failed to deserialize response")["response"]["hits"]
        .clone();

    matches
        .as_array()
        .expect("failed to deserialize results")
        .clone()
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
