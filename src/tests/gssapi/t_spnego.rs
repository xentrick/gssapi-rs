use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:27"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:32"]
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
        /* cred_usage_stored */
        #[no_mangle]
        #[c2rust::src_loc = "816:1"]
        pub fn gss_set_neg_mechs(_: *mut OM_uint32, _: gss_cred_id_t,
                                 _: gss_OID_set) -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:29"]
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
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:32"]
pub mod common_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, gss_name_struct,
                          gss_name_t, gss_cred_id_struct, gss_cred_id_t,
                          OM_uint32, gss_ctx_id_t, gss_OID_set_desc,
                          gss_OID_desc};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn display_oid(tag: *const libc::c_char, oid: gss_OID);
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
        #[c2rust::src_loc = "40:21"]
        pub static mut mech_iakerb: gss_OID_desc;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:32"]
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
pub use self::types_h::{__uint8_t, __uint32_t, __off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_OID_set, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_cred_usage_t,
                         gss_name_struct, gss_cred_id_struct,
                         gss_ctx_id_struct, gss_acquire_cred,
                         gss_release_cred, gss_init_sec_context,
                         gss_accept_sec_context, gss_delete_sec_context,
                         gss_release_name, gss_release_buffer,
                         gss_release_oid_set, gss_set_neg_mechs};
use self::stdio_h::{stderr, fprintf};
use self::stdlib_h::{malloc, free, exit};
use self::string_h::{memcpy, memcmp, strlen};
use self::assert_h::__assert_fail;
use self::common_h::{display_oid, display_canon_name, establish_contexts,
                     import_name, check_gsserr, mechset_spnego, mechset_krb5,
                     mech_iakerb, mech_spnego, mech_krb5};
use self::gssapi_krb5_h::krb5_gss_register_acceptor_identity;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2010  by the Massachusetts Institute of Technology.
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
 *
 */
#[c2rust::src_loc = "34:21"]
static mut mech_krb5_wrong: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 9 as libc::c_int as OM_uint32,
                                elements:
                                    b"*\x86H\x82\xf7\x12\x01\x02\x02\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "37:18"]
pub static mut mechset_krb5_wrong: gss_OID_set_desc =
    unsafe {
        {
            let mut init =
                gss_OID_set_desc_struct{count: 1 as libc::c_int as size_t,
                                        elements:
                                            &mech_krb5_wrong as
                                                *const gss_OID_desc as
                                                *mut gss_OID_desc,};
            init
        }
    };
/*
 * Test program for SPNEGO and gss_set_neg_mechs
 *
 * Example usage:
 *
 * kinit testuser
 * ./t_spnego host/test.host@REALM testhost.keytab
 */
/* Replace *tok and *len with the concatenation of prefix and *tok. */
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn prepend(mut prefix: *const libc::c_void,
                             mut plen: size_t, mut tok: *mut *mut uint8_t,
                             mut len: *mut size_t) {
    let mut newtok: *mut uint8_t = 0 as *mut uint8_t;
    newtok = malloc(plen.wrapping_add(*len)) as *mut uint8_t;
    if !newtok.is_null() {
    } else {
        __assert_fail(b"newtok != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_spnego.c\x00" as *const u8 as *const libc::c_char,
                      55 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"void prepend(const void *, size_t, uint8_t **, size_t *)\x00")).as_ptr());
    }
    memcpy(newtok as *mut libc::c_void, prefix, plen);
    memcpy(newtok.offset(plen as isize) as *mut libc::c_void,
           *tok as *const libc::c_void, *len);
    free(*tok as *mut libc::c_void);
    *tok = newtok;
    *len = plen.wrapping_add(*len);
}
/* Replace *tok and *len with *tok wrapped in a DER tag with the given tag
 * byte.  *len must be less than 2^16. */
