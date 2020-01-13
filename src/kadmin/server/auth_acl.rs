use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
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
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:27"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
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
    use super::stdint_uintn_h::{uint8_t, uint16_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "6310:1"]
        pub fn krb5_string_to_deltat(string: *mut libc::c_char,
                                     deltatp: *mut krb5_deltat)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
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
    #[c2rust::src_loc = "2237:1"]
    pub unsafe extern "C" fn data_eq(mut d1: krb5_data, mut d2: krb5_data)
     -> libc::c_int {
        return (d1.length == d2.length &&
                    (d1.length == 0 as libc::c_int as libc::c_uint ||
                         memcmp(d1.data as *const libc::c_void,
                                d2.data as *const libc::c_void,
                                d1.length as libc::c_ulong) == 0)) as
                   libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "2244:1"]
    pub unsafe extern "C" fn data_eq_string(mut d: krb5_data,
                                            mut s: *const libc::c_char)
     -> libc::c_int {
        return (d.length as libc::c_ulong == strlen(s) &&
                    (d.length == 0 as libc::c_int as libc::c_uint ||
                         memcmp(d.data as *const libc::c_void,
                                s as *const libc::c_void,
                                d.length as libc::c_ulong) == 0)) as
                   libc::c_int;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::string_h::{memcmp, strlen};
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1202:8"]
        pub type _kdb_log_context;
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
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:27"]
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
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:27"]
pub mod plugin_h {
    #[c2rust::src_loc = "34:1"]
    pub type krb5_plugin_vtable = *mut krb5_plugin_vtable_st;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
        /* Generic declarations for dynamic modules implementing krb5 plugin
 * modules. */
        /* krb5_plugin_vtable is an abstract type.  Module initvt functions will cast
 * it to the appropriate interface-specific vtable type. */
        #[c2rust::src_loc = "34:16"]
        pub type krb5_plugin_vtable_st;
    }
    /* KRB5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:27"]
pub mod k5_buf_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-buf.h - k5buf interface declarations */
/*
 * Copyright 2008 Massachusetts Institute of Technology.
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
 * The k5buf module is intended to allow multi-step string construction in a
 * fixed or dynamic buffer without the need to check for a failure at each step
 * (and without aborting on malloc failure).  If an allocation failure occurs
 * or the fixed buffer runs out of room, the buffer will be set to an error
 * state which can be detected with k5_buf_status.  Data in a buffer is
 * terminated with a zero byte so that it can be used as a C string.
 *
 * k5buf structures are usually stack-allocated.  Do not put k5buf structure
 * pointers into public APIs.  It is okay to reference the data and len fields
 * of a buffer (they will be NULL/0 if the buffer is in an error state), but do
 * not change them.
 */
    /* Buffer type values */
    #[c2rust::src_loc = "48:1"]
    pub type k5buftype = libc::c_uint;
    #[c2rust::src_loc = "48:59"]
    pub const K5BUF_DYNAMIC_ZAP: k5buftype = 3;
    #[c2rust::src_loc = "48:44"]
    pub const K5BUF_DYNAMIC: k5buftype = 2;
    #[c2rust::src_loc = "48:31"]
    pub const K5BUF_FIXED: k5buftype = 1;
    #[c2rust::src_loc = "48:18"]
    pub const K5BUF_ERROR: k5buftype = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct k5buf {
        pub buftype: k5buftype,
        pub data: *mut libc::c_void,
        pub space: size_t,
        pub len: size_t,
    }
    use super::stddef_h::size_t;
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
        /* Extend the length of buf by len and return a pointer to the reserved space,
 * to be filled in by the caller.  Return NULL on error. */
        #[no_mangle]
        #[c2rust::src_loc = "93:1"]
        pub fn k5_buf_get_space(buf: *mut k5buf, len: size_t)
         -> *mut libc::c_void;
        /* Truncate BUF.  LEN must be between 0 and the existing buffer
 * length, or an assertion failure will result. */
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn k5_buf_truncate(buf: *mut k5buf, len: size_t);
        /*
 * Free the storage used in the dynamic buffer BUF.  The caller may choose to
 * take responsibility for freeing the data pointer instead of using this
 * function.  If BUF is a fixed buffer, an assertion failure will result.
 * Freeing a buffer in the error state, a buffer initialized with EMPTY_K5BUF,
 * or a zeroed k5buf structure is a no-op.
 */
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn k5_buf_free(buf: *mut k5buf);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:29"]
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
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:29"]
pub mod admin_h {
    /*
 * A module can optionally include <kadm5/admin.h> to inspect principal or
 * policy records from requests that add or modify principals or policies.
 * Note that fields of principal and policy structures are only valid if the
 * corresponding bit is set in the accompanying mask parameter.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "190:16"]
    pub struct _kadm5_principal_ent_t {
        pub principal: krb5_principal,
        pub princ_expire_time: krb5_timestamp,
        pub last_pwd_change: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub max_life: krb5_deltat,
        pub mod_name: krb5_principal,
        pub mod_date: krb5_timestamp,
        pub attributes: krb5_flags,
        pub kvno: krb5_kvno,
        pub mkvno: krb5_kvno,
        pub policy: *mut libc::c_char,
        pub aux_attributes: libc::c_long,
        pub max_renewable_life: krb5_deltat,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_key_data: krb5_int16,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _kadm5_policy_ent_t {
        pub policy: *mut libc::c_char,
        pub pw_min_life: libc::c_long,
        pub pw_max_life: libc::c_long,
        pub pw_min_length: libc::c_long,
        pub pw_min_classes: libc::c_long,
        pub pw_history_num: libc::c_long,
        pub policy_refcnt: libc::c_long,
        pub pw_max_fail: krb5_kvno,
        pub pw_failcnt_interval: krb5_deltat,
        pub pw_lockout_duration: krb5_deltat,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    use super::krb5_h::{krb5_principal, krb5_timestamp, krb5_deltat,
                        krb5_flags, krb5_kvno, krb5_int16};
    use super::kdb_h::{krb5_tl_data, krb5_key_data};
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/kadm5_auth_plugin.h:30"]
pub mod kadm5_auth_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2017 by the Massachusetts Institute of Technology.
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
    /*
 * Declarations for kadm5_auth plugin module implementors.
 *
 * The kadm5_auth pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * kadm5_auth plugin modules should define a function named
 * kadm5_auth_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   kadm5_auth_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                             krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_kadm5_auth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* An abstract type for kadm5_auth module data. */
    #[c2rust::src_loc = "67:1"]
    pub type kadm5_auth_moddata = *mut kadm5_auth_moddata_st;
    /*
 * A module can optionally generate restrictions when checking permissions for
 * adding or modifying a principal entry.  Restriction fields will only be
 * honored if the corresponding mask bit is set.  The operable mask bits are
 * defined in <kadmin/admin.h> and are:
 *
 * - KADM5_ATTRIBUTES for require_attrs, forbid_attrs
 * - KADM5_POLICY for policy
 * - KADM5_POLICY_CLR to require that policy be unset
 * - KADM5_PRINC_EXPIRE_TIME for princ_lifetime
 * - KADM5_PW_EXPIRATION for pw_lifetime
 * - KADM5_MAX_LIFE for max_life
 * - KADM5_MAX_RLIFE for max_renewable_life
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:8"]
    pub struct kadm5_auth_restrictions {
        pub mask: libc::c_long,
        pub require_attrs: krb5_flags,
        pub forbid_attrs: krb5_flags,
        pub princ_lifetime: krb5_deltat,
        pub pw_lifetime: krb5_deltat,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub policy: *mut libc::c_char,
    }
    /* ** Method type declarations ***/
    /*
 * Optional: Initialize module data.  acl_file is the realm's configured ACL
 * file, or NULL if none was configured.  Return 0 on success,
 * KRB5_PLUGIN_NO_HANDLE if the module is inoperable (due to configuration, for
 * example), and any other error code to abort kadmind startup.  Optionally set
 * *data_out to a module data object to be passed to future calls.
 */
    #[c2rust::src_loc = "112:1"]
    pub type kadm5_auth_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *const libc::c_char,
                                    _: *mut kadm5_auth_moddata)
                   -> krb5_error_code>;
    /* Optional: Release resources used by module data. */
    #[c2rust::src_loc = "117:1"]
    pub type kadm5_auth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata)
                   -> ()>;
    /*
 * Each check method below should return 0 to explicitly authorize the request,
 * KRB5_PLUGIN_NO_HANDLE to neither authorize nor deny the request, and any
 * other error code (such as EPERM) to explicitly deny the request.  If a check
 * method is not defined, the module will neither authorize nor deny the
 * request.  A request succeeds if at least one kadm5_auth module explicitly
 * authorizes the request and none of the modules explicitly deny it.
 */
    /* Optional: authorize an add-principal operation, and optionally generate
 * restrictions. */
    #[c2rust::src_loc = "131:1"]
    pub type kadm5_auth_addprinc_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal,
                                    _: *const _kadm5_principal_ent_t,
                                    _: libc::c_long,
                                    _: *mut *mut kadm5_auth_restrictions)
                   -> krb5_error_code>;
    /* Optional: authorize a modify-principal operation, and optionally generate
 * restrictions. */
    #[c2rust::src_loc = "140:1"]
    pub type kadm5_auth_modprinc_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal,
                                    _: *const _kadm5_principal_ent_t,
                                    _: libc::c_long,
                                    _: *mut *mut kadm5_auth_restrictions)
                   -> krb5_error_code>;
    /* Optional: authorize a set-string operation. */
    #[c2rust::src_loc = "148:1"]
    pub type kadm5_auth_setstr_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char)
                   -> krb5_error_code>;
    /* Optional: authorize a change-password operation. */
    #[c2rust::src_loc = "155:1"]
    pub type kadm5_auth_cpw_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a randomize-keys operation. */
    #[c2rust::src_loc = "160:1"]
    pub type kadm5_auth_chrand_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a set-key operation. */
    #[c2rust::src_loc = "166:1"]
    pub type kadm5_auth_setkey_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a purgekeys operation. */
    #[c2rust::src_loc = "172:1"]
    pub type kadm5_auth_purgekeys_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a delete-principal operation. */
    #[c2rust::src_loc = "178:1"]
    pub type kadm5_auth_delprinc_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a rename-principal operation. */
    #[c2rust::src_loc = "184:1"]
    pub type kadm5_auth_renprinc_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a get-principal operation. */
    #[c2rust::src_loc = "191:1"]
    pub type kadm5_auth_getprinc_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a get-strings operation. */
    #[c2rust::src_loc = "197:1"]
    pub type kadm5_auth_getstrs_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize an extract-keys operation. */
    #[c2rust::src_loc = "203:1"]
    pub type kadm5_auth_extract_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a list-principals operation. */
    #[c2rust::src_loc = "209:1"]
    pub type kadm5_auth_listprincs_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize an add-policy operation. */
    #[c2rust::src_loc = "214:1"]
    pub type kadm5_auth_addpol_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: *const libc::c_char,
                                    _: *const _kadm5_policy_ent_t,
                                    _: libc::c_long) -> krb5_error_code>;
    /* Optional: authorize a modify-policy operation. */
    #[c2rust::src_loc = "220:1"]
    pub type kadm5_auth_modpol_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: *const libc::c_char,
                                    _: *const _kadm5_policy_ent_t,
                                    _: libc::c_long) -> krb5_error_code>;
    /* Optional: authorize a delete-policy operation. */
    #[c2rust::src_loc = "226:1"]
    pub type kadm5_auth_delpol_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: *const libc::c_char)
                   -> krb5_error_code>;
    /* Optional: authorize a get-policy operation.  client_policy is the client
 * principal's policy name, or NULL if it does not have one. */
    #[c2rust::src_loc = "232:1"]
    pub type kadm5_auth_getpol_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char)
                   -> krb5_error_code>;
    /* Optional: authorize a list-policies operation. */
    #[c2rust::src_loc = "238:1"]
    pub type kadm5_auth_listpols_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize an iprop operation. */
    #[c2rust::src_loc = "243:1"]
    pub type kadm5_auth_iprop_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /*
 * Optional: receive a notification that the most recent authorized operation
 * has ended.  If a kadm5_auth module is also a KDB module, it can assume that
 * all KDB methods invoked between a kadm5_auth authorization method invocation
 * and a kadm5_auth end invocation are performed as part of the authorized
 * operation.
 *
 * The end method may be invoked without a preceding authorization method in
 * some cases; the module must be prepared to ignore such calls.
 */
    #[c2rust::src_loc = "257:1"]
    pub type kadm5_auth_end_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata)
                   -> ()>;
    /*
 * Optional: free a restrictions object.  This method does not need to be
 * defined if the module does not generate restrictions objects, or if it
 * returns aliases to restrictions objects contained from within the module
 * data.
 */
    #[c2rust::src_loc = "266:1"]
    pub type kadm5_auth_free_restrictions_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: *mut kadm5_auth_restrictions) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "272:16"]
    pub struct kadm5_auth_vtable_st {
        pub name: *const libc::c_char,
        pub init: kadm5_auth_init_fn,
        pub fini: kadm5_auth_fini_fn,
        pub addprinc: kadm5_auth_addprinc_fn,
        pub modprinc: kadm5_auth_modprinc_fn,
        pub setstr: kadm5_auth_setstr_fn,
        pub cpw: kadm5_auth_cpw_fn,
        pub chrand: kadm5_auth_chrand_fn,
        pub setkey: kadm5_auth_setkey_fn,
        pub purgekeys: kadm5_auth_purgekeys_fn,
        pub delprinc: kadm5_auth_delprinc_fn,
        pub renprinc: kadm5_auth_renprinc_fn,
        pub getprinc: kadm5_auth_getprinc_fn,
        pub getstrs: kadm5_auth_getstrs_fn,
        pub extract: kadm5_auth_extract_fn,
        pub listprincs: kadm5_auth_listprincs_fn,
        pub addpol: kadm5_auth_addpol_fn,
        pub modpol: kadm5_auth_modpol_fn,
        pub delpol: kadm5_auth_delpol_fn,
        pub getpol: kadm5_auth_getpol_fn,
        pub listpols: kadm5_auth_listpols_fn,
        pub iprop: kadm5_auth_iprop_fn,
        pub end: kadm5_auth_end_fn,
        pub free_restrictions: kadm5_auth_free_restrictions_fn,
    }
    /* kadm5_auth vtable for major version 1. */
    #[c2rust::src_loc = "272:1"]
    pub type kadm5_auth_vtable = *mut kadm5_auth_vtable_st;
    use super::krb5_h::{krb5_flags, krb5_deltat, krb5_error_code,
                        krb5_context, krb5_const_principal};
    use super::admin_h::{_kadm5_principal_ent_t, _kadm5_policy_ent_t};
    extern "C" {
        #[c2rust::src_loc = "67:16"]
        pub type kadm5_auth_moddata_st;
    }
    /* Mandatory: name of module. */
    /* Minor version 1 ends here. */
    /* KRB5_KADM5_AUTH_PLUGIN_H */
}
#[c2rust::header_src = "/usr/include/ctype.h:32"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[no_mangle]
        #[c2rust::src_loc = "122:12"]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "272:15"]
        pub fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "276:15"]
        pub fn strspn(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "345:1"]
        pub fn strtok_r(__s: *mut libc::c_char, __delim: *const libc::c_char,
                        __save_ptr: *mut *mut libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "786:1"]
        pub fn fileno(__stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:27"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:27"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:27"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:31"]
