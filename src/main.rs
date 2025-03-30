use std::{collections::HashSet, path::Path};

use clap::Parser;
use serde::{Deserialize, Serialize};
use toml::{Table, Value, map::Map};

use git_digger::Repository;

const REPO: &str = "https://github.com/szabgab/public-mdbooks/";
struct Language {
    code: &'static str,
    name: &'static str,
}
struct Preprocessor {
    name: &'static str,
    cratesio: &'static str,
    description: &'static str,
}
const LANGUAGES: [Language; 12] = [
    Language {
        code: "en",
        name: "English",
    },
    Language {
        code: "de",
        name: "German",
    },
    Language {
        code: "fr",
        name: "French",
    },
    Language {
        code: "es",
        name: "Spanish",
    },
    Language {
        code: "ja",
        name: "Japanese",
    },
    Language {
        code: "he",
        name: "Hebrew",
    },
    Language {
        code: "zh",
        name: "Chinese",
    },
    Language {
        code: "vi",
        name: "Vietnamese",
    },
    Language {
        code: "pt",
        name: "Portuguese",
    },
    Language {
        code: "kr",
        name: "Korean",
    },
    Language {
        code: "ca",
        name: "Catalan",
    },
    Language {
        code: "sv",
        name: "Swedish",
    },
];

const PREPROCESSORS: [Preprocessor; 16] = [
    Preprocessor {
        name: "admonish",
        cratesio: "https://crates.io/crates/mdbook-admonish",
        description: "A preprocessor for mdbook to add Material Design admonishments.",
    },
    Preprocessor {
        name: "alerts",
        cratesio: "https://crates.io/crates/mdbook-alerts",
        description: "mdBook preprocessor to add GitHub Flavored Markdown's Alerts to your book.",
    },
    Preprocessor {
        name: "aquascope",
        cratesio: "https://crates.io/crates/mdbook-aquascope",
        description: "Interactive Aquascope editor for your mdBook",
    },
    Preprocessor {
        name: "embedify",
        cratesio: "https://crates.io/crates/mdbook-embedify",
        description: "A rust based mdbook preprocessor plugin that allows you to embed apps to your book, like youtube, codepen and some other apps.",
    },
    Preprocessor {
        name: "footnote",
        cratesio: "https://crates.io/crates/mdbook-footnote",
        description: "mdbook preprocessor for footnotes.",
    },
    Preprocessor {
        name: "hints",
        cratesio: "https://crates.io/crates/mdbook-hints",
        description: "mdBook preprocessor to add hover hints to your book.",
    },
    Preprocessor {
        name: "katex",
        cratesio: "https://crates.io/crates/mdbook-katex",
        description: "mdBook preprocessor rendering LaTeX equations to HTML.",
    },
    Preprocessor {
        name: "mathpunc",
        cratesio: "https://crates.io/crates/mdbook-mathpunc",
        description: "An mdbook preprocessor that prevents line breaks between inline math blocks and punctuation marks when using katex.",
    },
    Preprocessor {
        name: "mermaid",
        cratesio: "https://crates.io/crates/mdbook-mermaid",
        description: "mdbook preprocessor to add mermaid support.",
    },
    Preprocessor {
        name: "numeq",
        cratesio: "https://crates.io/crates/mdbook-numeq",
        description: "An mdbook preprocessor for automatically numbering centered equations.",
    },
    Preprocessor {
        name: "numthm",
        cratesio: "https://crates.io/crates/mdbook-numthm",
        description: "An mdbook preprocessor for automatically numbering theorems, lemmas, etc.",
    },
    Preprocessor {
        name: "pikchr",
        cratesio: "https://crates.io/crates/mdbook-pikchr",
        description: "A mdbook preprocessor to render pikchr code blocks as images in your book.",
    },
    Preprocessor {
        name: "quiz",
        cratesio: "https://crates.io/crates/mdbook-quiz",
        description: "Interactive quizzes for your mdBook.",
    },
    Preprocessor {
        name: "repl",
        cratesio: "https://crates.io/crates/mdbook-repl",
        description: "A rust based mdbook preprocessor that allows you to execute code in your mdbook without any server. Python, Typescript, Javascript etc.",
    },
    Preprocessor {
        name: "template",
        cratesio: "https://crates.io/crates/mdbook-template",
        description: "A mdbook preprocessor that allows the re-usability of template files with dynamic arguments. (deprecated)",
    },
    Preprocessor {
        name: "toc",
        cratesio: "https://crates.io/crates/mdbook-toc",
        description: "mdbook preprocessor to add Table of Contents.",
    },
];

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

    #[serde(default = "default_empty_string")]
    raw_book_toml: String,
    book: Option<BookToml>,
    everything: Option<Map<String, Value>>,
    error: Option<String>,
}

