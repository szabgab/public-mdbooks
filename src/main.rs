use clap::Parser;
use serde::{Deserialize, Serialize};

use git_digger::Repository;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(
        long,
        default_value_t = 0,
        help = "Limit the number of repos we process."
    )]
    limit: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct BookMeta {
    title: String,

    #[serde(deserialize_with = "from_url")]
    repo: Repository,
    site: Option<String>,
    comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Book {
    title: String,
    src: String,
    language: Option<String>,

    #[serde(alias = "text-direction")]
    text_direction: Option<String>,
    multilingual: Option<bool>,
    authors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BookToml {
    book: Book,
}

fn main() {
    env_logger::init();
    let args = Cli::parse();

    let repos_dir = std::fs::canonicalize("repos").unwrap();

    let books = read_the_mdbooks_file();

    let mut count = 0;
    for book in &books {
        log::info!("book: {:?}", book);
        book.repo.update_repository(&repos_dir, false).unwrap();
        count += 1;
        if args.limit > 0 && count >= args.limit {
            break;
        }
    }

    log::info!("Start processing repos");
    let mut count = 0;
    for book in books {
        log::info!("book: {:?}", book);
        count += 1;
        if args.limit > 0 && count >= args.limit {
            break;
        }
        let book_toml = book.repo.path(&repos_dir).join("book.toml");
        if !book_toml.exists() {
            log::error!("book.toml does not exist: {:?}", book_toml);
            continue;
        }

        let content = std::fs::read_to_string(&book_toml).unwrap();

        let data = match toml::from_str::<BookToml>(&content) {
            Ok(data) => data,
            Err(err) => {
                log::error!("Error parsing toml {book_toml:?}: {:?}", err);
                continue;
            }
        };
        println!("{:?}", data);
    }

    // Go over all the cloned repos and check if they are still in the mdbooks.yaml file
    //list content of a directory
    //let path = PathBuf::from(repos_dir);
    //let entries = std::fs::read_dir(path).unwrap();
    //for entry in entries {
    //    let entry = entry.unwrap();
    //    let path = entry.path();
    //    println!("{:?}", path);

    //    std::process::exit(0);
    //}
}

fn read_the_mdbooks_file() -> Vec<BookMeta> {
    let file = std::fs::read_to_string("mdbooks.yaml").unwrap();
    let books: Vec<BookMeta> = serde_yaml::from_str(&file).unwrap();
    books
}

use serde::de;

fn from_url<'de, D>(deserializer: D) -> Result<Repository, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let r = Repository::from_url(&s).unwrap();
    Ok(r)
}