pub mod adm_proto_h {
    use super::krb5_h::{krb5_flags, krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn krb5_klog_syslog(_: libc::c_int, _: *const libc::c_char,
                                _: ...) -> libc::c_int;
        /* str_conv.c */
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn krb5_flagspec_to_mask(_: *const libc::c_char,
                                     _: *mut krb5_flags, _: *mut krb5_flags)
         -> krb5_error_code;
    }
    /* KRB5_ADM_PROTO_H__ */
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __off_t, __off64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_boolean, krb5_kvno, krb5_enctype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, krb5_const_principal,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _profile_t, krb5_parse_name,
                       krb5_free_principal, krb5_string_to_deltat,
                       krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, data_eq, data_eq_string,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, error_message};
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_init_dynamic,
                         k5_buf_get_space, k5_buf_truncate, k5_buf_free};
pub use self::kdb_h::{_krb5_key_data, _krb5_tl_data, krb5_tl_data,
                      krb5_key_data};
pub use self::admin_h::{_kadm5_principal_ent_t, _kadm5_policy_ent_t};
pub use self::kadm5_auth_plugin_h::{kadm5_auth_moddata,
                                    kadm5_auth_restrictions,
                                    kadm5_auth_init_fn, kadm5_auth_fini_fn,
                                    kadm5_auth_addprinc_fn,
                                    kadm5_auth_modprinc_fn,
                                    kadm5_auth_setstr_fn, kadm5_auth_cpw_fn,
                                    kadm5_auth_chrand_fn,
                                    kadm5_auth_setkey_fn,
                                    kadm5_auth_purgekeys_fn,
                                    kadm5_auth_delprinc_fn,
                                    kadm5_auth_renprinc_fn,
                                    kadm5_auth_getprinc_fn,
                                    kadm5_auth_getstrs_fn,
                                    kadm5_auth_extract_fn,
                                    kadm5_auth_listprincs_fn,
                                    kadm5_auth_addpol_fn,
                                    kadm5_auth_modpol_fn,
                                    kadm5_auth_delpol_fn,
                                    kadm5_auth_getpol_fn,
                                    kadm5_auth_listpols_fn,
                                    kadm5_auth_iprop_fn, kadm5_auth_end_fn,
                                    kadm5_auth_free_restrictions_fn,
                                    kadm5_auth_vtable_st, kadm5_auth_vtable,
                                    kadm5_auth_moddata_st};
