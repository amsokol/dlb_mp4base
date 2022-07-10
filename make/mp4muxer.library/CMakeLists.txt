cmake_minimum_required(VERSION 3.23)

project(mp4muxer)

add_library(mp4muxer
    ../../frontend/mp4_muxer_lib.c
    ../../frontend/ema_mp4_mux_api.c
    ../../src/mp4_isom.c
    ../../src/mp4_muxer.c
    ../../src/mp4_stream.c
    ../../src/esparser/dsi.c
    ../../src/esparser/parser_aac.c
    ../../src/esparser/parser_ac4.c
    ../../src/esparser/parser_avc_dec.c
    ../../src/esparser/parser_avc_dpb.c
    ../../src/esparser/parser_avc.c
    ../../src/esparser/parser_dd.c
    ../../src/esparser/parser_hevc_dec.c
    ../../src/esparser/parser_hevc.c
    ../../src/esparser/parser.c
    ../../src/util/io_base.c
    ../../src/util/io_buffer.c
    ../../src/util/io_file.c
    ../../src/util/list_itr.c
    ../../src/util/msg_log.c
    ../../src/util/registry.c
    ../../src/util/utils.c
)

target_include_directories(mp4muxer PRIVATE ../../include)
