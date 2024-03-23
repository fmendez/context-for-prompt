use std::path::{Path, PathBuf};

use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use ignore::WalkBuilder;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    root: String,
    #[clap(short, long, use_value_delimiter = true)]
    extensions_to_ignore: Vec<String>,
    #[clap(short, long, default_value = "false")]
    debug: bool,

    #[clap(long, default_value = "true")]
    hidden: bool,
}

fn filter_files(root: &Path, extensions_to_ignore: &[String], hidden: bool) -> Vec<PathBuf> {
    let mut files = vec![];

    for result in WalkBuilder::new(root).hidden(hidden).build() {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.file_type().map_or(false, |ft| ft.is_file())
            && !extensions_to_ignore.iter().any(|ext| {
                entry
                    .path()
                    .extension()
                    .map_or(false, |e| e.to_str() == Some(ext))
            })
        {
            files.push(entry.into_path());
        }
    }

    files
}

fn set_clipboard_content(files: Vec<PathBuf>) -> String {
    let mut clipboard_content = String::new();
    for file in files {
        let file_display = format!("file: {}", file.display());
        clipboard_content.push_str(&file_display);
        clipboard_content.push('\n');

        clipboard_content.push_str("----------- content start -------------\n");

        let content = std::fs::read_to_string(&file).unwrap();
        clipboard_content.push_str(&content);
        clipboard_content.push('\n');

        clipboard_content.push_str("----------- content end -------------\n");
    }

    clipboard_content
}

fn main() {
    let args = Args::parse();
    let root = Path::new(&args.root);

    let files = filter_files(root, &args.extensions_to_ignore, args.hidden);

    let clipboard_content = set_clipboard_content(files);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(clipboard_content.clone()).unwrap();
    if args.debug {
        println!("{}", &clipboard_content);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_filter_files() {
        let dir = tempdir().unwrap();

        let test_files = vec![
            "test.txt",
            "test.lock",
            "another_test.txt",
            "another_test.js",
        ];
        for file in test_files {
            let file_path = dir.path().join(file);
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "Hello, world!").unwrap();
        }

        let ignored_extensions = vec!["lock".to_string(), "js".to_string()];
        let files = filter_files(dir.path(), &ignored_extensions, false);

        assert!(!files.is_empty());
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_set_clipboard_content() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let files = vec![file_path];
        let clipboard_content = set_clipboard_content(files);

        assert!(!clipboard_content.is_empty());
        assert!(clipboard_content.contains("Hello, world!"));
    }
}