pub use self::ctype_h::{C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl, _ISblank,
                        _ISgraph, _ISprint, _ISspace, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc, tolower};
use self::string_h::{memset, memcmp, strcmp, strdup, strcspn, strspn,
                     strtok_r, strlen};
use self::stdlib_h::{malloc, calloc, free};
use self::stdio_h::{fclose, fopen, fgets, fileno};
use self::fcntl_h::fcntl;
use self::errno_h::__errno_location;
use self::libintl_h::dgettext;
use self::adm_proto_h::{krb5_klog_syslog, krb5_flagspec_to_mask};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "63:8"]
pub struct acl_entry {
    pub next: *mut acl_entry,
    pub client: krb5_principal,
    pub op_allowed: uint32_t,
    pub target: krb5_principal,
    pub rs: *mut kadm5_auth_restrictions,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "86:8"]
pub struct wildstate {
    pub nwild: libc::c_int,
    pub backref: [*const krb5_data; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "91:8"]
pub struct acl_state {
    pub list: *mut acl_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "58:8"]
pub struct acl_op_table {
    pub op: libc::c_char,
    pub mask: uint32_t,
}
#[c2rust::src_loc = "71:34"]
static mut acl_op_table: [acl_op_table; 12] =
    [{
         let mut init =
             acl_op_table{op: 'a' as i32 as libc::c_char,
                          mask: 1 as libc::c_int as uint32_t,};
         init
     },
     {
         let mut init =
             acl_op_table{op: 'd' as i32 as libc::c_char,
                          mask: 2 as libc::c_int as uint32_t,};
         init
     },
     {
         let mut init =
             acl_op_table{op: 'm' as i32 as libc::c_char,
                          mask: 4 as libc::c_int as uint32_t,};
         init
     },
     {
         let mut init =
             acl_op_table{op: 'c' as i32 as libc::c_char,
                          mask: 8 as libc::c_int as uint32_t,};
         init
     },
     {
         let mut init =
             acl_op_table{op: 'i' as i32 as libc::c_char,
                          mask: 32 as libc::c_int as uint32_t,};
         init
     },
     {
         let mut init =
             acl_op_table{op: 'l' as i32 as libc::c_char,
                          mask: 128 as libc::c_int as uint32_t,};
         init
     },
     {
         let mut init =
             acl_op_table{op: 'p' as i32 as libc::c_char,
                          mask: 512 as libc::c_int as uint32_t,};
         init
     },
     {
         let mut init =
             acl_op_table{op: 's' as i32 as libc::c_char,
                          mask: 256 as libc::c_int as uint32_t,};
         init
     },
     {
         let mut init =
             acl_op_table{op: 'x' as i32 as libc::c_char,
                          mask:
                              (1 as libc::c_int | 2 as libc::c_int |
                                   4 as libc::c_int | 8 as libc::c_int |
                                   32 as libc::c_int | 128 as libc::c_int |
                                   512 as libc::c_int | 256 as libc::c_int) as
                                  uint32_t,};
         init
     },
     {
         let mut init =
             acl_op_table{op: '*' as i32 as libc::c_char,
                          mask:
                              (1 as libc::c_int | 2 as libc::c_int |
                                   4 as libc::c_int | 8 as libc::c_int |
                                   32 as libc::c_int | 128 as libc::c_int |
                                   512 as libc::c_int | 256 as libc::c_int) as
                                  uint32_t,};
         init
     },
     {
         let mut init =
             acl_op_table{op: 'e' as i32 as libc::c_char,
                          mask: 64 as libc::c_int as uint32_t,};
         init
     },
     {
         let mut init =
             acl_op_table{op: '\u{0}' as i32 as libc::c_char,
                          mask: 0 as libc::c_int as uint32_t,};
         init
     }];
/*
 * Get a line from the ACL file.  Lines ending with \ are continued on the next
 * line.  The caller should set *lineno to 1 and *incr to 0 before the first
 * call.  On successful return, *lineno will be the line number of the line
 * read.  Return a pointer to the line on success, or NULL on end of file or
 * read failure.
 */
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn get_line(mut fp: *mut FILE,
                              mut fname: *const libc::c_char,
                              mut lineno: *mut libc::c_int,
                              mut incr: *mut libc::c_int)
 -> *mut libc::c_char {
    let chunksize: libc::c_int = 128 as libc::c_int;
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut old_len: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Increment *lineno by the number of newlines from the last line. */
    *lineno += *incr;
    *incr = 0 as libc::c_int;
    k5_buf_init_dynamic(&mut buf);
    loop  {
        /* Read at least part of a line into the buffer. */
        old_len = buf.len;
        p =
            k5_buf_get_space(&mut buf, chunksize as size_t) as
                *mut libc::c_char;
        if p.is_null() { return 0 as *mut libc::c_char }
        if fgets(p, chunksize, fp).is_null() {
            /* We reached the end.  Return a final unterminated line, if there
             * is one and it's not a comment. */
            k5_buf_truncate(&mut buf, old_len);
            if buf.len > 0 as libc::c_int as libc::c_ulong &&
                   *(buf.data as *mut libc::c_char) as libc::c_int !=
                       '#' as i32 {
                return buf.data as *mut libc::c_char
            }
            k5_buf_free(&mut buf);
            return 0 as *mut libc::c_char
        }
        /* Set the buffer length based on the actual amount read. */
        k5_buf_truncate(&mut buf, old_len.wrapping_add(strlen(p)));
        p = buf.data as *mut libc::c_char;
        if buf.len > 0 as libc::c_int as libc::c_ulong &&
               *p.offset(buf.len.wrapping_sub(1 as libc::c_int as
                                                  libc::c_ulong) as isize) as
                   libc::c_int == '\n' as i32 {
            /* We have a complete raw line in the buffer. */
            *incr += 1;
            k5_buf_truncate(&mut buf,
                            buf.len.wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong));
            if buf.len > 0 as libc::c_int as libc::c_ulong &&
                   *p.offset(buf.len.wrapping_sub(1 as libc::c_int as
                                                      libc::c_ulong) as isize)
                       as libc::c_int == '\\' as i32 {
                /* This line has a continuation marker; keep reading. */
                k5_buf_truncate(&mut buf,
                                buf.len.wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong));
            } else if buf.len == 0 as libc::c_int as libc::c_ulong ||
                          *p as libc::c_int == '#' as i32 {
                /* This line is empty or a comment.  Start over. */
                *lineno += *incr;
                *incr = 0 as libc::c_int;
                k5_buf_truncate(&mut buf, 0 as libc::c_int as size_t);
            } else { return buf.data as *mut libc::c_char }
        }
    };
}
/*
 * Parse a restrictions field.  Return NULL on failure.
 *
 * Allowed restrictions are:
 *      [+-]flagname            (recognized by krb5_flagspec_to_mask)
 *                              flag is forced to indicated value
 *      -clearpolicy            policy is forced clear
 *      -policy pol             policy is forced to be "pol"
 *      -{expire,pwexpire,maxlife,maxrenewlife} deltat
 *                              associated value will be forced to
 *                              MIN(deltat, requested value)
 */
