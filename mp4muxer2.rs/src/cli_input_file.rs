use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Clone, Debug)]
pub struct InputFile {
    /// Input track file name
    #[clap(value_name = "FILE", parse(from_os_str))]
    pub input: PathBuf,

    /// Media language, e.g. 'rus'
    #[clap(long="lang", value_name = "language", validator=media_lang_validator)]
    pub media_lang: Option<String>,

    /// Media name, e.g. 'Dub, Blu-ray'
    #[clap(
        long = "name",
        value_name = "name",
        multiple_values = false,
        use_value_delimiter = false
    )]
    pub media_name: Option<String>,

    /// Timescale
    #[clap(long = "ts", value_name = "timescale")]
    pub media_timescale: Option<u32>,

    /// Set framerate only for video such as 23.97 or 30000/1001
    #[clap(long = "fr", value_name = "framerate", value_parser=parse_framerate)]
    pub input_video_frame_rate: Option<Framerate>,
}

pub fn parse_input_file(
    value: &str,
) -> Result<InputFile, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut args = to_clap_args(value);
    args.insert(0, String::from("input-file"));

    Ok(InputFile::try_parse_from(args)?)
}

/*
        match InputFile::try_parse_from(args) {
            Err(err) => {
                let err = err.to_string();
                let ss: Vec<&str> = err.split('\n').collect();
                bail!("Invalid input file arguments: {}\n{}", f, ss[0])
            }
            Ok(file) => {
                {
                    if let Err(err) = OpenOptions::new().read(true).open(&file.input) {
                        bail!(
                            "Failed to open input file \"{}\": {}",
                            file.input.to_str().unwrap_or("<unknown file>"),
                            err
                        );
                    }
                }

                if let Some(frame_rate) = file.input_video_frame_rate {
                    ema_mp4_mux_set_video_framerate(handle, frame_rate.nome, frame_rate.deno)?;
                }

                ema_mp4_mux_set_input(
                    handle,
                    file.input.into_os_string().into_string().unwrap(),
                    file.media_lang,
                    file.media_name,
                    None,
                    file.media_timescale.unwrap_or(0),
                    0,
                    0,
                )?;
            }
        }
*/

fn to_clap_args(string: &str) -> Vec<String> {
    let mut single_quoted_started = false;
    let mut double_quoted_started = false;

    // split
    let words: Vec<&str> = string
        .split(|c: char| {
            match c {
                '\'' => {
                    if !double_quoted_started {
                        single_quoted_started = !single_quoted_started;
                    }
                }
                '\"' => {
                    if !single_quoted_started {
                        double_quoted_started = !double_quoted_started;
                    }
                }
                ',' => return !single_quoted_started && !double_quoted_started,
                _ => {}
            }
            false
        })
        .collect();

    // add '--'
    let mut args = Vec::with_capacity(words.len());
    args.push(String::from(words[0]));
    if words.len() > 1 {
        for a in words[1..].iter() {
            args.push("--".to_string() + a);
        }
    }

    args
}

fn media_lang_validator(v: &str) -> Result<()> {
    match v.len() {
        3 => Ok(()),
        _ => Err(anyhow!("must be 3 characters long e.g. 'eng'")),
    }
}

#[derive(Clone, Debug)]
pub struct Framerate {
    pub nome: u32,
    pub deno: u32,
}

fn parse_framerate(
    value: &str,
) -> Result<Framerate, Box<dyn std::error::Error + Send + Sync + 'static>> {
    Ok(if let Some((nome, deno)) = value.split_once('/') {
        Framerate {
            nome: nome.parse::<u32>()?,
            deno: deno.parse::<u32>()?,
        }
    } else {
        Framerate {
            nome: (value.parse::<f64>()? * 1000.0) as u32,
            deno: 1000,
        }
    })
}
