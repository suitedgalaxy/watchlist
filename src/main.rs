use std::io::{BufRead, BufReader, Write};
use clap::{Parser, Subcommand};

mod terminal_editor;

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
                            if let Ok(Ok(w)) = r { Some(w) } else { eprintln!("{r:?}"); None }
                        )
                    {
                        println!("{wm:?}");
                    }
                    
                }
                Err(e) => eprintln!("file error: {e}"),
            }
        },
        CliMode::Append => {
            let mut wm = WatchableMedia::default();
            match std::fs::OpenOptions::new().append(true).open(cli.watchlist_file) {
                Ok(mut f) => if let Ok(()) = terminal_editor::edit_watchable_media(&mut wm) {
                    let _ = f.write("\n".as_bytes());
                    let _ = f.write_all(ron::to_string(&wm).unwrap().as_bytes());
                    println!("added {wm:?}");
                } else {
                    eprintln!("error with prompting");
                }
                Err(e) => eprintln!("file error: {e}"),
            }
        },
        CliMode::ReadWrite => {
            
        },
    }
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

use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchableMedia {
    pub title: String,
    pub year: Option<u16>,
    // todo: genres
    // todo: country
    pub media_type: MediaType,
    pub updated: NaiveDate,
}
impl Default for WatchableMedia {
    fn default() -> Self {
        Self {
            title: Default::default(),
            year: Default::default(),
            media_type: Default::default(),
            updated: chrono::Local::now().date_naive()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaType {
    Movie {
        watched: bool,
    },
    TvShow {
        watch_progress: MultiPartWatchProgress,
        ongoing: bool,
    },
    Anime {
        watch_progress: MultiPartWatchProgress,
        ongoing: bool,
    },
}
impl Default for MediaType {
    fn default() -> Self {
        Self::Movie { watched: false }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MultiPartWatchProgress {
    Virgin,
    Partial(WatchPosition),
    Finished(WatchPosition),
}
impl Default for MultiPartWatchProgress {
    fn default() -> Self {
        Self::Virgin
    }
}

/// represents the last episode/season watched
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WatchPosition {
    pub season: u16,
    pub episode: Option<u16>,
}
