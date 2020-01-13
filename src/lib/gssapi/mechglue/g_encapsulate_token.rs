use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:33"]
pub mod gssapi_h {
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    /* mech_set */
    /* XXXX these are not part of the GSSAPI C bindings!  (but should be) */
    /* XXXX This is a necessary evil until the spec is fixed */
    /*
 * RFC 5587
 */
    #[c2rust::src_loc = "845:1"]
    pub type gss_const_buffer_t = *const gss_buffer_desc;
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:33"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::gss_OID_desc;
    extern "C" {
        /* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
        /*
 * $Id$
 */
        /* * helper macros **/
        /* this code knows that an int on the wire is 32 bits.  The type of
   num should be at least this big, or the extra shifts may do weird
   things */
        /* * malloc wrappers; these may actually do something later */
        /* * helper functions **/
        /* hide names from applications, especially glib applications */
        #[no_mangle]
        #[c2rust::src_loc = "156:1"]
        pub fn gssint_g_make_token_header(mech: *const gss_OID_desc,
                                          body_size: libc::c_uint,
                                          buf: *mut *mut libc::c_uchar,
                                          tok_type: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "154:1"]
        pub fn gssint_g_token_size(mech: *const gss_OID_desc,
                                   body_size: libc::c_uint) -> libc::c_uint;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:33"]
pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* To the extent possible under law, Painless Security, LLC has waived
 * all copyright and related or neighboring rights to GSS-API Memory
 * Management Header. This work is published from: United States.
 */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn gssalloc_malloc(mut size: size_t)
     -> *mut libc::c_void {
        return malloc(size);
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::malloc;
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:33"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::stddef_h::size_t;
pub use self::gssapi_h::{gss_OID, gss_OID_desc_struct, OM_uint32, gss_uint32,
                         gss_buffer_t, gss_buffer_desc_struct, gss_OID_desc,
                         gss_buffer_desc, gss_const_buffer_t, gss_const_OID};
use self::string_h::memcpy;
use self::gssapiP_generic_h::{gssint_g_make_token_header,
                              gssint_g_token_size};
pub use self::gssapi_alloc_h::gssalloc_malloc;
use self::stdlib_h::malloc;
use self::assert_h::__assert_fail;
/* draft-josefsson-gss-capsulate */
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
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn gss_encapsulate_token(mut input_token:
                                                   gss_const_buffer_t,
                                               mut token_oid: gss_const_OID,
                                               mut output_token: gss_buffer_t)
 -> OM_uint32 {
    let mut tokenSize: libc::c_uint = 0; /* TOK_ID */
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if input_token.is_null() || token_oid.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if output_token.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    tokenSize =
        gssint_g_token_size(token_oid, (*input_token).length as libc::c_uint);
    if tokenSize > 2 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"tokenSize > 2\x00" as *const u8 as
                          *const libc::c_char,
                      b"g_encapsulate_token.c\x00" as *const u8 as
                          *const libc::c_char,
                      51 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 81],
                                                &[libc::c_char; 81]>(b"OM_uint32 gss_encapsulate_token(gss_const_buffer_t, gss_const_OID, gss_buffer_t)\x00")).as_ptr());
    }
    tokenSize = tokenSize.wrapping_sub(2 as libc::c_int as libc::c_uint);
    (*output_token).value = gssalloc_malloc(tokenSize as size_t);
    if (*output_token).value.is_null() {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    buf = (*output_token).value as *mut libc::c_uchar;
    gssint_g_make_token_header(token_oid,
                               (*input_token).length as libc::c_uint,
                               &mut buf, -(1 as libc::c_int));
    memcpy(buf as *mut libc::c_void, (*input_token).value,
           (*input_token).length);
    (*output_token).length = tokenSize as size_t;
    return 0 as libc::c_int as OM_uint32;
}
