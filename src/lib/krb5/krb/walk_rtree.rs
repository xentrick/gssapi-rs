use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:34"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:34"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:34"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:34"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /*
 * end wordsize.h
 */
    /*
 * begin "base-defs.h"
 */
    /*
 * Basic definitions for Kerberos V5 library
 */
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
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
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
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
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* This file is generated, please don't edit it directly.  */
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* General definitions for Kerberos version 5. */
/*
 * Copyright 1989, 1990, 1995, 2001, 2003, 2007, 2011 by the Massachusetts
 * Institute of Technology.  All Rights Reserved.
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
        /* * @defgroup KRB5_H krb5 library API
 * @{
 */
        /* By default, do not expose deprecated interfaces. */
        /* !KRB5_CONFIG__ */
        /* for *_MAX */
        /* from profile.h */
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4758:1"]
        pub fn krb5_free_data_contents(context: krb5_context,
                                       val: *mut krb5_data);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:34"]
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
    #[c2rust::src_loc = "2262:1"]
    pub unsafe extern "C" fn empty_data() -> krb5_data {
        return make_data(0 as *mut libc::c_void,
                         0 as libc::c_int as libc::c_uint);
    }
    #[inline]
    #[c2rust::src_loc = "2251:1"]
    pub unsafe extern "C" fn make_data(mut data: *mut libc::c_void,
                                       mut len: libc::c_uint) -> krb5_data {
        let mut d: krb5_data =
            krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
        d.magic = -(1760647422 as libc::c_long) as krb5_magic;
        d.data = data as *mut libc::c_char;
        d.length = len;
        return d;
    }
    #[inline]
    #[c2rust::src_loc = "2326:1"]
    pub unsafe extern "C" fn k5memdup0(mut in_0: *const libc::c_void,
                                       mut len: size_t,
                                       mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void =
            k5alloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
        return ptr;
    }
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        ptr =
            calloc(if nmemb != 0 {
                       nmemb
                   } else { 1 as libc::c_int as libc::c_ulong },
                   if size != 0 {
                       size
                   } else { 1 as libc::c_int as libc::c_ulong });
        *code =
            if ptr.is_null() { 12 as libc::c_int } else { 0 as libc::c_int };
        return ptr;
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data, krb5_context, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stddef_h::size_t;
    use super::string_h::{memcpy, memcmp};
    use super::stdlib_h::calloc;
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
        #[no_mangle]
        #[c2rust::src_loc = "898:1"]
        pub fn krb5int_copy_data_contents(_: krb5_context,
                                          _: *const krb5_data,
                                          _: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1004:1"]
        pub fn krb5int_free_data_list(context: krb5_context,
                                      data: *mut krb5_data);
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:34"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:34"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:34"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn profile_get_values(profile: profile_t,
                                  names: *const *const libc::c_char,
                                  ret_values: *mut *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn profile_free_list(list: *mut *mut libc::c_char);
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdlib.h:34"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:34"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "272:15"]
        pub fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:35"]
pub mod int_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_data, krb5_principal,
                        krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "40:1"]
        pub fn krb5int_tgtname(context: krb5_context, _: *const krb5_data,
                               _: *const krb5_data, _: *mut krb5_principal)
         -> krb5_error_code;
    }
    /* KRB5_INT_FUNC_PROTO__ */
}
pub use self::types_h::__int32_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _profile_t, krb5_free_principal,
                       krb5_free_data_contents};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, empty_data, make_data, k5memdup0,
                         k5alloc, k5calloc, data_eq, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5int_copy_data_contents,
                         krb5int_free_data_list};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_get_values, profile_free_list};
use self::stdlib_h::{calloc, realloc, free};
use self::string_h::{strcspn, memcmp, memcpy};
use self::int_proto_h::krb5int_tgtname;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/walk_rtree.c */
/*
 * Copyright 1990,1991,2008,2009 by the Massachusetts Institute of Technology.
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
 * krb5_walk_realm_tree()
 * krb5_free_realm_tree()
 *
 * internal function, used by krb5_get_cred_from_kdc()
 */
