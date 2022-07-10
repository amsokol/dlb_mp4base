use crate::mp4_muxer_lib::{
    ema_mp4_mux_consistency_check_clang, ema_mp4_mux_set_cbrand_clang,
    ema_mp4_mux_set_dv_bl_compatible_id_clang, ema_mp4_mux_set_dv_profile_clang,
    ema_mp4_mux_set_input_clang, ema_mp4_mux_set_max_duration_clang, ema_mp4_mux_set_mbrand_clang,
    ema_mp4_mux_set_moov_timescale_clang, ema_mp4_mux_set_output_clang,
    ema_mp4_mux_set_output_format_clang, ema_mp4_mux_set_sampleentry_dvh1_clang,
    ema_mp4_mux_set_sampleentry_hvc1_clang, ema_mp4_mux_set_video_framerate_clang,
};
use crate::mp4muxer_types::ema_mp4_ctrl_handle_t;
use anyhow::{bail, Result};
use clap::builder::TypedValueParser;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, AppSettings, Arg, Command, Error,
    ErrorKind, Parser,
};
use std::ffi::CString;
use std::fs::OpenOptions;
use std::os::raw::c_void;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = crate_name!())]
#[clap(author = crate_authors!("\n"))]
#[clap(version = crate_version!())]
#[clap(about = crate_description!(), long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Cli {
    /// Overwrites the existing output .mp4 file if there is one
    #[clap(long)]
    overwrite: bool,

    /// Overrides the timescale of the entire presentation
    #[clap(long, value_name = "arg")]
    mpeg4_timescale: Option<u32>,

    /// Specifies the ISO base media file format brand in the format
    #[clap(long, value_name = "arg")]
    mpeg4_brand: Option<String>,

    /// Specifies the ISO base media file format compatible brand(s)
    /// in the format of a comma separated list, for example mp42,iso6,isom,msdh,dby1
    #[clap(long, value_name = "arg")]
    mpeg4_comp_brand: Option<String>,

    /// Sets the output file format or the specification to which the
    /// output file must conform. Valid values include 'mp4' and 'frag-mp4'.
    /// 'mp4' is the default value
    #[clap(long, value_name = "arg", possible_values = ["mp4", "frag-mp4"], default_value = "mp4")]
    output_format: String,

    /// Sets the maximum fragment duration in milliseconds.
    /// By default, the max duration is 2s
    #[clap(long, value_name = "arg")]
    mpeg4_max_frag_duration: Option<u32>,

    #[clap(
        long,
        value_name = "arg",
        help = "Sets the Dolby Vision profile. This option is MANDATORY for \
        DoVi elementary stream: Valid profile values are:\n\
        4 - dvhe.04, BL codec: HEVC10; EL codec: HEVC10; BL compatibility: SDR/HDR.\n\
        5 - dvhe.05, BL codec: HEVC10; EL codec: N/A;    BL compatibility: None.\n\
        7 - dvhe.07, BL codec: HEVC10; EL codec: HEVC10; BL compatibility: Blu-ray HDR10.\n\
        8 - dvhe.08, BL codec: HEVC10; EL codec: N/A;    BL compatibility: SDR/HDR.\n\
        9 - dvav.09, BL codec: AVC;    EL codec: N/A;    BL compatibility: SDR/HDR.",
        possible_values = ["4", "5", "7", "8", "9"]
    )]
    dv_profile: Option<u8>,

    /// Sets the Dolby Vision base layer compatible ID, if the profile index is 8,
    /// this option must be set by user.
    #[clap(long, value_name = "arg", possible_values = ["1", "2", "4"], required_if_eq("dv-profile", "8"))]
    dv_bl_compatible_id: Option<u8>,

    /// Set the elementary stream index (starting 1) to set HEVC track's sample entry name to 'dvh1',
    /// default sample entry box name is 'dvhe' for non-cross compatible stream.
    #[clap(long, value_name = "stream index", conflicts_with = "hvc1flag")]
    dvh1flag: Option<i32>,

    /// Set the elementary stream index (starting 1) to set HEVC track's sample entry name to 'hvc1',
    /// default sample entry box name is 'hev1' for cross compatible stream"
    #[clap(long, value_name = "stream index")]
    hvc1flag: Option<i32>,

    /// Output .mp4 file name
    #[clap(long, value_name = "FILE", parse(from_os_str))]
    output_file: PathBuf,

    #[clap(
        long,
        value_name = "one or more input files with options",
        multiple_occurrences = true,
        required = true,
        // allow_hyphen_values = true,
        help = "Add elementary streams to MP4 container.\n\
        Comma delimited parameters:\n\
        <file> - file to add (supports H264, H265, AC3, EC3, and AC4). Mandatory.\n\
        lang=<language> - media language, e.g. 'rus'\n\
        name=<name> - media name, e.g. 'Dub, Blu-ray'\n\
        ts=<timescale> - timescale integer value\n\
        fr=<framerate> - set framerate only for video such as 23.97 or 24000/1001\n\
        Example :\n\
        --input-file video.hevc,fr=23.97 --input-file audio.ac3,lang=rus,name=\"Dub, Blu-ray\""
    )]
    input_file: Vec<String>,
}

