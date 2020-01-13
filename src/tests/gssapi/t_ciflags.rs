use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:33"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:33"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
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
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
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
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* output_name */
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        /* mechanisms */
        /* Last argument new for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "594:1"]
        pub fn gss_inquire_context(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: *mut gss_name_t, _: *mut gss_name_t,
                                   _: *mut OM_uint32, _: *mut gss_OID,
                                   _: *mut OM_uint32, _: *mut libc::c_int,
                                   _: *mut libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "421:1"]
        pub fn gss_acquire_cred(_: *mut OM_uint32, _: gss_name_t,
                                _: OM_uint32, _: gss_OID_set,
                                _: gss_cred_usage_t, _: *mut gss_cred_id_t,
                                _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
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
#[c2rust::header_src = "/usr/include/assert.h:35"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:37"]
pub mod gssapi_ext_h {
    use super::gssapi_h::{OM_uint32, gss_cred_id_t, gss_OID_desc_struct,
                          gss_OID, gss_buffer_desc_struct, gss_buffer_t};
    extern "C" {
        /*
 * Heimdal extension
 */
        #[no_mangle]
        #[c2rust::src_loc = "188:1"]
        pub fn gss_set_cred_option(_: *mut OM_uint32, _: *mut gss_cred_id_t,
                                   _: gss_OID, _: gss_buffer_t) -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:37"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "102:33"]
        pub static GSS_KRB5_CRED_NO_CI_FLAGS_X: gss_OID;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint32_t, __off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_cred_usage_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         gss_release_name, gss_inquire_context,
                         gss_delete_sec_context, gss_release_cred,
                         gss_acquire_cred};
use self::stdio_h::{stderr, fprintf};
use self::assert_h::__assert_fail;
use self::gssapi_ext_h::gss_set_cred_option;
use self::common_h::{establish_contexts, import_name, check_gsserr,
                     mech_spnego, mech_krb5};
use self::gssapi_krb5_h::GSS_KRB5_CRED_NO_CI_FLAGS_X;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/t_ciflags.c - GSS_KRB5_CRED_NO_CI_FLAGS_X tests */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn flagtest(mut mech: gss_OID, mut icred: gss_cred_id_t,
                              mut tname: gss_name_t, mut inflags: OM_uint32,
                              mut expflags: OM_uint32) {
    let mut ictx: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut actx: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    establish_contexts(mech, icred, 0 as gss_cred_id_t, tname, inflags,
                       &mut ictx, &mut actx, 0 as *mut gss_name_t,
                       0 as *mut gss_OID, 0 as *mut gss_cred_id_t);
    major =
        gss_inquire_context(&mut minor, actx, 0 as *mut gss_name_t,
                            0 as *mut gss_name_t, 0 as *mut OM_uint32,
                            0 as *mut gss_OID, &mut flags,
                            0 as *mut libc::c_int, 0 as *mut libc::c_int);
    check_gsserr(b"gss_inquire_context\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if flags == expflags {
    } else {
        __assert_fail(b"flags == expflags\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_ciflags.c\x00" as *const u8 as *const libc::c_char,
                      52 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[libc::c_char; 72]>(b"void flagtest(gss_OID, gss_cred_id_t, gss_name_t, OM_uint32, OM_uint32)\x00")).as_ptr());
    }
    gss_delete_sec_context(&mut minor, &mut ictx, 0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
}
#[c2rust::src_loc = "58:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut icred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut tname: gss_name_t = 0 as *mut gss_name_struct;
    let mut empty_buffer: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    if argc != 2 as libc::c_int {
        fprintf(stderr,
                b"Usage: %s targetname\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        return 1 as libc::c_int
    }
    tname = import_name(*argv.offset(1 as libc::c_int as isize));
    /* With no flags, the initiator asserts conf, integ, trans */
    flagtest(&mut mech_krb5, 0 as gss_cred_id_t, tname,
             0 as libc::c_int as OM_uint32,
             (16 as libc::c_int | 32 as libc::c_int | 256 as libc::c_int) as
                 OM_uint32);
    flagtest(&mut mech_spnego, 0 as gss_cred_id_t, tname,
             0 as libc::c_int as OM_uint32,
             (16 as libc::c_int | 32 as libc::c_int | 256 as libc::c_int) as
                 OM_uint32);
    /* The initiator also asserts most flags specified by the caller. */
    flagtest(&mut mech_krb5, 0 as gss_cred_id_t, tname,
             8 as libc::c_int as OM_uint32,
             (16 as libc::c_int | 32 as libc::c_int | 256 as libc::c_int |
                  8 as libc::c_int) as OM_uint32);
    flagtest(&mut mech_spnego, 0 as gss_cred_id_t, tname,
             8 as libc::c_int as OM_uint32,
             (16 as libc::c_int | 32 as libc::c_int | 256 as libc::c_int |
                  8 as libc::c_int) as OM_uint32);
    /* Get a normal initiator cred and re-test with no flags. */
    major =
        gss_acquire_cred(&mut minor, 0 as gss_name_t,
                         0xffffffff as libc::c_ulong as OM_uint32,
                         0 as gss_OID_set, 1 as libc::c_int, &mut icred,
                         0 as *mut gss_OID_set, 0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    flagtest(&mut mech_krb5, icred, tname, 0 as libc::c_int as OM_uint32,
             (16 as libc::c_int | 32 as libc::c_int | 256 as libc::c_int) as
                 OM_uint32);
    flagtest(&mut mech_spnego, icred, tname, 0 as libc::c_int as OM_uint32,
             (16 as libc::c_int | 32 as libc::c_int | 256 as libc::c_int) as
                 OM_uint32);
    /* Suppress confidentiality and integrity flags on the initiator cred and
     * check that they are suppressed, but can still be asserted explicitly. */
    major =
        gss_set_cred_option(&mut minor, &mut icred,
                            GSS_KRB5_CRED_NO_CI_FLAGS_X, &mut empty_buffer);
    check_gsserr(b"gss_set_cred_option\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    flagtest(&mut mech_krb5, icred, tname, 0 as libc::c_int as OM_uint32,
             256 as libc::c_int as OM_uint32);
    flagtest(&mut mech_krb5, icred, tname, 16 as libc::c_int as OM_uint32,
             (16 as libc::c_int | 256 as libc::c_int) as OM_uint32);
    flagtest(&mut mech_krb5, icred, tname, 32 as libc::c_int as OM_uint32,
             (32 as libc::c_int | 256 as libc::c_int) as OM_uint32);
    flagtest(&mut mech_krb5, icred, tname,
             (16 as libc::c_int | 32 as libc::c_int) as OM_uint32,
             (16 as libc::c_int | 32 as libc::c_int | 256 as libc::c_int) as
                 OM_uint32);
    flagtest(&mut mech_spnego, icred, tname, 0 as libc::c_int as OM_uint32,
             256 as libc::c_int as OM_uint32);
    flagtest(&mut mech_spnego, icred, tname, 32 as libc::c_int as OM_uint32,
             (32 as libc::c_int | 256 as libc::c_int) as OM_uint32);
    flagtest(&mut mech_spnego, icred, tname, 16 as libc::c_int as OM_uint32,
             (16 as libc::c_int | 256 as libc::c_int) as OM_uint32);
    flagtest(&mut mech_spnego, icred, tname,
             (16 as libc::c_int | 32 as libc::c_int) as OM_uint32,
             (16 as libc::c_int | 32 as libc::c_int | 256 as libc::c_int) as
                 OM_uint32);
    gss_release_name(&mut minor, &mut tname);
    gss_release_cred(&mut minor, &mut icred);
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
