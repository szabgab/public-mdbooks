use std::path::Path;

use clap::Parser;
use serde::{Deserialize, Serialize};
use toml::{Table, Value, map::Map};

use git_digger::Repository;

const PREPROCESSORS: [&str; 3] = ["admonish", "alerts", "embedify"];

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(
        long,
        default_value_t = 0,
        help = "Limit the number of repos we process."
    )]
    limit: u32,

    #[arg(long, help = "Clone the repositories")]
    clone: bool,

    #[arg(long, help = "Create the report")]
    report: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
#[serde(deny_unknown_fields)]
struct MDBook {
    title: String,

    #[serde(deserialize_with = "from_url")]
    repo: Repository,
    folder: Option<String>,

    site: Option<String>,
    description: Option<String>,
    comment: Option<String>,

    book: Option<BookToml>,
    everything: Option<Map<String, Value>>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BookToml {
    book: Book,
    rust: Option<Rust>,
    build: Option<Build>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Book {
    title: String,
    src: Option<String>,
    language: Option<String>,

    #[serde(alias = "text-direction")]
    text_direction: Option<String>,
    multilingual: Option<bool>,
    authors: Option<Vec<String>>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Rust {
    edition: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Build {
    #[serde(alias = "build-dir")]
    build_dir: Option<String>,
    #[serde(alias = "create-missing")]
    create_missing: Option<bool>,
    #[serde(alias = "extra-watch-dirs")]
    extra_watch_dirs: Option<Vec<String>>,
    #[serde(alias = "use-default-preprocessors")]
    use_default_preprocessors: Option<bool>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Cli::parse();

    let repos_dir = std::fs::canonicalize("repos")?;

    let mut mdbooks = read_the_mdbooks_file()?;

    if args.clone {
        let mut count = 0;
        for mdbook in &mut mdbooks {
            log::info!("book: {:?}", mdbook);
            match mdbook.repo.update_repository(&repos_dir, false) {
                Ok(_) => {}
                Err(err) => {
                    log::error!("Error updating repo: {:?}", err);
                    mdbook.error = Some(format!("{:?}", err));
                    continue;
                }
            }
            count += 1;
            if args.limit > 0 && count >= args.limit {
                break;
            }
        }
    }

    if args.report {
        log::info!("Start processing repos");
        let mut count = 0;
        let src_path = Path::new("report/src");
        if !src_path.exists() {
            std::fs::create_dir("report/src")?;
        }

        std::fs::copy("report/SUMMARY.md", "report/src/SUMMARY.md")?;
        for mdbook in &mut mdbooks {
            log::info!("book: {:?}", mdbook);
            count += 1;
            if args.limit > 0 && count >= args.limit {
                break;
            }
            let book_toml_file = if let Some(folder) = mdbook.folder.clone() {
                mdbook.repo.path(&repos_dir).join(folder).join("book.toml")
            } else {
                mdbook.repo.path(&repos_dir).join("book.toml")
            };

            log::info!("book.toml: {:?}", book_toml_file);
            if !book_toml_file.exists() {
                log::error!("book.toml does not exist: {:?}", book_toml_file);
                mdbook.error = Some("book.toml does not exist".to_string());
                continue;
            }

            let content = std::fs::read_to_string(&book_toml_file)?;

            let everything = match toml::from_str::<Table>(&content) {
                Ok(data) => data,
                Err(err) => {
                    log::error!("Error parsing toml {book_toml_file:?}: {:?}", err);
                    mdbook.error = Some(err.to_string());
                    continue;
                }
            };

            mdbook.everything = Some(everything);

            let data = match toml::from_str::<BookToml>(&content) {
                Ok(data) => data,
                Err(err) => {
                    log::error!("Error parsing toml {book_toml_file:?}: {:?}", err);
                    mdbook.error = Some(err.to_string());
                    continue;
                }
            };
            mdbook.book = Some(data);
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

        index_page(&mdbooks);
        book_toml_page();
        errors_page(&mdbooks);
        src_page(&mdbooks);
        language_page(&mdbooks);
        text_direction_page(&mdbooks);
        multilingual_page(&mdbooks);
        rust_page(&mdbooks);
        build_page(&mdbooks);
        output_page(&mdbooks);
        preprocessor_page(&mdbooks);
        for name in PREPROCESSORS {
            preprocessor_details_page(&mdbooks, name);
        }
        extra_page(&mdbooks);
    }

    let count_errors = mdbooks
        .iter()
        .filter(|mdbook| mdbook.error.is_some())
        .count();

    if count_errors > 0 {
        log::error!("There were {count_errors} errors");
        //std::process::exit(1);
    }
    Ok(())
}

fn index_page(mdbooks: &Vec<MDBook>) {
    let now = chrono::Utc::now();
    let mut md = String::from("# Public mdBooks\n\n");
    md += "This is a list of mdBooks for which the source is also available available.\n";
    md += "The list is generated from the `mdbooks.yaml` file in our [repository](https://github.com/szabgab/public-mdbooks).\n\n";
    md += "If you would like to add a book to this list, or add a description please submit a PR to the `mdbooks.yaml` file.\n\n";
    md += "Check out the [mdBook User manual](https://rust-lang.github.io/mdBook/) for more information.\n\n";
    md += format!("Total number of books: {}\n\n", mdbooks.len()).as_str();
    md += format!("Generated at: {}\n\n", now.format("%Y-%m-%d %H:%M:%S")).as_str();
    md += "| Title | Repo | Description | Comment |\n";
    md += "|-------|------|-------------|---------|\n";
    for mdbook in mdbooks {
        md += format!(
            "| [{}]({}) | [repo]({}) | {} | {} |\n",
            mdbook.title,
            mdbook.site.clone().unwrap_or("".to_string()),
            mdbook.repo.url(),
            mdbook.description.clone().unwrap_or("".to_string()),
            mdbook.comment.clone().unwrap_or("".to_string()),
        )
        .as_str();
    }
    std::fs::write("report/src/index.md", md).unwrap();
}

fn book_toml_page() {
    let mut md = String::from("# book.toml\n\n");
    md += "The book.toml file is the main [configuration file](https://rust-lang.github.io/mdBook/format/configuration/) of every mdbook.\n";
    md += "In this chapter we analyzet the content of the book.toml files in the listed mdbooks.\n\n";
    std::fs::write("report/src/book-toml.md", md).unwrap();
}

fn errors_page(mdbooks: &Vec<MDBook>) {
    let mut md = String::from("# Errors in the mdbooks\n\n");

    let count_errors = mdbooks
        .iter()
        .filter(|mdbook| mdbook.error.is_some())
        .count();

    md += "The errors are as reported by our parser. They might or might not be real problems.\n";
    md += "If you think the error is incorrect, please open an issue on [our repository](https://github.com/szabgab/public-mdbooks).\n";
    md += "If you think the problem is with the specific mdbook, please open an issue on the repository of that mdbook.\n";
    md += "We still need to clean up the error messages.\n\n";
    md += format!(
        "Total number of errors {} (in {} books)\n\n",
        count_errors,
        mdbooks.len()
    )
    .as_str();
    md += "---\n\n";

    for mdbook in mdbooks {
        if mdbook.error.is_none() {
            continue;
        }
        md += format!(
            "* [{}]({})\n* [repo]({})\n\n{}\n\n---\n\n",
            mdbook.title,
            mdbook.site.clone().unwrap_or("".to_string()),
            mdbook.repo.url(),
            mdbook.error.clone().unwrap_or("".to_string())
        )
        .as_str();
    }
    std::fs::write("report/src/errors.md", md).unwrap();
}

fn src_page(mdbooks: &Vec<MDBook>) {
    let mut md = String::from("# The book.src field\n\n");

    md += "| Title | Repo | src |\n";
    md += "|-------|------|-------------|\n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | [repo]({}) | {} | \n",
            mdbook.title,
            mdbook.site.clone().unwrap_or("".to_string()),
            mdbook.repo.url(),
            if bk.book.src.is_none() {
                "-".to_string()
            } else {
                bk.book.src.clone().unwrap()
            }
        )
        .as_str();
    }

    std::fs::write("report/src/src.md", md).unwrap();
}

fn language_page(mdbooks: &Vec<MDBook>) {
    let mut md = String::from("# The book.language field\n\n");

    md += "| Title | Repo | language |\n";
    md += "|-------|------|-------------|\n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | [repo]({}) | {} | \n",
            mdbook.title,
            mdbook.site.clone().unwrap_or("".to_string()),
            mdbook.repo.url(),
            if bk.book.language.is_none() {
                "-".to_string()
            } else {
                bk.book.language.clone().unwrap()
            }
        )
        .as_str();
    }

    std::fs::write("report/src/language.md", md).unwrap();
}

fn text_direction_page(mdbooks: &Vec<MDBook>) {
    let mut md = String::from("# The book.text-direction field\n\n");

    md += "| Title | Repo | text-direction |\n";
    md += "|-------|------|-------------|\n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | [repo]({}) | {} | \n",
            mdbook.title,
            mdbook.site.clone().unwrap_or("".to_string()),
            mdbook.repo.url(),
            if bk.book.text_direction.is_none() {
                "-".to_string()
            } else {
                bk.book.text_direction.clone().unwrap()
            }
        )
        .as_str();
    }

    std::fs::write("report/src/text-direction.md", md).unwrap();
}