/*
 * Structure to help with finding the common suffix between client and
 * server realm during hierarchical traversal.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "41:8"]
pub struct hstate {
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub tail: *mut libc::c_char,
    pub dot: *mut libc::c_char,
}
#[no_mangle]
#[c2rust::src_loc = "95:1"]
pub unsafe extern "C" fn krb5_walk_realm_tree(mut context: krb5_context,
                                              mut client: *const krb5_data,
                                              mut server: *const krb5_data,
                                              mut tree:
                                                  *mut *mut krb5_principal,
                                              mut realm_sep: libc::c_int)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut capvals: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if (*client).data.is_null() || (*server).data.is_null() {
        return -(1765328199 as libc::c_long) as krb5_error_code
    }
    if data_eq(*client, *server) != 0 {
        return -(1765328199 as libc::c_long) as krb5_error_code
    }
    retval = rtree_capath_vals(context, client, server, &mut capvals);
    if retval != 0 { return retval }
    if !capvals.is_null() {
        retval = rtree_capath_tree(context, client, server, capvals, tree);
        return retval
    }
    retval = rtree_hier_tree(context, client, server, tree, realm_sep);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "123:1"]
pub unsafe extern "C" fn k5_client_realm_path(mut context: krb5_context,
                                              mut client: *const krb5_data,
                                              mut server: *const krb5_data,
                                              mut rpath_out:
                                                  *mut *mut krb5_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut capvals: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: size_t = 0;
    let mut rpath: *mut krb5_data = 0 as *mut krb5_data;
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    retval = rtree_capath_vals(context, client, server, &mut capvals);
    if retval != 0 { return retval }
    /* A capaths value of "." means no intermediates. */
    if !capvals.is_null() &&
           !(*capvals.offset(0 as libc::c_int as isize)).is_null() &&
           **capvals.offset(0 as libc::c_int as isize) as libc::c_int ==
               '.' as i32 {
        profile_free_list(capvals);
        capvals = 0 as *mut *mut libc::c_char
    }
    /* Count capaths (if any) and allocate space.  Leave room for the client
     * realm, server realm, and terminator. */
    i = 0 as libc::c_int as size_t;
    while !capvals.is_null() && !(*capvals.offset(i as isize)).is_null() {
        i = i.wrapping_add(1)
    }
    rpath =
        calloc(i.wrapping_add(3 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    if rpath.is_null() { return 12 as libc::c_int }
    /* Populate rpath with the client realm, capaths, and server realm. */
    retval =
        krb5int_copy_data_contents(context, client,
                                   &mut *rpath.offset(0 as libc::c_int as
                                                          isize));
    if !(retval != 0) {
        i = 0 as libc::c_int as size_t;
        loop  {
            if !(!capvals.is_null() &&
                     !(*capvals.offset(i as isize)).is_null()) {
                current_block = 5601891728916014340;
                break ;
            }
            d =
                make_data(*capvals.offset(i as isize) as *mut libc::c_void,
                          strcspn(*capvals.offset(i as isize),
                                  b"\t \x00" as *const u8 as
                                      *const libc::c_char) as libc::c_uint);
            retval =
                krb5int_copy_data_contents(context, &mut d,
                                           &mut *rpath.offset(i.wrapping_add(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                                                  as isize));
            if retval != 0 { current_block = 14255403184164566457; break ; }
            i = i.wrapping_add(1)
        }
        match current_block {
            14255403184164566457 => { }
            _ => {
                retval =
                    krb5int_copy_data_contents(context, server,
                                               &mut *rpath.offset(i.wrapping_add(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong)
                                                                      as
                                                                      isize));
                if !(retval != 0) {
                    /* Terminate rpath and return it. */
                    *rpath.offset(i.wrapping_add(2 as libc::c_int as
                                                     libc::c_ulong) as isize)
                        = empty_data();
                    *rpath_out = rpath;
                    rpath = 0 as *mut krb5_data
                }
            }
        }
    }
    profile_free_list(capvals);
    krb5int_free_data_list(context, rpath);
    return retval;
}
/* ANL - Modified to allow Configurable Authentication Paths.
 * This modification removes the restriction on the choice of realm
 * names, i.e. they nolonger have to be hierarchical. This
 * is allowed by RFC 1510: "If a hierarchical orginization is not used
 * it may be necessary to consult some database in order to construct
 * an authentication path between realms."  The database is contained
 * in the [capaths] section of the krb5.conf file.
 * Client to server paths are defined. There are n**2 possible
 * entries, but only those entries which are needed by the client
 * or server need be present in its krb5.conf file. (n entries or 2*n
 * entries if the same krb5.conf is used for clients and servers)
 *
 * for example: ESnet will be running a KDC which will share
 * inter-realm keys with its many orginizations which include among
 * other ANL, NERSC and PNL. Each of these orginizations wants to
 * use its DNS name in the realm, ANL.GOV. In addition ANL wants
 * to authenticatite to HAL.COM via a K5.MOON and K5.JUPITER
 * A [capaths] section of the krb5.conf file for the ANL.GOV clients
 * and servers would look like:
 *
 * [capaths]
 * ANL.GOV = {
 *              NERSC.GOV = ES.NET
 *              PNL.GOV = ES.NET
 *              ES.NET = .
 *              HAL.COM = K5.MOON
 *              HAL.COM = K5.JUPITER
 * }
 * NERSC.GOV = {
 *              ANL.GOV = ES.NET
 * }
 * PNL.GOV = {
 *              ANL.GOV = ES.NET
 * }
 * ES.NET = {
 *              ANL.GOV = .
 * }
 * HAL.COM = {
 *              ANL.GOV = K5.JUPITER
 *              ANL.GOV = K5.MOON
 * }
 *
 * In the above a "." is used to mean directly connected since the
 * the profile routines cannot handle a null entry.
 *
 * If no client-to-server path is found, the default hierarchical path
 * is still generated.
 *
 * This version of the Configurable Authentication Path modification
 * differs from the previous versions prior to K5 beta 5 in that
 * the profile routines are used, and the explicite path from
 * client's realm to server's realm must be given. The modifications
 * will work together.
 * DEE - 5/23/95
 */
/*
 * Build a tree given a set of profile values retrieved by
 * walk_rtree_capath_vals().
 */
#[c2rust::src_loc = "234:1"]
unsafe extern "C" fn rtree_capath_tree(mut context: krb5_context,
                                       mut client: *const krb5_data,
                                       mut server: *const krb5_data,
                                       mut vals: *mut *mut libc::c_char,
                                       mut rettree: *mut *mut krb5_principal)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut nvals: libc::c_uint = 0;
    let mut nlinks: libc::c_uint = 0;
    let mut nprincs: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut srcrealm: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut dstrealm: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut tree: *mut krb5_principal = 0 as *mut krb5_principal;
    let mut pprinc: *mut krb5_principal = 0 as *mut krb5_principal;
    *rettree = 0 as *mut krb5_principal;
    pprinc = 0 as *mut krb5_principal;
    tree = pprinc;
    nvals = 0 as libc::c_int as libc::c_uint;
    while !(*vals.offset(nvals as isize)).is_null() {
        nvals = nvals.wrapping_add(1)
    }
    if !(*vals.offset(0 as libc::c_int as isize)).is_null() &&
           **vals.offset(0 as libc::c_int as isize) as libc::c_int ==
               '.' as i32 {
        nlinks = 0 as libc::c_int as libc::c_uint
    } else { nlinks = nvals }
    nprincs = nlinks.wrapping_add(2 as libc::c_int as libc::c_uint);
    tree =
        calloc(nprincs.wrapping_add(1 as libc::c_int as libc::c_uint) as
                   libc::c_ulong,
               ::std::mem::size_of::<krb5_principal>() as libc::c_ulong) as
            *mut krb5_principal;
    if tree.is_null() {
        retval = 12 as libc::c_int
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while i < nprincs.wrapping_add(1 as libc::c_int as libc::c_uint) {
            let ref mut fresh0 = *tree.offset(i as isize);
            *fresh0 = 0 as krb5_principal;
            i = i.wrapping_add(1)
        }
        /* Invariant: PPRINC points one past end of list. */
        pprinc =
            &mut *tree.offset(0 as libc::c_int as isize) as
                *mut krb5_principal;
        /* Local TGS name */
        let fresh1 = pprinc;
        pprinc = pprinc.offset(1);
        retval = krb5int_tgtname(context, client, client, fresh1);
        if !(retval != 0) {
            srcrealm = *client;
            i = 0 as libc::c_int as libc::c_uint;
            loop  {
                if !(i < nlinks) {
                    current_block = 4761528863920922185;
                    break ;
                }
                dstrealm.data = *vals.offset(i as isize);
                dstrealm.length =
                    strcspn(*vals.offset(i as isize),
                            b"\t \x00" as *const u8 as *const libc::c_char) as
                        libc::c_uint;
                let fresh2 = pprinc;
                pprinc = pprinc.offset(1);
                retval =
                    krb5int_tgtname(context, &mut dstrealm, &mut srcrealm,
                                    fresh2);
                if retval != 0 {
                    current_block = 15639821522674154916;
                    break ;
                }
                srcrealm = dstrealm;
                i = i.wrapping_add(1)
            }
            match current_block {
                15639821522674154916 => { }
                _ => {
                    let fresh3 = pprinc;
                    pprinc = pprinc.offset(1);
                    retval =
                        krb5int_tgtname(context, server, &mut srcrealm,
                                        fresh3);
                    if !(retval != 0) { *rettree = tree }
                }
            }
        }
    }
    profile_free_list(vals);
    if retval != 0 {
        while !pprinc.is_null() &&
                  pprinc >
                      &mut *tree.offset(0 as libc::c_int as isize) as
                          *mut krb5_principal {
            /* krb5_free_principal() correctly handles null input */
            pprinc = pprinc.offset(-1);
            krb5_free_principal(context, *pprinc);
            *pprinc = 0 as krb5_principal
        }
        free(tree as *mut libc::c_void);
    }
    return retval;
}
/*
 * Get realm list from "capaths" section of the profile.  Deliberately
 * returns success but leaves VALS null if profile_get_values() fails
 * by not finding anything.
 */
#[c2rust::src_loc = "298:1"]
unsafe extern "C" fn rtree_capath_vals(mut context: krb5_context,
                                       mut client: *const krb5_data,
                                       mut server: *const krb5_data,
                                       mut vals: *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    /* null-terminated realm names */
    let mut clientz: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut serverz: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    *vals = 0 as *mut *mut libc::c_char;
    clientz =
        k5memdup0((*client).data as *const libc::c_void,
                  (*client).length as size_t, &mut retval) as
            *mut libc::c_char;
    if !clientz.is_null() {
        serverz =
            k5memdup0((*server).data as *const libc::c_void,
                      (*server).length as size_t, &mut retval) as
                *mut libc::c_char;
        if !serverz.is_null() {
            key[0 as libc::c_int as usize] =
                b"capaths\x00" as *const u8 as *const libc::c_char;
            key[1 as libc::c_int as usize] = clientz;
            key[2 as libc::c_int as usize] = serverz;
            key[3 as libc::c_int as usize] = 0 as *const libc::c_char;
            retval =
                profile_get_values((*context).profile, key.as_mut_ptr(), vals)
                    as krb5_error_code;
            match retval {
                -1429577726 | -1429577725 => {
                    /*
         * Not found; don't return an error.
         */
                    retval = 0 as libc::c_int
                }
                _ => { }
            }
        }
    }
    free(clientz as *mut libc::c_void);
    free(serverz as *mut libc::c_void);
    return retval;
}
/*
 * Build tree by hierarchical traversal.
 */
#[c2rust::src_loc = "344:1"]
unsafe extern "C" fn rtree_hier_tree(mut context: krb5_context,
                                     mut client: *const krb5_data,
                                     mut server: *const krb5_data,
                                     mut rettree: *mut *mut krb5_principal,
                                     mut sep: libc::c_int)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut realms: *mut krb5_data = 0 as *mut krb5_data;
    let mut dstrealm: *const krb5_data = 0 as *const krb5_data;
    let mut srcrealm: *const krb5_data = 0 as *const krb5_data;
    let mut tree: *mut krb5_principal = 0 as *mut krb5_principal;
    let mut pprinc: *mut krb5_principal = 0 as *mut krb5_principal;
    let mut nrealms: size_t = 0;
    let mut nprincs: size_t = 0;
    let mut i: size_t = 0;
    *rettree = 0 as *mut krb5_principal;
    retval =
        rtree_hier_realms(context, client, server, &mut realms, &mut nrealms,
                          sep);
    if retval != 0 { return retval }
    nprincs = nrealms;
    tree =
        calloc(nprincs.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<krb5_principal>() as libc::c_ulong) as
            *mut krb5_principal;
    pprinc = tree;
    if tree.is_null() {
        retval = 12 as libc::c_int
    } else {
        i = 0 as libc::c_int as size_t;
        while i < nrealms {
            let ref mut fresh4 = *tree.offset(i as isize);
            *fresh4 = 0 as krb5_principal;
            i = i.wrapping_add(1)
        }
        srcrealm = client;
        i = 0 as libc::c_int as size_t;
        loop  {
            if !(i < nrealms) { current_block = 26972500619410423; break ; }
            dstrealm = &mut *realms.offset(i as isize) as *mut krb5_data;
            let fresh5 = pprinc;
            pprinc = pprinc.offset(1);
            retval = krb5int_tgtname(context, dstrealm, srcrealm, fresh5);
            if retval != 0 { current_block = 17828502359114684314; break ; }
            srcrealm = dstrealm;
            i = i.wrapping_add(1)
        }
        match current_block {
            17828502359114684314 => { }
            _ => {
                *rettree = tree;
                free_realmlist(context, realms, nrealms);
                return 0 as libc::c_int
            }
        }
    }
    while !pprinc.is_null() && pprinc > tree {
        pprinc = pprinc.offset(-1);
        krb5_free_principal(context, *pprinc);
        *pprinc = 0 as krb5_principal
    }
    free_realmlist(context, realms, nrealms);
    free(tree as *mut libc::c_void);
    return retval;
}
/*
 * Construct list of realms between client and server.
 */
