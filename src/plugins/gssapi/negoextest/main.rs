use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:33"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "657:5"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "652:1"]
    pub unsafe extern "C" fn store_32_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = val;
    }
    use super::stdint_uintn_h::uint32_t;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:34"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "122:16"]
    pub struct gss_channel_bindings_struct {
        pub initiator_addrtype: OM_uint32,
        pub initiator_address: gss_buffer_desc,
        pub acceptor_addrtype: OM_uint32,
        pub acceptor_address: gss_buffer_desc,
        pub application_data: gss_buffer_desc,
    }
    #[c2rust::src_loc = "122:1"]
    pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
    /* mech_set */
    /* XXXX these are not part of the GSSAPI C bindings!  (but should be) */
    /* XXXX This is a necessary evil until the spec is fixed */
    /*
 * RFC 5587
 */
    #[c2rust::src_loc = "845:1"]
    pub type gss_const_buffer_t = *const gss_buffer_desc;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        /* This is the gssapi.h prologue. */
/* no xom.h */
/* End of gssapi.h prologue. */
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
 * Determine platform-dependent configuration.
 */
        /* __cplusplus */
        /*
 * First, include stddef.h to get size_t defined.
 */
        /*
 * POSIX says that sys/types.h is where size_t is defined.
 */
        /*
 * $Id$
 */
        /*
 * First, define the three platform-dependent pointer types.
 */
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:35"]
pub mod gssapi_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:16"]
    pub struct gss_buffer_set_desc_struct {
        pub count: size_t,
        pub elements: *mut gss_buffer_desc,
    }
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
    use super::stddef_h::size_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32, gss_buffer_desc_struct,
                          gss_buffer_t, gss_OID_desc_struct, gss_OID,
                          gss_const_buffer_t, gss_OID_desc, gss_const_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn gss_add_buffer_set_member(_: *mut OM_uint32, _: gss_buffer_t,
                                         _: *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "240:27"]
        pub static mut GSS_C_INQ_NEGOEX_KEY: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "241:27"]
        pub static mut GSS_C_INQ_NEGOEX_VERIFY_KEY: gss_OID;
        /* draft-josefsson-gss-capsulate */
        #[no_mangle]
        #[c2rust::src_loc = "508:1"]
        pub fn gss_encapsulate_token(_: gss_const_buffer_t, _: gss_const_OID,
                                     _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn gss_oid_equal(_: gss_const_OID, _: gss_const_OID)
         -> libc::c_int;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "634:1"]
        pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:36"]
pub mod gssapi_alloc_h {
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn gssalloc_malloc(mut size: size_t)
     -> *mut libc::c_void {
        return malloc(size);
    }
    /* not _WIN32 or DEBUG_GSSALLOC */
    #[inline]
    #[c2rust::src_loc = "119:1"]
    pub unsafe extern "C" fn gssalloc_strdup(mut str: *const libc::c_char)
     -> *mut libc::c_char {
        let mut size: size_t =
            strlen(str).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut copy: *mut libc::c_char =
            gssalloc_malloc(size) as *mut libc::c_char;
        if !copy.is_null() {
            memcpy(copy as *mut libc::c_void, str as *const libc::c_void,
                   size);
            *copy.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                             as isize) = '\u{0}' as i32 as libc::c_char
        }
        return copy;
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::malloc;
    use super::string_h::{strlen, memcpy};
}
pub use self::types_h::{__uint8_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed, store_32_le};
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_cred_usage_t,
                         gss_const_OID, gss_const_buffer_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_t,
                             gss_add_buffer_set_member, GSS_C_INQ_NEGOEX_KEY,
                             GSS_C_INQ_NEGOEX_VERIFY_KEY,
                             gss_encapsulate_token, gss_oid_equal};
