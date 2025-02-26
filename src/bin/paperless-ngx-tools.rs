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

    #[arg(short, long, help = "Do not make changes")]
    noop: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// list correspondants
    ListCorrespondents {
        /// Filter by Correspondant Name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// list documents, optionally by filter mechanism
    ListDocuments {
        /// filter by correspondent name
        #[arg(short, long)]
        correspondent: Option<String>,
    },
    /// list document IDs, optionally by filter mechanism
    ListDocumentIds {
        /// filter by correspondent name
        #[arg(short, long)]
        correspondent: Option<String>,
    },
    /// move documents from one correspondent to another
    MigrateCorrespondents {
        /// Correspondent ID to move documents from
        #[arg(short, long)]
        from: Vec<i32>,

        /// Correspondent ID to move documents to
        #[arg(short, long)]
        to: i32,
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
        .set_url(&cfg.url)
        .set_auth_token(&cfg.auth)
        .set_no_op(args.noop)
        .build()?;

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
                println!("{}: {} [{:?}]", doc.id, doc.title, doc.tags);
            }
        }
        Some(Commands::ListDocumentIds { correspondent }) => {
            let mut correspondent_obj = None;
            if let Some(name) = correspondent {
                debug!("Looking up correspondent by name: {}", name);
                correspondent_obj = Some(client.correspondent_for_name(name.clone()).await?);
                debug!("Got correspondent: {:?}", correspondent_obj)
            }
            let doc_ids = client.document_ids(correspondent_obj).await?;
            for id in doc_ids {
                println!("{id}");
            }
        }
        Some(Commands::MigrateCorrespondents { from, to }) => {
            let to_correspondent = client.correspondent_get(to).await?;

            for from_id in from {
                let from_correspondent = client.correspondent_get(from_id).await?;

                info!("Moving from {} to {}", from_correspondent, to_correspondent);
                let doc_ids = client.document_ids(Some(from_correspondent)).await?;

                client
                    .documents_bulk_set_correspondent(doc_ids, &to_correspondent)
                    .await?;
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
