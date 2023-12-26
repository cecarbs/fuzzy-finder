use clap::{Args, Parser, Subcommand};
use dirs::home_dir;
// use indicatif::{ProgressBar, ProgressStyle, Style};
use indicatif::{ProgressBar, ProgressStyle};
use nucleo::{pattern, Config, Matcher, Nucleo};
use nucleo::{Utf32Str, Utf32String};
use pattern::{Atom, AtomKind, CaseMatching, Normalization};
use std::fs::{read_dir, DirEntry};
use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use std::u64;
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
    walk_directory_and_fuzzy_match();
    // walk_directory_and_fuzzy_match_at_end();
    // Print the matching results, highlighting matches
    // for (i, result) in results.iter().enumerate() {
    //     let highlighted = highlight(&words[i], &query);
    //     println!("{}. {}", i + 1, highlighted);
    // }
}

fn walk_directory_and_fuzzy_match() {
    let start = Instant::now();

    let projects_dir = dirs::home_dir().unwrap().join("Projects");

    let walker = WalkDir::new(projects_dir).into_iter();

    let mut haystack = Vec::new();
    //
    let needle = "rust";
    for entry in walker {
        let entry = entry.unwrap();

        if entry.file_type().is_dir() && entry.file_name().to_str().unwrap() == "rust" {
            let path = entry.path().to_str();
            haystack.push(String::from(path.unwrap()));
        }
    }

    let mut matcher = nucleo_matcher::Matcher::new(Config::DEFAULT.match_paths());

    let matches =
        nucleo_matcher::pattern::Pattern::parse(needle, CaseMatching::Ignore, Normalization::Smart)
            .match_list(haystack, &mut matcher);

    println!(
        "the matches from the nucleo_matcher crate are: {:?}",
        matches
    );

    let duration = start.elapsed();
    println!("Total time: {:?}", duration);
    // 1.26 seconds to run
}

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
