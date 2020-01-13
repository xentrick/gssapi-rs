use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:38"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:38"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:38"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:38"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:43"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:43"]
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
        /* cred_handle */
        #[no_mangle]
        #[c2rust::src_loc = "437:1"]
        pub fn gss_init_sec_context(_: *mut OM_uint32, _: gss_cred_id_t,
                                    _: *mut gss_ctx_id_t, _: gss_name_t,
                                    _: gss_OID, _: OM_uint32, _: OM_uint32,
                                    _: gss_channel_bindings_t,
                                    _: gss_buffer_t, _: *mut gss_OID,
                                    _: gss_buffer_t, _: *mut OM_uint32,
                                    _: *mut OM_uint32) -> OM_uint32;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn gss_accept_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_cred_id_t, _: gss_buffer_t,
                                      _: gss_channel_bindings_t,
                                      _: *mut gss_name_t, _: *mut gss_OID,
                                      _: gss_buffer_t, _: *mut OM_uint32,
                                      _: *mut OM_uint32,
                                      _: *mut gss_cred_id_t) -> OM_uint32;
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* qop_state */
        #[no_mangle]
        #[c2rust::src_loc = "530:1"]
        pub fn gss_display_status(_: *mut OM_uint32, _: OM_uint32,
                                  _: libc::c_int, _: gss_OID,
                                  _: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
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
#[c2rust::header_src = "/usr/include/stdio.h:38"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:39"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:40"]
pub mod string_h {
    extern "C" {
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
#[c2rust::header_src = "/usr/include/assert.h:41"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:43"]
pub mod common_h {
    use super::gssapi_h::{gss_name_t, OM_uint32, gss_OID_desc};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn import_name(str: *const libc::c_char) -> gss_name_t;
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
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint32_t, __off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         gss_init_sec_context, gss_accept_sec_context,
                         gss_delete_sec_context, gss_display_status,
                         gss_release_name, gss_release_buffer};
use self::stdio_h::{stderr, fprintf};
use self::stdlib_h::exit;
use self::string_h::{memcmp, strcmp, strlen};
use self::assert_h::__assert_fail;
use self::common_h::{import_name, check_gsserr, mech_spnego, mech_krb5};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/t_err.c - Test accept_sec_context error generation */
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
/*
 * This test program verifies that the krb5 gss_accept_sec_context can produce
 * error tokens and that gss_init_sec_context can interpret them.
 */
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn check_replay_error(mut msg: *const libc::c_char,
                                        mut major: OM_uint32,
                                        mut minor: OM_uint32) {
    let mut tmpmin: OM_uint32 = 0;
    let mut msg_ctx: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut replay: *const libc::c_char =
        b"Request is a replay\x00" as *const u8 as *const libc::c_char;
    let mut m: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    if major != (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        fprintf(stderr,
                b"%s: expected major code GSS_S_FAILURE\n\x00" as *const u8 as
                    *const libc::c_char, msg);
        check_gsserr(msg, major, minor);
        exit(1 as libc::c_int);
    }
    gss_display_status(&mut tmpmin, minor, 2 as libc::c_int, 0 as gss_OID,
                       &mut msg_ctx, &mut m);
    if m.length != strlen(replay) ||
           memcmp(m.value, replay as *const libc::c_void, m.length) !=
               0 as libc::c_int {
        fprintf(stderr,
                b"%s: expected replay error; got %.*s\n\x00" as *const u8 as
                    *const libc::c_char, msg, m.length as libc::c_int,
                m.value as *mut libc::c_char);
        exit(1 as libc::c_int);
    }
    gss_release_buffer(&mut tmpmin, &mut m);
}
#[c2rust::src_loc = "68:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    let mut mech: gss_OID = &mut mech_krb5;
    let mut tname: gss_name_t = 0 as *mut gss_name_struct;
    let mut itok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut atok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut ictx: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut actx: gss_ctx_id_t = 0 as gss_ctx_id_t;
    argv = argv.offset(1);
    if !(*argv).is_null() &&
           strcmp(*argv, b"--spnego\x00" as *const u8 as *const libc::c_char)
               == 0 as libc::c_int {
        mech = &mut mech_spnego;
        argv = argv.offset(1)
    }
    if (*argv).is_null() ||
           !(*argv.offset(1 as libc::c_int as isize)).is_null() {
        fprintf(stderr,
                b"Usage: t_err targetname\n\x00" as *const u8 as
                    *const libc::c_char);
        return 1 as libc::c_int
    }
    tname = import_name(*argv);
    /* Get the initial context token. */
    flags =
        (4 as libc::c_int | 8 as libc::c_int | 2 as libc::c_int) as OM_uint32;
    major =
        gss_init_sec_context(&mut minor, 0 as gss_cred_id_t, &mut ictx, tname,
                             mech, flags,
                             0xffffffff as libc::c_ulong as OM_uint32,
                             0 as gss_channel_bindings_t, 0 as gss_buffer_t,
                             0 as *mut gss_OID, &mut itok,
                             0 as *mut OM_uint32, 0 as *mut OM_uint32);
    check_gsserr(b"gss_init_sec_context(1)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if major ==
           ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int) as
               libc::c_uint {
    } else {
        __assert_fail(b"major == GSS_S_CONTINUE_NEEDED\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_err.c\x00" as *const u8 as *const libc::c_char,
                      95 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"int main(int, char **)\x00")).as_ptr());
    }
    /* Process this token into an acceptor context, then discard it. */
    major =
        gss_accept_sec_context(&mut minor, &mut actx, 0 as gss_cred_id_t,
                               &mut itok, 0 as gss_channel_bindings_t,
                               0 as *mut gss_name_t, 0 as *mut gss_OID,
                               &mut atok, 0 as *mut OM_uint32,
                               0 as *mut OM_uint32, 0 as *mut gss_cred_id_t);
    check_gsserr(b"gss_accept_sec_context(1)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    gss_release_buffer(&mut minor, &mut atok);
    gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
    /* Process the same token again, producing a replay error. */
    major =
        gss_accept_sec_context(&mut minor, &mut actx, 0 as gss_cred_id_t,
                               &mut itok, 0 as gss_channel_bindings_t,
                               0 as *mut gss_name_t, 0 as *mut gss_OID,
                               &mut atok, 0 as *mut OM_uint32,
                               0 as *mut OM_uint32, 0 as *mut gss_cred_id_t);
    check_replay_error(b"gss_accept_sec_context(2)\x00" as *const u8 as
                           *const libc::c_char, major, minor);
    if atok.length != 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"atok.length != 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_err.c\x00" as *const u8 as *const libc::c_char,
                      110 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"int main(int, char **)\x00")).as_ptr());
    }
    /* Send the error token back the initiator. */
    gss_release_buffer(&mut minor, &mut itok);
    major =
        gss_init_sec_context(&mut minor, 0 as gss_cred_id_t, &mut ictx, tname,
                             mech, flags,
                             0xffffffff as libc::c_ulong as OM_uint32,
                             0 as gss_channel_bindings_t, &mut atok,
                             0 as *mut gss_OID, &mut itok,
                             0 as *mut OM_uint32, 0 as *mut OM_uint32);
    check_replay_error(b"gss_init_sec_context(2)\x00" as *const u8 as
                           *const libc::c_char, major, minor);
    gss_release_name(&mut minor, &mut tname);
    gss_release_buffer(&mut minor, &mut itok);
    gss_release_buffer(&mut minor, &mut atok);
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