#[c2rust::src_loc = "393:1"]
unsafe extern "C" fn rtree_hier_realms(mut context: krb5_context,
                                       mut client: *const krb5_data,
                                       mut server: *const krb5_data,
                                       mut realms: *mut *mut krb5_data,
                                       mut nrealms: *mut size_t,
                                       mut sep: libc::c_int)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut c: hstate =
        hstate{str_0: 0 as *mut libc::c_char,
               len: 0,
               tail: 0 as *mut libc::c_char,
               dot: 0 as *mut libc::c_char,};
    let mut s: hstate =
        hstate{str_0: 0 as *mut libc::c_char,
               len: 0,
               tail: 0 as *mut libc::c_char,
               dot: 0 as *mut libc::c_char,};
    let mut ctweens: *mut krb5_data = 0 as *mut krb5_data;
    let mut stweens: *mut krb5_data = 0 as *mut krb5_data;
    let mut twp: *mut krb5_data = 0 as *mut krb5_data;
    let mut r: *mut krb5_data = 0 as *mut krb5_data;
    let mut rp: *mut krb5_data = 0 as *mut krb5_data;
    let mut nctween: size_t = 0;
    let mut nstween: size_t = 0;
    *realms = 0 as *mut krb5_data;
    *nrealms = 0 as libc::c_int as size_t;
    rp = 0 as *mut krb5_data;
    r = rp;
    c.str_0 = (*client).data;
    c.len = (*client).length as size_t;
    c.tail = 0 as *mut libc::c_char;
    c.dot = c.tail;
    s.str_0 = (*server).data;
    s.len = (*server).length as size_t;
    s.tail = 0 as *mut libc::c_char;
    s.dot = s.tail;
    comtail(&mut c, &mut s, sep);
    adjtail(&mut c, &mut s, sep);
    retval =
        rtree_hier_tweens(context, &mut c, &mut ctweens, &mut nctween,
                          1 as libc::c_int, sep);
    if !(retval != 0) {
        retval =
            rtree_hier_tweens(context, &mut s, &mut stweens, &mut nstween,
                              0 as libc::c_int, sep);
        if !(retval != 0) {
            r =
                calloc(nctween.wrapping_add(nstween),
                       ::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
                    *mut krb5_data;
            rp = r;
            if r.is_null() {
                retval = 12 as libc::c_int
            } else {
                /* Copy client realm "tweens" forward. */
                twp = ctweens;
                loop  {
                    if !(twp <
                             &mut *ctweens.offset(nctween as isize) as
                                 *mut krb5_data) {
                        current_block = 13472856163611868459;
                        break ;
                    }
                    retval = krb5int_copy_data_contents(context, twp, rp);
                    if retval != 0 {
                        current_block = 1243955722683473459;
                        break ;
                    }
                    rp = rp.offset(1);
                    twp = twp.offset(1)
                }
                match current_block {
                    1243955722683473459 => { }
                    _ =>
                    /* Copy server realm "tweens" backward. */
                    {
                        twp =
                            &mut *stweens.offset(nstween as isize) as
                                *mut krb5_data;
                        loop  {
                            let fresh6 = twp;
                            twp = twp.offset(-1);
                            if !(fresh6 > stweens) { break ; }
                            retval =
                                krb5int_copy_data_contents(context, twp, rp);
                            if retval != 0 { break ; }
                            rp = rp.offset(1)
                        }
                    }
                }
            }
        }
    }
    free(ctweens as *mut libc::c_void);
    free(stweens as *mut libc::c_void);
    if retval != 0 {
        free_realmlist(context, r,
                       rp.wrapping_offset_from(r) as libc::c_long as size_t);
        return retval
    }
    *realms = r;
    *nrealms = rp.wrapping_offset_from(r) as libc::c_long as size_t;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "454:1"]
