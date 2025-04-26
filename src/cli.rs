use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub struct Args {
    pub title: String,
    pub artist: String,
    pub url: Option<String>,
    pub list: bool,
    pub max_results: Option<usize>,
    pub help: bool,
}

#[derive(Debug, PartialEq, PartialOrd, Ord)]
pub enum InvalidArgsError {
    MissingArgument,
    Unknown(String),
    MissingValue(String),
    InvalidValue((String, String)),
}

impl Args {
    fn new() -> Self {
        Args {
            title: "".to_string(),
            artist: "".to_string(),
            url: None,
            list: false,
            max_results: None,
            help: false,
        }
    }

    /// Should only be called with `std::env::args()` as argument.
    /// The argument is only there to enable testing.
    pub fn parse<T: Iterator<Item = String>>(args: T) -> Result<Self, InvalidArgsError> {
        let sanatized_args = args
            .skip(1) // first arg is the executable name
            .map(|a| a.trim().to_string())
            .filter(|a| !a.is_empty())
            .collect::<Vec<String>>();

        // Split arguments into `&["--option", "some value", ...]` slices
        let arg_slices = sanatized_args
            .chunk_by(|a, b| a.starts_with("--") && !b.starts_with("--"))
            .collect::<Vec<&[String]>>();

        if arg_slices.is_empty() {
            self::usage();
            return Err(InvalidArgsError::MissingArgument);
        }

        let mut args = Self::new();

        for arg_slice in arg_slices {
            args.parse_arg_slice(arg_slice)?; // Early return on argument parsing error
        }

        if args.title.is_empty() && args.url.as_ref().is_some_and(|u| u.is_empty()) {
            return Err(InvalidArgsError::MissingArgument);
        }

        Ok(args)
    }

    fn parse_arg_slice(&mut self, arg_pair: &[String]) -> Result<(), InvalidArgsError> {
        match arg_pair {
            [flag] => self.parse_flag_arg(flag)?,
            [arg_name, arg_value] => self.parse_arg_pair((arg_name, arg_value))?,
            [arg, ..] => return Err(InvalidArgsError::MissingValue(arg.to_string())),
            _ => return Err(InvalidArgsError::MissingArgument),
        };
        Ok(())
    }

    fn parse_flag_arg(&mut self, flag: &str) -> Result<(), InvalidArgsError> {
        match flag.trim_start_matches("--") {
            "list" => self.list = true,
            "help" => self.help = true,
            "title" | "artist" | "url" => {
                return Err(InvalidArgsError::MissingValue(flag.to_string()))
            }
            _ => return Err(InvalidArgsError::Unknown(flag.to_string())),
        };

        Ok(())
    }

    fn parse_arg_pair(&mut self, (arg, val): (&str, &str)) -> Result<(), InvalidArgsError> {
        match (arg.trim_start_matches("--"), val) {
            (arg @ "title" | arg @ "artist" | arg @ "url", "") => {
                return Err(InvalidArgsError::MissingValue(arg.to_string()))
            }
            ("title", title) => self.title = title.to_string(),
            ("artist", artist) => self.artist = artist.to_string(),
            ("url", url) => self.url = Some(url.to_string()),
            ("list", max_results) => {
                self.list = true;
                let max = max_results.parse::<usize>().map_err(|_| {
                    InvalidArgsError::InvalidValue((arg.to_string(), max_results.to_string()))
                })?;
                self.max_results = Some(max);
            }
            (unknown_arg, _) => return Err(InvalidArgsError::Unknown(unknown_arg.to_string())),
        }

        Ok(())
    }
}

/// Print help text
pub fn usage() {
    println!(
        "Fetch and print plain text song lyrics from genius.com.

Available options:

--help                       Print this help text and exit.
--title  <title>  (required) Title of the song to search lyrics for.
--artist <artist> (optional) Restrict search results by the name of the song's
                             artist.
--url    <url>    (optional) Rather than searching, fetch lyrics from this
                             genius.com url directly. If present `--artist`,
                             `--title`, and `--list` will be ignored.
--list   [count]  (optional) Print list of [count] (default 20) genius.com search
                             results, rather than song lyrics. Useful for 
                             further filtering, manually or through external tools.
"
    );
}

impl Error for InvalidArgsError {}

impl Eq for InvalidArgsError {}

impl Display for InvalidArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingArgument => {
                write!(
                    f,
                    "At least one of `--title` or `--url` arguments is required"
                )
            }
            Self::Unknown(arg) => write!(f, "Argument `{arg}` unknown"),
            Self::MissingValue(arg) => write!(f, "Argument `{arg}` requires a value"),
            Self::InvalidValue((arg, val)) => write!(f, "Argument `{arg}` has invalid value {val}"),
        }
    }
}
