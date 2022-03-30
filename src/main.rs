use ansi_term::Color::*;
use reqwest::{
    self,
    header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use spotify_search::*;
use std::{
    env,
    io::{self, Read},
};

// TOKIO let's us use "async" on our main function
#[tokio::main]
async fn main() {
    // Get query value from Launch Arguments (CLI)
    // let args: Vec<String> = env::args().collect();
    // let search_query = &args[1];

    // Read user input from console application
    let mut search_query = String::new();
    println!("Artist to look for?");
    io::stdin()
        .read_line(&mut search_query)
        .expect("Error with the name given");

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query.trim()
    );

    let api_key = "Bearer TOKEN_KEY";
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, api_key.parse().unwrap()); // Authorization token - API token
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap()); // Specify what we want to retrieve
    headers.insert(ACCEPT, "application/json".parse().unwrap()); // Specify what our client supports

    let response = client
        .get(url)
        .headers(headers)
        // Confirms that the request was sent
        .send()
        .await
        // Retrieve the response body
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            // On success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                Ok(parsed) => {
                    let parsed = parsed.tracks.items.iter().collect();
                    print_result(parsed)
                }
                Err(_) => println!("No result from the query"),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Invalid token");
        }
        other => {
            panic!("Unexpected error: {:?}", other);
        }
    };
}

fn print_result(tracks: Vec<&Track>) {
    // Enables ansi_term to work on Win10's terminal
    let _enabled = ansi_term::enable_ansi_support();
    println!(
        "{}",
        RGB(230, 230, 0).paint("\nResults\n--------------------------------")
    );
    for track in tracks {
        println!("Song:   {}", RGB(0, 255, 85).paint(&track.name));
        println!("Album:  {}", RGB(0, 153, 153).paint(&track.album.name));
        println!(
            "Artist: {}",
            RGB(153, 204, 255).paint(
                track
                    .album
                    .artists
                    .iter()
                    .map(|artist| artist.name.to_string())
                    .collect::<String>()
            )
        );
        println!(
            "Link:   {}",
            RGB(255, 153, 0).paint(&track.external_urls.spotify)
        );
        println!(
            "{}",
            RGB(230, 230, 0).paint("--------------------------------")
        );
    }
}