unsafe extern "C" fn free_realmlist(mut context: krb5_context,
                                    mut realms: *mut krb5_data,
                                    mut nrealms: size_t) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < nrealms {
        krb5_free_data_contents(context, &mut *realms.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free(realms as *mut libc::c_void);
}
/*
 * Build a list of realms between a given realm and the common
 * suffix.  The original realm is included, but the "tail" is only
 * included if DOTAIL is true.
 *
 * Warning: This function intentionally aliases memory.  Caller must
 * make copies as needed and not call krb5_free_data_contents, etc.
 */
#[c2rust::src_loc = "474:1"]
unsafe extern "C" fn rtree_hier_tweens(mut context: krb5_context,
                                       mut realm: *mut hstate,
                                       mut tweens: *mut *mut krb5_data,
                                       mut ntweens: *mut size_t,
                                       mut dotail: libc::c_int,
                                       mut sep: libc::c_int)
 -> krb5_error_code {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rtail: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rlen: size_t = 0;
    let mut n: size_t = 0;
    let mut tws: *mut krb5_data = 0 as *mut krb5_data;
    let mut ntws: *mut krb5_data = 0 as *mut krb5_data;
    r = (*realm).str_0;
    rlen = (*realm).len;
    rtail = (*realm).tail;
    tws = 0 as *mut krb5_data;
    ntws = tws;
    *tweens = ntws;
    n = 0 as libc::c_int as size_t;
    *ntweens = n;
    p = r;
    lp = p;
    while p < &mut *r.offset(rlen as isize) as *mut libc::c_char {
        if !(*p as libc::c_int != sep &&
                 &mut *p.offset(1 as libc::c_int as isize) as
                     *mut libc::c_char !=
                     &mut *r.offset(rlen as isize) as *mut libc::c_char) {
            if lp == rtail && dotail == 0 { break ; }
            ntws =
                realloc(tws as *mut libc::c_void,
                        n.wrapping_add(1 as libc::c_int as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_data>()
                                                                           as
                                                                           libc::c_ulong))
                    as *mut krb5_data;
            if ntws.is_null() {
                free(tws as *mut libc::c_void);
                return 12 as libc::c_int
            }
            tws = ntws;
            let ref mut fresh7 = (*tws.offset(n as isize)).data;
            *fresh7 = lp;
            (*tws.offset(n as isize)).length =
                (&mut *r.offset(rlen as isize) as
                     *mut libc::c_char).wrapping_offset_from(lp) as
                    libc::c_long as libc::c_uint;
            n = n.wrapping_add(1);
            if lp == rtail { break ; }
            lp =
                &mut *p.offset(1 as libc::c_int as isize) as *mut libc::c_char
        }
        p = p.offset(1)
    }
    *tweens = tws;
    *ntweens = n;
    return 0 as libc::c_int;
}
/*
 * Adjust suffixes that each starts at the beginning of a component,
 * to avoid the problem where "BC.EXAMPLE.COM" is erroneously reported
 * as a parent of "ABC.EXAMPLE.COM".
 */
#[c2rust::src_loc = "520:1"]
unsafe extern "C" fn adjtail(mut c: *mut hstate, mut s: *mut hstate,
                             mut sep: libc::c_int) {
    let mut cfull: libc::c_int = 0;
    let mut sfull: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = (*c).tail;
    sp = (*s).tail;
    if cp.is_null() || sp.is_null() { return }
    /*
     * Is it a full component?  Yes, if it's the beginning of the
     * string or there's a separator to the left.
     *
     * The index of -1 is valid because it only gets evaluated if the
     * pointer is not at the beginning of the string.
     */
    cfull =
        (cp == (*c).str_0 ||
             *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int == sep)
            as libc::c_int;
    sfull =
        (sp == (*s).str_0 ||
             *sp.offset(-(1 as libc::c_int) as isize) as libc::c_int == sep)
            as libc::c_int;
    /*
     * If they're both full components, we're done.
     */
    if cfull != 0 && sfull != 0 {
        return
    } else {
        if !(*c).dot.is_null() && !(*s).dot.is_null() {
            cp = (*c).dot.offset(1 as libc::c_int as isize);
            sp = (*s).dot.offset(1 as libc::c_int as isize);
            /*
         * Out of bounds? Can only happen if there are trailing dots.
         */
            if cp >=
                   &mut *(*c).str_0.offset((*c).len as isize) as
                       *mut libc::c_char ||
                   sp >=
                       &mut *(*s).str_0.offset((*s).len as isize) as
                           *mut libc::c_char {
                sp = 0 as *mut libc::c_char;
                cp = sp
            }
        } else { sp = 0 as *mut libc::c_char; cp = sp }
    }
    (*c).tail = cp;
    (*s).tail = sp;
}
/*
 * Find common suffix of C and S.
 *
 * C->TAIL and S->TAIL will point to the respective suffixes.  C->DOT
 * and S->DOT will point to the nearest instances of SEP to the right
 * of the start of each suffix.  Caller must initialize TAIL and DOT
 * pointers to null.
 */
#[c2rust::src_loc = "568:1"]
unsafe extern "C" fn comtail(mut c: *mut hstate, mut s: *mut hstate,
                             mut sep: libc::c_int) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cdot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sdot: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*c).len == 0 as libc::c_int as libc::c_ulong ||
           (*s).len == 0 as libc::c_int as libc::c_ulong {
        return
    }
    sdot = 0 as *mut libc::c_char;
    cdot = sdot;
    /*
     * ANSI/ISO C allows a pointer one past the end but not one
     * before the beginning of an array.
     */
    cp = &mut *(*c).str_0.offset((*c).len as isize) as *mut libc::c_char;
    sp = &mut *(*s).str_0.offset((*s).len as isize) as *mut libc::c_char;
    /*
     * Set CP and SP to point to the common suffix of each string.
     * When we run into separators (dots, unless someone has a X.500
     * style realm), keep pointers to the latest pair.
     */
    while cp > (*c).str_0 && sp > (*s).str_0 {
        cp = cp.offset(-1);
        sp = sp.offset(-1);
        if *cp as libc::c_int != *sp as libc::c_int {
            /*
             * Didn't match, so most recent match is one byte to the
             * right (or not at all).
             */
            cp = cp.offset(1);
            sp = sp.offset(1);
            break ;
        } else if *cp as libc::c_int == sep { cdot = cp; sdot = sp }
    }
    /*
         * Keep track of matching dots.
         */
    /* No match found at all. */
    if cp == &mut *(*c).str_0.offset((*c).len as isize) as *mut libc::c_char {
        return
    }
    (*c).tail = cp;
    (*s).tail = sp;
    (*c).dot = cdot;
    (*s).dot = sdot;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 1989,1990,1991,1992,1993,1994,1995,2000,2001,
 * 2003,2006,2007,2008,2009 by the Massachusetts Institute of Technology,
 * Cambridge, MA, USA.  All Rights Reserved.
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
 * This prototype for k5-int.h (Krb5 internals include file)
 * includes the user-visible definitions from krb5.h and then
 * includes other definitions that are not user-visible but are
 * required for compiling Kerberos internal routines.
 *
 * John Gilmore, Cygnus Support, Sat Jan 21 22:45:52 PST 1995
 */
