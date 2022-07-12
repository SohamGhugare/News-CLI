use std::env;
use std::format;
use dotenv::dotenv;
use std::error::Error;
use serde::Deserialize;
use colour::{ dark_green, yellow };

fn main() -> Result<(), Box<dyn Error>>{

    let api_key = get_api_key();
    let url = format!("https://newsapi.org/v2/top-headlines?country=in&apiKey={}", api_key);   
    let articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(&url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn get_api_key() -> String {
    
    dotenv().expect(".env file not found.");
    let mut api_key = String::new();
    match env::var("API_KEY") {
        Ok(v) => api_key.push_str(&v),
        Err(_e) => panic!("API_KEY environment variable is not set.")

    };
    api_key
}

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        dark_green!("> {}\n", a.title);
        yellow!("- {}\n\n", a.url);
    }
}
