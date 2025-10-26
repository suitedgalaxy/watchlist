use std::io::Write;

use chrono::NaiveDate;
use crate::{MediaType, MultiPartWatchProgress, WatchPosition, WatchableMedia};

pub fn edit_watchable_media(media: &mut WatchableMedia) -> Result<(), std::io::Error> {
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

    fn reshow(media: &WatchableMedia) -> Result<(), std::io::Error> {
        crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0),
        )?;
        println!("{media:#?}");
        println!();
        Ok(())
    }

    fn prompt(prompt_str: &str) -> Result<String, std::io::Error> {
        println!("{prompt_str}");
        std::io::stdout().flush()?;
        let mut inp = String::new();
        std::io::stdin().read_line(&mut inp)?;
        Ok(inp)
    }

    fn prompt_watch_progress(watch_progress: &mut MultiPartWatchProgress) -> Result<(), std::io::Error> {
        loop {
            match prompt("Enter watch progress variant (virgin, partial, finished) or skip: ")?.trim() {
                "" => break,
                "virgin" => *watch_progress = MultiPartWatchProgress::Virgin,
                "partial" => *watch_progress = MultiPartWatchProgress::Partial(Default::default()),
                "finished" => *watch_progress = MultiPartWatchProgress::Partial(Default::default()),
                _ => continue,
            }
            break
        }
        match watch_progress {
            MultiPartWatchProgress::Virgin => (),
            MultiPartWatchProgress::Partial(pos) |
            MultiPartWatchProgress::Finished(pos) => {
                let season = loop {
                    if let Ok(season) = prompt("Enter season: ")?.trim().parse::<u16>() {
                        break season;
                    }
                };
                let episode = loop {
                    let episode_inp = prompt("Enter episode (empty for None): ")?.trim().to_owned();
                    if episode_inp.is_empty() { break None } else if let Ok(episode) = episode_inp.parse::<u16>() {
                        break Some(episode)
                    }
                };
                *pos = WatchPosition { season, episode };
            }
        }
        Ok(())
    }

    loop {
        reshow(&media)?;
        let inp = prompt("(0) finish (1) edit title (2) edit year (3) edit media type (4) edit updated")?;
        match inp.trim() {
            "0" => break,
            "1" => media.title = prompt("Enter title: ")?.trim().to_owned(),
            "2" => loop {
                let year_inp = prompt("Enter year (empty for None): ")?.trim().to_owned();
                media.year = if year_inp.is_empty() { None } else {
                    if let Ok(year) = year_inp.parse::<u16>() {
                        Some(year)
                    } else {
                        reshow(&media)?;
                        continue
                    }
                };
                break
            },
            "3" => {
                loop {
                    match prompt("Enter variant (movie, tvshow, anime) or skip: ")?.trim() {
                        "" => break,
                        "movie" => media.media_type = MediaType::Movie { watched: false },
                        "tvshow" => media.media_type = MediaType::TvShow { watch_progress: Default::default(), ongoing: false },
                        "anime" => media.media_type = MediaType::Anime { watch_progress: Default::default(), ongoing: false },
                        _ => {
                            reshow(&media)?;
                            continue
                        }
                    }
                    break
                }
                reshow(&media)?;
                match &mut media.media_type {
                    MediaType::Movie { watched } => loop {
                        match prompt("(y/n) watched: ")?.trim() {
                            "y" => *watched = true,
                            "n" => *watched = false,
                            _ => continue,
                        }
                        break
                    },
                    MediaType::TvShow { watch_progress, ongoing } => {
                        loop {
                            match prompt("(y/n) ongoing: ")?.trim() {
                                "y" => *ongoing = true,
                                "n" => *ongoing = false,
                                _ => continue,
                            }
                            break
                        }
                        prompt_watch_progress(watch_progress)?;
                    },
                    MediaType::Anime { watch_progress, ongoing } => {
                        loop {
                            match prompt("(y/n) ongoing: ")?.trim() {
                                "y" => *ongoing = true,
                                "n" => *ongoing = false,
                                _ => continue,
                            }
                            break
                        }
                        prompt_watch_progress(watch_progress)?;
                    },

                }
            },
            "4" => loop {
                let updated_inp = prompt("Enter updated date (yyyy-mm-dd): ")?.trim().to_owned();
                media.updated = if let Ok(date) = updated_inp.trim().parse::<NaiveDate>() { date } else {
                    reshow(&media)?;
                    continue
                }
            },
            _ => continue,
        }
    }

    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
