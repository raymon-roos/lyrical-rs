use crate::cli::*;

fn fake_args<'a>(args: &'a [&'a str]) -> impl Iterator<Item = String> + 'a {
    std::iter::once(String::new()).chain(args.iter().map(ToString::to_string))
}

#[test]
fn error_on_missing_args() {
    assert_eq!(
        Args::parse(fake_args(&[""])).err(),
        Some(InvalidArgsError::MissingArgument)
    );
}

#[test]
fn can_parse_arg_with_value() {
    assert_eq!(
        Args::parse(fake_args(&["--title"])),
        Err(InvalidArgsError::MissingValue("--title".to_string()))
    );
    assert!(Args::parse(fake_args(&["--title", "test"])).is_ok());
}

#[test]
fn can_parse_multiple_args_with_value() {
    assert_eq!(
        Args::parse(fake_args(&["--title", "test", "--artist"])),
        Err(InvalidArgsError::MissingValue("--artist".to_string()))
    );

    assert_eq!(
        Args::parse(fake_args(&["--title", "--artist", "test"])),
        Err(InvalidArgsError::MissingValue("--title".to_string()))
    );

    assert_eq!(
        Args::parse(fake_args(&["--title", "test", "--artist", "test"])),
        Ok(Args {
            title: "test".to_string(),
            artist: "test".to_string(),
            url: None,
            list: false,
            max_results: None,
            help: false
        })
    );

    assert_eq!(
        Args::parse(fake_args(&["--url", "--title", "test", "--artist", "test"])),
        Err(InvalidArgsError::MissingValue("--url".to_string()))
    );

    assert_eq!(
        Args::parse(fake_args(&[
            "--url", "test", "--title", "test", "--artist", "test"
        ])),
        Ok(Args {
            title: "test".to_string(),
            artist: "test".to_string(),
            url: Some("test".to_string()),
            list: false,
            max_results: None,
            help: false
        })
    );
}

#[test]
fn can_parse_arg_with_optional_value() {
    assert_eq!(
        Args::parse(fake_args(&[
            "--list", "--title", "test", "--artist", "test"
        ])),
        Ok(Args {
            title: "test".to_string(),
            artist: "test".to_string(),
            url: None,
            list: true,
            max_results: None,
            help: false
        })
    );

    assert_eq!(
        Args::parse(fake_args(&[
            "--list", "30", "--title", "test", "--artist", "test"
        ])),
        Ok(Args {
            title: "test".to_string(),
            artist: "test".to_string(),
            url: None,
            list: true,
            max_results: Some(30),
            help: false
        })
    );

    assert!(matches!(
        Args::parse(fake_args(&["--list", "notAnumber"])),
        Err(InvalidArgsError::InvalidValue(_))
    ));
}
