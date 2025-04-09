use rayon::prelude::*;

use crate::GroupBy;
use crate::{
    groups::FileType,
    unique_id::{UniqueID, generate_unique_id},
};
use crossbeam::channel;

use crate::FilesizeType;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    thread,
};

pub enum Error {
    NoMetadataForPath(PathBuf),
    CouldNotReadDir(PathBuf),
}

enum Message {
    SizeEntry(Option<UniqueID>, PathBuf, u64),
    Error { error: Error },
}

fn walk(tx: channel::Sender<Message>, entries: &[PathBuf], filesize_type: FilesizeType) {
    entries.into_par_iter().for_each_with(tx, |tx_ref, entry| {
        if let Ok(metadata) = entry.symlink_metadata() {
            let unique_id = generate_unique_id(&metadata);

            let size = filesize_type.size(&metadata);

            tx_ref
                .send(Message::SizeEntry(unique_id, entry.to_path_buf(), size))
                .unwrap();

            if metadata.is_dir() {
                let mut children = vec![];
                match fs::read_dir(entry) {
                    Ok(child_entries) => {
                        for child_entry in child_entries.flatten() {
                            children.push(child_entry.path());
                        }
                    }
                    Err(_) => {
                        tx_ref
                            .send(Message::Error {
                                error: Error::CouldNotReadDir(entry.clone()),
                            })
                            .unwrap();
                    }
                }

                walk(tx_ref.clone(), &children[..], filesize_type);
            };
        } else {
            tx_ref
                .send(Message::Error {
                    error: Error::NoMetadataForPath(entry.clone()),
                })
                .unwrap();
        };
    });
}

fn get_ext<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .extension()
        .unwrap_or(path.as_ref().file_name().unwrap_or_default())
        .to_ascii_lowercase()
        .to_str()
        .unwrap_or_default()
        .to_owned()
}
fn get_filename<P: AsRef<Path>>(path: P) -> String {
    path.as_ref()
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_owned()
}

pub struct Walk<'a> {
    root_dirs: &'a Vec<PathBuf>,
    num_threads: usize,
    filesize_type: FilesizeType,
    group_by: GroupBy,
}

impl<'a> Walk<'a> {
    pub fn new(
        root_dirs: &'a Vec<PathBuf>,
        num_threads: usize,
        filesize_type: FilesizeType,
        group_by: GroupBy,
    ) -> Walk<'a> {
        Walk {
            root_dirs,
            num_threads,
            filesize_type,
            group_by,
        }
    }

    pub fn run(&self) -> (u64, HashMap<String, u64>, Vec<Error>) {
        let (tx, rx) = channel::unbounded();
        let group_by = self.group_by;

        let receiver_thread = thread::spawn(move || {
            let mut total = 0;
            let mut ids = HashSet::new();
            let mut sizes: HashMap<String, u64> = HashMap::new();
            let mut error_messages: Vec<Error> = Vec::new();

            for msg in rx {
                match msg {
                    Message::SizeEntry(unique_id, path, size) => {
                        if let Some(unique_id) = unique_id {
                            // Only count this entry if the ID has not been seen
                            if !ids.insert(unique_id) {
                                continue;
                            }

                            match group_by {
                                GroupBy::Type => {
                                    let filetype = FileType::get_filetype(&get_ext(path));
                                    sizes
                                        .entry(filetype.to_string())
                                        .and_modify(|s| *s += size)
                                        .or_insert(size);
                                }
                                GroupBy::Extension => {
                                    let ext = get_ext(path);
                                    sizes.entry(ext).and_modify(|s| *s += size).or_insert(size);
                                }
                                GroupBy::FileName => {
                                    let filename = get_filename(path);
                                    sizes
                                        .entry(filename)
                                        .and_modify(|s| *s += size)
                                        .or_insert(size);
                                }
                            }
                            total += size;
                        } else {
                            total += size;
                        }
                    }
                    Message::Error { error } => {
                        error_messages.push(error);
                    }
                }
            }
            (total, sizes, error_messages)
        });

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.num_threads)
            .build()
            .unwrap();
        pool.install(|| walk(tx, self.root_dirs, self.filesize_type));

        receiver_thread.join().unwrap()
    }
}