use self::stdlib_h::{malloc, free, getenv, atoi};
use self::string_h::{strlen, strcmp, memcmp, memset, memcpy};
use self::assert_h::__assert_fail;
pub use self::gssapi_alloc_h::{gssalloc_malloc, gssalloc_strdup};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/gssapi/negoextest/main.c - GSS test module for NegoEx */
/*
 * Copyright (C) 2019 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "38:8"]
pub struct test_context {
    pub initiator: libc::c_int,
    pub hops: uint8_t,
}
/* cred_handle */
/* hops remaining; 0 means established */
#[no_mangle]
#[c2rust::src_loc = "43:1"]
pub unsafe extern "C" fn gss_init_sec_context(mut minor_status:
                                                  *mut OM_uint32,
                                              mut claimant_cred_handle:
                                                  gss_cred_id_t,
                                              mut context_handle:
                                                  *mut gss_ctx_id_t,
                                              mut target_name: gss_name_t,
                                              mut mech_type: gss_OID,
                                              mut req_flags: OM_uint32,
                                              mut time_req: OM_uint32,
                                              mut input_chan_bindings:
                                                  gss_channel_bindings_t,
                                              mut input_token: gss_buffer_t,
                                              mut actual_mech: *mut gss_OID,
                                              mut output_token: gss_buffer_t,
                                              mut ret_flags: *mut OM_uint32,
                                              mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    let mut ctx: *mut test_context = *context_handle as *mut test_context;
    let mut major: OM_uint32 = 0;
    let mut tok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut envstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut hops: uint8_t = 0;
    let mut mech_last_octet: uint8_t = 0;
    if input_token.is_null() ||
           (*input_token).length == 0 as libc::c_int as libc::c_ulong {
        envstr = getenv(b"HOPS\x00" as *const u8 as *const libc::c_char);
        hops =
            if !envstr.is_null() { atoi(envstr) } else { 1 as libc::c_int } as
                uint8_t;
        if hops as libc::c_int > 0 as libc::c_int {
        } else {
            __assert_fail(b"hops > 0\x00" as *const u8 as *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          63 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 207],
                                                    &[libc::c_char; 207]>(b"OM_uint32 gss_init_sec_context(OM_uint32 *, gss_cred_id_t, gss_ctx_id_t *, gss_name_t, gss_OID, OM_uint32, OM_uint32, gss_channel_bindings_t, gss_buffer_t, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *)\x00")).as_ptr());
        }
    } else if (*input_token).length == 4 as libc::c_int as libc::c_ulong &&
                  memcmp((*input_token).value,
                         b"fail\x00" as *const u8 as *const libc::c_char as
                             *const libc::c_void,
                         4 as libc::c_int as libc::c_ulong) ==
                      0 as libc::c_int {
        *minor_status = 12345 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    } else {
        hops =
            *((*input_token).value as
                  *mut uint8_t).offset(0 as libc::c_int as isize)
    }
    mech_last_octet =
        *((*mech_type).elements as
              *mut uint8_t).offset((*mech_type).length.wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                       as isize);
    envstr = getenv(b"INIT_FAIL\x00" as *const u8 as *const libc::c_char);
    if !envstr.is_null() && atoi(envstr) == mech_last_octet as libc::c_int {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if ctx.is_null() {
        ctx =
            malloc(::std::mem::size_of::<test_context>() as libc::c_ulong) as
                *mut test_context;
        if !ctx.is_null() {
        } else {
            __assert_fail(b"ctx != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          79 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 207],
                                                    &[libc::c_char; 207]>(b"OM_uint32 gss_init_sec_context(OM_uint32 *, gss_cred_id_t, gss_ctx_id_t *, gss_name_t, gss_OID, OM_uint32, OM_uint32, gss_channel_bindings_t, gss_buffer_t, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *)\x00")).as_ptr());
        }
        (*ctx).initiator = 1 as libc::c_int;
        (*ctx).hops = hops;
        *context_handle = ctx as gss_ctx_id_t
    } else if !ctx.is_null() {
        if (*ctx).initiator != 0 {
        } else {
            __assert_fail(b"ctx->initiator\x00" as *const u8 as
                              *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          84 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 207],
                                                    &[libc::c_char; 207]>(b"OM_uint32 gss_init_sec_context(OM_uint32 *, gss_cred_id_t, gss_ctx_id_t *, gss_name_t, gss_OID, OM_uint32, OM_uint32, gss_channel_bindings_t, gss_buffer_t, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *)\x00")).as_ptr());
        }
        (*ctx).hops = (*ctx).hops.wrapping_sub(1);
        if (*ctx).hops as libc::c_int == hops as libc::c_int {
        } else {
            __assert_fail(b"ctx->hops == hops\x00" as *const u8 as
                              *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          86 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 207],
                                                    &[libc::c_char; 207]>(b"OM_uint32 gss_init_sec_context(OM_uint32 *, gss_cred_id_t, gss_ctx_id_t *, gss_name_t, gss_OID, OM_uint32, OM_uint32, gss_channel_bindings_t, gss_buffer_t, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *)\x00")).as_ptr());
        }
    }
    if (*ctx).hops as libc::c_int > 0 as libc::c_int {
        /* Generate a token containing the remaining hop count. */
        (*ctx).hops = (*ctx).hops.wrapping_sub(1);
        tok.value = &mut (*ctx).hops as *mut uint8_t as *mut libc::c_void;
        tok.length = 1 as libc::c_int as size_t;
        major =
            gss_encapsulate_token(&mut tok as *mut gss_buffer_desc as
                                      gss_const_buffer_t,
                                  mech_type as gss_const_OID, output_token);
        if major == 0 as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"major == GSS_S_COMPLETE\x00" as *const u8 as
                              *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          95 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 207],
                                                    &[libc::c_char; 207]>(b"OM_uint32 gss_init_sec_context(OM_uint32 *, gss_cred_id_t, gss_ctx_id_t *, gss_name_t, gss_OID, OM_uint32, OM_uint32, gss_channel_bindings_t, gss_buffer_t, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *)\x00")).as_ptr());
        }
    }
    return if (*ctx).hops as libc::c_int > 0 as libc::c_int {
               ((1 as libc::c_int)) << 0 as libc::c_int + 0 as libc::c_int
           } else { 0 as libc::c_int } as OM_uint32;
}
/* time_rec */
#[no_mangle]
#[c2rust::src_loc = "101:1"]
pub unsafe extern "C" fn gss_accept_sec_context(mut minor_status:
                                                    *mut OM_uint32,
                                                mut context_handle:
                                                    *mut gss_ctx_id_t,
                                                mut verifier_cred_handle:
                                                    gss_cred_id_t,
                                                mut input_token: gss_buffer_t,
                                                mut input_chan_bindings:
                                                    gss_channel_bindings_t,
                                                mut src_name: *mut gss_name_t,
                                                mut mech_type: *mut gss_OID,
                                                mut output_token:
                                                    gss_buffer_t,
                                                mut ret_flags: *mut OM_uint32,
                                                mut time_rec: *mut OM_uint32,
                                                mut delegated_cred_handle:
                                                    *mut gss_cred_id_t)
 -> OM_uint32 {
    let mut ctx: *mut test_context = *context_handle as *mut test_context;
    let mut hops: uint8_t = 0;
    let mut mech_last_octet: uint8_t = 0;
    let mut envstr: *const libc::c_char = 0 as *const libc::c_char;
    /*
     * The unwrapped token sits at the end and is just one byte giving the
     * remaining number of hops.  The final octet of the mech encoding should
     * be just prior to it.
     */
    if (*input_token).length >= 2 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"input_token->length >= 2\x00" as *const u8 as
                          *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      120 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 197],
                                                &[libc::c_char; 197]>(b"OM_uint32 gss_accept_sec_context(OM_uint32 *, gss_ctx_id_t *, gss_cred_id_t, gss_buffer_t, gss_channel_bindings_t, gss_name_t *, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *, gss_cred_id_t *)\x00")).as_ptr());
    }
    hops =
        *((*input_token).value as
              *mut uint8_t).offset((*input_token).length.wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)
                                       as isize);
    mech_last_octet =
        *((*input_token).value as
              *mut uint8_t).offset((*input_token).length.wrapping_sub(2 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)
                                       as isize);
    envstr = getenv(b"ACCEPT_FAIL\x00" as *const u8 as *const libc::c_char);
    if !envstr.is_null() && atoi(envstr) == mech_last_octet as libc::c_int {
        (*output_token).value =
            gssalloc_strdup(b"fail\x00" as *const u8 as *const libc::c_char)
                as *mut libc::c_void;
        if !(*output_token).value.is_null() {
        } else {
            __assert_fail(b"output_token->value != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          127 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 197],
                                                    &[libc::c_char; 197]>(b"OM_uint32 gss_accept_sec_context(OM_uint32 *, gss_ctx_id_t *, gss_cred_id_t, gss_buffer_t, gss_channel_bindings_t, gss_name_t *, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *, gss_cred_id_t *)\x00")).as_ptr());
        }
        (*output_token).length = 4 as libc::c_int as size_t;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*context_handle).is_null() {
        ctx =
            malloc(::std::mem::size_of::<test_context>() as libc::c_ulong) as
                *mut test_context;
        if !ctx.is_null() {
        } else {
            __assert_fail(b"ctx != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          134 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 197],
                                                    &[libc::c_char; 197]>(b"OM_uint32 gss_accept_sec_context(OM_uint32 *, gss_ctx_id_t *, gss_cred_id_t, gss_buffer_t, gss_channel_bindings_t, gss_name_t *, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *, gss_cred_id_t *)\x00")).as_ptr());
        }
        (*ctx).initiator = 0 as libc::c_int;
        (*ctx).hops = hops;
        *context_handle = ctx as gss_ctx_id_t
    } else {
        if (*ctx).initiator == 0 {
        } else {
            __assert_fail(b"!ctx->initiator\x00" as *const u8 as
                              *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          139 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 197],
                                                    &[libc::c_char; 197]>(b"OM_uint32 gss_accept_sec_context(OM_uint32 *, gss_ctx_id_t *, gss_cred_id_t, gss_buffer_t, gss_channel_bindings_t, gss_name_t *, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *, gss_cred_id_t *)\x00")).as_ptr());
        }
        (*ctx).hops = (*ctx).hops.wrapping_sub(1);
        if (*ctx).hops as libc::c_int == hops as libc::c_int {
        } else {
            __assert_fail(b"ctx->hops == hops\x00" as *const u8 as
                              *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          141 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 197],
                                                    &[libc::c_char; 197]>(b"OM_uint32 gss_accept_sec_context(OM_uint32 *, gss_ctx_id_t *, gss_cred_id_t, gss_buffer_t, gss_channel_bindings_t, gss_name_t *, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *, gss_cred_id_t *)\x00")).as_ptr());
        }
    }
    if (*ctx).hops as libc::c_int > 0 as libc::c_int {
        /* Generate a token containing the remaining hop count. */
        (*ctx).hops = (*ctx).hops.wrapping_sub(1);
        (*output_token).value = gssalloc_malloc(1 as libc::c_int as size_t);
        if !(*output_token).value.is_null() {
        } else {
            __assert_fail(b"output_token->value != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          148 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 197],
                                                    &[libc::c_char; 197]>(b"OM_uint32 gss_accept_sec_context(OM_uint32 *, gss_ctx_id_t *, gss_cred_id_t, gss_buffer_t, gss_channel_bindings_t, gss_name_t *, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *, gss_cred_id_t *)\x00")).as_ptr());
        }
        memcpy((*output_token).value,
               &mut (*ctx).hops as *mut uint8_t as *const libc::c_void,
               1 as libc::c_int as libc::c_ulong);
        (*output_token).length = 1 as libc::c_int as size_t
    }
    return if (*ctx).hops as libc::c_int > 0 as libc::c_int {
               ((1 as libc::c_int)) << 0 as libc::c_int + 0 as libc::c_int
           } else { 0 as libc::c_int } as OM_uint32;
}
/* token_buffer */
#[no_mangle]
#[c2rust::src_loc = "156:1"]
pub unsafe extern "C" fn gss_delete_sec_context(mut minor_status:
                                                    *mut OM_uint32,
                                                mut context_handle:
                                                    *mut gss_ctx_id_t,
                                                mut output_token:
                                                    gss_buffer_t)
 -> OM_uint32 {
    free(*context_handle as *mut libc::c_void);
    *context_handle = 0 as gss_ctx_id_t;
    return 0 as libc::c_int as OM_uint32;
}
/* Function Prototypes */
#[no_mangle]
#[c2rust::src_loc = "165:1"]
pub unsafe extern "C" fn gss_acquire_cred(mut minor_status: *mut OM_uint32,
                                          mut desired_name: gss_name_t,
                                          mut time_req: OM_uint32,
                                          mut desired_mechs: gss_OID_set,
                                          mut cred_usage: gss_cred_usage_t,
                                          mut output_cred_handle:
                                              *mut gss_cred_id_t,
                                          mut actual_mechs: *mut gss_OID_set,
                                          mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "175:1"]
