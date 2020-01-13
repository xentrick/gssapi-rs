use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:30"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:30"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:30"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/unistd.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:30"]
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
        /* *
 * Build a principal name using null-terminated strings.
 *
 * @param [in]  context         Library context
 * @param [out] princ           Principal name
 * @param [in]  rlen            Realm name length
 * @param [in]  realm           Realm name
 * @param [in]  ...             List of char * components, ending with NULL
 *
 * Call krb5_free_principal() to free @a princ when it is no longer needed.
 *
 * @note krb5_build_principal() and krb5_build_principal_alloc_va() perform the
 * same task.  krb5_build_principal() takes variadic arguments.
 * krb5_build_principal_alloc_va() takes a pre-computed @a varargs pointer.
 *
 * @code
 * Example of how to build principal H/S@R
 *     krb5_build_principal(context, &principal,
 *                          strlen("R"), "R", "H", "S", (char*)NULL);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4025:1"]
        pub fn krb5_build_principal(context: krb5_context,
                                    princ: *mut krb5_principal,
                                    rlen: libc::c_uint,
                                    realm: *const libc::c_char, _: ...)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6184:1"]
        pub fn krb5_free_host_realm(context: krb5_context,
                                    realmlist: *const *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6151:1"]
        pub fn krb5_get_host_realm(context: krb5_context,
                                   host: *const libc::c_char,
                                   realmsp: *mut *mut *mut libc::c_char)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:30"]
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
    use super::stddef_h::size_t;
    use super::string_h::memcpy;
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
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:30"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    extern "C" {
        /* Used by profile_init_flags(). */
        /* Allow module declaration */
        /*
 * Used by the profile iterator in prof_get.c
 */
        /* __cplusplus */
        /* path as C string */
        /* list of : separated paths, C string */
        /* path as C string */
        /* list of : separated paths, C string */
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn profile_release_string(str: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn profile_get_string(profile: profile_t,
                                  name: *const libc::c_char,
                                  subname: *const libc::c_char,
                                  subsubname: *const libc::c_char,
                                  def_val: *const libc::c_char,
                                  ret_string: *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn profile_get_boolean(profile: profile_t,
                                   name: *const libc::c_char,
                                   subname: *const libc::c_char,
                                   subsubname: *const libc::c_char,
                                   def_val: libc::c_int,
                                   ret_default: *mut libc::c_int)
         -> libc::c_long;
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/netdb.h:30"]
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
#[c2rust::header_src = "/usr/include/bits/socket.h:30"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:30"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/ctype.h:33"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:30"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:30"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:30"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:31"]
pub mod os_proto_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:1"]
        pub fn k5_primary_domain() -> *mut libc::c_char;
    }
    /* KRB5_LIBOS_INT_PROTO__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/fake-addrinfo.h:32"]
pub mod fake_addrinfo_h {
    use super::netdb_h::addrinfo;
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    use super::stddef_h::size_t;
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
        #[no_mangle]
        #[c2rust::src_loc = "219:1"]
        pub fn krb5int_getnameinfo(sa: *const sockaddr, salen: socklen_t,
                                   hbuf: *mut libc::c_char, hbuflen: size_t,
                                   sbuf: *mut libc::c_char, sbuflen: size_t,
                                   flags: libc::c_int) -> libc::c_int;
    }
    /* FAI_DEFINED */
}
pub use self::types_h::{__int32_t, __socklen_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::unistd_h::{socklen_t, gethostname};
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _profile_t, krb5_build_principal,
                       krb5_free_host_realm, krb5_get_host_realm};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, k5memdup0, k5alloc, k5calloc,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_release_string,
                          profile_get_string, profile_get_boolean};
