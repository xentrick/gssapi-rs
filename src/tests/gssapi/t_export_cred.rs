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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:29"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:29"]
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
    pub type gss_OID_set_desc = gss_OID_set_desc_struct;
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
        /* prf_out */
        #[no_mangle]
        #[c2rust::src_loc = "805:1"]
        pub fn gss_store_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                              _: gss_cred_usage_t, _: gss_OID, _: OM_uint32,
                              _: OM_uint32, _: *mut gss_OID_set,
                              _: *mut gss_cred_usage_t) -> OM_uint32;
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
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:29"]
pub mod common_h {
    use super::gssapi_h::{gss_cred_id_t, gss_OID_desc_struct, gss_OID,
                          gss_cred_id_struct, gss_name_struct, gss_name_t,
                          OM_uint32, gss_ctx_id_t, gss_OID_set_desc,
                          gss_OID_desc};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn export_import_cred(cred: *mut gss_cred_id_t);
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
        #[c2rust::src_loc = "42:25"]
        pub static mut mechset_spnego: gss_OID_set_desc;
        #[no_mangle]
        #[c2rust::src_loc = "41:25"]
        pub static mut mechset_krb5: gss_OID_set_desc;
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
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_OID_set, gss_buffer_desc_struct, gss_buffer_t,
                         gss_cred_usage_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         gss_release_name, gss_store_cred,
                         gss_delete_sec_context, gss_release_cred,
                         gss_acquire_cred};
use self::stdio_h::{stderr, fprintf};
use self::stdlib_h::exit;
use self::common_h::{export_import_cred, establish_contexts, import_name,
                     check_gsserr, mechset_spnego, mechset_krb5, mech_spnego,
                     mech_krb5};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2011 by the Massachusetts Institute of Technology.
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
/* Display a usage error message and exit. */
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"Usage: t_export_cred [-k|-s] [-i initiatorname] [-a acceptorname] targetname\n\x00"
                as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "40:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    let mut initiator_name: gss_name_t = 0 as gss_name_t;
    let mut acceptor_name: gss_name_t = 0 as gss_name_t;
    let mut target_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut initiator_cred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut acceptor_cred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut delegated_cred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut initiator_context: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut acceptor_context: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut mech: gss_OID = 0 as gss_OID;
    let mut mechs: gss_OID_set = 0 as gss_OID_set;
    let mut optchar: libc::c_char = 0;
    /* Parse arguments. */
    argv = argv.offset(1);
    while !(*argv).is_null() && **argv as libc::c_int == '-' as i32 {
        optchar = *(*argv).offset(1 as libc::c_int as isize);
        argv = argv.offset(1);
        if optchar as libc::c_int == 'i' as i32 {
            if (*argv).is_null() { usage(); }
            let fresh0 = argv;
            argv = argv.offset(1);
            initiator_name = import_name(*fresh0)
        } else if optchar as libc::c_int == 'a' as i32 {
            if (*argv).is_null() { usage(); }
            let fresh1 = argv;
            argv = argv.offset(1);
            acceptor_name = import_name(*fresh1)
        } else if optchar as libc::c_int == 'k' as i32 {
            mech = &mut mech_krb5;
            mechs = &mut mechset_krb5
        } else if optchar as libc::c_int == 's' as i32 {
            mech = &mut mech_spnego;
            mechs = &mut mechset_spnego
        } else { usage(); }
    }
    if (*argv).is_null() ||
           !(*argv.offset(1 as libc::c_int as isize)).is_null() {
        usage();
    }
    target_name = import_name(*argv.offset(0 as libc::c_int as isize));
    /* Get initiator cred and export/import it. */
    major =
        gss_acquire_cred(&mut minor, initiator_name,
                         0xffffffff as libc::c_ulong as OM_uint32, mechs,
                         1 as libc::c_int, &mut initiator_cred,
                         0 as *mut gss_OID_set, 0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred(initiator)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    export_import_cred(&mut initiator_cred);
    /* Get acceptor cred and export/import it. */
    major =
        gss_acquire_cred(&mut minor, acceptor_name,
                         0xffffffff as libc::c_ulong as OM_uint32, mechs,
                         2 as libc::c_int, &mut acceptor_cred,
                         0 as *mut gss_OID_set, 0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred(acceptor)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    export_import_cred(&mut acceptor_cred);
    /* Initiate and accept a security context (one-token exchange only),
     * delegating credentials. */
    flags =
        (4 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int |
             32 as libc::c_int | 1 as libc::c_int) as OM_uint32;
    establish_contexts(mech, initiator_cred, acceptor_cred, target_name,
                       flags, &mut initiator_context, &mut acceptor_context,
                       0 as *mut gss_name_t, 0 as *mut gss_OID,
                       &mut delegated_cred);
    /* Import, release, export, and store delegated creds */
    export_import_cred(&mut delegated_cred);
    major =
        gss_store_cred(&mut minor, delegated_cred, 1 as libc::c_int,
                       0 as gss_OID, 1 as libc::c_int as OM_uint32,
                       1 as libc::c_int as OM_uint32, 0 as *mut gss_OID_set,
                       0 as *mut gss_cred_usage_t);
    check_gsserr(b"gss_store_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    gss_release_name(&mut minor, &mut initiator_name);
    gss_release_name(&mut minor, &mut acceptor_name);
    gss_release_name(&mut minor, &mut target_name);
    gss_release_cred(&mut minor, &mut initiator_cred);
    gss_release_cred(&mut minor, &mut acceptor_cred);
    gss_release_cred(&mut minor, &mut delegated_cred);
    gss_delete_sec_context(&mut minor, &mut initiator_context,
                           0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut acceptor_context,
                           0 as gss_buffer_t);
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
