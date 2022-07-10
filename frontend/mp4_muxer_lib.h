/************************************************************************************************************
 * Copyright (c) 2017, Dolby Laboratories Inc.
 * All rights reserved.

 * Redistribution and use in source and binary forms, with or without modification, are permitted
 * provided that the following conditions are met:

 * 1. Redistributions of source code must retain the above copyright notice, this list of conditions
 *    and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions
 *    and the following disclaimer in the documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or
 *    promote products derived from this software without specific prior written permission.

 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 ************************************************************************************************************/
/*<
    @file  mp4_muxer_lib.h
    @brief Defines mp4muxer library
*/

unsigned int
ema_mp4_mux_create_clang(void **handle);

void ema_mp4_mux_destroy_clang(void *handle);

unsigned int
ema_mp4_mux_set_video_framerate_clang(void *handle, unsigned int nome, unsigned int deno);

unsigned int
ema_mp4_mux_set_input_clang(void *handle,
                            const char *fn,
                            const char *lang,
                            const char *name,
                            const char *enc_name,
                            unsigned int time_scale,
                            unsigned int chunk_span_size,
                            unsigned int tid);

unsigned int
ema_mp4_mux_set_output_clang(void *handle, int buf_out, const char *fn);

unsigned int
ema_mp4_mux_set_moov_timescale_clang(void *handle, unsigned int timescale);

unsigned int
ema_mp4_mux_set_mbrand_clang(void *handle, const char *mbrand);

unsigned int
ema_mp4_mux_set_cbrand_clang(void *handle, const char *cbrand);

unsigned int
ema_mp4_mux_set_output_format_clang(void *handle, const char *outfm);

unsigned int
ema_mp4_mux_set_max_duration_clang(void *handle, unsigned int max_duration);

unsigned int
ema_mp4_mux_set_dv_profile_clang(void *handle, unsigned int profile);

unsigned int
ema_mp4_mux_set_dv_bl_compatible_id_clang(void *handle, int compatible_id);

unsigned int
ema_mp4_mux_set_sampleentry_dvh1_clang(void *handle, int es_idx);

unsigned int
ema_mp4_mux_set_sampleentry_hvc1_clang(void *handle, int es_idx);

void ema_mp4_mux_consistency_check_clang(void *handle);

unsigned int
ema_mp4_mux_start_clang(void *handle);
