use std::path::Path;

use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use ignore::WalkBuilder;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    root: String,
    #[clap(short, long, default_value = "")]
    extensions_to_ignore: Vec<String>,
    #[clap(short, long, default_value = "false")]
    debug: bool,

    #[clap(long, default_value = "true")]
    hidden: bool,
}

fn main() {
    let args = Args::parse();
    let root = Path::new(&args.root);
    let mut files = vec![];

    for result in WalkBuilder::new(root).hidden(args.hidden).build() {
        let entry = match result {
            Ok(e) => e,
            Err(err) => {
                eprintln!("ERROR: {}", err);
                continue;
            }
        };

        // Check if the file should be ignored based on its extension
        if entry.file_type().map_or(false, |ft| ft.is_file())
            && !args.extensions_to_ignore.iter().any(|ext| {
                entry
                    .path()
                    .extension()
                    .map_or(false, |e| e.to_str() == Some(ext))
            })
        {
            files.push(entry.into_path());
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
