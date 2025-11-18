use std::{fs::{self, File, OpenOptions}, io::{self, Write as _, BufRead as _, BufReader, BufWriter}, path::Path};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use watchlist::{SiteData, VideoItem, VideoWork, VideoWorkMedium, WatchData, WatchPosition, WatchStatus};

fn main() {
    let config = Config::parse();
    match config.mode {
        Mode::ListAll => {
            match File::open(&config.datafile) {
                Err(e) => eprintln!("error opening datafile {e:?}"),
                Ok(f) => {
                    for video_item in FileVideoItems::new(&f) {
                        println!("{video_item:#?}");
                    }
                },
            }
        },
        Mode::Append => {
            match create_video_item() {
                Err(e) => eprintln!("error creating data {e:?}"),
                Ok(video_item) => match OpenOptions::new().append(true).open(&config.datafile) {
                    Err(e) => eprintln!("error opening datafile {e:?}"),
                    Ok(mut f) => match write_video_item_to_file(&mut f, &video_item) {
                        Ok(()) => (),
                        Err(Error::IO(e)) => eprintln!("error writing to file {e:?}"),
                        Err(Error::RON(e)) => eprintln!("error stringifying work {e:?} {video_item:?}"),
                    }
                }
            }
        },
        Mode::ListDetails { name } => {
            match File::open(&config.datafile) {
                Err(e) => eprintln!("error opening datafile {e:?}"),
                Ok(f) => {
                    for video_item in FileVideoItems::new(&f)
                        .filter(|vi| vi.work.title == name)
                    {
                        println!("{video_item:#?}");
                    }
                },
            }
        },
        Mode::Edit { name } => {
            match File::open(&config.datafile) {
                Err(e) => eprintln!("error opening datafile {e:?}"),
                Ok(f) => {
                    let datafile_path = Path::new(&config.datafile);
                    let tempfile_path = Path::new(&config.tempfile);
                    match edit_video_items_by_name_to_temp_file(
                        FileVideoItems::new(&f),
                        &name,
                        tempfile_path,
                    ) {
                        Ok(true) => println!("encountered an error during temp file creation; will not overwrite data file"),
                        Ok(false) => {
                            match fs::remove_file(datafile_path) {
                                Err(e) => eprintln!("error removing data file {e:?}"),
                                Ok(()) => match fs::rename(tempfile_path, datafile_path) {
                                    Err(e) => eprintln!("error renaming temp file to data file {e:?}"),
                                    Ok(()) => (),
                                }
                            }
                        },
                        Err(e) => eprintln!("error creating temp file {e:?}"),
                    }
                },
            }
        },
        Mode::Remove { name } => {
            match File::open(&config.datafile) {
                Err(e) => eprintln!("error opening datafile {e:?}"),
                Ok(f) => {
                    let datafile_path = Path::new(&config.datafile);
                    let tempfile_path = Path::new(&config.tempfile);
                    match remove_video_items_by_name_to_temp_file(
                        FileVideoItems::new(&f),
                        &name,
                        tempfile_path,
                    ) {
                        Ok(true) => println!("encountered an error during temp file creation; will not overwrite data file"),
                        Ok(false) => {
                            match fs::remove_file(datafile_path) {
                                Err(e) => eprintln!("error removing data file {e:?}"),
                                Ok(()) => match fs::rename(tempfile_path, datafile_path) {
                                    Err(e) => eprintln!("error renaming temp file to data file {e:?}"),
                                    Ok(()) => (),
                                }
                            }
                        },
                        Err(e) => eprintln!("error creating temp file {e:?}"),
                    }
                },
            }
        },
    }
}

#[derive(Parser)]
struct Config {
    #[arg(short, long, default_value = "watchlist.ron")]
    datafile: String,

    #[arg(short, long, default_value = "watchlist.temp.ron")]
    tempfile: String,

    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand)]
enum Mode {
    /// [alias l]
    #[command(alias = "l")]
    ListAll,
    /// [alias d]
    #[command(alias = "d")]
    ListDetails {
        name: String,
    },
    /// [alias a]
    #[command(alias = "a")]
    Append,
    /// [alias e]
    #[command(alias = "e")]
    Edit {
        name: String,
    },
    /// [alias r]
    #[command(alias = "r")]
    Remove {
        name: String,
    }
}

