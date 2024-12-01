use std::{fs::OpenOptions, io::Write};

use aoc_utils::{make_real_path, make_sample_path, MyResult};
use clap::Parser;
use reqwest::blocking::Client;
use scraper::{Html, Selector};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    day: i32,
    #[arg(long)]
    url_prefix: String,
    #[arg(long)]
    user_session: String,
}

impl Cli {
    fn get(&self, suffix: &str) -> MyResult<String> {
        let url = format!("{}day/{}{}", self.url_prefix, self.day, suffix);
        println!("Getting from {}", url);
        let response = Client::new()
            .get(url)
            .header("cookie", format!("session={};", self.user_session))
            .send()?;
        Ok(response.text()?)
    }
    fn download_samples(&self) -> MyResult<()> {
        let document = Html::parse_document(&self.get("")?);
        for (pre_index, pre) in document.select(&Selector::parse("pre")?).enumerate() {
            let pre_index = pre_index as i32;
            let file_path = make_sample_path(self.day, pre_index);
            println!("Writing to {}", file_path);
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_path)?;
            let text = pre.text().collect::<String>();

            file.write_all(text.as_bytes())?;
        }
        Ok(())
    }
    fn download_real(&self) -> MyResult<()> {
        let real_input = self.get("/input")?;
        let file_path = make_real_path(self.day);
        println!("Writing to {}", file_path);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        file.write_all(real_input.as_bytes())?;
        Ok(())
    }
}

fn main() -> MyResult<()> {
    let args = argfile::expand_args_from(
        std::env::args_os(),
        argfile::parse_fromfile,
        argfile::PREFIX,
    )?;
    let cli = Cli::parse_from(args);
    cli.download_samples()?;
    cli.download_real()?;
    Ok(())
}
