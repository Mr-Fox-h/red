use clap::Parser;
use owo_colors::OwoColorize;
use std::{fs, path::PathBuf};
use tabled::{
    Table, Tabled,
    settings::{
        Color, Remove, Style,
        object::{Columns, Rows},
    },
};

#[derive(Debug, Tabled)]
struct Index {
    #[tabled(rename = "No.")]
    index: u64,
}

#[derive(Debug, Tabled)]
struct Context {
    #[tabled(rename = "Content")]
    content: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Concatenate FILE(s) to standard output.")]
struct Cli {
    file: Option<PathBuf>,
    #[arg(short, long, help = "Hide line number")]
    no_num: bool,
    #[arg(short = 'A', long, help = "show non-printable characters")]
    show_all: bool,
}

fn main() {
    let cli: Cli = Cli::parse();
    let file: PathBuf = cli.file.unwrap();

    if let Ok(is_exist) = fs::exists(&file) {
        if is_exist && file.is_file() {
            let (indices, contexts) = get_file_contents(&file, cli.show_all);
            let combined: Vec<(Index, Context)> =
                indices.into_iter().zip(contexts.into_iter()).collect();
            let mut table = Table::new(combined);
            table.with(Style::empty());
            if cli.no_num {
                table.with(Remove::column(Columns::first()));
                table.modify(Rows::first(), Color::FG_BRIGHT_BLACK);
            } else {
                table.modify(Columns::first(), Color::FG_BRIGHT_BLACK);
                table.modify(Rows::first(), Color::FG_BRIGHT_BLACK);
            }
            println!("{}", table);
        } else {
            println!("{}", "error:\nCan't read directory.".red());
        }
    } else {
        println!("{}", "error:\nCan't read file.".red());
    }
}

fn get_file_contents(file: &PathBuf, show_all: bool) -> (Vec<Index>, Vec<Context>) {
    let content = std::fs::read(file).expect("error: can't read the file.");
    let mut index = 0;
    let mut indices = Vec::new();
    let mut contexts = Vec::new();

    let mut line = String::new();
    let mut prev_byte = None;

    for &byte in &content {
        match byte {
            b'\n' => {
                index += 1;
                if show_all {
                    line.push_str("󰌑"); // LF
                }
                contexts.push(Context {
                    content: line.clone(),
                });
                line.clear();
                prev_byte = Some(b'\n');
            }
            b'\r' => {
                if show_all {
                    line.push_str(""); // CR
                } else {
                    line.push('\r');
                }
                prev_byte = Some(b'\r');
            }
            b'\t' => {
                if show_all {
                    line.push(''); // visual tab
                } else {
                    line.push('\t');
                }
            }
            b' ' => {
                if show_all {
                    line.push(''); // visual space
                } else {
                    line.push(' ');
                }
            }
            0x00 => {
                if show_all {
                    line.push_str("󰟢"); // NULL
                } else {
                    line.push('\0');
                }
            }
            0x01..=0x1F => {
                if show_all {
                    line.push_str(&format!("󰨖")); // substitute for control chars
                } else {
                    line.push(byte as char);
                }
            }
            0x7F => {
                if show_all {
                    line.push_str(""); // DEL
                } else {
                    line.push(byte as char);
                }
            }
            _ => {
                line.push(byte as char);
            }
        }
    }

    if !line.is_empty() || prev_byte != Some(b'\n') {
        index += 1;
        contexts.push(Context { content: line });
    }

    for i in 1..=index {
        indices.push(Index { index: i as u64 });
    }

    (indices, contexts)
}
