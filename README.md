### Lyrical-rs ♪

A little Rust program that fetches plaintext lyrics from [genius.com](https://genius.com/)
and dumps them to the terminal, potential successor of
https://github.com/raymon-roos/lyrical-php.

Back when I used to use foobar2000, (in a deep, dark past where I used Windows...),
I loved having a lyrics panel plugin. Lyrics are a large part of what I enjoy in songs.

My music player of choice these days is [cmus](https://cmus.github.io/). I haven't found
a convenient lyrics program that runs in the terminal yet, so I made my own. Though as yet
its features are limited.

### Dependencies

- Rust & cargo, v1.83
- OpenSSL with headers are required on linux[^ssl] (they're already included in the Nix shell).
- Crates used: [`reqwest`](https://crates.io/crates/reqwest) for sending HTTP requests,
  [`serde_json`](https://crates.io/crates/serde_json) for de-serialising JSON,
  [`scraper`](https://crates.io/crates/scraper) for parsing HTML.

[^ssl]: https://crates.io/crates/reqwest

### Instructions

#### Create an API client token

1. Create a genius account.
2. Create an API client: <https://genius.com/api-clients>. Just fill in some random
   things, it doesn't matter for this program.
3. Generate an access token for your "client" and save it to
   `$XDG_CONFIG_HOME/lyrical/token`. Maybe make the file only readable and writeable by
   your own user.

#### Run or install on Nix:

**Try out without installing:**

```sh
nix run 'github:raymon-roos/lyrical-rs' --artist 'windmills' --title 'true natural'
# See all available options:
nix run 'github:raymon-roos/lyrical-rs' --help
```

**Install into home-manager or system configuration using flakes:**

```nix
# flake.nix:
inputs = {
  lyrical.url = "github:raymon-roos/lyrical-rs";
  lyrical.inputs.nixpkgs.follows = "nixpkgs";
}

# home-manager configuration:
home.packages = [
    inputs.lyrical.packages.${pkgs.system}.default
];

# Alternatively, NixOS system configuration:
environment = {
  systemPackages = [
    inputs.lyrical.packages.${pkgs.system}.default
  ];
```

### Installation on Linux:

1. Clone this repo to a location of your choice.
2. Make sure you have the OpenSSL headers installed, as per [^ssl]
3. Run `cargo install --path .` (make sure that `$CARGO_HOME/bin` is in your `$PATH`)

### Usage

`lyrical-rs --help`

```
Available options:

--help                       Print this help text and exit.
--title  <title>  (required) Title of the song to search lyrics for.
--artist <artist> (optional) Restrict search results by the name of the
                             song's artist.
--url    <url>    (optional) Rather than searching, fetch lyrics from this
                             genius.com url directly. If present, `--artist`,
                             `--title` and `--list` will be ignored.
--list   [count]  (optional) Print list of [count] (default 20) genius.com search
                             results, rather than song lyrics. Useful for
                             further filtering, manually or through external tools.
```

**Example**

```sh
lyrical-rs --artist 'Sleep Token' --title 'Euclid' --list
lyrical-rs --url 'https://genius.com/The-arcadian-wild-wander-wonder-lyrics'
```

I use a bash script to get the currently playing song in cmus and to save the lyrics to
a file. For inspiration, see
<https://github.com/raymon-roos/scripts/blob/main/lyrics_in_terminal.sh>. That way I can
use [fd](https://github.com/sharkdp/fd) for searching files, rather than implementing my
own file searching algorithm. This whole thing could be a bash script, if you can find
a good way to parse HTML…

### Eventual goals

Eventually I might combine multiple sources for lyrics, support multiple music players,
add time cues (karaoke mode), and make the program interactive to allow for more advanced
features. We'll see…
