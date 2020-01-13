use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "35:1"]
    pub type ptrdiff_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:37"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:37"]
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
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    /*
 * For now, define a QOP-type as an OM_uint32 (pending resolution of ongoing
 * discussions).
 */
    #[c2rust::src_loc = "134:1"]
    pub type gss_qop_t = OM_uint32;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
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
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* time_rec */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "489:1"]
        pub fn gss_get_mic(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_qop_t,
                           _: gss_buffer_t, _: gss_buffer_t) -> OM_uint32;
        /* message_token */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn gss_verify_mic(_: *mut OM_uint32, _: gss_ctx_id_t,
                              _: gss_buffer_t, _: gss_buffer_t,
                              _: *mut gss_qop_t) -> OM_uint32;
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "508:1"]
        pub fn gss_wrap(_: *mut OM_uint32, _: gss_ctx_id_t, _: libc::c_int,
                        _: gss_qop_t, _: gss_buffer_t, _: *mut libc::c_int,
                        _: gss_buffer_t) -> OM_uint32;
        /* output_message_buffer */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "520:1"]
        pub fn gss_unwrap(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_buffer_t,
                          _: gss_buffer_t, _: *mut libc::c_int,
                          _: *mut gss_qop_t) -> OM_uint32;
        /* output_name */
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:37"]
pub mod gssapi_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "248:16"]
    pub struct gss_iov_buffer_desc_struct {
        pub type_0: OM_uint32,
        pub buffer: gss_buffer_desc,
    }
    #[c2rust::src_loc = "248:1"]
    pub type gss_iov_buffer_desc = gss_iov_buffer_desc_struct;
    use super::gssapi_h::{OM_uint32, gss_buffer_desc, gss_ctx_id_struct,
                          gss_ctx_id_t, gss_qop_t, gss_buffer_desc_struct,
                          gss_buffer_t};
    extern "C" {
        /*
 * AEAD extensions
 */
        #[no_mangle]
        #[c2rust::src_loc = "207:1"]
        pub fn gss_wrap_aead(_: *mut OM_uint32, _: gss_ctx_id_t,
                             _: libc::c_int, _: gss_qop_t, _: gss_buffer_t,
                             _: gss_buffer_t, _: *mut libc::c_int,
                             _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn gss_unwrap_aead(_: *mut OM_uint32, _: gss_ctx_id_t,
                               _: gss_buffer_t, _: gss_buffer_t,
                               _: gss_buffer_t, _: *mut libc::c_int,
                               _: *mut gss_qop_t) -> OM_uint32;
        /* Packet data */
        /* Mechanism header */
        /* Mechanism specific parameters */
        /* Mechanism trailer */
        /* Padding */
        /* Complete wrap token */
        /* Sign only packet data */
        /* MIC token destination */
        /* indicates GSS should allocate */
        /* indicates caller should free */
        /*
 * Sign and optionally encrypt a sequence of buffers. The buffers
 * shall be ordered HEADER | DATA | PADDING | TRAILER. Suitable
 * space for the header, padding and trailer should be provided
 * by calling gss_wrap_iov_length(), or the ALLOCATE flag should
 * be set on those buffers.
 *
 * Encryption is in-place. SIGN_ONLY buffers are untouched. Only
 * a single PADDING buffer should be provided. The order of the
 * buffers in memory does not matter. Buffers in the IOV should
 * be arranged in the order above, and in the case of multiple
 * DATA buffers the sender and receiver should agree on the
 * order.
 *
 * With GSS_C_DCE_STYLE it is acceptable to not provide PADDING
 * and TRAILER, but the caller must guarantee the plaintext data
 * being encrypted is correctly padded, otherwise an error will
 * be returned.
 *
 * While applications that have knowledge of the underlying
 * cryptosystem may request a specific configuration of data
 * buffers, the only generally supported configurations are:
 *
 *  HEADER | DATA | PADDING | TRAILER
 *
 * which will emit GSS_Wrap() compatible tokens, and:
 *
 *  HEADER | SIGN_ONLY | DATA | PADDING | TRAILER
 *
 * for AEAD.
 *
 * The typical (special cased) usage for DCE is as follows:
 *
 *  SIGN_ONLY_1 | DATA | SIGN_ONLY_2 | HEADER
 */
        #[no_mangle]
        #[c2rust::src_loc = "307:1"]
        pub fn gss_wrap_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                            _: libc::c_int, _: gss_qop_t, _: *mut libc::c_int,
                            _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
        /* iov_count */
        /*
 * Verify and optionally decrypt a sequence of buffers. To process
 * a GSS-API message without separate buffer, pass STREAM | DATA.
 * Upon return DATA will contain the decrypted or integrity
 * protected message. Only a single DATA buffer may be provided
 * with this usage. DATA by default will point into STREAM, but if
 * the ALLOCATE flag is set a copy will be returned.
 *
 * Otherwise, decryption is in-place. SIGN_ONLY buffers are
 * untouched.
 */
        #[no_mangle]
        #[c2rust::src_loc = "328:1"]
        pub fn gss_unwrap_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                              _: *mut libc::c_int, _: *mut gss_qop_t,
                              _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
        /* iov_count */
        /*
 * Query HEADER, PADDING and TRAILER buffer lengths. DATA buffers
 * should be provided so the correct padding length can be determined.
 */
        #[no_mangle]
        #[c2rust::src_loc = "341:1"]
        pub fn gss_wrap_iov_length(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: libc::c_int, _: gss_qop_t,
                                   _: *mut libc::c_int,
                                   _: *mut gss_iov_buffer_desc,
                                   _: libc::c_int) -> OM_uint32;
        /* iov_count */
        /*
 * Produce a GSSAPI MIC token for a sequence of buffers.  All SIGN_ONLY and
 * DATA buffers will be signed, in the order they appear.  One MIC_TOKEN buffer
 * must be included for the result.  Suitable space should be provided for the
 * MIC_TOKEN buffer by calling gss_get_mic_iov_length, or the ALLOCATE flag
 * should be set on that buffer.  If the ALLOCATE flag is used, use
 * gss_release_iov_buffer to free the allocated buffer within the iov list when
 * it is no longer needed.
 */
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn gss_get_mic_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                               _: gss_qop_t, _: *mut gss_iov_buffer_desc,
                               _: libc::c_int) -> OM_uint32;
        /* iov_count */
        /*
 * Query the MIC_TOKEN buffer length within the iov list.
 */
        #[no_mangle]
        #[c2rust::src_loc = "371:1"]
        pub fn gss_get_mic_iov_length(_: *mut OM_uint32, _: gss_ctx_id_t,
                                      _: gss_qop_t,
                                      _: *mut gss_iov_buffer_desc,
                                      _: libc::c_int) -> OM_uint32;
        /* iov_count */
        /*
 * Verify the MIC_TOKEN buffer within the iov list against the SIGN_ONLY and
 * DATA buffers in the order they appear.  Return values are the same as for
 * gss_verify_mic.
 */
        #[no_mangle]
        #[c2rust::src_loc = "383:1"]
        pub fn gss_verify_mic_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                                  _: *mut gss_qop_t,
                                  _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
        /* iov_count */
        /*
 * Release buffers that have the ALLOCATED flag set.
 */
        #[no_mangle]
        #[c2rust::src_loc = "394:1"]
        pub fn gss_release_iov_buffer(_: *mut OM_uint32,
                                      _: *mut gss_iov_buffer_desc,
                                      _: libc::c_int) -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:37"]
pub mod krb5_h {
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:34"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:35"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:37"]
pub mod common_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, gss_cred_id_struct,
                          gss_cred_id_t, gss_name_struct, gss_name_t,
                          OM_uint32, gss_ctx_id_t, gss_OID_desc};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "59:1"]
        pub fn establish_contexts(imech: gss_OID, icred: gss_cred_id_t,
                                  acred: gss_cred_id_t, tname: gss_name_t,
                                  flags: OM_uint32, ictx: *mut gss_ctx_id_t,
                                  actx: *mut gss_ctx_id_t,
                                  src_name: *mut gss_name_t,
                                  amech: *mut gss_OID,
                                  deleg_cred: *mut gss_cred_id_t);
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn import_name(str: *const libc::c_char) -> gss_name_t;
        #[no_mangle]
        #[c2rust::src_loc = "52:1"]
        pub fn errout(msg: *const libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "46:1"]
        pub fn check_gsserr(msg: *const libc::c_char, major: OM_uint32,
                            minor: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "39:21"]
        pub static mut mech_spnego: gss_OID_desc;
        #[no_mangle]
        #[c2rust::src_loc = "38:21"]
        pub static mut mech_krb5: gss_OID_desc;
    }
    /* COMMON_H */
}
pub use self::stddef_h::{size_t, ptrdiff_t};
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_qop_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         gss_delete_sec_context, gss_get_mic, gss_verify_mic,
                         gss_wrap, gss_unwrap, gss_release_name,
                         gss_release_buffer};