fn multilingual_page(mdbooks: &Vec<MDBook>) {
    let mut md = String::from("# The book.multilingual field\n\n");

    md += "| Title | Repo | multilingual |\n";
    md += "|-------|------|-------------|\n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | [repo]({}) | {} | \n",
            mdbook.title,
            mdbook.site.clone().unwrap_or("".to_string()),
            mdbook.repo.url(),
            if bk.book.multilingual.is_none() {
                "-".to_string()
            } else {
                match bk.book.multilingual {
                    None => String::from("?"),
                    Some(multilingual) => multilingual.to_string(),
                }
            }
        )
        .as_str();
    }

    std::fs::write("report/src/multilingual.md", md).unwrap();
}

fn rust_page(mdbooks: &Vec<MDBook>) {
    let mut md = String::from("# The rust.edition field\n\n");

    md += "| Title | Repo | editon |\n";
    md += "|-------|------|-------------|\n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | [repo]({}) | {} | \n",
            mdbook.title,
            mdbook.site.clone().unwrap_or("".to_string()),
            mdbook.repo.url(),
            match &bk.rust {
                None => String::new(),
                Some(rust) => rust.edition.clone(),
            }
        )
        .as_str();
    }

    std::fs::write("report/src/rust.md", md).unwrap();
}

fn build_page(mdbooks: &Vec<MDBook>) {
    let mut md = String::from("# The build table\n\n");

    md += "| Title | Repo | build-dir | create-missing | extra-watch-dirs | use-default-preprocessors |\n";
    md += "|-------|------|-------------|--------|------|------| \n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | [repo]({}) | {} | {} | {} | {} |\n",
            mdbook.title,
            mdbook.site.clone().unwrap_or("".to_string()),
            mdbook.repo.url(),
            match &bk.build {
                None => String::new(),
                Some(build) => match build.build_dir.clone() {
                    None => String::new(),
                    Some(build_dir) => build_dir.clone(),
                },
            },
            match &bk.build {
                None => String::new(),
                Some(build) => match build.create_missing {
                    None => String::new(),
                    Some(create_missing) => create_missing.to_string(),
                },
            },
            match &bk.build {
                None => String::new(),
                Some(build) => match build.extra_watch_dirs.clone() {
                    None => String::new(),
                    Some(extra_wath_dirs) => extra_wath_dirs.join(","),
                },
            },
            match &bk.build {
                None => String::new(),
                Some(build) => match build.use_default_preprocessors {
                    None => String::new(),
                    Some(use_default_preprocessors) => use_default_preprocessors.to_string(),
                },
            }
        )
        .as_str();
    }

    std::fs::write("report/src/build.md", md).unwrap();
}

