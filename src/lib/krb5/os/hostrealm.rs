use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/unistd.h:33"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "877:1"]
        pub fn gethostname(__name: *mut libc::c_char, __len: size_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:33"]
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
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:33"]
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_context, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::hostrealm_module_handle;
    use super::k5_err_h::errinfo;
    use super::plugin_h::krb5_plugin_initvt_fn;
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
    use super::string_h::memcpy;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1202:8"]
        pub type _kdb_log_context;
        #[c2rust::src_loc = "1207:8"]
        pub type k5_tls_vtable_st;
        #[c2rust::src_loc = "1205:8"]
        pub type localauth_module_handle;
        #[c2rust::src_loc = "1204:8"]
        pub type ccselect_module_handle;
        #[c2rust::src_loc = "1203:16"]
        pub type krb5_preauth_context_st;
        #[c2rust::src_loc = "1200:8"]
        pub type _kdb5_dal_handle;
        #[no_mangle]
        #[c2rust::src_loc = "1173:1"]
        pub fn k5_plugin_free_modules(context: krb5_context,
                                      modules: *mut krb5_plugin_initvt_fn);
        #[no_mangle]
        #[c2rust::src_loc = "1168:1"]
        pub fn k5_plugin_load_all(context: krb5_context,
                                  interface_id: libc::c_int,
                                  modules: *mut *mut krb5_plugin_initvt_fn)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1177:1"]
        pub fn k5_plugin_register(context: krb5_context,
                                  interface_id: libc::c_int,
                                  modname: *const libc::c_char,
                                  module: krb5_plugin_initvt_fn)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/hostrealm_plugin.h:36"]
pub mod hostrealm_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2013 by the Massachusetts Institute of Technology.
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
 * Declarations for hostrealm plugin module implementors.
 *
 * The hostrealm pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * Hostrealm plugin modules should define a function named
 * hostrealm_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   hostrealm_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                            krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_hostrealm_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* An abstract type for hostrealm module data. */
    #[c2rust::src_loc = "67:1"]
    pub type krb5_hostrealm_moddata = *mut krb5_hostrealm_moddata_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "124:16"]
    pub struct krb5_hostrealm_vtable_st {
        pub name: *const libc::c_char,
        pub init: krb5_hostrealm_init_fn,
        pub fini: krb5_hostrealm_fini_fn,
        pub host_realm: krb5_hostrealm_host_realm_fn,
        pub fallback_realm: krb5_hostrealm_fallback_realm_fn,
        pub default_realm: krb5_hostrealm_default_realm_fn,
        pub free_list: krb5_hostrealm_free_list_fn,
    }
    /*
 * Mandatory (if any of the query methods are implemented): Release the memory
 * returned by one of the interface methods.
 */
    #[c2rust::src_loc = "115:1"]
    pub type krb5_hostrealm_free_list_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_hostrealm_moddata,
                                    _: *mut *mut libc::c_char) -> ()>;
    /*
 * Optional: Determine the possible default realms of the local host.  Return
 * success with a null-terminated list of realms in *realms_out,
 * KRB5_PLUGIN_NO_HANDLE to defer to later modules, or another error to
 * terminate processing.
 */
    #[c2rust::src_loc = "106:1"]
    pub type krb5_hostrealm_default_realm_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_hostrealm_moddata,
                                    _: *mut *mut *mut libc::c_char)
                   -> krb5_error_code>;
    /*
 * Optional: Determine the possible realms of a hostname, using heuristic or
 * less secure mechanisms (ones which should be used after trying referrals
 * when getting a service ticket).  Return success with a null-terminated list
 * of realms in *realms_out, KRB5_PLUGIN_NO_HANDLE to defer to later modules,
 * or another error to terminate processing.
 */
    #[c2rust::src_loc = "95:1"]
    pub type krb5_hostrealm_fallback_realm_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_hostrealm_moddata,
                                    _: *const libc::c_char,
                                    _: *mut *mut *mut libc::c_char)
                   -> krb5_error_code>;
    /*
 * Optional: Determine the possible realms of a hostname, using only secure,
 * authoritative mechanisms (ones which should be used prior to trying
 * referrals when getting a service ticket).  Return success with a
 * null-terminated list of realms in *realms_out, KRB5_PLUGIN_NO_HANDLE to
 * defer to later modules, or another error to terminate processing.
 */
    #[c2rust::src_loc = "83:1"]
    pub type krb5_hostrealm_host_realm_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_hostrealm_moddata,
                                    _: *const libc::c_char,
                                    _: *mut *mut *mut libc::c_char)
                   -> krb5_error_code>;
    /* Optional: Release resources used by module data. */
    #[c2rust::src_loc = "120:1"]
    pub type krb5_hostrealm_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_hostrealm_moddata) -> ()>;
    /* ** Method type declarations ***/
    /* Optional: Initialize module data. */
    #[c2rust::src_loc = "72:1"]
    pub type krb5_hostrealm_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut krb5_hostrealm_moddata)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_context, krb5_error_code};
    extern "C" {
        #[c2rust::src_loc = "67:16"]
        pub type krb5_hostrealm_moddata_st;
    }
    /* KRB5_HOSTREALM_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:33"]
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
#[c2rust::header_src = "/usr/include/ctype.h:37"]
pub mod ctype_h {
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[no_mangle]
        #[c2rust::src_loc = "122:12"]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netdb.h:33"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "565:8"]
    pub struct addrinfo {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut libc::c_char,
        pub ai_next: *mut addrinfo,
    }
    use super::unistd_h::socklen_t;
    use super::socket_h::sockaddr;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:33"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:33"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/errno.h:33"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "276:15"]
        pub fn strspn(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:33"]
pub mod k5_trace_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::krb5_context;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-trace.h */
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
 * This header contains trace macro definitions, which map trace points within
 * the code to krb5int_trace() calls with descriptive text strings.
 *
 * A new trace macro must be defined in this file for each new location to
 * be traced; the TRACE() macro should never be used directly.  This keeps
 * the tracing logic centralized in one place, to facilitate integration with
 * alternate tracing backends such as DTrace.
 *
 * Trace logging is intended to aid power users in diagnosing configuration
 * problems by showing what's going on behind the scenes of complex operations.
 * Although trace logging is sometimes useful to developers, it is not intended
 * as a replacement for a debugger, and it is not desirable to drown the user
 * in output.  Observe the following guidelines when adding trace points:
 *
 *   - Avoid mentioning function or variable names in messages.
 *
 *   - Try to convey what decisions are being made and what external inputs
 *     they are based on, not the process of making decisions.
 *
 *   - It is generally not necessary to trace before returning an unrecoverable
 *     error.  If an error code is unclear by itself, make it clearer with
 *     krb5_set_error_message().
 *
 *   - Keep macros simple.  Add format specifiers to krb5int_trace's formatter
 *     as necessary (and document them here) instead of transforming macro
 *     arguments.
 *
 *   - Like printf, the trace formatter interface is not type-safe.  Check your
 *     formats carefully.  Cast integral arguments to the appropriate type if
 *     they do not already patch.
 *
 * The following specifiers are supported by the formatter (see the
 * implementation in lib/krb5/os/trace.c for details):
 *
 *   {int}         int, in decimal
 *   {long}        long, in decimal
 *   {str}         const char *, display as C string
 *   {lenstr}      size_t and const char *, as a counted string
 *   {hexlenstr}   size_t and const char *, as hex bytes
 *   {hashlenstr}  size_t and const char *, as four-character hex hash
 *   {raddr}       struct remote_address *, show socket type, address, port
 *   {data}        krb5_data *, display as counted string
 *   {hexdata}     krb5_data *, display as hex bytes
 *   {errno}       int, display as number/errorstring
 *   {kerr}        krb5_error_code, display as number/errorstring
 *   {keyblock}    const krb5_keyblock *, display enctype and hash of key
 *   {key}         krb5_key, display enctype and hash of key
 *   {cksum}       const krb5_checksum *, display cksumtype and hex checksum
 *   {princ}       krb5_principal, unparse and display
 *   {ptype}       krb5_int32, krb5_principal type, display name
 *   {patype}      krb5_preauthtype, a single padata type number
 *   {patypes}     krb5_pa_data **, display list of padata type numbers
 *   {etype}       krb5_enctype, display shortest name of enctype
 *   {etypes}      krb5_enctype *, display list of enctypes
 *   {ccache}      krb5_ccache, display type:name
 *   {keytab}      krb5_keytab, display name
 *   {creds}       krb5_creds *, display clientprinc -> serverprinc
 */
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn krb5int_trace(context: krb5_context, fmt: *const libc::c_char,
                             _: ...);
    }
    /* K5_TRACE_H */
    /* DISABLE_TRACING */
    /* Try to optimize away argument evaluation and function call when we're not
 * tracing, if this source file knows the internals of the context. */
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:34"]
pub mod os_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    use super::plugin_h::{krb5_plugin_vtable_st, krb5_plugin_vtable};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn hostrealm_profile_initvt(context: krb5_context,
                                        maj_ver: libc::c_int,
                                        min_ver: libc::c_int,
                                        vtable: krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "199:1"]
        pub fn hostrealm_registry_initvt(context: krb5_context,
                                         maj_ver: libc::c_int,
                                         min_ver: libc::c_int,
                                         vtable: krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "202:1"]
        pub fn hostrealm_dns_initvt(context: krb5_context,
                                    maj_ver: libc::c_int,
                                    min_ver: libc::c_int,
                                    vtable: krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "204:1"]
        pub fn hostrealm_domain_initvt(context: krb5_context,
                                       maj_ver: libc::c_int,
                                       min_ver: libc::c_int,
                                       vtable: krb5_plugin_vtable)
         -> krb5_error_code;
    }
    /* KRB5_LIBOS_INT_PROTO__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/fake-addrinfo.h:35"]
pub mod fake_addrinfo_h {
    use super::netdb_h::addrinfo;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2001,2002,2003,2004 by the Massachusetts Institute of Technology,
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
        /* Approach overview:

   If a system version is available but buggy, save handles to it (via
   inline functions in a support library), redefine the names to refer
   to library functions, and in those functions, call the system
   versions and fix up the returned data.  Use the native data
   structures and flag values.

   If no system version exists, use gethostby* and fake it.  Define
   the data structures and flag values locally.


   On macOS, getaddrinfo results aren't cached (though
   gethostbyname results are), so we need to build a cache here.  Now
   things are getting really messy.  Because the cache is in use, we
   use getservbyname, and throw away thread safety.  (Not that the
   cache is thread safe, but when we get locking support, that'll be
   dealt with.)  This code needs tearing down and rebuilding, soon.


   Note that recent Windows developers' code has an interesting hack:
   When you include the right header files, with the right set of
   macros indicating system versions, you'll get an inline function
   that looks for getaddrinfo (or whatever) in the system library, and
   calls it if it's there.  If it's not there, it fakes it with
   gethostby* calls.

   We're taking a simpler approach: A system provides these routines or
   it does not.

   Someday, we may want to take into account different versions (say,
   different revs of GNU libc) where some are broken in one way, and
   some work or are broken in another way.  Cross that bridge when we
   come to it.  */
        /* To do, maybe:

   + For AIX 4.3.3, using the RFC 2133 definition: Implement
   AI_NUMERICHOST.  It's not defined in the header file.

   For certain (old?) versions of GNU libc, AI_NUMERICHOST is
   defined but not implemented.

   + Use gethostbyname2, inet_aton and other IPv6 or thread-safe
   functions if available.  But, see
   https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=135182 for one
   gethostbyname2 problem on Linux.  And besides, if a platform is
   supporting IPv6 at all, they really should be doing getaddrinfo
   by now.

   + inet_ntop, inet_pton

   + Conditionally export/import the function definitions, so a
   library can have a single copy instead of multiple.

   + Upgrade host requirements to include working implementations of
   these functions, and throw all this away.  Pleeease?  :-)  */
        /* ! HAVE_GETADDRINFO */
        /* Fudge things on older gai implementations.  */
