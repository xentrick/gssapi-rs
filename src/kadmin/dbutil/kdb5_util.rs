use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/bits/types.h:56"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:56"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:56"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:56"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:56"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:56"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:56"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
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
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[c2rust::src_loc = "8475:1"]
    pub type krb5_pre_send_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "8391:1"]
    pub type krb5_trace_callback
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *const krb5_trace_info,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "8387:1"]
    pub type krb5_trace_info = _krb5_trace_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8387:16"]
    pub struct _krb5_trace_info {
        pub message: *const libc::c_char,
    }
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
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
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[no_mangle]
        #[c2rust::src_loc = "821:1"]
        pub fn krb5_c_random_seed(context: krb5_context, data: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "839:1"]
        pub fn krb5_c_string_to_key(context: krb5_context,
                                    enctype: krb5_enctype,
                                    string: *const krb5_data,
                                    salt: *const krb5_data,
                                    key: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1049:1"]
        pub fn krb5_c_valid_enctype(ktype: krb5_enctype) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4313:1"]
        pub fn krb5_principal2salt(context: krb5_context,
                                   pr: krb5_const_principal,
                                   ret: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4887:1"]
        pub fn krb5_get_default_realm(context: krb5_context,
                                      lrealm: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4903:1"]
        pub fn krb5_set_default_realm(context: krb5_context,
                                      lrealm: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4912:1"]
        pub fn krb5_free_default_realm(context: krb5_context,
                                       lrealm: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "6266:1"]
        pub fn krb5_string_to_enctype(string: *mut libc::c_char,
                                      enctypep: *mut krb5_enctype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8023:1"]
        pub fn krb5_get_error_message(ctx: krb5_context,
                                      code: krb5_error_code)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "8032:1"]
        pub fn krb5_free_error_message(ctx: krb5_context,
                                       msg: *const libc::c_char);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:56"]
pub mod k5_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1208:8"]
    pub struct _krb5_context {
        pub magic: krb5_magic,
        pub in_tkt_etypes: *mut krb5_enctype,
        pub tgs_etypes: *mut krb5_enctype,
        pub os_context: _krb5_os_context,
        pub default_realm: *mut libc::c_char,
        pub profile: profile_t,
        pub dal_handle: *mut kdb5_dal_handle,
        pub clockskew: krb5_deltat,
        pub kdc_default_options: krb5_flags,
        pub library_options: krb5_flags,
        pub profile_secure: krb5_boolean,
        pub fcc_default_format: libc::c_int,
        pub prompt_types: *mut krb5_prompt_type,
        pub udp_pref_limit: libc::c_int,
        pub use_conf_ktypes: krb5_boolean,
        pub libkrb5_plugins: plugin_dir_handle,
        pub preauth_context: krb5_preauth_context,
        pub ccselect_handles: *mut *mut ccselect_module_handle,
        pub localauth_handles: *mut *mut localauth_module_handle,
        pub hostrealm_handles: *mut *mut hostrealm_module_handle,
        pub tls: *mut k5_tls_vtable_st,
        pub err: errinfo,
        pub err_fmt: *mut libc::c_char,
        pub kdblog_context: *mut _kdb_log_context,
        pub allow_weak_crypto: krb5_boolean,
        pub ignore_acceptor_hostname: krb5_boolean,
        pub enforce_ok_as_delegate: krb5_boolean,
        pub dns_canonicalize_hostname: dns_canonhost,
        pub trace_callback: krb5_trace_callback,
        pub trace_callback_data: *mut libc::c_void,
        pub kdc_send_hook: krb5_pre_send_fn,
        pub kdc_send_hook_data: *mut libc::c_void,
        pub kdc_recv_hook: krb5_post_recv_fn,
        pub kdc_recv_hook_data: *mut libc::c_void,
        pub plugins: [plugin_interface; 13],
        pub plugin_base_dir: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1137:8"]
    pub struct plugin_interface {
        pub modules: *mut *mut plugin_mapping,
        pub configured: krb5_boolean,
    }
    #[c2rust::src_loc = "1194:1"]
    pub type dns_canonhost = libc::c_uint;
    #[c2rust::src_loc = "1197:5"]
    pub const CANONHOST_FALLBACK: dns_canonhost = 2;
    #[c2rust::src_loc = "1196:5"]
    pub const CANONHOST_TRUE: dns_canonhost = 1;
    #[c2rust::src_loc = "1195:5"]
    pub const CANONHOST_FALSE: dns_canonhost = 0;
    #[c2rust::src_loc = "1203:1"]
    pub type krb5_preauth_context = *mut krb5_preauth_context_st;
    #[c2rust::src_loc = "1201:1"]
    pub type kdb5_dal_handle = _kdb5_dal_handle;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "702:16"]
    pub struct _krb5_os_context {
        pub magic: krb5_magic,
        pub time_offset: krb5_int32,
        pub usec_offset: krb5_int32,
        pub os_flags: krb5_int32,
        pub default_ccname: *mut libc::c_char,
    }
    #[inline]
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::kdb_log_h::_kdb_log_context;
    use super::stddef_h::size_t;
    use super::string_h::explicit_bzero;
    use super::stdlib_h::free;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1207:8"]
        pub type k5_tls_vtable_st;
        #[c2rust::src_loc = "1206:8"]
        pub type hostrealm_module_handle;
        #[c2rust::src_loc = "1205:8"]
        pub type localauth_module_handle;
        #[c2rust::src_loc = "1204:8"]
        pub type ccselect_module_handle;
        #[c2rust::src_loc = "1203:16"]
        pub type krb5_preauth_context_st;
        #[c2rust::src_loc = "1200:8"]
        pub type _kdb5_dal_handle;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:61"]
pub mod kdb_log_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:16"]
    pub struct _kdb_log_context {
        pub iproprole: iprop_role,
        pub ulog: *mut kdb_hlog_t,
        pub ulogentries: uint32_t,
        pub ulogfd: libc::c_int,
    }
    #[c2rust::src_loc = "81:1"]
    pub type kdb_hlog_t = kdb_hlog;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct kdb_hlog {
        pub kdb_hmagic: uint32_t,
        pub db_version_num: uint16_t,
        pub kdb_num: uint32_t,
        pub kdb_first_time: kdbe_time_t,
        pub kdb_last_time: kdbe_time_t,
        pub kdb_first_sno: kdb_sno_t,
        pub kdb_last_sno: kdb_sno_t,
        pub kdb_state: uint16_t,
        pub kdb_block: uint16_t,
    }
    use super::iprop_hdr_h::{iprop_role, IPROP_NULL};
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::iprop_h::{kdbe_time_t, kdb_sno_t};
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
        /* #pragma ident        "@(#)kdb_log.h  1.3     04/02/23 SMI" */
        /*
 * DB macros
 */
        /*
 * Current DB version #
 */
        /*
 * DB log states
 */
        /*
 * DB log constants
 */
        /*
 * Default ulog file attributes
 */
        /* in seconds */
        /*
 * Max size of update entry + update header
 * We make this large since resizing can be costly.
 */
        /* Default size of principal record */
        /* 256 MB log file */
        /*
 * Prototype declarations
 */
        #[no_mangle]
        #[c2rust::src_loc = "61:1"]
        pub fn ulog_map(context: krb5_context, logname: *const libc::c_char,
                        entries: uint32_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn ulog_set_role(ctx: krb5_context, role: iprop_role)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn ulog_fini(context: krb5_context);
    }
    /* !_KDB_LOG_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:61"]
pub mod iprop_h {
    #[c2rust::src_loc = "22:1"]
    pub type kdb_sno_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:8"]
    pub struct kdbe_time_t {
        pub seconds: uint32_t,
        pub useconds: uint32_t,
    }
    use super::stdint_uintn_h::uint32_t;
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop_hdr.h:61"]
pub mod iprop_hdr_h {
    #[c2rust::src_loc = "32:1"]
    pub type iprop_role = libc::c_uint;
    #[c2rust::src_loc = "35:5"]
    pub const IPROP_REPLICA: iprop_role = 2;
    #[c2rust::src_loc = "34:5"]
    pub const IPROP_MASTER: iprop_role = 1;
    #[c2rust::src_loc = "33:5"]
    pub const IPROP_NULL: iprop_role = 0;
    /* !_IPROP_HDR_H */
    /*
 * Full resync dump versioning
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:56"]
pub mod k5_err_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-err.h */
/*
 * Copyright 2006, 2007 Massachusetts Institute of Technology.
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
 *
 * Error-message handling
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct errinfo {
        pub code: libc::c_long,
        pub msg: *mut libc::c_char,
    }
    /* K5_ERR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:56"]
pub mod k5_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2006 Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 */
    /* Just those definitions which are needed by util/support/plugins.c,
   which gets compiled before util/et is built, which happens before
   we can construct krb5.h, which is included by k5-int.h.

   So, no krb5 types.  */
    /*
 * Plugins normally export fixed symbol names, but when statically
 * linking plugins, we need a different symbol name for each plugin.
 * The first argument to PLUGIN_SYMBOL_NAME acts as the
 * differentiator, and is only used for static plugin linking.
 *
 * Although this macro (and thus this header file) are used in plugins
 * whose code lies inside the krb5 tree, plugins maintained separately
 * from the krb5 tree do not need it; they can just use the fixed
 * symbol name unconditionally.
 */
    /* opaque */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct plugin_dir_handle {
        pub files: *mut *mut plugin_file_handle,
    }
    extern "C" {
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:56"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:56"]
pub mod com_err_h {
    /*
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * Copyright 1995 by Cygnus Support.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */
    /* Header file for common error description library. */
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    #[c2rust::src_loc = "27:1"]
    pub type et_old_error_hook_func
        =
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: errcode_t,
                                    _: *const libc::c_char,
                                    _: *mut __va_list_tag) -> ()>;
    use super::internal::__va_list_tag;
    extern "C" {
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
        /*@modifies internalState@*/
        /*@modifies internalState@*/
        /*
 * The display routine should be application specific.  A global hook,
 * may cause inappropriate display procedures to be called between
 * applications under non-Unix environments.
 */
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn set_com_err_hook(_: et_old_error_hook_func)
         -> et_old_error_hook_func;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:57"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    /* S4U2Self OK */
    /* Creation flags */
    /* Entry get flags */
/* Name canonicalization requested */
    /* Include authorization data generated by backend */
    /* Is AS-REQ (client referrals only) */
    /* Map cross-realm principals */
    /* Protocol transition */
    /* Constrained delegation */
    /* User-to-user */
    /* Cross-realm */
    /* Issuing referral */
    /* KDB iteration flags */
    /* String attribute names recognized by krb5 */
    /*
 * Note --- these structures cannot be modified without changing the
 * database version number in libkdb.a, but should be expandable by
 * adding new tl_data types.
 */
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    /* NOT saved */
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:16"]
    pub struct _krb5_db_entry_new {
        pub magic: krb5_magic,
        pub len: krb5_ui_2,
        pub mask: krb5_ui_4,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_tl_data: krb5_int16,
        pub n_key_data: krb5_int16,
        pub e_length: krb5_ui_2,
        pub e_data: *mut krb5_octet,
        pub princ: krb5_principal,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /*
 * A principal database entry.  Extensions to this structure currently use the
 * tl_data list.  The e_data and e_length fields are not used by any calling
 * code except kdb5_util dump and load, which marshal and unmarshal the array
 * in the dump record.  KDB modules may use these fields internally as long as
 * they set e_length appropriately (non-zero if the data should be marshalled
 * across dump and load, zero if not) and handle null e_data values in
 * caller-constructed principal entries.
 */
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_magic,
                        krb5_ui_4, krb5_flags, krb5_deltat, krb5_timestamp,
                        krb5_kvno, krb5_principal, krb5_enctype, krb5_int32,
                        krb5_context, krb5_error_code, krb5_principal_data,
                        krb5_const_principal, krb5_boolean, krb5_data,
                        krb5_keyblock};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "358:1"]
        pub fn krb5_db_open(kcontext: krb5_context,
                            db_args: *mut *mut libc::c_char,
                            mode: libc::c_int) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "363:1"]
        pub fn krb5_db_fini(kcontext: krb5_context) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "370:1"]
        pub fn krb5_db_get_principal(kcontext: krb5_context,
                                     search_for: krb5_const_principal,
                                     flags: libc::c_uint,
                                     entry: *mut *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn krb5_db_free_principal(kcontext: krb5_context,
                                      entry: *mut krb5_db_entry);
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn krb5_db_put_principal(kcontext: krb5_context,
                                     entry: *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "404:1"]
        pub fn krb5_db_fetch_mkey(context: krb5_context,
                                  mname: krb5_principal, etype: krb5_enctype,
                                  fromkeyboard: krb5_boolean,
                                  twice: krb5_boolean,
                                  db_args: *mut libc::c_char,
                                  kvno: *mut krb5_kvno, salt: *mut krb5_data,
                                  key: *mut krb5_keyblock) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "413:1"]
        pub fn krb5_db_fetch_mkey_list(context: krb5_context,
                                       mname: krb5_principal,
                                       mkey: *const krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn krb5_db_setup_mkey_name(context: krb5_context,
                                       keyname: *const libc::c_char,
                                       realm: *const libc::c_char,
                                       fullname: *mut *mut libc::c_char,
                                       principal: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "472:1"]
        pub fn krb5_dbe_find_mkey(context: krb5_context,
                                  entry: *mut krb5_db_entry,
                                  mkey: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn krb5_dbe_update_last_pwd_change(context: krb5_context,
                                               entry: *mut krb5_db_entry,
                                               stamp: krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "631:1"]
        pub fn krb5_dbe_ark(context: krb5_context,
                            master_key: *mut krb5_keyblock,
                            ks_tuple: *mut krb5_key_salt_tuple,
                            ks_tuple_count: libc::c_int,
                            db_entry: *mut krb5_db_entry) -> krb5_error_code;
    }
    /* NOT saved */
    /* members currently changed/set */
    /* When the client expires */
    /* When its passwd expires */
    /* Last successful passwd */
    /* Last failed passwd attempt */
    /* # of failed passwd attempt */
    /* Length of extra data */
    /* Extra data to be saved */
    /* Length, data */
    /* Linked list */
    /* key_data must be sorted by kvno in descending order. */
    /* Array */
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:57"]
pub mod admin_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _kadm5_config_params {
        pub mask: libc::c_long,
        pub realm: *mut libc::c_char,
        pub kadmind_port: libc::c_int,
        pub kpasswd_port: libc::c_int,
        pub admin_server: *mut libc::c_char,
        pub dbname: *mut libc::c_char,
        pub acl_file: *mut libc::c_char,
        pub dict_file: *mut libc::c_char,
        pub mkey_from_kbd: libc::c_int,
        pub stash_file: *mut libc::c_char,
        pub mkey_name: *mut libc::c_char,
        pub enctype: krb5_enctype,
        pub max_life: krb5_deltat,
        pub max_rlife: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub flags: krb5_flags,
        pub keysalts: *mut krb5_key_salt_tuple,
        pub num_keysalts: krb5_int32,
        pub kvno: krb5_kvno,
        pub iprop_enabled: libc::c_int,
        pub iprop_ulogsize: uint32_t,
        pub iprop_poll_time: krb5_deltat,
        pub iprop_logfile: *mut libc::c_char,
        pub iprop_port: libc::c_int,
        pub iprop_resync_timeout: libc::c_int,
        pub kadmind_listen: *mut libc::c_char,
        pub kpasswd_listen: *mut libc::c_char,
        pub iprop_listen: *mut libc::c_char,
    }
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    use super::krb5_h::{krb5_enctype, krb5_deltat, krb5_timestamp, krb5_flags,
                        krb5_int32, krb5_kvno, krb5_context, krb5_error_code};
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "294:1"]
        pub fn kadm5_get_config_params(context: krb5_context,
                                       use_kdc_config: libc::c_int,
                                       params_in: *mut kadm5_config_params,
                                       params_out: *mut kadm5_config_params)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "299:1"]
        pub fn kadm5_free_config_params(context: krb5_context,
                                        params: *mut kadm5_config_params)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "465:1"]
        pub fn kadm5_init_krb5_context(_: *mut krb5_context)
         -> krb5_error_code;
    }
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src = "/usr/include/string.h:56"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "252:14"]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:56"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:56"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
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
        #[no_mangle]
        #[c2rust::src_loc = "341:12"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: ::std::ffi::VaList) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:56"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:56"]
