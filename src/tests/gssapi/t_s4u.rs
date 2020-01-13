use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:53"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:53"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:53"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:53"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:54"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:57"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:57"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:57"]
pub mod gssapi_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:16"]
    pub struct gss_buffer_set_desc_struct {
        pub count: size_t,
        pub elements: *mut gss_buffer_desc,
    }
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
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
    use super::stddef_h::size_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32, gss_cred_id_struct,
                          gss_cred_id_t, gss_OID_desc_struct, gss_OID,
                          gss_name_struct, gss_name_t,
                          gss_OID_set_desc_struct, gss_OID_set,
                          gss_cred_usage_t, gss_buffer_desc_struct,
                          gss_buffer_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn gss_release_buffer_set(_: *mut OM_uint32,
                                      _: *mut gss_buffer_set_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "160:1"]
        pub fn gss_inquire_cred_by_oid(_: *mut OM_uint32, _: gss_cred_id_t,
                                       _: gss_OID, _: *mut gss_buffer_set_t)
         -> OM_uint32;
        /* iov_count */
        /*
 * Protocol transition
 */
        #[no_mangle]
        #[c2rust::src_loc = "403:1"]
        pub fn gss_acquire_cred_impersonate_name(_: *mut OM_uint32,
                                                 _: gss_cred_id_t,
                                                 _: gss_name_t, _: OM_uint32,
                                                 _: gss_OID_set,
                                                 _: gss_cred_usage_t,
                                                 _: *mut gss_cred_id_t,
                                                 _: *mut gss_OID_set,
                                                 _: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "465:1"]
        pub fn gss_set_name_attribute(_: *mut OM_uint32, _: gss_name_t,
                                      _: libc::c_int, _: gss_buffer_t,
                                      _: gss_buffer_t) -> OM_uint32;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:57"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    #[c2rust::src_loc = "225:1"]
    pub type krb5_pointer = *mut libc::c_void;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "228:16"]
    pub struct krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub type_0: krb5_int32,
    }
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "323:16"]
    pub struct _krb5_address {
        pub magic: krb5_magic,
        pub addrtype: krb5_addrtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "323:1"]
    pub type krb5_address = _krb5_address;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    #[c2rust::src_loc = "1936:1"]
    pub type krb5_ticket_times = _krb5_ticket_times;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1946:16"]
    pub struct _krb5_authdata {
        pub magic: krb5_magic,
        pub ad_type: krb5_authdatatype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "1946:1"]
    pub type krb5_authdata = _krb5_authdata;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2013:16"]
    pub struct _krb5_creds {
        pub magic: krb5_magic,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub keyblock: krb5_keyblock,
        pub times: krb5_ticket_times,
        pub is_skey: krb5_boolean,
        pub ticket_flags: krb5_flags,
        pub addresses: *mut *mut krb5_address,
        pub ticket: krb5_data,
        pub second_ticket: krb5_data,
        pub authdata: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "2013:1"]
    pub type krb5_creds = _krb5_creds;
    #[c2rust::src_loc = "2278:1"]
    pub type krb5_cc_cursor = krb5_pointer;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    extern "C" {
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
        #[no_mangle]
        #[c2rust::src_loc = "2386:1"]
        pub fn krb5_cc_destroy(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2499:1"]
        pub fn krb5_cc_start_seq_get(context: krb5_context,
                                     cache: krb5_ccache,
                                     cursor: *mut krb5_cc_cursor)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2520:1"]
        pub fn krb5_cc_next_cred(context: krb5_context, cache: krb5_ccache,
                                 cursor: *mut krb5_cc_cursor,
                                 creds: *mut krb5_creds) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2538:1"]
        pub fn krb5_cc_end_seq_get(context: krb5_context, cache: krb5_ccache,
                                   cursor: *mut krb5_cc_cursor)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4677:1"]
        pub fn krb5_free_cred_contents(context: krb5_context,
                                       val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "4496:1"]
        pub fn krb5_is_config_principal(context: krb5_context,
                                        principal: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "2922:1"]
        pub fn krb5_init_context(context: *mut krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "4341:1"]
        pub fn krb5_cc_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               cache: *mut krb5_ccache) -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdio.h:53"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:54"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:55"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:57"]
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
        #[c2rust::src_loc = "52:1"]
        pub fn errout(msg: *const libc::c_char);
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:57"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{OM_uint32, gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "179:1"]
        pub fn krb5_gss_register_acceptor_identity(_: *const libc::c_char)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "111:33"]
        pub static GSS_KRB5_GET_CRED_IMPERSONATOR: gss_OID;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdint_intn_h::int32_t;
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
                         gss_delete_sec_context, gss_release_name,
                         gss_release_buffer, gss_release_oid_set,
                         gss_inquire_cred, gss_canonicalize_name};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_t,
                             gss_key_value_element_struct,
                             gss_key_value_element_desc,
                             gss_key_value_set_struct, gss_key_value_set_desc,
                             gss_const_key_value_set_t,
                             gss_release_buffer_set, gss_inquire_cred_by_oid,
                             gss_acquire_cred_impersonate_name,
                             gss_set_name_attribute, gss_store_cred_into};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_addrtype,
                       krb5_enctype, krb5_authdatatype, krb5_flags,
                       krb5_timestamp, krb5_error_code, krb5_magic,
                       _krb5_data, krb5_data, krb5_pointer,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_context, _krb5_keyblock, krb5_keyblock,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_creds, krb5_creds, krb5_cc_cursor,
                       krb5_ccache, _krb5_context, _krb5_ccache,
                       krb5_cc_destroy, krb5_cc_start_seq_get,
                       krb5_cc_next_cred, krb5_cc_end_seq_get,
                       krb5_free_cred_contents, krb5_is_config_principal,
                       krb5_init_context, krb5_free_context, krb5_cc_resolve};
