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
    @file  mp4_muxer_lib.c
    @brief Implements mp4muxer library
*/

#include "ema_mp4_ifc.h"
#include "mp4_muxer.h"

unsigned int
ema_mp4_mux_create_clang(void **handle)
{
    return (unsigned int)ema_mp4_mux_create((ema_mp4_ctrl_handle_t *)handle);
}

void ema_mp4_mux_destroy_clang(void *handle)
{
    ema_mp4_mux_destroy((ema_mp4_ctrl_handle_t)handle);
}

unsigned int
ema_mp4_mux_set_video_framerate_clang(void *handle, unsigned int nome, unsigned int deno)
{
    return ema_mp4_mux_set_video_framerate((ema_mp4_ctrl_handle_t)handle, (uint32_t)nome, (uint32_t)deno);
}

unsigned int
ema_mp4_mux_set_input_clang(void *handle,
                            const char *fn,
                            const char *lang,
                            const char *name,
                            const char *enc_name,
                            unsigned int time_scale,
                            unsigned int chunk_span_size,
                            unsigned int tid)
{
    return ema_mp4_mux_set_input((ema_mp4_ctrl_handle_t)handle,
                                 (int8_t *)(!fn || fn[0] != '\0' ? fn : 0),
                                 (int8_t *)(!lang || lang[0] != '\0' ? lang : 0),
                                 (int8_t *)(!name || name[0] != '\0' ? name : 0),
                                 (int8_t *)(!enc_name || enc_name[0] != '\0' ? enc_name : 0),
                                 (uint32_t)time_scale,
                                 (uint32_t)chunk_span_size,
                                 (uint32_t)tid);
}

unsigned int
ema_mp4_mux_set_output_clang(void *handle, int buf_out, const char *fn)
{
    return ema_mp4_mux_set_output((ema_mp4_ctrl_handle_t)handle, (int32_t)buf_out, (const int8_t *)(!fn || fn[0] != '\0' ? fn : 0));
}

unsigned int
ema_mp4_mux_set_moov_timescale_clang(void *handle, unsigned int timescale)
{
    return ema_mp4_mux_set_moov_timescale((ema_mp4_ctrl_handle_t)handle, (uint32_t)timescale);
}

unsigned int
ema_mp4_mux_set_mbrand_clang(void *handle, const char *mbrand)
{
    return ema_mp4_mux_set_mbrand((ema_mp4_ctrl_handle_t)handle, (const int8_t *)(!mbrand || mbrand[0] != '\0' ? mbrand : 0));
}

unsigned int
ema_mp4_mux_set_cbrand_clang(void *handle, const char *cbrand)
{
    return ema_mp4_mux_set_cbrand((ema_mp4_ctrl_handle_t)handle, (const int8_t *)(!cbrand || cbrand[0] != '\0' ? cbrand : 0));
}

unsigned int
ema_mp4_mux_set_output_format_clang(void *handle, const char *outfm)
{
    return ema_mp4_mux_set_output_format((ema_mp4_ctrl_handle_t)handle, (const int8_t *)(!outfm || outfm[0] != '\0' ? outfm : 0));
}

unsigned int
ema_mp4_mux_set_max_duration_clang(void *handle, unsigned int max_duration)
{
    return ema_mp4_mux_set_max_duration((ema_mp4_ctrl_handle_t)handle, (uint32_t)max_duration);
}

unsigned int
ema_mp4_mux_set_dv_profile_clang(void *handle, unsigned int profile)
{
    return ema_mp4_mux_set_dv_profile((ema_mp4_ctrl_handle_t)handle, (int32_t)profile);
}

unsigned int
ema_mp4_mux_set_dv_bl_compatible_id_clang(void *handle, int compatible_id)
{
    return ema_mp4_mux_set_dv_bl_compatible_id((ema_mp4_ctrl_handle_t)handle, (int32_t)compatible_id);
}

unsigned int
ema_mp4_mux_set_sampleentry_dvh1_clang(void *handle, int es_idx)
{
    return ema_mp4_mux_set_sampleentry_dvh1((ema_mp4_ctrl_handle_t)handle, (int32_t)es_idx);
}

unsigned int
ema_mp4_mux_set_sampleentry_hvc1_clang(void *handle, int es_idx)
{
    return ema_mp4_mux_set_sampleentry_hvc1((ema_mp4_ctrl_handle_t)handle, (int32_t)es_idx);
}

void ema_mp4_mux_consistency_check_clang(void *handle)
{
    ema_mp4_ctrl_handle_t ema_handle = (ema_mp4_ctrl_handle_t)handle;

    if ((ema_handle->usr_cfg_mux.output_mode & EMA_MP4_FRAG) || !ema_handle->usr_cfg_mux.chunk_span_time)
    {
        int32_t ua;
        /** no interleave by size */
        for (ua = 0; ua < ema_handle->usr_cfg_mux.es_num; ua++)
        {
            if (ema_handle->usr_cfg_ess[ua].chunk_span_size)
            {
                ema_handle->usr_cfg_ess[ua].chunk_span_size = 0;
            }
        }

        /** no interleave by time */
        if (ema_handle->usr_cfg_mux.output_mode & EMA_MP4_FRAG)
        {
            ema_handle->usr_cfg_mux.chunk_span_time = 0;
            /** sp chunk op basically just take care of sample description and dref */
        }
    }
}

unsigned int
ema_mp4_mux_start_clang(void *handle)
{
    return ema_mp4_mux_start((ema_mp4_ctrl_handle_t)handle);
}
