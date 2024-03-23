use std::path::Path;

use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    root: String,
    #[clap(short, long, default_value = "false")]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    let root = Path::new(&args.root);
    let mut files = vec![];
    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }
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

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(clipboard_content.clone()).unwrap();
    if args.debug {
        println!("{}", &clipboard_content);
    }
}
