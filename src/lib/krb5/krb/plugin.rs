use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:27"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
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
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::plugin_mapping;
    use super::stddef_h::size_t;
    use super::string_h::memcpy;
    use super::stdlib_h::calloc;
    extern "C" {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:27"]
pub mod plugin_h {
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
    /*
 * krb5_plugin_initvt_fn is the type of all module initvt functions.  Based on
 * the maj_ver argument, the initvt function should cast vtable to the
 * appropriate type and then fill it in.  If a vtable has been expanded,
 * min_ver indicates which version of the vtable is being filled in.
 */
    #[c2rust::src_loc = "42:1"]
    pub type krb5_plugin_initvt_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: libc::c_int,
                                    _: libc::c_int, _: krb5_plugin_vtable)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "34:1"]
    pub type krb5_plugin_vtable = *mut krb5_plugin_vtable_st;
    use super::krb5_h::{krb5_error_code, krb5_context};
    extern "C" {
        #[c2rust::src_loc = "34:16"]
        pub type krb5_plugin_vtable_st;
    }
    /* KRB5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:27"]
pub mod k5_plugin_h {
    /* opaque */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct plugin_dir_handle {
        pub files: *mut *mut plugin_file_handle,
    }
    use super::k5_err_h::errinfo;
    extern "C" {
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
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn krb5int_open_plugin(_: *const libc::c_char,
                                   _: *mut *mut plugin_file_handle,
                                   _: *mut errinfo) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn krb5int_close_plugin(_: *mut plugin_file_handle);
        #[no_mangle]
        #[c2rust::src_loc = "98:1"]
        pub fn krb5int_get_plugin_func(_: *mut plugin_file_handle,
                                       _: *const libc::c_char,
                                       _:
                                           *mut Option<unsafe extern "C" fn()
                                                           -> ()>,
                                       _: *mut errinfo) -> libc::c_long;
    }
    /* K5_PLUGIN_H */
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:27"]
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
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:27"]
pub mod k5_platform_h {
    extern "C" {
        /*
 * Compose two path components, inserting the platform-appropriate path
 * separator if needed.  If path2 is an absolute path, path1 will be discarded
 * and path_out will be a copy of path2.  Returns 0 on success or ENOMEM on
 * allocation failure.
 */
        #[no_mangle]
        #[c2rust::src_loc = "1074:1"]
        pub fn k5_path_join(path1: *const libc::c_char,
                            path2: *const libc::c_char,
                            path_out: *mut *mut libc::c_char) -> libc::c_long;
    }
    /* K5_PLATFORM_H */
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
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:27"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
pub use self::types_h::__int32_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_context, krb5_post_recv_fn,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _profile_t,
                       krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, k5memdup0, k5alloc, k5calloc,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::plugin_h::{krb5_plugin_initvt_fn, krb5_plugin_vtable,
                         krb5_plugin_vtable_st};
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle,
                            krb5int_open_plugin, krb5int_close_plugin,
                            krb5int_get_plugin_func};
