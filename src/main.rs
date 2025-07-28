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
}

fn main() {
    let cli: Cli = Cli::parse();
    let file: PathBuf = cli.file.unwrap();

    if let Ok(is_exist) = fs::exists(&file) {
        if is_exist && file.is_file() {
            let (indices, contexts) = get_file_contents(&file);
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

fn get_file_contents(file: &PathBuf) -> (Vec<Index>, Vec<Context>) {
    let contents = fs::read_to_string(file).expect("error: can't read the file.");
    let mut index = 0;
    let mut indices = Vec::new();
    let mut contexts = Vec::new();

    for line in contents.lines() {
        index += 1;
        indices.push(Index { index });
        contexts.push(Context {
            content: line.to_string(),
        });
    }

    (indices, contexts)
}