fn output_page(mdbooks: &Vec<MDBook>) {
    let mut md = String::from("# output\n\n");

    md += "| Title | Repo | output field | \n";
    md += "|-------|------|-------------| \n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let table = mdbook.everything.as_ref().unwrap();
        let fields = match table.get("output") {
            None => String::new(),
            Some(output) => {
                let mut fields = String::new();
                match output {
                    Value::Table(t) => {
                        t.iter().for_each(|(k, _v)| {
                            fields += k;
                            fields += " ";
                        });
                    }
                    _ => {
                        fields += "unknown";
                    }
                }
                fields
            }
        };

        md += format!(
            "| [{}]({}) | [repo]({}) | {} | \n",
            mdbook.title,
            mdbook.site.clone().unwrap_or("".to_string()),
            mdbook.repo.url(),
            fields,
        )
        .as_str();
    }

    std::fs::write("report/src/output.md", md).unwrap();
}

fn preprocessor_page(mdbooks: &Vec<MDBook>) {
    let mut md = String::from("# preprocessor\n\n");

    md += "| Title | Repo | preprocessor field | \n";
    md += "|-------|------|-------------| \n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let table = mdbook.everything.as_ref().unwrap();
        let fields = match table.get("preprocessor") {
            None => String::new(),
            Some(preprocessor) => {
                let mut fields = String::new();
                match preprocessor {
                    Value::Table(t) => {
                        t.iter().for_each(|(k, _v)| {
                            if PREPROCESSORS.contains(&k.as_str()) {
                                fields += format!("[{k}](preprocessor-{k}.md)").as_str();
                            } else {
                                fields += k;
                            }
                            fields += " ";
                        });
                    }
                    _ => {
                        fields += "unknown";
                    }
                }
                fields
            }
        };

        md += format!(
            "| [{}]({}) | [repo]({}) | {} | \n",
            mdbook.title,
            mdbook.site.clone().unwrap_or("".to_string()),
            mdbook.repo.url(),
            fields,
        )
        .as_str();
    }

    std::fs::write("report/src/preprocessor.md", md).unwrap();
}