/* AIX 4.3.3 is based on RFC 2133; no AI_NUMERICHOST.  */
        /* Partial RFC 2553 implementations may not have AI_ADDRCONFIG and
   friends, which RFC 3493 says are now part of the getaddrinfo
   interface, and we'll want to use.  */
        /* Call out to stuff defined in libkrb5support.  */
        #[no_mangle]
        #[c2rust::src_loc = "214:1"]
        pub fn krb5int_getaddrinfo(node: *const libc::c_char,
                                   service: *const libc::c_char,
                                   hints: *const addrinfo,
                                   aip: *mut *mut addrinfo) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn krb5int_freeaddrinfo(ai: *mut addrinfo);
    }
    /* FAI_DEFINED */
}
pub use self::types_h::{__int32_t, __socklen_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::unistd_h::{socklen_t, gethostname};
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _profile_t};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, k5alloc, k5calloc, k5memdup0,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         k5_plugin_free_modules, k5_plugin_load_all,
                         k5_plugin_register};
pub use self::k5_err_h::errinfo;
pub use self::hostrealm_plugin_h::{krb5_hostrealm_moddata,
                                   krb5_hostrealm_vtable_st,
                                   krb5_hostrealm_free_list_fn,
                                   krb5_hostrealm_default_realm_fn,
                                   krb5_hostrealm_fallback_realm_fn,
                                   krb5_hostrealm_host_realm_fn,
                                   krb5_hostrealm_fini_fn,
                                   krb5_hostrealm_init_fn,
                                   krb5_hostrealm_moddata_st};
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::plugin_h::{krb5_plugin_initvt_fn, krb5_plugin_vtable,
                         krb5_plugin_vtable_st};
