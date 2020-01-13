use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
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
#[c2rust::header_src = "/usr/include/bits/types.h:7"]
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
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:7"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:7"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:7"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:7"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:7"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:7"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:7"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_tm.h:7"]
pub mod struct_tm_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "7:8"]
    pub struct tm {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
        pub tm_gmtoff: libc::c_long,
        pub tm_zone: *const libc::c_char,
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:7"]
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
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[no_mangle]
        #[c2rust::src_loc = "839:1"]
        pub fn krb5_c_string_to_key(context: krb5_context,
                                    enctype: krb5_enctype,
                                    string: *const krb5_data,
                                    salt: *const krb5_data,
                                    key: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3664:1"]
        pub fn krb5_principal_compare(context: krb5_context,
                                      princ1: krb5_const_principal,
                                      princ2: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "4313:1"]
        pub fn krb5_principal2salt(context: krb5_context,
                                   pr: krb5_const_principal,
                                   ret: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6096:1"]
        pub fn krb5_read_password(context: krb5_context,
                                  prompt: *const libc::c_char,
                                  prompt2: *const libc::c_char,
                                  return_pwd: *mut libc::c_char,
                                  size_return: *mut libc::c_uint)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6266:1"]
        pub fn krb5_string_to_enctype(string: *mut libc::c_char,
                                      enctypep: *mut krb5_enctype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6341:1"]
        pub fn krb5_enctype_to_name(enctype: krb5_enctype,
                                    shortest: krb5_boolean,
                                    buffer: *mut libc::c_char, buflen: size_t)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:7"]
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
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
    }
    #[inline]
    #[c2rust::src_loc = "2338:1"]
    pub unsafe extern "C" fn ts2tt(mut timestamp: krb5_timestamp) -> time_t {
        return timestamp as uint32_t as time_t;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_timestamp};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::kdb_log_h::_kdb_log_context;
    use super::stdint_uintn_h::uint32_t;
    use super::time_t_h::time_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:12"]
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
    use super::iprop_hdr_h::iprop_role;
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::iprop_h::{kdbe_time_t, kdb_sno_t};
    /* !_KDB_LOG_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:12"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop_hdr.h:12"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:7"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:7"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:7"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:7"]
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
    extern "C" {
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:8"]
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
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /* # of array elements */
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
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
    /* Length, data */
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "282:16"]
    pub struct _krb5_actkvno_node {
        pub next: *mut _krb5_actkvno_node,
        pub act_kvno: krb5_kvno,
        pub act_time: krb5_timestamp,
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
    /* SECURID */
    /* String attributes may not always be represented in tl-data.  kadmin clients
 * must use the get_strings and set_string RPCs. */
    /* NDR encoded validation info */
    /* ASN.1 encoded ServerReferralInfo */
    /* ASN.1 encoded PA-SVR-REFERRAL-DATA */
    /* Each entry is a permitted SPN */
    /* LM OWF */
    /* <I>IssuerDN<S>SubjectDN */
    /* Timestamp of admin unlock */
    /* version number for KRB5_TL_ACTKVNO data */
    /* version number for KRB5_TL_MKEY_AUX data */
    #[c2rust::src_loc = "282:1"]
    pub type krb5_actkvno_node = _krb5_actkvno_node;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "288:16"]
    pub struct _krb5_mkey_aux_node {
        pub next: *mut _krb5_mkey_aux_node,
        pub mkey_kvno: krb5_kvno,
        pub latest_mkey: krb5_key_data,
    }
    #[c2rust::src_loc = "288:1"]
    pub type krb5_mkey_aux_node = _krb5_mkey_aux_node;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "294:16"]
    pub struct _krb5_keylist_node {
        pub keyblock: krb5_keyblock,
        pub kvno: krb5_kvno,
        pub next: *mut _krb5_keylist_node,
    }
    #[c2rust::src_loc = "294:1"]
    pub type krb5_keylist_node = _krb5_keylist_node;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_data,
                        krb5_magic, krb5_ui_4, krb5_flags, krb5_deltat,
                        krb5_timestamp, krb5_kvno, krb5_principal,
                        krb5_enctype, krb5_int32, krb5_keyblock, krb5_context,
                        krb5_principal_data, krb5_const_principal,
                        krb5_error_code, krb5_pointer};
    use super::k5_int_h::_krb5_context;
    extern "C" {
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
        /* kvno of mkey protecting the latest_mkey */
        /* most recent mkey */
        /*
 * Iterate over principals in the KDB.  If the callback may write to the DB,
 * the caller must get an exclusive lock with krb5_db_lock before iterating,
 * and release it with krb5_db_unlock after iterating.
 */
        #[no_mangle]
        #[c2rust::src_loc = "388:1"]
        pub fn krb5_db_iterate(kcontext: krb5_context,
                               match_entry: *mut libc::c_char,
                               func:
                                   Option<unsafe extern "C" fn(_:
                                                                   krb5_pointer,
                                                               _:
                                                                   *mut krb5_db_entry)
                                              -> libc::c_int>,
                               func_arg: krb5_pointer, iterflags: krb5_flags)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "394:1"]
        pub fn krb5_db_store_master_key(kcontext: krb5_context,
                                        keyfile: *mut libc::c_char,
                                        mname: krb5_principal,
                                        kvno: krb5_kvno,
                                        key: *mut krb5_keyblock,
                                        master_pwd: *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn krb5_dbe_encrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         dbkey: *const krb5_keyblock,
                                         keysalt: *const krb5_keysalt,
                                         keyver: libc::c_int,
                                         key_data: *mut krb5_key_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "466:1"]
        pub fn krb5_dbe_find_act_mkey(context: krb5_context,
                                      act_mkey_list: *mut krb5_actkvno_node,
                                      act_kvno: *mut krb5_kvno,
                                      act_mkey: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "483:1"]
        pub fn krb5_db_mkey_list_alias(kcontext: krb5_context)
         -> *mut krb5_keylist_node;
        /* Set *mkvno to mkvno in entry tl_data, or minimum value from mkey_list. */
        #[no_mangle]
        #[c2rust::src_loc = "487:1"]
        pub fn krb5_dbe_get_mkvno(context: krb5_context,
                                  entry: *mut krb5_db_entry,
                                  mkvno: *mut krb5_kvno) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "498:1"]
        pub fn krb5_dbe_lookup_mkey_aux(context: krb5_context,
                                        entry: *mut krb5_db_entry,
                                        mkey_aux_data_list:
                                            *mut *mut krb5_mkey_aux_node)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "502:1"]
        pub fn krb5_dbe_update_mkvno(context: krb5_context,
                                     entry: *mut krb5_db_entry,
                                     mkvno: krb5_kvno) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "507:1"]
        pub fn krb5_dbe_lookup_actkvno(context: krb5_context,
                                       entry: *mut krb5_db_entry,
                                       actkvno_list:
                                           *mut *mut krb5_actkvno_node)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "512:1"]
        pub fn krb5_dbe_update_mkey_aux(context: krb5_context,
                                        entry: *mut krb5_db_entry,
                                        mkey_aux_data_list:
                                            *mut krb5_mkey_aux_node)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "517:1"]
        pub fn krb5_dbe_update_actkvno(context: krb5_context,
                                       entry: *mut krb5_db_entry,
                                       actkvno_list: *const krb5_actkvno_node)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "542:1"]
        pub fn krb5_dbe_update_mod_princ_data(context: krb5_context,
                                              entry: *mut krb5_db_entry,
                                              mod_date: krb5_timestamp,
                                              mod_princ: krb5_const_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "654:1"]
        pub fn krb5_db_get_key_data_kvno(context: krb5_context,
                                         count: libc::c_int,
                                         data: *mut krb5_key_data)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "848:1"]
        pub fn krb5_dbe_free_key_data_contents(_: krb5_context,
                                               _: *mut krb5_key_data);
        #[no_mangle]
        #[c2rust::src_loc = "854:1"]
        pub fn krb5_dbe_free_actkvno_list(_: krb5_context,
                                          _: *mut krb5_actkvno_node);
        #[no_mangle]
        #[c2rust::src_loc = "857:1"]
        pub fn krb5_dbe_free_mkey_aux_list(_: krb5_context,
                                           _: *mut krb5_mkey_aux_node);
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:9"]
pub mod admin_h {
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
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
                        krb5_int32, krb5_kvno};
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src = "/usr/include/regex.h:28"]
pub mod regex_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "517:9"]
    pub struct regmatch_t {
        pub rm_so: regoff_t,
        pub rm_eo: regoff_t,
    }
    #[c2rust::src_loc = "490:1"]
    pub type regoff_t = libc::c_int;
    #[c2rust::src_loc = "478:1"]
    pub type regex_t = re_pattern_buffer;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "413:8"]
    pub struct re_pattern_buffer {
        pub buffer: *mut re_dfa_t,
        pub allocated: __re_long_size_t,
        pub used: __re_long_size_t,
        pub syntax: reg_syntax_t,
        pub fastmap: *mut libc::c_char,
        pub translate: *mut libc::c_uchar,
        pub re_nsub: size_t,
        #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
        #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits =
                   "1..=2")]
        #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits =
                   "3..=3")]
        #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
        #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
        #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
        #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits =
                   "7..=7")]
        pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[c2rust::src_loc = "72:1"]
    pub type reg_syntax_t = libc::c_ulong;
    #[c2rust::src_loc = "56:1"]
    pub type __re_long_size_t = libc::c_ulong;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "417:10"]
        pub type re_dfa_t;
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn regcomp(__preg: *mut regex_t, __pattern: *const libc::c_char,
                       __cflags: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "643:1"]
        pub fn regexec(__preg: *const regex_t, __String: *const libc::c_char,
                       __nmatch: size_t, __pmatch: *mut regmatch_t,
                       __eflags: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "651:1"]
        pub fn regfree(__preg: *mut regex_t);
    }
}
#[c2rust::header_src = "/usr/include/assert.h:7"]
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
#[c2rust::header_src = "/usr/include/string.h:7"]
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
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:7"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:7"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "137:14"]
        pub static mut stdin: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "347:12"]
        pub fn vprintf(_: *const libc::c_char, _: ::std::ffi::VaList)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:7"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "36:14"]
        pub static mut optarg: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "50:12"]
        pub static mut optind: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
                      __shortopts: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/time.h:7"]
