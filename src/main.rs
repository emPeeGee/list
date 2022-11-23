use clap::{Arg, Command};
use colored::Colorize;
use std::{fs, io};

struct EntriesView {
    name_len: usize,
    entries: Vec<Entry>,
}

enum EntryType {
    File,
    Dir,
}

struct Entry {
    name: String,
    len: u64,
    modified: u64,
    accessed: u64,
    created: u64,
    kind: EntryType,
    readonly: bool,
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
    let matches = Command::new("list")
        .about("ls clone")
        .version("0.0.1")
        .arg_required_else_help(true)
        .author("emPeeGee")
        .arg(
            Arg::new("path")
                .default_value(".")
                .help("location to list the contained entries")
                .takes_value(true),
        )
        .subcommand(
            Command::new("reverse")
                .short_flag('r')
                .long_flag("reverse")
                .about("reverse listing"),
        )
        .subcommand(
            Command::new("non-hidden")
                .short_flag('h')
                .long_flag("hidden")
                .about("Listen without hidden"),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let mut entries = get_entries_from_path(path).unwrap();

    match matches.subcommand() {
        Some(("reverse", _)) => entries.reverse(),
        Some(("non-hidden", _)) => entries.retain(|e| !e.name.starts_with(".")),
        _ => {} // If all subcommands are defined above, anything else is unreachable
    }

    output(&entries);
}

fn get_entries_from_path(path: &String) -> Result<Vec<Entry>, io::Error> {
    let entries = fs::read_dir(path)
        .expect("something is wrong with provided path")
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
                    len: metadata.len(),
                    readonly: metadata.permissions().readonly(),
                    modified: metadata.modified().unwrap().elapsed().unwrap().as_secs(),
                    accessed: metadata.accessed().unwrap().elapsed().unwrap().as_secs(),
                    created: metadata.created().unwrap().elapsed().unwrap().as_secs(),
                    kind,
                }
            })
        })
        .collect::<Result<Vec<_>, io::Error>>();

    entries
}

fn output(entries: &Vec<Entry>) {
    let max_name_len = entries.iter().map(|entry| entry.name.len()).max().unwrap();
    let header_row = format!(
        "{:2} {:max_name_len$} {:4} {:7} {:8} {:5}",
        "#", "name", "Len", "Mod At", "Readonly", "Kind"
    )
    .black()
    .on_yellow();

    println!("{}", header_row);
    entries.iter().enumerate().for_each(|(entry_idx, e)| {
        let name = format!("{:max_name_len$}", e.name);
        let name = match e.kind {
            EntryType::File => name.green(),
            EntryType::Dir => name.blue(),
        };

        let row = format!(
            "{:2} {:max_name_len$} {:4} {:7} {:8} {:5}",
            entry_idx,
            name,
            e.len,
            e.modified,
            e.readonly,
            e.kind.as_str()
        );

        let row = match is_even(entry_idx) {
            true => row.on_bright_black(),
            false => row.on_black(),
        };

        println!("{}", row);
    })
}

// should be generic
fn is_even(number: usize) -> bool {
    number % 2 == 0
}