fn preprocessor_details_page(mdbooks: &Vec<MDBook>, preprocessor: &str) {
    let mut md = format!("# preprocessor {preprocessor}\n\n");

    md += "| Title | Repo | preprocessor field | \n";
    md += "|-------|------|-------------| \n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let table = mdbook.everything.as_ref().unwrap();
        match table.get("preprocessor") {
            None => continue,
            Some(preprocessor_table) => match preprocessor_table.get(preprocessor) {
                None => continue,
                Some(data) => {
                    let mut fields = String::new();
                    match data {
                        Value::Table(t) => {
                            t.iter().for_each(|(k, _v)| {
                                fields += k;
                                fields += " ";
                            });
                        }
                        _ => {
                            fields += "unknown";
                        }
                    }
                    md += format!(
                        "| [{}]({}) | [repo]({}) | {} | \n",
                        mdbook.title,
                        mdbook.site.clone().unwrap_or("".to_string()),
                        mdbook.repo.url(),
                        fields,
                    )
                    .as_str();
                }
            },
        };
    }
    let path = format!("report/src/preprocessor-{preprocessor}.md");
    std::fs::write(path, md).unwrap();
}

fn extra_page(mdbooks: &Vec<MDBook>) {
    let mut md = String::from("# Extra fields in book.toml\n\n");
    md += "We have not dealt with these fields yet. If there are any fields here that should be probably reported as an error.\n\n";

    let known = [
        String::from("book"),
        String::from("rust"),
        String::from("build"),
        String::from("output"),
        String::from("preprocessor"),
    ];

    md += "| Title | Repo | extra fields | \n";
    md += "|-------|------|-------------| \n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let table = mdbook.everything.as_ref().unwrap();
        let mut fields = String::new();
        table
            .iter()
            .filter(|(k, _v)| !known.contains(*k))
            .for_each(|(k, _v)| {
                fields += k;
                fields += " ";
            });
        if fields.is_empty() {
            fields = String::from("-");
        }

        md += format!(
            "| [{}]({}) | [repo]({}) | {} | \n",
            mdbook.title,
            mdbook.site.clone().unwrap_or("".to_string()),
            mdbook.repo.url(),
            fields,
        )
        .as_str();
    }

    std::fs::write("report/src/extra.md", md).unwrap();
}

fn read_the_mdbooks_file() -> Result<Vec<MDBook>, Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("mdbooks.yaml")?;
    let mut books: Vec<MDBook> = serde_yaml::from_str(&file)?;
    books.sort_by(|a, b| a.title.cmp(&b.title));
    Ok(books)
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
