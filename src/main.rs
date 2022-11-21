use std::{fs, io, time::SystemTime};

enum EntryType {
    File,
    Dir,
}

struct Entry {
    name: String,
    len: u64,
    modified: u64,
    metadata: fs::Metadata,
    kind: EntryType,
}

impl EntryType {
    fn as_str(&self) -> &'static str {
        match self {
            EntryType::Dir => "Dir",
            EntryType::File => "File",
        }
    }
}

fn main() {
    let paths = fs::read_dir(".").unwrap();

    paths.for_each(|path| {
        println!("Name: {}", path.unwrap().path().display());
    });

    let mut entries = fs::read_dir(".")
        .unwrap()
        .map(|res| {
            res.map(|e| {
                let metadata = e.metadata().unwrap();
                let kind = if metadata.is_file() {
                    EntryType::File
                } else {
                    EntryType::Dir
                };

                Entry {
                    name: e.file_name().into_string().unwrap(),
                    metadata: metadata.to_owned(),
                    len: metadata.len(),
                    modified: metadata.modified().unwrap().elapsed().unwrap().as_secs(),
                    kind,
                }
            })
        })
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    for e in entries {
        println!(
            "{} {} {} {} {}",
            e.name,
            e.len,
            e.modified,
            e.metadata.is_dir(),
            e.kind.as_str()
        );
    }

    // TODO:
    // get path from args
    // table view
    // and much more ???
}
