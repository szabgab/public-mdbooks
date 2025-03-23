use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use git_digger::get_owner_and_repo;

#[derive(Debug, Serialize, Deserialize)]
struct BookMeta {
    title: String,
    repo: String,
    site: String,
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
    let repos_dir = "repos";

    let books = read_the_mdbooks_file();
    for book in books {
        println!("{:?}", book);
        git_clone_repository(&book.repo);
        // //     let book_toml = temp_dir.join("book.toml");
        // //     let content = std::fs::read_to_string(book_toml).unwrap();
        // //     let data = toml::from_str::<toml::Value>(&content).unwrap();
    }
    // println!("-------");

    //list content of a directory
    let path = PathBuf::from(repos_dir);
    let entries = std::fs::read_dir(path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{:?}", path);
        let book_toml = path.join("book.toml");
        let content = std::fs::read_to_string(book_toml).unwrap();
        let data = toml::from_str::<BookToml>(&content).unwrap();

        println!("{:?}", data);
        // std::process::exit(0);
    }
}

fn read_the_mdbooks_file() -> Vec<BookMeta> {
    let file = std::fs::read_to_string("mdbooks.yaml").unwrap();
    let books: Vec<BookMeta> = serde_yaml::from_str(&file).unwrap();
    books
}

fn git_clone_repository(repo: &str) {
    let (host, owner, name) = get_owner_and_repo(repo);

    //let temp_dir = std::env::temp_dir();
    //let temp_dir = temp_dir.join("mdbooks");
    //let temp_dir = temp_dir.join(repo);
    //let temp_dir_str = temp_dir.to_str().unwrap();
    //let _ = std::fs::create_dir_all(temp_dir_str);
    //let _ = git2::Repository::clone(repo, temp_dir_str);
}

fn repo_to_path(repo: &str) -> PathBuf {
    let mut path = PathBuf::from("repos");
    let parts: Vec<&str> = repo.split('/').collect();
    for part in parts {
        path.push(part);
    }
    path
}