#[c2rust::src_loc = "167:1"]
unsafe extern "C" fn parse_restrictions(mut str: *const libc::c_char,
                                        mut fname: *const libc::c_char)
 -> *mut kadm5_auth_restrictions {
    let mut current_block: u64;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut delims: *const libc::c_char =
        b"\t\n\x0c\x0b\r ,\x00" as *const u8 as *const libc::c_char;
    let mut delta: krb5_deltat = 0;
    let mut rs: *mut kadm5_auth_restrictions =
        0 as *mut kadm5_auth_restrictions;
    copy = strdup(str);
    if copy.is_null() { return 0 as *mut kadm5_auth_restrictions }
    rs =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<kadm5_auth_restrictions>() as
                   libc::c_ulong) as *mut kadm5_auth_restrictions;
    if rs.is_null() {
        free(copy as *mut libc::c_void);
        return 0 as *mut kadm5_auth_restrictions
    }
    (*rs).forbid_attrs = !(0 as libc::c_int);
    token = strtok_r(copy, delims, &mut save);
    loop  {
        if token.is_null() { current_block = 10891380440665537214; break ; }
        if krb5_flagspec_to_mask(token, &mut (*rs).require_attrs,
                                 &mut (*rs).forbid_attrs) == 0 as libc::c_int
           {
            (*rs).mask |= 0x10 as libc::c_int as libc::c_long
        } else if strcmp(token,
                         b"-clearpolicy\x00" as *const u8 as
                             *const libc::c_char) == 0 as libc::c_int {
            (*rs).mask |= 0x1000 as libc::c_int as libc::c_long
        } else {
            /* Everything else needs an argument. */
            arg = strtok_r(0 as *mut libc::c_char, delims, &mut save);
            if arg.is_null() { current_block = 18138177649957003941; break ; }
            if strcmp(token,
                      b"-policy\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
                if !(*rs).policy.is_null() {
                    current_block = 18138177649957003941;
                    break ;
                }
                (*rs).policy = strdup(arg);
                if (*rs).policy.is_null() {
                    current_block = 18138177649957003941;
                    break ;
                }
                (*rs).mask |= 0x800 as libc::c_int as libc::c_long
            } else {
                /* All other arguments must be a deltat. */
                if krb5_string_to_deltat(arg, &mut delta) != 0 as libc::c_int
                   {
                    current_block = 18138177649957003941;
                    break ;
                }
                if strcmp(token,
                          b"-expire\x00" as *const u8 as *const libc::c_char)
                       == 0 as libc::c_int {
                    (*rs).princ_lifetime = delta;
                    (*rs).mask |= 0x2 as libc::c_int as libc::c_long
                } else if strcmp(token,
                                 b"-pwexpire\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*rs).pw_lifetime = delta;
                    (*rs).mask |= 0x4 as libc::c_int as libc::c_long
                } else if strcmp(token,
                                 b"-maxlife\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    (*rs).max_life = delta;
                    (*rs).mask |= 0x20 as libc::c_int as libc::c_long
                } else {
                    if !(strcmp(token,
                                b"-maxrenewlife\x00" as *const u8 as
                                    *const libc::c_char) == 0 as libc::c_int)
                       {
                        current_block = 18138177649957003941;
                        break ;
                    }
                    (*rs).max_renewable_life = delta;
                    (*rs).mask |= 0x2000 as libc::c_int as libc::c_long
                }
            }
        }
        token = strtok_r(0 as *mut libc::c_char, delims, &mut save)
    }
    match current_block {
        18138177649957003941 => {
            krb5_klog_syslog(3 as libc::c_int,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"%s: invalid restrictions: %s\x00" as
                                          *const u8 as *const libc::c_char),
                             fname, str);
            free(copy as *mut libc::c_void);
            free((*rs).policy as *mut libc::c_void);
            free(rs as *mut libc::c_void);
            return 0 as *mut kadm5_auth_restrictions
        }
        _ => { free(copy as *mut libc::c_void); return rs }
    };
}
#[c2rust::src_loc = "247:1"]
unsafe extern "C" fn free_acl_entry(mut entry: *mut acl_entry) {
    krb5_free_principal(0 as krb5_context, (*entry).client);
    krb5_free_principal(0 as krb5_context, (*entry).target);
    if !(*entry).rs.is_null() {
        free((*(*entry).rs).policy as *mut libc::c_void);
        free((*entry).rs as *mut libc::c_void);
    }
    free(entry as *mut libc::c_void);
}
/* Parse the four fields of an ACL entry and return a structure representing
 * it.  Log a message and return NULL on error. */