impl MDBook {
    fn relative(&self) -> String {
        let relative = self.repo.path(Path::new(""));
        format!("./{}.md", relative.as_os_str().to_str().unwrap())
    }
}

fn default_empty_string() -> String {
    String::new()
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
        clone_repositories(&args, &repos_dir, &mut mdbooks);
    }

    if args.report {
        log::info!("Start processing repos");
        let mut count = 0;
        let src_path = Path::new("report/src");
        if !src_path.exists() {
            std::fs::create_dir("report/src")?;
        }

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
            mdbook.raw_book_toml = content.clone();

            let everything = match toml::from_str::<Table>(&content) {
                Ok(data) => data,
                Err(err) => {
                    log::error!("Error parsing toml {book_toml_file:?}: {:?}", err);
                    mdbook.error = Some(err.to_string());
                    continue;
                }
            };

            {
                let valid_fields = [
                    String::from("book"),
                    String::from("rust"),
                    String::from("build"),
                    String::from("output"),
                    String::from("preprocessor"),
                ];
                let mut fields = String::new();
                everything
                    .iter()
                    .filter(|(k, _v)| !valid_fields.contains(*k))
                    .for_each(|(k, _v)| {
                        fields += k;
                        fields += " ";
                    });

                if !fields.is_empty() {
                    log::error!("Extra fields in book.toml {book_toml_file:?}: {:?}", fields);
                    mdbook.error = Some(format!("Extra fields in book.toml: {:?}", fields));
                }
            }

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

        let mut summary = String::from("# Summary\n\n");

        summary.push_str(index_page(&mdbooks).as_str());
        summary.push_str(errors_page(&mdbooks).as_str());
        summary.push_str(book_toml_page().as_str());
        summary.push_str(book_page(&mdbooks).as_str());
        summary.push_str(rust_page(&mdbooks).as_str());
        summary.push_str(build_page(&mdbooks).as_str());
        summary.push_str(output_page(&mdbooks).as_str());
        summary.push_str(preprocessor_page(&mdbooks).as_str());
        summary.push_str(create_book_pages(&mdbooks).as_str());

        std::fs::write("report/src/SUMMARY.md", summary.as_bytes())?;
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

fn clone_repositories(args: &Cli, repos_dir: &Path, mdbooks: &mut Vec<MDBook>) {
    let mut count = 0;
    for mdbook in mdbooks {
        log::info!("book: {:?}", mdbook);
        match mdbook.repo.update_repository(repos_dir, false) {
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

fn index_page(mdbooks: &Vec<MDBook>) -> String {
    let summary = String::from("- [mdBooks](./index.md)\n");
    let now = chrono::Utc::now();
    let mut md = String::from("# Public mdBooks\n\n");
    md += "This is a list of mdBooks with public source.\n";
    md += "If you are using mdBook we hope that this site will help you learn how others are using it, which plugins are available and how to use those.\n";
    md += "If you are developing plugins for mdBook or mdBook itself, then we hope this will help you see who uses your plugin and how it is being used.\n";
    md += "The list is generated from the `mdbooks.yaml` file in our [repository](https://github.com/szabgab/public-mdbooks).\n\n";
    md += "If you would like to add a book to this list, or add a description please submit a PR to change the `mdbooks.yaml` file.\n\n";
    md += "Check out the [mdBook User manual](https://rust-lang.github.io/mdBook/) for more information.\n\n";
    md += format!("Total number of books: {}\n\n", mdbooks.len()).as_str();
    md += format!("Generated at: {}\n\n", now.format("%Y-%m-%d %H:%M:%S")).as_str();
    md += "| Title | Repo | Description | Comment |\n";
    md += "|-------|------|-------------|---------|\n";
    for mdbook in mdbooks {
        md += format!(
            "| [{}]({}) | [repo]({}) | {} | {} |\n",
            mdbook.title,
            mdbook.relative(),
            mdbook.repo.url(),
            mdbook.description.clone().unwrap_or("".to_string()),
            mdbook.comment.clone().unwrap_or("".to_string()),
        )
        .as_str();
    }
    std::fs::write("report/src/index.md", md).unwrap();

    summary
}

fn book_toml_page() -> String {
    let summary = String::from("- [book.toml](./book-toml.md)\n");

    let mut md = String::from("# book.toml\n\n");
    md += "The book.toml file is the main [configuration file](https://rust-lang.github.io/mdBook/format/configuration/) of every mdbook.\n";
    md +=
        "In this chapter we analyzet the content of the book.toml files in the listed mdbooks.\n\n";
    std::fs::write("report/src/book-toml.md", md).unwrap();

    summary
}

fn book_page(mdbooks: &Vec<MDBook>) -> String {
    let mut summary = String::from("  - [book](./book.md)\n");

    let mut md = String::from("# book\n\n");
    md += "The `book` section in the `book.toml file\n";
    std::fs::write("report/src/book.md", md).unwrap();

    summary.push_str(src_page(mdbooks).as_str());
    summary.push_str(language_page(mdbooks).as_str());
    summary.push_str(text_direction_page(mdbooks).as_str());
    summary.push_str(multilingual_page(mdbooks).as_str());

    summary
}

fn errors_page(mdbooks: &Vec<MDBook>) -> String {
    let summary = String::from("- [errors](./errors.md)\n");
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
        md += format!("* [{}]({})\n", mdbook.title, mdbook.relative(),).as_str();

        md += format!("* [repo]({})\n\n", mdbook.repo.url(),).as_str();
        md += format!("{}\n\n", mdbook.error.clone().unwrap_or("".to_string())).as_str();
        md += "---\n\n";
    }
    std::fs::write("report/src/errors.md", md).unwrap();

    summary
}

fn src_page(mdbooks: &Vec<MDBook>) -> String {
    let summary = String::from("    - [src](./src.md)\n");
    let mut md = String::from("# The book.src field\n\n");

    md += "| Title | src |\n";
    md += "|-------|-------------|\n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | {} | \n",
            mdbook.title,
            mdbook.relative(),
            if bk.book.src.is_none() {
                "-".to_string()
            } else {
                bk.book.src.clone().unwrap()
            }
        )
        .as_str();
    }

    std::fs::write("report/src/src.md", md).unwrap();

    summary
}

fn language_page(mdbooks: &Vec<MDBook>) -> String {
    let mut summary = String::from("    - [language](./language.md)\n");
    let mut md = String::from("# The book.language field\n\n");

    let mut missing = HashSet::new();
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }
        let bk = mdbook.book.as_ref().unwrap();
        if bk.book.language.is_none() {
            continue;
        }
        if LANGUAGES
            .iter()
            .any(|l| l.code == bk.book.language.clone().unwrap())
        {
            continue;
        }
        missing.insert(bk.book.language.clone().unwrap());
    }
    if !missing.is_empty() {
        md += format!(
            "The following languages are not in our list of supported languages: {:?} Please open an [issue]({})\n\n",
            missing,
            REPO
        )
        .as_str();
    }

    md += "| Title | language |\n";
    md += "|-------|-------------|\n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | {} | \n",
            mdbook.title,
            mdbook.relative(),
            if bk.book.language.is_none() {
                "-".to_string()
            } else {
                bk.book.language.clone().unwrap()
            }
        )
        .as_str();
    }

    std::fs::write("report/src/language.md", md).unwrap();

    for language in LANGUAGES {
        summary.push_str(single_language_page(mdbooks, &language).as_str());
    }

    summary
}

