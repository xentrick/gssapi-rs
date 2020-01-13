use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:26"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:26"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:26"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:30"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:30"]
pub mod gssapi_h {
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
    /* OM_STRING */
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
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        /* status_string */
        #[no_mangle]
        #[c2rust::src_loc = "540:1"]
        pub fn gss_indicate_mechs(_: *mut OM_uint32, _: *mut gss_OID_set)
         -> OM_uint32;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* buffer */
        #[no_mangle]
        #[c2rust::src_loc = "579:1"]
        pub fn gss_release_oid_set(_: *mut OM_uint32, _: *mut gss_OID_set)
         -> OM_uint32;
        /* oid */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "707:1"]
        pub fn gss_oid_to_str(_: *mut OM_uint32, _: gss_OID, _: gss_buffer_t)
         -> OM_uint32;
        /* mechs */
        #[no_mangle]
        #[c2rust::src_loc = "861:1"]
        pub fn gss_inquire_attrs_for_mech(_: *mut OM_uint32, _: gss_const_OID,
                                          _: *mut gss_OID_set,
                                          _: *mut gss_OID_set) -> OM_uint32;
        /* known_mech_attrs */
        #[no_mangle]
        #[c2rust::src_loc = "868:1"]
        pub fn gss_display_mech_attr(_: *mut OM_uint32, _: gss_const_OID,
                                     _: gss_buffer_t, _: gss_buffer_t,
                                     _: gss_buffer_t) -> OM_uint32;
        /*
 * RFC 5801
 */
        #[no_mangle]
        #[c2rust::src_loc = "907:1"]
        pub fn gss_inquire_saslname_for_mech(_: *mut OM_uint32, _: gss_OID,
                                             _: gss_buffer_t, _: gss_buffer_t,
                                             _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "916:1"]
        pub fn gss_inquire_mech_for_saslname(_: *mut OM_uint32,
                                             _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:26"]
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
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:30"]
pub mod common_h {
    use super::gssapi_h::OM_uint32;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "46:1"]
        pub fn check_gsserr(msg: *const libc::c_char, major: OM_uint32,
                            minor: OM_uint32);
    }
    /* COMMON_H */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint32_t, __off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_uint32, OM_uint32, gss_OID_desc_struct,
                         gss_OID_desc, gss_OID, gss_OID_set_desc_struct,
                         gss_OID_set, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_const_OID, gss_indicate_mechs,
                         gss_release_buffer, gss_release_oid_set,
                         gss_oid_to_str, gss_inquire_attrs_for_mech,
                         gss_display_mech_attr, gss_inquire_saslname_for_mech,
                         gss_inquire_mech_for_saslname};