pub mod time_h {
    use super::stddef_h::size_t;
    use super::struct_tm_h::tm;
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                        __format: *const libc::c_char, __tp: *const tm)
         -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "123:1"]
        pub fn localtime(__timer: *const time_t) -> *mut tm;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:7"]
pub mod k5_platform_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "887:1"]
        pub fn krb5int_strlcpy(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/libintl.h:7"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/server_internal.h:9"]
pub mod server_internal_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::krb5_context;
    use super::kdb_h::krb5_key_data;
    use super::admin_h::kadm5_ret_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn krb5_free_key_data_contents(context: krb5_context,
                                           key: *mut krb5_key_data)
         -> kadm5_ret_t;
    }
    /* __KADM5_SERVER_INTERNAL_H__ */
    /* * @}*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/dbutil/kdb5_util.h:12"]
pub mod kdb5_util_h {
    use super::krb5_h::{krb5_principal_data, krb5_principal, krb5_context,
                        krb5_enctype, krb5_deltat, krb5_timestamp, krb5_flags,
                        krb5_int32, krb5_kvno, krb5_keyblock, krb5_magic,
                        krb5_octet, krb5_error_code};
    use super::k5_int_h::_krb5_context;
    use super::admin_h::kadm5_config_params;
    use super::kdb_h::{krb5_key_salt_tuple, krb5_db_entry};
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/dbutil/kdb5_util.h */
/*
 * Copyright 1992, 2008, 2009 by the Massachusetts Institute of Technology.
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
        #[no_mangle]
        #[c2rust::src_loc = "32:14"]
        pub static mut progname: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "35:23"]
        pub static mut master_princ: krb5_principal;
        #[no_mangle]
        #[c2rust::src_loc = "38:12"]
        pub static mut exit_status: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "39:21"]
        pub static mut util_context: krb5_context;
        #[no_mangle]
        #[c2rust::src_loc = "40:28"]
        pub static mut global_params: kadm5_config_params;
        #[no_mangle]
        #[c2rust::src_loc = "45:18"]
        pub static mut new_mkvno: krb5_kvno;
        #[no_mangle]
        #[c2rust::src_loc = "46:22"]
        pub static mut new_master_keyblock: krb5_keyblock;
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn usage();
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn master_key_convert(context: krb5_context,
                                  db_entry: *mut krb5_db_entry)
         -> krb5_error_code;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __off_t, __off64_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdarg_h::va_list;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::struct_tm_h::tm;
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _profile_t, krb5_c_string_to_key,
                       krb5_unparse_name, krb5_principal_compare,
                       krb5_principal2salt, krb5_free_unparsed_name,
                       krb5_timeofday, krb5_read_password,
                       krb5_string_to_enctype, krb5_enctype_to_name};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, ts_after, ts2tt, plugin_mapping,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle};
pub use self::kdb_log_h::{_kdb_log_context, kdb_hlog_t, kdb_hlog};
pub use self::iprop_h::{kdb_sno_t, kdbe_time_t};
pub use self::iprop_hdr_h::{iprop_role, IPROP_REPLICA, IPROP_MASTER,
                            IPROP_NULL};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, com_err};
pub use self::kdb_h::{_krb5_key_data, _krb5_tl_data, krb5_tl_data,
                      krb5_key_data, _krb5_keysalt, krb5_keysalt,
                      _krb5_db_entry_new, krb5_db_entry,
                      __krb5_key_salt_tuple, krb5_key_salt_tuple,
                      _krb5_actkvno_node, krb5_actkvno_node,
                      _krb5_mkey_aux_node, krb5_mkey_aux_node,
                      _krb5_keylist_node, krb5_keylist_node,
                      krb5_db_get_principal, krb5_db_free_principal,
                      krb5_db_put_principal, krb5_db_iterate,
                      krb5_db_store_master_key, krb5_dbe_encrypt_key_data,
                      krb5_dbe_find_act_mkey, krb5_db_mkey_list_alias,
                      krb5_dbe_get_mkvno, krb5_dbe_lookup_mkey_aux,
                      krb5_dbe_update_mkvno, krb5_dbe_lookup_actkvno,
                      krb5_dbe_update_mkey_aux, krb5_dbe_update_actkvno,
                      krb5_dbe_update_mod_princ_data,
                      krb5_db_get_key_data_kvno,
                      krb5_dbe_free_key_data_contents,
                      krb5_dbe_free_actkvno_list,
                      krb5_dbe_free_mkey_aux_list};
pub use self::admin_h::{kadm5_ret_t, _kadm5_config_params,
                        kadm5_config_params};
pub use self::regex_h::{regmatch_t, regoff_t, regex_t, re_pattern_buffer,
                        reg_syntax_t, __re_long_size_t, re_dfa_t, regcomp,
                        regexec, regfree};
use self::assert_h::__assert_fail;
use self::string_h::{memset, strcmp, strchr, strlen, explicit_bzero};
use self::stdlib_h::{atoi, malloc, free};
use self::stdio_h::{stdin, stdout, fflush, printf, vprintf, asprintf, fgets};
use self::getopt_core_h::{optarg, optind, getopt};
use self::time_h::{strftime, localtime};
use self::k5_platform_h::krb5int_strlcpy;
use self::libintl_h::dgettext;
use self::server_internal_h::krb5_free_key_data_contents;
use self::kdb5_util_h::{progname, master_princ, exit_status, util_context,
                        global_params, new_mkvno, new_master_keyblock, usage,
                        master_key_convert};
extern "C" {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2009 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* current mkey */
    #[no_mangle]
    #[c2rust::src_loc = "32:20"]
    pub static mut master_kvno: krb5_kvno;
    #[no_mangle]
    #[c2rust::src_loc = "34:18"]
    pub static mut master_salt: krb5_data;
    #[no_mangle]
    #[c2rust::src_loc = "35:14"]
    pub static mut mkey_fullname: *mut libc::c_char;
    #[no_mangle]
    #[c2rust::src_loc = "36:14"]
    pub static mut mkey_password: *mut libc::c_char;
    #[no_mangle]
    #[c2rust::src_loc = "41:1"]
    pub fn get_date(_: *mut libc::c_char) -> time_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1014:8"]