#[c2rust::src_loc = "261:1"]
unsafe extern "C" fn parse_entry(mut context: krb5_context,
                                 mut client: *const libc::c_char,
                                 mut ops: *const libc::c_char,
                                 mut target: *const libc::c_char,
                                 mut rs: *const libc::c_char,
                                 mut line: *const libc::c_char,
                                 mut fname: *const libc::c_char)
 -> *mut acl_entry {
    let mut current_block: u64;
    let mut entry: *mut acl_entry = 0 as *mut acl_entry;
    let mut op: *const libc::c_char = 0 as *const libc::c_char;
    let mut rop: libc::c_char = 0;
    let mut t: libc::c_int = 0;
    entry =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<acl_entry>() as libc::c_ulong) as
            *mut acl_entry;
    if entry.is_null() { return 0 as *mut acl_entry }
    op = ops;
    loop  {
        if !(*op != 0) { current_block = 5689001924483802034; break ; }
        rop =
            if *(*__ctype_b_loc()).offset(*op as libc::c_uchar as libc::c_int
                                              as isize) as libc::c_int &
                   _ISupper as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                tolower(*op as libc::c_uchar as libc::c_int)
            } else { *op as libc::c_int } as libc::c_char;
        t = 0 as libc::c_int;
        while acl_op_table[t as usize].op != 0 {
            if rop as libc::c_int ==
                   acl_op_table[t as usize].op as libc::c_int {
                if rop as libc::c_int == *op as libc::c_int {
                    (*entry).op_allowed |= acl_op_table[t as usize].mask
                } else {
                    (*entry).op_allowed &= !acl_op_table[t as usize].mask
                }
                break ;
            } else { t += 1 }
        }
        if acl_op_table[t as usize].op == 0 {
            krb5_klog_syslog(3 as libc::c_int,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"Unrecognized ACL operation \'%c\' in %s\x00"
                                          as *const u8 as
                                          *const libc::c_char),
                             *op as libc::c_int, line);
            current_block = 4770989289089330907;
            break ;
        } else { op = op.offset(1) }
    }
    match current_block {
        5689001924483802034 => {
            if strcmp(client, b"*\x00" as *const u8 as *const libc::c_char) !=
                   0 as libc::c_int {
                if krb5_parse_name(context, client, &mut (*entry).client) !=
                       0 as libc::c_int {
                    krb5_klog_syslog(3 as libc::c_int,
                                     dgettext(b"mit-krb5\x00" as *const u8 as
                                                  *const libc::c_char,
                                              b"Cannot parse client principal \'%s\'\x00"
                                                  as *const u8 as
                                                  *const libc::c_char),
                                     client);
                    current_block = 4770989289089330907;
                } else { current_block = 18317007320854588510; }
            } else { current_block = 18317007320854588510; }
            match current_block {
                4770989289089330907 => { }
                _ => {
                    if !target.is_null() &&
                           strcmp(target,
                                  b"*\x00" as *const u8 as
                                      *const libc::c_char) != 0 as libc::c_int
                       {
                        if krb5_parse_name(context, target,
                                           &mut (*entry).target) !=
                               0 as libc::c_int {
                            krb5_klog_syslog(3 as libc::c_int,
                                             dgettext(b"mit-krb5\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      b"Cannot parse target principal \'%s\'\x00"
                                                          as *const u8 as
                                                          *const libc::c_char),
                                             target);
                            current_block = 4770989289089330907;
                        } else { current_block = 7056779235015430508; }
                    } else { current_block = 7056779235015430508; }
                    match current_block {
                        4770989289089330907 => { }
                        _ => {
                            if !rs.is_null() {
                                (*entry).rs = parse_restrictions(rs, fname);
                                if (*entry).rs.is_null() {
                                    current_block = 4770989289089330907;
                                } else {
                                    current_block = 11932355480408055363;
                                }
                            } else { current_block = 11932355480408055363; }
                            match current_block {
                                4770989289089330907 => { }
                                _ => { return entry }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    free_acl_entry(entry);
    return 0 as *mut acl_entry;
}
/* Parse the contents of an ACL line. */
#[c2rust::src_loc = "324:1"]
unsafe extern "C" fn parse_line(mut context: krb5_context,
                                mut line: *const libc::c_char,
                                mut fname: *const libc::c_char)
 -> *mut acl_entry {
    let mut entry: *mut acl_entry = 0 as *mut acl_entry;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ops: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ops_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut target_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ws: *const libc::c_char =
        b"\t\n\x0c\x0b\r ,\x00" as *const u8 as *const libc::c_char;
    /*
     * Format:
     *  entry ::= [<whitespace>] <principal> <whitespace> <opstring>
     *            [<whitespace> <target> [<whitespace> <restrictions>
     *                                    [<whitespace>]]]
     */
    /* Make a copy and remove any trailing whitespace. */
    copy = strdup(line);
    if copy.is_null() { return 0 as *mut acl_entry }
    end = copy.offset(strlen(copy) as isize);
    while end > copy &&
              *(*__ctype_b_loc()).offset(*end.offset(-(1 as libc::c_int) as
                                                         isize) as libc::c_int
                                             as isize) as libc::c_int &
                  _ISspace as libc::c_int as libc::c_ushort as libc::c_int !=
                  0 {
        end = end.offset(-1);
        *end = '\u{0}' as i32 as libc::c_char
    }
    /* Find the beginning and end of each field.  The end of restrictions is
     * the end of copy. */
    client = copy.offset(strspn(copy, ws) as isize);
    client_end = client.offset(strcspn(client, ws) as isize);
    ops = client_end.offset(strspn(client_end, ws) as isize);
    ops_end = ops.offset(strcspn(ops, ws) as isize);
    target = ops_end.offset(strspn(ops_end, ws) as isize);
    target_end = target.offset(strcspn(target, ws) as isize);
    rs = target_end.offset(strspn(target_end, ws) as isize);
    /* Terminate the first three fields. */
    *target_end = '\u{0}' as i32 as libc::c_char;
    *ops_end = *target_end;
    *client_end = *ops_end;
    /* The last two fields are optional; represent them as NULL if not present.
     * The first two fields are required. */
    if *target as libc::c_int == '\u{0}' as i32 {
        target = 0 as *mut libc::c_char
    }
    if *rs as libc::c_int == '\u{0}' as i32 { rs = 0 as *mut libc::c_char }
    if *client as libc::c_int != '\u{0}' as i32 &&
           *ops as libc::c_int != '\u{0}' as i32 {
        entry = parse_entry(context, client, ops, target, rs, line, fname)
    }
    free(copy as *mut libc::c_void);
    return entry;
}
/* Free all ACL entries. */
#[c2rust::src_loc = "373:1"]
unsafe extern "C" fn free_acl_entries(mut state: *mut acl_state) {
    let mut entry: *mut acl_entry = 0 as *mut acl_entry;
    let mut next: *mut acl_entry = 0 as *mut acl_entry;
    entry = (*state).list;
    while !entry.is_null() {
        next = (*entry).next;
        free_acl_entry(entry);
        entry = next
    }
    (*state).list = 0 as *mut acl_entry;
}
/* Open and parse the ACL file. */
#[c2rust::src_loc = "386:1"]
unsafe extern "C" fn load_acl_file(mut context: krb5_context,
                                   mut fname: *const libc::c_char,
                                   mut state: *mut acl_state)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut entry_slot: *mut *mut acl_entry = 0 as *mut *mut acl_entry;
    let mut lineno: libc::c_int = 0;
    let mut incr: libc::c_int = 0;
    (*state).list = 0 as *mut acl_entry;
    /* Open the ACL file for reading. */
    fp = fopen(fname, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"%s while opening ACL file %s\x00" as
                                      *const u8 as *const libc::c_char),
                         error_message(*__errno_location() as errcode_t),
                         fname);
        ret = *__errno_location();
        krb5_set_error_message(context, *__errno_location(),
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Cannot open %s: %s\x00" as *const u8
                                            as *const libc::c_char), fname,
                               error_message(ret as errcode_t));
        return ret
    }
    fcntl(fileno(fp), 2 as libc::c_int, 1 as libc::c_int);
    lineno = 1 as libc::c_int;
    incr = 0 as libc::c_int;
    entry_slot = &mut (*state).list;
    loop 
         /* Get a non-comment line. */
         {
        line = get_line(fp, fname, &mut lineno, &mut incr);
        if line.is_null() { break ; }
        /* Parse it.  Fail out on syntax error. */
        *entry_slot = parse_line(context, line, fname);
        if (*entry_slot).is_null() {
            krb5_klog_syslog(3 as libc::c_int,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"%s: syntax error at line %d <%.10s...>\x00"
                                          as *const u8 as
                                          *const libc::c_char), fname, lineno,
                             line);
            krb5_set_error_message(context, 22 as libc::c_int,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"%s: syntax error at line %d <%.10s...>\x00"
                                                as *const u8 as
                                                *const libc::c_char), fname,
                                   lineno, line);
            free_acl_entries(state);
            free(line as *mut libc::c_void);
            fclose(fp);
            return 22 as libc::c_int
        }
        entry_slot = &mut (**entry_slot).next;
        free(line as *mut libc::c_void);
    }
    fclose(fp);
    return 0 as libc::c_int;
}
/*
 * See if two data entries match.  If e1 is a wildcard (matching a whole
 * component only) and targetflag is false, save an alias to e2 into
 * ws->backref.  If e1 is a back-reference and targetflag is true, compare the
 * appropriate entry in ws->backref to e2.  If ws is NULL, do not store or
 * match back-references.
 */
