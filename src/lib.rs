use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchableMedia {
    pub title: Box<str>,
    pub year: Option<u16>,
    // todo: genres
    // todo: country
    pub media_type: MediaType,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum MultiPartWatchProgress {
    Virgin,
    Partial(WatchPosition),
    Finished(WatchPosition),
}

/// represents the last episode watched
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchPosition {
    pub season: u16,
    pub episode: u16,
}