/* KRB5_GENERAL__ */
/*
 * Begin "k5-config.h"
 */
/*
 * Machine-type definitions: PC Clone 386 running Microloss Windows
 */
/* From autoconf.h */
/* HAVE_SYS_TYPES_H */
/* HAVE_SYS_TYPES_H */
/* KRB5_SYSTYPES__ */
/* one day */
/* one week */
/* Thu Jan  1 00:00:00 2038 UTC */
/*
 * Windows requires a different api interface to each function. Here
 * just define it as NULL.
 */
/* #define KRB5_OLD_CRYPTO is done in krb5.h */
/* KRB5_CONFIG__ */
/*
 * End "k5-config.h"
 */
/*
 * After loading the configuration definitions, load the Kerberos definitions.
 */
/* Get mutex support; currently used only for the replay cache.  */
/* Get error info support.  */
/* Get string buffer support. */
/* Define tracing macros. */
/* Profile variables.  Constants are named KRB5_CONF_STRING, where STRING
 * matches the variable name.  Keep these alphabetized. */
/* Cache configuration variables */
/* Error codes used in KRB_ERROR protocol messages.
   Return values of library routines are based on a different error table
   (which allows non-ambiguous error codes between subsystems) */
/* KDC errors */
/* No error */
/* Client's entry in DB expired */
/* Server's entry in DB expired */
/* Requested pvno not supported */
/* C's key encrypted in old master */
/* S's key encrypted in old master */
/* Client not found in Kerberos DB */
/* Server not found in Kerberos DB */
/* Multiple entries in Kerberos DB */
/* The C or S has a null key */
/* Tkt ineligible for postdating */
/* Requested starttime > endtime */
/* KDC policy rejects request */
/* KDC can't do requested opt. */
/* No support for encryption type */
/* No support for checksum type */
/* No support for padata type */
/* No support for transited type */
/* C's creds have been revoked */
/* S's creds have been revoked */
/* TGT has been revoked */
/* C not yet valid */
/* S not yet valid */
/* Password has expired */
/* Preauthentication failed */
/* Additional preauthentication */
                                           /* required */