pub use self::ctype_h::{_ISupper, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, __ctype_b_loc, tolower};
pub use self::netdb_h::addrinfo;
pub use self::socket_h::sockaddr;
pub use self::sockaddr_h::sa_family_t;
use self::errno_h::__errno_location;
use self::string_h::{strlen, strspn, strchr, strdup, memset, memcpy};
use self::k5_trace_h::krb5int_trace;
use self::stdlib_h::{free, abort, calloc};
use self::os_proto_h::{hostrealm_profile_initvt, hostrealm_registry_initvt,
                       hostrealm_dns_initvt, hostrealm_domain_initvt};
use self::fake_addrinfo_h::{krb5int_getaddrinfo, krb5int_freeaddrinfo};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/os/hostrealm.c - realm-of-host and default-realm APIs */
/*
 * Copyright (C) 2013 by the Massachusetts Institute of Technology.
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
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "45:8"]
pub struct hostrealm_module_handle {
    pub vt: krb5_hostrealm_vtable_st,
    pub data: krb5_hostrealm_moddata,
}
/* Release a list of hostrealm module handles. */
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn free_handles(mut context: krb5_context,
                                  mut handles:
                                      *mut *mut hostrealm_module_handle) {
    let mut h: *mut hostrealm_module_handle =
        0 as *mut hostrealm_module_handle;
    let mut hp: *mut *mut hostrealm_module_handle =
        0 as *mut *mut hostrealm_module_handle;
    if handles.is_null() { return }
    hp = handles;
    while !(*hp).is_null() {
        h = *hp;
        if (*h).vt.fini.is_some() {
            (*h).vt.fini.expect("non-null function pointer")(context,
                                                             (*h).data);
        }
        free(h as *mut libc::c_void);
        hp = hp.offset(1)
    }
    free(handles as *mut libc::c_void);
}
/* Get the registered hostrealm modules including all built-in modules, in the
 * proper order. */
