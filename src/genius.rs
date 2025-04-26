use reqwest::{blocking::Client, IntoUrl, Url};
use scraper::{Html, Selector};
use serde_json::Value;
use std::{env, error::Error, fmt::Display, fs::read_to_string};

pub struct Genius {
    client: Client,
}

#[derive(Debug, PartialEq, PartialOrd, Ord)]
pub enum GeniusError {
    NoResultsFound(String),
}

impl Genius {
    pub fn new() -> Self {
        Self {
            client: Self::create_client(),
        }
    }

    pub fn search_lyrics(&self, artist: &str, title: &str) -> Result<String, GeniusError> {
        let results = self.search(artist, title, 1);

        if results.is_empty() {
            return Err(GeniusError::NoResultsFound(format!("{artist} - {title}")));
        }

        Ok(self.lyrics_from_url(results.first().unwrap()))
    }

    pub fn lyrics_from_url<U: IntoUrl + Display>(&self, url: &U) -> String {
        let parsed_url = Url::parse(url.as_str()).expect("URL is invalid");

        let page = self.get_lyrics_page(parsed_url);

        let lyrics = Self::get_lyrics_from_page(&page);

        format!("Lyrics retrieved from {url}\n\n{lyrics}")
    }

    pub fn search(&self, artist: &str, title: &str, max_results: usize) -> Vec<String> {
        let matches = self.query(format!("{artist} {title}"));

        let mut results: Vec<String> = Vec::with_capacity(max_results);

        for result in &matches {
            match result["type"].as_str() {
                Some("song") => (),
                _ => continue,
            }

            if !artist.is_empty() {
                match result["result"]["artist_names"].as_str() {
                    Some(names) if names.to_lowercase().contains(artist) => (),
                    _ => continue,
                }
            }

            match result["result"]["title_with_featured"].as_str() {
                Some(song_title) if song_title.to_lowercase().contains(title) => (),
                _ => continue,
            }

            results.push(
                result["result"]["url"]
                    .as_str()
                    .expect("failed to deserialize url from results")
                    .to_string(),
            );
        }

        results.into_iter().take(max_results).collect()
    }

    fn query(&self, query: String) -> Vec<Value> {
        let matches = self
            .client
            .get("https://api.genius.com/search")
            .query(&[("q", query), ("per_page", "20".to_string())])
            .bearer_auth(Self::read_token())
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

    fn get_lyrics_page(&self, url: Url) -> Html {
        Html::parse_document(
            self.client
                .get(url)
                .send()
                .expect("failed to request lyrics, to many redirects or network error?")
                .text()
                .expect("failed to decode song page response")
                .replace("<br/>", "\n")
                .as_str(),
        )
    }

    fn get_lyrics_from_page(page: &Html) -> String {
        let selector = Selector::parse(r#"div[data-lyrics-container="true"]"#).unwrap();
        let verse_separator = ["\n"];
        page.select(&selector)
            .flat_map(|e| e.text().chain(verse_separator))
            .collect()
    }

    fn create_client() -> reqwest::blocking::Client {
        reqwest::blocking::Client::builder()
            .https_only(true)
            .build()
            .expect("failed to create HTTP client")
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
}

impl Error for GeniusError {}

impl Eq for GeniusError {}

impl Display for GeniusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoResultsFound(search) => write!(f, "No lyrics found for `{search}`"),
        }
    }
}
