use clap::{Args, Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle, Style};
use nucleo::Matcher;
use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;

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
                ProgressStyle::new()
                    .tick_chars("[=>-]")
                    .template("[{elapsed_precise}] {bar} {pos}/{len}"),
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

            progress.set_length(entries.count());
            for entry in entries {
                progress.inc(1);

                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if matcher
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