fn single_language_page(mdbooks: &Vec<MDBook>, language: &Language) -> String {
    let summary = format!(
        "      - [{}](./language-{}.md)\n",
        language.name, language.code
    );
    let mut md = format!(
        "# {} - book.language = {}\n\n",
        language.name, language.code
    );

    md += "| Title | language |\n";
    md += "|-------|-------------|\n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        if bk.book.language.is_none() {
            continue;
        }
        if bk.book.language.clone().unwrap() != language.code {
            continue;
        }
        md += format!(
            "| [{}]({}) | {} | \n",
            mdbook.title,
            mdbook.relative(),
            bk.book.language.clone().unwrap()
        )
        .as_str();
    }

    std::fs::write(format!("report/src/language-{}.md", language.code), md).unwrap();

    summary
}

fn text_direction_page(mdbooks: &Vec<MDBook>) -> String {
    let summary = String::from("    - [text-direction](./text-direction.md)\n");
    let mut md = String::from("# The book.text-direction field\n\n");

    md += "| Title |  text-direction |\n";
    md += "|-------|-------------|\n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | {} | \n",
            mdbook.title,
            mdbook.relative(),
            if bk.book.text_direction.is_none() {
                "-".to_string()
            } else {
                bk.book.text_direction.clone().unwrap()
            }
        )
        .as_str();
    }

    std::fs::write("report/src/text-direction.md", md).unwrap();

    summary
}