#[c2rust::src_loc = "65:1"]
unsafe extern "C" fn der_wrap(mut tag: uint8_t, mut tok: *mut *mut uint8_t,
                              mut len: *mut size_t) {
    let mut lenbuf: [libc::c_char; 3] = [0; 3];
    let mut wrapped: *mut uint8_t = 0 as *mut uint8_t;
    let mut llen: size_t = 0;
    if *len < 128 as libc::c_int as libc::c_ulong {
        lenbuf[0 as libc::c_int as usize] = *len as libc::c_char;
        llen = 1 as libc::c_int as size_t
    } else if *len < 256 as libc::c_int as libc::c_ulong {
        lenbuf[0 as libc::c_int as usize] =
            0x81 as libc::c_int as libc::c_char;
        lenbuf[1 as libc::c_int as usize] = *len as libc::c_char;
        llen = 2 as libc::c_int as size_t
    } else {
        if *len >> 16 as libc::c_int == 0 as libc::c_int as libc::c_ulong {
        } else {
            __assert_fail(b"*len >> 16 == 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"t_spnego.c\x00" as *const u8 as
                              *const libc::c_char,
                          80 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 45],
                                                    &[libc::c_char; 45]>(b"void der_wrap(uint8_t, uint8_t **, size_t *)\x00")).as_ptr());
        }
        lenbuf[0 as libc::c_int as usize] =
            0x82 as libc::c_int as libc::c_char;
        lenbuf[1 as libc::c_int as usize] =
            (*len >> 8 as libc::c_int) as libc::c_char;
        lenbuf[2 as libc::c_int as usize] =
            (*len & 0xff as libc::c_int as libc::c_ulong) as libc::c_char;
        llen = 3 as libc::c_int as size_t
    }
    wrapped =
        malloc((1 as libc::c_int as
                    libc::c_ulong).wrapping_add(llen).wrapping_add(*len)) as
            *mut uint8_t;
    if !wrapped.is_null() {
    } else {
        __assert_fail(b"wrapped != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_spnego.c\x00" as *const u8 as *const libc::c_char,
                      87 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void der_wrap(uint8_t, uint8_t **, size_t *)\x00")).as_ptr());
    }
    *wrapped = tag;
    memcpy(wrapped.offset(1 as libc::c_int as isize) as *mut libc::c_void,
           lenbuf.as_mut_ptr() as *const libc::c_void, llen);
    memcpy(wrapped.offset(1 as libc::c_int as isize).offset(llen as isize) as
               *mut libc::c_void, *tok as *const libc::c_void, *len);
    free(*tok as *mut libc::c_void);
    *tok = wrapped;
    *len =
        (1 as libc::c_int as
             libc::c_ulong).wrapping_add(llen).wrapping_add(*len);
}
/*
 * Create a SPNEGO initiator token for the erroneous Microsoft krb5 mech OID,
 * wrapping a krb5 token ktok.  The token should look like:
 *
 * 60 <len> (GSS framing sequence)
 *   06 06 2B 06 01 05 05 02 (SPNEGO OID)
 *   A0 <len> (NegotiationToken choice 0, negTokenInit)
 *     30 <len> (sequence)
 *       A0 0D (context tag 0, mechTypes)
 *         30 0B (sequence of)
 *           06 09 2A 86 48 82 F7 12 01 02 02 (wrong krb5 OID)
 *       A2 <len> (context tag 2, mechToken)
 *         04 <len> (octet string)
 *           <mech token octets>
 */
