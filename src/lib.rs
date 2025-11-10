#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WatchableMedia {
    // unchanging properties
    pub title: String,
    pub year: u16,
    pub media_type: MediaType,

    // changing / opinionated properties
    pub tracker_site: Option<String>,
    pub watch_site: Option<String>,
    pub watch_position: Option<WatchPosition>,
    pub watch_status: WatchStatus,
    pub ongoing: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum MediaType {
    Movie,
    TvShow,
    Anime,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum WatchStatus {
    Virgin,
    Partial,
    /// this wording is bad; it means finished for single part media (movies) and
    /// "seen all that has been released" for multi part media, whether it is ongoing or not
    Exhausted,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WatchPosition {
    pub season: u16,
    /// if none, means "watched the season"
    pub episode: Option<u16>,
}