#[derive(Parser, Debug)]
struct InputFile {
    /// Input track file name
    #[clap(value_name = "FILE", parse(from_os_str))]
    input: PathBuf,

    /// Media language, e.g. 'rus'
    #[clap(long="lang", value_name = "language", validator=media_lang_validator)]
    media_lang: Option<String>,

    /// Media name, e.g. 'Dub, Blu-ray'
    #[clap(
        long = "name",
        value_name = "name",
        multiple_values = false,
        use_value_delimiter = false
    )]
    media_name: Option<String>,

    /// Timescale
    #[clap(long = "ts", value_name = "timescale")]
    media_timescale: Option<u32>,

    /// Set framerate only for video such as 23.97 or 30000/1001
    #[clap(long = "fr", value_name = "framerate", value_parser=FramerateValueParser::new())]
    input_video_frame_rate: Option<Framerate>,
}

#[derive(Clone, Debug)]
struct Framerate {
    nome: u32,
    deno: u32,
}

#[derive(Clone)]
struct FramerateValueParser {}

impl TypedValueParser for FramerateValueParser {
    type Value = Framerate;

    fn parse_ref(
        &self,
        _cmd: &Command,
        _arg: Option<&Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, Error> {
        let value = value
            .to_str()
            .ok_or_else(|| Error::raw(ErrorKind::InvalidUtf8, "invalid framerate value"))?;

        let values: Vec<&str> = value.split('/').collect();

        if values.len() == 2 {
            if let Ok(nome) = values[0].parse::<u32>() {
                if let Ok(deno) = values[1].parse::<u32>() {
                    return Ok(Framerate { nome, deno });
                } else {
                    return Err(Error::raw(
                        ErrorKind::InvalidValue,
                        "invalid deno of framerate value",
                    ));
                }
            } else {
                return Err(Error::raw(
                    ErrorKind::InvalidValue,
                    "invalid nome of framerate value",
                ));
            }
        }

        if let Ok(framerate) = value.parse::<f64>() {
            return Ok(Framerate {
                nome: (framerate * 1000.0) as u32,
                deno: 1000,
            });
        }

        Err(Error::raw(
            ErrorKind::InvalidValue,
            "invalid framerate value",
        ))
    }
}

impl FramerateValueParser {
    /// Parse bool-like string values, everything else is `true`
    pub fn new() -> Self {
        Self {}
    }
}

fn media_lang_validator(v: &str) -> Result<()> {
    if v.len() != 3 {
        bail!("Input lang code '{}' is not correct", v)
    }

    Ok(())
}

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

pub fn parse_cli(handle: *mut c_void) -> Result<()> {
    let cli = Cli::parse();

    // --input-file
    for f in cli.input_file {
        let mut args = to_clap_args(f.as_str());
        args.insert(0, String::from("inputfile"));

        match InputFile::try_parse_from(args) {
            Err(_) => bail!("Invalid input file arguments: {:?}", f),
            Ok(file) => {
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
    }

    /* output file overwrite check */
    /* if no "--overwrite" option, if the output file had been exist, return error and exit.*/
    /* if providing "--overwrite" option, always create output file */
    {
        if OpenOptions::new().read(true).open(&cli.output_file).is_ok() && !cli.overwrite {
            bail!("Output file had been existed, please using '--overwrite' if you want to overwrite it");
        }
    }

    // --output-file
    ema_mp4_mux_set_output(
        handle,
        0,
        cli.output_file.into_os_string().into_string().unwrap(),
    )?;

    // --mpeg4-timescale
    if let Some(ts) = cli.mpeg4_timescale {
        ema_mp4_mux_set_moov_timescale(handle, ts)?;
    }

    // --mpeg4-brand
    if let Some(brand) = cli.mpeg4_brand {
        ema_mp4_mux_set_mbrand(handle, brand)?;
    }

    // --mpeg4-comp-brand
    if let Some(brand) = cli.mpeg4_comp_brand {
        ema_mp4_mux_set_cbrand(handle, brand)?;
    }

    // --output-format
    ema_mp4_mux_set_output_format(handle, cli.output_format)?;

    // --mpeg4-max-frag-duration
    if let Some(duration) = cli.mpeg4_max_frag_duration {
        ema_mp4_mux_set_max_duration(handle, duration)?;
    }

    // --dv-profile
    if let Some(dv_profile) = cli.dv_profile {
        ema_mp4_mux_set_dv_profile(handle, dv_profile)?;
    }

    // --dv-bl-compatible-id
    if let Some(dv_bl_compatible_id) = cli.dv_bl_compatible_id {
        ema_mp4_mux_set_dv_bl_compatible_id(handle, dv_bl_compatible_id)?;
    }

    // --dvh1flag
    if let Some(dvh1flag) = cli.dvh1flag {
        ema_mp4_mux_set_sampleentry_dvh1(handle, dvh1flag)?;
    }

    // --hvc1flag
    if let Some(hvc1flag) = cli.hvc1flag {
        ema_mp4_mux_set_sampleentry_hvc1(handle, hvc1flag)?;
    }

    /* consistency check */
    ema_mp4_mux_consistency_check(handle);

    Ok(())
}

fn ema_mp4_mux_set_video_framerate(
    handle: ema_mp4_ctrl_handle_t,
    nome: u32,
    deno: u32,
) -> Result<()> {
    let res;

    unsafe {
        res = ema_mp4_mux_set_video_framerate_clang(handle, nome, deno);
    }

    if res != 0 {
        bail!("Failed to set video framerate with error code = {}", res);
    }

    Ok(())
}

fn ema_mp4_mux_set_input(
    handle: ema_mp4_ctrl_handle_t,
    filename: String,
    lang: Option<String>,
    name: Option<String>,
    enc_name: Option<String>,
    time_scale: u32,
    chunk_span_size: u32,
    tid: u32,
) -> Result<()> {
    let res;

    let filename = CString::new(filename).unwrap();
    let lang = CString::new(lang.unwrap_or_default()).unwrap();
    let name = CString::new(normalize_media_name(&name.unwrap_or_default())).unwrap();
    let enc_name = CString::new(enc_name.unwrap_or_default()).unwrap();

    unsafe {
        res = ema_mp4_mux_set_input_clang(
            handle,
            filename.as_ptr(),
            lang.as_ptr(),
            name.as_ptr(),
            enc_name.as_ptr(),
            time_scale,
            chunk_span_size,
            tid,
        );
    }

    if res != 0 {
        bail!("Failed to set track input with error code = {}", res);
    }

    Ok(())
}

fn normalize_media_name(name: &str) -> &str {
    if name.is_empty() {
        return name;
    }

    let s = name.trim();
    if s.len() < 2 {
        return s;
    }

    if s.starts_with('\"') && s.ends_with('\"') {
        // trim ".."
        return &s[1..s.len() - 1]; // s1.trim_matches('\"');
    }

    if s.starts_with('\'') && s.ends_with('\'') {
        // trim ".."
        return &s[1..s.len() - 1]; // s1.trim_matches('\"');
    }

    s
}

fn ema_mp4_mux_set_output(
    handle: ema_mp4_ctrl_handle_t,
    buf_out: i32,
    filename: String,
) -> Result<()> {
    let res;

    let filename = CString::new(filename).unwrap();

    unsafe {
        res = ema_mp4_mux_set_output_clang(handle, buf_out, filename.as_ptr());
    }

    if res != 0 {
        bail!("Failed to set output file with error code = {}", res);
    }

    Ok(())
}

fn ema_mp4_mux_set_moov_timescale(handle: ema_mp4_ctrl_handle_t, timescale: u32) -> Result<()> {
    let res;

    unsafe {
        res = ema_mp4_mux_set_moov_timescale_clang(handle, timescale);
    }

    if res != 0 {
        bail!("Failed to set timescale with error code = {}", res);
    }

    Ok(())
}

fn ema_mp4_mux_set_mbrand(handle: ema_mp4_ctrl_handle_t, mbrand: String) -> Result<()> {
    let res;

    let mbrand = CString::new(mbrand).unwrap();

    unsafe {
        res = ema_mp4_mux_set_mbrand_clang(handle, mbrand.as_ptr());
    }

    if res != 0 {
        bail!("Failed to set mpeg4 brand with error code = {}", res);
    }

    Ok(())
}

fn ema_mp4_mux_set_cbrand(handle: ema_mp4_ctrl_handle_t, cbrand: String) -> Result<()> {
    let res;

    let cbrand = CString::new(cbrand).unwrap();

    unsafe {
        res = ema_mp4_mux_set_cbrand_clang(handle, cbrand.as_ptr());
    }

    if res != 0 {
        bail!(
            "Failed to set mpeg4 compatibility brand with error code = {}",
            res
        );
    }

    Ok(())
}

fn ema_mp4_mux_set_output_format(handle: ema_mp4_ctrl_handle_t, outfm: String) -> Result<()> {
    let res;

    let outfm = CString::new(outfm).unwrap();

    unsafe {
        res = ema_mp4_mux_set_output_format_clang(handle, outfm.as_ptr());
    }

    if res != 0 {
        bail!("Failed to set output format with error code = {}", res);
    }

    Ok(())
}

fn ema_mp4_mux_set_max_duration(handle: ema_mp4_ctrl_handle_t, duration: u32) -> Result<()> {
    let res;

    unsafe {
        res = ema_mp4_mux_set_max_duration_clang(handle, duration);
    }

    if res != 0 {
        bail!(
            "Failed to set mpeg4 max fragment duration with error code = {}",
            res
        );
    }

    Ok(())
}

fn ema_mp4_mux_set_dv_profile(handle: ema_mp4_ctrl_handle_t, dv_profile: u8) -> Result<()> {
    let res;

    unsafe {
        res = ema_mp4_mux_set_dv_profile_clang(handle, dv_profile as u32);
    }

    if res != 0 {
        bail!(
            "Failed to set Dolby Vision profile with error code = {}",
            res
        );
    }

    Ok(())
}

fn ema_mp4_mux_set_dv_bl_compatible_id(
    handle: ema_mp4_ctrl_handle_t,
    compatible_id: u8,
) -> Result<()> {
    let res;

    unsafe {
        res = ema_mp4_mux_set_dv_bl_compatible_id_clang(handle, compatible_id as i32);
    }

    if res != 0 {
        bail!(
            "Failed to set Dolby Vision profile compatible ID with error code = {}",
            res
        );
    }

    Ok(())
}

fn ema_mp4_mux_set_sampleentry_dvh1(handle: ema_mp4_ctrl_handle_t, es_idx: i32) -> Result<()> {
    let res;

    unsafe {
        res = ema_mp4_mux_set_sampleentry_dvh1_clang(handle, es_idx);
    }

    if res != 0 {
        bail!("Failed to set dvh1 track ID with error code = {}", res);
    }

    Ok(())
}

fn ema_mp4_mux_set_sampleentry_hvc1(handle: ema_mp4_ctrl_handle_t, es_idx: i32) -> Result<()> {
    let res;

    unsafe {
        res = ema_mp4_mux_set_sampleentry_hvc1_clang(handle, es_idx);
    }

    if res != 0 {
        bail!("Failed to set hvc1 track ID with error code = {}", res);
    }

    Ok(())
}

fn ema_mp4_mux_consistency_check(handle: ema_mp4_ctrl_handle_t) {
    unsafe {
        ema_mp4_mux_consistency_check_clang(handle);
    }
}