pub use self::k5_err_h::errinfo;
pub use self::profile_h::{profile_t, profile_get_values, profile_free_list};
use self::stdio_h::asprintf;
use self::k5_platform_h::k5_path_join;
use self::libintl_h::dgettext;
use self::string_h::{strlen, strchr, strcmp, memset, memcpy};
use self::assert_h::__assert_fail;
use self::stdlib_h::{free, realloc, calloc};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/plugin.c - Plugin framework functions */
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
/*
 * A plugin_mapping structure maps a module name to a built-in or dynamic
 * module.  modname is always present; the other three fields can be in four
 * different states:
 *
 * - If dyn_path and dyn_handle are null but module is set, the mapping is to a
 *   built-in module.
 * - If dyn_path is set but dyn_handle and module are null, the mapping is to a
 *   dynamic module which hasn't been loaded yet.
 * - If all three fields are set, the mapping is to a dynamic module which has
 *   been loaded and is ready to use.
 * - If all three fields are null, the mapping is to a dynamic module which
 *   failed to load and should be ignored.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "43:8"]
pub struct plugin_mapping {
    pub modname: *mut libc::c_char,
    pub dyn_path: *mut libc::c_char,
    pub dyn_handle: *mut plugin_file_handle,
    pub module: krb5_plugin_initvt_fn,
}
#[no_mangle]
#[c2rust::src_loc = "50:13"]
pub static mut interface_names: [*const libc::c_char; 13] =
    [b"pwqual\x00" as *const u8 as *const libc::c_char,
     b"kadm5_hook\x00" as *const u8 as *const libc::c_char,
     b"clpreauth\x00" as *const u8 as *const libc::c_char,
     b"kdcpreauth\x00" as *const u8 as *const libc::c_char,
     b"ccselect\x00" as *const u8 as *const libc::c_char,
     b"localauth\x00" as *const u8 as *const libc::c_char,
     b"hostrealm\x00" as *const u8 as *const libc::c_char,
     b"audit\x00" as *const u8 as *const libc::c_char,
     b"tls\x00" as *const u8 as *const libc::c_char,
     b"kdcauthdata\x00" as *const u8 as *const libc::c_char,
     b"certauth\x00" as *const u8 as *const libc::c_char,
     b"kadm5_auth\x00" as *const u8 as *const libc::c_char,
     b"kdcpolicy\x00" as *const u8 as *const libc::c_char];
/* Return the context's interface structure for id, or NULL if invalid. */
#[inline]
#[c2rust::src_loc = "67:1"]
unsafe extern "C" fn get_interface(mut context: krb5_context,
                                   mut id: libc::c_int)
 -> *mut plugin_interface {
    if context.is_null() || id < 0 as libc::c_int || id >= 13 as libc::c_int {
        return 0 as *mut plugin_interface
    }
    return &mut *(*context).plugins.as_mut_ptr().offset(id as isize) as
               *mut plugin_interface;
}
/* Release the memory associated with the mapping list entry map. */
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn free_plugin_mapping(mut map: *mut plugin_mapping) {
    if map.is_null() { return }
    free((*map).modname as *mut libc::c_void);
    free((*map).dyn_path as *mut libc::c_void);
    if !(*map).dyn_handle.is_null() {
        krb5int_close_plugin((*map).dyn_handle);
    }
    free(map as *mut libc::c_void);
}
#[c2rust::src_loc = "88:1"]
unsafe extern "C" fn free_mapping_list(mut list: *mut *mut plugin_mapping) {
    let mut mp: *mut *mut plugin_mapping = 0 as *mut *mut plugin_mapping;
    mp = list;
    while !mp.is_null() && !(*mp).is_null() {
        free_plugin_mapping(*mp);
        mp = mp.offset(1)
    }
    free(list as *mut libc::c_void);
}
/* Construct a plugin mapping object.  path may be NULL (for a built-in
 * module), or may be relative to the plugin base directory. */
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn make_plugin_mapping(mut context: krb5_context,
                                         mut name: *const libc::c_char,
                                         mut namelen: size_t,
                                         mut path: *const libc::c_char,
                                         mut module: krb5_plugin_initvt_fn,
                                         mut map_out:
                                             *mut *mut plugin_mapping)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut map: *mut plugin_mapping = 0 as *mut plugin_mapping;
    /* Create the mapping entry. */
    map =
        k5alloc(::std::mem::size_of::<plugin_mapping>() as libc::c_ulong,
                &mut ret) as *mut plugin_mapping;
    if map.is_null() { return ret }
    (*map).modname =
        k5memdup0(name as *const libc::c_void, namelen, &mut ret) as
            *mut libc::c_char;
    if !(*map).modname.is_null() {
        if !path.is_null() {
            if k5_path_join((*context).plugin_base_dir, path,
                            &mut (*map).dyn_path) != 0 {
                current_block = 17641196543962408034;
            } else { current_block = 14523784380283086299; }
        } else { current_block = 14523784380283086299; }
        match current_block {
            17641196543962408034 => { }
            _ => {
                (*map).module = module;
                *map_out = map;
                return 0 as libc::c_int
            }
        }
    }
    free_plugin_mapping(map);
    return 12 as libc::c_int;
}
/*
 * Register a mapping from modname to either dyn_path (for an auto-registered
 * dynamic module) or to module (for a builtin module).  dyn_path may be
 * relative to the plugin base directory.
 */
