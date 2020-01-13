use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:31"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:31"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:34"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:34"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
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
        /* name_equal */
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
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
        /* oid */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "707:1"]
        pub fn gss_oid_to_str(_: *mut OM_uint32, _: gss_OID, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:34"]
pub mod common_h {
    use super::gssapi_h::{gss_name_t, OM_uint32};
    extern "C" {
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
    }
    /* COMMON_H */
}
pub use self::stddef_h::size_t;
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_name_t, gss_uint32, OM_uint32,
                         gss_OID_desc_struct, gss_OID, gss_buffer_desc_struct,
                         gss_buffer_desc, gss_buffer_t, gss_name_struct,
                         gss_display_name, gss_release_name,
                         gss_release_buffer, gss_oid_to_str};
use self::string_h::{memcmp, strlen};
use self::common_h::{import_name, errout, check_gsserr};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1996, Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */
/*
 * Simple test program for testing how GSSAPI import name works.  (May
 * be made into a more full-fledged test program later.)
 */
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn oid_str(mut type_0: libc::c_char)
 -> *const libc::c_char {
    match type_0 as libc::c_int {
        112 => {
            /* GSS_KRB5_NT_PRINCIPAL_NAME */
            return b"{ 1 2 840 113554 1 2 2 1 }\x00" as *const u8 as
                       *const libc::c_char
        }
        101 => {
            /* GSS_KRB5_NT_ENTERPRISE_NAME */
            return b"{ 1 2 840 113554 1 2 2 6 }\x00" as *const u8 as
                       *const libc::c_char
        }
        104 => {
            /* GSS_C_NT_HOSTBASED_SERVICE */
            return b"{ 1 2 840 113554 1 2 1 4 }\x00" as *const u8 as
                       *const libc::c_char
        }
        _ => { }
    }
    return b"no_oid\x00" as *const u8 as *const libc::c_char;
}
/* Return true if buf has the same contents as str, plus a zero byte if
 * indicated by buf_includes_nullterm. */
#[c2rust::src_loc = "52:1"]
unsafe extern "C" fn buf_eq_str(mut buf: gss_buffer_t,
                                mut str: *const libc::c_char,
                                mut buf_includes_nullterm: libc::c_int)
 -> libc::c_int {
    let mut len: size_t =
        strlen(str).wrapping_add((if buf_includes_nullterm != 0 {
                                      1 as libc::c_int
                                  } else { 0 as libc::c_int }) as
                                     libc::c_ulong);
    return ((*buf).length == len &&
                memcmp((*buf).value, str as *const libc::c_void, len) ==
                    0 as libc::c_int) as libc::c_int;
}
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn test_import_name(mut name: *const libc::c_char) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut gss_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut name_oid: gss_OID = 0 as *mut gss_OID_desc_struct;
    gss_name = import_name(name);
    major = gss_display_name(&mut minor, gss_name, &mut buf, &mut name_oid);
    check_gsserr(b"gss_display_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    if buf_eq_str(&mut buf, name.offset(2 as libc::c_int as isize),
                  0 as libc::c_int) == 0 {
        errout(b"wrong name string\x00" as *const u8 as *const libc::c_char);
    }
    gss_release_buffer(&mut minor, &mut buf);
    major = gss_oid_to_str(&mut minor, name_oid, &mut buf);
    check_gsserr(b"gss_oid_to_str\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    if buf_eq_str(&mut buf, oid_str(*name), 1 as libc::c_int) == 0 {
        errout(b"wrong name type\x00" as *const u8 as *const libc::c_char);
    }
    gss_release_buffer(&mut minor, &mut buf);
    gss_release_name(&mut minor, &mut gss_name);
}
#[c2rust::src_loc = "84:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    test_import_name(b"p:user@MIT.EDU\x00" as *const u8 as
                         *const libc::c_char);
    test_import_name(b"e:enterprise@mit.edu@MIT.EDU\x00" as *const u8 as
                         *const libc::c_char);
    test_import_name(b"h:HOST@dc1.mit.edu\x00" as *const u8 as
                         *const libc::c_char);
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