#[c2rust::src_loc = "444:1"]
unsafe extern "C" fn match_data(mut e1: *const krb5_data,
                                mut e2: *const krb5_data,
                                mut targetflag: krb5_boolean,
                                mut ws: *mut wildstate) -> krb5_boolean {
    let mut n: libc::c_int = 0;
    if data_eq_string(*e1, b"*\x00" as *const u8 as *const libc::c_char) != 0
       {
        if !ws.is_null() && targetflag == 0 {
            if (*ws).nwild < 9 as libc::c_int {
                let fresh0 = (*ws).nwild;
                (*ws).nwild = (*ws).nwild + 1;
                (*ws).backref[fresh0 as usize] = e2
            }
        }
        return 1 as libc::c_int as krb5_boolean
    }
    if !ws.is_null() && targetflag != 0 &&
           (*e1).length == 2 as libc::c_int as libc::c_uint &&
           *(*e1).data.offset(0 as libc::c_int as isize) as libc::c_int ==
               '*' as i32 &&
           *(*e1).data.offset(1 as libc::c_int as isize) as libc::c_int >=
               '1' as i32 &&
           *(*e1).data.offset(1 as libc::c_int as isize) as libc::c_int <=
               '9' as i32 {
        n =
            *(*e1).data.offset(1 as libc::c_int as isize) as libc::c_int -
                '1' as i32;
        if n >= (*ws).nwild { return 0 as libc::c_int as krb5_boolean }
        return data_eq(*e2, *(*ws).backref[n as usize]) as krb5_boolean
    } else { return data_eq(*e2, *e1) as krb5_boolean };
}
/* Return true if p1 matches p2.  p1 may contain wildcards if targetflag is
 * false, or backreferences if it is true. */
#[c2rust::src_loc = "471:1"]
unsafe extern "C" fn match_princ(mut p1: krb5_const_principal,
                                 mut p2: krb5_const_principal,
                                 mut targetflag: krb5_boolean,
                                 mut ws: *mut wildstate) -> krb5_boolean {
    let mut i: libc::c_int = 0;
    /* The principals must be of the same length. */
    if (*p1).length != (*p2).length {
        return 0 as libc::c_int as krb5_boolean
    }
    /* The realm must match, and does not interact with wildcard state. */
    if match_data(&(*p1).realm, &(*p2).realm, targetflag, 0 as *mut wildstate)
           == 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    /* All components of the principals must match. */
    i = 0 as libc::c_int;
    while i < (*p1).length {
        if match_data(&mut *(*p1).data.offset(i as isize),
                      &mut *(*p2).data.offset(i as isize), targetflag, ws) ==
               0 {
            return 0 as libc::c_int as krb5_boolean
        }
        i += 1
    }
    return 1 as libc::c_int as krb5_boolean;
}
/* Find an ACL entry matching principal and target_principal.  Return NULL if
 * none is found. */
