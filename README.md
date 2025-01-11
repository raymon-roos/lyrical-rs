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

- rust & cargo, v1.83
- [`reqwest`](https://crates.io/crates/reqwest) for sending HTTP requests
- [`serde_json`](https://crates.io/crates/serde_json) for de-serialising json
- [`scraper`](https://crates.io/crates/scraper) for parsing html

### How to use

1. Clone this repo to a location of your choice.
2. Run `cargo build --release`
3. Create a genius account.
4. Create an API client: <https://genius.com/api-clients>. Just fill in some random
   things, it doesn't matter for this script.
5. Generate an access token for your "client" and save it to
   `$XDG_CONFIG_HOME/lyrical/token`. Maybe make the file only readable and writeable by
   your own user.

```sh
# First argument is the artist's name, second argument is the song title.
# Both are required.
./target/release/lyrical 'Windmills' 'True Natural'
```

I use a bash script to get the currently playing song in cmus and to save the lyrics to
a file. For inspiration, see <https://github.com/raymon-roos/scripts>. That way I can use
[fd](https://github.com/sharkdp/fd) for searching files, rather than implementing my own
file searching algorithm. This whole thing could be a bash script, if you can find a good
way to parse HTML…

### Eventual goals

Eventually I might combine multiple sources for lyrics, support multiple music players,
add time cues (karaoke mode), and make the program interactive to allow for more advanced
features. We'll see…
