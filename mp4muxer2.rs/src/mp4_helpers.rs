#[allow(non_camel_case_types)]
pub type ema_mp4_ctrl_handle_t = *mut std::os::raw::c_void;

/**************** Return codes *********************/
/** 0x0: OK */
const EMA_MP4_MUXED_OK: u32 = 0x0; /* successful */

/** 0x1?: input config */
const EMA_MP4_MUXED_PARAM_ERR: u32 = 0x10; /* parameter error */
const EMA_MP4_MUXED_TOO_MANY_ES: u32 = 0x11; /* too many es to mux */
const EMA_MP4_MUXED_NO_ES: u32 = 0x12; /* no es to mux */
const EMA_MP4_MUXED_UNKNOW_ES: u32 = 0x13; /* es unknown */
const EMA_MP4_MUXED_NO_OUTPUT: u32 = 0x14; /* no output */
const EMA_MP4_MUXED_OPEN_FILE_ERR: u32 = 0x15; /* file open err */
const EMA_MP4_MUXED_EOES: u32 = 0x16; /* end of es */
const EMA_MP4_MUXED_IO_ERR: u32 = 0x17; /* I/O err */
const EMA_MP4_MUXED_CLI_ERR: u32 = 0x18; /* CLI err */
const EMA_MP4_MUXED_EMPTY_ES: u32 = 0x19; /* empty es to mux */

/** 0x2?: I/O operation */
const EMA_MP4_MUXED_WRITE_ERR: u32 = 0x20; /* write error */
const EMA_MP4_MUXED_READ_ERR: u32 = 0x21; /* read error */

/** 0x4?: parsing */
const EMA_MP4_MUXED_SYNC_ERR: u32 = 0x40; /* parsing ES error (sync) */
const EMA_MP4_MUXED_ES_ERR: u32 = 0x41; /* parsing ES error */
const EMA_MP4_MUXED_MP4_ERR: u32 = 0x42; /* parsing mp4 file err */
const EMA_MP4_MUXED_NO_CONFIG_ERR: u32 = 0x43; /* no config found before payload starts */
const EMA_MP4_MUXED_MULTI_SD_ERR: u32 = 0x44; /* multiple sample descriptions necessary but deactivated */
const EMA_MP4_MUXED_CONFIG_ERR: u32 = 0x45; /* unallowable config change */
const EMA_MP4_MUXED_NO_SUPPORT: u32 = 0x49; /* not supported syntax/semantics */

/** 0x8?: resource */
const EMA_MP4_MUXED_NO_MEM: u32 = 0x80; /* no memory */

/** 0x10?: bugs */
const EMA_MP4_MUXED_BUGGY: u32 = 0x100; /* unknown bug */

/** 0x11?: exit */
const EMA_MP4_MUXED_EXIT: u32 = 0x110; /* exit by design */

pub fn error_by_code(code: u32) -> String {
    match code {
        EMA_MP4_MUXED_OK => "successful (EMA_MP4_MUXED_OK)".to_string(),
        EMA_MP4_MUXED_PARAM_ERR => "parameter error (EMA_MP4_MUXED_PARAM_ERR)".to_string(),
        EMA_MP4_MUXED_NO_ES => "no es to mux (EMA_MP4_MUXED_NO_ES)".to_string(),
        EMA_MP4_MUXED_TOO_MANY_ES => "too many es to mux (EMA_MP4_MUXED_TOO_MANY_ES)".to_string(),
        EMA_MP4_MUXED_UNKNOW_ES => "es unknown (EMA_MP4_MUXED_UNKNOW_ES)".to_string(),
        EMA_MP4_MUXED_NO_OUTPUT => "no output (EMA_MP4_MUXED_NO_OUTPUT)".to_string(),
        EMA_MP4_MUXED_OPEN_FILE_ERR => "file open err (EMA_MP4_MUXED_OPEN_FILE_ERR)".to_string(),
        EMA_MP4_MUXED_EOES => "end of es (EMA_MP4_MUXED_EOES)".to_string(),
        EMA_MP4_MUXED_IO_ERR => "I/O err (EMA_MP4_MUXED_IO_ERR)".to_string(),
        EMA_MP4_MUXED_CLI_ERR => "CLI err (EMA_MP4_MUXED_CLI_ERR)".to_string(),
        EMA_MP4_MUXED_EMPTY_ES => "empty es to mux (EMA_MP4_MUXED_EMPTY_ES)".to_string(),
        EMA_MP4_MUXED_WRITE_ERR => "write error (EMA_MP4_MUXED_WRITE_ERR)".to_string(),
        EMA_MP4_MUXED_READ_ERR => "read error (EMA_MP4_MUXED_READ_ERR)".to_string(),
        EMA_MP4_MUXED_SYNC_ERR => "parsing ES error (sync) (EMA_MP4_MUXED_SYNC_ERR)".to_string(),
        EMA_MP4_MUXED_ES_ERR => "parsing ES error (EMA_MP4_MUXED_ES_ERR)".to_string(),
        EMA_MP4_MUXED_MP4_ERR => "parsing mp4 file err (EMA_MP4_MUXED_MP4_ERR)".to_string(),
        EMA_MP4_MUXED_NO_CONFIG_ERR => {
            "no config found before payload starts (EMA_MP4_MUXED_NO_CONFIG_ERR)".to_string()
        }
        EMA_MP4_MUXED_MULTI_SD_ERR => {
            "multiple sample descriptions necessary but deactivated (EMA_MP4_MUXED_MULTI_SD_ERR)"
                .to_string()
        }
        EMA_MP4_MUXED_CONFIG_ERR => {
            "unallowable config change (EMA_MP4_MUXED_CONFIG_ERR)".to_string()
        }
        EMA_MP4_MUXED_NO_SUPPORT => {
            "not supported syntax/semantics (EMA_MP4_MUXED_NO_SUPPORT)".to_string()
        }
        EMA_MP4_MUXED_NO_MEM => "no memory (EMA_MP4_MUXED_NO_MEM)".to_string(),
        EMA_MP4_MUXED_BUGGY => "unknown bug (EMA_MP4_MUXED_BUGGY)".to_string(),
        EMA_MP4_MUXED_EXIT => "exit by design (EMA_MP4_MUXED_EXIT)".to_string(),
        _ => format!("unknown error {}", code),
    }
}