pub use self::gssapi_ext_h::{gss_iov_buffer_desc_struct, gss_iov_buffer_desc,
                             gss_wrap_aead, gss_unwrap_aead, gss_wrap_iov,
                             gss_unwrap_iov, gss_wrap_iov_length,
                             gss_get_mic_iov, gss_get_mic_iov_length,
                             gss_verify_mic_iov, gss_release_iov_buffer};
pub use self::krb5_h::krb5_boolean;
use self::stdio_h::snprintf;
use self::stdlib_h::{malloc, free};
use self::string_h::{memcpy, memcmp, strcmp, strlen};
use self::common_h::{establish_contexts, import_name, errout, check_gsserr,
                     mech_spnego, mech_krb5};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/t_iov.c - Test program for IOV functions */
/*
 * Copyright (C) 2013 by the Massachusetts Institute of Technology.
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
/* Concatenate iov (except for sign-only buffers) into a contiguous token. */
#[c2rust::src_loc = "40:1"]
unsafe extern "C" fn concat_iov(mut iov: *mut gss_iov_buffer_desc,
                                mut iovlen: size_t,
                                mut buf_out: *mut *mut libc::c_char,
                                mut len_out: *mut size_t) {
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Concatenate the result into a contiguous buffer. */
    len = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < iovlen {
        if (*iov.offset(i as isize)).type_0 & !(0xffff0000 as libc::c_uint) !=
               11 as libc::c_int as libc::c_uint {
            len =
                (len as
                     libc::c_ulong).wrapping_add((*iov.offset(i as
                                                                  isize)).buffer.length)
                    as size_t as size_t
        }
        i = i.wrapping_add(1)
    }
    buf = malloc(len) as *mut libc::c_char;
    if buf.is_null() {
        errout(b"malloc failed\x00" as *const u8 as *const libc::c_char);
    }
    len = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < iovlen {
        if !((*iov.offset(i as isize)).type_0 & !(0xffff0000 as libc::c_uint)
                 == 11 as libc::c_int as libc::c_uint) {
            memcpy(buf.offset(len as isize) as *mut libc::c_void,
                   (*iov.offset(i as isize)).buffer.value,
                   (*iov.offset(i as isize)).buffer.length);
            len =
                (len as
                     libc::c_ulong).wrapping_add((*iov.offset(i as
                                                                  isize)).buffer.length)
                    as size_t as size_t
        }
        i = i.wrapping_add(1)
    }
    *buf_out = buf;
    *len_out = len;
}
#[c2rust::src_loc = "67:1"]
unsafe extern "C" fn check_encrypted(mut msg: *const libc::c_char,
                                     mut conf: libc::c_int,
                                     mut buf: *const libc::c_char,
                                     mut plain: *const libc::c_char) {
    let mut same: libc::c_int =
        (memcmp(buf as *const libc::c_void, plain as *const libc::c_void,
                strlen(plain)) == 0 as libc::c_int) as libc::c_int;
    if conf != 0 && same != 0 || conf == 0 && same == 0 { errout(msg); };
}
/*
 * Wrap str in standard form (HEADER | DATA | PADDING | TRAILER) using the
 * caller-provided array iov, which must have space for four elements.  Library
 * allocation will be used for the header/padding/trailer buffers, so the
 * caller must check and free them.
 */
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn wrap_std(mut ctx: gss_ctx_id_t,
                              mut str: *mut libc::c_char,
                              mut iov: *mut gss_iov_buffer_desc,
                              mut conf: libc::c_int) {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut oconf: libc::c_int = 0;
    /* Lay out iov array. */
    (*iov.offset(0 as libc::c_int as isize)).type_0 =
        (2 as libc::c_int | 0x10000 as libc::c_int) as OM_uint32;
    (*iov.offset(1 as libc::c_int as isize)).type_0 =
        1 as libc::c_int as OM_uint32;
    let ref mut fresh0 =
        (*iov.offset(1 as libc::c_int as isize)).buffer.value;
    *fresh0 = str as *mut libc::c_void;
    (*iov.offset(1 as libc::c_int as isize)).buffer.length = strlen(str);
    (*iov.offset(2 as libc::c_int as isize)).type_0 =
        (9 as libc::c_int | 0x10000 as libc::c_int) as OM_uint32;
    (*iov.offset(3 as libc::c_int as isize)).type_0 =
        (7 as libc::c_int | 0x10000 as libc::c_int) as OM_uint32;
    /* Wrap.  This will allocate header/padding/trailer buffers as necessary
     * and encrypt str in place. */
    major =
        gss_wrap_iov(&mut minor, ctx, conf, 0 as libc::c_int as gss_qop_t,
                     &mut oconf, iov, 4 as libc::c_int);
    check_gsserr(b"gss_wrap_iov(std)\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    if oconf != conf {
        errout(b"gss_wrap_iov(std) conf\x00" as *const u8 as
                   *const libc::c_char);
    };
}
/* Create standard tokens using gss_wrap_iov and ctx1, and make sure we can
 * unwrap them using ctx2 in all of the supported ways. */
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn test_standard_wrap(mut ctx1: gss_ctx_id_t,
                                        mut ctx2: gss_ctx_id_t,
                                        mut conf: libc::c_int) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut iov: [gss_iov_buffer_desc; 4] =
        [gss_iov_buffer_desc{type_0: 0,
                             buffer:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as
                                                         *mut libc::c_void,},};
            4];
    let mut stiov: [gss_iov_buffer_desc; 2] =
        [gss_iov_buffer_desc{type_0: 0,
                             buffer:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as
                                                         *mut libc::c_void,},};
            2];
    let mut qop: gss_qop_t = 0;
    let mut input: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut output: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut string1: *const libc::c_char =
        b"The swift brown fox jumped over the lazy dog.\x00" as *const u8 as
            *const libc::c_char;
    let mut string2: *const libc::c_char =
        b"Now is the time!\x00" as *const u8 as *const libc::c_char;
    let mut string3: *const libc::c_char =
        b"x\x00" as *const u8 as *const libc::c_char;
    let mut string4: *const libc::c_char =
        b"!@#\x00" as *const u8 as *const libc::c_char;
    let mut data: [libc::c_char; 1024] = [0; 1024];
    let mut fulltoken: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut oconf: libc::c_int = 0;
    let mut offset: ptrdiff_t = 0;
    /* Wrap a standard token and unwrap it using the iov array. */
    memcpy(data.as_mut_ptr() as *mut libc::c_void,
           string1 as *const libc::c_void,
           strlen(string1).wrapping_add(1 as libc::c_int as libc::c_ulong));
    wrap_std(ctx1, data.as_mut_ptr(), iov.as_mut_ptr(), conf);
    check_encrypted(b"gss_wrap_iov(std1) encryption\x00" as *const u8 as
                        *const libc::c_char, conf, data.as_mut_ptr(),
                    string1);
    major =
        gss_unwrap_iov(&mut minor, ctx2, &mut oconf, &mut qop,
                       iov.as_mut_ptr(), 4 as libc::c_int);
    check_gsserr(b"gss_unwrap_iov(std1)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if oconf != conf || qop != 0 as libc::c_int as libc::c_uint {
        errout(b"gss_unwrap_iov(std1) conf/qop\x00" as *const u8 as
                   *const libc::c_char);
    }
    if iov[1 as libc::c_int as usize].buffer.value !=
           data.as_mut_ptr() as *mut libc::c_void ||
           iov[1 as libc::c_int as usize].buffer.length != strlen(string1) {
        errout(b"gss_unwrap_iov(std1) data buffer\x00" as *const u8 as
                   *const libc::c_char);
    }
    if memcmp(data.as_mut_ptr() as *const libc::c_void,
              string1 as *const libc::c_void,
              iov[1 as libc::c_int as usize].buffer.length) !=
           0 as libc::c_int {
        errout(b"gss_unwrap_iov(std1) decryption\x00" as *const u8 as
                   *const libc::c_char);
    }
    gss_release_iov_buffer(&mut minor, iov.as_mut_ptr(), 4 as libc::c_int);
    /* Wrap a standard token and unwrap it using gss_unwrap(). */
    memcpy(data.as_mut_ptr() as *mut libc::c_void,
           string2 as *const libc::c_void,
           strlen(string2).wrapping_add(1 as libc::c_int as libc::c_ulong));
    wrap_std(ctx1, data.as_mut_ptr(), iov.as_mut_ptr(), conf);
    concat_iov(iov.as_mut_ptr(), 4 as libc::c_int as size_t, &mut fulltoken,
               &mut len);
    input.value = fulltoken as *mut libc::c_void;
    input.length = len;
    major =
        gss_unwrap(&mut minor, ctx2, &mut input, &mut output, &mut oconf,
                   &mut qop);
    check_gsserr(b"gss_unwrap(std2)\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    if oconf != conf || qop != 0 as libc::c_int as libc::c_uint {
        errout(b"gss_unwrap(std2) conf/qop\x00" as *const u8 as
                   *const libc::c_char);
    }
    if output.length != strlen(string2) ||
           memcmp(output.value, string2 as *const libc::c_void, output.length)
               != 0 as libc::c_int {
        errout(b"gss_unwrap(std2) decryption\x00" as *const u8 as
                   *const libc::c_char);
    }
    gss_release_buffer(&mut minor, &mut output);
    gss_release_iov_buffer(&mut minor, iov.as_mut_ptr(), 4 as libc::c_int);
    free(fulltoken as *mut libc::c_void);
    /* Wrap a standard token and unwrap it using a stream buffer. */
    memcpy(data.as_mut_ptr() as *mut libc::c_void,
           string3 as *const libc::c_void,
           strlen(string3).wrapping_add(1 as libc::c_int as libc::c_ulong));
    wrap_std(ctx1, data.as_mut_ptr(), iov.as_mut_ptr(), conf);
    concat_iov(iov.as_mut_ptr(), 4 as libc::c_int as size_t, &mut fulltoken,
               &mut len);
    stiov[0 as libc::c_int as usize].type_0 = 10 as libc::c_int as OM_uint32;
    stiov[0 as libc::c_int as usize].buffer.value =
        fulltoken as *mut libc::c_void;
    stiov[0 as libc::c_int as usize].buffer.length = len;
    stiov[1 as libc::c_int as usize].type_0 = 1 as libc::c_int as OM_uint32;
    major =
        gss_unwrap_iov(&mut minor, ctx2, &mut oconf, &mut qop,
                       stiov.as_mut_ptr(), 2 as libc::c_int);
    check_gsserr(b"gss_unwrap_iov(std3)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if oconf != conf || qop != 0 as libc::c_int as libc::c_uint {
        errout(b"gss_unwrap_iov(std3) conf/qop\x00" as *const u8 as
                   *const libc::c_char);
    }
    if stiov[1 as libc::c_int as usize].buffer.length != strlen(string3) ||
           memcmp(stiov[1 as libc::c_int as usize].buffer.value,
                  string3 as *const libc::c_void, strlen(string3)) !=
               0 as libc::c_int {
        errout(b"gss_unwrap_iov(std3) decryption\x00" as *const u8 as
                   *const libc::c_char);
    }
    offset =
        (stiov[1 as libc::c_int as usize].buffer.value as
             *mut libc::c_char).wrapping_offset_from(fulltoken) as
            libc::c_long;
    if offset < 0 as libc::c_int as libc::c_long || offset as size_t > len {
        errout(b"gss_unwrap_iov(std3) offset\x00" as *const u8 as
                   *const libc::c_char);
    }
    gss_release_iov_buffer(&mut minor, iov.as_mut_ptr(), 4 as libc::c_int);
    free(fulltoken as *mut libc::c_void);
    /* Wrap a token using gss_wrap and unwrap it using a stream buffer with
     * allocation and copying. */
    input.value = string4 as *mut libc::c_char as *mut libc::c_void;
    input.length = strlen(string4);
    major =
        gss_wrap(&mut minor, ctx1, conf, 0 as libc::c_int as gss_qop_t,
                 &mut input, &mut oconf, &mut output);
    check_gsserr(b"gss_wrap(std4)\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    if oconf != conf {
        errout(b"gss_wrap(std4) conf\x00" as *const u8 as
                   *const libc::c_char);
    }
    stiov[0 as libc::c_int as usize].type_0 = 10 as libc::c_int as OM_uint32;
    stiov[0 as libc::c_int as usize].buffer = output;
    stiov[1 as libc::c_int as usize].type_0 =
        (1 as libc::c_int | 0x10000 as libc::c_int) as OM_uint32;
    major =
        gss_unwrap_iov(&mut minor, ctx2, &mut oconf, &mut qop,
                       stiov.as_mut_ptr(), 2 as libc::c_int);
    check_gsserr(b"gss_unwrap_iov(std4)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if stiov[1 as libc::c_int as usize].type_0 & 0xffff0000 as libc::c_uint &
           0x20000 as libc::c_int as libc::c_uint == 0 {
        errout(b"gss_unwrap_iov(std4) allocated\x00" as *const u8 as
                   *const libc::c_char);
    }
    if oconf != conf || qop != 0 as libc::c_int as libc::c_uint {
        errout(b"gss_unwrap_iov(std4) conf/qop\x00" as *const u8 as
                   *const libc::c_char);
    }
    if stiov[1 as libc::c_int as usize].buffer.length != strlen(string4) ||
           memcmp(stiov[1 as libc::c_int as usize].buffer.value,
                  string4 as *const libc::c_void, strlen(string4)) !=
               0 as libc::c_int {
        errout(b"gss_unwrap_iov(std4) decryption\x00" as *const u8 as
                   *const libc::c_char);
    }
    gss_release_buffer(&mut minor, &mut output);
    gss_release_iov_buffer(&mut minor, stiov.as_mut_ptr(), 2 as libc::c_int);
}
/*
 * Wrap an AEAD token (HEADER | SIGN_ONLY | DATA | PADDING | TRAILER) using the
 * caller-provided array iov, which must have space for five elements, and the
 * caller-provided buffer data, which must be big enough to handle the test
 * inputs.  Library allocation will not be used.
 */
