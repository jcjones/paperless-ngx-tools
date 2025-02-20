use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use paperless_ngx_api::client::{PaperlessNgxClient, PaperlessNgxClientBuilder};
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

async fn print_task_status(
    client: &PaperlessNgxClient,
    uuid: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {spinner} {msg}").unwrap());

    loop {
        let status = client.task_status(uuid).await?;
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

    println!(
        "Loading configuration from {:?}",
        confy::get_configuration_file_path("paperless-ngx-upload", None)?
    );

    let mut cfg: Config = confy::load("paperless-ngx-upload", None).unwrap();
    let args = Args::parse();

    if let Some(u) = args.url {
        cfg.url = u;
    }

    let client = PaperlessNgxClientBuilder::default()
        .set_url(cfg.url)
        .set_auth_token(cfg.auth)
        .build()?;

    for filepath in args.files {
        let task_uuid = client.upload(&filepath).await?;
        print_task_status(&client, &task_uuid).await?;
    }

    Ok(())
}
