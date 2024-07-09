mod openai;

use anyhow::Result;
use clap::Parser;
use openai::OpenAiClient;
use std::{fs, io::stdin};
use tokio_stream::StreamExt;

#[derive(Parser)]
#[command(version, about, author, help_template = format!("
{{usage}}
  
{{all-args}}

{}Version:{} {{version}}
{}Authors:{} {{author}}

{{about}}

{}Report Bugs:{} {}{}/issues{}

Thanks for using! 😊
", 
crossterm::style::Attribute::Bold, crossterm::style::Attribute::Reset,
crossterm::style::Attribute::Bold, crossterm::style::Attribute::Reset,
crossterm::style::Attribute::Bold, crossterm::style::Attribute::Reset,
crossterm::style::Attribute::Underlined,
get_repository_url("Cargo.toml").expect("Failed to get repository URL"),
crossterm::style::Attribute::Reset))]
struct Opt {
    #[clap(short, long, help = "OpenAI API key")]
    api_key: String,
    #[clap(
        short,
        long,
        help = "OpenAI API endpoint",
        default_value = "https://api.openai.com/v1/chat/completions"
    )]
    endpoint: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();
    let client = OpenAiClient::new(opt.api_key, opt.endpoint);
    let mut chat = String::new();
    let stdin = stdin();
    println!(
        "{}Me: {}",
        crossterm::style::Attribute::Bold,
        crossterm::style::Attribute::Reset
    );
    while let Ok(size) = stdin.read_line(&mut chat) {
        if size == 0 {
            break;
        }
        read_stream(client.clone(), chat.clone()).await?;
        println!(
            "{}Me: {}",
            crossterm::style::Attribute::Bold,
            crossterm::style::Attribute::Reset
        );
    }
    Ok(())
}

async fn read_stream(client: OpenAiClient, prompt: String) -> Result<()> {
    let res = client.create_chat(prompt).await?;
    let mut stream = res.bytes_stream();
    println!(
        "{}AI: {}",
        crossterm::style::Attribute::Bold,
        crossterm::style::Attribute::Reset
    );
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let json = serde_json::from_slice::<openai::ChatCompletionChunk>(&chunk)?;
        for choice in json.choices {
            if choice.finish_reason.is_some() {
                return Ok(());
            }
            print!("{}", choice.delta.content);
        }
    }
    Ok(())
}

fn get_repository_url(file_path: &str) -> Option<String> {
    let cargo_toml = fs::read_to_string(file_path).ok()?;
    let value: toml::Value = toml::from_str(&cargo_toml).ok()?;

    value
        .get("package")?
        .get("repository")?
        .as_str()
        .map(|s| s.to_string())
}