/* Requested server and */
                                           /* ticket don't match*/
/* Server principal valid for */
                                           /*   user2user only */
/* KDC policy rejected transited */
                                           /*   path */
/* A service is not
                                            * available that is
                                            * required to process the
                                            * request */
/* Application errors */
/* Decrypt integrity check failed */
/* Ticket expired */
/* Ticket not yet valid */
/* Request is a replay */
/* The ticket isn't for us */
/* Ticket/authenticator don't match */
/* Clock skew too great */
/* Incorrect net address */
/* Protocol version mismatch */
/* Invalid message type */
/* Message stream modified */
/* Message out of order */
/* Key version is not available */
/* Service key not available */
/* Mutual authentication failed */
/* Incorrect message direction */
/* Alternative authentication */
                                        /* method required */
/* Incorrect sequence numnber */
                                        /* in message */
/* Inappropriate type of */
                                        /* checksum in message */
/* Policy rejects transited path */
/* Response too big for UDP, */
                                        /*   retry with TCP */
/* other errors */
/* Generic error (description */
                                        /* in e-text) */
/* Field is too long for impl. */
/* PKINIT server-reported errors */
/* client cert not trusted */
/* client signature verify failed */
/* invalid Diffie-Hellman parameters */
/* client cert not verifiable to */
                                                   /* trusted root cert */
/* client cert had invalid signature */
/* client cert was revoked */
/* client cert revoked, reason unknown */
/* mismatch between client cert and */
                                                   /* principal name */
/* bad extended key use */
/* bad digest algorithm in client cert */
/* missing paChecksum in PA-PK-AS-REQ */
/* bad digest algorithm in SignedData */
/* The IAKERB proxy could
                                                      not find a KDC */
/* The KDC did not respond
                                                      to the IAKERB proxy */
/* RFC 6113 */
/* RFC 6113 */
/* err table base max offset for protocol err codes */
/*
 * A null-terminated array of this structure is returned by the KDC as
 * the data part of the ETYPE_INFO preauth type.  It informs the
 * client which encryption types are supported.
 * The  same data structure is used by both etype-info and etype-info2
 * but s2kparams must be null when encoding etype-info.
 */
/*
 *  This is essentially -1 without sign extension which can screw up
 *  comparisons on 64 bit machines. If the length is this value, then
 *  the salt data is not present. This is to distinguish between not
 *  being set and being of 0 length.
 */
/* RFC 4537 */
/* sam_type values -- informational only */
/*  Enigma Logic */
/*  Digital Pathways */
/*  S/key where  KDC has key 0 */
/*  Traditional S/Key */
/*  Security Dynamics */
/*  CRYPTOCard */
/* XXX need to figure out who has which numbers assigned */
/*  ActivCard decimal mode */
/*  ActivCard hex mode */
/*  Digital Pathways hex mode */
/* experimental */
/* testing */
/* special */
/* Array of checksums */
/* information */
/* KRB5_SAM_* values */
/* informational */
/* KRB5_SAM_* values */
/* copied */
/* krb5_enc_sam_response_enc */
/*
 * Keep the pkinit definitions in a separate file so that the plugin
 * only has to include k5-int-pkinit.h rather than k5-int.h
 */
/* -1 for unspecified */
/* -1 for unspecified */
/* -1 for unspecified */
/* -1 for unspecified */
/* -1 for unspecified */
/* Plain text of an encrypted PA-FX-COOKIE value produced by the KDC. */
/* In PAC options, indicates Resource-Based Constrained Delegation support. */
/* struct stat, stat() */
/* MAXPATHLEN */
/* prototypes for file-related
                                           syscalls; flags for open &
                                           friends */