use self::stdio_h::{stderr, fprintf, printf};
use self::stdlib_h::exit;
use self::string_h::{strcmp, strlen};
use self::common_h::{enumerate_attributes, display_oid, display_canon_name,
                     establish_contexts, import_name, errout, check_k5err,
                     check_gsserr, mechset_spnego, mechset_krb5, mech_spnego,
                     mech_krb5};
use self::gssapi_krb5_h::{krb5_gss_register_acceptor_identity,
                          GSS_KRB5_GET_CRED_IMPERSONATOR};
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
/*
 * Test program for protocol transition (S4U2Self) and constrained delegation
 * (S4U2Proxy)
 *
 * Note: because of name canonicalization, the following tips may help
 * when configuring with Active Directory:
 *
 * - Create a computer account FOO$
 * - Set the UPN to host/foo.domain (no suffix); this is necessary to
 *   be able to send an AS-REQ as this principal, otherwise you would
 *   need to use the canonical name (FOO$), which will cause principal
 *   comparison errors in gss_accept_sec_context().
 * - Add a SPN of host/foo.domain
 * - Configure the computer account to support constrained delegation with
 *   protocol transition (Trust this computer for delegation to specified
 *   services only / Use any authentication protocol)
 * - Add host/foo.domain to the keytab (possibly easiest to do this
 *   with ktadd)
 *
 * For S4U2Proxy to work the TGT must be forwardable too.
 *
 * Usage eg:
 *
 * kinit -k -t test.keytab -f 'host/test.win.mit.edu@WIN.MIT.EDU'
 * ./t_s4u p:delegtest@WIN.MIT.EDU p:HOST/WIN-EQ7E4AA2WR8.win.mit.edu@WIN.MIT.EDU test.keytab
 */
