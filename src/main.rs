use std::io::{BufRead, BufReader, Write};

use chrono::NaiveDate;
use watchlist::{MediaType, MultiPartWatchProgress, WatchPosition, WatchableMedia};
use clap::{Parser, Subcommand};

fn main() {
    let cli = Cli::parse();
    match cli.mode {
        CliMode::Read => {
            match std::fs::OpenOptions::new().read(true).open(cli.watchlist_file) {
                Ok(f) => {
                    for wm in BufReader::new(f)
                        .lines()
                        .map(|r|
                            r.map(|l|
                                ron::from_str::<WatchableMedia>(&l)
                            )
                        )
                        .filter_map(|r|
                            if let Ok(Ok(w)) = r { Some(w) } else { None }
                        )
                    {
                        println!("{wm:?}");
                    }
                    
                }
                Err(e) => eprintln!("file error: {e}"),
            }
        },
        CliMode::Append => {
            match std::fs::OpenOptions::new().append(true).open(cli.watchlist_file) {
                Ok(mut f) => if let Some(wm) = prompt_create_wm() {
                    let _ = f.write("\n".as_bytes());
                    let _ = f.write_all(ron::to_string(&wm).unwrap().as_bytes());
                }
                Err(e) => eprintln!("file error: {e}"),
            }
        },
        CliMode::ReadWrite => {},
    }
    // let media = WatchableMedia {
    //     title: "Chainsaw Man".to_owned().into_boxed_str(),
    //     year: Some(2022),
    //     media_type: MediaType::Anime {
    //         watch_progress: MultiPartWatchProgress::Finished(WatchPosition {
    //             season: 1,
    //             episode: 12,
    //         }),
    //         ongoing: true,
    //     },
    //     updated: NaiveDate::from_ymd_opt(2025, 10, 24).unwrap(),
    // };
    // match ron::to_string(&media) {
    //     Ok(s) => println!("{s}"),
    //     Err(_) => eprintln!("error"),
    // }
    
}

fn prompt_create_wm() -> Option<WatchableMedia> {
    let media = WatchableMedia {
        title: "Chainsaw Man".to_owned().into_boxed_str(),
        year: Some(2022),
        media_type: MediaType::Anime {
            watch_progress: MultiPartWatchProgress::Finished(WatchPosition {
                season: 1,
                episode: 12,
            }),
            ongoing: true,
        },
        updated: NaiveDate::from_ymd_opt(2025, 10, 24).unwrap(),
    };
    Some(media)
}

#[derive(Parser)]
struct Cli {
    watchlist_file: String,
    #[command(subcommand)]
    mode: CliMode,
}

#[derive(Subcommand)]
enum CliMode {
    /// read watchlist and display [alias: r]
    #[command(alias = "r")]
    Read,
    /// prompt to append a list item [alias: a]
    #[command(alias = "a")]
    Append,
    /// read watchlist, output, and prompt to edit [alias: rw]
    #[command(alias = "rw")]
    ReadWrite,
}