pub mod stat_h {
    use super::types_h::__mode_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "308:1"]
        pub fn umask(__mask: __mode_t) -> __mode_t;
    }
}
#[c2rust::header_src = "/usr/include/locale.h:58"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:59"]
pub mod adm_proto_h {
    use super::krb5_h::{krb5_boolean, krb5_int32, krb5_error_code};
    use super::kdb_h::krb5_key_salt_tuple;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn krb5_string_to_keysalts(_: *const libc::c_char,
                                       _: *const libc::c_char,
                                       _: *const libc::c_char,
                                       _: krb5_boolean,
                                       _: *mut *mut krb5_key_salt_tuple,
                                       _: *mut krb5_int32) -> krb5_error_code;
    }
    /* KRB5_ADM_PROTO_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/dbutil/kdb5_util.h:61"]
pub mod kdb5_util_h {
    use super::krb5_h::{krb5_principal_data, krb5_principal};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "35:23"]
        pub static mut master_princ: krb5_principal;
        #[no_mangle]
        #[c2rust::src_loc = "82:1"]
        pub fn kdb5_add_mkey(argc: libc::c_int, argv: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn kdb5_use_mkey(argc: libc::c_int, argv: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn kdb5_list_mkeys(argc: libc::c_int,
                               argv: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "85:1"]
        pub fn kdb5_update_princ_encryption(argc: libc::c_int,
                                            argv: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "86:1"]
        pub fn tabdump(argc: libc::c_int, argv: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn kdb5_purge_mkeys(argc: libc::c_int,
                                argv: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn kdb5_stash(argc: libc::c_int, argv: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn kdb5_destroy(argc: libc::c_int, argv: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn load_db(argc: libc::c_int, argv: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "78:1"]
        pub fn dump_db(argc: libc::c_int, argv: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn kdb5_create(argc: libc::c_int, argv: *mut *mut libc::c_char);
    }
}
pub use self::internal::__va_list_tag;
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __mode_t, __off_t, __off64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _profile_t, krb5_c_random_seed,
                       krb5_c_string_to_key, krb5_c_valid_enctype,
                       krb5_free_context, krb5_parse_name,
                       krb5_principal2salt, krb5_free_principal,
                       krb5_free_keyblock_contents, krb5_timeofday,
                       krb5_get_default_realm, krb5_set_default_realm,
                       krb5_free_default_realm, krb5_string_to_enctype,
                       krb5_get_error_message, krb5_free_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, zapfree, plugin_mapping,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle};
pub use self::kdb_log_h::{_kdb_log_context, kdb_hlog_t, kdb_hlog, ulog_map,
                          ulog_set_role, ulog_fini};
pub use self::iprop_h::{kdb_sno_t, kdbe_time_t};
pub use self::iprop_hdr_h::{iprop_role, IPROP_REPLICA, IPROP_MASTER,
                            IPROP_NULL};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, et_old_error_hook_func, com_err,
                          set_com_err_hook};
pub use self::kdb_h::{_krb5_key_data, _krb5_tl_data, krb5_tl_data,
                      krb5_key_data, _krb5_db_entry_new, krb5_db_entry,
                      __krb5_key_salt_tuple, krb5_key_salt_tuple,
                      krb5_db_open, krb5_db_fini, krb5_db_get_principal,
                      krb5_db_free_principal, krb5_db_put_principal,
                      krb5_db_fetch_mkey, krb5_db_fetch_mkey_list,
                      krb5_db_setup_mkey_name, krb5_dbe_find_mkey,
                      krb5_dbe_update_last_pwd_change, krb5_dbe_ark};
pub use self::admin_h::{_kadm5_config_params, kadm5_config_params,
                        kadm5_get_config_params, kadm5_free_config_params,
                        kadm5_init_krb5_context};
use self::string_h::{memset, strcmp, strrchr, strlen, explicit_bzero};
use self::stdlib_h::{atoi, malloc, realloc, free, exit};
use self::stdio_h::{stderr, fprintf, printf, vfprintf, asprintf};
use self::libintl_h::dgettext;
use self::stat_h::umask;
use self::locale_h::setlocale;
use self::adm_proto_h::krb5_string_to_keysalts;
use self::kdb5_util_h::{master_princ, kdb5_add_mkey, kdb5_use_mkey,
                        kdb5_list_mkeys, kdb5_update_princ_encryption,
                        tabdump, kdb5_purge_mkeys, kdb5_stash, kdb5_destroy,
                        load_db, dump_db, kdb5_create};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "126:8"]
pub struct _cmd_table {
    pub name: *mut libc::c_char,
    pub func: cmd_func,
    pub opendb: libc::c_int,
}
#[c2rust::src_loc = "124:1"]
pub type cmd_func
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char)
               -> ()>;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/dbutil/kdb5_util.c - Administer a KDC database */
