mod cli;
mod genius;
#[cfg(test)]
mod tests;

use cli::Args;
use genius::Genius;

fn main() {
    let cli = Args::parse(std::env::args());

    match cli {
        Err(err) => println!("{err}"),
        Ok(cli) if cli.help => cli::usage(),
        Ok(cli) if cli.url.is_some() => {
            println!("{}", Genius::new().lyrics_from_url(cli.url.unwrap()));
        }
        Ok(cli) if cli.list => {
            let results = Genius::new()
                .search(&cli.artist, &cli.title, cli.max_results.unwrap_or(20))
                .join("\n");
            println!("{results}");
        }
        Ok(cli) => match Genius::new().search_lyrics(cli.artist, cli.title) {
            Ok(lyrics) => println!("{lyrics}"),
            Err(err) => println!("{err}"),
        },
    };
}