#[c2rust::src_loc = "205:1"]
unsafe extern "C" fn wrap_aead(mut ctx: gss_ctx_id_t,
                               mut sign: *const libc::c_char,
                               mut wrap: *const libc::c_char,
                               mut iov: *mut gss_iov_buffer_desc,
                               mut data: *mut libc::c_char,
                               mut conf: libc::c_int) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut oconf: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Lay out iov array. */
    (*iov.offset(0 as libc::c_int as isize)).type_0 =
        2 as libc::c_int as OM_uint32;
    (*iov.offset(1 as libc::c_int as isize)).type_0 =
        11 as libc::c_int as OM_uint32;
    let ref mut fresh1 =
        (*iov.offset(1 as libc::c_int as isize)).buffer.value;
    *fresh1 = sign as *mut libc::c_char as *mut libc::c_void;
    (*iov.offset(1 as libc::c_int as isize)).buffer.length = strlen(sign);
    (*iov.offset(2 as libc::c_int as isize)).type_0 =
        1 as libc::c_int as OM_uint32;
    let ref mut fresh2 =
        (*iov.offset(2 as libc::c_int as isize)).buffer.value;
    *fresh2 = wrap as *mut libc::c_char as *mut libc::c_void;
    (*iov.offset(2 as libc::c_int as isize)).buffer.length = strlen(wrap);
    (*iov.offset(3 as libc::c_int as isize)).type_0 =
        9 as libc::c_int as OM_uint32;
    (*iov.offset(4 as libc::c_int as isize)).type_0 =
        7 as libc::c_int as OM_uint32;
    /* Get header/padding/trailer lengths. */
    major =
        gss_wrap_iov_length(&mut minor, ctx, conf,
                            0 as libc::c_int as gss_qop_t, &mut oconf, iov,
                            5 as libc::c_int);
    check_gsserr(b"gss_wrap_iov_length(aead)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if oconf != conf {
        errout(b"gss_wrap_iov_length(aead) conf\x00" as *const u8 as
                   *const libc::c_char);
    }
    if (*iov.offset(1 as libc::c_int as isize)).buffer.value !=
           sign as *mut libc::c_void ||
           (*iov.offset(1 as libc::c_int as isize)).buffer.length !=
               strlen(sign) {
        errout(b"gss_wrap_iov_length(aead) sign-only buffer\x00" as *const u8
                   as *const libc::c_char);
    }
    if (*iov.offset(2 as libc::c_int as isize)).buffer.value !=
           wrap as *mut libc::c_void ||
           (*iov.offset(2 as libc::c_int as isize)).buffer.length !=
               strlen(wrap) {
        errout(b"gss_wrap_iov_length(aead) data buffer\x00" as *const u8 as
                   *const libc::c_char);
    }
    /* Set iov buffer pointers using returned lengths. */
    let ref mut fresh3 =
        (*iov.offset(0 as libc::c_int as isize)).buffer.value;
    *fresh3 = data as *mut libc::c_void;
    ptr =
        data.offset((*iov.offset(0 as libc::c_int as isize)).buffer.length as
                        isize);
    memcpy(ptr as *mut libc::c_void, wrap as *const libc::c_void,
           strlen(wrap));
    let ref mut fresh4 =
        (*iov.offset(2 as libc::c_int as isize)).buffer.value;
    *fresh4 = ptr as *mut libc::c_void;
    ptr =
        ptr.offset((*iov.offset(2 as libc::c_int as isize)).buffer.length as
                       isize);
    let ref mut fresh5 =
        (*iov.offset(3 as libc::c_int as isize)).buffer.value;
    *fresh5 = ptr as *mut libc::c_void;
    ptr =
        ptr.offset((*iov.offset(3 as libc::c_int as isize)).buffer.length as
                       isize);
    let ref mut fresh6 =
        (*iov.offset(4 as libc::c_int as isize)).buffer.value;
    *fresh6 = ptr as *mut libc::c_void;
    /* Wrap the AEAD token. */
    major =
        gss_wrap_iov(&mut minor, ctx, conf, 0 as libc::c_int as gss_qop_t,
                     &mut oconf, iov, 5 as libc::c_int);
    check_gsserr(b"gss_wrap_iov(aead)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if oconf != conf {
        errout(b"gss_wrap_iov(aead) conf\x00" as *const u8 as
                   *const libc::c_char);
    }
    if (*iov.offset(1 as libc::c_int as isize)).buffer.value !=
           sign as *mut libc::c_void ||
           (*iov.offset(1 as libc::c_int as isize)).buffer.length !=
               strlen(sign) {
        errout(b"gss_wrap_iov(aead) sign-only buffer\x00" as *const u8 as
                   *const libc::c_char);
    }
    if (*iov.offset(2 as libc::c_int as isize)).buffer.length != strlen(wrap)
       {
        errout(b"gss_wrap_iov(aead) data buffer\x00" as *const u8 as
                   *const libc::c_char);
    }
    check_encrypted(b"gss_wrap_iov(aead) encryption\x00" as *const u8 as
                        *const libc::c_char, conf,
                    (*iov.offset(2 as libc::c_int as isize)).buffer.value as
                        *const libc::c_char, wrap);
}
/* Create AEAD tokens using gss_wrap_iov and ctx1, and make sure we can unwrap
 * them using ctx2 in all of the supported ways. */
