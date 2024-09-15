use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io};

use dlx::chop;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the directory to fix
    #[arg(short, long)]
    path: String,

    /// Extension of the file to delete
    #[arg(short, long)]
    ext: String,

    /// Whether to consider sub-directories
    #[clap(long, short)]
    sub: bool,

    /// No changes will be made if this is passed
    #[clap(long, short)]
    dry: bool,

    /// Consider files created older than this ["y","mon","w","d","h","m","s", "ms", "µs", "ns"]
    #[clap(long, short)]
    created_before: Option<String>,

    /// Consider files modified older than this ["y","mon","w","d","h","m","s", "ms", "µs", "ns"]
    #[clap(long, short)]
    modified_before: Option<String>,
}

fn main() {
    let mut files: HashMap<PathBuf, u64> = HashMap::new();
    let args = Args::parse();
    let mut confirm = String::new();
    chop(
        args.path.as_str(),
        args.sub,
        args.dry,
        args.created_before.as_ref(),
        args.modified_before.as_ref(),
        args.ext.as_str(),
        &mut files,
    );
    if !args.dry {
        println!("Do you want to delete these files. Only yes will be accepted.");
        io::stdin()
            .read_line(&mut confirm)
            .expect("Failed to read confirmation");
        if confirm.eq("yes\n") {
            for (file, _) in files {
                println!("Deleting {:?}", file);
                match fs::remove_file(&file) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to delete: {:?} due to error: {:#?}", file, e)
                    }
                }
            }
        }
    }
}
