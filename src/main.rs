use std::collections::HashMap;
use std::fs::{DirEntry, File};
use std::io::{BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let path = std::env::args().skip(1).next().unwrap_or(".".into());
    let Some(projects) = walk_dir(path) else {
        eprintln!("Invalid path.");
        return;
    };

    let mut project_map: HashMap<usize, PathBuf> = (0..).zip(projects.into_iter()).collect();
    let mut removed_idxs = vec![];

    (0..project_map.len())
        .for_each(|idx| println!("{idx}: {}", project_map.get(&idx).unwrap().display()));

    loop {
        let input = read_prompt("idx, go, or quit: ").expect("Failed to read input. Cancelling.");
        match input.to_lowercase().trim() {
            "quit" => return,
            "go" => {
                run(project_map);
                println!("Done.");
                return;
            }
            maybe_idx => {
                match maybe_idx.parse() {
                    Ok(idx) => {
                        if idx > project_map.len() || removed_idxs.contains(&idx) {
                            eprintln!("Out of bounds idx {idx}.");
                            continue;
                        }

                        let removed = project_map.remove(&idx).expect("Already asserted it's within bounds.");
                        println!("Removed idx {idx}: {}", removed.display());
                        removed_idxs.push(idx);
                    }
                    Err(_) => {
                        eprintln!("Bad input.");
                    }
                }
            }
        }
    }
}

fn run(projects: HashMap<usize, PathBuf>) {
    projects.into_iter()
        .map(|(_, path)| path)
        .for_each(|path| {
            println!("Running `cargo clean` in {}.\n", path.display());

            let mut cmd = Command::new("cargo");
            cmd.arg("clean");
            cmd.current_dir(path.clone());
            cmd.spawn().expect(&format!("Failed to run `cargo clean` in {}", path.display()));
        })
}

fn read_prompt(prompt: &str) -> Option<String> {
    print!("{prompt}");
    let _ = std::io::stdout().flush();

    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();
    match lock.read_line(&mut buf) {
        Ok(_) => Some(buf),
        _ => None,
    }
}

fn walk_dir<P: AsRef<Path>>(path: P) -> Option<Vec<PathBuf>> {
    fn is_cargo_project<P: AsRef<Path>>(path: P) -> bool {
        let mut buf = PathBuf::from(path.as_ref());
        buf.push("Cargo.toml");
        File::open(buf).is_ok()
    }

    let mut projects = vec![];

    std::fs::read_dir(path).ok()?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .for_each(|folder| {
            if is_cargo_project(folder.path()) {
                projects.push(folder.path());
            } else {
                let mut sub_projects = walk_dir(folder.path()).unwrap_or(vec![]);
                projects.append(&mut sub_projects);
            }
        });

    Some(projects)
}