#[c2rust::src_loc = "134:1"]
unsafe extern "C" fn register_module(mut context: krb5_context,
                                     mut interface: *mut plugin_interface,
                                     mut modname: *const libc::c_char,
                                     mut dyn_path: *const libc::c_char,
                                     mut module: krb5_plugin_initvt_fn)
 -> krb5_error_code {
    let mut list: *mut *mut plugin_mapping = 0 as *mut *mut plugin_mapping;
    let mut count: size_t = 0;
    /* Allocate list space for another element and a terminator. */
    list = (*interface).modules;
    count = 0 as libc::c_int as size_t;
    while !list.is_null() && !(*list.offset(count as isize)).is_null() {
        count = count.wrapping_add(1)
    }
    list =
        realloc((*interface).modules as *mut libc::c_void,
                count.wrapping_add(2 as libc::c_int as
                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut plugin_mapping>()
                                                                       as
                                                                       libc::c_ulong))
            as *mut *mut plugin_mapping;
    if list.is_null() { return 12 as libc::c_int }
    let ref mut fresh0 =
        *list.offset(count.wrapping_add(1 as libc::c_int as libc::c_ulong) as
                         isize);
    *fresh0 = 0 as *mut plugin_mapping;
    let ref mut fresh1 = *list.offset(count as isize);
    *fresh1 = *fresh0;
    (*interface).modules = list;
    /* Create a new mapping structure and add it to the list. */
    return make_plugin_mapping(context, modname, strlen(modname), dyn_path,
                               module, &mut *list.offset(count as isize));
}
/* Parse a profile module string of the form "modname:modpath" into a mapping
 * entry. */
