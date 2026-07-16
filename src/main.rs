use clap::{Parser, Subcommand};

use rq::{count_stats, grep_lines, read_file_content};
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Grep {
            pattern,
            path,
            ignore_case,
        } => {
            let file_content = read_file_content(&path);
            let lower_case_pattern = pattern.to_lowercase();
            let lower_case_content = file_content.to_lowercase();
            let results = if ignore_case {
                grep_lines(&lower_case_pattern, &lower_case_content)
            } else {
                grep_lines(&pattern, &file_content)
            };
            for line in results {
                println!("{}", line);
            }
        }
        Commands::Count { path } => {
            let file_content = read_file_content(&path);
            let (lines, words, chars) = count_stats(&file_content);
            println!("lines: {}  words: {}  chars: {}", lines, words, chars);
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Grep {
        pattern: String,
        path: String,
        #[arg(long)]
        ignore_case: bool,
    },
    Count {
        path: String,
    },
}