use self::stdio_h::{stderr, fprintf, printf};
use self::string_h::memcmp;
use self::common_h::check_gsserr;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2009  by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn dump_known_mech_attrs(mut mech: gss_OID) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut mech_attrs: gss_OID_set = 0 as gss_OID_set;
    let mut known_attrs: gss_OID_set = 0 as gss_OID_set;
    let mut i: size_t = 0;
    major =
        gss_inquire_attrs_for_mech(&mut minor, mech as gss_const_OID,
                                   &mut mech_attrs, &mut known_attrs);
    check_gsserr(b"gss_inquire_attrs_for_mech\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    printf(b"Known attributes\n\x00" as *const u8 as *const libc::c_char);
    printf(b"----------------\n\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i < (*known_attrs).count {
        let mut name: gss_buffer_desc =
            {
                let mut init =
                    gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                           value: 0 as *mut libc::c_void,};
                init
            };
        let mut short_desc: gss_buffer_desc =
            {
                let mut init =
                    gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                           value: 0 as *mut libc::c_void,};
                init
            };
        let mut long_desc: gss_buffer_desc =
            {
                let mut init =
                    gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                           value: 0 as *mut libc::c_void,};
                init
            };
        major =
            gss_display_mech_attr(&mut minor,
                                  &mut *(*known_attrs).elements.offset(i as
                                                                           isize)
                                      as *mut gss_OID_desc_struct as
                                      gss_const_OID, &mut name,
                                  &mut short_desc, &mut long_desc);
        check_gsserr(b"gss_display_mech_attr\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        printf(b"%.*s (%.*s): %.*s\n\x00" as *const u8 as *const libc::c_char,
               short_desc.length as libc::c_int,
               short_desc.value as *mut libc::c_char,
               name.length as libc::c_int, name.value as *mut libc::c_char,
               long_desc.length as libc::c_int,
               long_desc.value as *mut libc::c_char);
        gss_release_buffer(&mut minor, &mut name);
        gss_release_buffer(&mut minor, &mut short_desc);
        gss_release_buffer(&mut minor, &mut long_desc);
        i = i.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    gss_release_oid_set(&mut minor, &mut mech_attrs);
    gss_release_oid_set(&mut minor, &mut known_attrs);
}
#[c2rust::src_loc = "66:1"]
unsafe extern "C" fn dump_mech_attrs(mut mech: gss_OID) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut mech_attrs: gss_OID_set = 0 as gss_OID_set;
    let mut known_attrs: gss_OID_set = 0 as gss_OID_set;
    let mut i: size_t = 0;
    major =
        gss_inquire_attrs_for_mech(&mut minor, mech as gss_const_OID,
                                   &mut mech_attrs, &mut known_attrs);
    check_gsserr(b"gss_inquire_attrs_for_mech\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    printf(b"Mech attrs:  \x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i < (*mech_attrs).count {
        let mut name: gss_buffer_desc =
            {
                let mut init =
                    gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                           value: 0 as *mut libc::c_void,};
                init
            };
        let mut short_desc: gss_buffer_desc =
            {
                let mut init =
                    gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                           value: 0 as *mut libc::c_void,};
                init
            };
        let mut long_desc: gss_buffer_desc =
            {
                let mut init =
                    gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                           value: 0 as *mut libc::c_void,};
                init
            };
        major =
            gss_display_mech_attr(&mut minor,
                                  &mut *(*mech_attrs).elements.offset(i as
                                                                          isize)
                                      as *mut gss_OID_desc_struct as
                                      gss_const_OID, &mut name,
                                  &mut short_desc, &mut long_desc);
        check_gsserr(b"gss_display_mech_attr\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        printf(b"%.*s \x00" as *const u8 as *const libc::c_char,
               name.length as libc::c_int, name.value as *mut libc::c_char);
        gss_release_buffer(&mut minor, &mut name);
        gss_release_buffer(&mut minor, &mut short_desc);
        gss_release_buffer(&mut minor, &mut long_desc);
        i = i.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    gss_release_oid_set(&mut minor, &mut mech_attrs);
    gss_release_oid_set(&mut minor, &mut known_attrs);
}
#[c2rust::src_loc = "99:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut mechs: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut i: size_t = 0;
    major = gss_indicate_mechs(&mut minor, &mut mechs);
    check_gsserr(b"gss_indicate_mechs\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if (*mechs).count > 0 as libc::c_int as libc::c_ulong {
        dump_known_mech_attrs((*mechs).elements);
    }
    i = 0 as libc::c_int as size_t;
    while i < (*mechs).count {
        let mut oidstr: gss_buffer_desc =
            {
                let mut init =
                    gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                           value: 0 as *mut libc::c_void,};
                init
            };
        let mut sasl_mech_name: gss_buffer_desc =
            {
                let mut init =
                    gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                           value: 0 as *mut libc::c_void,};
                init
            };
        let mut mech_name: gss_buffer_desc =
            {
                let mut init =
                    gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                           value: 0 as *mut libc::c_void,};
                init
            };
        let mut mech_description: gss_buffer_desc =
            {
                let mut init =
                    gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                           value: 0 as *mut libc::c_void,};
                init
            };
        let mut oid: gss_OID = 0 as gss_OID;
        major =
            gss_oid_to_str(&mut minor,
                           &mut *(*mechs).elements.offset(i as isize),
                           &mut oidstr);
        if !(major &
                 ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                      (0o377 as libc::c_ulong as OM_uint32) <<
                          16 as libc::c_int) != 0) {
            major =
                gss_inquire_saslname_for_mech(&mut minor,
                                              &mut *(*mechs).elements.offset(i
                                                                                 as
                                                                                 isize),
                                              &mut sasl_mech_name,
                                              &mut mech_name,
                                              &mut mech_description);
            if major &
                   ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
                        |
                        (0o377 as libc::c_ulong as OM_uint32) <<
                            16 as libc::c_int) != 0 {
                gss_release_buffer(&mut minor, &mut oidstr);
            } else {
                printf(b"------------------------------------------------------------------------------\n\x00"
                           as *const u8 as *const libc::c_char);
                printf(b"OID        : %.*s\n\x00" as *const u8 as
                           *const libc::c_char, oidstr.length as libc::c_int,
                       oidstr.value as *mut libc::c_char);
                printf(b"SASL mech  : %.*s\n\x00" as *const u8 as
                           *const libc::c_char,
                       sasl_mech_name.length as libc::c_int,
                       sasl_mech_name.value as *mut libc::c_char);
                printf(b"Mech name  : %.*s\n\x00" as *const u8 as
                           *const libc::c_char,
                       mech_name.length as libc::c_int,
                       mech_name.value as *mut libc::c_char);
                printf(b"Mech desc  : %.*s\n\x00" as *const u8 as
                           *const libc::c_char,
                       mech_description.length as libc::c_int,
                       mech_description.value as *mut libc::c_char);
                dump_mech_attrs(&mut *(*mechs).elements.offset(i as isize));
                printf(b"------------------------------------------------------------------------------\n\x00"
                           as *const u8 as *const libc::c_char);
                major =
                    gss_inquire_mech_for_saslname(&mut minor,
                                                  &mut sasl_mech_name,
                                                  &mut oid);
                check_gsserr(b"gss_inquire_mech_for_saslname\x00" as *const u8
                                 as *const libc::c_char, major, minor);
                if oid.is_null() ||
                       (*oid).length !=
                           (*(*mechs).elements.offset(i as isize)).length &&
                           memcmp((*oid).elements,
                                  (*(*mechs).elements.offset(i as
                                                                 isize)).elements,
                                  (*oid).length as libc::c_ulong) !=
                               0 as libc::c_int {
                    gss_release_buffer(&mut minor, &mut oidstr);
                    gss_oid_to_str(&mut minor, oid, &mut oidstr);
                    fprintf(stderr,
                            b"Got different OID %.*s for mechanism %.*s\n\x00"
                                as *const u8 as *const libc::c_char,
                            oidstr.length as libc::c_int,
                            oidstr.value as *mut libc::c_char,
                            sasl_mech_name.length as libc::c_int,
                            sasl_mech_name.value as *mut libc::c_char);
                }
                gss_release_buffer(&mut minor, &mut oidstr);
                gss_release_buffer(&mut minor, &mut sasl_mech_name);
                gss_release_buffer(&mut minor, &mut mech_name);
                gss_release_buffer(&mut minor, &mut mech_description);
            }
        }
        i = i.wrapping_add(1)
    }
    gss_release_oid_set(&mut minor, &mut mechs);
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