#[c2rust::src_loc = "69:1"]
unsafe extern "C" fn get_modules(mut context: krb5_context,
                                 mut modules_out:
                                     *mut *mut krb5_plugin_initvt_fn)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let intf: libc::c_int = 6 as libc::c_int;
    *modules_out = 0 as *mut krb5_plugin_initvt_fn;
    /* Register built-in modules. */
    ret =
        k5_plugin_register(context, intf,
                           b"registry\x00" as *const u8 as
                               *const libc::c_char,
                           Some(hostrealm_registry_initvt as
                                    unsafe extern "C" fn(_: krb5_context,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             krb5_plugin_vtable)
                                        -> krb5_error_code));
    if ret != 0 { return ret }
    ret =
        k5_plugin_register(context, intf,
                           b"profile\x00" as *const u8 as *const libc::c_char,
                           Some(hostrealm_profile_initvt as
                                    unsafe extern "C" fn(_: krb5_context,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             krb5_plugin_vtable)
                                        -> krb5_error_code));
    if ret != 0 { return ret }
    ret =
        k5_plugin_register(context, intf,
                           b"dns\x00" as *const u8 as *const libc::c_char,
                           Some(hostrealm_dns_initvt as
                                    unsafe extern "C" fn(_: krb5_context,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             krb5_plugin_vtable)
                                        -> krb5_error_code));
    if ret != 0 { return ret }
    ret =
        k5_plugin_register(context, intf,
                           b"domain\x00" as *const u8 as *const libc::c_char,
                           Some(hostrealm_domain_initvt as
                                    unsafe extern "C" fn(_: krb5_context,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             krb5_plugin_vtable)
                                        -> krb5_error_code));
    if ret != 0 { return ret }
    return k5_plugin_load_all(context, intf, modules_out);
}
/* Initialize context->hostrealm_handles with a list of module handles. */
#[c2rust::src_loc = "97:1"]
unsafe extern "C" fn load_hostrealm_modules(mut context: krb5_context)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut list: *mut *mut hostrealm_module_handle =
        0 as *mut *mut hostrealm_module_handle;
    let mut handle: *mut hostrealm_module_handle =
        0 as *mut hostrealm_module_handle;
    let mut modules: *mut krb5_plugin_initvt_fn =
        0 as *mut krb5_plugin_initvt_fn;
    let mut mod_0: *mut krb5_plugin_initvt_fn =
        0 as *mut krb5_plugin_initvt_fn;
    let mut count: size_t = 0;
    ret = get_modules(context, &mut modules);
    if !(ret != 0 as libc::c_int) {
        /* Allocate a large enough list of handles. */
        count = 0 as libc::c_int as size_t;
        while (*modules.offset(count as isize)).is_some() {
            count = count.wrapping_add(1)
        }
        list =
            k5alloc(count.wrapping_add(1 as libc::c_int as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut hostrealm_module_handle>()
                                                                           as
                                                                           libc::c_ulong),
                    &mut ret) as *mut *mut hostrealm_module_handle;
        if !list.is_null() {
            /* Initialize each module, ignoring ones that fail. */
            count = 0 as libc::c_int as size_t;
            mod_0 = modules;
            loop  {
                if !(*mod_0).is_some() {
                    current_block = 14136749492126903395;
                    break ;
                }
                handle =
                    k5alloc(::std::mem::size_of::<hostrealm_module_handle>()
                                as libc::c_ulong, &mut ret) as
                        *mut hostrealm_module_handle;
                if handle.is_null() {
                    current_block = 4005560482714005869;
                    break ;
                }
                ret =
                    (*mod_0).expect("non-null function pointer")(context,
                                                                 1 as
                                                                     libc::c_int,
                                                                 1 as
                                                                     libc::c_int,
                                                                 &mut (*handle).vt
                                                                     as
                                                                     *mut krb5_hostrealm_vtable_st
                                                                     as
                                                                     krb5_plugin_vtable);
                if ret != 0 as libc::c_int {
                    if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"hostrealm module failed to init vtable: {kerr}\x00"
                                          as *const u8 as *const libc::c_char,
                                      ret);
                    }
                    free(handle as *mut libc::c_void);
                } else {
                    (*handle).data = 0 as krb5_hostrealm_moddata;
                    if (*handle).vt.init.is_some() {
                        ret =
                            (*handle).vt.init.expect("non-null function pointer")(context,
                                                                                  &mut (*handle).data);
                        if ret != 0 as libc::c_int {
                            if (*context).trace_callback.is_some() {
                                krb5int_trace(context,
                                              b"hostrealm module {str} failed to init: {kerr}\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              (*handle).vt.name, ret);
                            }
                            free(handle as *mut libc::c_void);
                            current_block = 13536709405535804910;
                        } else { current_block = 2569451025026770673; }
                    } else { current_block = 2569451025026770673; }
                    match current_block {
                        13536709405535804910 => { }
                        _ => {
                            let fresh0 = count;
                            count = count.wrapping_add(1);
                            let ref mut fresh1 =
                                *list.offset(fresh0 as isize);
                            *fresh1 = handle;
                            let ref mut fresh2 = *list.offset(count as isize);
                            *fresh2 = 0 as *mut hostrealm_module_handle
                        }
                    }
                }
                mod_0 = mod_0.offset(1)
            }
            match current_block {
                4005560482714005869 => { }
                _ => {
                    let ref mut fresh3 = *list.offset(count as isize);
                    *fresh3 = 0 as *mut hostrealm_module_handle;
                    ret = 0 as libc::c_int;
                    (*context).hostrealm_handles = list;
                    list = 0 as *mut *mut hostrealm_module_handle
                }
            }
        }
    }
    k5_plugin_free_modules(context, modules);
    free_handles(context, list);
    return ret;
}
/* Invoke a module's host_realm method, if it has one. */
#[c2rust::src_loc = "153:1"]
unsafe extern "C" fn host_realm(mut context: krb5_context,
                                mut h: *mut hostrealm_module_handle,
                                mut host: *const libc::c_char,
                                mut realms_out: *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    if (*h).vt.host_realm.is_none() {
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    return (*h).vt.host_realm.expect("non-null function pointer")(context,
                                                                  (*h).data,
                                                                  host,
                                                                  realms_out);
}
/* Invoke a module's fallback_realm method, if it has one. */
#[c2rust::src_loc = "163:1"]
unsafe extern "C" fn fallback_realm(mut context: krb5_context,
                                    mut h: *mut hostrealm_module_handle,
                                    mut host: *const libc::c_char,
                                    mut realms_out:
                                        *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    if (*h).vt.fallback_realm.is_none() {
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    return (*h).vt.fallback_realm.expect("non-null function pointer")(context,
                                                                      (*h).data,
                                                                      host,
                                                                      realms_out);
}
/* Invoke a module's default_realm method, if it has one. */
#[c2rust::src_loc = "173:1"]
unsafe extern "C" fn default_realm(mut context: krb5_context,
                                   mut h: *mut hostrealm_module_handle,
                                   mut realms_out:
                                       *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    if (*h).vt.default_realm.is_none() {
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    return (*h).vt.default_realm.expect("non-null function pointer")(context,
                                                                     (*h).data,
                                                                     realms_out);
}
/* Invoke a module's free_list method. */
#[c2rust::src_loc = "183:1"]
unsafe extern "C" fn free_list(mut context: krb5_context,
                               mut h: *mut hostrealm_module_handle,
                               mut list: *mut *mut libc::c_char) {
    (*h).vt.free_list.expect("non-null function pointer")(context, (*h).data,
                                                          list);
}
/* Copy a null-terminated list of strings. */
#[c2rust::src_loc = "191:1"]
unsafe extern "C" fn copy_list(mut in_0: *mut *mut libc::c_char,
                               mut out: *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut count: size_t = 0; /* XXX */
    let mut i: size_t = 0;
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    *out = 0 as *mut *mut libc::c_char;
    count = 0 as libc::c_int as size_t;
    while !(*in_0.offset(count as isize)).is_null() {
        count = count.wrapping_add(1)
    }
    list =
        calloc(count.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) as
            *mut *mut libc::c_char;
    if list.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int as size_t;
    while i < count {
        let ref mut fresh4 = *list.offset(i as isize);
        *fresh4 = strdup(*in_0.offset(i as isize));
        if (*list.offset(i as isize)).is_null() {
            krb5_free_host_realm(0 as krb5_context, list);
            return 12 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    *out = list;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn translate_gai_error(mut num: libc::c_int)
 -> krb5_error_code {
    match num {
        -9 => { return 97 as libc::c_int }
        -3 => { return 11 as libc::c_int }
        -1 => { return 22 as libc::c_int }
        -4 => { return -(1765328145 as libc::c_long) as krb5_error_code }
        -6 => { return 97 as libc::c_int }
        -10 => { return 12 as libc::c_int }
        -5 => { return -(1765328144 as libc::c_long) as krb5_error_code }
        -2 => { return -(1765328143 as libc::c_long) as krb5_error_code }
        -12 => { return 22 as libc::c_int }
        -8 => { return -(1765328142 as libc::c_long) as krb5_error_code }
        -7 => { return 22 as libc::c_int }
        -11 => { return *__errno_location() }
        _ => { }
    }
    abort();
}
/* Get the canonical form of the local host name, using forward
 * canonicalization only. */
#[no_mangle]
#[c2rust::src_loc = "256:1"]
pub unsafe extern "C" fn krb5int_get_fq_local_hostname(mut hostname_out:
                                                           *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut err: libc::c_int = 0;
    *hostname_out = 0 as *mut libc::c_char;
    if gethostname(buf.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong) == -(1 as libc::c_int) {
        return *__errno_location()
    }
    memset(&mut hints as *mut addrinfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hints.ai_flags = 0x2 as libc::c_int | 0x20 as libc::c_int;
    err =
        krb5int_getaddrinfo(buf.as_mut_ptr(), 0 as *const libc::c_char,
                            &mut hints, &mut ai);
    if err != 0 { return translate_gai_error(err) }
    if (*ai).ai_canonname.is_null() {
        krb5int_freeaddrinfo(ai);
        return -(1765328145 as libc::c_long) as krb5_error_code
    }
    *hostname_out = strdup((*ai).ai_canonname);
    krb5int_freeaddrinfo(ai);
    return if (*hostname_out).is_null() {
               12 as libc::c_int
           } else { 0 as libc::c_int };
}
#[c2rust::src_loc = "282:1"]
unsafe extern "C" fn clean_hostname(mut context: krb5_context,
                                    mut host: *const libc::c_char,
                                    mut cleanname_out: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cleanname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: krb5_error_code = 0;
    let mut l: size_t = 0;
    *cleanname_out = 0 as *mut libc::c_char;
    if !host.is_null() {
        cleanname = strdup(host);
        if cleanname.is_null() { return 12 as libc::c_int }
    } else {
        ret = krb5int_get_fq_local_hostname(&mut cleanname);
        if ret != 0 { return ret }
    }
    /* Fold to lowercase. */
    p = cleanname;
    while *p != 0 {
        if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as
                                          isize) as libc::c_int &
               _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            *p = tolower(*p as libc::c_uchar as libc::c_int) as libc::c_char
        }
        p = p.offset(1)
    }
    /* Strip off trailing dot. */
    l = strlen(cleanname);
    if l > 0 as libc::c_int as libc::c_ulong &&
           *cleanname.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                 as isize) as libc::c_int == '.' as i32 {
        *cleanname.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
                              isize) = '\u{0}' as i32 as libc::c_char
    }
    *cleanname_out = cleanname;
    return 0 as libc::c_int;
}
/* Return true if name appears to be an IPv4 or IPv6 address. */
#[no_mangle]
#[c2rust::src_loc = "317:1"]
pub unsafe extern "C" fn k5_is_numeric_address(mut name: *const libc::c_char)
 -> krb5_boolean {
    let mut ndots: libc::c_int = 0 as libc::c_int;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    /* If name contains only numbers and three dots, consider it to be an IPv4
     * address. */
    if strspn(name, b"01234567890.\x00" as *const u8 as *const libc::c_char)
           == strlen(name) {
        p = name;
        while *p != 0 {
            if *p as libc::c_int == '.' as i32 { ndots += 1 }
            p = p.offset(1)
        }
        if ndots == 3 as libc::c_int {
            return 1 as libc::c_int as krb5_boolean
        }
    }
    /* If name contains a colon, consider it to be an IPv6 address. */
    if !strchr(name, ':' as i32).is_null() {
        return 1 as libc::c_int as krb5_boolean
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* Construct a one-element realm list containing a copy of realm. */
#[no_mangle]
#[c2rust::src_loc = "342:1"]
pub unsafe extern "C" fn k5_make_realmlist(mut realm: *const libc::c_char,
                                           mut realms_out:
                                               *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut realms: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    *realms_out = 0 as *mut *mut libc::c_char;
    realms =
        calloc(2 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) as
            *mut *mut libc::c_char;
    if realms.is_null() { return 12 as libc::c_int }
    let ref mut fresh5 = *realms.offset(0 as libc::c_int as isize);
    *fresh5 = strdup(realm);
    if (*realms.offset(0 as libc::c_int as isize)).is_null() {
        free(realms as *mut libc::c_void);
        return 12 as libc::c_int
    }
    *realms_out = realms;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "360:1"]
pub unsafe extern "C" fn krb5_get_host_realm(mut context: krb5_context,
                                             mut host: *const libc::c_char,
                                             mut realms_out:
                                                 *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut hp: *mut *mut hostrealm_module_handle =
        0 as *mut *mut hostrealm_module_handle;
    let mut realms: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cleanname: *mut libc::c_char = 0 as *mut libc::c_char;
    *realms_out = 0 as *mut *mut libc::c_char;
    if (*context).hostrealm_handles.is_null() {
        ret = load_hostrealm_modules(context);
        if ret != 0 {
            current_block = 3035409287871002607;
        } else { current_block = 11875828834189669668; }
    } else { current_block = 11875828834189669668; }
    match current_block {
        11875828834189669668 => {
            ret = clean_hostname(context, host, &mut cleanname);
            if !(ret != 0) {
                /* Give each module a chance to determine the host's realms. */
                hp = (*context).hostrealm_handles;
                loop  {
                    if (*hp).is_null() {
                        current_block = 5143058163439228106;
                        break ;
                    }
                    ret = host_realm(context, *hp, cleanname, &mut realms);
                    if ret == 0 as libc::c_int {
                        ret = copy_list(realms, realms_out);
                        free_list(context, *hp, realms);
                        current_block = 3035409287871002607;
                        break ;
                    } else {
                        if ret as libc::c_long !=
                               -(1765328135 as libc::c_long) {
                            current_block = 3035409287871002607;
                            break ;
                        }
                        hp = hp.offset(1)
                    }
                }
                match current_block {
                    3035409287871002607 => { }
                    _ => {
                        /* Return a list containing the "referral realm" (an empty realm), as a
     * cue to try referrals. */
                        ret =
                            k5_make_realmlist(b"\x00" as *const u8 as
                                                  *const libc::c_char,
                                              realms_out)
                    }
                }
            }
        }
        _ => { }
    }
    free(cleanname as *mut libc::c_void);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "400:1"]
pub unsafe extern "C" fn krb5_get_fallback_host_realm(mut context:
                                                          krb5_context,
                                                      mut hdata:
                                                          *mut krb5_data,
                                                      mut realms_out:
                                                          *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut hp: *mut *mut hostrealm_module_handle =
        0 as *mut *mut hostrealm_module_handle;
    let mut realms: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut defrealm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cleanname: *mut libc::c_char = 0 as *mut libc::c_char;
    *realms_out = 0 as *mut *mut libc::c_char;
    /* Convert hdata into a string and clean it. */
    host =
        k5memdup0((*hdata).data as *const libc::c_void,
                  (*hdata).length as size_t, &mut ret) as *mut libc::c_char;
    if !host.is_null() {
        ret = clean_hostname(context, host, &mut cleanname);
        free(host as *mut libc::c_void);
        if !(ret != 0) {
            if (*context).hostrealm_handles.is_null() {
                ret = load_hostrealm_modules(context);
                if ret != 0 {
                    current_block = 4317932568550761545;
                } else { current_block = 17965632435239708295; }
            } else { current_block = 17965632435239708295; }
            match current_block {
                4317932568550761545 => { }
                _ =>
                /* Give each module a chance to determine the fallback realms. */
                {
                    hp = (*context).hostrealm_handles;
                    loop  {
                        if (*hp).is_null() {
                            current_block = 12124785117276362961;
                            break ;
                        }
                        ret =
                            fallback_realm(context, *hp, cleanname,
                                           &mut realms);
                        if ret == 0 as libc::c_int {
                            ret = copy_list(realms, realms_out);
                            free_list(context, *hp, realms);
                            current_block = 4317932568550761545;
                            break ;
                        } else {
                            if ret as libc::c_long !=
                                   -(1765328135 as libc::c_long) {
                                current_block = 4317932568550761545;
                                break ;
                            }
                            hp = hp.offset(1)
                        }
                    }
                    match current_block {
                        4317932568550761545 => { }
                        _ => {
                            /* Return a list containing the default realm. */
                            ret =
                                krb5_get_default_realm(context,
                                                       &mut defrealm);
                            if !(ret != 0) {
                                ret = k5_make_realmlist(defrealm, realms_out);
                                krb5_free_default_realm(context, defrealm);
                            }
                        }
                    }
                }
            }
        }
    }
    free(cleanname as *mut libc::c_void);
    return ret;
}
/* * @deprecated Replaced by krb5_auth_con_getrecvsubkey(). */
/* *
 * Retrieve the local sequence number from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] seqnumber       Local sequence number
 *
 * Retrieve the local sequence number from @a auth_context and return it in @a
 * seqnumber.  The #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag must be set in @a
 * auth_context for this function to be useful.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Retrieve the remote sequence number from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] seqnumber       Remote sequence number
 *
 * Retrieve the remote sequence number from @a auth_context and return it in @a
 * seqnumber.  The #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag must be set in @a
 * auth_context for this function to be useful.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Cause an auth context to use cipher state.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 *
 * Prepare @a auth_context to use cipher state when krb5_mk_priv() or
 * krb5_rd_priv() encrypt or decrypt data.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Set the replay cache in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] rcache           Replay cache haddle
 *
 * This function sets the replay cache in @a auth_context to @a rcache.  @a
 * rcache will be closed when @a auth_context is freed, so the caller should
 * relinguish that responsibility.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Retrieve the replay cache from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] rcache          Replay cache handle
 *
 * This function fetches the replay cache from @a auth_context.  The caller
 * should not close @a rcache.
 *
 * @retval 0 (always)
 */
/* *
 * Retrieve the authenticator from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] authenticator   Authenticator
 *
 * Use krb5_free_authenticator() to free @a authenticator when it is no longer
 * needed.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
/* *
 * Set checksum type in an an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] cksumtype        Checksum type
 *
 * This function sets the checksum type in @a auth_context to be used by
 * krb5_mk_req() for the authenticator checksum.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
/*
 * end "func-proto.h"
 */
/*
 * begin stuff from libos.h
 */
/* *
 * @brief Read a password from keyboard input.
 *
 * @param [in]     context      Library context
 * @param [in]     prompt       First user prompt when reading password
 * @param [in]     prompt2      Second user prompt (NULL to prompt only once)
 * @param [out]    return_pwd   Returned password
 * @param [in,out] size_return  On input, maximum size of password; on output,
 *                              size of password read
 *
 * This function reads a password from keyboard input and stores it in @a
 * return_pwd.  @a size_return should be set by the caller to the amount of
 * storage space available in @a return_pwd; on successful return, it will be
 * set to the length of the password read.
 *
 * @a prompt is printed to the terminal, followed by ": ", and then a password
 * is read from the keyboard.
 *
 * If @a prompt2 is NULL, the password is read only once.  Otherwise, @a
 * prompt2 is printed to the terminal and a second password is read.  If the
 * two passwords entered are not identical, KRB5_LIBOS_BADPWDMATCH is returned.
 *
 * Echoing is turned off when the password is read.
 *
 * @retval
 *  0   Success
 * @return
 * Error in reading or verifying the password
 * @return
 * Kerberos error codes
 */
/* *
 * Convert a principal name to a local name.
 *
 * @param [in]  context         Library context
 * @param [in]  aname           Principal name
 * @param [in]  lnsize_in       Space available in @a lname
 * @param [out] lname           Local name buffer to be filled in
 *
 * If @a aname does not correspond to any local account, KRB5_LNAME_NOTRANS is
 * returned.  If @a lnsize_in is too small for the local name,
 * KRB5_CONFIG_NOTENUFSPACE is returned.
 *
 * Local names, rather than principal names, can be used by programs that
 * translate to an environment-specific name (for example, a user account
 * name).
 *
 * @retval
 * 0  Success
 * @retval
 *  System errors
 * @return
 * Kerberos error codes
 */
/* *
 * Get the Kerberos realm names for a host.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Host name (or NULL)
 * @param [out] realmsp         Null-terminated list of realm names
 *
 * Fill in @a realmsp with a pointer to a null-terminated list of realm names.
 * If there are no known realms for the host, a list containing the referral
 * (empty) realm is returned.
 *
 * If @a host is NULL, the local host's realms are determined.
 *
 * Use krb5_free_host_realm() to release @a realmsp when it is no longer
 * needed.
 *
 * @retval
 *  0   Success
 * @retval
 *  ENOMEM  Insufficient memory
 * @return
 * Kerberos error codes
 */
/* *
 *
 * @param [in] context           Library context
 * @param [in] hdata             Host name (or NULL)
 * @param [out] realmsp          Null-terminated list of realm names
 *
 * Fill in @a realmsp with a pointer to a null-terminated list of realm names
 * obtained through heuristics or insecure resolution methods which have lower
 * priority than KDC referrals.
 *
 * If @a host is NULL, the local host's realms are determined.
 *
 * Use krb5_free_host_realm() to release @a realmsp when it is no longer
 * needed.
 */
/* *
 * Free the memory allocated by krb5_get_host_realm().
 *
 * @param [in] context          Library context
 * @param [in] realmlist        List of realm names to be released
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
#[no_mangle]
#[c2rust::src_loc = "449:1"]
pub unsafe extern "C" fn krb5_free_host_realm(mut context: krb5_context,
                                              mut list:
                                                  *const *mut libc::c_char)
 -> krb5_error_code {
    let mut p: *const *mut libc::c_char = 0 as *const *mut libc::c_char;
    p = list;
    while !p.is_null() && !(*p).is_null() {
        free(*p as *mut libc::c_void);
        p = p.offset(1)
    }
    free(list as *mut *mut libc::c_char as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* Get the system default realm using hostrealm modules. */
#[c2rust::src_loc = "461:1"]
unsafe extern "C" fn get_default_realm(mut context: krb5_context,
                                       mut realm_out: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut hp: *mut *mut hostrealm_module_handle =
        0 as *mut *mut hostrealm_module_handle;
    let mut realms: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    *realm_out = 0 as *mut libc::c_char;
    if (*context).hostrealm_handles.is_null() {
        ret = load_hostrealm_modules(context);
        if ret != 0 { return ret }
    }
    /* Give each module a chance to determine the default realm. */
    hp = (*context).hostrealm_handles;
    while !(*hp).is_null() {
        ret = default_realm(context, *hp, &mut realms);
        if ret == 0 as libc::c_int {
            if (*realms).is_null() {
                ret = -(1765328160 as libc::c_long) as krb5_error_code
            } else {
                *realm_out =
                    strdup(*realms.offset(0 as libc::c_int as isize));
                if (*realm_out).is_null() { ret = 12 as libc::c_int }
            }
            free_list(context, *hp, realms);
            return ret
        } else {
            if ret as libc::c_long != -(1765328135 as libc::c_long) {
                return ret
            }
        }
        hp = hp.offset(1)
    }
    return -(1765328160 as libc::c_long) as krb5_error_code;
}
/* *
 * Retrieve the default realm.
 *
 * @param [in]  context         Library context
 * @param [out] lrealm          Default realm name
 *
 * Retrieves the default realm to be used if no user-specified realm is
 * available.
 *
 * Use krb5_free_default_realm() to free @a lrealm when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
#[no_mangle]
#[c2rust::src_loc = "496:1"]
pub unsafe extern "C" fn krb5_get_default_realm(mut context: krb5_context,
                                                mut realm_out:
                                                    *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    *realm_out = 0 as *mut libc::c_char;
    if context.is_null() ||
           (*context).magic as libc::c_long != -(1760647388 as libc::c_long) {
        return -(1760647388 as libc::c_long) as krb5_error_code
    }
    if (*context).default_realm.is_null() {
        ret = get_default_realm(context, &mut (*context).default_realm);
        if ret != 0 { return ret }
    }
    *realm_out = strdup((*context).default_realm);
    return if (*realm_out).is_null() {
               12 as libc::c_int
           } else { 0 as libc::c_int };
}
/* *
 * Override the default realm for the specified context.
 *
 * @param [in]     context      Library context
 * @param [in]     lrealm       Realm name for the default realm
 *
 * If @a lrealm is NULL, clear the default realm setting.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
#[no_mangle]
#[c2rust::src_loc = "515:1"]
pub unsafe extern "C" fn krb5_set_default_realm(mut context: krb5_context,
                                                mut realm:
                                                    *const libc::c_char)
 -> krb5_error_code {
    if context.is_null() ||
           (*context).magic as libc::c_long != -(1760647388 as libc::c_long) {
        return -(1760647388 as libc::c_long) as krb5_error_code
    }
    if !(*context).default_realm.is_null() {
        free((*context).default_realm as *mut libc::c_void);
        (*context).default_realm = 0 as *mut libc::c_char
    }
    /* Allow the caller to clear the default realm setting by passing NULL. */
    if !realm.is_null() {
        (*context).default_realm = strdup(realm);
        if (*context).default_realm.is_null() { return 12 as libc::c_int }
    }
    return 0 as libc::c_int;
}
/* *
 * Free a default realm string returned by krb5_get_default_realm().
 *
 * @param [in] context          Library context
 * @param [in] lrealm           Realm to be freed
 */
#[no_mangle]
#[c2rust::src_loc = "536:1"]
pub unsafe extern "C" fn krb5_free_default_realm(mut context: krb5_context,
                                                 mut realm:
                                                     *mut libc::c_char) {
    free(realm as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "542:1"]
pub unsafe extern "C" fn k5_hostrealm_free_context(mut context:
                                                       krb5_context) {
    free_handles(context, (*context).hostrealm_handles);
    (*context).hostrealm_handles = 0 as *mut *mut hostrealm_module_handle;
}