pub unsafe extern "C" fn gss_acquire_cred_with_password(mut minor_status:
                                                            *mut OM_uint32,
                                                        desired_name:
                                                            gss_name_t,
                                                        password:
                                                            gss_buffer_t,
                                                        mut time_req:
                                                            OM_uint32,
                                                        desired_mechs:
                                                            gss_OID_set,
                                                        mut cred_usage:
                                                            gss_cred_usage_t,
                                                        mut output_cred_handle:
                                                            *mut gss_cred_id_t,
                                                        mut actual_mechs:
                                                            *mut gss_OID_set,
                                                        mut time_rec:
                                                            *mut OM_uint32)
 -> OM_uint32 {
    return 0 as libc::c_int as OM_uint32;
}
/* time_rec */
#[no_mangle]
#[c2rust::src_loc = "187:1"]
pub unsafe extern "C" fn gss_release_cred(mut minor_status: *mut OM_uint32,
                                          mut cred_handle: *mut gss_cred_id_t)
 -> OM_uint32 {
    return 0 as libc::c_int as OM_uint32;
}
/* output_name_type */
#[no_mangle]
#[c2rust::src_loc = "193:1"]
pub unsafe extern "C" fn gss_import_name(mut minor_status: *mut OM_uint32,
                                         mut input_name_buffer: gss_buffer_t,
                                         mut input_name_type: gss_OID,
                                         mut output_name: *mut gss_name_t)
 -> OM_uint32 {
    static mut dummy: libc::c_int = 0;
    /*
     * We don't need to remember anything about names, but we do need to
     * distinguish them from GSS_C_NO_NAME (to determine the direction of
     * gss_query_meta_data() and gss_exchange_meta_data()), so assign an
     * arbitrary data pointer.
     */
    *output_name = &mut dummy as *mut libc::c_int as gss_name_t;
    return 0 as libc::c_int as OM_uint32;
}
/* output_name */
#[no_mangle]
#[c2rust::src_loc = "209:1"]
pub unsafe extern "C" fn gss_release_name(mut minor_status: *mut OM_uint32,
                                          mut input_name: *mut gss_name_t)
 -> OM_uint32 {
    return 0 as libc::c_int as OM_uint32;
}
/* qop_state */
#[no_mangle]
#[c2rust::src_loc = "215:1"]
pub unsafe extern "C" fn gss_display_status(mut minor_status: *mut OM_uint32,
                                            mut status_value: OM_uint32,
                                            mut status_type: libc::c_int,
                                            mut mech_type: gss_OID,
                                            mut message_context:
                                                *mut OM_uint32,
                                            mut status_string: gss_buffer_t)
 -> OM_uint32 {
    if status_type == 2 as libc::c_int &&
           status_value == 12345 as libc::c_int as libc::c_uint {
        (*status_string).value =
            gssalloc_strdup(b"failure from acceptor\x00" as *const u8 as
                                *const libc::c_char) as *mut libc::c_void;
        if !(*status_string).value.is_null() {
        } else {
            __assert_fail(b"status_string->value != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"main.c\x00" as *const u8 as *const libc::c_char,
                          222 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 94],
                                                    &[libc::c_char; 94]>(b"OM_uint32 gss_display_status(OM_uint32 *, OM_uint32, int, gss_OID, OM_uint32 *, gss_buffer_t)\x00")).as_ptr());
        }
        (*status_string).length =
            strlen((*status_string).value as *const libc::c_char);
        return 0 as libc::c_int as OM_uint32
    }
    return (5 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
/* cred_usage_stored */
/*
 * A mech can make itself negotiable via NegoEx (draft-zhu-negoex) by
 * implementing the following three SPIs, and also implementing
 * gss_inquire_sec_context_by_oid() and answering the GSS_C_INQ_NEGOEX_KEY and
 * GSS_C_INQ_NEGOEX_VERIFY_KEY OIDs.  The answer must be in two buffers: the
 * first contains the key contents, and the second contains the key enctype as
 * a four-byte little-endian integer.
 *
 * By default, NegoEx mechanisms will not be directly negotiated via SPNEGO.
 * If direct SPNEGO negotiation is required for interoperability, implement
 * gss_inquire_attrs_for_mech() and assert the GSS_C_MA_NEGOEX_AND_SPNEGO
 * attribute (along with any applicable RFC 5587 attributes).
 */
#[no_mangle]
#[c2rust::src_loc = "229:1"]
pub unsafe extern "C" fn gssspi_query_meta_data(mut minor_status:
                                                    *mut OM_uint32,
                                                mut mech_oid: gss_const_OID,
                                                mut cred_handle:
                                                    gss_cred_id_t,
                                                mut context_handle:
                                                    *mut gss_ctx_id_t,
                                                targ_name: gss_name_t,
                                                mut req_flags: OM_uint32,
                                                mut meta_data: gss_buffer_t)
 -> OM_uint32 {
    let mut envstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut mech_last_octet: uint8_t = 0;
    let mut initiator: libc::c_int =
        (targ_name != 0 as gss_name_t) as libc::c_int;
    mech_last_octet =
        *((*mech_oid).elements as
              *mut uint8_t).offset((*mech_oid).length.wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                       as isize);
    envstr =
        getenv(if initiator != 0 {
                   b"INIT_QUERY_FAIL\x00" as *const u8 as *const libc::c_char
               } else {
                   b"ACCEPT_QUERY_FAIL\x00" as *const u8 as
                       *const libc::c_char
               });
    if !envstr.is_null() && atoi(envstr) == mech_last_octet as libc::c_int {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    envstr =
        getenv(if initiator != 0 {
                   b"INIT_QUERY_NONE\x00" as *const u8 as *const libc::c_char
               } else {
                   b"ACCEPT_QUERY_NONE\x00" as *const u8 as
                       *const libc::c_char
               });
    if !envstr.is_null() && atoi(envstr) == mech_last_octet as libc::c_int {
        return 0 as libc::c_int as OM_uint32
    }
    (*meta_data).value =
        gssalloc_strdup(b"X\x00" as *const u8 as *const libc::c_char) as
            *mut libc::c_void;
    (*meta_data).length = 1 as libc::c_int as size_t;
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "252:1"]
pub unsafe extern "C" fn gssspi_exchange_meta_data(mut minor_status:
                                                       *mut OM_uint32,
                                                   mut mech_oid:
                                                       gss_const_OID,
                                                   mut cred_handle:
                                                       gss_cred_id_t,
                                                   mut context_handle:
                                                       *mut gss_ctx_id_t,
                                                   targ_name: gss_name_t,
                                                   mut req_flags: OM_uint32,
                                                   mut meta_data:
                                                       gss_const_buffer_t)
 -> OM_uint32 {
    let mut envstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut mech_last_octet: uint8_t = 0;
    let mut initiator: libc::c_int =
        (targ_name != 0 as gss_name_t) as libc::c_int;
    mech_last_octet =
        *((*mech_oid).elements as
              *mut uint8_t).offset((*mech_oid).length.wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                       as isize);
    envstr =
        getenv(if initiator != 0 {
                   b"INIT_EXCHANGE_FAIL\x00" as *const u8 as
                       *const libc::c_char
               } else {
                   b"ACCEPT_EXCHANGE_FAIL\x00" as *const u8 as
                       *const libc::c_char
               });
    if !envstr.is_null() && atoi(envstr) == mech_last_octet as libc::c_int {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*meta_data).length == 1 as libc::c_int as libc::c_ulong &&
           memcmp((*meta_data).value,
                  b"X\x00" as *const u8 as *const libc::c_char as
                      *const libc::c_void, 1 as libc::c_int as libc::c_ulong)
               == 0 as libc::c_int {
    } else {
        __assert_fail(b"meta_data->length == 1 && memcmp(meta_data->value, \"X\", 1) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      268 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 144],
                                                &[libc::c_char; 144]>(b"OM_uint32 gssspi_exchange_meta_data(OM_uint32 *, gss_const_OID, gss_cred_id_t, gss_ctx_id_t *, const gss_name_t, OM_uint32, gss_const_buffer_t)\x00")).as_ptr());
    }
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "272:1"]
pub unsafe extern "C" fn gssspi_query_mechanism_info(mut minor_status:
                                                         *mut OM_uint32,
                                                     mut mech_oid:
                                                         gss_const_OID,
                                                     mut auth_scheme:
                                                         *mut libc::c_uchar)
 -> OM_uint32 {
    /* Copy the mech OID encoding and right-pad it with zeros. */
    memset(auth_scheme as *mut libc::c_void, 0 as libc::c_int,
           16 as libc::c_int as libc::c_ulong);
    if (*mech_oid).length <= 16 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"mech_oid->length <= 16\x00" as *const u8 as
                          *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      278 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[libc::c_char; 83]>(b"OM_uint32 gssspi_query_mechanism_info(OM_uint32 *, gss_const_OID, unsigned char *)\x00")).as_ptr());
    }
    memcpy(auth_scheme as *mut libc::c_void, (*mech_oid).elements,
           (*mech_oid).length as libc::c_ulong);
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "283:1"]
pub unsafe extern "C" fn gss_inquire_sec_context_by_oid(mut minor_status:
                                                            *mut OM_uint32,
                                                        context_handle:
                                                            gss_ctx_id_t,
                                                        desired_object:
                                                            gss_OID,
                                                        mut data_set:
                                                            *mut gss_buffer_set_t)
 -> OM_uint32 {
    let mut ctx: *mut test_context = context_handle as *mut test_context;
    let mut major: OM_uint32 = 0;
    let mut keybytes: [uint8_t; 32] =
        [0 as libc::c_int as uint8_t, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut typebytes: [uint8_t; 4] = [0; 4];
    let mut key: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut type_0: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut envstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ask_verify: libc::c_int = 0;
    if gss_oid_equal(desired_object as gss_const_OID,
                     GSS_C_INQ_NEGOEX_KEY as gss_const_OID) != 0 {
        ask_verify = 0 as libc::c_int
    } else if gss_oid_equal(desired_object as gss_const_OID,
                            GSS_C_INQ_NEGOEX_VERIFY_KEY as gss_const_OID) != 0
     {
        ask_verify = 1 as libc::c_int
    } else { return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int }
    /*
     * By default, make a key available only if the context is established.
     * This can be overridden to "always", "init-always", "accept-always",
     * or "never".
     */
    envstr = getenv(b"KEY\x00" as *const u8 as *const libc::c_char);
    if !envstr.is_null() &&
           strcmp(envstr, b"never\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    } else {
        if (*ctx).hops as libc::c_int > 0 as libc::c_int {
            if envstr.is_null() {
                return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
            } else {
                if strcmp(envstr,
                          b"init-always\x00" as *const u8 as
                              *const libc::c_char) == 0 as libc::c_int &&
                       (*ctx).initiator == 0 {
                    return (16 as libc::c_ulong as OM_uint32) <<
                               16 as libc::c_int
                } else {
                    if strcmp(envstr,
                              b"accept-always\x00" as *const u8 as
                                  *const libc::c_char) == 0 as libc::c_int &&
                           (*ctx).initiator != 0 {
                        return (16 as libc::c_ulong as OM_uint32) <<
                                   16 as libc::c_int
                    }
                }
            }
        }
    }
    /* Perturb the key so that each side's verifier key is equal to the other's
     * checksum key. */
    keybytes[0 as libc::c_int as usize] =
        (ask_verify ^ (*ctx).initiator) as uint8_t;
    /* Supply an all-zeros aes256-sha1 negoex key. */
    if gss_oid_equal(desired_object as gss_const_OID,
                     GSS_C_INQ_NEGOEX_KEY as gss_const_OID) != 0 ||
           gss_oid_equal(desired_object as gss_const_OID,
                         GSS_C_INQ_NEGOEX_VERIFY_KEY as gss_const_OID) != 0 {
        store_32_le(0x12 as libc::c_int as libc::c_uint,
                    typebytes.as_mut_ptr() as *mut libc::c_void);
        key.value = keybytes.as_mut_ptr() as *mut libc::c_void;
        key.length = ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong;
        type_0.value = typebytes.as_mut_ptr() as *mut libc::c_void;
        type_0.length =
            ::std::mem::size_of::<[uint8_t; 4]>() as libc::c_ulong;
        major = gss_add_buffer_set_member(minor_status, &mut key, data_set);
        if major != 0 as libc::c_int as libc::c_uint { return major }
        return gss_add_buffer_set_member(minor_status, &mut type_0, data_set)
    }
    return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