fn multilingual_page(mdbooks: &Vec<MDBook>) -> String {
    let summary = String::from("    - [multilingual](./multilingual.md)\n");
    let mut md = String::from("# The book.multilingual field\n\n");

    md += "| Title | multilingual |\n";
    md += "|-------|-------------|\n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | {} | \n",
            mdbook.title,
            mdbook.relative(),
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

    summary
}

fn rust_page(mdbooks: &Vec<MDBook>) -> String {
    let summary = String::from("  - [rust](./rust.md)\n");

    let mut md = String::from("# The rust.edition field\n\n");

    md += "| Title | editon |\n";
    md += "|-------|-------------|\n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | {} | \n",
            mdbook.title,
            mdbook.relative(),
            match &bk.rust {
                None => String::new(),
                Some(rust) => rust.edition.clone(),
            }
        )
        .as_str();
    }

    std::fs::write("report/src/rust.md", md).unwrap();

    summary
}

fn build_page(mdbooks: &Vec<MDBook>) -> String {
    let mut md = String::from("# The build table\n\n");
    let summary = String::from("  - [build](./build.md)\n");

    md += "| Title | build-dir | create-missing | extra-watch-dirs | use-default-preprocessors |\n";
    md += "|-------|-------------|--------|------|------| \n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let bk = mdbook.book.as_ref().unwrap();
        md += format!(
            "| [{}]({}) | {} | {} | {} | {} |\n",
            mdbook.title,
            mdbook.relative(),
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

    summary
}

fn output_page(mdbooks: &Vec<MDBook>) -> String {
    let mut md = String::from("# output\n\n");
    let summary = String::from("  - [output](./output.md)\n");

    md += "| Title | output field | \n";
    md += "|-------|-------------| \n";
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
            "| [{}]({}) | {} | \n",
            mdbook.title,
            mdbook.relative(),
            fields,
        )
        .as_str();
    }

    std::fs::write("report/src/output.md", md).unwrap();

    summary
}

