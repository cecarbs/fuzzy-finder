use clap::{Args, Parser, Subcommand};
use dirs::home_dir;
// use indicatif::{ProgressBar, ProgressStyle, Style};
use indicatif::{ProgressBar, ProgressStyle};
use nucleo::{pattern, Config, Matcher, Nucleo};
use nucleo::{Utf32Str, Utf32String};
use std::path::PathBuf;
use std::time::Instant;
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Find directories and files containing "rust" in the path
    Find {
        /// Starting directory for the search
        #[clap(short, long, default_value = ".")]
        directory: String,
    },
}
fn main() {
    walk_directory_and_fuzzy_match_at_end();
    // Print the matching results, highlighting matches
    // for (i, result) in results.iter().enumerate() {
    //     let highlighted = highlight(&words[i], &query);
    //     println!("{}. {}", i + 1, highlighted);
    // }
}

fn get_starting_directory(starting_directory: Option<&str>) -> PathBuf {
    match starting_directory {
        Some(proj_dir) => dirs::home_dir().unwrap().join(proj_dir),
        None => dirs::home_dir().unwrap(),
    }
}
fn search_for_directory(
    starting_directory: Option<&str>,
    directory_to_search_for: &str,
) -> Vec<Utf32String> {
    let directory = get_starting_directory(starting_directory);

    let walker = WalkDir::new(directory).into_iter();

    let mut directories: Vec<Utf32String> = Vec::new();

    for entry in walker {
        let entry = entry.unwrap();

        if entry.file_type().is_dir()
            && entry.file_name().to_str().unwrap() == directory_to_search_for
        {
            let path = entry.path().to_str();
            directories.push(Utf32String::from(path.unwrap()));
        }
    }
    directories
}

fn fuzzy_match_on_search_results(search_term: &str, search_results: Vec<Utf32String>) {
    let mut nucleo = Matcher::new(Config::DEFAULT.match_paths());

    for path in &search_results {
        let result = nucleo.fuzzy_match(path.slice(..), Utf32Str::Ascii(search_term.as_bytes()));

        if result.is_some() {
            let score = result.unwrap();
            if score > 100 {
                println!("Path is {:?} and score is {:?}", path, score);
            }
        }
    }
}
// TODO: Utilize 1 fn and have that function take in a directory (if none is passed it defaults to
// home) and the needle to look for
fn walk_directory_and_fuzzy_match_at_end() {
    let start = Instant::now();

    let projects_dir = dirs::home_dir().unwrap().join("Projects");
    //
    let walker = WalkDir::new(projects_dir).into_iter();
    let mut haystack = Vec::new();
    //
    let needle = "rust";
    for entry in walker {
        let entry = entry.unwrap();

        if entry.file_type().is_dir() && entry.file_name().to_str().unwrap() == "rust" {
            let path = entry.path().to_str();
            haystack.push(Utf32String::from(path.unwrap()));
        }
    }

    let mut nucleo = Matcher::new(nucleo::Config::DEFAULT.match_paths());

    for word in &haystack {
        let result = nucleo.fuzzy_match(word.slice(..), Utf32Str::Ascii(needle.as_bytes()));

        // println!("Match score for '{:?}': {:?}", word, result);
        if result.is_some() {
            let score = result.unwrap();
            if score > 100 {
                println!("Path is {:?} and score is {:?}", word, score);
            }
        }
    }

    let duration = start.elapsed();
    println!("Total time: {:?}", duration);
    // 1.26 seconds to run
}

// Helper function to highlight matches
fn highlight(text: &str, query: &str) -> String {
    let mut highlighted = String::new();
    let mut prev_index = 0;
    for (index, matched) in text.match_indices(query) {
        highlighted.push_str(&text[prev_index..index]);
        highlighted.push_str("\x1b[31m"); // Start red highlighting
        highlighted.push_str(&text[index..index + matched.len()]);
        highlighted.push_str("\x1b[0m"); // Reset highlighting
        prev_index = index + matched.len();
    }
    highlighted.push_str(&text[prev_index..]);
    highlighted
}