#[c2rust::src_loc = "111:1"]
unsafe extern "C" fn create_mskrb5_spnego_token(mut ktok: gss_buffer_t,
                                                mut tok_out:
                                                    *mut gss_buffer_desc) {
    let mut tok: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: size_t = 0;
    len = (*ktok).length;
    tok = malloc(len) as *mut uint8_t;
    if !tok.is_null() {
    } else {
        __assert_fail(b"tok != NULL\x00" as *const u8 as *const libc::c_char,
                      b"t_spnego.c\x00" as *const u8 as *const libc::c_char,
                      119 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 65],
                                                &[libc::c_char; 65]>(b"void create_mskrb5_spnego_token(gss_buffer_t, gss_buffer_desc *)\x00")).as_ptr());
    }
    memcpy(tok as *mut libc::c_void, (*ktok).value, len);
    /* Wrap the krb5 token in OCTET STRING and [2] tags. */
    der_wrap(0x4 as libc::c_int as uint8_t, &mut tok, &mut len);
    der_wrap(0xa2 as libc::c_int as uint8_t, &mut tok, &mut len);
    /* Prepend the wrong krb5 OID inside OBJECT IDENTIFIER and [0] tags. */
    prepend(b"\xa0\r0\x0b\x06\t*\x86H\x82\xf7\x12\x01\x02\x02\x00" as
                *const u8 as *const libc::c_char as *const libc::c_void,
            15 as libc::c_int as size_t, &mut tok, &mut len);
    /* Wrap the previous two things in SEQUENCE and [0] tags. */
    der_wrap(0x30 as libc::c_int as uint8_t, &mut tok, &mut len);
    der_wrap(0xa0 as libc::c_int as uint8_t, &mut tok, &mut len);
    /* Prepend the SPNEGO OID in an OBJECT IDENTIFIER tag. */
    prepend(b"\x06\x06+\x06\x01\x05\x05\x02\x00" as *const u8 as
                *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as size_t, &mut tok, &mut len);
    /* Wrap the whole thing in an [APPLICATION 0] tag. */
    der_wrap(0x60 as libc::c_int as uint8_t, &mut tok, &mut len);
    (*tok_out).value = tok as *mut libc::c_void;
    (*tok_out).length = len;
}
/*
 * Test that the SPNEGO acceptor code accepts and properly reflects back the
 * erroneous Microsoft mech OID in the supportedMech field of the NegTokenResp
 * message.  Use acred as the verifier cred handle.
 */
#[c2rust::src_loc = "143:1"]
unsafe extern "C" fn test_mskrb_oid(mut tname: gss_name_t,
                                    mut acred: gss_cred_id_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut ictx: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut actx: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut atok: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut ktok: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut stok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut atok_oid: *const libc::c_uchar = 0 as *const libc::c_uchar;
    /*
     * Our SPNEGO mech no longer acquires creds for the wrong mech OID, so we
     * have to construct a SPNEGO token ourselves.
     */
    major =
        gss_init_sec_context(&mut minor, 0 as gss_cred_id_t, &mut ictx, tname,
                             &mut mech_krb5, 0 as libc::c_int as OM_uint32,
                             0xffffffff as libc::c_ulong as OM_uint32,
                             0 as gss_channel_bindings_t, &mut atok,
                             0 as *mut gss_OID, &mut ktok,
                             0 as *mut OM_uint32, 0 as *mut OM_uint32);
    check_gsserr(b"gss_init_sec_context(mskrb)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if major == 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"major == GSS_S_COMPLETE\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_spnego.c\x00" as *const u8 as *const libc::c_char,
                      160 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void test_mskrb_oid(gss_name_t, gss_cred_id_t)\x00")).as_ptr());
    }
    create_mskrb5_spnego_token(&mut ktok, &mut stok);
    /*
     * Look directly at the DER encoding of the response token.  Since we
     * didn't request mutual authentication, the SPNEGO reply will contain no
     * underlying mech token; therefore, the encoding of the correct
     * NegotiationToken response is completely predictable:
     *
     *   A1 14 (choice 1, length 20, meaning negTokenResp)
     *     30 12 (sequence, length 18)
     *       A0 03 (context tag 0, length 3)
     *         0A 01 00 (enumerated value 0, meaning accept-completed)
     *       A1 0B (context tag 1, length 11)
     *         06 09 (object identifier, length 9)
     *           2A 86 48 82 F7 12 01 02 02 (the erroneous krb5 OID)
     *
     * So we can just compare the length to 22 and the nine bytes at offset 13
     * to the expected OID.
     */
    major =
        gss_accept_sec_context(&mut minor, &mut actx, acred, &mut stok,
                               0 as gss_channel_bindings_t,
                               0 as *mut gss_name_t, 0 as *mut gss_OID,
                               &mut atok, 0 as *mut OM_uint32,
                               0 as *mut OM_uint32, 0 as *mut gss_cred_id_t);
    check_gsserr(b"gss_accept_sec_context(mskrb)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if atok.length == 22 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"atok.length == 22\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_spnego.c\x00" as *const u8 as *const libc::c_char,
                      184 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void test_mskrb_oid(gss_name_t, gss_cred_id_t)\x00")).as_ptr());
    }
    atok_oid =
        (atok.value as *mut libc::c_uchar).offset(13 as libc::c_int as isize);
    if memcmp(atok_oid as *const libc::c_void, mech_krb5_wrong.elements,
              9 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
    } else {
        __assert_fail(b"memcmp(atok_oid, mech_krb5_wrong.elements, 9) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"t_spnego.c\x00" as *const u8 as *const libc::c_char,
                      186 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"void test_mskrb_oid(gss_name_t, gss_cred_id_t)\x00")).as_ptr());
    }
    gss_delete_sec_context(&mut minor, &mut ictx, 0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
    gss_release_buffer(&mut minor, &mut ktok);
    gss_release_buffer(&mut minor, &mut atok);
    free(stok.value);
}
/* Check that we return a compatibility NegTokenInit2 message containing
 * NegHints for an empty initiator token. */
