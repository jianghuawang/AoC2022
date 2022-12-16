use anyhow::Result;
use dotenv::dotenv;
use reqwest::{cookie::Jar, Url};
use std::{
    env::{args, var},
    process::exit,
    sync::Arc,
};

const NAME: &str = "Jianghua Wang";

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<_> = args().collect();
    if args.len() != 3 {
        println!("Usage: ./get_input year day");
        exit(0)
    }
    let (year, day) = (args[1].parse::<i32>()?, args[2].parse::<i32>()?);

    dotenv().ok();
    let session = var("session")?;
    let cookie = format!("session={}", session);
    let url = "https://adventofcode.com/".parse::<Url>()?;

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    let client = reqwest::Client::builder()
        .user_agent(NAME)
        .cookie_provider(Arc::new(jar))
        .build()?;
    let req_url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let text = client.get(req_url).send().await?.text().await?;

    let file_name = format!("data/day{day}.txt");
    std::fs::write(file_name, text)?;
    Ok(())
}
