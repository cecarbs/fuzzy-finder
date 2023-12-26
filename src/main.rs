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
    // test_with_directories();

    // pattern_match_crate();
    // walk_directory();
    walk_directory_and_fuzzy_match_at_end();
    // Print the matching results, highlighting matches
    // for (i, result) in results.iter().enumerate() {
    //     let highlighted = highlight(&words[i], &query);
    //     println!("{}. {}", i + 1, highlighted);
    // }
}

// TODO: THIS IS WHAT IS MEANT BY USING THE HIGH LEVEL nucleo CRATE INSTEAD OF CALLING
// .match_list()
fn test_with_directories() {
    let haystack = vec![
        Utf32String::from("Projects/rust"),
        Utf32String::from("Projects/something/rust_analyzer"),
        Utf32String::from("Projects/rust/fuzzy-finder"),
        Utf32String::from("Projects/rust/t"),
        Utf32String::from("Projects/javascript/fuzzy-finder"),
    ];

    let needle = "rust";

    let mut nucleo = Matcher::new(nucleo::Config::DEFAULT.match_paths());
    // let mut nucleo = nucleo_matcher::Matcher::new(nucleo_matcher::Config::DEFAULT.match_paths());

    for word in &haystack {
        let result = nucleo.fuzzy_match(word.slice(..), Utf32Str::Ascii(needle.as_bytes()));
        black_box(result); // Prevent compiler optimizations

        println!("Match score for '{}': {:?}", word, result);
    }
}

fn pattern_match_crate() {
    let paths = ["apple", "foo/bar", "bar/foo", "foobar", "banana"];
    let mut matcher = nucleo_matcher::Matcher::new(Config::DEFAULT.match_paths());
    // let matches = nucleo_matcher::pattern::Pattern::parse(
    //     "foo bar",
    //     CaseMatching::Ignore,
    //     Normalization::Smart,
    // )
    // .match_list(paths, &mut matcher);
    // TODO: use the needle and haystack with the nucleo crate to see if it provices the same
    // result
    let matches = nucleo_matcher::pattern::Pattern::parse(
        "foo bar",
        CaseMatching::Ignore,
        Normalization::Smart,
    )
    .match_list(paths, &mut matcher);

    println!(
        "the matches from the nucleo_matcher crate are: {:?}",
        matches
    );
}

fn walk_directory() {
    let start = Instant::now();
    // let home_dir = dirs::home_dir().unwrap();
    let projects_dir = dirs::home_dir().unwrap().join("Projects");

    let walker = WalkDir::new(projects_dir).into_iter();

    for entry in walker {
        let entry = entry.unwrap();
        // If the entry is a direcotry
        if entry.file_type().is_dir() {
            // print its path in a user-friendly format
            let path = entry.path().to_str();
            println!("{}", path.unwrap());
            // println!("{}", entry.path().display());
        }
    }
    let duration = start.elapsed();
    println!("Total time: {:?}", duration);
}