/*
 * (C) Copyright 1990,1991, 1996, 2008, 2009 by the Massachusetts Institute of Technology.
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
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
/*
 * Copyright 2009 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/*
 * XXX Ick, ick, ick.  These global variables shouldn't be global....
 */
#[no_mangle]
#[c2rust::src_loc = "66:7"]
pub static mut mkey_password: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * I can't figure out any way for this not to be global, given how ss
 * works.
 */
#[no_mangle]
#[c2rust::src_loc = "73:5"]
pub static mut exit_status: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "74:14"]
pub static mut util_context: krb5_context =
    0 as *const _krb5_context as *mut _krb5_context;
#[no_mangle]
#[c2rust::src_loc = "75:21"]
pub static mut global_params: kadm5_config_params =
    kadm5_config_params{mask: 0,
                        realm: 0 as *const libc::c_char as *mut libc::c_char,
                        kadmind_port: 0,
                        kpasswd_port: 0,
                        admin_server:
                            0 as *const libc::c_char as *mut libc::c_char,
                        dbname: 0 as *const libc::c_char as *mut libc::c_char,
                        acl_file:
                            0 as *const libc::c_char as *mut libc::c_char,
                        dict_file:
                            0 as *const libc::c_char as *mut libc::c_char,
                        mkey_from_kbd: 0,
                        stash_file:
                            0 as *const libc::c_char as *mut libc::c_char,
                        mkey_name:
                            0 as *const libc::c_char as *mut libc::c_char,
                        enctype: 0,
                        max_life: 0,
                        max_rlife: 0,
                        expiration: 0,
                        flags: 0,
                        keysalts:
                            0 as *const krb5_key_salt_tuple as
                                *mut krb5_key_salt_tuple,
                        num_keysalts: 0,
                        kvno: 0,
                        iprop_enabled: 0,
                        iprop_ulogsize: 0,
                        iprop_poll_time: 0,
                        iprop_logfile:
                            0 as *const libc::c_char as *mut libc::c_char,
                        iprop_port: 0,
                        iprop_resync_timeout: 0,
                        kadmind_listen:
                            0 as *const libc::c_char as *mut libc::c_char,
                        kpasswd_listen:
                            0 as *const libc::c_char as *mut libc::c_char,
                        iprop_listen:
                            0 as *const libc::c_char as *mut libc::c_char,};