fn preprocessor_page(mdbooks: &Vec<MDBook>) -> String {
    let mut md = String::from("# preprocessor\n\n");
    let preprocessor_names = PREPROCESSORS.iter().map(|p| p.name).collect::<Vec<&str>>();

    md += "| Title | preprocessors | \n";
    md += "|-------|-------------| \n";
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
                            if preprocessor_names.contains(&k.as_str()) {
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
            "| [{}]({}) |  {} | \n",
            mdbook.title,
            mdbook.relative(),
            fields,
        )
        .as_str();
    }

    let mut summary = String::from("  - [preprocessor](./preprocessor.md)\n");
    std::fs::write("report/src/preprocessor.md", md).unwrap();

    for preprocessor in PREPROCESSORS.iter() {
        preprocessor_details_page(mdbooks, preprocessor);
        summary += format!(
            "    - [{}](./preprocessor-{}.md)\n",
            preprocessor.name, preprocessor.name
        )
        .as_str();
    }

    summary
}

fn preprocessor_details_page(mdbooks: &Vec<MDBook>, preprocessor: &Preprocessor) {
    let mut md = format!("# preprocessor {}\n\n", preprocessor.name);

    md += format!(
        "The preprocessor {} is available on [crates.io]({}).\n\n",
        preprocessor.name, preprocessor.cratesio
    )
    .as_str();
    md += format!("{}\n\n", preprocessor.description).as_str();

    md += "| Title | preprocessor field | \n";
    md += "|-------|-------------| \n";
    for mdbook in mdbooks {
        if mdbook.book.is_none() {
            continue;
        }

        let table = mdbook.everything.as_ref().unwrap();
        match table.get("preprocessor") {
            None => continue,
            Some(preprocessor_table) => match preprocessor_table.get(preprocessor.name) {
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
                        "| [{}]({}) | {} | \n",
                        mdbook.title,
                        mdbook.relative(),
                        fields,
                    )
                    .as_str();
                }
            },
        };
    }
    let path = format!("report/src/preprocessor-{}.md", preprocessor.name);
    std::fs::write(path, md).unwrap();
}

fn books_page() {
    let mut md = String::from("# Public mdBooks\n\n");
    md += "In this section you can find detailed information about the mdBooks.\n";

    std::fs::write("report/src/books.md", md).unwrap();
}

fn create_book_pages(mdbooks: &Vec<MDBook>) -> String {
    let mut summary = String::from("* [Public mdBooks](./books.md)\n");
    books_page();

    for mdbook in mdbooks {
        let filename = mdbook
            .repo
            .path(&std::fs::canonicalize("report/src/").unwrap());
        let path = filename.parent().unwrap();
        log::warn!("path: {:?}", path);
        std::fs::create_dir_all(path).unwrap();
        let mut md = format!("# {}\n\n", mdbook.title);
        let folder = if mdbook.folder.is_none() {
            "".to_string()
        } else {
            format!("  (folder: {})", mdbook.folder.clone().unwrap())
        };
        md += format!("* [repo]({}){}\n", mdbook.repo.url(), folder).as_str();
        md += match &mdbook.site {
            Some(site) => format!("* [site]({})\n", site.to_owned()),
            None => String::from("* site: NA\n"),
        }
        .as_str();
        md += format!(
            "* description: {}\n",
            mdbook.description.clone().unwrap_or("NA".to_string())
        )
        .as_str();
        md += format!(
            "* comment: {}\n",
            mdbook.comment.clone().unwrap_or("NA".to_string())
        )
        .as_str();
        md += format!("\n## book.toml\n\n```toml\n{}\n```\n", mdbook.raw_book_toml).as_str();

        // TODO: use add_extension when it becomes available
        let filename = format!("{}.md", filename.as_os_str().to_str().unwrap());
        log::warn!("filename: {:?}", filename);
        std::fs::write(filename, md).unwrap();

        summary += format!("  * [{}]({})\n", mdbook.title, mdbook.relative()).as_str();
    }
    summary
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
