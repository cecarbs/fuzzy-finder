use clap::{Args, Parser, Subcommand};
// use indicatif::{ProgressBar, ProgressStyle, Style};
use indicatif::{ProgressBar, ProgressStyle};
// use nucleo_matcher::{Config, Matcher};
use nucleo::{Config, Matcher, Settings};
// use nucleo_matcher::pattern::{Atom, AtomKind, CaseMatching, Pattern};
use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;
use std::u64;

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
// TODO: Based on the documenation, need to swap to nucelo crate
// *Note* This function is not recommended for building a full fuzzy
// matching application that can match large numbers of matches (like all
// files in a directory) as all matching is done on the current thread,
// effectively blocking the UI. For such applications the high level
// `nucleo` crate can be used instead.
fn main() {
    let query = "apple";
    let words = vec!["apple", "banana", "cherry", "apricot", "pineapple"];

    // Create a matcher with default settings
    let matcher = Matcher::new(Settings::default());

    let results = matcher.fuzzy_match(&words, &query);

    // Print the matching results, highlighting matches
    for (i, result) in results.iter().enumerate() {
        let highlighted = highlight(&words[i], &query);
        println!("{}. {}", i + 1, highlighted);
    }
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
fn example() {
    // For almost all use cases the Pattern API should be used instead of calling the matcher
    // methods directly. Pattern::parse will construct a single Atom (a single match operation) for
    // each word. the pattern can contain special characterst to control what kind of match is
    // performed (see AtomKind).
    let paths = ["foo/bar", "bar/foo", "foobar"];
    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    let matches = Pattern::parse("foo bar", CaseMatching::Ignore).match_list(paths, &mut matcher);

    assert_eq!(
        matches,
        vec![("foo/bar", 168), ("bar/foo", 168), ("foobar", 140)]
    );
    let matches = Pattern::parse("^foo, bar", CaseMatching::Ignore).match_list(paths, &mut matcher);
    assert_eq!(matches, vec![("foo/bar", 168), ("foobar", 140)]);

    // If the pattern should be matched literally (without special parsing) use Pattern::new
    // instead
    let paths = ["foo/bar", "bar/foo", "foobar"];
    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    let matches = Pattern::new("foo bar", CaseMatching::Ignore, AtomKind::Fuzzy)
        .match_list(paths, &mut matcher);
    assert_eq!(
        matches,
        vec![("foo/bar", 168), ("bar/foo", 168), ("foobar", 140)]
    );
    let paths = ["^foo/bar", "bar/^foo", "foobar"];
    let matches = Pattern::parse("^foo, bar", CaseMatching::Ignore).match_list(paths, &mut matcher);
    assert_eq!(matches, vec![("foo/bar", 188), ("bar/^foo", 188)]);

    // If word segmentation is also not desired, a single Atom can be constructed directly
    let paths = ["foobar", "foo bar"];
    let mut matcher = Matcher::new(Config::DEFAULT);
    let matches = Atom::new("foo bar", CaseMatching::Ignore, AtomKind::Fuzzy, false)
        .match_list(paths, &mut matcher);
    assert_eq!(matches, vec![("foo bar", 192)]);
}