pub struct kvnos_in_use {
    pub kvno: krb5_kvno,
    pub use_count: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
#[c2rust::src_loc = "641:8"]
pub struct update_enc_mkvno {
    pub re_match_count: libc::c_uint,
    pub already_current: libc::c_uint,
    pub updated: libc::c_uint,
    #[bitfield(name = "dry_run", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "verbose", ty = "libc::c_uint", bits = "1..=1")]
    pub dry_run_verbose: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub preg: regex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1019:8"]
pub struct purge_args {
    pub kcontext: krb5_context,
    pub kvnos: *mut kvnos_in_use,
    pub num_kvnos: libc::c_uint,
}
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn strdate(mut when: krb5_timestamp)
 -> *const libc::c_char {
    let mut tm: *mut tm = 0 as *mut tm;
    static mut out: [libc::c_char; 40] = [0; 40];
    let mut lcltim: time_t = ts2tt(when);
    tm = localtime(&mut lcltim);
    if tm.is_null() ||
           strftime(out.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 40]>() as
                        libc::c_ulong,
                    b"%a %b %d %H:%M:%S %Z %Y\x00" as *const u8 as
                        *const libc::c_char, tm) ==
               0 as libc::c_int as libc::c_ulong {
        krb5int_strlcpy(out.as_mut_ptr(),
                        b"(error)\x00" as *const u8 as *const libc::c_char,
                        ::std::mem::size_of::<[libc::c_char; 40]>() as
                            libc::c_ulong);
    }
    return out.as_mut_ptr();
}
#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn get_next_kvno(mut context: krb5_context,
                                       mut entry: *mut krb5_db_entry)
 -> krb5_kvno {
    let mut new_kvno: krb5_kvno = 0;
    new_kvno =
        krb5_db_get_key_data_kvno(context, (*entry).n_key_data as libc::c_int,
                                  (*entry).key_data) as krb5_kvno;
    new_kvno = new_kvno.wrapping_add(1);
    /* deal with wrapping */
    if new_kvno == 0 as libc::c_int as libc::c_uint {
        new_kvno = 1 as libc::c_int as krb5_kvno
    } /* knvo must not be 0 as this is special value (IGNORE_VNO) */
    return new_kvno;
}
#[no_mangle]
#[c2rust::src_loc = "72:1"]
pub unsafe extern "C" fn add_new_mkey(mut context: krb5_context,
                                      mut master_entry: *mut krb5_db_entry,
                                      mut new_mkey: *mut krb5_keyblock,
                                      mut use_mkvno: krb5_kvno)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut old_key_data_count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut new_mkey_kvno: krb5_kvno = 0;
    let mut tmp_key_data: krb5_key_data =
        krb5_key_data{key_data_ver: 0,
                      key_data_kvno: 0,
                      key_data_type: [0; 2],
                      key_data_length: [0; 2],
                      key_data_contents: [0 as *mut krb5_octet; 2],};
    let mut mkey_aux_data_head: *mut krb5_mkey_aux_node =
        0 as *mut krb5_mkey_aux_node;
    let mut mkey_aux_data: *mut *mut krb5_mkey_aux_node =
        0 as *mut *mut krb5_mkey_aux_node;
    let mut keylist_node: *mut krb5_keylist_node =
        0 as *mut krb5_keylist_node;
    let mut master_keylist: *mut krb5_keylist_node =
        krb5_db_mkey_list_alias(context);
    /* do this before modifying master_entry key_data */
    new_mkey_kvno = get_next_kvno(context, master_entry);
    /* verify the requested mkvno if not 0 is the one that would be used here. */
    if use_mkvno != 0 as libc::c_int as libc::c_uint &&
           new_mkey_kvno != use_mkvno {
        return -(1780008426 as libc::c_long) as krb5_error_code
    }
    old_key_data_count = (*master_entry).n_key_data as libc::c_int;
    /* alloc enough space to hold new and existing key_data */
    /*
     * The encrypted key is malloc'ed by krb5_dbe_encrypt_key_data and
     * krb5_key_data key_data_contents is a pointer to this key.  Using some
     * logic from master_key_convert().
     */
    i = 0 as libc::c_int;
    while i < (*master_entry).n_key_data as libc::c_int {
        krb5_free_key_data_contents(context,
                                    &mut *(*master_entry).key_data.offset(i as
                                                                              isize));
        i += 1
    }
    free((*master_entry).key_data as *mut libc::c_void);
    (*master_entry).key_data =
        malloc((::std::mem::size_of::<krb5_key_data>() as
                    libc::c_ulong).wrapping_mul((old_key_data_count +
                                                     1 as libc::c_int) as
                                                    libc::c_ulong)) as
            *mut krb5_key_data;
    if (*master_entry).key_data.is_null() { return 12 as libc::c_int }
    memset((*master_entry).key_data as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<krb5_key_data>() as
                libc::c_ulong).wrapping_mul((old_key_data_count +
                                                 1 as libc::c_int) as
                                                libc::c_ulong));
    (*master_entry).n_key_data =
        (old_key_data_count + 1 as libc::c_int) as krb5_int16;
    /* Note, mkey does not have salt */
    /* add new mkey encrypted with itself to mkey princ entry */
    retval =
        krb5_dbe_encrypt_key_data(context, new_mkey, new_mkey,
                                  0 as *const krb5_keysalt,
                                  new_mkey_kvno as libc::c_int,
                                  &mut *(*master_entry).key_data.offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize));
    if retval != 0 { return retval }
    /* the mvkno should be that of the newest mkey */
    retval = krb5_dbe_update_mkvno(context, master_entry, new_mkey_kvno);
    if retval != 0 {
        krb5_free_key_data_contents(context,
                                    &mut *(*master_entry).key_data.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize));
        return retval
    }
    /*
     * Need to decrypt old keys with the current mkey which is in the global
     * master_keyblock and encrypt those keys with the latest mkey.  And while
     * the old keys are being decrypted, use those to create the
     * KRB5_TL_MKEY_AUX entries which store the latest mkey encrypted by one of
     * the older mkeys.
     *
     * The new mkey is followed by existing keys.
     *
     * First, set up for creating a krb5_mkey_aux_node list which will be used
     * to update the mkey aux data for the mkey princ entry.
     */
    mkey_aux_data_head =
        malloc(::std::mem::size_of::<krb5_mkey_aux_node>() as libc::c_ulong)
            as *mut krb5_mkey_aux_node;
    if mkey_aux_data_head.is_null() {
        retval = 12 as libc::c_int
    } else {
        memset(mkey_aux_data_head as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<krb5_mkey_aux_node>() as libc::c_ulong);
        mkey_aux_data = &mut mkey_aux_data_head;
        keylist_node = master_keylist;
        i = 1 as libc::c_int;
        loop  {
            if keylist_node.is_null() {
                current_block = 17784502470059252271;
                break ;
            }
            /*
         * Create a list of krb5_mkey_aux_node nodes.  One node contains the new
         * mkey encrypted by an old mkey and the old mkey's kvno (one node per
         * old mkey).
         */
            if (*mkey_aux_data).is_null() {
                /* *mkey_aux_data points to next field of previous node */
                *mkey_aux_data =
                    malloc(::std::mem::size_of::<krb5_mkey_aux_node>() as
                               libc::c_ulong) as *mut krb5_mkey_aux_node;
                if (*mkey_aux_data).is_null() {
                    retval = 12 as libc::c_int;
                    current_block = 11061325223384880367;
                    break ;
                } else {
                    memset(*mkey_aux_data as *mut libc::c_void,
                           0 as libc::c_int,
                           ::std::mem::size_of::<krb5_mkey_aux_node>() as
                               libc::c_ulong);
                }
            }
            memset(&mut tmp_key_data as *mut krb5_key_data as
                       *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<krb5_key_data>() as libc::c_ulong);
            /* encrypt the new mkey with the older mkey */
            retval =
                krb5_dbe_encrypt_key_data(context,
                                          &mut (*keylist_node).keyblock,
                                          new_mkey, 0 as *const krb5_keysalt,
                                          new_mkey_kvno as libc::c_int,
                                          &mut tmp_key_data);
            if retval != 0 { current_block = 11061325223384880367; break ; }
            (**mkey_aux_data).latest_mkey = tmp_key_data;
            (**mkey_aux_data).mkey_kvno = (*keylist_node).kvno;
            mkey_aux_data = &mut (**mkey_aux_data).next;
            /*
         * Store old key in master_entry keydata past the new mkey
         */
            retval =
                krb5_dbe_encrypt_key_data(context, new_mkey,
                                          &mut (*keylist_node).keyblock,
                                          0 as *const krb5_keysalt,
                                          (*keylist_node).kvno as libc::c_int,
                                          &mut *(*master_entry).key_data.offset(i
                                                                                    as
                                                                                    isize));
            if retval != 0 { current_block = 11061325223384880367; break ; }
            keylist_node = (*keylist_node).next;
            i += 1
        }
        match current_block {
            11061325223384880367 => { }
            _ => {
                if i == old_key_data_count + 1 as libc::c_int {
                } else {
                    __assert_fail(b"i == old_key_data_count + 1\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"kdb5_mkey.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  182 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 88],
                                                            &[libc::c_char; 88]>(b"krb5_error_code add_new_mkey(krb5_context, krb5_db_entry *, krb5_keyblock *, krb5_kvno)\x00")).as_ptr());
                }
                retval =
                    krb5_dbe_update_mkey_aux(context, master_entry,
                                             mkey_aux_data_head);
                if !(retval != 0) {
                    (*master_entry).mask |=
                        (0x20000 as libc::c_int | 0x40000 as libc::c_int) as
                            libc::c_uint
                }
            }
        }
    }
    krb5_dbe_free_mkey_aux_list(context, mkey_aux_data_head);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "195:1"]
