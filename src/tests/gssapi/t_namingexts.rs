use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:26"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
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
    /* OM_STRING */
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
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
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
        /* Function Prototypes */
        #[no_mangle]
        #[c2rust::src_loc = "421:1"]
        pub fn gss_acquire_cred(_: *mut OM_uint32, _: gss_name_t,
                                _: OM_uint32, _: gss_OID_set,
                                _: gss_cred_usage_t, _: *mut gss_cred_id_t,
                                _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* name_equal */
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
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
        /* buffer */
        #[no_mangle]
        #[c2rust::src_loc = "579:1"]
        pub fn gss_release_oid_set(_: *mut OM_uint32, _: *mut gss_OID_set)
         -> OM_uint32;
        /* set */
        #[no_mangle]
        #[c2rust::src_loc = "584:1"]
        pub fn gss_inquire_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                                _: *mut gss_name_t, _: *mut OM_uint32,
                                _: *mut gss_cred_usage_t, _: *mut gss_OID_set)
         -> OM_uint32;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:30"]
pub mod gssapi_ext_h {
    #[c2rust::src_loc = "488:1"]
    pub type gss_any_t = *mut gss_any;
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, OM_uint32,
                          gss_name_struct, gss_name_t, gss_buffer_desc_struct,
                          gss_buffer_t};
    extern "C" {
        #[c2rust::src_loc = "488:16"]
        pub type gss_any;
        #[no_mangle]
        #[c2rust::src_loc = "434:27"]
        pub static mut GSS_C_NT_COMPOSITE_EXPORT: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "465:1"]
        pub fn gss_set_name_attribute(_: *mut OM_uint32, _: gss_name_t,
                                      _: libc::c_int, _: gss_buffer_t,
                                      _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_name_attribute(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "481:1"]
        pub fn gss_export_name_composite(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "490:1"]
        pub fn gss_map_name_to_any(_: *mut OM_uint32, _: gss_name_t,
                                   _: libc::c_int, _: gss_buffer_t,
                                   _: *mut gss_any_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn gss_release_any_name_mapping(_: *mut OM_uint32, _: gss_name_t,
                                            _: gss_buffer_t,
                                            _: *mut gss_any_t) -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:30"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[c2rust::src_loc = "8142:1"]
    pub type krb5_pac = *mut krb5_pac_data;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[c2rust::src_loc = "8140:8"]
        pub type krb5_pac_data;
        #[no_mangle]
        #[c2rust::src_loc = "2922:1"]
        pub fn krb5_init_context(context: *mut krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8207:1"]
        pub fn krb5_pac_get_types(context: krb5_context, pac: krb5_pac,
                                  len: *mut size_t,
                                  types: *mut *mut krb5_ui_4)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
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
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
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
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:30"]
pub mod common_h {
    use super::gssapi_h::{gss_name_struct, gss_name_t, gss_OID_desc_struct,
                          gss_OID, gss_cred_id_struct, gss_cred_id_t,
                          OM_uint32, gss_ctx_id_t, gss_OID_set_desc,
                          gss_OID_desc};
    use super::krb5_h::{_krb5_context, krb5_context, krb5_error_code};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn enumerate_attributes(name: gss_name_t, noisy: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn display_canon_name(tag: *const libc::c_char, name: gss_name_t,
                                  mech: gss_OID);
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
        #[c2rust::src_loc = "49:1"]
        pub fn check_k5err(context: krb5_context, msg: *const libc::c_char,
                           code: krb5_error_code);
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:30"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::OM_uint32;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "179:1"]
        pub fn krb5_gss_register_acceptor_identity(_: *const libc::c_char)
         -> OM_uint32;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__int32_t, __uint32_t, __off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_OID_set, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_cred_usage_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         GSS_C_NT_EXPORT_NAME, gss_acquire_cred,
                         gss_release_cred, gss_delete_sec_context,
                         gss_display_name, gss_import_name, gss_release_name,
                         gss_release_buffer, gss_release_oid_set,
                         gss_inquire_cred, gss_canonicalize_name};
pub use self::gssapi_ext_h::{gss_any_t, gss_any, GSS_C_NT_COMPOSITE_EXPORT,
                             gss_set_name_attribute,
                             gss_delete_name_attribute,
                             gss_export_name_composite, gss_map_name_to_any,
                             gss_release_any_name_mapping};
pub use self::krb5_h::{krb5_int32, krb5_ui_4, krb5_error_code, krb5_context,
                       krb5_pac, _krb5_context, krb5_pac_data,
                       krb5_init_context, krb5_pac_get_types};
use self::stdio_h::{stderr, fprintf, printf};
use self::stdlib_h::{free, exit};
use self::string_h::{strcmp, strlen};
use self::common_h::{enumerate_attributes, display_canon_name,
                     establish_contexts, import_name, check_k5err,
                     check_gsserr, mechset_spnego, mechset_krb5, mech_spnego,
                     mech_krb5};
use self::gssapi_krb5_h::krb5_gss_register_acceptor_identity;
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
#[c2rust::src_loc = "32:12"]
static mut use_spnego: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "34:1"]
unsafe extern "C" fn display_name(mut tag: *const libc::c_char,
                                  mut name: gss_name_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    major = gss_display_name(&mut minor, name, &mut buf, 0 as *mut gss_OID);
    check_gsserr(b"gss_display_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    printf(b"%s:\t%.*s\n\x00" as *const u8 as *const libc::c_char, tag,
           buf.length as libc::c_int, buf.value as *mut libc::c_char);
    gss_release_buffer(&mut minor, &mut buf);
}
#[c2rust::src_loc = "48:1"]
unsafe extern "C" fn test_export_import_name(mut name: gss_name_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut exported_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut imported_name: gss_name_t = 0 as gss_name_t;
    let mut imported_name_comp: gss_name_t = 0 as gss_name_t;
    let mut i: libc::c_uint = 0;
    major = gss_export_name_composite(&mut minor, name, &mut exported_name);
    check_gsserr(b"gss_export_name_composite\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    printf(b"Exported name:\n\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < exported_name.length {
        if i.wrapping_rem(32 as libc::c_int as libc::c_uint) ==
               0 as libc::c_int as libc::c_uint {
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        printf(b"%02x\x00" as *const u8 as *const libc::c_char,
               *(exported_name.value as *mut libc::c_char).offset(i as isize)
                   as libc::c_int & 0xff as libc::c_int);
        i = i.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    major =
        gss_import_name(&mut minor, &mut exported_name, GSS_C_NT_EXPORT_NAME,
                        &mut imported_name);
    check_gsserr(b"gss_import_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    major =
        gss_import_name(&mut minor, &mut exported_name,
                        GSS_C_NT_COMPOSITE_EXPORT, &mut imported_name_comp);
    check_gsserr(b"gss_import_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    gss_release_buffer(&mut minor, &mut exported_name);
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    display_canon_name(b"Re-imported name\x00" as *const u8 as
                           *const libc::c_char, imported_name,
                       &mut mech_krb5);
    printf(b"Re-imported attributes:\n\n\x00" as *const u8 as
               *const libc::c_char);
    enumerate_attributes(imported_name, 0 as libc::c_int);
    display_name(b"Re-imported (as composite) name\x00" as *const u8 as
                     *const libc::c_char, imported_name_comp);
    printf(b"Re-imported (as composite) attributes:\n\n\x00" as *const u8 as
               *const libc::c_char);
    enumerate_attributes(imported_name_comp, 0 as libc::c_int);
    gss_release_name(&mut minor, &mut imported_name);
    gss_release_name(&mut minor, &mut imported_name_comp);
}
#[c2rust::src_loc = "90:1"]
unsafe extern "C" fn test_greet_authz_data(mut name: gss_name_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut attr: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut value: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    attr.value =
        b"urn:greet:greeting\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_void;
    attr.length = strlen(attr.value as *mut libc::c_char);
    major = gss_delete_name_attribute(&mut minor, name, &mut attr);
    if major == (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        fprintf(stderr,
                b"Warning: greet_client plugin not installed\n\x00" as
                    *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    check_gsserr(b"gss_delete_name_attribute\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    value.value =
        b"Hello, acceptor world!\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_void;
    value.length = strlen(value.value as *mut libc::c_char);
    major =
        gss_set_name_attribute(&mut minor, name, 1 as libc::c_int, &mut attr,
                               &mut value);
    if major == (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        return
    }
    check_gsserr(b"gss_set_name_attribute\x00" as *const u8 as
                     *const libc::c_char, major, minor);
}
#[c2rust::src_loc = "115:1"]
unsafe extern "C" fn test_map_name_to_any(mut name: gss_name_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut type_id: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut pac: krb5_pac = 0 as *mut krb5_pac_data;
    let mut context: krb5_context = 0 as krb5_context;
    let mut ret: krb5_error_code = 0;
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut types: *mut krb5_ui_4 = 0 as *mut krb5_ui_4;
    type_id.value =
        b"mspac\x00" as *const u8 as *const libc::c_char as *mut libc::c_void;
    type_id.length = strlen(type_id.value as *mut libc::c_char);
    major =
        gss_map_name_to_any(&mut minor, name, 1 as libc::c_int, &mut type_id,
                            &mut pac as *mut krb5_pac as *mut gss_any_t);
    if major == (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        return
    }
    check_gsserr(b"gss_map_name_to_any\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    ret = krb5_init_context(&mut context);
    check_k5err(context,
                b"krb5_init_context\x00" as *const u8 as *const libc::c_char,
                ret);
    if krb5_pac_get_types(context, pac, &mut len, &mut types) ==
           0 as libc::c_int {
        printf(b"PAC buffer types:\x00" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int as size_t;
        while i < len {
            printf(b" %d\x00" as *const u8 as *const libc::c_char,
                   *types.offset(i as isize));
            i = i.wrapping_add(1)
        }
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        free(types as *mut libc::c_void);
    }
    gss_release_any_name_mapping(&mut minor, name, &mut type_id,
                                 &mut pac as *mut krb5_pac as *mut gss_any_t);
}
#[c2rust::src_loc = "149:1"]
unsafe extern "C" fn init_accept_sec_context(mut verifier_cred_handle:
                                                 gss_cred_id_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    let mut source_name: gss_name_t = 0 as gss_name_t;
    let mut target_name: gss_name_t = 0 as gss_name_t;
    let mut initiator_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut acceptor_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut mech: gss_OID =
        if use_spnego != 0 { &mut mech_spnego } else { &mut mech_krb5 };
    major =
        gss_inquire_cred(&mut minor, verifier_cred_handle, &mut target_name,
                         0 as *mut OM_uint32, 0 as *mut gss_cred_usage_t,
                         0 as *mut gss_OID_set);
    check_gsserr(b"gss_inquire_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    display_canon_name(b"Target name\x00" as *const u8 as *const libc::c_char,
                       target_name, &mut mech_krb5);
    flags = (4 as libc::c_int | 8 as libc::c_int) as OM_uint32;
    establish_contexts(mech, verifier_cred_handle, verifier_cred_handle,
                       target_name, flags, &mut initiator_context,
                       &mut acceptor_context, &mut source_name,
                       0 as *mut gss_OID, 0 as *mut gss_cred_id_t);
    display_canon_name(b"Source name\x00" as *const u8 as *const libc::c_char,
                       source_name, &mut mech_krb5);
    enumerate_attributes(source_name, 1 as libc::c_int);
    test_export_import_name(source_name);
    test_map_name_to_any(source_name);
    gss_release_name(&mut minor, &mut source_name);
    gss_release_name(&mut minor, &mut target_name);
    gss_delete_sec_context(&mut minor, &mut initiator_context,
                           0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut acceptor_context,
                           0 as gss_buffer_t);
}
#[c2rust::src_loc = "179:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut cred_handle: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut mechs: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut actual_mechs: gss_OID_set = 0 as gss_OID_set;
    let mut tmp_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut name: gss_name_t = 0 as *mut gss_name_struct;
    if argc > 1 as libc::c_int &&
           strcmp(*argv.offset(1 as libc::c_int as isize),
                  b"--spnego\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        use_spnego += 1;
        argc -= 1;
        argv = argv.offset(1)
    }
    if argc < 2 as libc::c_int {
        fprintf(stderr,
                b"Usage: %s [--spnego] principal [keytab]\n\x00" as *const u8
                    as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    tmp_name = import_name(*argv.offset(1 as libc::c_int as isize));
    major =
        gss_canonicalize_name(&mut minor, tmp_name, &mut mech_krb5,
                              &mut name);
    check_gsserr(b"gss_canonicalze_name\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    gss_release_name(&mut minor, &mut tmp_name);
    test_greet_authz_data(name);
    if argc >= 3 as libc::c_int {
        major =
            krb5_gss_register_acceptor_identity(*argv.offset(2 as libc::c_int
                                                                 as isize));
        check_gsserr(b"krb5_gss_register_acceptor_identity\x00" as *const u8
                         as *const libc::c_char, major, minor);
    }
    mechs =
        if use_spnego != 0 { &mut mechset_spnego } else { &mut mechset_krb5 };
    /* get default cred */
    major =
        gss_acquire_cred(&mut minor, name,
                         0xffffffff as libc::c_ulong as OM_uint32, mechs,
                         0 as libc::c_int, &mut cred_handle,
                         &mut actual_mechs, 0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    gss_release_oid_set(&mut minor, &mut actual_mechs);
    init_accept_sec_context(cred_handle);
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    gss_release_cred(&mut minor, &mut cred_handle);
    gss_release_oid_set(&mut minor, &mut actual_mechs);
    gss_release_name(&mut minor, &mut name);
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
