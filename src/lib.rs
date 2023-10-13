use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use duration_str::parse;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn chop(
    path: &str,
    sub: bool,
    dry: bool,
    created: Option<&String>,
    modified: Option<&String>,
    ext: &str,
    buffet: &mut HashMap<PathBuf, u64>,
) {
    let mut subs = vec![];
    let current_dir = std::path::PathBuf::from(path);
    for entry in fs::read_dir(current_dir).expect("Not a valid directory") {
        let entry = entry.expect("Not valid");
        let path = entry.path();
        if path.is_file() && path.extension().is_some() {
            let extension = path.extension().as_ref().unwrap().to_str().unwrap();
            if extension.eq(ext) {
                let k = path.clone();
                let v = entry
                    .metadata()
                    .unwrap()
                    .created()
                    .unwrap()
                    .elapsed()
                    .unwrap()
                    .as_secs();
                match created {
                    None => {
                        println!("Picked file {:?} which was created {} seconds ago", k, v);
                        buffet.insert(k, v);
                    }
                    Some(creation) => match parse(creation) {
                        Ok(creation) => {
                            if entry
                                .metadata()
                                .unwrap()
                                .created()
                                .unwrap()
                                .elapsed()
                                .unwrap()
                                .gt(&creation)
                            {
                                println!("Picked file {:?} which was created {} seconds ago", k, v);
                                buffet.insert(k, v);
                            }
                        }
                        Err(_) => {
                            eprintln!("Unable to parse creation time given as {:#?}", creation);
                        }
                    },
                };
            }
        } else if path.is_dir() && sub {
            subs.push(String::from(
                path.to_str().expect("Cannot convert path to string"),
            ))
        }
    }
    for p in &subs {
        chop(p.as_str(), sub, dry, created, modified, ext, buffet)
    }
}

#[cfg(test)]
mod tests {
    use duration_str::parse;

    use super::*;

    #[test]
    fn it_works() {
        let current_dir = std::path::PathBuf::from("/Users/m/D/scdl");
        for entry in fs::read_dir(current_dir).expect("Not a valid directory") {
            let entry = entry.expect("Not valid");
            let path = entry.path();
            if path.is_file() {
                let creation = entry.metadata().expect("No metadata").created().unwrap();
                let duration = parse("4h").unwrap();
                println!(
                    "File {} was created {:#?} seconds ago",
                    entry.path().to_str().unwrap(),
                    creation.elapsed().unwrap().as_secs()
                );
                println!(
                    "File older than {:?} = {}",
                    duration,
                    creation.elapsed().unwrap().gt(&duration)
                )
            }
        }
    }
}