pub unsafe extern "C" fn kdb5_add_mkey(mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char) {
    let mut optchar: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    let mut pw_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pw_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut do_stash: libc::c_int = 0 as libc::c_int;
    let mut pwd: krb5_data =
        krb5_data{magic: 0,
                  length: 0,
                  data: 0 as *const libc::c_char as *mut libc::c_char,};
    let mut new_mkey_kvno: krb5_kvno = 0;
    let mut new_mkeyblock: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *const krb5_octet as *mut krb5_octet,};
    let mut new_master_enctype: krb5_enctype = 0x1ff as libc::c_int;
    let mut new_mkey_password: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut master_entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut now: krb5_timestamp = 0;
    /*
     * The command table entry for this command causes open_db_and_mkey() to be
     * called first to open the KDB and get the current mkey.
     */
    memset(&mut new_mkeyblock as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    master_salt.data = 0 as *mut libc::c_char;
    loop  {
        optchar =
            getopt(argc, argv as *const *mut libc::c_char,
                   b"e:s\x00" as *const u8 as *const libc::c_char);
        if !(optchar != -(1 as libc::c_int)) { break ; }
        match optchar {
            101 => {
                if krb5_string_to_enctype(optarg, &mut new_master_enctype) !=
                       0 {
                    com_err(progname, 22 as libc::c_int as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"%s is an invalid enctype\x00" as
                                         *const u8 as *const libc::c_char),
                            optarg);
                    exit_status += 1;
                    return
                }
            }
            115 => { do_stash += 1 }
            63 | _ => { usage(); return }
        }
    }
    if new_master_enctype == 0x1ff as libc::c_int {
        new_master_enctype = global_params.enctype
    }
    retval =
        krb5_db_get_principal(util_context,
                              master_princ as krb5_const_principal,
                              0 as libc::c_int as libc::c_uint,
                              &mut master_entry);
    if retval != 0 as libc::c_int {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting master key principal %s\x00" as
                             *const u8 as *const libc::c_char),
                mkey_fullname);
        exit_status += 1
    } else {
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Creating new master key for master key principal \'%s\'\n\x00"
                            as *const u8 as *const libc::c_char),
               mkey_fullname);
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"You will be prompted for a new database Master Password.\n\x00"
                            as *const u8 as *const libc::c_char));
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"It is important that you NOT FORGET this password.\n\x00"
                            as *const u8 as *const libc::c_char));
        fflush(stdout);
        pw_size = 1024 as libc::c_int as libc::c_uint;
        pw_str = malloc(pw_size as libc::c_ulong) as *mut libc::c_char;
        if pw_str.is_null() {
            com_err(progname, 12 as libc::c_int as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while creating new master key\x00" as *const u8
                                 as *const libc::c_char));
            exit_status += 1
        } else {
            retval =
                krb5_read_password(util_context,
                                   b"Enter KDC database master key\x00" as
                                       *const u8 as *const libc::c_char,
                                   b"Re-enter KDC database master key to verify\x00"
                                       as *const u8 as *const libc::c_char,
                                   pw_str, &mut pw_size);
            if retval != 0 {
                com_err(progname, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while reading new master key from keyboard\x00"
                                     as *const u8 as *const libc::c_char));
                exit_status += 1
            } else {
                new_mkey_password = pw_str;
                pwd.data = new_mkey_password;
                pwd.length = strlen(new_mkey_password) as libc::c_uint;
                retval =
                    krb5_principal2salt(util_context,
                                        master_princ as krb5_const_principal,
                                        &mut master_salt);
                if retval != 0 {
                    com_err(progname, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while calculating master key salt\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    exit_status += 1
                } else {
                    retval =
                        krb5_c_string_to_key(util_context, new_master_enctype,
                                             &mut pwd, &mut master_salt,
                                             &mut new_mkeyblock);
                    if retval != 0 {
                        com_err(progname, retval as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"while transforming master key from password\x00"
                                             as *const u8 as
                                             *const libc::c_char));
                        exit_status += 1
                    } else {
                        new_mkey_kvno =
                            get_next_kvno(util_context, master_entry);
                        retval =
                            add_new_mkey(util_context, master_entry,
                                         &mut new_mkeyblock, new_mkey_kvno);
                        if retval != 0 {
                            com_err(progname, retval as errcode_t,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"adding new master key to master principal\x00"
                                                 as *const u8 as
                                                 *const libc::c_char));
                            exit_status += 1
                        } else {
                            retval = krb5_timeofday(util_context, &mut now);
                            if retval != 0 {
                                com_err(progname, retval as errcode_t,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"while getting current time\x00"
                                                     as *const u8 as
                                                     *const libc::c_char));
                                exit_status += 1
                            } else {
                                retval =
                                    krb5_dbe_update_mod_princ_data(util_context,
                                                                   master_entry,
                                                                   now,
                                                                   master_princ
                                                                       as
                                                                       krb5_const_principal);
                                if retval != 0 {
                                    com_err(progname, retval as errcode_t,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"while updating the master key principal modification time\x00"
                                                         as *const u8 as
                                                         *const libc::c_char));
                                    exit_status += 1
                                } else {
                                    retval =
                                        krb5_db_put_principal(util_context,
                                                              master_entry);
                                    if retval != 0 {
                                        com_err(progname, retval as errcode_t,
                                                dgettext(b"mit-krb5\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"while adding master key entry to the database\x00"
                                                             as *const u8 as
                                                             *const libc::c_char));
                                        exit_status += 1
                                    } else if do_stash != 0 {
                                        retval =
                                            krb5_db_store_master_key(util_context,
                                                                     global_params.stash_file,
                                                                     master_princ,
                                                                     new_mkey_kvno,
                                                                     &mut new_mkeyblock,
                                                                     mkey_password);
                                        if retval != 0 {
                                            com_err(progname,
                                                    retval as errcode_t,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"while storing key\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char));
                                            printf(dgettext(b"mit-krb5\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            b"Warning: couldn\'t stash master key.\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    /* clean up */
    krb5_db_free_principal(util_context, master_entry);
    explicit_bzero(new_mkeyblock.contents as *mut libc::c_char as
                       *mut libc::c_void, new_mkeyblock.length as size_t);
    free(new_mkeyblock.contents as *mut libc::c_void);
    if !pw_str.is_null() {
        explicit_bzero(pw_str as *mut libc::c_void, pw_size as size_t);
        free(pw_str as *mut libc::c_void);
    }
    free(master_salt.data as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "351:1"]
pub unsafe extern "C" fn kdb5_use_mkey(mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char) {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut use_kvno: krb5_kvno = 0;
    let mut now: krb5_timestamp = 0;
    let mut start_time: krb5_timestamp = 0;
    let mut actkvno_list: *mut krb5_actkvno_node =
        0 as *mut krb5_actkvno_node;
    let mut new_actkvno: *mut krb5_actkvno_node = 0 as *mut krb5_actkvno_node;
    let mut prev_actkvno: *mut krb5_actkvno_node =
        0 as *mut krb5_actkvno_node;
    let mut cur_actkvno: *mut krb5_actkvno_node = 0 as *mut krb5_actkvno_node;
    let mut master_entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut keylist_node: *mut krb5_keylist_node =
        0 as *mut krb5_keylist_node;
    let mut inserted: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut master_keylist: *mut krb5_keylist_node =
        krb5_db_mkey_list_alias(util_context);
    if argc < 2 as libc::c_int || argc > 3 as libc::c_int {
        /* usage calls exit */
        usage();
    }
    use_kvno = atoi(*argv.offset(1 as libc::c_int as isize)) as krb5_kvno;
    if use_kvno == 0 as libc::c_int as libc::c_uint {
        com_err(progname, 22 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"0 is an invalid KVNO value\x00" as *const u8 as
                             *const libc::c_char));
        exit_status += 1;
        return
    } else {
        /* verify use_kvno is valid */
        keylist_node = master_keylist;
        while !keylist_node.is_null() {
            if use_kvno == (*keylist_node).kvno { break ; }
            keylist_node = (*keylist_node).next
        }
        if keylist_node.is_null() {
            com_err(progname, 22 as libc::c_int as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"%d is an invalid KVNO value\x00" as *const u8
                                 as *const libc::c_char), use_kvno);
            exit_status += 1;
            return
        }
    }
    retval = krb5_timeofday(util_context, &mut now);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting current time\x00" as *const u8 as
                             *const libc::c_char));
        exit_status += 1;
        return
    }
    if argc == 3 as libc::c_int {
        let mut t: time_t = get_date(*argv.offset(2 as libc::c_int as isize));
        if t == -(1 as libc::c_int) as libc::c_long {
            com_err(progname, 0 as libc::c_int as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"could not parse date-time string \'%s\'\x00" as
                                 *const u8 as *const libc::c_char),
                    *argv.offset(2 as libc::c_int as isize));
            exit_status += 1;
            return
        } else { start_time = t as krb5_timestamp }
    } else { start_time = now }
    /*
     * Need to:
     *
     * 1. get mkey princ
     * 2. get krb5_actkvno_node list
     * 3. add use_kvno to actkvno list (sorted in right spot)
     * 4. update mkey princ's tl data
     * 5. put mkey princ.
     */
    retval =
        krb5_db_get_principal(util_context,
                              master_princ as krb5_const_principal,
                              0 as libc::c_int as libc::c_uint,
                              &mut master_entry);
    if retval != 0 as libc::c_int {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting master key principal %s\x00" as
                             *const u8 as *const libc::c_char),
                mkey_fullname);
        exit_status += 1
    } else {
        retval =
            krb5_dbe_lookup_actkvno(util_context, master_entry,
                                    &mut actkvno_list);
        if retval != 0 as libc::c_int {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while looking up active version of master key\x00"
                                 as *const u8 as *const libc::c_char));
            exit_status += 1
        } else {
            /*
     * If an entry already exists with the same kvno either delete it or if it's
     * the only entry, just set its active time.
     */
            prev_actkvno = 0 as *mut krb5_actkvno_node;
            cur_actkvno = actkvno_list;
            while !cur_actkvno.is_null() {
                if (*cur_actkvno).act_kvno == use_kvno {
                    /* delete it */
                    if !prev_actkvno.is_null() {
                        (*prev_actkvno).next = (*cur_actkvno).next;
                        (*cur_actkvno).next = 0 as *mut _krb5_actkvno_node;
                        krb5_dbe_free_actkvno_list(util_context, cur_actkvno);
                    } else if !(*cur_actkvno).next.is_null() {
                        /* delete it from front of list */
                        actkvno_list = (*cur_actkvno).next;
                        (*cur_actkvno).next = 0 as *mut _krb5_actkvno_node;
                        krb5_dbe_free_actkvno_list(util_context, cur_actkvno);
                    } else {
                        /* There's only one entry, go ahead and change the time */
                        (*cur_actkvno).act_time = start_time;
                        inserted = 1 as libc::c_int as krb5_boolean
                    }
                    break ;
                } else {
                    prev_actkvno = cur_actkvno;
                    cur_actkvno = (*cur_actkvno).next
                }
            }
            if inserted == 0 {
                /* alloc enough space to hold new and existing key_data */
                new_actkvno =
                    malloc(::std::mem::size_of::<krb5_actkvno_node>() as
                               libc::c_ulong) as *mut krb5_actkvno_node;
                if new_actkvno.is_null() {
                    com_err(progname, 12 as libc::c_int as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while adding new master key\x00" as
                                         *const u8 as *const libc::c_char));
                    exit_status += 1;
                    current_block = 1404258721598844850;
                } else {
                    memset(new_actkvno as *mut libc::c_void, 0 as libc::c_int,
                           ::std::mem::size_of::<krb5_actkvno_node>() as
                               libc::c_ulong);
                    (*new_actkvno).act_kvno = use_kvno;
                    (*new_actkvno).act_time = start_time;
                    /* insert new act kvno node */
                    if actkvno_list.is_null() {
                        /* new actkvno is the list */
                        actkvno_list = new_actkvno
                    } else {
                        prev_actkvno = 0 as *mut krb5_actkvno_node;
                        cur_actkvno = actkvno_list;
                        while !cur_actkvno.is_null() {
                            if ts_after((*cur_actkvno).act_time,
                                        (*new_actkvno).act_time) != 0 {
                                if !prev_actkvno.is_null() {
                                    (*prev_actkvno).next = new_actkvno;
                                    (*new_actkvno).next = cur_actkvno
                                } else {
                                    (*new_actkvno).next = actkvno_list;
                                    actkvno_list = new_actkvno
                                }
                                break ;
                            } else if (*cur_actkvno).next.is_null() {
                                /* end of line, just add new node to end of list */
                                (*cur_actkvno).next = new_actkvno;
                                break ;
                            } else {
                                prev_actkvno = cur_actkvno;
                                cur_actkvno = (*cur_actkvno).next
                            }
                        }
                    }
                    current_block = 15237655884915618618;
                }
            } else { current_block = 15237655884915618618; }
            match current_block {
                1404258721598844850 => { }
                _ => {
                    if ts_after((*actkvno_list).act_time, now) != 0 {
                        com_err(progname, 22 as libc::c_int as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"there must be one master key currently active\x00"
                                             as *const u8 as
                                             *const libc::c_char));
                        exit_status += 1
                    } else {
                        retval =
                            krb5_dbe_update_actkvno(util_context,
                                                    master_entry,
                                                    actkvno_list);
                        if retval != 0 {
                            com_err(progname, retval as errcode_t,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"while updating actkvno data for master principal entry\x00"
                                                 as *const u8 as
                                                 *const libc::c_char));
                            exit_status += 1
                        } else {
                            retval =
                                krb5_dbe_update_mod_princ_data(util_context,
                                                               master_entry,
                                                               now,
                                                               master_princ as
                                                                   krb5_const_principal);
                            if retval != 0 {
                                com_err(progname, retval as errcode_t,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"while updating the master key principal modification time\x00"
                                                     as *const u8 as
                                                     *const libc::c_char));
                                exit_status += 1
                            } else {
                                retval =
                                    krb5_db_put_principal(util_context,
                                                          master_entry);
                                if retval != 0 {
                                    com_err(progname, retval as errcode_t,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"while adding master key entry to the database\x00"
                                                         as *const u8 as
                                                         *const libc::c_char));
                                    exit_status += 1
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    /* clean up */
    krb5_db_free_principal(util_context,
                           master_entry); /* assume actkvno entry not found */
    krb5_dbe_free_actkvno_list(util_context, actkvno_list);
}
#[no_mangle]
#[c2rust::src_loc = "542:1"]
pub unsafe extern "C" fn kdb5_list_mkeys(mut argc: libc::c_int,
                                         mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    let mut output_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut enctype: [libc::c_char; 8192] = [0; 8192];
    let mut act_kvno: krb5_kvno = 0;
    let mut act_time: krb5_timestamp = 0;
    let mut actkvno_list: *mut krb5_actkvno_node =
        0 as *mut krb5_actkvno_node;
    let mut cur_actkvno: *mut krb5_actkvno_node = 0 as *mut krb5_actkvno_node;
    let mut master_entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut cur_kb_node: *mut krb5_keylist_node = 0 as *mut krb5_keylist_node;
    let mut act_mkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut master_keylist: *mut krb5_keylist_node =
        krb5_db_mkey_list_alias(util_context);
    if master_keylist.is_null() {
        com_err(progname, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"master keylist not initialized\x00" as *const u8 as
                             *const libc::c_char));
        exit_status += 1;
        return
    }
    retval =
        krb5_db_get_principal(util_context,
                              master_princ as krb5_const_principal,
                              0 as libc::c_int as libc::c_uint,
                              &mut master_entry);
    if retval != 0 as libc::c_int {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting master key principal %s\x00" as
                             *const u8 as *const libc::c_char),
                mkey_fullname);
        exit_status += 1
    } else {
        retval =
            krb5_dbe_lookup_actkvno(util_context, master_entry,
                                    &mut actkvno_list);
        if retval != 0 as libc::c_int {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while looking up active kvno list\x00" as
                                 *const u8 as *const libc::c_char));
            exit_status += 1
        } else {
            retval =
                krb5_dbe_find_act_mkey(util_context, actkvno_list,
                                       &mut act_kvno, &mut act_mkey);
            if retval != 0 as libc::c_int {
                com_err(progname, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while looking up active master key\x00" as
                                     *const u8 as *const libc::c_char));
                exit_status += 1
            } else {
                printf(b"Master keys for Principal: %s\n\x00" as *const u8 as
                           *const libc::c_char, mkey_fullname);
                cur_kb_node = master_keylist;
                while !cur_kb_node.is_null() {
                    retval =
                        krb5_enctype_to_name((*cur_kb_node).keyblock.enctype,
                                             0 as libc::c_int as krb5_boolean,
                                             enctype.as_mut_ptr(),
                                             ::std::mem::size_of::<[libc::c_char; 8192]>()
                                                 as libc::c_ulong);
                    if retval != 0 {
                        com_err(progname, retval as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"while getting enctype description\x00"
                                             as *const u8 as
                                             *const libc::c_char));
                        exit_status += 1;
                        break ;
                    } else {
                        act_time = -(1 as libc::c_int);
                        cur_actkvno = actkvno_list;
                        while !cur_actkvno.is_null() {
                            if (*cur_actkvno).act_kvno == (*cur_kb_node).kvno
                               {
                                act_time = (*cur_actkvno).act_time;
                                break ;
                            } else { cur_actkvno = (*cur_actkvno).next }
                        }
                        if (*cur_kb_node).kvno == act_kvno {
                            /* * indicates kvno is currently active */
                            retval =
                                asprintf(&mut output_str as
                                             *mut *mut libc::c_char,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"KVNO: %d, Enctype: %s, Active on: %s *\n\x00"
                                                      as *const u8 as
                                                      *const libc::c_char),
                                         (*cur_kb_node).kvno,
                                         enctype.as_mut_ptr(),
                                         strdate(act_time))
                        } else if act_time != -(1 as libc::c_int) {
                            retval =
                                asprintf(&mut output_str as
                                             *mut *mut libc::c_char,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"KVNO: %d, Enctype: %s, Active on: %s\n\x00"
                                                      as *const u8 as
                                                      *const libc::c_char),
                                         (*cur_kb_node).kvno,
                                         enctype.as_mut_ptr(),
                                         strdate(act_time))
                        } else {
                            retval =
                                asprintf(&mut output_str as
                                             *mut *mut libc::c_char,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"KVNO: %d, Enctype: %s, No activate time set\n\x00"
                                                      as *const u8 as
                                                      *const libc::c_char),
                                         (*cur_kb_node).kvno,
                                         enctype.as_mut_ptr())
                        }
                        if retval == -(1 as libc::c_int) {
                            com_err(progname, 12 as libc::c_int as errcode_t,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"asprintf could not allocate enough memory to hold output\x00"
                                                 as *const u8 as
                                                 *const libc::c_char));
                            exit_status += 1;
                            break ;
                        } else {
                            printf(b"%s\x00" as *const u8 as
                                       *const libc::c_char, output_str);
                            free(output_str as *mut libc::c_void);
                            output_str = 0 as *mut libc::c_char;
                            cur_kb_node = (*cur_kb_node).next
                        }
                    }
                }
            }
        }
    }
    /* clean up */
    krb5_db_free_principal(util_context, master_entry);
    free(output_str as *mut libc::c_void);
    krb5_dbe_free_actkvno_list(util_context, actkvno_list);
}
/* XXX Duplicated in libkadm5srv! */
/*
 * Function: glob_to_regexp
 *
 * Arguments:
 *
 *      glob    (r) the shell-style glob (?*[]) to convert
 *      realm   (r) the default realm to append, or NULL
 *      regexp  (w) the ed-style regexp created from glob
 *
 * Effects:
 *
 * regexp is filled in with allocated memory contained a regular
 * expression to be used with re_comp/compile that matches what the
 * shell-style glob would match.  If glob does not contain an "@"
 * character and realm is not NULL, "@*" is appended to the regexp.
 *
 * Conversion algorithm:
 *
 *      quoted characters are copied quoted
 *      ? is converted to .
 *      * is converted to .*
 *      active characters are quoted: ^, $, .
 *      [ and ] are active but supported and have the same meaning, so
 *              they are copied
 *      other characters are copied
 *      regexp is anchored with ^ and $
 */