#[c2rust::src_loc = "158:1"]
unsafe extern "C" fn parse_modstr(mut context: krb5_context,
                                  mut modstr: *const libc::c_char,
                                  mut map_out: *mut *mut plugin_mapping)
 -> krb5_error_code {
    let mut sep: *const libc::c_char = 0 as *const libc::c_char;
    *map_out = 0 as *mut plugin_mapping;
    sep = strchr(modstr, ':' as i32);
    if sep.is_null() {
        krb5_set_error_message(context,
                               -(1750600191 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Invalid module specifier %s\x00" as
                                            *const u8 as *const libc::c_char),
                               modstr);
        return -(1750600191 as libc::c_long) as krb5_error_code
    }
    return make_plugin_mapping(context, modstr,
                               sep.wrapping_offset_from(modstr) as
                                   libc::c_long as size_t,
                               sep.offset(1 as libc::c_int as isize), None,
                               map_out);
}
/* Return true if value is found in list. */
#[c2rust::src_loc = "178:1"]
unsafe extern "C" fn find_in_list(mut list: *mut *mut libc::c_char,
                                  mut value: *const libc::c_char)
 -> krb5_boolean {
    while !(*list).is_null() {
        if strcmp(*list, value) == 0 as libc::c_int {
            return 1 as libc::c_int as krb5_boolean
        }
        list = list.offset(1)
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* Get the list of values for the profile variable varname in the section for
 * interface id, or NULL if no values are set. */
#[c2rust::src_loc = "190:1"]
unsafe extern "C" fn get_profile_var(mut context: krb5_context,
                                     mut id: libc::c_int,
                                     mut varname: *const libc::c_char,
                                     mut out: *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut path: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    *out = 0 as *mut *mut libc::c_char;
    path[0 as libc::c_int as usize] =
        b"plugins\x00" as *const u8 as *const libc::c_char;
    path[1 as libc::c_int as usize] = interface_names[id as usize];
    path[2 as libc::c_int as usize] = varname;
    path[3 as libc::c_int as usize] = 0 as *const libc::c_char;
    ret =
        profile_get_values((*context).profile, path.as_mut_ptr(), out) as
            krb5_error_code;
    return if ret as libc::c_long == -(1429577725 as libc::c_long) {
               0 as libc::c_int
           } else { ret };
}
/* Expand *list_inout to contain the mappings from modstrs, followed by the
 * existing built-in module mappings. */
#[c2rust::src_loc = "207:1"]
unsafe extern "C" fn make_full_list(mut context: krb5_context,
                                    mut modstrs: *mut *mut libc::c_char,
                                    mut list_inout:
                                        *mut *mut *mut plugin_mapping)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut count: size_t = 0;
    let mut pos: size_t = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut list: *mut *mut plugin_mapping = 0 as *mut *mut plugin_mapping;
    let mut mp: *mut *mut plugin_mapping = 0 as *mut *mut plugin_mapping;
    let mut mod_0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    /* Allocate space for all of the modules plus a null terminator. */
    count = 0 as libc::c_int as size_t;
    while !(*modstrs.offset(count as isize)).is_null() {
        count = count.wrapping_add(1)
    }
    mp = *list_inout;
    while !mp.is_null() && !(*mp).is_null() {
        mp = mp.offset(1);
        count = count.wrapping_add(1)
    }
    list =
        calloc(count.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<*mut plugin_mapping>() as libc::c_ulong)
            as *mut *mut plugin_mapping;
    if list.is_null() { return 12 as libc::c_int }
    /* Parse each profile module entry and store it in the list. */
    mod_0 = modstrs;
    pos = 0 as libc::c_int as size_t;
    while !(*mod_0).is_null() {
        ret = parse_modstr(context, *mod_0, &mut *list.offset(pos as isize));
        if ret != 0 as libc::c_int { free_mapping_list(list); return ret }
        mod_0 = mod_0.offset(1);
        pos = pos.wrapping_add(1)
    }
    /* Cannibalize the old list of built-in modules. */
    mp = *list_inout;
    while !mp.is_null() && !(*mp).is_null() {
        let ref mut fresh2 = *list.offset(pos as isize);
        *fresh2 = *mp;
        mp = mp.offset(1);
        pos = pos.wrapping_add(1)
    }
    if pos == count {
    } else {
        __assert_fail(b"pos == count\x00" as *const u8 as *const libc::c_char,
                      b"plugin.c\x00" as *const u8 as *const libc::c_char,
                      235 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 81],
                                                &[libc::c_char; 81]>(b"krb5_error_code make_full_list(krb5_context, char **, struct plugin_mapping ***)\x00")).as_ptr());
    }
    /* Filter out duplicates, preferring earlier entries to later ones. */
    i = 0 as libc::c_int as size_t;
    pos = 0 as libc::c_int as size_t;
    while i < count {
        j = 0 as libc::c_int as size_t;
        while j < pos {
            if strcmp((**list.offset(i as isize)).modname,
                      (**list.offset(j as isize)).modname) == 0 as libc::c_int
               {
                free_plugin_mapping(*list.offset(i as isize));
                break ;
            } else { j = j.wrapping_add(1) }
        }
        if j == pos {
            let fresh3 = pos;
            pos = pos.wrapping_add(1);
            let ref mut fresh4 = *list.offset(fresh3 as isize);
            *fresh4 = *list.offset(i as isize)
        }
        i = i.wrapping_add(1)
    }
    let ref mut fresh5 = *list.offset(pos as isize);
    *fresh5 = 0 as *mut plugin_mapping;
    free(*list_inout as *mut libc::c_void);
    *list_inout = list;
    return 0 as libc::c_int;
}
/* Remove any entries from list which match values in disabled. */
#[c2rust::src_loc = "256:1"]
unsafe extern "C" fn remove_disabled_modules(mut list:
                                                 *mut *mut plugin_mapping,
                                             mut disable:
                                                 *mut *mut libc::c_char) {
    let mut in_0: *mut *mut plugin_mapping = 0 as *mut *mut plugin_mapping;
    let mut out: *mut *mut plugin_mapping = 0 as *mut *mut plugin_mapping;
    out = list;
    in_0 = list;
    while !(*in_0).is_null() {
        if find_in_list(disable, (**in_0).modname) != 0 {
            free_plugin_mapping(*in_0);
        } else { let fresh6 = out; out = out.offset(1); *fresh6 = *in_0 }
        in_0 = in_0.offset(1)
    }
    *out = 0 as *mut plugin_mapping;
}
/* Modify list to include only the entries matching strings in enable, in
 * the order they are listed there. */