#[no_mangle]
#[c2rust::src_loc = "77:1"]
pub unsafe extern "C" fn usage() {
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"Usage: kdb5_util [-r realm] [-d dbname] [-k mkeytype] [-kv mkeyVNO]\n\t        [-M mkeyname] [-m] [-sf stashfilename] [-P password]\n\t        [-x db_args]* cmd [cmd_options]\n\tcreate  [-s]\n\tdestroy [-f]\n\tstash   [-f keyfile]\n\tdump    [-old|-b6|-b7|-r13|-r18] [-verbose]\n\t        [-mkey_convert] [-new_mkey_file mkey_file]\n\t        [-rev] [-recurse] [filename [princs...]]\n\tload    [-old|-b6|-b7|-r13|-r18] [-verbose] [-update] filename\n\tark     [-e etype_list] principal\n\tadd_mkey [-e etype] [-s]\n\tuse_mkey kvno [time]\n\tlist_mkeys\n\x00"
                         as *const u8 as *const libc::c_char));
    /* avoid a string length compiler warning */
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\tupdate_princ_encryption [-f] [-n] [-v] [princ-pattern]\n\tpurge_mkeys [-f] [-n] [-v]\n\ttabdump [-H] [-c] [-e] [-n] [-o outfile] dumptype\n\nwhere,\n\t[-x db_args]* - any number of database specific arguments.\n\t\t\tLook at each database documentation for supported arguments\n\x00"
                         as *const u8 as *const libc::c_char));
    exit(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "109:15"]
