mod cli;
mod cli_input_file;
mod mp4;
mod mp4_helpers;

use crate::mp4_helpers::{ema_mp4_ctrl_handle_t, error_by_code};
use anyhow::{bail, Result};
use cli::parse_cli;
use std::ptr::null_mut;

fn main() -> Result<()> {
    let mut ema_handle: ema_mp4_ctrl_handle_t = null_mut();

    /**** create muxer handle */
    ema_mp4_mux_create(&mut ema_handle)?;

    let mut err = parse_cli(ema_handle);

    if err.is_ok() {
        err = ema_mp4_mux_start(ema_handle);
    }

    /**** clean up. parser and mux already done and their resource released */
    ema_mp4_mux_destroy(ema_handle);

    err
}

fn ema_mp4_mux_create(handle: &mut ema_mp4_ctrl_handle_t) -> Result<()> {
    let res;

    unsafe {
        res = mp4::ema_mp4_mux_create_clang(handle);
    }

    if res != 0 {
        bail!(
            "Failed to create MP4 muxer with error: {}",
            error_by_code(res)
        );
    }

    Ok(())
}

fn ema_mp4_mux_start(handle: ema_mp4_ctrl_handle_t) -> Result<()> {
    let res;

    unsafe {
        res = mp4::ema_mp4_mux_start_clang(handle);
    }

    if res != 0 {
        bail!("Muxing failed with error: {}", error_by_code(res));
    }

    Ok(())
}

fn ema_mp4_mux_destroy(handle: ema_mp4_ctrl_handle_t) {
    unsafe {
        mp4::ema_mp4_mux_destroy_clang(handle);
    }
}