#[c2rust::src_loc = "496:1"]
unsafe extern "C" fn find_entry(mut state: *mut acl_state,
                                mut client: krb5_const_principal,
                                mut target: krb5_const_principal)
 -> *mut acl_entry {
    let mut entry: *mut acl_entry = 0 as *mut acl_entry;
    let mut ws: wildstate =
        wildstate{nwild: 0, backref: [0 as *const krb5_data; 9],};
    let mut current_block_2: u64;
    entry = (*state).list;
    while !entry.is_null() {
        memset(&mut ws as *mut wildstate as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<wildstate>() as libc::c_ulong);
        if !(*entry).client.is_null() {
            if match_princ((*entry).client as krb5_const_principal, client,
                           0 as libc::c_int as krb5_boolean, &mut ws) == 0 {
                current_block_2 = 16559507199688588974;
            } else { current_block_2 = 2473556513754201174; }
        } else { current_block_2 = 2473556513754201174; }
        match current_block_2 {
            2473556513754201174 => {
                if !(*entry).target.is_null() {
                    if target.is_null() {
                        current_block_2 = 16559507199688588974;
                    } else if match_princ((*entry).target as
                                              krb5_const_principal, target,
                                          1 as libc::c_int as krb5_boolean,
                                          &mut ws) == 0 {
                        current_block_2 = 16559507199688588974;
                    } else { current_block_2 = 2868539653012386629; }
                } else { current_block_2 = 2868539653012386629; }
                match current_block_2 {
                    16559507199688588974 => { }
                    _ => { return entry }
                }
            }
            _ => { }
        }
        entry = (*entry).next
    }
    return 0 as *mut acl_entry;
}
/* Return true if op is permitted for this principal.  Set *rs_out (if not
 * NULL) according to any restrictions in the ACL entry. */
