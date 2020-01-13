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
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:30"]
pub mod gssapi_ext_h {
    /* Credential store extensions */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "530:8"]
    pub struct gss_key_value_element_struct {
        pub key: *const libc::c_char,
        pub value: *const libc::c_char,
    }
    #[c2rust::src_loc = "534:1"]
    pub type gss_key_value_element_desc = gss_key_value_element_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "536:8"]
    pub struct gss_key_value_set_struct {
        pub count: OM_uint32,
        pub elements: *mut gss_key_value_element_desc,
    }
    #[c2rust::src_loc = "540:1"]
    pub type gss_key_value_set_desc = gss_key_value_set_struct;
    #[c2rust::src_loc = "541:1"]
    pub type gss_const_key_value_set_t = *const gss_key_value_set_desc;
    use super::gssapi_h::{OM_uint32, gss_name_struct, gss_name_t,
                          gss_OID_set_desc_struct, gss_OID_set,
                          gss_cred_usage_t, gss_cred_id_t, gss_cred_id_struct,
                          gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "545:1"]
        pub fn gss_acquire_cred_from(_: *mut OM_uint32, _: gss_name_t,
                                     _: OM_uint32, _: gss_OID_set,
                                     _: gss_cred_usage_t,
                                     _: gss_const_key_value_set_t,
                                     _: *mut gss_cred_id_t,
                                     _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
        /* acceptor_time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "572:1"]
        pub fn gss_store_cred_into(_: *mut OM_uint32, _: gss_cred_id_t,
                                   _: gss_cred_usage_t, _: gss_OID,
                                   _: OM_uint32, _: OM_uint32,
                                   _: gss_const_key_value_set_t,
                                   _: *mut gss_OID_set,
                                   _: *mut gss_cred_usage_t) -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:30"]
pub mod krb5_h {
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
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
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:30"]
pub mod common_h {
    use super::gssapi_h::{gss_name_t, OM_uint32, gss_OID_desc};
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
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_cred_usage_t,
                         gss_name_struct, gss_cred_id_struct,
                         gss_ctx_id_struct, gss_acquire_cred,
                         gss_release_cred, gss_init_sec_context,
                         gss_accept_sec_context, gss_delete_sec_context,
                         gss_release_name, gss_release_buffer};
pub use self::gssapi_ext_h::{gss_key_value_element_struct,
                             gss_key_value_element_desc,
                             gss_key_value_set_struct, gss_key_value_set_desc,
                             gss_const_key_value_set_t, gss_acquire_cred_from,
                             gss_store_cred_into};
pub use self::krb5_h::krb5_boolean;
use self::stdio_h::{stderr, fprintf};
use self::stdlib_h::{calloc, free, exit};
use self::common_h::{import_name, errout, check_gsserr, mech_krb5};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2011 Red Hat, Inc.
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"Usage: t_credstore [-sabi] principal [{key value} ...]\n\x00" as
                *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "40:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut store: gss_key_value_set_desc =
        gss_key_value_set_desc{count: 0,
                               elements:
                                   0 as *mut gss_key_value_element_desc,};
    let mut name: gss_name_t = 0 as *mut gss_name_struct;
    let mut cred_usage: gss_cred_usage_t = 0 as libc::c_int;
    let mut mechs: gss_OID_set = 0 as gss_OID_set;
    let mut cred: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut ictx: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut actx: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut itok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut atok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut store_creds: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut replay: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut opt: libc::c_char = 0;
    /* Parse options. */
    argv = argv.offset(1);
    while !(*argv).is_null() && **argv as libc::c_int == '-' as i32 {
        opt = *(*argv).offset(1 as libc::c_int as isize);
        if opt as libc::c_int == 's' as i32 {
            store_creds = 1 as libc::c_int as krb5_boolean
        } else if opt as libc::c_int == 'r' as i32 {
            replay = 1 as libc::c_int as krb5_boolean
        } else if opt as libc::c_int == 'a' as i32 {
            cred_usage = 2 as libc::c_int
        } else if opt as libc::c_int == 'b' as i32 {
            cred_usage = 0 as libc::c_int
        } else if opt as libc::c_int == 'i' as i32 {
            cred_usage = 1 as libc::c_int
        } else { usage(); }
        argv = argv.offset(1)
    }
    /* Get the principal name. */
    if (*argv).is_null() { usage(); }
    let fresh0 = argv;
    argv = argv.offset(1);
    name = import_name(*fresh0);
    /* Put any remaining arguments into the store. */
    store.elements =
        calloc(argc as libc::c_ulong,
               ::std::mem::size_of::<gss_key_value_element_struct>() as
                   libc::c_ulong) as *mut gss_key_value_element_desc;
    if store.elements.is_null() {
        errout(b"OOM\x00" as *const u8 as *const libc::c_char);
    }
    store.count = 0 as libc::c_int as OM_uint32;
    while !(*argv).is_null() {
        if (*argv.offset(1 as libc::c_int as isize)).is_null() { usage(); }
        let ref mut fresh1 =
            (*store.elements.offset(store.count as isize)).key;
        *fresh1 = *argv;
        let ref mut fresh2 =
            (*store.elements.offset(store.count as isize)).value;
        *fresh2 = *argv.offset(1 as libc::c_int as isize);
        store.count = store.count.wrapping_add(1);
        argv = argv.offset(2 as libc::c_int as isize)
    }
    if store_creds != 0 {
        /* Acquire default creds and try to store them in the cred store. */
        major =
            gss_acquire_cred(&mut minor, 0 as gss_name_t,
                             0 as libc::c_int as OM_uint32, 0 as gss_OID_set,
                             1 as libc::c_int, &mut cred,
                             0 as *mut gss_OID_set, 0 as *mut OM_uint32);
        check_gsserr(b"gss_acquire_cred\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        major =
            gss_store_cred_into(&mut minor, cred, 1 as libc::c_int,
                                0 as gss_OID, 1 as libc::c_int as OM_uint32,
                                0 as libc::c_int as OM_uint32,
                                &mut store as *mut gss_key_value_set_desc as
                                    gss_const_key_value_set_t,
                                0 as *mut gss_OID_set,
                                0 as *mut gss_cred_usage_t);
        check_gsserr(b"gss_store_cred_into\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        gss_release_cred(&mut minor, &mut cred);
    }
    /* Try to acquire creds from store. */
    major =
        gss_acquire_cred_from(&mut minor, name, 0 as libc::c_int as OM_uint32,
                              mechs, cred_usage,
                              &mut store as *mut gss_key_value_set_desc as
                                  gss_const_key_value_set_t, &mut cred,
                              0 as *mut gss_OID_set, 0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred_from\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    if replay != 0 {
        /* Induce a replay using cred as the acceptor cred, to test the replay
         * cache indicated by the store. */
        major =
            gss_init_sec_context(&mut minor, 0 as gss_cred_id_t, &mut ictx,
                                 name, &mut mech_krb5,
                                 0 as libc::c_int as OM_uint32,
                                 0xffffffff as libc::c_ulong as OM_uint32,
                                 0 as gss_channel_bindings_t,
                                 0 as gss_buffer_t, 0 as *mut gss_OID,
                                 &mut itok, 0 as *mut OM_uint32,
                                 0 as *mut OM_uint32);
        check_gsserr(b"gss_init_sec_context\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        gss_delete_sec_context(&mut minor, &mut ictx, 0 as gss_buffer_t);
        major =
            gss_accept_sec_context(&mut minor, &mut actx, cred, &mut itok,
                                   0 as gss_channel_bindings_t,
                                   0 as *mut gss_name_t, 0 as *mut gss_OID,
                                   &mut atok, 0 as *mut OM_uint32,
                                   0 as *mut OM_uint32,
                                   0 as *mut gss_cred_id_t);
        check_gsserr(b"gss_accept_sec_context(1)\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        gss_release_buffer(&mut minor, &mut atok);
        gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
        major =
            gss_accept_sec_context(&mut minor, &mut actx, cred, &mut itok,
                                   0 as gss_channel_bindings_t,
                                   0 as *mut gss_name_t, 0 as *mut gss_OID,
                                   &mut atok, 0 as *mut OM_uint32,
                                   0 as *mut OM_uint32,
                                   0 as *mut gss_cred_id_t);
        check_gsserr(b"gss_accept_sec_context(2)\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        gss_release_buffer(&mut minor, &mut itok);
        gss_release_buffer(&mut minor, &mut atok);
        gss_delete_sec_context(&mut minor, &mut actx, 0 as gss_buffer_t);
    }
    gss_release_name(&mut minor, &mut name);
    gss_release_cred(&mut minor, &mut cred);
    free(store.elements as *mut libc::c_void);
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