#[c2rust::src_loc = "273:1"]
unsafe extern "C" fn filter_enabled_modules(mut list:
                                                *mut *mut plugin_mapping,
                                            mut enable:
                                                *mut *mut libc::c_char) {
    let mut count: size_t = 0;
    let mut i: size_t = 0;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut tmp: *mut plugin_mapping = 0 as *mut plugin_mapping;
    /* Count the number of existing entries. */
    count = 0 as libc::c_int as size_t;
    while !(*list.offset(count as isize)).is_null() {
        count = count.wrapping_add(1)
    }
    /* For each string in enable, look for a matching module. */
    while !(*enable).is_null() {
        i = pos;
        while i < count {
            if strcmp((**list.offset(i as isize)).modname, *enable) ==
                   0 as libc::c_int {
                /* Swap the matching module into the next result position. */
                tmp = *list.offset(pos as isize);
                let fresh7 = pos;
                pos = pos.wrapping_add(1);
                let ref mut fresh8 = *list.offset(fresh7 as isize);
                *fresh8 = *list.offset(i as isize);
                let ref mut fresh9 = *list.offset(i as isize);
                *fresh9 = tmp;
                break ;
            } else { i = i.wrapping_add(1) }
        }
        enable = enable.offset(1)
    }
    /* Free all mappings which didn't match and terminate the list. */
    i = pos;
    while i < count {
        free_plugin_mapping(*list.offset(i as isize));
        i = i.wrapping_add(1)
    }
    let ref mut fresh10 = *list.offset(pos as isize);
    *fresh10 = 0 as *mut plugin_mapping;
}
/* Ensure that a plugin interface is configured.  id must be valid. */
#[c2rust::src_loc = "302:1"]
unsafe extern "C" fn configure_interface(mut context: krb5_context,
                                         mut id: libc::c_int)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut interface: *mut plugin_interface =
        &mut *(*context).plugins.as_mut_ptr().offset(id as isize) as
            *mut plugin_interface;
    let mut modstrs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut enable: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut disable: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if (*interface).configured != 0 { return 0 as libc::c_int }
    /* Detect consistency errors when plugin interfaces are added. */
    if (::std::mem::size_of::<[*const libc::c_char; 13]>() as
            libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                            as libc::c_ulong) ==
           13 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"sizeof(interface_names) / sizeof(*interface_names) == PLUGIN_NUM_INTERFACES\x00"
                          as *const u8 as *const libc::c_char,
                      b"plugin.c\x00" as *const u8 as *const libc::c_char,
                      314 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"krb5_error_code configure_interface(krb5_context, int)\x00")).as_ptr());
    }
    /* Get profile variables for this interface. */
    ret =
        get_profile_var(context, id,
                        b"module\x00" as *const u8 as *const libc::c_char,
                        &mut modstrs);
    if !(ret != 0) {
        ret =
            get_profile_var(context, id,
                            b"disable\x00" as *const u8 as
                                *const libc::c_char, &mut disable);
        if !(ret != 0) {
            ret =
                get_profile_var(context, id,
                                b"enable_only\x00" as *const u8 as
                                    *const libc::c_char, &mut enable);
            if !(ret != 0) {
                /* Create the full list of dynamic and built-in modules. */
                if !modstrs.is_null() {
                    ret =
                        make_full_list(context, modstrs,
                                       &mut (*interface).modules);
                    if ret != 0 {
                        current_block = 10267229552094895881;
                    } else { current_block = 10599921512955367680; }
                } else { current_block = 10599921512955367680; }
                match current_block {
                    10267229552094895881 => { }
                    _ => {
                        /* Remove disabled modules. */
                        if !disable.is_null() {
                            remove_disabled_modules((*interface).modules,
                                                    disable);
                        }
                        /* Filter and re-order the list according to enable-modules. */
                        if !enable.is_null() {
                            filter_enabled_modules((*interface).modules,
                                                   enable);
                        }
                    }
                }
            }
        }
    }
    profile_free_list(modstrs);
    profile_free_list(enable);
    profile_free_list(disable);
    return ret;
}
/* If map is for a dynamic module which hasn't been loaded yet, attempt to load
 * it.  Only try to load a module once. */