#[c2rust::src_loc = "686:1"]
unsafe extern "C" fn glob_to_regexp(mut glob: *mut libc::c_char,
                                    mut realm: *mut libc::c_char,
                                    mut regexp: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut append_realm: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    /* validate the glob */
    if *glob.offset(strlen(glob).wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong) as isize) as
           libc::c_int == '\\' as i32 {
        return 22 as libc::c_int
    }
    /* A character of glob can turn into two in regexp, plus ^ and $ */
    /* and trailing null.  If glob has no @, also allocate space for */
    /* the realm. */
    append_realm =
        (!realm.is_null() && strchr(glob, '@' as i32).is_null()) as
            libc::c_int;
    p =
        malloc(strlen(glob).wrapping_mul(2 as libc::c_int as
                                             libc::c_ulong).wrapping_add(3 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong).wrapping_add((if append_realm
                                                                                                                 !=
                                                                                                                 0
                                                                                                             {
                                                                                                              3
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                          } else {
                                                                                                              0
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                          })
                                                                                                             as
                                                                                                             libc::c_ulong))
            as *mut libc::c_char;
    if p.is_null() { return 12 as libc::c_int }
    *regexp = p;
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '^' as i32 as libc::c_char;
    while *glob != 0 {
        match *glob as libc::c_int {
            63 => {
                let fresh1 = p;
                p = p.offset(1);
                *fresh1 = '.' as i32 as libc::c_char
            }
            42 => {
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 = '.' as i32 as libc::c_char;
                let fresh3 = p;
                p = p.offset(1);
                *fresh3 = '*' as i32 as libc::c_char
            }
            46 | 94 | 36 => {
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = '\\' as i32 as libc::c_char;
                let fresh5 = p;
                p = p.offset(1);
                *fresh5 = *glob
            }
            92 => {
                let fresh6 = p;
                p = p.offset(1);
                *fresh6 = '\\' as i32 as libc::c_char;
                glob = glob.offset(1);
                let fresh7 = p;
                p = p.offset(1);
                *fresh7 = *glob
            }
            _ => { let fresh8 = p; p = p.offset(1); *fresh8 = *glob }
        }
        glob = glob.offset(1)
    }
    if append_realm != 0 {
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = '@' as i32 as libc::c_char;
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = '.' as i32 as libc::c_char;
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = '*' as i32 as libc::c_char
    }
    let fresh12 = p;
    p = p.offset(1);
    *fresh12 = '$' as i32 as libc::c_char;
    let fresh13 = p;
    p = p.offset(1);
    *fresh13 = '\u{0}' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "742:1"]
