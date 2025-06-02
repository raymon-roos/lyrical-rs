use reqwest::{blocking::Client, IntoUrl, Url};
use scraper::{Html, Selector};
use serde_json::Value;
use std::{error::Error, fmt::Display};

pub struct Genius {
    client: Client,
    api_token: String,
}

#[derive(Debug, PartialEq, PartialOrd, Ord)]
pub enum GeniusError {
    NoResultsFound(String),
}

impl Genius {
    pub fn new(api_token: String) -> Self {
        Self {
            client: Self::create_client(),
            api_token,
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
        let artist = artist.to_lowercase();
        let title = title.to_lowercase();
        let query = format!("{artist} {title}");
        let matches = self.query(query.clone());
        let mut results = Self::filter_matches(&matches, &artist, &title, max_results);

        // Sometimes, a song's artist/title contains a segment like "(Ft. Other Artist)",
        // "(XYZ remix)", (Live in Some Place), (Original), (clean), (by Somebody), or
        // (Unreleased). This can mess with the search results. Should the search term
        // contain parentheses, and no result had been found, then try again having
        // removed this segment.
        let delimiters = ['(', '{', '['];
        if results.is_empty() && title.contains(delimiters) {
            // Primitive implementation, assuming parenthesized segment is always at
            // the end of the title
            if let Some((new_title, _)) = title.split_once(delimiters) {
                let new_title = new_title.trim();
                let matches = self.query(format!("{artist} - {new_title}"));
                eprintln!("INFO: retrying search with `{artist} - {new_title}`");
                results = Self::filter_matches(&matches, &artist, new_title, max_results);
            }
        }

        results
    }

    fn query(&self, query: String) -> Vec<Value> {
        let matches = self
            .client
            .get("https://api.genius.com/search")
            .query(&[("q", query), ("per_page", "20".to_string())])
            .bearer_auth(&self.api_token)
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

    fn filter_matches(
        matches: &[Value],
        artist: &str,
        title: &str,
        max_results: usize,
    ) -> Vec<String> {
        let mut results: Vec<String> = Vec::with_capacity(max_results);

        for result in matches {
            if results.len() >= max_results {
                break;
            }

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

        results
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