use anyhow::{anyhow, Result};
use clap::error::Error;
use clap::{Arg, ArgMatches, Args, Command, CommandFactory, FromArgMatches, Parser};
use std::collections::VecDeque;
use std::path::PathBuf;

#[derive(Default)]
pub struct InputFiles {
    pub files: Vec<InputFile>,
}

#[derive(Default)]
pub struct InputFile {
    pub file: PathBuf,
    pub name: Option<String>,
    pub language: Option<String>,
    pub timescale: Option<u32>,
    pub framerate: Option<(u32, u32)>,
}

#[derive(Parser)]
struct InputFileArgs {
    /// Input track file name.
    #[clap(
        long = "input-file",
        short = 'i',
        value_name = "FILE",
        required = true,
        multiple_occurrences = true,
        allow_hyphen_values = true,
        help_heading = Some("INPUT FILE(S)"),
        value_parser=clap::value_parser!(PathBuf)
    )]
    file: Vec<PathBuf>,

    /// Media name, e.g. 'Dub, Studio'. [Optional]
    #[clap(
        long = "name",
        short = 'n',
        value_name = "name",
        allow_hyphen_values = true,
        multiple_occurrences = true,
        help_heading = Some("INPUT FILE(S)")
    )]
    name: Vec<String>,

    /// Media language, e.g. 'rus'. [Optional]
    #[clap(
        long="language",
        short = 'l',
        value_name = "language",
        multiple_occurrences = true,
        help_heading = Some("INPUT FILE(S)"),
        validator=media_lang_validator
    )]
    language: Vec<String>,

    /// Timescale. [Optional]
    #[clap(
        long = "timescale",
        short = 't',
        value_name = "timescale",
        multiple_occurrences = true,
        help_heading = Some("INPUT FILE(S)"),
        value_parser=clap::value_parser!(u32)
    )]
    timescale: Vec<u32>,

    /// Set framerate only for video such as 23.97 or 24000/1001. [Optional]
    #[clap(
        long = "framerate",
        short = 'f',
        value_name = "framerate",
        multiple_occurrences = true,
        help_heading = Some("INPUT FILE(S)"),
        value_parser=parse_framerate
    )]
    framerate: Vec<(u32, u32)>,
}

impl Args for InputFiles {
    fn augment_args(cmd: Command<'_>) -> Command<'_> {
        let mut args: Vec<Arg> = vec![];

        for arg in InputFileArgs::command().get_arguments() {
            if !["help", "version"].contains(&arg.get_name()) {
                args.push(arg.clone())
            }
        }

        cmd.args(args)
    }
    fn augment_args_for_update(cmd: Command<'_>) -> Command<'_> {
        InputFiles::augment_args(cmd)
    }
}

impl InputFiles {
    fn get_argument_sequence<T>(
        matches: &mut ArgMatches,
        arg: &str,
    ) -> (VecDeque<usize>, VecDeque<T>)
    where
        T: Clone + Send + Sync + 'static,
    {
        (
            matches.indices_of(arg).unwrap_or_default().collect(),
            matches.remove_many::<T>(arg).unwrap_or_default().collect(),
        )
    }
}

impl FromArgMatches for InputFiles {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let mut matches = matches.clone();
        Self::from_arg_matches_mut(&mut matches)
    }

    fn from_arg_matches_mut(matches: &mut ArgMatches) -> Result<Self, Error> {
        let files = InputFiles::get_argument_sequence::<PathBuf>(matches, "file");
        let mut names = InputFiles::get_argument_sequence::<String>(matches, "name");
        let mut languages = InputFiles::get_argument_sequence::<String>(matches, "language");
        let mut timescales = InputFiles::get_argument_sequence::<u32>(matches, "timescale");
        let mut framerates = InputFiles::get_argument_sequence::<(u32, u32)>(matches, "framerate");

        let mut input_files = InputFiles {
            ..Default::default()
        };

        let mut i = 1;
        for file in files.1 {
            let mut file = InputFile {
                file,
                ..Default::default()
            };

            // get next input file index
            let mut next = usize::MAX;
            if i < files.0.len() {
                next = files.0[i];
            }
            i += 1;

            // set name if provided
            while !names.0.is_empty() {
                let idx = names.0[0];

                if idx < next {
                    file.name = names.1.pop_front();
                    let _ = names.0.pop_front();
                } else {
                    break;
                }
            }

            // set language if provided
            while !languages.0.is_empty() {
                let idx = languages.0[0];

                if idx < next {
                    file.language = languages.1.pop_front();
                    let _ = languages.0.pop_front();
                } else {
                    break;
                }
            }

            // set timescale if provided
            while !timescales.0.is_empty() {
                let idx = timescales.0[0];

                if idx < next {
                    file.timescale = timescales.1.pop_front();
                    let _ = timescales.0.pop_front();
                } else {
                    break;
                }
            }

            // set framerate if provided
            while !framerates.0.is_empty() {
                let idx = framerates.0[0];

                if idx < next {
                    file.framerate = framerates.1.pop_front();
                    let _ = framerates.0.pop_front();
                } else {
                    break;
                }
            }

            input_files.files.push(file);
        }

        Ok(input_files)
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        let mut matches = matches.clone();
        self.update_from_arg_matches_mut(&mut matches)
    }

    fn update_from_arg_matches_mut(&mut self, matches: &mut ArgMatches) -> Result<(), Error> {
        self.files = InputFiles::from_arg_matches_mut(matches)?.files;
        Ok(())
    }
}

fn media_lang_validator(v: &str) -> Result<()> {
    match v.len() {
        3 => Ok(()),
        _ => Err(anyhow!("must be 3 characters long e.g. 'eng'")),
    }
}

fn parse_framerate(
    value: &str,
) -> Result<(u32, u32), Box<dyn std::error::Error + Send + Sync + 'static>> {
    Ok(if let Some((nome, deno)) = value.split_once('/') {
        (nome.parse::<u32>()?, deno.parse::<u32>()?)
    } else {
        ((value.parse::<f64>()? * 1000.0) as u32, 1000)
    })
}
