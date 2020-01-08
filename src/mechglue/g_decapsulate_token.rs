use ::libc;

pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* To the extent possible under law, Painless Security, LLC has waived
     * all copyright and related or neighboring rights to GSS-API Memory
     * Management Header. This work is published from: United States.
     */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
}

/* _GSSAPIP_GENERIC_H_ */
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_const_OID;
pub use crate::gssapi_h::gss_const_buffer_t;
pub use crate::gssapi_h::gss_int32;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::src::mechglue::g_decapsulate_token::gssapi_alloc_h::gssalloc_malloc;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int32_t;

pub use crate::stdlib::uint32_t;

/*
 * Copyright (c) 2011, PADL Software Pty Ltd.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * 3. Neither the name of PADL Software nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY PADL SOFTWARE AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL PADL SOFTWARE OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
#[no_mangle]

pub unsafe extern "C" fn gss_decapsulate_token(
    mut input_token: crate::gssapi_h::gss_const_buffer_t,
    mut token_oid: crate::gssapi_h::gss_const_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut body_size = 0u32;
    let mut buf_in = 0 as *mut u8;
    if input_token.is_null() || token_oid.is_null() {
        return (1u32) << 24i32;
    }
    if output_token.is_null() {
        return (2u32) << 24i32;
    }
    buf_in = (*input_token).value as *mut u8;
    minor = crate::src::generic::util_token::gssint_g_verify_token_header(
        token_oid,
        &mut body_size,
        &mut buf_in,
        -(1i32),
        (*input_token).length as u32,
        0x1i32,
    ) as crate::gssapi_h::OM_uint32;
    if minor != 0u32 {
        return (9u32) << 16i32;
    }
    (*output_token).value = gssalloc_malloc(body_size as crate::stddef_h::size_t);
    if (*output_token).value.is_null() {
        return (13u32) << 16i32;
    }
    crate::stdlib::memcpy(
        (*output_token).value,
        buf_in as *const libc::c_void,
        body_size as usize,
    );
    (*output_token).length = body_size as crate::stddef_h::size_t;
    return 0u32;
}