#[c2rust::src_loc = "525:1"]
unsafe extern "C" fn acl_check(mut data: kadm5_auth_moddata, mut op: uint32_t,
                               mut client: krb5_const_principal,
                               mut target: krb5_const_principal,
                               mut rs_out: *mut *mut kadm5_auth_restrictions)
 -> krb5_error_code {
    let mut entry: *mut acl_entry = 0 as *mut acl_entry;
    if !rs_out.is_null() { *rs_out = 0 as *mut kadm5_auth_restrictions }
    entry = find_entry(data as *mut acl_state, client, target);
    if entry.is_null() {
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    if (*entry).op_allowed & op == 0 {
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    if !rs_out.is_null() && !(*entry).rs.is_null() && (*(*entry).rs).mask != 0
       {
        *rs_out = (*entry).rs
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "546:1"]
unsafe extern "C" fn acl_init(mut context: krb5_context,
                              mut acl_file: *const libc::c_char,
                              mut data_out: *mut kadm5_auth_moddata)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut state: *mut acl_state = 0 as *mut acl_state;
    *data_out = 0 as kadm5_auth_moddata;
    if acl_file.is_null() {
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    state =
        malloc(::std::mem::size_of::<acl_state>() as libc::c_ulong) as
            *mut acl_state;
    (*state).list = 0 as *mut acl_entry;
    ret = load_acl_file(context, acl_file, state);
    if ret != 0 { free(state as *mut libc::c_void); return ret }
    *data_out = state as kadm5_auth_moddata;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "567:1"]
unsafe extern "C" fn acl_fini(mut context: krb5_context,
                              mut data: kadm5_auth_moddata) {
    if data.is_null() { return }
    free_acl_entries(data as *mut acl_state);
    free(data as *mut libc::c_void);
}
#[c2rust::src_loc = "576:1"]
unsafe extern "C" fn acl_addprinc(mut context: krb5_context,
                                  mut data: kadm5_auth_moddata,
                                  mut client: krb5_const_principal,
                                  mut target: krb5_const_principal,
                                  mut ent: *const _kadm5_principal_ent_t,
                                  mut mask: libc::c_long,
                                  mut rs_out:
                                      *mut *mut kadm5_auth_restrictions)
 -> krb5_error_code {
    return acl_check(data, 1 as libc::c_int as uint32_t, client, target,
                     rs_out);
}
#[c2rust::src_loc = "585:1"]
unsafe extern "C" fn acl_modprinc(mut context: krb5_context,
                                  mut data: kadm5_auth_moddata,
                                  mut client: krb5_const_principal,
                                  mut target: krb5_const_principal,
                                  mut ent: *const _kadm5_principal_ent_t,
                                  mut mask: libc::c_long,
                                  mut rs_out:
                                      *mut *mut kadm5_auth_restrictions)
 -> krb5_error_code {
    return acl_check(data, 4 as libc::c_int as uint32_t, client, target,
                     rs_out);
}
#[c2rust::src_loc = "594:1"]
unsafe extern "C" fn acl_setstr(mut context: krb5_context,
                                mut data: kadm5_auth_moddata,
                                mut client: krb5_const_principal,
                                mut target: krb5_const_principal,
                                mut key: *const libc::c_char,
                                mut value: *const libc::c_char)
 -> krb5_error_code {
    return acl_check(data, 4 as libc::c_int as uint32_t, client, target,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "602:1"]
unsafe extern "C" fn acl_cpw(mut context: krb5_context,
                             mut data: kadm5_auth_moddata,
                             mut client: krb5_const_principal,
                             mut target: krb5_const_principal)
 -> krb5_error_code {
    return acl_check(data, 8 as libc::c_int as uint32_t, client, target,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "609:1"]
unsafe extern "C" fn acl_chrand(mut context: krb5_context,
                                mut data: kadm5_auth_moddata,
                                mut client: krb5_const_principal,
                                mut target: krb5_const_principal)
 -> krb5_error_code {
    return acl_check(data, 8 as libc::c_int as uint32_t, client, target,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "616:1"]
unsafe extern "C" fn acl_setkey(mut context: krb5_context,
                                mut data: kadm5_auth_moddata,
                                mut client: krb5_const_principal,
                                mut target: krb5_const_principal)
 -> krb5_error_code {
    return acl_check(data, 256 as libc::c_int as uint32_t, client, target,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "623:1"]
unsafe extern "C" fn acl_purgekeys(mut context: krb5_context,
                                   mut data: kadm5_auth_moddata,
                                   mut client: krb5_const_principal,
                                   mut target: krb5_const_principal)
 -> krb5_error_code {
    return acl_check(data, 4 as libc::c_int as uint32_t, client, target,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "630:1"]
unsafe extern "C" fn acl_delprinc(mut context: krb5_context,
                                  mut data: kadm5_auth_moddata,
                                  mut client: krb5_const_principal,
                                  mut target: krb5_const_principal)
 -> krb5_error_code {
    return acl_check(data, 2 as libc::c_int as uint32_t, client, target,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "637:1"]
unsafe extern "C" fn acl_renprinc(mut context: krb5_context,
                                  mut data: kadm5_auth_moddata,
                                  mut client: krb5_const_principal,
                                  mut src: krb5_const_principal,
                                  mut dest: krb5_const_principal)
 -> krb5_error_code {
    let mut rs: *mut kadm5_auth_restrictions =
        0 as *mut kadm5_auth_restrictions;
    if acl_check(data, 2 as libc::c_int as uint32_t, client, src,
                 0 as *mut *mut kadm5_auth_restrictions) == 0 as libc::c_int
           &&
           acl_check(data, 1 as libc::c_int as uint32_t, client, dest,
                     &mut rs) == 0 as libc::c_int && rs.is_null() {
        return 0 as libc::c_int
    }
    return -(1765328135 as libc::c_long) as krb5_error_code;
}
#[c2rust::src_loc = "650:1"]
unsafe extern "C" fn acl_getprinc(mut context: krb5_context,
                                  mut data: kadm5_auth_moddata,
                                  mut client: krb5_const_principal,
                                  mut target: krb5_const_principal)
 -> krb5_error_code {
    return acl_check(data, 32 as libc::c_int as uint32_t, client, target,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "657:1"]
unsafe extern "C" fn acl_getstrs(mut context: krb5_context,
                                 mut data: kadm5_auth_moddata,
                                 mut client: krb5_const_principal,
                                 mut target: krb5_const_principal)
 -> krb5_error_code {
    return acl_check(data, 32 as libc::c_int as uint32_t, client, target,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "664:1"]
unsafe extern "C" fn acl_extract(mut context: krb5_context,
                                 mut data: kadm5_auth_moddata,
                                 mut client: krb5_const_principal,
                                 mut target: krb5_const_principal)
 -> krb5_error_code {
    return acl_check(data, 64 as libc::c_int as uint32_t, client, target,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "671:1"]
unsafe extern "C" fn acl_listprincs(mut context: krb5_context,
                                    mut data: kadm5_auth_moddata,
                                    mut client: krb5_const_principal)
 -> krb5_error_code {
    return acl_check(data, 128 as libc::c_int as uint32_t, client,
                     0 as krb5_const_principal,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "678:1"]
unsafe extern "C" fn acl_addpol(mut context: krb5_context,
                                mut data: kadm5_auth_moddata,
                                mut client: krb5_const_principal,
                                mut policy: *const libc::c_char,
                                mut ent: *const _kadm5_policy_ent_t,
                                mut mask: libc::c_long) -> krb5_error_code {
    return acl_check(data, 1 as libc::c_int as uint32_t, client,
                     0 as krb5_const_principal,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "686:1"]
unsafe extern "C" fn acl_modpol(mut context: krb5_context,
                                mut data: kadm5_auth_moddata,
                                mut client: krb5_const_principal,
                                mut policy: *const libc::c_char,
                                mut ent: *const _kadm5_policy_ent_t,
                                mut mask: libc::c_long) -> krb5_error_code {
    return acl_check(data, 4 as libc::c_int as uint32_t, client,
                     0 as krb5_const_principal,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "694:1"]
unsafe extern "C" fn acl_delpol(mut context: krb5_context,
                                mut data: kadm5_auth_moddata,
                                mut client: krb5_const_principal,
                                mut policy: *const libc::c_char)
 -> krb5_error_code {
    return acl_check(data, 2 as libc::c_int as uint32_t, client,
                     0 as krb5_const_principal,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "701:1"]
unsafe extern "C" fn acl_getpol(mut context: krb5_context,
                                mut data: kadm5_auth_moddata,
                                mut client: krb5_const_principal,
                                mut policy: *const libc::c_char,
                                mut client_policy: *const libc::c_char)
 -> krb5_error_code {
    return acl_check(data, 32 as libc::c_int as uint32_t, client,
                     0 as krb5_const_principal,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "709:1"]
unsafe extern "C" fn acl_listpols(mut context: krb5_context,
                                  mut data: kadm5_auth_moddata,
                                  mut client: krb5_const_principal)
 -> krb5_error_code {
    return acl_check(data, 128 as libc::c_int as uint32_t, client,
                     0 as krb5_const_principal,
                     0 as *mut *mut kadm5_auth_restrictions);
}
#[c2rust::src_loc = "716:1"]
unsafe extern "C" fn acl_iprop(mut context: krb5_context,
                               mut data: kadm5_auth_moddata,
                               mut client: krb5_const_principal)
 -> krb5_error_code {
    return acl_check(data, 512 as libc::c_int as uint32_t, client,
                     0 as krb5_const_principal,
                     0 as *mut *mut kadm5_auth_restrictions);
}
/* initvt declarations for built-in modules */
#[no_mangle]
#[c2rust::src_loc = "723:1"]
pub unsafe extern "C" fn kadm5_auth_acl_initvt(mut context: krb5_context,
                                               mut maj_ver: libc::c_int,
                                               mut min_ver: libc::c_int,
                                               mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: kadm5_auth_vtable = 0 as *mut kadm5_auth_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as kadm5_auth_vtable;
    (*vt).name = b"acl\x00" as *const u8 as *const libc::c_char;
    (*vt).init =
        Some(acl_init as
                 unsafe extern "C" fn(_: krb5_context, _: *const libc::c_char,
                                      _: *mut kadm5_auth_moddata)
                     -> krb5_error_code);
    (*vt).fini =
        Some(acl_fini as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata)
                     -> ());
    (*vt).addprinc =
        Some(acl_addprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: *const _kadm5_principal_ent_t,
                                      _: libc::c_long,
                                      _: *mut *mut kadm5_auth_restrictions)
                     -> krb5_error_code);
    (*vt).modprinc =
        Some(acl_modprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: *const _kadm5_principal_ent_t,
                                      _: libc::c_long,
                                      _: *mut *mut kadm5_auth_restrictions)
                     -> krb5_error_code);
    (*vt).setstr =
        Some(acl_setstr as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char)
                     -> krb5_error_code);
    (*vt).cpw =
        Some(acl_cpw as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).chrand =
        Some(acl_chrand as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).setkey =
        Some(acl_setkey as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).purgekeys =
        Some(acl_purgekeys as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).delprinc =
        Some(acl_delprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).renprinc =
        Some(acl_renprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).getprinc =
        Some(acl_getprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).getstrs =
        Some(acl_getstrs as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).extract =
        Some(acl_extract as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).listprincs =
        Some(acl_listprincs as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).addpol =
        Some(acl_addpol as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const _kadm5_policy_ent_t,
                                      _: libc::c_long) -> krb5_error_code);
    (*vt).modpol =
        Some(acl_modpol as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const _kadm5_policy_ent_t,
                                      _: libc::c_long) -> krb5_error_code);
    (*vt).delpol =
        Some(acl_delpol as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char)
                     -> krb5_error_code);
    (*vt).getpol =
        Some(acl_getpol as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char)
                     -> krb5_error_code);
    (*vt).listpols =
        Some(acl_listpols as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).iprop =
        Some(acl_iprop as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    return 0 as libc::c_int;
}
