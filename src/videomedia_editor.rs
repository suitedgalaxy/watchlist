use std::io::Write as _;

use watchlist::{SiteData, VideoMedia, VideoWork, VideoWorkMedium, WatchData, WatchPosition, WatchStatus};

pub fn prompt_create_video_work() -> Result<VideoWork, std::io::Error> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let title: String = loop {
        print!("title: ");
        stdout.flush()?;
        let mut title = String::new();
        stdin.read_line(&mut title)?;
        break title.trim().to_string();
    };
    // println!("VideoWork {{\n\ttitle: {title}\n}}");
    let year: u16 = loop {
        print!("year: ");
        stdout.flush()?;
        let mut inp = String::new();
        stdin.read_line(&mut inp)?;
        let Ok(year) = inp.trim().parse::<u16>() else { continue; };
        break year;
    };
    // println!("VideoWork {{\n\ttitle: {title}\n\tyear: {year}\n}}");
    let medium: VideoWorkMedium = loop {
        print!("medium (movie, tvshow, anime): ");
        stdout.flush()?;
        let mut inp = String::new();
        stdin.read_line(&mut inp)?;
        match inp.trim() {
            "movie" => break VideoWorkMedium::Movie,
            "tvshow" => break VideoWorkMedium::TvShow,
            "anime" => break VideoWorkMedium::Anime,
            _ => continue,
        }
    };
    // println!("VideoWork {{\n\ttitle: {title}\n\tyear: {year}\n\tmedium: {medium:?}\n}}");
    let video_work = VideoWork {
        title,
        year,
        medium,
    };
    Ok(video_work)
}

pub fn prompt_create_video_media() -> Result<VideoMedia, std::io::Error> {
    let work = prompt_create_video_work()?;
    match work.medium {
        VideoWorkMedium::Movie => prompt_create_movie_video_media(work),
        VideoWorkMedium::TvShow => prompt_create_tvshow_video_media(work),
        VideoWorkMedium::Anime => prompt_create_anime_video_media(work),
    }
}

pub fn prompt_create_movie_video_media(work: VideoWork) -> Result<VideoMedia, std::io::Error> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let watched: WatchStatus = loop {
        print!("watched (true/false)? ");
        stdout.flush()?;
        let mut inp = String::new();
        stdin.read_line(&mut inp)?;
        match inp.trim() {
            "true" => break WatchStatus::Exhausted,
            "false" => break WatchStatus::Virgin,
            _ => continue,
        }
    };
    let updated = chrono::Local::now().date_naive();
    let video_media = VideoMedia {
        work,
        site_data: SiteData { tracker: None, watch: None },
        watch_data: WatchData { status: watched, position: None },
        ongoing: false,
        updated,
    };
    Ok(video_media)
}

pub fn prompt_create_tvshow_video_media(work: VideoWork) -> Result<VideoMedia, std::io::Error> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let watch_status: WatchStatus = loop {
        print!("watch status (virgin, partial, exhausted): ");
        stdout.flush()?;
        let mut inp = String::new();
        stdin.read_line(&mut inp)?;
        match inp.trim() {
            "virgin" => break WatchStatus::Virgin,
            "partial" => break WatchStatus::Partial,
            "exhausted" => break WatchStatus::Exhausted,
            _ => continue,
        }
    };
    let watch_position: Option<WatchPosition> = if let WatchStatus::Partial | WatchStatus::Exhausted = watch_status {
        let season: u16 = loop {
            print!("season: ");
            stdout.flush()?;
            let mut inp = String::new();
            stdin.read_line(&mut inp)?;
            let Ok(season) = inp.trim().parse::<u16>() else { continue; };
            break season;
        };
        let episode: Option<u16> = loop {
            print!("episode (true/false)? ");
            stdout.flush()?;
            let mut inp = String::new();
            stdin.read_line(&mut inp)?;
            match inp.trim() {
                "true" => break Some( loop {
                    print!("episode: ");
                    stdout.flush()?;
                    let mut inp = String::new();
                    stdin.read_line(&mut inp)?;
                    let Ok(episode) = inp.trim().parse::<u16>() else { continue; };
                    break episode;
                }),
                "false" => break None,
                _ => continue,
            }
        };
        let watch_position = WatchPosition {
            season,
            episode,
        };
        Some(watch_position)
    } else {
        None
    };
    let ongoing: bool = loop {
        print!("ongoing (true/false)? ");
        stdout.flush()?;
        let mut inp = String::new();
        stdin.read_line(&mut inp)?;
        match inp.trim() {
            "true" => break true,
            "false" => break false,
            _ => continue,
        }
    };
    let updated = chrono::Local::now().date_naive();
    let video_media = VideoMedia {
        work,
        site_data: SiteData { tracker: None, watch: None },
        watch_data: WatchData { status: watch_status, position: watch_position },
        ongoing,
        updated,
    };
    Ok(video_media)
}

pub fn prompt_create_anime_video_media(work: VideoWork) -> Result<VideoMedia, std::io::Error> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let watch_status: WatchStatus = loop {
        print!("watch status (virgin, partial, exhausted): ");
        stdout.flush()?;
        let mut inp = String::new();
        stdin.read_line(&mut inp)?;
        match inp.trim() {
            "virgin" => break WatchStatus::Virgin,
            "partial" => break WatchStatus::Partial,
            "exhausted" => break WatchStatus::Exhausted,
            _ => continue,
        }
    };
    let watch_position: Option<WatchPosition> = if let WatchStatus::Partial | WatchStatus::Exhausted = watch_status {
        let season: u16 = loop {
            print!("season: ");
            stdout.flush()?;
            let mut inp = String::new();
            stdin.read_line(&mut inp)?;
            let Ok(season) = inp.trim().parse::<u16>() else { continue; };
            break season;
        };
        let episode: Option<u16> = loop {
            print!("episode (true/false)? ");
            stdout.flush()?;
            let mut inp = String::new();
            stdin.read_line(&mut inp)?;
            match inp.trim() {
                "true" => break Some( loop {
                    print!("episode: ");
                    stdout.flush()?;
                    let mut inp = String::new();
                    stdin.read_line(&mut inp)?;
                    let Ok(episode) = inp.trim().parse::<u16>() else { continue; };
                    break episode;
                }),
                "false" => break None,
                _ => continue,
            }
        };
        let watch_position = WatchPosition {
            season,
            episode,
        };
        Some(watch_position)
    } else {
        None
    };
    let ongoing: bool = loop {
        print!("ongoing (true/false)? ");
        stdout.flush()?;
        let mut inp = String::new();
        stdin.read_line(&mut inp)?;
        match inp.trim() {
            "true" => break true,
            "false" => break false,
            _ => continue,
        }
    };
    let updated = chrono::Local::now().date_naive();
    let video_media = VideoMedia {
        work,
        site_data: SiteData { tracker: None, watch: None },
        watch_data: WatchData { status: watch_status, position: watch_position },
        ongoing,
        updated,
    };
    Ok(video_media)
}