unsafe extern "C" fn update_princ_encryption_1(mut cb: *mut libc::c_void,
                                               mut ent: *mut krb5_db_entry)
 -> libc::c_int {
    let mut current_block: u64;
    let mut p: *mut update_enc_mkvno = cb as *mut update_enc_mkvno;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: krb5_error_code = 0;
    let mut match_0: libc::c_int = 0;
    let mut now: krb5_timestamp = 0;
    let mut result: libc::c_int = 0;
    let mut old_mkvno: krb5_kvno = 0;
    retval =
        krb5_unparse_name(util_context, (*ent).princ as krb5_const_principal,
                          &mut pname);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"getting string representation of principal name\x00"
                             as *const u8 as *const libc::c_char));
        current_block = 3734429906022694831;
    } else {
        if krb5_principal_compare(util_context,
                                  (*ent).princ as krb5_const_principal,
                                  master_princ as krb5_const_principal) != 0 {
            current_block = 374097460395145338;
        } else {
            match_0 =
                (regexec(&mut (*p).preg, pname, 0 as libc::c_int as size_t,
                         0 as *mut regmatch_t, 0 as libc::c_int) ==
                     0 as libc::c_int) as libc::c_int;
            if match_0 == 0 {
                current_block = 374097460395145338;
            } else {
                (*p).re_match_count = (*p).re_match_count.wrapping_add(1);
                retval =
                    krb5_dbe_get_mkvno(util_context, ent, &mut old_mkvno);
                if retval != 0 {
                    com_err(progname, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"determining master key used for principal \'%s\'\x00"
                                         as *const u8 as *const libc::c_char),
                            pname);
                    current_block = 3734429906022694831;
                } else if old_mkvno == new_mkvno {
                    if (*p).dry_run() as libc::c_int != 0 &&
                           (*p).verbose() as libc::c_int != 0 {
                        printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"would skip:   %s\n\x00" as *const u8
                                            as *const libc::c_char), pname);
                    } else if (*p).verbose() != 0 {
                        printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"skipping: %s\n\x00" as *const u8 as
                                            *const libc::c_char), pname);
                    }
                    (*p).already_current =
                        (*p).already_current.wrapping_add(1);
                    current_block = 374097460395145338;
                } else if (*p).dry_run() != 0 {
                    if (*p).verbose() != 0 {
                        printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"would update: %s\n\x00" as *const u8
                                            as *const libc::c_char), pname);
                    }
                    (*p).updated = (*p).updated.wrapping_add(1);
                    current_block = 374097460395145338;
                } else {
                    if (*p).verbose() != 0 {
                        printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"updating: %s\n\x00" as *const u8 as
                                            *const libc::c_char), pname);
                    }
                    retval = master_key_convert(util_context, ent);
                    if retval != 0 {
                        com_err(progname, retval as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"error re-encrypting key for principal \'%s\'\x00"
                                             as *const u8 as
                                             *const libc::c_char), pname);
                        current_block = 3734429906022694831;
                    } else {
                        retval = krb5_timeofday(util_context, &mut now);
                        if retval != 0 {
                            com_err(progname, retval as errcode_t,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"while getting current time\x00"
                                                 as *const u8 as
                                                 *const libc::c_char));
                            current_block = 3734429906022694831;
                        } else {
                            retval =
                                krb5_dbe_update_mod_princ_data(util_context,
                                                               ent, now,
                                                               master_princ as
                                                                   krb5_const_principal);
                            if retval != 0 {
                                com_err(progname, retval as errcode_t,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"while updating principal \'%s\' modification time\x00"
                                                     as *const u8 as
                                                     *const libc::c_char),
                                        pname);
                                current_block = 3734429906022694831;
                            } else {
                                (*ent).mask |=
                                    0x20000 as libc::c_int as libc::c_uint;
                                retval =
                                    krb5_db_put_principal(util_context, ent);
                                if retval != 0 {
                                    com_err(progname, retval as errcode_t,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"while updating principal \'%s\' key data in the database\x00"
                                                         as *const u8 as
                                                         *const libc::c_char),
                                            pname);
                                    current_block = 3734429906022694831;
                                } else {
                                    (*p).updated =
                                        (*p).updated.wrapping_add(1);
                                    current_block = 374097460395145338;
                                }
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            3734429906022694831 => { }
            _ => {
                result = 0 as libc::c_int;
                current_block = 913416764859453880;
            }
        }
    }
    match current_block {
        3734429906022694831 => { exit_status += 1; result = 1 as libc::c_int }
        _ => { }
    }
    if !pname.is_null() { krb5_free_unparsed_name(util_context, pname); }
    return result;
}
#[no_mangle]
#[c2rust::src_loc = "843:1"]
pub unsafe extern "C" fn are_you_sure(mut format: *const libc::c_char,
                                      mut args: ...) -> libc::c_int {
    let mut va: ::std::ffi::VaListImpl;
    let mut ansbuf: [libc::c_char; 100] = [0; 100];
    va = args.clone();
    vprintf(format, va.as_va_list());
    printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                    b"\n(type \'yes\' to confirm)? \x00" as *const u8 as
                        *const libc::c_char));
    fflush(stdout);
    if fgets(ansbuf.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as
                 libc::c_int, stdin).is_null() {
        return 0 as libc::c_int
    }
    if strcmp(ansbuf.as_mut_ptr(),
              b"yes\n\x00" as *const u8 as *const libc::c_char) != 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "861:1"]