pub static mut master_keyblock: krb5_keyblock =
    krb5_keyblock{magic: 0,
                  enctype: 0,
                  length: 0,
                  contents: 0 as *const krb5_octet as *mut krb5_octet,};
#[no_mangle]
#[c2rust::src_loc = "110:13"]
pub static mut master_kvno: krb5_kvno = 0;
/* fetched */
#[no_mangle]
#[c2rust::src_loc = "112:7"]
pub static mut mkey_fullname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "113:16"]
pub static mut master_entry: *mut krb5_db_entry =
    0 as *const krb5_db_entry as *mut krb5_db_entry;
#[no_mangle]
#[c2rust::src_loc = "114:9"]
pub static mut valid_master_key: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "116:7"]
pub static mut progname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "117:14"]
pub static mut manual_mkey: krb5_boolean = 0 as libc::c_int as krb5_boolean;
#[no_mangle]
#[c2rust::src_loc = "118:14"]
pub static mut dbactive: krb5_boolean = 0 as libc::c_int as krb5_boolean;
#[no_mangle]
#[c2rust::src_loc = "130:3"]
pub static mut cmd_table: [_cmd_table; 13] =
    unsafe {
        [{
             let mut init =
                 _cmd_table{name:
                                b"create\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            func:
                                Some(kdb5_create as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                b"destroy\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            func:
                                Some(kdb5_destroy as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                b"stash\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            func:
                                Some(kdb5_stash as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                b"dump\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            func:
                                Some(dump_db as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                b"load\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            func:
                                Some(load_db as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 0 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                b"ark\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            func:
                                Some(add_random_key as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                b"add_mkey\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            func:
                                Some(kdb5_add_mkey as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                b"use_mkey\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            func:
                                Some(kdb5_use_mkey as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                b"list_mkeys\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            func:
                                Some(kdb5_list_mkeys as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                b"update_princ_encryption\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            func:
                                Some(kdb5_update_princ_encryption as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                b"purge_mkeys\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            func:
                                Some(kdb5_purge_mkeys as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                b"tabdump\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                            func:
                                Some(tabdump as
                                         unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> ()),
                            opendb: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 _cmd_table{name:
                                0 as *const libc::c_char as *mut libc::c_char,
                            func: None,
                            opendb: 0 as libc::c_int,};
             init
         }]
    };
#[c2rust::src_loc = "146:1"]
unsafe extern "C" fn cmd_lookup(mut name: *mut libc::c_char)
 -> *mut _cmd_table {
    let mut cmd: *mut _cmd_table = cmd_table.as_mut_ptr();
    while !(*cmd).name.is_null() {
        if strcmp((*cmd).name, name) == 0 as libc::c_int {
            return cmd
        } else { cmd = cmd.offset(1) }
    }
    return 0 as *mut _cmd_table;
}
#[no_mangle]
#[c2rust::src_loc = "162:8"]
pub static mut db5util_db_args: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "163:8"]
pub static mut db5util_db_args_size: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "165:1"]
unsafe extern "C" fn extended_com_err_fn(mut myprog: *const libc::c_char,
                                         mut code: errcode_t,
                                         mut fmt: *const libc::c_char,
                                         mut args: ::std::ffi::VaList) {
    let mut emsg: *const libc::c_char = 0 as *const libc::c_char;
    if code != 0 {
        emsg = krb5_get_error_message(util_context, code as krb5_error_code);
        fprintf(stderr, b"%s: %s \x00" as *const u8 as *const libc::c_char,
                myprog, emsg);
        krb5_free_error_message(util_context, emsg);
    } else {
        fprintf(stderr, b"%s: \x00" as *const u8 as *const libc::c_char,
                myprog);
    }
    vfprintf(stderr, fmt, args.as_va_list());
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "180:1"]
pub unsafe extern "C" fn add_db_arg(mut arg: *mut libc::c_char)
 -> libc::c_int {
    let mut temp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    db5util_db_args_size += 1;
    temp =
        realloc(db5util_db_args as *mut libc::c_void,
                (::std::mem::size_of::<*mut libc::c_char>() as
                     libc::c_ulong).wrapping_mul((db5util_db_args_size +
                                                      1 as libc::c_int) as
                                                     libc::c_ulong)) as
            *mut *mut libc::c_char;
    if temp.is_null() { return 0 as libc::c_int }
    db5util_db_args = temp;
    let ref mut fresh0 =
        *db5util_db_args.offset((db5util_db_args_size - 1 as libc::c_int) as
                                    isize);
    *fresh0 = arg;
    let ref mut fresh1 =
        *db5util_db_args.offset(db5util_db_args_size as isize);
    *fresh1 = 0 as *mut libc::c_char;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "194:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut cmd: *mut _cmd_table = 0 as *mut _cmd_table;
    let mut koptarg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut db_name_tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd_argc: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    setlocale(6 as libc::c_int, b"\x00" as *const u8 as *const libc::c_char);
    set_com_err_hook(Some(extended_com_err_fn as
                              unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: errcode_t,
                                                   _: *const libc::c_char,
                                                   _: ::std::ffi::VaList)
                                  -> ()));
    /*
     * Ensure that "progname" is set before calling com_err.
     */
    progname =
        if !strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).is_null() {
            strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).offset(1 as libc::c_int as isize)
        } else { *argv.offset(0 as libc::c_int as isize) };
    retval = kadm5_init_krb5_context(&mut util_context);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing Kerberos code\x00" as *const u8
                             as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    cmd_argv =
        malloc((::std::mem::size_of::<*mut libc::c_char>() as
                    libc::c_ulong).wrapping_mul(argc as libc::c_ulong)) as
            *mut *mut libc::c_char;
    if cmd_argv.is_null() {
        com_err(progname, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while creating sub-command arguments\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    memset(cmd_argv as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut libc::c_char>() as
                libc::c_ulong).wrapping_mul(argc as libc::c_ulong));
    cmd_argc = 0 as libc::c_int;
    argv = argv.offset(1);
    argc -= 1;
    while !(*argv).is_null() {
        if strcmp(*argv, b"-P\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int &&
               {
                   argc -= 1;
                   !(if argc > 0 as libc::c_int {
                         argv = argv.offset(1);
                         koptarg = *argv;
                         koptarg
                     } else {
                         usage();
                         0 as *mut libc::c_void as *mut libc::c_char
                     }).is_null()
               } {
            mkey_password = koptarg;
            manual_mkey = 1 as libc::c_int as krb5_boolean
        } else if strcmp(*argv, b"-d\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int &&
                      {
                          argc -= 1;
                          !(if argc > 0 as libc::c_int {
                                argv = argv.offset(1);
                                koptarg = *argv;
                                koptarg
                            } else {
                                usage();
                                0 as *mut libc::c_void as *mut libc::c_char
                            }).is_null()
                      } {
            global_params.dbname = koptarg;
            global_params.mask |= 0x2 as libc::c_int as libc::c_long;
            if asprintf(&mut db_name_tmp as *mut *mut libc::c_char,
                        b"dbname=%s\x00" as *const u8 as *const libc::c_char,
                        global_params.dbname) < 0 as libc::c_int {
                com_err(progname, 12 as libc::c_int as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while parsing command arguments\x00" as
                                     *const u8 as *const libc::c_char));
                exit(1 as libc::c_int);
            }
            if add_db_arg(db_name_tmp) == 0 {
                com_err(progname, 12 as libc::c_int as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while parsing command arguments\n\x00" as
                                     *const u8 as *const libc::c_char));
                exit(1 as libc::c_int);
            }
        } else if strcmp(*argv, b"-x\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int &&
                      {
                          argc -= 1;
                          !(if argc > 0 as libc::c_int {
                                argv = argv.offset(1);
                                koptarg = *argv;
                                koptarg
                            } else {
                                usage();
                                0 as *mut libc::c_void as *mut libc::c_char
                            }).is_null()
                      } {
            if add_db_arg(koptarg) == 0 {
                com_err(progname, 12 as libc::c_int as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while parsing command arguments\n\x00" as
                                     *const u8 as *const libc::c_char));
                exit(1 as libc::c_int);
            }
        } else if strcmp(*argv, b"-r\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int &&
                      {
                          argc -= 1;
                          !(if argc > 0 as libc::c_int {
                                argv = argv.offset(1);
                                koptarg = *argv;
                                koptarg
                            } else {
                                usage();
                                0 as *mut libc::c_void as *mut libc::c_char
                            }).is_null()
                      } {
            global_params.realm = koptarg;
            global_params.mask |= 0x1 as libc::c_int as libc::c_long;
            /* not sure this is really necessary */
            retval =
                krb5_set_default_realm(util_context, global_params.realm);
            if retval != 0 {
                com_err(progname, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while setting default realm name\x00" as
                                     *const u8 as *const libc::c_char));
                exit(1 as libc::c_int);
            }
        } else if strcmp(*argv, b"-k\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int &&
                      {
                          argc -= 1;
                          !(if argc > 0 as libc::c_int {
                                argv = argv.offset(1);
                                koptarg = *argv;
                                koptarg
                            } else {
                                usage();
                                0 as *mut libc::c_void as *mut libc::c_char
                            }).is_null()
                      } {
            if krb5_string_to_enctype(koptarg, &mut global_params.enctype) !=
                   0 {
                com_err(progname, 22 as libc::c_int as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b": %s is an invalid enctype\x00" as
                                     *const u8 as *const libc::c_char),
                        koptarg);
                exit(1 as libc::c_int);
            } else {
                global_params.mask |= 0x200 as libc::c_int as libc::c_long
            }
        } else if strcmp(*argv,
                         b"-kv\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int &&
                      {
                          argc -= 1;
                          !(if argc > 0 as libc::c_int {
                                argv = argv.offset(1);
                                koptarg = *argv;
                                koptarg
                            } else {
                                usage();
                                0 as *mut libc::c_void as *mut libc::c_char
                            }).is_null()
                      } {
            global_params.kvno = atoi(koptarg) as krb5_kvno;
            if global_params.kvno == 0 as libc::c_int as libc::c_uint {
                com_err(progname, 22 as libc::c_int as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b": %s is an invalid mkeyVNO\x00" as
                                     *const u8 as *const libc::c_char),
                        koptarg);
                exit(1 as libc::c_int);
            } else {
                global_params.mask |=
                    0x20000000 as libc::c_int as libc::c_long
            }
        } else if strcmp(*argv, b"-M\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int &&
                      {
                          argc -= 1;
                          !(if argc > 0 as libc::c_int {
                                argv = argv.offset(1);
                                koptarg = *argv;
                                koptarg
                            } else {
                                usage();
                                0 as *mut libc::c_void as *mut libc::c_char
                            }).is_null()
                      } {
            global_params.mkey_name = koptarg;
            global_params.mask |= 0x4 as libc::c_int as libc::c_long
        } else if strcmp(*argv,
                         b"-sf\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int &&
                      {
                          argc -= 1;
                          !(if argc > 0 as libc::c_int {
                                argv = argv.offset(1);
                                koptarg = *argv;
                                koptarg
                            } else {
                                usage();
                                0 as *mut libc::c_void as *mut libc::c_char
                            }).is_null()
                      } {
            global_params.stash_file = koptarg;
            global_params.mask |= 0x100 as libc::c_int as libc::c_long
        } else if strcmp(*argv, b"-m\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            manual_mkey = 1 as libc::c_int as krb5_boolean;
            global_params.mkey_from_kbd = 1 as libc::c_int;
            global_params.mask |= 0x40000 as libc::c_int as libc::c_long
        } else {
            let fresh2 = cmd_argc;
            cmd_argc = cmd_argc + 1;
            let ref mut fresh3 = *cmd_argv.offset(fresh2 as isize);
            *fresh3 = *argv
        }
        argv = argv.offset(1);
        argc -= 1
    }
    if (*cmd_argv.offset(0 as libc::c_int as isize)).is_null() { usage(); }
    cmd = cmd_lookup(*cmd_argv.offset(0 as libc::c_int as isize));
    if cmd.is_null() { usage(); }
    if (*util_context).default_realm.is_null() {
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        retval = krb5_get_default_realm(util_context, &mut temp);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while getting default realm\x00" as *const u8
                                 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
        krb5_free_default_realm(util_context, temp);
    }
    retval =
        kadm5_get_config_params(util_context, 1 as libc::c_int,
                                &mut global_params, &mut global_params);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while retreiving configuration parameters\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /*
     * Dump creates files which should not be world-readable.  It is
     * easiest to do a single umask call here.
     */
    umask(0o77 as libc::c_int as __mode_t);
    master_keyblock.enctype = global_params.enctype;
    if master_keyblock.enctype != 0x1ff as libc::c_int &&
           krb5_c_valid_enctype(master_keyblock.enctype) == 0 {
        com_err(progname, -(1765328233 as libc::c_long),
                b"while setting up enctype %d\x00" as *const u8 as
                    *const libc::c_char, master_keyblock.enctype);
    }
    if (*cmd).opendb != 0 && open_db_and_mkey() != 0 { return exit_status }
    if global_params.iprop_enabled == 1 as libc::c_int {
        ulog_set_role(util_context, IPROP_MASTER);
    } else { ulog_set_role(util_context, IPROP_NULL); }
    Some((*cmd).func.expect("non-null function pointer")).expect("non-null function pointer")(cmd_argc,
                                                                                              cmd_argv);
    if !db_name_tmp.is_null() { free(db_name_tmp as *mut libc::c_void); }
    if !db5util_db_args.is_null() {
        free(db5util_db_args as *mut libc::c_void);
    }
    quit();
    kadm5_free_config_params(util_context, &mut global_params);
    krb5_free_context(util_context);
    free(cmd_argv as *mut libc::c_void);
    return exit_status;
}
/*
 * open_db_and_mkey: Opens the KDC and policy database, and sets the
 * global master_* variables.  Sets dbactive to TRUE if the databases
 * are opened, and valid_master_key to 1 if the global master
 * variables are set properly.  Returns 0 on success, and 1 on
 * failure, but it is not considered a failure if the master key
 * cannot be fetched (the master key stash file may not exist when the
 * program is run).
 */
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn open_db_and_mkey() -> libc::c_int {
    let mut retval: krb5_error_code = 0;
    let mut scratch: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut pwd: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut seed: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    dbactive = 0 as libc::c_int as krb5_boolean;
    valid_master_key = 0 as libc::c_int;
    retval =
        krb5_db_open(util_context, db5util_db_args,
                     0 as libc::c_int | 0x200 as libc::c_int);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing database\x00" as *const u8 as
                             *const libc::c_char));
        exit_status += 1;
        return 1 as libc::c_int
    }
    /* assemble & parse the master key name */
    retval =
        krb5_db_setup_mkey_name(util_context, global_params.mkey_name,
                                global_params.realm, &mut mkey_fullname,
                                &mut master_princ); /* user specified */
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while setting up master key name\x00" as *const u8
                             as *const libc::c_char));
        exit_status += 1;
        return 1 as libc::c_int
    }
    retval =
        krb5_db_get_principal(util_context,
                              master_princ as krb5_const_principal,
                              0 as libc::c_int as libc::c_uint,
                              &mut master_entry);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while retrieving master entry\x00" as *const u8 as
                             *const libc::c_char));
        exit_status += 1;
        krb5_db_fini(util_context);
        return 1 as libc::c_int
    }
    if global_params.mask & 0x20000000 as libc::c_int as libc::c_long != 0 {
        master_kvno = global_params.kvno
    } else { master_kvno = 0 as libc::c_int as krb5_kvno }
    /* the databases are now open, and the master principal exists */
    dbactive = 1 as libc::c_int as krb5_boolean;
    if !mkey_password.is_null() {
        pwd.data = mkey_password;
        pwd.length = strlen(mkey_password) as libc::c_uint;
        retval =
            krb5_principal2salt(util_context,
                                master_princ as krb5_const_principal,
                                &mut scratch);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while calculated master key salt\x00" as
                                 *const u8 as *const libc::c_char));
            exit_status += 1;
            return 1 as libc::c_int
        }
        /* If no encryption type is set, use the default */
        if master_keyblock.enctype == 0x1ff as libc::c_int {
            master_keyblock.enctype = 0x12 as libc::c_int
        }
        if krb5_c_valid_enctype(master_keyblock.enctype) == 0 {
            com_err(progname, -(1765328233 as libc::c_long),
                    b"while setting up enctype %d\x00" as *const u8 as
                        *const libc::c_char, master_keyblock.enctype);
        }
        retval =
            krb5_c_string_to_key(util_context, master_keyblock.enctype,
                                 &mut pwd, &mut scratch,
                                 &mut master_keyblock);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while transforming master key from password\x00"
                                 as *const u8 as *const libc::c_char));
            exit_status += 1;
            return 1 as libc::c_int
        }
        free(scratch.data as *mut libc::c_void);
        mkey_password = 0 as *mut libc::c_char
    } else {
        retval =
            krb5_db_fetch_mkey(util_context, master_princ,
                               master_keyblock.enctype, manual_mkey,
                               0 as libc::c_int as krb5_boolean,
                               global_params.stash_file, &mut master_kvno,
                               0 as *mut krb5_data, &mut master_keyblock);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while reading master key\x00" as *const u8 as
                                 *const libc::c_char));
            com_err(progname, 0 as libc::c_int as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Warning: proceeding without master key\x00" as
                                 *const u8 as *const libc::c_char));
            exit_status += 1;
            return 0 as libc::c_int
        }
    }
    retval =
        krb5_db_fetch_mkey_list(util_context, master_princ,
                                &mut master_keyblock);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                b"while getting master key list\x00" as *const u8 as
                    *const libc::c_char);
        com_err(progname, 0 as libc::c_int as errcode_t,
                b"Warning: proceeding without master key list\x00" as
                    *const u8 as *const libc::c_char);
        exit_status += 1;
        return 0 as libc::c_int
    }
    seed.length = master_keyblock.length;
    seed.data = master_keyblock.contents as *mut libc::c_char;
    retval = krb5_c_random_seed(util_context, &mut seed);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while seeding random number generator\x00" as
                             *const u8 as *const libc::c_char));
        exit_status += 1;
        memset(master_keyblock.contents as *mut libc::c_void,
               0 as libc::c_int, master_keyblock.length as libc::c_ulong);
        krb5_free_keyblock_contents(util_context, &mut master_keyblock);
        return 1 as libc::c_int
    }
    if global_params.iprop_enabled != 0 {
        if ulog_map(util_context, global_params.iprop_logfile,
                    global_params.iprop_ulogsize) != 0 {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"%s: Could not map log\n\x00" as *const u8 as
                                 *const libc::c_char), progname);
            exit_status += 1;
            return 1 as libc::c_int
        }
    }
    valid_master_key = 1 as libc::c_int;
    dbactive = 1 as libc::c_int as krb5_boolean;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "489:1"]