/* libos.spec */
/* Internal structure of an opaque key identifier */
/*
     * Cache of data private to the cipher implementation, which we
     * don't want to have to recompute for every operation.  This may
     * include key schedules, iteration counts, etc.
     *
     * The cipher implementation is responsible for setting this up
     * whenever needed, and the enc_provider key_cleanup method must
     * then be provided to dispose of it.
     */
/* Write the SHA-256 hash of in (containing n elements) to out. */
/* Convenience function: zap and free ptr if it is non-NULL. */
/* Convenience function: zap and free zero-terminated str if it is non-NULL. */
/* Convenience function: zap and free krb5_data pointer if it is non-NULL. */
/*
 * End "los-proto.h"
 */
/*
 * Flags for the os_flags field
 *
 * KRB5_OS_TOFFSET_VALID means that the time offset fields are valid.
 * The intention is that this facility to correct the system clocks so
 * that they reflect the "real" time, for systems where for some
 * reason we can't set the system clock.  Instead we calculate the
 * offset between the system time and real time, and store the offset
 * in the os context so that we can correct the system clock as necessary.
 *
 * KRB5_OS_TOFFSET_TIME means that the time offset fields should be
 * returned as the time by the krb5 time routines.  This should only
 * be used for testing purposes (obviously!)
 */
/* lock mode flags */
/*
 * Begin "preauth.h"
 *
 * (Originally written by Glen Machin at Sandia Labs.)
 */
/*
 * Sandia National Laboratories also makes no representations about the
 * suitability of the modifications, or additions to this software for
 * any purpose.  It is provided "as is" without express or implied warranty.
 */
/* check logon hour restrictions */
/* sign with usage 27 instead of 26 */
/* padata from req_body is used*/
/* Bits 0-15 are critical in FAST options (RFC 6113 section 7.3). */
/*
 * AD-CAMMAC's other-verifiers field is a sequence of Verifier, which is an
 * extensible choice with only one selection, Verifier-MAC.  For the time being
 * we will represent this field directly as an array of krb5_verifier_mac.
 * That will have to change if other selections are added.
 */
/* Does not return a copy; original padata sequence responsible for freeing*/
/* Allocate a pa-data object with uninitialized contents of size len.  If len
 * is 0, set the contents field to NULL. */
/* Free a single pa-data object. */
/* Without copying, add single element *pa to *list, reallocating as necessary.
 * If *list is NULL, allocate a new list.  Set *pa to NULL on success. */
/* Without copying, add a pa-data element of type pa_type to *list with the
 * contents in data.  Set *data to empty_data() on success. */
/* Add an empty pa-data element of type pa_type to *list. */
/* KRB5_PREAUTH__ */
/*
 * End "preauth.h"
 */
/* #include "krb5/wordsize.h" -- comes in through base-defs.h. */
/* ** Plugin framework ***/
/*
 * This framework can be used to create pluggable interfaces.  Not all existing
 * pluggable interface use this framework, but new ones should.  A new
 * pluggable interface entails:
 *
 * - An interface ID definition in the list of #defines below.
 *
 * - A name in the interface_names array in lib/krb5/krb/plugins.c.
 *
 * - An installed public header file in include/krb5.  The public header should
 *   include <krb5/plugin.h> and should declare a vtable structure for each
 *   supported major version of the interface.
 *
 * - A consumer API implementation, located within the code unit which makes
 *   use of the pluggable interface.  The consumer API should consist of:
 *
 *   . An interface-specific handle type which contains a vtable structure for
 *     the module (or a union of several such structures, if there are multiple
 *     supported major versions) and, optionally, resource data bound to the
 *     handle.
 *
 *   . An interface-specific loader function which creates a handle or list of
 *     handles.  A list of handles would be created if the interface is a
 *     one-to-many interface where the consumer wants to consult all available
 *     modules; a single handle would be created for an interface where the
 *     consumer wants to consult a specific module.  The loader function should
 *     use k5_plugin_load or k5_plugin_load_all to produce one or a list of
 *     vtable initializer functions, and should use those functions to fill in
 *     the vtable structure for the module (if necessary, trying each supported
 *     major version starting from the most recent).  The loader function can
 *     also bind resource data into the handle based on caller arguments, if
 *     appropriate.
 *
 *   . For each plugin method, a wrapper function which accepts a krb5_context,
 *     a plugin handle, and the method arguments.  Wrapper functions should
 *     invoke the method function contained in the handle's vtable.
 *
 * - Possibly, built-in implementations of the interface, also located within
 *   the code unit which makes use of the interface.  Built-in implementations
 *   must be registered with k5_plugin_register before the first call to
 *   k5_plugin_load or k5_plugin_load_all.
 *
 * A pluggable interface should have one or more currently supported major
 * versions, starting at 1.  Each major version should have a current minor
 * version, also starting at 1.  If new methods are added to a vtable, the
 * minor version should be incremented and the vtable stucture should document
 * where each minor vtable version ends.  If method signatures for a vtable are
 * changed, the major version should be incremented.
 *
 * Plugin module implementations (either built-in or dynamically loaded) should
 * define a function named <interfacename>_<modulename>_initvt, matching the
 * signature of krb5_plugin_initvt_fn as declared in include/krb5/plugin.h.
 * The initvt function should check the given maj_ver argument against its own
 * supported major versions, cast the vtable pointer to the appropriate
 * interface-specific vtable type, and fill in the vtable methods, stopping as
 * appropriate for the given min_ver.  Memory for the vtable structure is
 * allocated by the caller, not by the module.
 *
 * Dynamic plugin modules are registered with the framework through the
 * [plugins] section of the profile, as described in the admin documentation
 * and krb5.conf man page.
 */