pub use self::netdb_h::addrinfo;
pub use self::socket_h::sockaddr;
pub use self::sockaddr_h::sa_family_t;
pub use self::ctype_h::{_ISupper, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, __ctype_b_loc, tolower};
use self::stdlib_h::{calloc, free};
use self::stdio_h::asprintf;
use self::errno_h::__errno_location;
use self::string_h::{strlen, strchr, strdup, memset, memcpy};
use self::os_proto_h::k5_primary_domain;
use self::fake_addrinfo_h::{krb5int_getaddrinfo, krb5int_freeaddrinfo,
                            krb5int_getnameinfo};
#[c2rust::src_loc = "42:1"]
unsafe extern "C" fn use_reverse_dns(mut context: krb5_context)
 -> krb5_boolean {
    let mut ret: krb5_error_code = 0;
    let mut value: libc::c_int = 0;
    ret =
        profile_get_boolean((*context).profile,
                            b"libdefaults\x00" as *const u8 as
                                *const libc::c_char,
                            b"rdns\x00" as *const u8 as *const libc::c_char,
                            0 as *const libc::c_char, 1 as libc::c_int,
                            &mut value) as krb5_error_code;
    if ret != 0 { return 1 as libc::c_int as krb5_boolean }
    return value as krb5_boolean;
}
/* Append a domain suffix to host and return the result in allocated memory.
 * Return NULL if no suffix is configured or on failure. */
#[c2rust::src_loc = "59:1"]
unsafe extern "C" fn qualify_shortname(mut context: krb5_context,
                                       mut host: *const libc::c_char)
 -> *mut libc::c_char {
    let mut ret: krb5_error_code = 0;
    let mut fqdn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prof_domain: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut os_domain: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut domain: *const libc::c_char = 0 as *const libc::c_char;
    ret =
        profile_get_string((*context).profile,
                           b"libdefaults\x00" as *const u8 as
                               *const libc::c_char,
                           b"qualify_shortname\x00" as *const u8 as
                               *const libc::c_char, 0 as *const libc::c_char,
                           0 as *const libc::c_char, &mut prof_domain) as
            krb5_error_code;
    if ret != 0 { return 0 as *mut libc::c_char }
    if prof_domain.is_null() { os_domain = k5_primary_domain() }
    domain = if !prof_domain.is_null() { prof_domain } else { os_domain };
    if !domain.is_null() && *domain as libc::c_int != '\u{0}' as i32 {
        if asprintf(&mut fqdn as *mut *mut libc::c_char,
                    b"%s.%s\x00" as *const u8 as *const libc::c_char, host,
                    domain) < 0 as libc::c_int {
            fqdn = 0 as *mut libc::c_char
        }
    }
    profile_release_string(prof_domain);
    free(os_domain as *mut libc::c_void);
    return fqdn;
}
#[no_mangle]
#[c2rust::src_loc = "88:1"]
pub unsafe extern "C" fn k5_expand_hostname(mut context: krb5_context,
                                            mut host: *const libc::c_char,
                                            mut is_fallback: krb5_boolean,
                                            mut canonhost_out:
                                                *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut hint: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut namebuf: [libc::c_char; 1025] = [0; 1025];
    let mut qualified: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut err: libc::c_int = 0;
    let mut canonhost: *const libc::c_char = 0 as *const libc::c_char;
    let mut use_dns: krb5_boolean = 0;
    *canonhost_out = 0 as *mut libc::c_char;
    canonhost = host;
    use_dns =
        ((*context).dns_canonicalize_hostname as libc::c_uint ==
             CANONHOST_TRUE as libc::c_int as libc::c_uint ||
             is_fallback != 0 &&
                 (*context).dns_canonicalize_hostname as libc::c_uint ==
                     CANONHOST_FALLBACK as libc::c_int as libc::c_uint) as
            libc::c_int as krb5_boolean;
    if use_dns != 0 {
        /* Try a forward lookup of the hostname. */
        memset(&mut hint as *mut addrinfo as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
        hint.ai_flags = 0x2 as libc::c_int;
        err =
            krb5int_getaddrinfo(host, 0 as *const libc::c_char, &mut hint,
                                &mut ai);
        if err == -(10 as libc::c_int) {
            current_block = 8814467906442195055;
        } else {
            if err == 0 && !(*ai).ai_canonname.is_null() {
                canonhost = (*ai).ai_canonname
            }
            if err == 0 && use_reverse_dns(context) != 0 {
                /* Try a reverse lookup of the address. */
                err =
                    krb5int_getnameinfo((*ai).ai_addr, (*ai).ai_addrlen,
                                        namebuf.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 1025]>()
                                            as libc::c_ulong,
                                        0 as *mut libc::c_char,
                                        0 as libc::c_int as size_t,
                                        8 as libc::c_int);
                if err == -(10 as libc::c_int) {
                    current_block = 8814467906442195055;
                } else {
                    if err == 0 { canonhost = namebuf.as_mut_ptr() }
                    current_block = 17833034027772472439;
                }
            } else { current_block = 17833034027772472439; }
        }
    } else { current_block = 17833034027772472439; }
    match current_block {
        17833034027772472439 => {
            /* If we didn't use DNS and the name is just one component, try to add a
     * domain suffix. */
            if canonhost == host && strchr(host, '.' as i32).is_null() {
                qualified = qualify_shortname(context, host);
                if !qualified.is_null() { canonhost = qualified }
            }
            copy = strdup(canonhost);
            if !copy.is_null() {
                /* Convert the hostname to lower case. */
                p = copy;
                while *p as libc::c_int != '\u{0}' as i32 {
                    if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as
                                                      libc::c_int as isize) as
                           libc::c_int &
                           _ISupper as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 {
                        *p =
                            tolower(*p as libc::c_uchar as libc::c_int) as
                                libc::c_char
                    }
                    p = p.offset(1)
                }
                /* Remove any trailing dot. */
                if *copy.offset(0 as libc::c_int as isize) as libc::c_int !=
                       '\u{0}' as i32 {
                    p =
                        copy.offset(strlen(copy) as
                                        isize).offset(-(1 as libc::c_int as
                                                            isize));
                    if *p as libc::c_int == '.' as i32 {
                        *p = '\u{0}' as i32 as libc::c_char
                    }
                }
                *canonhost_out = copy
            }
        }
        _ => { }
    }
    /* We only return success or ENOMEM. */
    if !ai.is_null() { krb5int_freeaddrinfo(ai); }
    free(qualified as *mut libc::c_void);
    return if (*canonhost_out).is_null() {
               12 as libc::c_int
           } else { 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "160:1"]