#[c2rust::src_loc = "351:1"]
unsafe extern "C" fn load_if_needed(mut context: krb5_context,
                                    mut map: *mut plugin_mapping,
                                    mut iname: *const libc::c_char) {
    let mut symname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut handle: *mut plugin_file_handle = 0 as *mut plugin_file_handle;
    let mut initvt_fn: Option<unsafe extern "C" fn() -> ()> = None;
    if (*map).module.is_some() || (*map).dyn_path.is_null() { return }
    if asprintf(&mut symname as *mut *mut libc::c_char,
                b"%s_%s_initvt\x00" as *const u8 as *const libc::c_char,
                iname, (*map).modname) < 0 as libc::c_int {
        return
    }
    if !(krb5int_open_plugin((*map).dyn_path, &mut handle,
                             &mut (*context).err) != 0) {
        if !(krb5int_get_plugin_func(handle, symname, &mut initvt_fn,
                                     &mut (*context).err) != 0) {
            free(symname as *mut libc::c_void);
            (*map).dyn_handle = handle;
            (*map).module =
                ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                        krb5_plugin_initvt_fn>(initvt_fn);
            return
        }
    }
    /* Clean up, and also null out map->dyn_path so we don't try again. */
    if !handle.is_null() { krb5int_close_plugin(handle); }
    free(symname as *mut libc::c_void);
    free((*map).dyn_path as *mut libc::c_void);
    (*map).dyn_path = 0 as *mut libc::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "381:1"]
