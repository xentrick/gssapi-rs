use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:46"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:46"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:46"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:46"]
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
        #[no_mangle]
        #[c2rust::src_loc = "6184:1"]
        pub fn krb5_free_host_realm(context: krb5_context,
                                    realmlist: *const *mut libc::c_char)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:46"]
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
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
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
                        krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:46"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:46"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:46"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:46"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/hostrealm_plugin.h:47"]
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
    /* ** Method type declarations ***/
    /* Optional: Initialize module data. */
    #[c2rust::src_loc = "72:1"]
    pub type krb5_hostrealm_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut krb5_hostrealm_moddata)
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
 * Mandatory (if any of the query methods are implemented): Release the memory
 * returned by one of the interface methods.
 */
    #[c2rust::src_loc = "115:1"]
    pub type krb5_hostrealm_free_list_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_hostrealm_moddata,
                                    _: *mut *mut libc::c_char) -> ()>;
    /* Optional: Release resources used by module data. */
    #[c2rust::src_loc = "120:1"]
    pub type krb5_hostrealm_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_hostrealm_moddata) -> ()>;
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
    /* hostrealm vtable for major version 1. */
    #[c2rust::src_loc = "124:1"]
    pub type krb5_hostrealm_vtable = *mut krb5_hostrealm_vtable_st;
    use super::krb5_h::{krb5_error_code, krb5_context};
    extern "C" {
        #[c2rust::src_loc = "67:16"]
        pub type krb5_hostrealm_moddata_st;
    }
    /* Mandatory: name of module. */
    /* Minor version 1 ends here. */
    /* KRB5_HOSTREALM_PLUGIN_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:46"]
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
#[c2rust::header_src = "/usr/include/string.h:46"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::types_h::__int32_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _profile_t,
                       krb5_free_host_realm};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, k5calloc, k5alloc, k5memdup0,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::hostrealm_plugin_h::{krb5_hostrealm_moddata,
                                   krb5_hostrealm_init_fn,
                                   krb5_hostrealm_host_realm_fn,
                                   krb5_hostrealm_fallback_realm_fn,
                                   krb5_hostrealm_default_realm_fn,
                                   krb5_hostrealm_free_list_fn,
                                   krb5_hostrealm_fini_fn,
                                   krb5_hostrealm_vtable_st,
                                   krb5_hostrealm_vtable,
                                   krb5_hostrealm_moddata_st};
use self::stdlib_h::{calloc, realloc, free};
use self::string_h::{strchr, strdup, memcpy};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/hostrealm/test/main.c - test module for host-realm interface */
/*
 * Copyright (C) 2010,2013 by the Massachusetts Institute of Technology.
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
 * This file implements two hostrealm modules named "test1" and "test2".
 *
 * The first module returns multi-element lists.  For host_realm and
 * fallback_realm, it returns a list of the host's components in order.  For
 * default_realm, it returns a list containing "one" and "two".
 *
 * The second module tests error handling.  For host_realm and fallback_realm,
 * it returns a fatal error on hosts beginning with 'z', a list containing "a"
 * for hosts beginning with 'a', and passes control to later modules otherwise.
 * For default_realm, it returns a fatal error.
 */
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn split_comps(mut context: krb5_context,
                                 mut data: krb5_hostrealm_moddata,
                                 mut host: *const libc::c_char,
                                 mut realms_out: *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut newptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut count: size_t = 0 as libc::c_int as size_t;
    *realms_out = 0 as *mut *mut libc::c_char;
    if *host as libc::c_int == '\u{0}' as i32 {
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    p = host;
    loop  {
        q = strchr(p, '.' as i32);
        word =
            if q.is_null() {
                strdup(p) as *mut libc::c_void
            } else {
                k5memdup0(p as *const libc::c_void,
                          q.wrapping_offset_from(p) as libc::c_long as size_t,
                          &mut ret)
            } as *mut libc::c_char;
        if word.is_null() { current_block = 14322980103485731342; break ; }
        newptr =
            realloc(list as *mut libc::c_void,
                    count.wrapping_add(2 as libc::c_int as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                           as
                                                                           libc::c_ulong))
                as *mut *mut libc::c_char;
        if newptr.is_null() { current_block = 14322980103485731342; break ; }
        list = newptr;
        let fresh0 = count;
        count = count.wrapping_add(1);
        let ref mut fresh1 = *list.offset(fresh0 as isize);
        *fresh1 = word;
        let ref mut fresh2 = *list.offset(count as isize);
        *fresh2 = 0 as *mut libc::c_char;
        word = 0 as *mut libc::c_char;
        if q.is_null() { current_block = 4956146061682418353; break ; }
        p = q.offset(1 as libc::c_int as isize)
    }
    match current_block {
        14322980103485731342 => {
            krb5_free_host_realm(context, list);
            free(word as *mut libc::c_void);
            return 12 as libc::c_int
        }
        _ => { *realms_out = list; return 0 as libc::c_int }
    };
}
#[c2rust::src_loc = "91:1"]
unsafe extern "C" fn multi_defrealm(mut context: krb5_context,
                                    mut data: krb5_hostrealm_moddata,
                                    mut realms_out:
                                        *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut one: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut two: *mut libc::c_char = 0 as *mut libc::c_char;
    *realms_out = 0 as *mut *mut libc::c_char;
    one = strdup(b"one\x00" as *const u8 as *const libc::c_char);
    if !one.is_null() {
        two = strdup(b"two\x00" as *const u8 as *const libc::c_char);
        if !two.is_null() {
            list =
                calloc(3 as libc::c_int as libc::c_ulong,
                       ::std::mem::size_of::<*mut libc::c_char>() as
                           libc::c_ulong) as *mut *mut libc::c_char;
            if !list.is_null() {
                let ref mut fresh3 = *list.offset(0 as libc::c_int as isize);
                *fresh3 = one;
                let ref mut fresh4 = *list.offset(1 as libc::c_int as isize);
                *fresh4 = two;
                let ref mut fresh5 = *list.offset(2 as libc::c_int as isize);
                *fresh5 = 0 as *mut libc::c_char;
                *realms_out = list;
                return 0 as libc::c_int
            }
        }
    }
    free(one as *mut libc::c_void);
    free(two as *mut libc::c_void);
    free(list as *mut libc::c_void);
    return 12 as libc::c_int;
}
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn maybe_realm(mut context: krb5_context,
                                 mut data: krb5_hostrealm_moddata,
                                 mut host: *const libc::c_char,
                                 mut realms_out: *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    *realms_out = 0 as *mut *mut libc::c_char;
    if *host as libc::c_int == 'z' as i32 {
        return -(1765328139 as libc::c_long) as krb5_error_code
    }
    if *host as libc::c_int != 'a' as i32 {
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    a = strdup(b"a\x00" as *const u8 as *const libc::c_char);
    if a.is_null() { return 12 as libc::c_int }
    list =
        calloc(2 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) as
            *mut *mut libc::c_char;
    if list.is_null() {
        free(a as *mut libc::c_void);
        return 12 as libc::c_int
    }
    let ref mut fresh6 = *list.offset(0 as libc::c_int as isize);
    *fresh6 = a;
    let ref mut fresh7 = *list.offset(1 as libc::c_int as isize);
    *fresh7 = 0 as *mut libc::c_char;
    *realms_out = list;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "145:1"]
unsafe extern "C" fn error(mut context: krb5_context,
                           mut data: krb5_hostrealm_moddata,
                           mut realms_out: *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    *realms_out = 0 as *mut *mut libc::c_char;
    return -(1765328139 as libc::c_long) as krb5_error_code;
}
#[c2rust::src_loc = "152:1"]
unsafe extern "C" fn free_realmlist(mut context: krb5_context,
                                    mut data: krb5_hostrealm_moddata,
                                    mut list: *mut *mut libc::c_char) {
    krb5_free_host_realm(context, list);
}
#[no_mangle]
#[c2rust::src_loc = "166:1"]
pub unsafe extern "C" fn hostrealm_test1_initvt(mut context: krb5_context,
                                                mut maj_ver: libc::c_int,
                                                mut min_ver: libc::c_int,
                                                mut vtable:
                                                    krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_hostrealm_vtable = 0 as *mut krb5_hostrealm_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_hostrealm_vtable;
    (*vt).name = b"test1\x00" as *const u8 as *const libc::c_char;
    (*vt).host_realm =
        Some(split_comps as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_hostrealm_moddata,
                                      _: *const libc::c_char,
                                      _: *mut *mut *mut libc::c_char)
                     -> krb5_error_code);
    (*vt).fallback_realm =
        Some(split_comps as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_hostrealm_moddata,
                                      _: *const libc::c_char,
                                      _: *mut *mut *mut libc::c_char)
                     -> krb5_error_code);
    (*vt).default_realm =
        Some(multi_defrealm as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_hostrealm_moddata,
                                      _: *mut *mut *mut libc::c_char)
                     -> krb5_error_code);
    (*vt).free_list =
        Some(free_realmlist as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_hostrealm_moddata,
                                      _: *mut *mut libc::c_char) -> ());
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "183:1"]
pub unsafe extern "C" fn hostrealm_test2_initvt(mut context: krb5_context,
                                                mut maj_ver: libc::c_int,
                                                mut min_ver: libc::c_int,
                                                mut vtable:
                                                    krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_hostrealm_vtable = 0 as *mut krb5_hostrealm_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_hostrealm_vtable;
    (*vt).name = b"test2\x00" as *const u8 as *const libc::c_char;
    (*vt).host_realm =
        Some(maybe_realm as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_hostrealm_moddata,
                                      _: *const libc::c_char,
                                      _: *mut *mut *mut libc::c_char)
                     -> krb5_error_code);
    (*vt).default_realm =
        Some(error as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_hostrealm_moddata,
                                      _: *mut *mut *mut libc::c_char)
                     -> krb5_error_code);
    (*vt).free_list =
        Some(free_realmlist as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_hostrealm_moddata,
                                      _: *mut *mut libc::c_char) -> ());
    return 0 as libc::c_int;
}