#[c2rust::src_loc = "260:1"]
unsafe extern "C" fn test_aead(mut ctx1: gss_ctx_id_t, mut ctx2: gss_ctx_id_t,
                               mut conf: libc::c_int) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut iov: [gss_iov_buffer_desc; 5] =
        [gss_iov_buffer_desc{type_0: 0,
                             buffer:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as
                                                         *mut libc::c_void,},};
            5];
    let mut stiov: [gss_iov_buffer_desc; 3] =
        [gss_iov_buffer_desc{type_0: 0,
                             buffer:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as
                                                         *mut libc::c_void,},};
            3];
    let mut qop: gss_qop_t = 0;
    let mut input: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut assoc: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut output: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut sign: *const libc::c_char =
        b"This data is only signed.\x00" as *const u8 as *const libc::c_char;
    let mut wrap: *const libc::c_char =
        b"This data is wrapped in-place.\x00" as *const u8 as
            *const libc::c_char;
    let mut data: [libc::c_char; 1024] = [0; 1024];
    let mut fulltoken: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut oconf: libc::c_int = 0;
    let mut offset: ptrdiff_t = 0;
    /* Wrap an AEAD token and unwrap it using the IOV array. */
    wrap_aead(ctx1, sign, wrap, iov.as_mut_ptr(), data.as_mut_ptr(), conf);
    major =
        gss_unwrap_iov(&mut minor, ctx2, &mut oconf, &mut qop,
                       iov.as_mut_ptr(), 5 as libc::c_int);
    check_gsserr(b"gss_unwrap_iov(aead1)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if oconf != conf || qop != 0 as libc::c_int as libc::c_uint {
        errout(b"gss_unwrap_iov(aead1) conf/qop\x00" as *const u8 as
                   *const libc::c_char);
    }
    if iov[1 as libc::c_int as usize].buffer.value !=
           sign as *mut libc::c_void ||
           iov[1 as libc::c_int as usize].buffer.length != strlen(sign) {
        errout(b"gss_unwrap_iov(aead1) sign-only buffer\x00" as *const u8 as
                   *const libc::c_char);
    }
    if iov[2 as libc::c_int as usize].buffer.length != strlen(wrap) ||
           memcmp(iov[2 as libc::c_int as usize].buffer.value,
                  wrap as *const libc::c_void,
                  iov[2 as libc::c_int as usize].buffer.length) !=
               0 as libc::c_int {
        errout(b"gss_unwrap_iov(aead1) decryption\x00" as *const u8 as
                   *const libc::c_char);
    }
    /* Wrap an AEAD token and unwrap it using gss_unwrap_aead. */
    wrap_aead(ctx1, sign, wrap, iov.as_mut_ptr(), data.as_mut_ptr(), conf);
    concat_iov(iov.as_mut_ptr(), 5 as libc::c_int as size_t, &mut fulltoken,
               &mut len);
    input.value = fulltoken as *mut libc::c_void;
    input.length = len;
    assoc.value = sign as *mut libc::c_char as *mut libc::c_void;
    assoc.length = strlen(sign);
    major =
        gss_unwrap_aead(&mut minor, ctx2, &mut input, &mut assoc, &mut output,
                        &mut oconf, &mut qop);
    check_gsserr(b"gss_unwrap_aead(aead2)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if output.length != strlen(wrap) ||
           memcmp(output.value, wrap as *const libc::c_void, output.length) !=
               0 as libc::c_int {
        errout(b"gss_unwrap_aead(aead2) decryption\x00" as *const u8 as
                   *const libc::c_char);
    }
    free(fulltoken as *mut libc::c_void);
    gss_release_buffer(&mut minor, &mut output);
    /* Wrap an AEAD token and unwrap it using a stream buffer. */
    wrap_aead(ctx1, sign, wrap, iov.as_mut_ptr(), data.as_mut_ptr(), conf);
    concat_iov(iov.as_mut_ptr(), 5 as libc::c_int as size_t, &mut fulltoken,
               &mut len);
    stiov[0 as libc::c_int as usize].type_0 = 10 as libc::c_int as OM_uint32;
    stiov[0 as libc::c_int as usize].buffer.value =
        fulltoken as *mut libc::c_void;
    stiov[0 as libc::c_int as usize].buffer.length = len;
    stiov[1 as libc::c_int as usize].type_0 = 11 as libc::c_int as OM_uint32;
    stiov[1 as libc::c_int as usize].buffer.value =
        sign as *mut libc::c_char as *mut libc::c_void;
    stiov[1 as libc::c_int as usize].buffer.length = strlen(sign);
    stiov[2 as libc::c_int as usize].type_0 = 1 as libc::c_int as OM_uint32;
    major =
        gss_unwrap_iov(&mut minor, ctx2, &mut oconf, &mut qop,
                       stiov.as_mut_ptr(), 3 as libc::c_int);
    check_gsserr(b"gss_unwrap_iov(aead3)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if oconf != conf || qop != 0 as libc::c_int as libc::c_uint {
        errout(b"gss_unwrap_iov(aead3) conf/qop\x00" as *const u8 as
                   *const libc::c_char);
    }
    if stiov[2 as libc::c_int as usize].buffer.length != strlen(wrap) ||
           memcmp(stiov[2 as libc::c_int as usize].buffer.value,
                  wrap as *const libc::c_void, strlen(wrap)) !=
               0 as libc::c_int {
        errout(b"gss_unwrap_iov(aead3) decryption\x00" as *const u8 as
                   *const libc::c_char);
    }
    offset =
        (stiov[2 as libc::c_int as usize].buffer.value as
             *mut libc::c_char).wrapping_offset_from(fulltoken) as
            libc::c_long;
    if offset < 0 as libc::c_int as libc::c_long || offset as size_t > len {
        errout(b"gss_unwrap_iov(aead3) offset\x00" as *const u8 as
                   *const libc::c_char);
    }
    free(fulltoken as *mut libc::c_void);
    gss_release_iov_buffer(&mut minor, iov.as_mut_ptr(), 4 as libc::c_int);
    /* Wrap a token using gss_wrap_aead and unwrap it using a stream buffer
     * with allocation and copying. */
    input.value = wrap as *mut libc::c_char as *mut libc::c_void;
    input.length = strlen(wrap);
    assoc.value = sign as *mut libc::c_char as *mut libc::c_void;
    assoc.length = strlen(sign);
    major =
        gss_wrap_aead(&mut minor, ctx1, conf, 0 as libc::c_int as gss_qop_t,
                      &mut assoc, &mut input, &mut oconf, &mut output);
    check_gsserr(b"gss_wrap_aead(aead4)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if oconf != conf {
        errout(b"gss_wrap(aead4) conf\x00" as *const u8 as
                   *const libc::c_char);
    }
    stiov[0 as libc::c_int as usize].type_0 = 10 as libc::c_int as OM_uint32;
    stiov[0 as libc::c_int as usize].buffer = output;
    stiov[1 as libc::c_int as usize].type_0 = 11 as libc::c_int as OM_uint32;
    stiov[1 as libc::c_int as usize].buffer = assoc;
    stiov[2 as libc::c_int as usize].type_0 =
        (1 as libc::c_int | 0x10000 as libc::c_int) as OM_uint32;
    major =
        gss_unwrap_iov(&mut minor, ctx2, &mut oconf, &mut qop,
                       stiov.as_mut_ptr(), 3 as libc::c_int);
    check_gsserr(b"gss_unwrap_iov(aead4)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if stiov[2 as libc::c_int as usize].type_0 & 0xffff0000 as libc::c_uint &
           0x20000 as libc::c_int as libc::c_uint == 0 {
        errout(b"gss_unwrap_iov(aead4) allocated\x00" as *const u8 as
                   *const libc::c_char);
    }
    if oconf != conf || qop != 0 as libc::c_int as libc::c_uint {
        errout(b"gss_unwrap_iov(aead4) conf/qop\x00" as *const u8 as
                   *const libc::c_char);
    }
    if stiov[2 as libc::c_int as usize].buffer.length != strlen(wrap) ||
           memcmp(stiov[2 as libc::c_int as usize].buffer.value,
                  wrap as *const libc::c_void, strlen(wrap)) !=
               0 as libc::c_int {
        errout(b"gss_unwrap_iov(aead4) decryption\x00" as *const u8 as
                   *const libc::c_char);
    }
    gss_release_buffer(&mut minor, &mut output);
    gss_release_iov_buffer(&mut minor, stiov.as_mut_ptr(), 3 as libc::c_int);
}
/*
 * Get a MIC for sign1, sign2, and sign3 using the caller-provided array iov,
 * which must have space for four elements, and the caller-provided buffer
 * data, which must be big enough for the MIC.  If data is NULL, the library
 * will be asked to allocate the MIC buffer.  The MIC will be located in
 * iov[3].buffer.
 */
#[c2rust::src_loc = "361:1"]
unsafe extern "C" fn mic(mut ctx: gss_ctx_id_t,
                         mut sign1: *const libc::c_char,
                         mut sign2: *const libc::c_char,
                         mut sign3: *const libc::c_char,
                         mut iov: *mut gss_iov_buffer_desc,
                         mut data: *mut libc::c_char) {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut allocated: krb5_boolean = 0;
    /* Lay out iov array. */
    (*iov.offset(0 as libc::c_int as isize)).type_0 =
        1 as libc::c_int as OM_uint32;
    let ref mut fresh7 =
        (*iov.offset(0 as libc::c_int as isize)).buffer.value;
    *fresh7 = sign1 as *mut libc::c_char as *mut libc::c_void;
    (*iov.offset(0 as libc::c_int as isize)).buffer.length = strlen(sign1);
    (*iov.offset(1 as libc::c_int as isize)).type_0 =
        11 as libc::c_int as OM_uint32;
    let ref mut fresh8 =
        (*iov.offset(1 as libc::c_int as isize)).buffer.value;
    *fresh8 = sign2 as *mut libc::c_char as *mut libc::c_void;
    (*iov.offset(1 as libc::c_int as isize)).buffer.length = strlen(sign2);
    (*iov.offset(2 as libc::c_int as isize)).type_0 =
        11 as libc::c_int as OM_uint32;
    let ref mut fresh9 =
        (*iov.offset(2 as libc::c_int as isize)).buffer.value;
    *fresh9 = sign3 as *mut libc::c_char as *mut libc::c_void;
    (*iov.offset(2 as libc::c_int as isize)).buffer.length = strlen(sign3);
    (*iov.offset(3 as libc::c_int as isize)).type_0 =
        12 as libc::c_int as OM_uint32;
    if data.is_null() {
        /* Ask the library to allocate the MIC buffer. */
        let ref mut fresh10 = (*iov.offset(3 as libc::c_int as isize)).type_0;
        *fresh10 |= 0x10000 as libc::c_int as libc::c_uint
    } else {
        /* Get the MIC length and use the caller-provided buffer. */
        major =
            gss_get_mic_iov_length(&mut minor, ctx,
                                   0 as libc::c_int as gss_qop_t, iov,
                                   4 as libc::c_int);
        check_gsserr(b"gss_get_mic_iov_length\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        let ref mut fresh11 =
            (*iov.offset(3 as libc::c_int as isize)).buffer.value;
        *fresh11 = data as *mut libc::c_void
    }
    major =
        gss_get_mic_iov(&mut minor, ctx, 0 as libc::c_int as gss_qop_t, iov,
                        4 as libc::c_int);
    check_gsserr(b"gss_get_mic_iov\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    allocated =
        ((*iov.offset(3 as libc::c_int as isize)).type_0 &
             0xffff0000 as libc::c_uint &
             0x20000 as libc::c_int as libc::c_uint !=
             0 as libc::c_int as libc::c_uint) as libc::c_int as krb5_boolean;
    if allocated !=
           (data == 0 as *mut libc::c_void as *mut libc::c_char) as
               libc::c_int as libc::c_uint {
        errout(b"gss_get_mic_iov allocated\x00" as *const u8 as
                   *const libc::c_char);
    };
}
#[c2rust::src_loc = "396:1"]
unsafe extern "C" fn test_mic(mut ctx1: gss_ctx_id_t,
                              mut ctx2: gss_ctx_id_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut iov: [gss_iov_buffer_desc; 4] =
        [gss_iov_buffer_desc{type_0: 0,
                             buffer:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as
                                                         *mut libc::c_void,},};
            4];
    let mut qop: gss_qop_t = 0;
    let mut concatbuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut micbuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut sign1: *const libc::c_char =
        b"Data and sign-only \x00" as *const u8 as *const libc::c_char;
    let mut sign2: *const libc::c_char =
        b"buffers are treated \x00" as *const u8 as *const libc::c_char;
    let mut sign3: *const libc::c_char =
        b"equally by gss_get_mic_iov\x00" as *const u8 as *const libc::c_char;
    let mut concat: [libc::c_char; 1024] = [0; 1024];
    let mut data: [libc::c_char; 1024] = [0; 1024];
    snprintf(concat.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
             b"%s%s%s\x00" as *const u8 as *const libc::c_char, sign1, sign2,
             sign3);
    concatbuf.value = concat.as_mut_ptr() as *mut libc::c_void;
    concatbuf.length = strlen(concat.as_mut_ptr());
    /* MIC with a caller-provided buffer and verify with the IOV array. */
    mic(ctx1, sign1, sign2, sign3, iov.as_mut_ptr(), data.as_mut_ptr());
    major =
        gss_verify_mic_iov(&mut minor, ctx2, &mut qop, iov.as_mut_ptr(),
                           4 as libc::c_int);
    check_gsserr(b"gss_verify_mic_iov(mic1)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if qop != 0 as libc::c_int as libc::c_uint {
        errout(b"gss_verify_mic_iov(mic1) qop\x00" as *const u8 as
                   *const libc::c_char);
    }
    /* MIC with an allocated buffer and verify with gss_verify_mic. */
    mic(ctx1, sign1, sign2, sign3, iov.as_mut_ptr(), 0 as *mut libc::c_char);
    major =
        gss_verify_mic(&mut minor, ctx2, &mut concatbuf,
                       &mut (*iov.as_mut_ptr().offset(3 as libc::c_int as
                                                          isize)).buffer,
                       &mut qop);
    check_gsserr(b"gss_verify_mic(mic2)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if qop != 0 as libc::c_int as libc::c_uint {
        errout(b"gss_verify_mic(mic2) qop\x00" as *const u8 as
                   *const libc::c_char);
    }
    gss_release_iov_buffer(&mut minor, iov.as_mut_ptr(), 4 as libc::c_int);
    /* MIC with gss_c_get_mic and verify using the IOV array (which is still
     * mostly set up from the last call to mic(). */
    major =
        gss_get_mic(&mut minor, ctx1, 0 as libc::c_int as gss_qop_t,
                    &mut concatbuf, &mut micbuf);
    check_gsserr(b"gss_get_mic(mic3)\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    iov[3 as libc::c_int as usize].buffer = micbuf;
    major =
        gss_verify_mic_iov(&mut minor, ctx2, &mut qop, iov.as_mut_ptr(),
                           4 as libc::c_int);
    check_gsserr(b"gss_verify_mic_iov(mic3)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if qop != 0 as libc::c_int as libc::c_uint {
        errout(b"gss_verify_mic_iov(mic3) qop\x00" as *const u8 as
                   *const libc::c_char);
    }
    gss_release_buffer(&mut minor, &mut micbuf);
}
/* Create a DCE-style token and make sure we can unwrap it. */
#[c2rust::src_loc = "440:1"]
unsafe extern "C" fn test_dce(mut ctx1: gss_ctx_id_t, mut ctx2: gss_ctx_id_t,
                              mut conf: libc::c_int) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut iov: [gss_iov_buffer_desc; 4] =
        [gss_iov_buffer_desc{type_0: 0,
                             buffer:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as
                                                         *mut libc::c_void,},};
            4];
    let mut qop: gss_qop_t = 0;
    let mut sign1: *const libc::c_char =
        b"First data to be signed\x00" as *const u8 as *const libc::c_char;
    let mut sign2: *const libc::c_char =
        b"Second data to be signed\x00" as *const u8 as *const libc::c_char;
    let mut wrap: *const libc::c_char =
        b"This data must align to 16 bytes\x00" as *const u8 as
            *const libc::c_char;
    let mut oconf: libc::c_int = 0;
    let mut data: [libc::c_char; 1024] = [0; 1024];
    /* Wrap a SIGN_ONLY_1 | DATA | SIGN_ONLY_2 | HEADER token. */
    memcpy(data.as_mut_ptr() as *mut libc::c_void,
           wrap as *const libc::c_void,
           strlen(wrap).wrapping_add(1 as libc::c_int as libc::c_ulong));
    iov[0 as libc::c_int as usize].type_0 = 11 as libc::c_int as OM_uint32;
    iov[0 as libc::c_int as usize].buffer.value =
        sign1 as *mut libc::c_char as *mut libc::c_void;
    iov[0 as libc::c_int as usize].buffer.length = strlen(sign1);
    iov[1 as libc::c_int as usize].type_0 = 1 as libc::c_int as OM_uint32;
    iov[1 as libc::c_int as usize].buffer.value =
        data.as_mut_ptr() as *mut libc::c_void;
    iov[1 as libc::c_int as usize].buffer.length = strlen(wrap);
    iov[2 as libc::c_int as usize].type_0 = 11 as libc::c_int as OM_uint32;
    iov[2 as libc::c_int as usize].buffer.value =
        sign2 as *mut libc::c_char as *mut libc::c_void;
    iov[2 as libc::c_int as usize].buffer.length = strlen(sign2);
    iov[3 as libc::c_int as usize].type_0 =
        (2 as libc::c_int | 0x10000 as libc::c_int) as OM_uint32;
    major =
        gss_wrap_iov(&mut minor, ctx1, conf, 0 as libc::c_int as gss_qop_t,
                     &mut oconf, iov.as_mut_ptr(), 4 as libc::c_int);
    check_gsserr(b"gss_wrap_iov(dce)\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    if oconf != conf {
        errout(b"gss_wrap_iov(dce) conf\x00" as *const u8 as
                   *const libc::c_char);
    }
    if iov[0 as libc::c_int as usize].buffer.value !=
           sign1 as *mut libc::c_void ||
           iov[0 as libc::c_int as usize].buffer.length != strlen(sign1) {
        errout(b"gss_wrap_iov(dce) sign1 buffer\x00" as *const u8 as
                   *const libc::c_char);
    }
    if iov[1 as libc::c_int as usize].buffer.value !=
           data.as_mut_ptr() as *mut libc::c_void ||
           iov[1 as libc::c_int as usize].buffer.length != strlen(wrap) {
        errout(b"gss_wrap_iov(dce) data buffer\x00" as *const u8 as
                   *const libc::c_char);
    }
    if iov[2 as libc::c_int as usize].buffer.value !=
           sign2 as *mut libc::c_void ||
           iov[2 as libc::c_int as usize].buffer.length != strlen(sign2) {
        errout(b"gss_wrap_iov(dce) sign2 buffer\x00" as *const u8 as
                   *const libc::c_char);
    }
    check_encrypted(b"gss_wrap_iov(dce) encryption\x00" as *const u8 as
                        *const libc::c_char, conf, data.as_mut_ptr(), wrap);
    /* Make sure we can unwrap it. */
    major =
        gss_unwrap_iov(&mut minor, ctx2, &mut oconf, &mut qop,
                       iov.as_mut_ptr(), 4 as libc::c_int);
    check_gsserr(b"gss_unwrap_iov(std1)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if oconf != conf || qop != 0 as libc::c_int as libc::c_uint {
        errout(b"gss_unwrap_iov(std1) conf/qop\x00" as *const u8 as
                   *const libc::c_char);
    }
    if iov[0 as libc::c_int as usize].buffer.value !=
           sign1 as *mut libc::c_void ||
           iov[0 as libc::c_int as usize].buffer.length != strlen(sign1) {
        errout(b"gss_unwrap_iov(dce) sign1 buffer\x00" as *const u8 as
                   *const libc::c_char);
    }
    if iov[1 as libc::c_int as usize].buffer.value !=
           data.as_mut_ptr() as *mut libc::c_void ||
           iov[1 as libc::c_int as usize].buffer.length != strlen(wrap) {
        errout(b"gss_unwrap_iov(dce) data buffer\x00" as *const u8 as
                   *const libc::c_char);
    }
    if iov[2 as libc::c_int as usize].buffer.value !=
           sign2 as *mut libc::c_void ||
           iov[2 as libc::c_int as usize].buffer.length != strlen(sign2) {
        errout(b"gss_unwrap_iov(dce) sign2 buffer\x00" as *const u8 as
                   *const libc::c_char);
    }
    if memcmp(data.as_mut_ptr() as *const libc::c_void,
              wrap as *const libc::c_void,
              iov[1 as libc::c_int as usize].buffer.length) !=
           0 as libc::c_int {
        errout(b"gss_unwrap_iov(dce) decryption\x00" as *const u8 as
                   *const libc::c_char);
    }
    gss_release_iov_buffer(&mut minor, iov.as_mut_ptr(), 4 as libc::c_int);
}
#[c2rust::src_loc = "493:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    let mut mech: gss_OID = &mut mech_krb5;
    let mut tname: gss_name_t = 0 as *mut gss_name_struct;
    let mut ictx: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut actx: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    /* Parse arguments. */
    argv = argv.offset(1);
    if !(*argv).is_null() &&
           strcmp(*argv, b"-s\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        mech = &mut mech_spnego;
        argv = argv.offset(1)
    }
    if (*argv).is_null() ||
           !(*argv.offset(1 as libc::c_int as isize)).is_null() {
        errout(b"Usage: t_iov [-s] targetname\x00" as *const u8 as
                   *const libc::c_char);
    }
    tname = import_name(*argv);
    flags =
        (4 as libc::c_int | 8 as libc::c_int | 2 as libc::c_int) as OM_uint32;
    establish_contexts(mech, 0 as gss_cred_id_t, 0 as gss_cred_id_t, tname,
                       flags, &mut ictx, &mut actx, 0 as *mut gss_name_t,
                       0 as *mut gss_OID, 0 as *mut gss_cred_id_t);
    /* Test standard token wrapping and unwrapping in both directions, with and
     * without confidentiality. */
    test_standard_wrap(ictx, actx, 0 as libc::c_int);
    test_standard_wrap(ictx, actx, 1 as libc::c_int);
    test_standard_wrap(actx, ictx, 0 as libc::c_int);
    test_standard_wrap(actx, ictx, 1 as libc::c_int);
    /* Test AEAD wrapping. */
    test_aead(ictx, actx, 0 as libc::c_int);
    test_aead(ictx, actx, 1 as libc::c_int);
    test_aead(actx, ictx, 0 as libc::c_int);
    test_aead(actx, ictx, 1 as libc::c_int);
    /* Test MIC tokens. */
    test_mic(ictx, actx);
    test_mic(actx, ictx);
    /* Test DCE wrapping with DCE-style contexts. */
    gss_delete_sec_context(&mut minor, &mut ictx, 0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
    flags =
        (4 as libc::c_int | 8 as libc::c_int | 0x1000 as libc::c_int) as
            OM_uint32;
    establish_contexts(mech, 0 as gss_cred_id_t, 0 as gss_cred_id_t, tname,
                       flags, &mut ictx, &mut actx, 0 as *mut gss_name_t,
                       0 as *mut gss_OID, 0 as *mut gss_cred_id_t);
    test_dce(ictx, actx, 0 as libc::c_int);
    test_dce(ictx, actx, 1 as libc::c_int);
    test_dce(actx, ictx, 0 as libc::c_int);
    test_dce(actx, ictx, 1 as libc::c_int);
    gss_release_name(&mut minor, &mut tname);
    gss_delete_sec_context(&mut minor, &mut ictx, 0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