// fn walk_directory_and_fuzzy_match_at_each_path() {
//     let start = Instant::now();
//
//     let projects_dir = dirs::home_dir().unwrap().join("Projects");
//
//     let walker = WalkDir::new(projects_dir).into_iter();
//     let mut haystack = Vec::new();
//
//     let mut nucleo = Matcher::new(nucleo::Config::DEFAULT.match_paths());
//
//     let needle = "rust";
//     for entry in walker {
//         let entry = entry.unwrap();
//
//         if entry.file_type().is_dir() {
//             let path = entry.path().to_str();
//         }
//     }
//
//     let duration = start.elapsed();
//     println!("Total time: {:?}", duration);
//     // 3.5 seconds to run
// }
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

        if entry.file_type().is_dir() {
            let path = entry.path().to_str();
            haystack.push(Utf32String::from(path.unwrap()));
            // if path.unwrap().ends_with("rust") {
            //     haystack.push(Utf32String::from(path.unwrap()));
            //     continue;
            // }
            // let something = entry.path().ends_with()
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

// fn main() {
//     let cli = Cli::parse();
//
//     match cli.command {
//         Command::Find { directory } => {
//             // TODO: make sure this starts from root
//             let path = PathBuf::from(directory);
//             let matcher = Matcher::default();
//             // TODO: change this, should be the search term from the command line
//             let search_term = "rust";
//
//             // Progress bar styling
//             let progress = ProgressBar::new(0);
//             progress.set_style(
//                 ProgressStyle::with_template("[{elapsed_precise}] {bar} {pos}/{len}")
//                     .unwrap()
//                     .tick_chars("[=>-]"),
//             );
//
//             // Initiate search
//             progress.set_message(format!(
//                 "Searching for '{}' in '{}'...",
//                 search_term,
//                 path.display()
//             ));
//
//             let mut matches = Vec::new();
//             let entries = match read_dir(&path) {
//                 Ok(entries) => entries,
//                 Err(error) => panic!("Error reading directory: {}", error),
//             };
//
//             // Progress bar for searching
//             progress.set_length(entries.count() as u64);
//
//             for entry in entries {
//                 progress.inc(1);
//
//                 // TODO: search the current directory
//                 if let Ok(entry) = entry {
//                     let entry_path = entry.path();
//                     if matcher.
//                         .match_many(&[entry_path.to_str().unwrap()], search_term)
//                         .is_empty()
//                     {
//                         continue;
//                     }
//                     matches.push(entry_path);
//                 }
//             }
//
//             progress.finish_and_clear();
//
//             println!("Found {} matches:", matches.len());
//             for match_path in matches {
//                 println!("- {}", match_path.display());
//             }
//         }
//     }
// }

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
// fn fuzzy_find_dirs_recursive(pattern: &str) -> Vec<String> {
//     // let config = Config::pattern(pattern).case_sensitive(true);
//     let config = Config::DEFAULT;
//     let mut results = Vec::new();
//     let matcher = Matcher::new(config);
//
//     // Start at the root directory
//     let mut current_path = std::path::PathBuf::from("/");
//
//     loop {
//         for entry in read_dir(&current_path).unwrap() {
//             let entry = entry.unwrap();
//             let entry_type = entry.metadata().unwrap().file_type();
//
//             // Check if it's a directory and not a symbolic link
//             if entry_type.is_dir() && !entry_type.is_symlink() {
//                 let name = entry.path().display().to_string();
//                 if matcher.match_score(&name) > 0 {
//                     results.push(name)
//                 }
//
//                 // Recursively search inside the directory
//                 current_path.push(entry.file_name());
//                 fuzzy_find_dirs_recursive(pattern);
//
//                 // Go back up on level
//                 current_path.pop();
//                 }
//             }
//             // Break out of the loop when no more directories to explore
//             if let std::result::Result::Ok(None) = read_dir(&current_path) {
//                 break;
//             }
//         }
//
//     results
// }

// TODO: Utilize the nucleo_matcher (low level crate instead)
// fn example() {
//     // For almost all use cases the Pattern API should be used instead of calling the matcher
//     // methods directly. Pattern::parse will construct a single Atom (a single match operation) for
//     // each word. the pattern can contain special characterst to control what kind of match is
//     // performed (see AtomKind).
//     let paths = ["foo/bar", "bar/foo", "foobar"];
//     let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
//     let matches = Pattern::parse("foo bar", CaseMatching::Ignore).match_list(paths, &mut matcher);
//
//     assert_eq!(
//         matches,
//         vec![("foo/bar", 168), ("bar/foo", 168), ("foobar", 140)]
//     );
//     let matches = Pattern::parse("^foo, bar", CaseMatching::Ignore).match_list(paths, &mut matcher);
//     assert_eq!(matches, vec![("foo/bar", 168), ("foobar", 140)]);
//
//     // If the pattern should be matched literally (without special parsing) use Pattern::new
//     // instead
//     let paths = ["foo/bar", "bar/foo", "foobar"];
//     let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
//     let matches = Pattern::new("foo bar", CaseMatching::Ignore, AtomKind::Fuzzy)
//         .match_list(paths, &mut matcher);
//     assert_eq!(
//         matches,
//         vec![("foo/bar", 168), ("bar/foo", 168), ("foobar", 140)]
//     );
//     let paths = ["^foo/bar", "bar/^foo", "foobar"];
//     let matches = Pattern::parse("^foo, bar", CaseMatching::Ignore).match_list(paths, &mut matcher);
//     assert_eq!(matches, vec![("foo/bar", 188), ("bar/^foo", 188)]);
//
//     // If word segmentation is also not desired, a single Atom can be constructed directly
//     let paths = ["foobar", "foo bar"];
//     let mut matcher = Matcher::new(Config::DEFAULT);
//     let matches = Atom::new("foo bar", CaseMatching::Ignore, AtomKind::Fuzzy, false)
//         .match_list(paths, &mut matcher);
//     assert_eq!(matches, vec![("foo bar", 192)]);
// }