fn edit_video_item(video_item: &mut VideoItem) -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        println!("{video_item:#?}");
        print!("1. video work\n\
                2. site data\n\
                3. watch data\n\
                4. ongoing\n\
                5. updated\n\
                > ");
        stdout.flush()?;
        let mut inp = String::new();
        stdin.read_line(&mut inp)?;
        match inp.trim() {
            "1" => loop {
                println!("{:#?}", video_item.work);
                print!("1. title\n\
                        2. year\n\
                        3. medium\n\
                        > ");
                stdout.flush()?;
                let mut inp = String::new();
                stdin.read_line(&mut inp)?;
                match inp.trim() {
                    "1" => {
                        print!("title: ");
                        stdout.flush()?;
                        let mut title = String::new();
                        stdin.read_line(&mut title)?;
                        if !title.trim().is_empty() {
                            video_item.work.title = title.trim().to_string();
                        }
                    },
                    "2" => loop {
                        print!("year: ");
                        stdout.flush()?;
                        let mut inp = String::new();
                        stdin.read_line(&mut inp)?;
                        if inp.trim().is_empty() {
                            break;
                        }
                        if let Ok(year) = inp.trim().parse::<u16>() {
                            video_item.work.year = year;
                            break;
                        }
                    },
                    "3" => loop {
                        print!("medium (movie, tvshow, anime): ");
                        stdout.flush()?;
                        let mut inp = String::new();
                        stdin.read_line(&mut inp)?;
                        match inp.trim() {
                            "movie" => {
                                video_item.work.medium = VideoWorkMedium::Movie;
                                break;
                            },
                            "tvshow" => {
                                video_item.work.medium = VideoWorkMedium::TvShow;
                                break;
                            },
                            "anime" => {
                                video_item.work.medium = VideoWorkMedium::Anime;    
                                break;
                            },
                            "" => break,
                            _ => continue,
                        }
                    },
                    "" => break,
                    _ => continue,
                }
            },
            "2" => loop {
                println!("{:#?}", video_item.site_data);
                print!("1. tracker\n\
                        2. watch\n\
                        > ");
                stdout.flush()?;
                let mut inp = String::new();
                stdin.read_line(&mut inp)?;
                match inp.trim() {
                    "1" => loop {
                        print!("tracker (true/false)? ");
                        stdout.flush()?;
                        let mut inp = String::new();
                        stdin.read_line(&mut inp)?;
                        match inp.trim() {
                            "true" => {
                                print!("tracker: ");
                                stdout.flush()?;
                                let mut tracker = String::new();
                                stdin.read_line(&mut tracker)?;
                                if !tracker.trim().is_empty() {
                                    video_item.site_data.tracker = Some(tracker.trim().to_string())
                                }
                                break;
                            },
                            "false" => {
                                video_item.site_data.tracker = None;
                                break;
                            },
                            "" => break,
                            _ => continue,
                        }
                    },
                    "2" => loop {
                        print!("watch (true/false)? ");
                        stdout.flush()?;
                        let mut inp = String::new();
                        stdin.read_line(&mut inp)?;
                        match inp.trim() {
                            "true" => {
                                print!("watch: ");
                                stdout.flush()?;
                                let mut watch = String::new();
                                stdin.read_line(&mut watch)?;
                                if !watch.trim().is_empty() {
                                    video_item.site_data.watch = Some(watch.trim().to_string())
                                }
                                break;
                            },
                            "false" => {
                                video_item.site_data.watch = None;
                                break;
                            },
                            "" => break,
                            _ => continue,
                        }
                    },
                    "" => break,
                    _ => continue,
                }
            },
            "3" => loop {
                println!("{:#?}", video_item.watch_data);
                print!("1. status\n\
                        2. position\n\
                        > ");
                stdout.flush()?;
                let mut inp = String::new();
                stdin.read_line(&mut inp)?;
                match inp.trim() {
                    "1" => loop {
                        print!("status (virgin, partial, exhausted): ");
                        stdout.flush()?;
                        let mut inp = String::new();
                        stdin.read_line(&mut inp)?;
                        match inp.trim() {
                            "virgin" => {
                                video_item.watch_data.status = WatchStatus::Virgin;
                                break;
                            },
                            "partial" => {
                                video_item.watch_data.status = WatchStatus::Partial;
                                break;
                            },
                            "exhausted" => {
                                video_item.watch_data.status = WatchStatus::Exhausted;
                                break;
                            },
                            "" => break,
                            _ => continue,
                        }
                    },
                    "2" => loop {
                        print!("position (true/false)? ");
                        stdout.flush()?;
                        let mut inp = String::new();
                        stdin.read_line(&mut inp)?;
                        match inp.trim() {
                            "true" => break {
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
                                video_item.watch_data.position = Some(WatchPosition { season, episode });
                            },
                            "false" => {
                                video_item.watch_data.position = None;
                                break;
                            },
                            "" => break,
                            _ => continue,
                        }
                    },
                    "" => break,
                    _ => continue,
                }
            },
            "4" => loop {
                print!("ongoing (true/false)? ");
                stdout.flush()?;
                let mut inp = String::new();
                stdin.read_line(&mut inp)?;
                match inp.trim() {
                    "true" => {
                        video_item.ongoing = true;
                        break;
                    },
                    "false" => {
                        video_item.ongoing = false;
                        break;
                    },
                    "" => break,
                    _ => continue,
                }
            },
            "5" => loop {
                print!("Enter updated date (yyyy-mm-dd): ");
                stdout.flush()?;
                let mut inp = String::new();
                stdin.read_line(&mut inp)?;
                if inp.trim().is_empty() {
                    break;
                }
                if let Ok(updated) = inp.trim().parse::<NaiveDate>() {
                    video_item.updated = updated;
                    break;
                }
            },
            "" => break,
            _ => continue,
        }
    }
    Ok(())
}