pub unsafe extern "C" fn kdb5_update_princ_encryption(mut argc: libc::c_int,
                                                      mut argv:
                                                          *mut *mut libc::c_char) {
    let mut data: update_enc_mkvno =
        {
            let mut init =
                update_enc_mkvno{dry_run_verbose: [0; 1],
                                 c2rust_padding: [0; 3],
                                 re_match_count:
                                     0 as libc::c_int as libc::c_uint,
                                 already_current: 0,
                                 updated: 0,
                                 preg:
                                     regex_t{buffer: 0 as *mut re_dfa_t,
                                             allocated: 0,
                                             used: 0,
                                             syntax: 0,
                                             fastmap: 0 as *mut libc::c_char,
                                             translate:
                                                 0 as *mut libc::c_uchar,
                                             re_nsub: 0,
                                             can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor:
                                                 [0; 1],
                                             c2rust_padding: [0; 7],},};
            init.set_dry_run(0);
            init.set_verbose(0);
            init
        };
    let mut name_pattern: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut force: libc::c_int = 0 as libc::c_int;
    let mut optchar: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    let mut actkvno_list: *mut krb5_actkvno_node =
        0 as *mut krb5_actkvno_node;
    let mut master_entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut regexp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut act_mkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut master_keylist: *mut krb5_keylist_node =
        krb5_db_mkey_list_alias(util_context);
    let mut iterflags: krb5_flags = 0 as libc::c_int;
    loop  {
        optchar =
            getopt(argc, argv as *const *mut libc::c_char,
                   b"fnv\x00" as *const u8 as *const libc::c_char);
        if !(optchar != -(1 as libc::c_int)) { break ; }
        match optchar {
            102 => { force = 1 as libc::c_int }
            110 => { data.set_dry_run(1 as libc::c_int as libc::c_uint) }
            118 => { data.set_verbose(1 as libc::c_int as libc::c_uint) }
            63 | 58 | _ => { usage(); }
        }
    }
    if !(*argv.offset(optind as isize)).is_null() {
        name_pattern = *argv.offset(optind as isize);
        if !(*argv.offset((optind + 1 as libc::c_int) as isize)).is_null() {
            usage();
        }
    }
    if master_keylist.is_null() {
        com_err(progname, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"master keylist not initialized\x00" as *const u8 as
                             *const libc::c_char));
        exit_status += 1
    } else {
        /* Line up "skip" and "update" messages for viewing.  */
        /* The glob_to_regexp code only cares if the "realm" parameter is
       NULL or not; the string data is irrelevant.  */
        if name_pattern.is_null() {
            name_pattern =
                b"*\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        if glob_to_regexp(name_pattern,
                          b"hi\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, &mut regexp) !=
               0 as libc::c_int {
            com_err(progname, 12 as libc::c_int as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"converting glob pattern \'%s\' to regular expression\x00"
                                 as *const u8 as *const libc::c_char),
                    name_pattern);
            exit_status += 1
        } else if regcomp(&mut data.preg, regexp,
                          (1 as libc::c_int) << 3 as libc::c_int) !=
                      0 as libc::c_int {
            /* XXX syslog msg or regerr(regerrno) */
            com_err(progname, 0 as libc::c_int as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"error compiling converted regexp \'%s\'\x00" as
                                 *const u8 as *const libc::c_char), regexp);
            exit_status += 1
        } else {
            retval =
                krb5_db_get_principal(util_context,
                                      master_princ as krb5_const_principal,
                                      0 as libc::c_int as libc::c_uint,
                                      &mut master_entry);
            if retval != 0 as libc::c_int {
                com_err(progname, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while getting master key principal %s\x00"
                                     as *const u8 as *const libc::c_char),
                        mkey_fullname);
                exit_status += 1
            } else {
                retval =
                    krb5_dbe_lookup_actkvno(util_context, master_entry,
                                            &mut actkvno_list);
                if retval != 0 as libc::c_int {
                    com_err(progname, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while looking up active kvno list\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    exit_status += 1
                } else {
                    retval =
                        krb5_dbe_find_act_mkey(util_context, actkvno_list,
                                               &mut new_mkvno, &mut act_mkey);
                    if retval != 0 {
                        com_err(progname, retval as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"while looking up active master key\x00"
                                             as *const u8 as
                                             *const libc::c_char));
                        exit_status += 1
                    } else {
                        new_master_keyblock = *act_mkey;
                        if force == 0 && data.dry_run() == 0 &&
                               are_you_sure(dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"Re-encrypt all keys not using master key vno %u?\x00"
                                                         as *const u8 as
                                                         *const libc::c_char),
                                            new_mkvno) == 0 {
                            printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"OK, doing nothing.\n\x00" as
                                                *const u8 as
                                                *const libc::c_char));
                            exit_status += 1
                        } else {
                            if data.verbose() != 0 {
                                if data.dry_run() != 0 {
                                    printf(dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"Principals whose keys WOULD BE re-encrypted to master key vno %u:\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char),
                                           new_mkvno);
                                } else {
                                    printf(dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"Principals whose keys are being re-encrypted to master key vno %u if necessary:\n\x00"
                                                        as *const u8 as
                                                        *const libc::c_char),
                                           new_mkvno);
                                }
                            }
                            if data.dry_run() == 0 {
                                /* Grab a write lock so we don't have to upgrade to a write lock and
         * reopen the DB while iterating. */
                                iterflags = 0x1 as libc::c_int
                            }
                            retval =
                                krb5_db_iterate(util_context, name_pattern,
                                                Some(update_princ_encryption_1
                                                         as
                                                         unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void,
                                                                              _:
                                                                                  *mut krb5_db_entry)
                                                             -> libc::c_int),
                                                &mut data as
                                                    *mut update_enc_mkvno as
                                                    krb5_pointer, iterflags);
                            /* If exit_status is set, then update_princ_encryption_1 already
       printed a message.  */
                            if retval != 0 as libc::c_int &&
                                   exit_status == 0 as libc::c_int {
                                com_err(progname, retval as errcode_t,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"trying to process principal database\x00"
                                                     as *const u8 as
                                                     *const libc::c_char));
                                exit_status += 1
                            }
                            if data.dry_run() != 0 {
                                printf(dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"%u principals processed: %u would be updated, %u already current\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char),
                                       data.re_match_count, data.updated,
                                       data.already_current);
                            } else {
                                printf(dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"%u principals processed: %u updated, %u already current\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char),
                                       data.re_match_count, data.updated,
                                       data.already_current);
                            }
                        }
                    }
                }
            }
        }
    }
    krb5_db_free_principal(util_context, master_entry);
    free(regexp as *mut libc::c_void);
    regfree(&mut data.preg);
    memset(&mut new_master_keyblock as *mut krb5_keyblock as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    krb5_dbe_free_actkvno_list(util_context, actkvno_list);
}
#[c2rust::src_loc = "1025:1"]
unsafe extern "C" fn find_mkvnos_in_use(mut ptr: krb5_pointer,
                                        mut entry: *mut krb5_db_entry)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut args: *mut purge_args = 0 as *mut purge_args;
    let mut i: libc::c_uint = 0;
    let mut mkvno: krb5_kvno = 0;
    args = ptr as *mut purge_args;
    retval = krb5_dbe_get_mkvno((*args).kcontext, entry, &mut mkvno);
    if retval != 0 { return retval }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*args).num_kvnos {
        if (*(*args).kvnos.offset(i as isize)).kvno == mkvno {
            /* XXX do I need to worry about use_count wrapping? */
            let ref mut fresh14 =
                (*(*args).kvnos.offset(i as isize)).use_count;
            *fresh14 = (*fresh14).wrapping_add(1);
            break ;
        } else { i = i.wrapping_add(1) }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1050:1"]
pub unsafe extern "C" fn kdb5_purge_mkeys(mut argc: libc::c_int,
                                          mut argv: *mut *mut libc::c_char) {
    let mut current_block: u64;
    let mut optchar: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    let mut now: krb5_timestamp = 0;
    let mut master_entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut force: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut dry_run: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut verbose: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut args: purge_args =
        purge_args{kcontext: 0 as *mut _krb5_context,
                   kvnos: 0 as *mut kvnos_in_use,
                   num_kvnos: 0,};
    let mut buf: [libc::c_char; 5] = [0; 5];
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut num_kvnos_inuse: libc::c_uint = 0;
    let mut num_kvnos_purged: libc::c_uint = 0;
    let mut old_key_data_count: libc::c_uint = 0;
    let mut actkvno_list: *mut krb5_actkvno_node =
        0 as *mut krb5_actkvno_node;
    let mut actkvno_entry: *mut krb5_actkvno_node =
        0 as *mut krb5_actkvno_node;
    let mut prev_actkvno_entry: *mut krb5_actkvno_node =
        0 as *mut krb5_actkvno_node;
    let mut mkey_aux_list: *mut krb5_mkey_aux_node =
        0 as *mut krb5_mkey_aux_node;
    let mut mkey_aux_entry: *mut krb5_mkey_aux_node =
        0 as *mut krb5_mkey_aux_node;
    let mut prev_mkey_aux_entry: *mut krb5_mkey_aux_node =
        0 as *mut krb5_mkey_aux_node;
    let mut old_key_data: *mut krb5_key_data = 0 as *mut krb5_key_data;
    /*
     * Verify that the master key list has been initialized before doing
     * anything else.
     */
    if krb5_db_mkey_list_alias(util_context).is_null() {
        com_err(progname, -(1780008435 as libc::c_long),
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"master keylist not initialized\x00" as *const u8 as
                             *const libc::c_char)); /* mkey princ will not be modified */
        exit_status += 1; /* implied */
        return
    }
    memset(&mut args as *mut purge_args as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<purge_args>() as libc::c_ulong);
    optind = 1 as libc::c_int;
    loop  {
        optchar =
            getopt(argc, argv as *const *mut libc::c_char,
                   b"fnv\x00" as *const u8 as *const libc::c_char);
        if !(optchar != -(1 as libc::c_int)) { break ; }
        match optchar {
            102 => { force = 1 as libc::c_int as krb5_boolean }
            110 => {
                dry_run = 1 as libc::c_int as krb5_boolean;
                force = 1 as libc::c_int as krb5_boolean
            }
            118 => { verbose = 1 as libc::c_int as krb5_boolean }
            63 | _ => { usage(); return }
        }
    }
    retval =
        krb5_db_get_principal(util_context,
                              master_princ as krb5_const_principal,
                              0 as libc::c_int as libc::c_uint,
                              &mut master_entry);
    if retval != 0 as libc::c_int {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting master key principal %s\x00" as
                             *const u8 as *const libc::c_char),
                mkey_fullname);
        exit_status += 1
    } else {
        if force == 0 {
            printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                *const libc::c_char,
                            b"Will purge all unused master keys stored in the \'%s\' principal, are you sure?\n\x00"
                                as *const u8 as *const libc::c_char),
                   mkey_fullname);
            printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                *const libc::c_char,
                            b"(type \'yes\' to confirm)? \x00" as *const u8 as
                                *const libc::c_char));
            if fgets(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 5]>() as
                         libc::c_ulong as libc::c_int, stdin).is_null() {
                exit_status += 1;
                current_block = 18804699954452808;
            } else if strcmp(buf.as_mut_ptr(),
                             b"yes\n\x00" as *const u8 as *const libc::c_char)
                          != 0 {
                exit_status += 1;
                current_block = 18804699954452808;
            } else {
                printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                    *const libc::c_char,
                                b"OK, purging unused master keys from \'%s\'...\n\x00"
                                    as *const u8 as *const libc::c_char),
                       mkey_fullname);
                current_block = 15597372965620363352;
            }
        } else { current_block = 15597372965620363352; }
        match current_block {
            18804699954452808 => { }
            _ => {
                /* save the old keydata */
                old_key_data_count =
                    (*master_entry).n_key_data as libc::c_uint;
                if old_key_data_count == 1 as libc::c_int as libc::c_uint {
                    if verbose != 0 {
                        printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"There is only one master key which can not be purged.\n\x00"
                                            as *const u8 as
                                            *const libc::c_char));
                    }
                } else {
                    old_key_data = (*master_entry).key_data;
                    args.kvnos =
                        malloc((::std::mem::size_of::<kvnos_in_use>() as
                                    libc::c_ulong).wrapping_mul(old_key_data_count
                                                                    as
                                                                    libc::c_ulong))
                            as *mut kvnos_in_use;
                    if args.kvnos.is_null() {
                        retval = 12 as libc::c_int;
                        com_err(progname, 12 as libc::c_int as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"while allocating args.kvnos\x00" as
                                             *const u8 as
                                             *const libc::c_char));
                        exit_status += 1
                    } else {
                        memset(args.kvnos as *mut libc::c_void,
                               0 as libc::c_int,
                               (::std::mem::size_of::<kvnos_in_use>() as
                                    libc::c_ulong).wrapping_mul(old_key_data_count
                                                                    as
                                                                    libc::c_ulong));
                        args.num_kvnos = old_key_data_count;
                        args.kcontext = util_context;
                        /* populate the kvnos array with all the current mkvnos */
                        i = 0 as libc::c_int as libc::c_uint;
                        while i < old_key_data_count {
                            (*args.kvnos.offset(i as isize)).kvno =
                                (*(*master_entry).key_data.offset(i as
                                                                      isize)).key_data_kvno
                                    as krb5_kvno;
                            i = i.wrapping_add(1)
                        }
                        retval =
                            krb5_db_iterate(util_context,
                                            0 as *mut libc::c_char,
                                            Some(find_mkvnos_in_use as
                                                     unsafe extern "C" fn(_:
                                                                              krb5_pointer,
                                                                          _:
                                                                              *mut krb5_db_entry)
                                                         -> krb5_error_code),
                                            &mut args as *mut purge_args as
                                                krb5_pointer,
                                            0 as libc::c_int);
                        if retval != 0 {
                            com_err(progname, retval as errcode_t,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"while finding master keys in use\x00"
                                                 as *const u8 as
                                                 *const libc::c_char));
                            exit_status += 1
                        } else {
                            /*
     * args.kvnos has been marked with the mkvno's that are currently protecting
     * princ entries
     */
                            if dry_run != 0 {
                                printf(dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"Would purge the following master key(s) from %s:\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char),
                                       mkey_fullname);
                            } else {
                                printf(dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"Purging the following master key(s) from %s:\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char),
                                       mkey_fullname);
                            }
                            /* find # of keys still in use or print out verbose info */
                            num_kvnos_purged =
                                0 as libc::c_int as libc::c_uint;
                            num_kvnos_inuse = num_kvnos_purged;
                            i = num_kvnos_inuse;
                            loop  {
                                if !(i < args.num_kvnos) {
                                    current_block = 12961834331865314435;
                                    break ;
                                }
                                if (*args.kvnos.offset(i as isize)).use_count
                                       > 0 as libc::c_int as libc::c_uint {
                                    num_kvnos_inuse =
                                        num_kvnos_inuse.wrapping_add(1)
                                } else if (*args.kvnos.offset(i as
                                                                  isize)).kvno
                                              == master_kvno {
                                    com_err(progname,
                                            -(1780008425 as libc::c_long),
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"master key stash file needs updating, command aborting\x00"
                                                         as *const u8 as
                                                         *const libc::c_char));
                                    exit_status += 1;
                                    current_block = 18804699954452808;
                                    break ;
                                } else {
                                    num_kvnos_purged =
                                        num_kvnos_purged.wrapping_add(1);
                                    printf(dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"KVNO: %d\n\x00" as
                                                        *const u8 as
                                                        *const libc::c_char),
                                           (*args.kvnos.offset(i as
                                                                   isize)).kvno);
                                }
                                i = i.wrapping_add(1)
                            }
                            match current_block {
                                18804699954452808 => { }
                                _ =>
                                /* this key would be deleted */
                                /* didn't find any keys to purge */
                                {
                                    if num_kvnos_inuse == args.num_kvnos {
                                        printf(dgettext(b"mit-krb5\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        b"All keys in use, nothing purged.\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char));
                                    } else if dry_run != 0 {
                                        /* bail before doing anything else */
                                        printf(dgettext(b"mit-krb5\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        b"%d key(s) would be purged.\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char),
                                               num_kvnos_purged); /* there's only 1 mkey per kvno */
                                    } else {
                                        retval =
                                            krb5_dbe_lookup_actkvno(util_context,
                                                                    master_entry,
                                                                    &mut actkvno_list);
                                        if retval != 0 as libc::c_int {
                                            com_err(progname,
                                                    retval as errcode_t,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"while looking up active kvno list\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char));
                                            exit_status += 1
                                        } else {
                                            retval =
                                                krb5_dbe_lookup_mkey_aux(util_context,
                                                                         master_entry,
                                                                         &mut mkey_aux_list);
                                            if retval != 0 as libc::c_int {
                                                com_err(progname,
                                                        retval as errcode_t,
                                                        dgettext(b"mit-krb5\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"while looking up mkey aux data list\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char));
                                                exit_status += 1
                                            } else {
                                                (*master_entry).key_data =
                                                    malloc((::std::mem::size_of::<krb5_key_data>()
                                                                as
                                                                libc::c_ulong).wrapping_mul(num_kvnos_inuse
                                                                                                as
                                                                                                libc::c_ulong))
                                                        as *mut krb5_key_data;
                                                if (*master_entry).key_data.is_null()
                                                   {
                                                    retval =
                                                        12 as libc::c_int;
                                                    com_err(progname,
                                                            12 as libc::c_int
                                                                as errcode_t,
                                                            dgettext(b"mit-krb5\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"while allocating key_data\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char));
                                                    exit_status += 1
                                                } else {
                                                    memset((*master_entry).key_data
                                                               as
                                                               *mut libc::c_void,
                                                           0 as libc::c_int,
                                                           (::std::mem::size_of::<krb5_key_data>()
                                                                as
                                                                libc::c_ulong).wrapping_mul(num_kvnos_inuse
                                                                                                as
                                                                                                libc::c_ulong));
                                                    (*master_entry).n_key_data
                                                        =
                                                        num_kvnos_inuse as
                                                            krb5_int16;
                                                    /*
     * Assuming that the latest mkey will not be purged because it will always
     * be "in use" so this code will not bother with encrypting keys again.
     */
                                                    k =
                                                        0 as libc::c_int as
                                                            libc::c_uint;
                                                    i = k;
                                                    while i <
                                                              old_key_data_count
                                                          {
                                                        j =
                                                            0 as libc::c_int
                                                                as
                                                                libc::c_uint;
                                                        while j <
                                                                  args.num_kvnos
                                                              {
                                                            if (*args.kvnos.offset(j
                                                                                       as
                                                                                       isize)).kvno
                                                                   ==
                                                                   (*old_key_data.offset(i
                                                                                             as
                                                                                             isize)).key_data_kvno
                                                                       as
                                                                       krb5_kvno
                                                               {
                                                                if (*args.kvnos.offset(j
                                                                                           as
                                                                                           isize)).use_count
                                                                       !=
                                                                       0 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint
                                                                   {
                                                                    let fresh15 =
                                                                        k;
                                                                    k =
                                                                        k.wrapping_add(1);
                                                                    *(*master_entry).key_data.offset(fresh15
                                                                                                         as
                                                                                                         isize)
                                                                        =
                                                                        *old_key_data.offset(i
                                                                                                 as
                                                                                                 isize);
                                                                    memset(&mut *old_key_data.offset(i
                                                                                                         as
                                                                                                         isize)
                                                                               as
                                                                               *mut krb5_key_data
                                                                               as
                                                                               *mut libc::c_void,
                                                                           0
                                                                               as
                                                                               libc::c_int,
                                                                           ::std::mem::size_of::<krb5_key_data>()
                                                                               as
                                                                               libc::c_ulong);
                                                                    break ;
                                                                } else {
                                                                    /* remove unused mkey */
                    /* adjust the actkno data */
                                                                    actkvno_entry
                                                                        =
                                                                        actkvno_list;
                                                                    prev_actkvno_entry
                                                                        =
                                                                        actkvno_entry;
                                                                    while !actkvno_entry.is_null()
                                                                          {
                                                                        if (*actkvno_entry).act_kvno
                                                                               ==
                                                                               (*args.kvnos.offset(j
                                                                                                       as
                                                                                                       isize)).kvno
                                                                           {
                                                                            if actkvno_entry
                                                                                   ==
                                                                                   actkvno_list
                                                                               {
                                                                                /* remove from head */
                                                                                actkvno_list
                                                                                    =
                                                                                    (*actkvno_entry).next
                                                                            } else if (*actkvno_entry).next.is_null()
                                                                             {
                                                                                /* remove from tail */
                                                                                (*prev_actkvno_entry).next
                                                                                    =
                                                                                    0
                                                                                        as
                                                                                        *mut _krb5_actkvno_node
                                                                            } else {
                                                                                /* remove in between */
                                                                                (*prev_actkvno_entry).next
                                                                                    =
                                                                                    (*actkvno_entry).next
                                                                            }
                                                                            (*actkvno_entry).next
                                                                                =
                                                                                0
                                                                                    as
                                                                                    *mut _krb5_actkvno_node;
                                                                            krb5_dbe_free_actkvno_list(util_context,
                                                                                                       actkvno_entry);
                                                                            break
                                                                                ;
                                                                            /* deleted entry, no need to loop further */
                                                                        } else {
                                                                            prev_actkvno_entry
                                                                                =
                                                                                actkvno_entry;
                                                                            actkvno_entry
                                                                                =
                                                                                (*actkvno_entry).next
                                                                        }
                                                                    }
                                                                    /* adjust the mkey aux data */
                                                                    mkey_aux_entry
                                                                        =
                                                                        mkey_aux_list;
                                                                    prev_mkey_aux_entry
                                                                        =
                                                                        mkey_aux_entry;
                                                                    while !mkey_aux_entry.is_null()
                                                                          {
                                                                        if (*mkey_aux_entry).mkey_kvno
                                                                               ==
                                                                               (*args.kvnos.offset(j
                                                                                                       as
                                                                                                       isize)).kvno
                                                                           {
                                                                            if mkey_aux_entry
                                                                                   ==
                                                                                   mkey_aux_list
                                                                               {
                                                                                mkey_aux_list
                                                                                    =
                                                                                    (*mkey_aux_entry).next
                                                                            } else if (*mkey_aux_entry).next.is_null()
                                                                             {
                                                                                (*prev_mkey_aux_entry).next
                                                                                    =
                                                                                    0
                                                                                        as
                                                                                        *mut _krb5_mkey_aux_node
                                                                            } else {
                                                                                (*prev_mkey_aux_entry).next
                                                                                    =
                                                                                    (*mkey_aux_entry).next
                                                                            }
                                                                            (*mkey_aux_entry).next
                                                                                =
                                                                                0
                                                                                    as
                                                                                    *mut _krb5_mkey_aux_node;
                                                                            krb5_dbe_free_mkey_aux_list(util_context,
                                                                                                        mkey_aux_entry);
                                                                            break
                                                                                ;
                                                                            /* deleted entry, no need to loop further */
                                                                        } else {
                                                                            prev_mkey_aux_entry
                                                                                =
                                                                                mkey_aux_entry;
                                                                            mkey_aux_entry
                                                                                =
                                                                                (*mkey_aux_entry).next
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            j =
                                                                j.wrapping_add(1)
                                                        }
                                                        i = i.wrapping_add(1)
                                                    }
                                                    if k == num_kvnos_inuse {
                                                    } else {
                                                        __assert_fail(b"k == num_kvnos_inuse\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      b"kdb5_mkey.c\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      1281 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint,
                                                                      (*::std::mem::transmute::<&[u8; 36],
                                                                                                &[libc::c_char; 36]>(b"void kdb5_purge_mkeys(int, char **)\x00")).as_ptr());
                                                    }
                                                    /* Free any key data entries we did not consume in the loop above. */
                                                    i =
                                                        0 as libc::c_int as
                                                            libc::c_uint;
                                                    while i <
                                                              old_key_data_count
                                                          {
                                                        krb5_dbe_free_key_data_contents(util_context,
                                                                                        &mut *old_key_data.offset(i
                                                                                                                      as
                                                                                                                      isize));
                                                        i = i.wrapping_add(1)
                                                    }
                                                    free(old_key_data as
                                                             *mut libc::c_void);
                                                    retval =
                                                        krb5_dbe_update_actkvno(util_context,
                                                                                master_entry,
                                                                                actkvno_list);
                                                    if retval != 0 {
                                                        com_err(progname,
                                                                retval as
                                                                    errcode_t,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"while updating actkvno data for master principal entry\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char));
                                                        exit_status += 1
                                                    } else {
                                                        retval =
                                                            krb5_dbe_update_mkey_aux(util_context,
                                                                                     master_entry,
                                                                                     mkey_aux_list);
                                                        if retval != 0 {
                                                            com_err(progname,
                                                                    retval as
                                                                        errcode_t,
                                                                    dgettext(b"mit-krb5\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char,
                                                                             b"while updating mkey_aux data for master principal entry\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char));
                                                            exit_status += 1
                                                        } else {
                                                            retval =
                                                                krb5_timeofday(util_context,
                                                                               &mut now);
                                                            if retval != 0 {
                                                                com_err(progname,
                                                                        retval
                                                                            as
                                                                            errcode_t,
                                                                        dgettext(b"mit-krb5\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"while getting current time\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
                                                                exit_status +=
                                                                    1
                                                            } else {
                                                                retval =
                                                                    krb5_dbe_update_mod_princ_data(util_context,
                                                                                                   master_entry,
                                                                                                   now,
                                                                                                   master_princ
                                                                                                       as
                                                                                                       krb5_const_principal);
                                                                if retval != 0
                                                                   {
                                                                    com_err(progname,
                                                                            retval
                                                                                as
                                                                                errcode_t,
                                                                            dgettext(b"mit-krb5\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"while updating the master key principal modification time\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
                                                                    exit_status
                                                                        += 1
                                                                } else {
                                                                    (*master_entry).mask
                                                                        |=
                                                                        (0x20000
                                                                             as
                                                                             libc::c_int
                                                                             |
                                                                             0x40000
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            libc::c_uint;
                                                                    retval =
                                                                        krb5_db_put_principal(util_context,
                                                                                              master_entry);
                                                                    if retval
                                                                           !=
                                                                           0 {
                                                                        com_err(progname,
                                                                                retval
                                                                                    as
                                                                                    errcode_t,
                                                                                dgettext(b"mit-krb5\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"while adding master key entry to the database\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
                                                                        exit_status
                                                                            +=
                                                                            1
                                                                    } else {
                                                                        printf(dgettext(b"mit-krb5\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char,
                                                                                        b"%d key(s) purged.\n\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char),
                                                                               num_kvnos_purged);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    krb5_db_free_principal(util_context, master_entry);
    free(args.kvnos as *mut libc::c_void);
    krb5_dbe_free_actkvno_list(util_context, actkvno_list);
    krb5_dbe_free_mkey_aux_list(util_context, mkey_aux_list);
}