pub unsafe extern "C" fn k5_plugin_load(mut context: krb5_context,
                                        mut interface_id: libc::c_int,
                                        mut modname: *const libc::c_char,
                                        mut module:
                                            *mut krb5_plugin_initvt_fn)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut interface: *mut plugin_interface =
        get_interface(context, interface_id);
    let mut mp: *mut *mut plugin_mapping = 0 as *mut *mut plugin_mapping;
    let mut map: *mut plugin_mapping = 0 as *mut plugin_mapping;
    if interface.is_null() { return 22 as libc::c_int }
    ret = configure_interface(context, interface_id);
    if ret != 0 as libc::c_int { return ret }
    mp = (*interface).modules;
    while !mp.is_null() && !(*mp).is_null() {
        map = *mp;
        if strcmp((*map).modname, modname) == 0 as libc::c_int {
            load_if_needed(context, map,
                           interface_names[interface_id as usize]);
            if (*map).module.is_some() {
                *module = (*map).module;
                return 0 as libc::c_int
            }
            break ;
        } else { mp = mp.offset(1) }
    }
    krb5_set_error_message(context,
                           -(1750600190 as libc::c_long) as krb5_error_code,
                           dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Could not find %s plugin module named \'%s\'\x00"
                                        as *const u8 as *const libc::c_char),
                           interface_names[interface_id as usize], modname);
    return -(1750600190 as libc::c_long) as krb5_error_code;
}
#[no_mangle]
#[c2rust::src_loc = "411:1"]
pub unsafe extern "C" fn k5_plugin_load_all(mut context: krb5_context,
                                            mut interface_id: libc::c_int,
                                            mut modules:
                                                *mut *mut krb5_plugin_initvt_fn)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut interface: *mut plugin_interface =
        get_interface(context, interface_id);
    let mut mp: *mut *mut plugin_mapping = 0 as *mut *mut plugin_mapping;
    let mut map: *mut plugin_mapping = 0 as *mut plugin_mapping;
    let mut list: *mut krb5_plugin_initvt_fn =
        0 as *mut krb5_plugin_initvt_fn;
    let mut count: size_t = 0;
    if interface.is_null() { return 22 as libc::c_int }
    ret = configure_interface(context, interface_id);
    if ret != 0 as libc::c_int { return ret }
    /* Count the modules and allocate a list to hold them. */
    mp = (*interface).modules;
    count = 0 as libc::c_int as size_t;
    while !mp.is_null() && !(*mp.offset(count as isize)).is_null() {
        count = count.wrapping_add(1)
    }
    list =
        calloc(count.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<krb5_plugin_initvt_fn>() as
                   libc::c_ulong) as *mut krb5_plugin_initvt_fn;
    if list.is_null() { return 12 as libc::c_int }
    /* Place each module's initvt function into list. */
    count = 0 as libc::c_int as size_t;
    mp = (*interface).modules;
    while !mp.is_null() && !(*mp).is_null() {
        map = *mp;
        load_if_needed(context, map, interface_names[interface_id as usize]);
        if (*map).module.is_some() {
            let fresh11 = count;
            count = count.wrapping_add(1);
            let ref mut fresh12 = *list.offset(fresh11 as isize);
            *fresh12 = (*map).module
        }
        mp = mp.offset(1)
    }
    *modules = list;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "447:1"]
pub unsafe extern "C" fn k5_plugin_free_modules(mut context: krb5_context,
                                                mut modules:
                                                    *mut krb5_plugin_initvt_fn) {
    free(modules as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "453:1"]
pub unsafe extern "C" fn k5_plugin_register(mut context: krb5_context,
                                            mut interface_id: libc::c_int,
                                            mut modname: *const libc::c_char,
                                            mut module: krb5_plugin_initvt_fn)
 -> krb5_error_code {
    let mut interface: *mut plugin_interface =
        get_interface(context, interface_id);
    if interface.is_null() { return 22 as libc::c_int }
    /* Disallow registering plugins after load.  We may need to reconsider
     * this, but it simplifies the design. */
    if (*interface).configured != 0 { return 22 as libc::c_int }
    return register_module(context, interface, modname,
                           0 as *const libc::c_char, module);
}
#[no_mangle]
#[c2rust::src_loc = "470:1"]
pub unsafe extern "C" fn k5_plugin_register_dyn(mut context: krb5_context,
                                                mut interface_id: libc::c_int,
                                                mut modname:
                                                    *const libc::c_char,
                                                mut modsubdir:
                                                    *const libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut interface: *mut plugin_interface =
        get_interface(context, interface_id);
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Disallow registering plugins after load. */
    if interface.is_null() || (*interface).configured != 0 {
        return 22 as libc::c_int
    }
    if asprintf(&mut fname as *mut *mut libc::c_char,
                b"%s%s\x00" as *const u8 as *const libc::c_char, modname,
                b".so\x00" as *const u8 as *const libc::c_char) <
           0 as libc::c_int {
        return 12 as libc::c_int
    }
    ret = k5_path_join(modsubdir, fname, &mut path) as krb5_error_code;
    free(fname as *mut libc::c_void);
    if ret != 0 { return ret }
    ret = register_module(context, interface, modname, path, None);
    free(path as *mut libc::c_void);
    return ret;
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
#[no_mangle]
#[c2rust::src_loc = "493:1"]
pub unsafe extern "C" fn k5_plugin_free_context(mut context: krb5_context) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 13 as libc::c_int {
        free_mapping_list((*context).plugins[i as usize].modules);
        i += 1
    }
    memset((*context).plugins.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[plugin_interface; 13]>() as libc::c_ulong);
}
