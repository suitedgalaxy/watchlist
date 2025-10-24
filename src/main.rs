use chrono::{Datelike as _, NaiveDate};
use watchlist::{MediaType, MultiPartWatchProgress, WatchPosition, WatchableMedia};
use clap::Parser;

fn main() {
    let _cli = Cli::parse();
    
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
    match ron::to_string(&media) {
        Ok(s) => println!("{s}"),
        Err(_) => eprintln!("error"),
    }
    
}



#[derive(Parser)]
struct Cli {
    watchlist_file: String,
}