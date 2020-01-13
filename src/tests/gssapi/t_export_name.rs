use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:40"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:40"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:40"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:40"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:44"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:44"]
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
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\x01\x05\x06\x04"},
 * corresponding to an object-identifier value of
 * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
 * 6(nametypes), 4(gss-api-exported-name)}.  The constant
 * GSS_C_NT_EXPORT_NAME should be initialized to point
 * to that gss_OID_desc.
 */
        #[no_mangle]
        #[c2rust::src_loc = "417:27"]
        pub static mut GSS_C_NT_EXPORT_NAME: gss_OID;
        /* output_name_type */
        #[no_mangle]
        #[c2rust::src_loc = "562:1"]
        pub fn gss_import_name(_: *mut OM_uint32, _: gss_buffer_t, _: gss_OID,
                               _: *mut gss_name_t) -> OM_uint32;
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
        /* qop_state */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "770:1"]
        pub fn gss_export_name(_: *mut OM_uint32, _: gss_name_t,
                               _: gss_buffer_t) -> OM_uint32;
        /* dest_name */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "784:1"]
        pub fn gss_canonicalize_name(_: *mut OM_uint32, _: gss_name_t,
                                     _: gss_OID, _: *mut gss_name_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:44"]
pub mod krb5_h {
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdio.h:40"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:41"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:42"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:44"]
pub mod gssapi_ext_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, OM_uint32,
                          gss_name_struct, gss_name_t, gss_buffer_desc_struct,
                          gss_buffer_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "434:27"]
        pub static mut GSS_C_NT_COMPOSITE_EXPORT: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "481:1"]
        pub fn gss_export_name_composite(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t) -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:44"]
pub mod common_h {
    use super::FILE_h::FILE;
    use super::gssapi_h::{gss_buffer_desc_struct, gss_buffer_t, gss_name_t,
                          OM_uint32, gss_OID_desc};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/common.h - Declarations for GSSAPI test utility functions */
/*
 * Copyright (C) 2012 by the Massachusetts Institute of Technology.
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
        /* Display an error message (containing msg) and exit if major is an error. */
        /* Display an error message (containing msg) and exit if code is an error. */
        /* Display an error message containing msg and exit. */
        /* Import a GSSAPI name based on a string of the form 'u:username',
 * 'p:principalname', or 'h:host@service' (or just 'h:service'). */
        /* Establish contexts using gss_init_sec_context and gss_accept_sec_context. */
        /* Export *cred to a token, then release *cred and replace it by re-importing
 * the token. */
        /* Display name as canonicalized to mech, preceded by tag. */
        /* Display oid in printable form, preceded by tag (if not NULL). */
        /* Display attributes of name, including hex value if noisy is true. */
        /* Display the contents of buf to fp in hex, followed by a newline. */
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn print_hex(fp: *mut FILE, buf: gss_buffer_t);
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:44"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "81:33"]
        pub static gss_mech_krb5: gss_OID;
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
pub use self::gssapi_h::{gss_name_t, gss_uint32, OM_uint32,
                         gss_OID_desc_struct, gss_OID_desc, gss_OID,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_name_struct, GSS_C_NT_EXPORT_NAME,
                         gss_import_name, gss_release_name,
                         gss_release_buffer, gss_export_name,
                         gss_canonicalize_name};
pub use self::krb5_h::krb5_boolean;
use self::stdio_h::{stdout, stderr, fprintf};
use self::stdlib_h::exit;
use self::string_h::memcmp;
use self::gssapi_ext_h::{GSS_C_NT_COMPOSITE_EXPORT,
                         gss_export_name_composite};
use self::common_h::{print_hex, import_name, check_gsserr, mech_spnego,
                     mech_krb5};
use self::gssapi_krb5_h::gss_mech_krb5;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/t_export_name.c - Test program for gss_export_name behavior */
/*
 * Copyright 2012 by the Massachusetts Institute of Technology.
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
 * Test program for gss_export_name, intended to be run from a Python test
 * script.  Imports a name, canonicalizes it to a mech, exports it,
 * re-imports/exports it to compare results, and then prints the hex form of
 * the exported name followed by a newline.
 *
 * Usage: ./t_export_name [-k|-s] user:username|krb5:princ|host:service@host
 *
 * The name is imported as a username, krb5 principal, or hostbased name.
 * By default or with -k, the name is canonicalized to the krb5 mech; -s
 * indicates SPNEGO instead.
 */
#[c2rust::src_loc = "46:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"Usage: t_export_name [-k|-s] name\n\x00" as *const u8 as
                *const libc::c_char);
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "53:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut mech: gss_OID = gss_mech_krb5;
    let mut name: gss_name_t = 0 as *mut gss_name_struct;
    let mut mechname: gss_name_t = 0 as *mut gss_name_struct;
    let mut impname: gss_name_t = 0 as *mut gss_name_struct;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut buf2: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut use_composite: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut ntype: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut name_arg: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt: libc::c_char = 0;
    /* Parse arguments. */
    while argc > 1 as libc::c_int &&
              *(*argv.offset(1 as libc::c_int as
                                 isize)).offset(0 as libc::c_int as isize) as
                  libc::c_int == '-' as i32 {
        opt =
            *(*argv.offset(1 as libc::c_int as
                               isize)).offset(1 as libc::c_int as isize);
        argc -= 1;
        argv = argv.offset(1);
        if opt as libc::c_int == 'k' as i32 {
            mech = &mut mech_krb5
        } else if opt as libc::c_int == 's' as i32 {
            mech = &mut mech_spnego
        } else if opt as libc::c_int == 'c' as i32 {
            use_composite = 1 as libc::c_int as krb5_boolean
        } else { usage(); }
    }
    if argc != 2 as libc::c_int { usage(); }
    name_arg = *argv.offset(1 as libc::c_int as isize);
    /* Import the name. */
    name = import_name(name_arg);
    /* Canonicalize and export the name. */
    major = gss_canonicalize_name(&mut minor, name, mech, &mut mechname);
    check_gsserr(b"gss_canonicalize_name\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if use_composite != 0 {
        major = gss_export_name_composite(&mut minor, mechname, &mut buf)
    } else { major = gss_export_name(&mut minor, mechname, &mut buf) }
    check_gsserr(b"gss_export_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    /* Import and re-export the name, and compare the results. */
    ntype =
        if use_composite != 0 {
            GSS_C_NT_COMPOSITE_EXPORT
        } else { GSS_C_NT_EXPORT_NAME };
    major = gss_import_name(&mut minor, &mut buf, ntype, &mut impname);
    check_gsserr(b"gss_import_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    if use_composite != 0 {
        major = gss_export_name_composite(&mut minor, impname, &mut buf2)
    } else { major = gss_export_name(&mut minor, impname, &mut buf2) }
    check_gsserr(b"gss_export_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    if buf.length != buf2.length ||
           memcmp(buf.value, buf2.value, buf.length) != 0 as libc::c_int {
        fprintf(stderr,
                b"Mismatched results:\n\x00" as *const u8 as
                    *const libc::c_char);
        print_hex(stderr, &mut buf);
        print_hex(stderr, &mut buf2);
        return 1 as libc::c_int
    }
    print_hex(stdout, &mut buf);
    gss_release_name(&mut minor, &mut name);
    gss_release_name(&mut minor, &mut mechname);
    gss_release_name(&mut minor, &mut impname);
    gss_release_buffer(&mut minor, &mut buf);
    gss_release_buffer(&mut minor, &mut buf2);
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
