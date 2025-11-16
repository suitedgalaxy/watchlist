use std::{fs, io::Write as _};
use clap::Parser;
use watchlist::VideoMedia;

mod videomedia_editor;
mod clap_config;
use clap_config::Mode;

fn main() {
    let config = clap_config::Config::parse();

    match config.mode {
        Mode::ListAll => {
            match fs::File::open(config.datafile) {
                Err(_) => eprintln!("error opening datafile"),
                Ok(f) => for res in iter_parse_file(&f) {
                    match res {
                        Err(_) => eprintln!("error reading line from datafile"),
                        Ok(Err(_)) => eprintln!("error parsing line from datafile"),
                        Ok(Ok(videomedia)) => {
                            println!("{videomedia:#?}")
                        }
                    }
                },
            }
        },
        Mode::Append => {
            match videomedia_editor::prompt_create_video_media() {
                Err(_) => eprintln!("error creating data"),
                Ok(videomedia) => match ron::to_string(&videomedia) {
                    Err(_) => eprintln!("error stringifying data"),
                    Ok(s) => {
                        match fs::OpenOptions::new().append(true).open(config.datafile) {
                            Err(_) => eprintln!("error opening datafile"),
                            Ok(mut f) => match f.write_all(format!("{s}\n").as_bytes()) {
                                Err(_) => eprintln!("error writing to datafile"),
                                Ok(()) => (),
                            }
                        }
                    }
                }
            }
        },
        Mode::ListDetails { name } => {
            match fs::File::open(config.datafile) {
                Err(_) => eprintln!("error opening datafile"),
                Ok(f) => {
                    let mut name_matches = Vec::new();
                    for res in iter_parse_file(&f) {
                        match res {
                            Err(_) => eprintln!("error reading line from datafile"),
                            Ok(Err(_)) => eprintln!("error parsing line from datafile"),
                            Ok(Ok(videomedia)) => {
                                if videomedia.work.title == name {
                                    name_matches.push(videomedia);
                                }
                            }
                        }
                    }
                    if name_matches.len() == 0 {
                        println!("no matches")
                    }
                    for videomedia in name_matches {
                        println!("{videomedia:#?}")
                    }
                },
            }
        },
        Mode::Edit { name: _ } => {
            todo!()
        },
        Mode::Remove { name: _ } => {
            todo!()
        },
    }
}

fn iter_parse_file(f: &fs::File) -> impl Iterator<Item = std::io::Result<ron::error::SpannedResult<VideoMedia>>> {
    use std::io::{BufReader, BufRead as _};
    BufReader::new(f)
        .lines()
        .map(|line_res|
            line_res.map(|line|
                ron::from_str::<VideoMedia>(&line)
            )
        )
}