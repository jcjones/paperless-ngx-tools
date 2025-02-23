use clap::{Parser, Subcommand};
use log::{debug, info};
use paperless_ngx_api::client::PaperlessNgxClientBuilder;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about = "Upload a document to Paperless-ngx")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long, help = "URL to use")]
    url: Option<String>,

    #[arg(long, help = "Auth token to use")]
    auth: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// list correspondants
    ListCorrespondents {
        /// filter by name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// list documents, optionally by filter mechanism
    ListDocuments {
        /// filter by correspondent name
        #[arg(short, long)]
        correspondent: Option<String>,
    },

    /// Stores the --auth and --url to the config file
    Store,
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    url: String,
    auth: String,
}

const APP_NAME: &str = "paperless-ngx-tools";

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
    if let Some(a) = args.auth {
        cfg.auth = a;
    }

    let client = PaperlessNgxClientBuilder::default()
        .set_url(cfg.url.clone())
        .set_auth_token(cfg.auth.clone())
        .build()?;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &args.command {
        Some(Commands::ListCorrespondents { name }) => {
            let corrs = client.correspondents(name.clone()).await?;
            for c in corrs {
                println!("{}: {}", c.id, c.name)
            }
        }
        Some(Commands::ListDocuments { correspondent }) => {
            let mut correspondent_obj = None;
            if let Some(name) = correspondent {
                debug!("Looking up correspondent by name: {}", name);
                correspondent_obj = Some(client.correspondent_for_name(name.clone()).await?);
                debug!("Got correspondent: {:?}", correspondent_obj)
            }
            let docs = client.documents(correspondent_obj).await?;
            for doc in docs {
                println!("{}: {} [{:?}]", doc.id, doc.title, doc.tags)
            }
        }
        Some(Commands::Store) => {
            confy::store(APP_NAME, None, &cfg)?;
            println!(
                "Stored configuration to {:?}",
                confy::get_configuration_file_path(APP_NAME, None)?
            );
        }
        None => {}
    }

    Ok(())
}
