use clap::{Args, Parser, Subcommand};
// use indicatif::{ProgressBar, ProgressStyle, Style};
use indicatif::{ProgressBar, ProgressStyle};
use nucleo_matcher::{Config, Matcher};
// use nucleo::{Matcher, Config};
use nucleo_matcher::pattern::{CaseMatching, Pattern, Atom, AtomKind};
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Find { directory } => {
            let path = PathBuf::from(directory);
            let matcher = Matcher::default();
            let search_term = "rust";

            let progress = ProgressBar::new(0);
            progress.set_style(
                ProgressStyle::with_template("[{elapsed_precise}] {bar} {pos}/{len}")
                    .unwrap()
                    .tick_chars("[=>-]"),
            );

            progress.set_message(format!(
                "Searching for '{}' in '{}'...",
                search_term,
                path.display()
            ));

            let mut matches = Vec::new();
            let entries = match read_dir(&path) {
                Ok(entries) => entries,
                Err(error) => panic!("Error reading directory: {}", error),
            };

            progress.set_length(entries.count() as u64);
            for entry in entries {
                progress.inc(1);

                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if matcher.
                        .match_many(&[entry_path.to_str().unwrap()], search_term)
                        .is_empty()
                    {
                        continue;
                    }
                    matches.push(entry_path);
                }
            }

            progress.finish_and_clear();

            println!("Found {} matches:", matches.len());
            for match_path in matches {
                println!("- {}", match_path.display());
            }
        }
    }
}

fn fuzzy_find_dirs_recursive(pattern: &str) -> Vec<String> {
    // let config = Config::pattern(pattern).case_sensitive(true);
    let config = Config::DEFAULT;
    let mut results = Vec::new();
    let matcher = Matcher::new(config);

    // Start at the root directory
    let mut current_path = std::path::PathBuf::from("/");

    loop {
        for entry in read_dir(&current_path).unwrap() {
            let entry = entry.unwrap();
            let entry_type = entry.metadata().unwrap().file_type();

            // Check if it's a directory and not a symbolic link
            if entry_type.is_dir() && !entry_type.is_symlink() {
                let name = entry.path().display().to_string();
                if matcher.match_score(&name) > 0 {
                    results.push(name)
                }

                // Recursively search inside the directory
                current_path.push(entry.file_name());
                fuzzy_find_dirs_recursive(pattern);

                // Go back up on level
                current_path.pop();
                }
            }
            // Break out of the loop when no more directories to explore
            if let std::result::Result::Ok(None) = read_dir(&current_path) {
                break;
            }
        }

    results
}

// TODO: Utilize the nucleo_matcher (low level crate instead)
fn example() {
    // For almost all use cases the Pattern API should be used instead of calling the matcher
    // methods directly. Pattern::parse will construct a single Atom (a single match operation) for
    // each word. the pattern can contain special characterst to control what kind of match is
    // performed (see AtomKind).
    let paths = ["foo/bar", "bar/foo", "foobar"];
    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    let matches = Pattern::parse("foo bar", CaseMatching::Ignore).match_list(paths, &mut matcher);

    assert_eq!(matches, vec![("foo/bar", 168), ("bar/foo", 168), ("foobar", 140)]);
    let matches = Pattern::parse("^foo, bar", CaseMatching::Ignore).match_list(paths, &mut matcher);
    assert_eq!(matches, vec![("foo/bar", 168), ("foobar", 140)]);

    // If the pattern should be matched literally (without special parsing) use Pattern::new
    // instead
    let paths = ["foo/bar", "bar/foo", "foobar"];
    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    let matches = Pattern::new("foo bar", CaseMatching::Ignore, AtomKind::Fuzzy).match_list(paths, &mut matcher);
    assert_eq!(matches, vec![("foo/bar", 168), ("bar/foo", 168), ("foobar", 140)]);
    let paths = ["^foo/bar", "bar/^foo", "foobar"];
    let matches = Pattern::parse("^foo, bar", CaseMatching::Ignore).match_list(paths, &mut matcher);
    assert_eq!(matches, vec![("foo/bar", 188), ("bar/^foo", 188)]);

    // If word segmentation is also not desired, a single Atom can be constructed directly
    let paths = ["foobar", "foo bar"];
    let mut matcher = Matcher::new(Config::DEFAULT);
    let matches = Atom::new("foo bar", CaseMatching::Ignore, AtomKind::Fuzzy, false).match_list(paths, &mut matcher);
    assert_eq!(matches, vec![("foo bar", 192)]);
}