#[c2rust::src_loc = "197:1"]
unsafe extern "C" fn test_neghints() {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut itok: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut atok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut actx: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut expected: *const libc::c_char =
        b"`G\x06\x06+\x06\x01\x05\x05\x02\xa0=0;\xa0\r0\x0b\x06\t*\x86H\x86\xf7\x12\x01\x02\x02\xa3*0(\xa0&\x1b$not_defined_in_RFC4178@please_ignore\x00"
            as *const u8 as *const libc::c_char;
    /* Produce a hint token. */
    major =
        gss_accept_sec_context(&mut minor, &mut actx, 0 as gss_cred_id_t,
                               &mut itok, 0 as gss_channel_bindings_t,
                               0 as *mut gss_name_t, 0 as *mut gss_OID,
                               &mut atok, 0 as *mut OM_uint32,
                               0 as *mut OM_uint32, 0 as *mut gss_cred_id_t);
    check_gsserr(b"gss_accept_sec_context(neghints)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    /* Verify it against the expected contents, which are fixed as long as we
     * only list the krb5 mech in the token. */
    if atok.length == strlen(expected) {
    } else {
        __assert_fail(b"atok.length == strlen(expected)\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_spnego.c\x00" as *const u8 as *const libc::c_char,
                      224 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 21],
                                                &[libc::c_char; 21]>(b"void test_neghints()\x00")).as_ptr());
    }
    if memcmp(atok.value, expected as *const libc::c_void, atok.length) ==
           0 as libc::c_int {
    } else {
        __assert_fail(b"memcmp(atok.value, expected, atok.length) == 0\x00" as
                          *const u8 as *const libc::c_char,
                      b"t_spnego.c\x00" as *const u8 as *const libc::c_char,
                      225 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 21],
                                                &[libc::c_char; 21]>(b"void test_neghints()\x00")).as_ptr());
    }
    gss_release_buffer(&mut minor, &mut atok);
    gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
}
#[c2rust::src_loc = "231:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    let mut verifier_cred_handle: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut initiator_cred_handle: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut actual_mechs: gss_OID_set = 0 as gss_OID_set;
    let mut initiator_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut acceptor_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut target_name: gss_name_t = 0 as *mut gss_name_struct;
    let mut source_name: gss_name_t = 0 as gss_name_t;
    let mut mech: gss_OID = 0 as gss_OID;
    let mut pref_oids: [gss_OID_desc; 2] =
        [gss_OID_desc{length: 0,
                      elements:
                          0 as *const libc::c_void as *mut libc::c_void,}; 2];
    let mut pref_mechs: gss_OID_set_desc =
        gss_OID_set_desc{count: 0,
                         elements:
                             0 as *const gss_OID_desc_struct as
                                 *mut gss_OID_desc_struct,};
    if argc < 2 as libc::c_int || argc > 3 as libc::c_int {
        fprintf(stderr,
                b"Usage: %s target_name [keytab]\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    target_name = import_name(*argv.offset(1 as libc::c_int as isize));
    if argc >= 3 as libc::c_int {
        major =
            krb5_gss_register_acceptor_identity(*argv.offset(2 as libc::c_int
                                                                 as isize));
        check_gsserr(b"krb5_gss_register_acceptor_identity\x00" as *const u8
                         as *const libc::c_char, major,
                     0 as libc::c_int as OM_uint32);
    }
    /* Get default initiator cred. */
    major =
        gss_acquire_cred(&mut minor, 0 as gss_name_t,
                         0xffffffff as libc::c_ulong as OM_uint32,
                         &mut mechset_spnego, 1 as libc::c_int,
                         &mut initiator_cred_handle, 0 as *mut gss_OID_set,
                         0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred(initiator)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    /*
     * The following test is designed to exercise SPNEGO reselection on the
     * client and server.  Unfortunately, it no longer does so after tickets
     * #8217 and #8021, since SPNEGO now only acquires a single krb5 cred and
     * there is no way to expand the underlying creds with gss_set_neg_mechs().
     * To fix this we need gss_acquire_cred_with_cred() or some other way to
     * turn a cred with a specifically requested mech set into a SPNEGO cred.
     */
    /* Make the initiator prefer IAKERB and offer krb5 as an alternative. */
    pref_oids[0 as libc::c_int as usize] = mech_iakerb;
    pref_oids[1 as libc::c_int as usize] = mech_krb5;
    pref_mechs.count = 2 as libc::c_int as size_t;
    pref_mechs.elements = pref_oids.as_mut_ptr();
    major =
        gss_set_neg_mechs(&mut minor, initiator_cred_handle, &mut pref_mechs);
    check_gsserr(b"gss_set_neg_mechs(initiator)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    /* Get default acceptor cred. */
    major =
        gss_acquire_cred(&mut minor, 0 as gss_name_t,
                         0xffffffff as libc::c_ulong as OM_uint32,
                         &mut mechset_spnego, 2 as libc::c_int,
                         &mut verifier_cred_handle, &mut actual_mechs,
                         0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred(acceptor)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    /* Restrict the acceptor to krb5 (which will force a reselection). */
    major =
        gss_set_neg_mechs(&mut minor, verifier_cred_handle,
                          &mut mechset_krb5);
    check_gsserr(b"gss_set_neg_mechs(acceptor)\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    flags = (4 as libc::c_int | 8 as libc::c_int) as OM_uint32;
    establish_contexts(&mut mech_spnego, initiator_cred_handle,
                       verifier_cred_handle, target_name, flags,
                       &mut initiator_context, &mut acceptor_context,
                       &mut source_name, &mut mech, 0 as *mut gss_cred_id_t);
    display_canon_name(b"Source name\x00" as *const u8 as *const libc::c_char,
                       source_name, &mut mech_krb5);
    display_oid(b"Source mech\x00" as *const u8 as *const libc::c_char, mech);
    /* Test acceptance of the erroneous Microsoft krb5 OID, with and without an
     * acceptor cred. */
    test_mskrb_oid(target_name, verifier_cred_handle);
    test_mskrb_oid(target_name, 0 as gss_cred_id_t);
    test_neghints();
    gss_delete_sec_context(&mut minor, &mut initiator_context,
                           0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut acceptor_context,
                           0 as gss_buffer_t);
    gss_release_name(&mut minor, &mut source_name);
    gss_release_name(&mut minor, &mut target_name);
    gss_release_cred(&mut minor, &mut initiator_cred_handle);
    gss_release_cred(&mut minor, &mut verifier_cred_handle);
    gss_release_oid_set(&mut minor, &mut actual_mechs);
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