#[c2rust::src_loc = "59:12"]
static mut use_spnego: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "61:1"]
unsafe extern "C" fn test_greet_authz_data(mut name: *mut gss_name_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut attr: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut value: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut canon: gss_name_t = 0 as *mut gss_name_struct;
    major =
        gss_canonicalize_name(&mut minor, *name, &mut mech_krb5, &mut canon);
    check_gsserr(b"gss_canonicalize_name\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    attr.value =
        b"greet:greeting\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_void;
    attr.length = strlen(attr.value as *mut libc::c_char);
    value.value =
        b"Hello, acceptor world!\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_void;
    value.length = strlen(value.value as *mut libc::c_char);
    major =
        gss_set_name_attribute(&mut minor, canon, 1 as libc::c_int, &mut attr,
                               &mut value);
    if major == (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        gss_release_name(&mut minor, &mut canon);
        return
    }
    check_gsserr(b"gss_set_name_attribute\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    gss_release_name(&mut minor, name);
    *name = canon;
}
#[c2rust::src_loc = "88:1"]
unsafe extern "C" fn init_accept_sec_context(mut claimant_cred_handle:
                                                 gss_cred_id_t,
                                             mut verifier_cred_handle:
                                                 gss_cred_id_t,
                                             mut deleg_cred_handle:
                                                 *mut gss_cred_id_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut flags: OM_uint32 = 0;
    let mut source_name: gss_name_t = 0 as gss_name_t;
    let mut target_name: gss_name_t = 0 as gss_name_t;
    let mut initiator_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut acceptor_context: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut mech: gss_OID = 0 as gss_OID;
    *deleg_cred_handle = 0 as gss_cred_id_t;
    major =
        gss_inquire_cred(&mut minor, verifier_cred_handle, &mut target_name,
                         0 as *mut OM_uint32, 0 as *mut gss_cred_usage_t,
                         0 as *mut gss_OID_set);
    check_gsserr(b"gss_inquire_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    display_canon_name(b"Target name\x00" as *const u8 as *const libc::c_char,
                       target_name, &mut mech_krb5);
    mech = if use_spnego != 0 { &mut mech_spnego } else { &mut mech_krb5 };
    display_oid(b"Target mech\x00" as *const u8 as *const libc::c_char, mech);
    flags = (4 as libc::c_int | 8 as libc::c_int) as OM_uint32;
    establish_contexts(mech, claimant_cred_handle, verifier_cred_handle,
                       target_name, flags, &mut initiator_context,
                       &mut acceptor_context, &mut source_name, &mut mech,
                       deleg_cred_handle);
    display_canon_name(b"Source name\x00" as *const u8 as *const libc::c_char,
                       source_name, &mut mech_krb5);
    display_oid(b"Source mech\x00" as *const u8 as *const libc::c_char, mech);
    enumerate_attributes(source_name, 1 as libc::c_int);
    gss_release_name(&mut minor, &mut source_name);
    gss_release_name(&mut minor, &mut target_name);
    gss_delete_sec_context(&mut minor, &mut initiator_context,
                           0 as gss_buffer_t);
    gss_delete_sec_context(&mut minor, &mut acceptor_context,
                           0 as gss_buffer_t);
}
#[c2rust::src_loc = "125:1"]
unsafe extern "C" fn check_ticket_count(mut cred: gss_cred_id_t,
                                        mut expected: libc::c_int) {
    let mut ret: krb5_error_code = 0;
    let mut context: krb5_context = 0 as krb5_context;
    let mut kcred: krb5_creds =
        krb5_creds{magic: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   keyblock:
                       krb5_keyblock{magic: 0,
                                     enctype: 0,
                                     length: 0,
                                     contents: 0 as *mut krb5_octet,},
                   times:
                       krb5_ticket_times{authtime: 0,
                                         starttime: 0,
                                         endtime: 0,
                                         renew_till: 0,},
                   is_skey: 0,
                   ticket_flags: 0,
                   addresses: 0 as *mut *mut krb5_address,
                   ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   second_ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   authdata: 0 as *mut *mut krb5_authdata,};
    let mut cur: krb5_cc_cursor = 0 as *mut libc::c_void;
    let mut ccache: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut store: gss_key_value_set_desc =
        gss_key_value_set_desc{count: 0,
                               elements:
                                   0 as *mut gss_key_value_element_desc,};
    let mut elem: gss_key_value_element_desc =
        gss_key_value_element_desc{key: 0 as *const libc::c_char,
                                   value: 0 as *const libc::c_char,};
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut ccname: *const libc::c_char =
        b"MEMORY:count\x00" as *const u8 as *const libc::c_char;
    store.count = 1 as libc::c_int as OM_uint32;
    store.elements = &mut elem;
    elem.key = b"ccache\x00" as *const u8 as *const libc::c_char;
    elem.value = ccname;
    major =
        gss_store_cred_into(&mut minor, cred, 1 as libc::c_int,
                            &mut mech_krb5, 1 as libc::c_int as OM_uint32,
                            0 as libc::c_int as OM_uint32,
                            &mut store as *mut gss_key_value_set_desc as
                                gss_const_key_value_set_t,
                            0 as *mut gss_OID_set,
                            0 as *mut gss_cred_usage_t);
    check_gsserr(b"gss_store_cred_into\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    ret = krb5_init_context(&mut context);
    check_k5err(context,
                b"krb5_init_context\x00" as *const u8 as *const libc::c_char,
                ret);
    ret = krb5_cc_resolve(context, ccname, &mut ccache);
    check_k5err(context,
                b"krb5_cc_resolve\x00" as *const u8 as *const libc::c_char,
                ret);
    ret = krb5_cc_start_seq_get(context, ccache, &mut cur);
    check_k5err(context,
                b"krb5_cc_start_seq_get\x00" as *const u8 as
                    *const libc::c_char, ret);
    while krb5_cc_next_cred(context, ccache, &mut cur, &mut kcred) == 0 {
        if krb5_is_config_principal(context,
                                    kcred.server as krb5_const_principal) == 0
           {
            count += 1
        }
        krb5_free_cred_contents(context, &mut kcred);
    }
    ret = krb5_cc_end_seq_get(context, ccache, &mut cur);
    check_k5err(context,
                b"krb5_cc_end_seq_get\x00" as *const u8 as
                    *const libc::c_char, ret);
    if expected != count {
        printf(b"Expected %d tickets but got %d\n\x00" as *const u8 as
                   *const libc::c_char, expected, count);
        exit(1 as libc::c_int);
    }
    krb5_cc_destroy(context, ccache);
    krb5_free_context(context);
}
#[c2rust::src_loc = "174:1"]
unsafe extern "C" fn constrained_delegate(mut desired_mechs: gss_OID_set,
                                          mut target: gss_name_t,
                                          mut delegated_cred_handle:
                                              gss_cred_id_t,
                                          mut verifier_cred_handle:
                                              gss_cred_id_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut initiator_context: gss_ctx_id_t = 0 as gss_ctx_id_t;
    let mut cred_name: gss_name_t = 0 as gss_name_t;
    let mut time_rec: OM_uint32 = 0;
    let mut lifetime: OM_uint32 = 0;
    let mut usage: gss_cred_usage_t = 0;
    let mut token: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut mechs: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    printf(b"Constrained delegation tests follow\n\x00" as *const u8 as
               *const libc::c_char);
    printf(b"-----------------------------------\n\n\x00" as *const u8 as
               *const libc::c_char);
    if gss_inquire_cred(&mut minor, verifier_cred_handle, &mut cred_name,
                        &mut lifetime, &mut usage, 0 as *mut gss_OID_set) ==
           0 as libc::c_int as libc::c_uint {
        display_canon_name(b"Proxy name\x00" as *const u8 as
                               *const libc::c_char, cred_name,
                           &mut mech_krb5);
        gss_release_name(&mut minor, &mut cred_name);
    }
    display_canon_name(b"Target name\x00" as *const u8 as *const libc::c_char,
                       target, &mut mech_krb5);
    if gss_inquire_cred(&mut minor, delegated_cred_handle, &mut cred_name,
                        &mut lifetime, &mut usage, &mut mechs) ==
           0 as libc::c_int as libc::c_uint {
        display_canon_name(b"Delegated name\x00" as *const u8 as
                               *const libc::c_char, cred_name,
                           &mut mech_krb5);
        display_oid(b"Delegated mech\x00" as *const u8 as *const libc::c_char,
                    &mut *(*mechs).elements.offset(0 as libc::c_int as
                                                       isize));
        gss_release_name(&mut minor, &mut cred_name);
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    major =
        gss_init_sec_context(&mut minor, delegated_cred_handle,
                             &mut initiator_context, target,
                             if !mechs.is_null() {
                                 &mut *(*mechs).elements.offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                             } else { &mut mech_krb5 },
                             (4 as libc::c_int | 8 as libc::c_int) as
                                 OM_uint32,
                             0xffffffff as libc::c_ulong as OM_uint32,
                             0 as gss_channel_bindings_t, 0 as gss_buffer_t,
                             0 as *mut gss_OID, &mut token,
                             0 as *mut OM_uint32, &mut time_rec);
    check_gsserr(b"gss_init_sec_context\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    gss_release_buffer(&mut minor, &mut token);
    gss_delete_sec_context(&mut minor, &mut initiator_context,
                           0 as gss_buffer_t);
    /* Ensure a second call does not acquire new ticket. */
    major =
        gss_init_sec_context(&mut minor, delegated_cred_handle,
                             &mut initiator_context, target,
                             if !mechs.is_null() {
                                 &mut *(*mechs).elements.offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                             } else { &mut mech_krb5 },
                             (4 as libc::c_int | 8 as libc::c_int) as
                                 OM_uint32,
                             0xffffffff as libc::c_ulong as OM_uint32,
                             0 as gss_channel_bindings_t, 0 as gss_buffer_t,
                             0 as *mut gss_OID, &mut token,
                             0 as *mut OM_uint32, &mut time_rec);
    check_gsserr(b"gss_init_sec_context\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    gss_release_buffer(&mut minor, &mut token);
    gss_delete_sec_context(&mut minor, &mut initiator_context,
                           0 as gss_buffer_t);
    gss_release_oid_set(&mut minor, &mut mechs);
    /* We expect three tickets: our TGT, the evidence ticket, and the ticket to
     * the target service. */
    check_ticket_count(delegated_cred_handle, 3 as libc::c_int);
}
#[c2rust::src_loc = "236:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut impersonator_cred_handle: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut user_cred_handle: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut delegated_cred_handle: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut user: gss_name_t = 0 as gss_name_t;
    let mut target: gss_name_t = 0 as gss_name_t;
    let mut mechs: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut bufset: gss_buffer_set_t = 0 as gss_buffer_set_t;
    if argc < 2 as libc::c_int || argc > 5 as libc::c_int {
        fprintf(stderr,
                b"Usage: %s [--spnego] [user] [proxy-target] [keytab]\n\x00"
                    as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        fprintf(stderr,
                b"       proxy-target and keytab are optional\n\x00" as
                    *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if strcmp(*argv.offset(1 as libc::c_int as isize),
              b"--spnego\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        use_spnego += 1;
        argc -= 1;
        argv = argv.offset(1)
    }
    user = import_name(*argv.offset(1 as libc::c_int as isize));
    if argc > 2 as libc::c_int &&
           strcmp(*argv.offset(2 as libc::c_int as isize),
                  b"-\x00" as *const u8 as *const libc::c_char) != 0 {
        target = import_name(*argv.offset(2 as libc::c_int as isize))
    }
    if argc > 3 as libc::c_int {
        major =
            krb5_gss_register_acceptor_identity(*argv.offset(3 as libc::c_int
                                                                 as isize));
        check_gsserr(b"krb5_gss_register_acceptor_identity\x00" as *const u8
                         as *const libc::c_char, major,
                     0 as libc::c_int as OM_uint32);
    }
    /* Get default cred. */
    mechs =
        if use_spnego != 0 { &mut mechset_spnego } else { &mut mechset_krb5 };
    major =
        gss_acquire_cred(&mut minor, 0 as gss_name_t,
                         0xffffffff as libc::c_ulong as OM_uint32, mechs,
                         0 as libc::c_int, &mut impersonator_cred_handle,
                         0 as *mut gss_OID_set, 0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    printf(b"Protocol transition tests follow\n\x00" as *const u8 as
               *const libc::c_char);
    printf(b"-----------------------------------\n\n\x00" as *const u8 as
               *const libc::c_char);
    test_greet_authz_data(&mut user);
    /* Get S4U2Self cred. */
    major =
        gss_acquire_cred_impersonate_name(&mut minor,
                                          impersonator_cred_handle, user,
                                          0xffffffff as libc::c_ulong as
                                              OM_uint32, mechs,
                                          1 as libc::c_int,
                                          &mut user_cred_handle,
                                          0 as *mut gss_OID_set,
                                          0 as *mut OM_uint32);
    check_gsserr(b"gss_acquire_cred_impersonate_name\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    init_accept_sec_context(user_cred_handle, impersonator_cred_handle,
                            &mut delegated_cred_handle);
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    if !target.is_null() && !delegated_cred_handle.is_null() {
        constrained_delegate(mechs, target, delegated_cred_handle,
                             impersonator_cred_handle);
    } else if !target.is_null() {
        fprintf(stderr,
                b"Warning: no delegated cred handle returned\n\n\x00" as
                    *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"Verify:\n\n\x00" as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b" - The TGT for the impersonating service is forwardable\n\x00"
                    as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b" - The T2A4D flag set on the impersonating service\'s UAC\n\x00"
                    as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b" - The user is not marked sensitive and cannot be delegated\n\x00"
                    as *const u8 as *const libc::c_char);
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    if !delegated_cred_handle.is_null() {
        /* Inquire impersonator status. */
        major =
            gss_inquire_cred_by_oid(&mut minor, user_cred_handle,
                                    GSS_KRB5_GET_CRED_IMPERSONATOR,
                                    &mut bufset);
        check_gsserr(b"gss_inquire_cred_by_oid\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        if (*bufset).count == 0 as libc::c_int as libc::c_ulong {
            errout(b"gss_inquire_cred_by_oid(user) returned NO impersonator\x00"
                       as *const u8 as *const libc::c_char);
        }
        gss_release_buffer_set(&mut minor, &mut bufset);
        major =
            gss_inquire_cred_by_oid(&mut minor, impersonator_cred_handle,
                                    GSS_KRB5_GET_CRED_IMPERSONATOR,
                                    &mut bufset);
        check_gsserr(b"gss_inquire_cred_by_oid\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        if (*bufset).count != 0 as libc::c_int as libc::c_ulong {
            errout(b"gss_inquire_cred_by_oid(svc) returned an impersonator\x00"
                       as *const u8 as *const libc::c_char);
        }
        gss_release_buffer_set(&mut minor, &mut bufset);
    }
    gss_release_name(&mut minor, &mut user);
    gss_release_name(&mut minor, &mut target);
    gss_release_cred(&mut minor, &mut delegated_cred_handle);
    gss_release_cred(&mut minor, &mut impersonator_cred_handle);
    gss_release_cred(&mut minor, &mut user_cred_handle);
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
