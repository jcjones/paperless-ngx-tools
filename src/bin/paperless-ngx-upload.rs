use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use paperless_ngx_api::client::PaperlessNgxClientBuilder;
use paperless_ngx_api::task::Task;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about = "Upload a document to Paperless-ngx")]
struct Args {
    /// Name of the person to greet
    #[arg(help = "Receipt to upload")]
    files: Vec<String>,

    #[arg(long, help = "URL to use")]
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    url: String,
    auth: String,
}

const APP_NAME: &str = "paperless-ngx-tools";

async fn print_task_status<'a>(task: Task<'a>) -> Result<(), Box<dyn std::error::Error>> {
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {spinner} {msg}").unwrap());

    loop {
        let status = task.status().await?;
        bar.set_message(format!(
            "Filename: {0} Status: {1}",
            status.task_file_name, status.status
        ));

        if let Some(result) = status.result {
            bar.finish();

            println!("Result: {}", result);
            if let Some(doc_id) = status.related_document {
                println!("Related DocID: {}", doc_id);
            }

            return Ok(());
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!(
        "Loading configuration from {:?}",
        confy::get_configuration_file_path(APP_NAME, None)?
    );

    let mut cfg: Config = confy::load(APP_NAME, None).unwrap();
    let args = Args::parse();

    if let Some(u) = args.url {
        cfg.url = u;
    }

    let client = PaperlessNgxClientBuilder::default()
        .set_url(&cfg.url)
        .set_auth_token(&cfg.auth)
        .build()?;

    for filepath in args.files {
        let task = client.upload(&filepath).await?;
        print_task_status(task).await?;
    }

    Ok(())
}
