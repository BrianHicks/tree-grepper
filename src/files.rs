use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub struct Files {
    todo: Vec<PathBuf>,
}

impl Files {
    pub fn new(todo: Vec<PathBuf>) -> Files {
        Files { todo }
    }
}

impl Iterator for Files {
    type Item = Result<(PathBuf, fs::Metadata)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match self.todo.pop() {
                Some(path) => path,
                None => return None,
            };

            let metadata = match fs::metadata(&next) {
                Ok(metadata) => metadata,
                Err(err) => {
                    return Some(Err(err).context(format!(
                        "couldn't get filesystem metadata for {}",
                        next.display()
                    )))
                }
            };

            if metadata.is_dir() {
                match fs::read_dir(&next) {
                    Ok(entries) => {
                        for entry_result in entries {
                            match entry_result {
                                Ok(entry) => self.todo.push(entry.path()),
                                Err(err) => {
                                    return Some(Err(err).context(format!(
                                        "couldn't read an entry from {}",
                                        next.display()
                                    )))
                                }
                            }
                        }
                    }
                    Err(err) => {
                        return Some(
                            Err(err).context(format!(
                                "couldn't read {} as a directory",
                                next.display()
                            )),
                        )
                    }
                }
            } else {
                return Some(Ok((next, metadata)));
            }
        }
    }
}