pub unsafe extern "C" fn quit() -> libc::c_int {
    let mut retval: krb5_error_code = 0;
    static mut finished: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    if finished != 0 { return 0 as libc::c_int }
    ulog_fini(util_context);
    retval = krb5_db_fini(util_context);
    zapfree(master_keyblock.contents as *mut libc::c_void,
            master_keyblock.length as size_t);
    krb5_free_principal(util_context, master_princ);
    finished = 1 as libc::c_int as krb5_boolean;
    if retval != 0 && retval as libc::c_long != -(1780008435 as libc::c_long)
       {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while closing database\x00" as *const u8 as
                             *const libc::c_char));
        exit_status += 1;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "510:1"]
unsafe extern "C" fn add_random_key(mut argc: libc::c_int,
                                    mut argv: *mut *mut libc::c_char) {
    let mut ret: krb5_error_code = 0;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut dbent: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut now: krb5_timestamp = 0;
    let mut keysalts: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    let mut num_keysalts: krb5_int32 = 0 as libc::c_int;
    let mut free_keysalts: libc::c_int = 0;
    let mut me: *mut libc::c_char = progname;
    let mut ks_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pr_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_mkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    if argc < 2 as libc::c_int { usage(); }
    argv = argv.offset(1);
    argc -= 1;
    while !(*argv).is_null() {
        if !(strcmp(*argv, b"-e\x00" as *const u8 as *const libc::c_char) ==
                 0) {
            break ;
        }
        argv = argv.offset(1);
        argc -= 1;
        ks_str = *argv;
        argv = argv.offset(1);
        argc -= 1
    }
    if argc < 1 as libc::c_int { usage(); }
    pr_str = *argv;
    ret = krb5_parse_name(util_context, pr_str, &mut princ);
    if ret != 0 {
        com_err(me, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing principal name %s\x00" as *const u8
                             as *const libc::c_char), pr_str);
        exit_status += 1;
        return
    }
    ret =
        krb5_db_get_principal(util_context, princ as krb5_const_principal,
                              0 as libc::c_int as libc::c_uint, &mut dbent);
    if ret != 0 {
        com_err(me, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while fetching principal %s\x00" as *const u8 as
                             *const libc::c_char), pr_str);
        exit_status += 1;
        return
    }
    ret =
        krb5_string_to_keysalts(ks_str, 0 as *const libc::c_char,
                                0 as *const libc::c_char,
                                0 as libc::c_int as krb5_boolean,
                                &mut keysalts, &mut num_keysalts);
    if ret != 0 {
        com_err(me, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing keysalts %s\x00" as *const u8 as
                             *const libc::c_char), ks_str);
        exit_status += 1;
        return
    }
    if num_keysalts == 0 || keysalts.is_null() {
        num_keysalts = global_params.num_keysalts;
        keysalts = global_params.keysalts;
        free_keysalts = 0 as libc::c_int
    } else { free_keysalts = 1 as libc::c_int }
    /* Find the mkey used to protect the existing keys */
    ret = krb5_dbe_find_mkey(util_context, dbent, &mut tmp_mkey);
    if ret != 0 {
        com_err(me, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while finding mkey\x00" as *const u8 as
                             *const libc::c_char));
        krb5_db_free_principal(util_context, dbent);
        exit_status += 1;
        return
    }
    ret = krb5_dbe_ark(util_context, tmp_mkey, keysalts, num_keysalts, dbent);
    if free_keysalts != 0 { free(keysalts as *mut libc::c_void); }
    if ret != 0 {
        com_err(me, ret as errcode_t,
                b"while randomizing principal %s\x00" as *const u8 as
                    *const libc::c_char, pr_str);
        krb5_db_free_principal(util_context, dbent);
        exit_status += 1;
        return
    }
    (*dbent).attributes &= !(0x200 as libc::c_int);
    ret = krb5_timeofday(util_context, &mut now);
    if ret != 0 {
        com_err(me, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting time\x00" as *const u8 as
                             *const libc::c_char));
        krb5_db_free_principal(util_context, dbent);
        exit_status += 1;
        return
    }
    ret = krb5_dbe_update_last_pwd_change(util_context, dbent, now);
    if ret != 0 {
        com_err(me, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while setting changetime\x00" as *const u8 as
                             *const libc::c_char));
        krb5_db_free_principal(util_context, dbent);
        exit_status += 1;
        return
    }
    ret = krb5_db_put_principal(util_context, dbent);
    krb5_db_free_principal(util_context, dbent);
    if ret != 0 {
        com_err(me, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while saving principal %s\x00" as *const u8 as
                             *const libc::c_char), pr_str);
        exit_status += 1;
        return
    }
    printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                    b"%s changed\n\x00" as *const u8 as *const libc::c_char),
           pr_str);
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
