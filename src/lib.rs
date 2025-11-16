#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VideoWork {
    pub title: String,
    pub year: u16,
    pub medium: VideoWorkMedium,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum VideoWorkMedium {
    Movie,
    TvShow,
    Anime,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VideoMedia {
    pub work: VideoWork,

    // changing / opinionated properties
    pub site_data: SiteData,
    pub watch_data: WatchData,
    pub ongoing: bool,
    pub updated: chrono::NaiveDate,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SiteData {
    pub tracker: Option<String>,
    pub watch: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WatchData {
    pub status: WatchStatus,
    pub position: Option<WatchPosition>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum WatchStatus {
    Virgin,
    Partial,
    /// this wording is bad; it means finished for single part media (movies) and
    /// "seen all that has been released" for multi part media, whether it is ongoing or not
    Exhausted,
}

/// the last position that has been watched i.e. should watch the episode after the position
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WatchPosition {
    pub season: u16,
    /// if none, means "watched the season"
    pub episode: Option<u16>,
}