fn create_video_item() -> Result<VideoItem, io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let work: VideoWork = {
        let title: String = loop {
            print!("title: ");
            stdout.flush()?;
            let mut title = String::new();
            stdin.read_line(&mut title)?;
            if title.trim().is_empty() {
                continue;
            }
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
        VideoWork {
            title,
            year,
            medium,
        }
    };
    let site_data: SiteData = {
        SiteData { tracker: None, watch: None }
    };
    let watch_data: WatchData = {
        match work.medium {
            VideoWorkMedium::Movie => {
                let status = loop {
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
                WatchData { status, position: None }
            }
            VideoWorkMedium::TvShow |
            VideoWorkMedium::Anime => {
                let status: WatchStatus = loop {
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
                let position = match status {
                    WatchStatus::Virgin => None,
                    WatchStatus::Partial |
                    WatchStatus::Exhausted => Some({
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
                        WatchPosition { season, episode }
                    })
                };
                WatchData { status, position }
            }
        }
    };
    let ongoing: bool = {
        match work.medium {
            VideoWorkMedium::Movie => false,
            VideoWorkMedium::TvShow |
            VideoWorkMedium::Anime => loop {
                print!("ongoing (true/false)? ");
                stdout.flush()?;
                let mut inp = String::new();
                stdin.read_line(&mut inp)?;
                match inp.trim() {
                    "true" => break true,
                    "false" => break false,
                    _ => continue,
                }
            }
        }
    };
    let updated: NaiveDate = {
        chrono::Local::now().date_naive()
    };
    Ok(VideoItem { work, site_data, watch_data, ongoing, updated })
}

struct FileVideoItems<'a>(BufReader<&'a File>);

impl<'a> Iterator for FileVideoItems<'a> {
    type Item = VideoItem;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut line = String::new();
            match self.0.read_line(&mut line) {
                Err(e) => {
                    eprintln!("error reading line from file {e:?}");
                    continue;
                },
                Ok(0) => return None,
                Ok(_) => (),
            }
            match ron::from_str(&line) {
                Err(e) => {
                    eprintln!("error parsing RON from line \"{line}\": {e:?}");
                    continue;
                },
                Ok(vi) => return Some(vi),
            }
        }
    }
}

impl<'a> FileVideoItems<'a> {
    fn new(f: &'a File) -> Self {
        Self(BufReader::new(f))
    }
}

#[derive(Debug)]
enum Error {
    IO(io::Error),
    RON(ron::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}

impl From<ron::Error> for Error {
    fn from(error: ron::Error) -> Self {
        Self::RON(error)
    }
}

fn write_video_item_to_file<'a>(f: &'a mut File, vi: &VideoItem) -> Result<(), Error> {
    let mut writer = BufWriter::new(f);
    let mut s = ron::to_string(vi)?;
    s.push('\n');
    writer.write_all(s.as_bytes())?;
    Ok(())
}

fn edit_video_items_by_name_to_temp_file(iter: impl Iterator<Item = VideoItem>, name: &str, filepath: &Path) -> Result<bool, io::Error> {
    let mut f = OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(filepath)?;
    let mut encountered_error = false;
    for mut video_item in iter {
        if video_item.work.title == name {
            match edit_video_item(&mut video_item) {
                Ok(()) => (),
                Err(e) => {
                    encountered_error = true;
                    eprintln!("error while editing {e:?}");
                },
            }
        }
        match write_video_item_to_file(&mut f, &video_item) {
            Ok(()) => (),
            Err(Error::IO(e)) => {
                encountered_error = true;
                eprintln!("error writing to file {e:?}");
            },
            Err(Error::RON(e)) => {
                encountered_error = true;
                eprintln!("error stringifying work {e:?} {video_item:?}");
            },
        }
    }
    Ok(encountered_error)
}

fn remove_video_items_by_name_to_temp_file(iter: impl Iterator<Item = VideoItem>, name: &str, filepath: &Path) -> Result<bool, io::Error> {
    let mut f = OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(filepath)?;
    let mut encountered_error = false;
    for video_item in iter {
        match video_item.work.title == name {
            true => println!("removed: {video_item:#?}"),
            false => match write_video_item_to_file(&mut f, &video_item) {
                Ok(()) => (),
                Err(Error::IO(e)) => {
                    encountered_error = true;
                    eprintln!("error writing to file {e:?}");
                },
                Err(Error::RON(e)) => {
                    encountered_error = true;
                    eprintln!("error stringifying work {e:?} {video_item:?}");
                },
            },
        }
    }
    Ok(encountered_error)
}