pub unsafe extern "C" fn krb5_expand_hostname(mut context: krb5_context,
                                              mut host: *const libc::c_char,
                                              mut canonhost_out:
                                                  *mut *mut libc::c_char)
 -> krb5_error_code {
    return k5_expand_hostname(context, host, 0 as libc::c_int as krb5_boolean,
                              canonhost_out);
}
/* If hostname appears to have a :port or :instance trailer (used in MSSQLSvc
 * principals), return a pointer to the separator.  Otherwise return NULL. */
#[c2rust::src_loc = "169:1"]
unsafe extern "C" fn find_trailer(mut hostname: *const libc::c_char)
 -> *const libc::c_char {
    let mut p: *const libc::c_char = strchr(hostname, ':' as i32);
    /* Look for a single colon followed by one or more characters.  An IPv6
     * address will have more than one colon, so don't accept that. */
    if p.is_null() ||
           *p.offset(1 as libc::c_int as isize) as libc::c_int ==
               '\u{0}' as i32 ||
           !strchr(p.offset(1 as libc::c_int as isize), ':' as i32).is_null()
       {
        return 0 as *const libc::c_char
    }
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "181:1"]
pub unsafe extern "C" fn krb5_sname_to_principal(mut context: krb5_context,
                                                 mut hostname:
                                                     *const libc::c_char,
                                                 mut sname:
                                                     *const libc::c_char,
                                                 mut type_0: krb5_int32,
                                                 mut princ_out:
                                                     *mut krb5_principal)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut realm: *const libc::c_char = 0 as *const libc::c_char;
    let mut trailer: *const libc::c_char = 0 as *const libc::c_char;
    let mut hrealms: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut canonhost: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostonly: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut concat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut localname: [libc::c_char; 64] = [0; 64];
    *princ_out = 0 as krb5_principal;
    if type_0 != 0 as libc::c_int && type_0 != 3 as libc::c_int {
        return -(1765328166 as libc::c_long) as krb5_error_code
    }
    /* If hostname is NULL, use the local hostname. */
    if hostname.is_null() {
        if gethostname(localname.as_mut_ptr(), 64 as libc::c_int as size_t) !=
               0 as libc::c_int {
            return *__errno_location()
        }
        hostname = localname.as_mut_ptr()
    }
    /* If sname is NULL, use "host". */
    if sname.is_null() {
        sname = b"host\x00" as *const u8 as *const libc::c_char
    }
    /* If there is a trailer, remove it for now. */
    trailer = find_trailer(hostname);
    if !trailer.is_null() {
        hostonly =
            k5memdup0(hostname as *const libc::c_void,
                      trailer.wrapping_offset_from(hostname) as libc::c_long
                          as size_t, &mut ret) as *mut libc::c_char;
        if hostonly.is_null() {
            current_block = 13680415504322852222;
        } else { hostname = hostonly; current_block = 7149356873433890176; }
    } else { current_block = 7149356873433890176; }
    match current_block {
        7149356873433890176 =>
        /* Canonicalize the hostname if appropriate. */
        {
            if type_0 == 3 as libc::c_int {
                ret = krb5_expand_hostname(context, hostname, &mut canonhost);
                if ret != 0 {
                    current_block = 13680415504322852222;
                } else {
                    hostname = canonhost;
                    current_block = 224731115979188411;
                }
            } else { current_block = 224731115979188411; }
            match current_block {
                13680415504322852222 => { }
                _ => {
                    /* Find the realm of the host. */
                    ret =
                        krb5_get_host_realm(context, hostname, &mut hrealms);
                    if !(ret != 0) {
                        if (*hrealms.offset(0 as libc::c_int as
                                                isize)).is_null() {
                            ret =
                                -(1765328167 as libc::c_long) as
                                    krb5_error_code
                        } else {
                            realm =
                                *hrealms.offset(0 as libc::c_int as isize);
                            /* If there was a trailer, put it back on the end. */
                            if !trailer.is_null() {
                                if asprintf(&mut concat as
                                                *mut *mut libc::c_char,
                                            b"%s%s\x00" as *const u8 as
                                                *const libc::c_char, hostname,
                                            trailer) < 0 as libc::c_int {
                                    ret = 12 as libc::c_int;
                                    current_block = 13680415504322852222;
                                } else {
                                    hostname = concat;
                                    current_block = 17281240262373992796;
                                }
                            } else { current_block = 17281240262373992796; }
                            match current_block {
                                13680415504322852222 => { }
                                _ => {
                                    ret =
                                        krb5_build_principal(context,
                                                             &mut princ as
                                                                 *mut krb5_principal,
                                                             strlen(realm) as
                                                                 libc::c_uint,
                                                             realm, sname,
                                                             hostname,
                                                             0 as
                                                                 *mut libc::c_void
                                                                 as
                                                                 *mut libc::c_char);
                                    if !(ret != 0) {
                                        (*princ).type_0 = type_0;
                                        *princ_out = princ
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    free(hostonly as *mut libc::c_void);
    free(canonhost as *mut libc::c_void);
    free(concat as *mut libc::c_void);
    krb5_free_host_realm(context, hrealms);
    return ret;
}
