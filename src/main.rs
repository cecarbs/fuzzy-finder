use std::fs;
use fuzzy_matcher::FuzzyMatcher;

fn fuzzy_find_files(root_dir: &str, query: &str ) -> Vec<String> {
    let mut matches = Vec::new();
    let matcher = FuzzyMatcher::new(query);

    // Recursively walk the directory tree
    for entry in fs::read_dir(root_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // Check if it's a directory and recurse
        if entry.file_type().unwrap().is_dir() {
            let sub_matches = fuzzy_find_files(path_to_str().unwrap(), query)
            matches.extend(sub_match);
        } else {

            // Check if filename matches query
            if matcher.is_match(path.file_name().unwrap().to_str().unwrap()) {
                matches.push(path.to_str().unwrap().to_owned())
            }
        }

        matches
    }
}

fn main() {
    let root_dir = ""; //path to home
    let query = ""; // file to search for 
    //
    let matches = fuzzy_find_files(root_dir, query);
    println!("Found {} matches for query '{}' in '{}':", matches.len(), query, root_dir);
    for match in matches {
        println!("{}", match);
    }
}