/* Holds krb5_context information about each pluggable interface. */
/* A list of plugin interface IDs.  Make sure to increment
 * PLUGIN_NUM_INTERFACES when a new interface is added, and add an entry to the
 * interface_names table in lib/krb5/krb/plugin.c. */
/* Retrieve the plugin module of type interface_id and name modname,
 * storing the result into module. */
/* Retrieve all plugin modules of type interface_id, storing the result
 * into modules.  Free the result with k5_plugin_free_handles. */
/* Release a module list allocated by k5_plugin_load_all. */
/* Register a plugin module of type interface_id and name modname. */
/*
 * Register a plugin module which is part of the krb5 tree but is built as a
 * dynamic plugin.  Look for the module in modsubdir relative to the
 * context->base_plugin_dir.
 */
/* Destroy the module state within context; used by krb5_free_context. */
/* private, in kdb5.h */
/* allowable clock skew */
/* Message size above which we'll try TCP first in send-to-kdc
       type code.  Aside from the 2**16 size limit, we put no
       absolute limit on the UDP packet size.  */
/* Use the config-file ktypes instead of app-specified?  */
/* locate_kdc module stuff */
/* preauth module stuff */
/* cache module stuff */
/* localauth module stuff */
/* hostrealm module stuff */
/* TLS module vtable (if loaded) */
/* error detail info */
/* For Sun iprop code; does this really have to be here?  */
/* could be used in a table to find an etype and initialize a block */
/* internal message representations */
/* user data */
/* client time, optional */
/* microsecond portion of time,
                                           optional */
/* sequence #, optional */
/* sender address */
/* recipient address, optional */
/* data integrity checksum */
/* encrypted part */
/* user data */
/* client time, optional */
/* microsecond portion of time, opt. */
/* sequence #, optional */
/* sender address */
/* recipient address, optional */
/*
 * Begin "asn1.h"
 */
/* ASN.1 encoding knowledge; KEEP IN SYNC WITH ASN.1 defs! */
/* here we use some knowledge of ASN.1 encodings */
/*
  Ticket is APPLICATION 1.
  Authenticator is APPLICATION 2.
  AS_REQ is APPLICATION 10.
  AS_REP is APPLICATION 11.
  TGS_REQ is APPLICATION 12.
  TGS_REP is APPLICATION 13.
  AP_REQ is APPLICATION 14.
  AP_REP is APPLICATION 15.
  KRB_SAFE is APPLICATION 20.
  KRB_PRIV is APPLICATION 21.
  KRB_CRED is APPLICATION 22.
  EncASRepPart is APPLICATION 25.
  EncTGSRepPart is APPLICATION 26.
  EncAPRepPart is APPLICATION 27.
  EncKrbPrivPart is APPLICATION 28.
  EncKrbCredPart is APPLICATION 29.
  KRB_ERROR is APPLICATION 30.
*/
/* allow either constructed or primitive encoding, so check for bit 6
   set or reset */
/* ************************************************************************
 * Prototypes for krb5_encode.c
 *************************************************************************/
/*
  krb5_error_code encode_krb5_structure(const krb5_structure *rep,
  krb5_data **code);
  modifies  *code
  effects   Returns the ASN.1 encoding of *rep in **code.
  Returns ASN1_MISSING_FIELD if a required field is emtpy in *rep.
  Returns ENOMEM if memory runs out.
*/
/* yes, the translation is identical to that used for KDC__REP */
/* yes, the translation is identical to that used for KDC__REP */
/* ************************************************************************
 * End of prototypes for krb5_encode.c
 *************************************************************************/
/* ************************************************************************
 * Prototypes for krb5_decode.c
 *************************************************************************/
/*
  krb5_error_code decode_krb5_structure(const krb5_data *code,
  krb5_structure **rep);

  requires  Expects **rep to not have been allocated;
  a new *rep is allocated regardless of the old value.
  effects   Decodes *code into **rep.
  Returns ENOMEM if memory is exhausted.
  Returns asn1 and krb5 errors.
*/
/* kdb.h */
/* Master key version number */
/* kvno of key_data elements (all the same) */
/* ************************************************************************
 * End of prototypes for krb5_decode.c
 *************************************************************************/
/* KRB5_ASN1__ */
/*
 * End "asn1.h"
 */
/*
 * Internal krb5 library routines
 */
/* Return true if s is non-empty and composed solely of digits. */
/*
 * Initialization routines.
 */
/* [De]serialize 4-byte integer */
/* [De]serialize 8-byte integer */
/* [De]serialize byte string */
/* Fill in the buffer with random alpha-numeric data. */
/* value to use when requesting a keytab entry and KVNO doesn't matter */
/* value to use when requesting a keytab entry and enctype doesn't matter */
/* To keep happy libraries which are (for now) accessing internal stuff */
/* Make sure to increment by one when changing the struct */
/* Used for KDB LDAP back end.  */
/*
     * pkinit asn.1 encode/decode functions
     */
/* Set *tag_out to the integrity tag of *enc.  (Does not allocate memory;
 * returned buffer is a subrange of *ctext.) */
/*
 * This structure was exposed and used in macros in krb5 1.2, so do not
 * change its ABI.
 */
/* routines always present */
/* routines to be included on extended version (write routines) */
/* Not sure it's ready for exposure just yet.  */
/*
 * Referral definitions and subfunctions.
 */
/* should move into k5-int.h */
/* chk_trans.c */
/* free_rtree.c */
#[no_mangle]
#[c2rust::src_loc = "615:1"]
pub unsafe extern "C" fn krb5_free_realm_tree(mut context: krb5_context,
                                              mut realms:
                                                  *mut krb5_principal) {
    let mut nrealms: *mut krb5_principal = realms;
    if realms.is_null() { return }
    while !(*nrealms).is_null() {
        krb5_free_principal(context, *nrealms);
        nrealms = nrealms.offset(1)
    }
    free(realms as *mut libc::c_void);
}
