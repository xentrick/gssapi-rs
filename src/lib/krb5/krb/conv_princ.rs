use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:38"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:38"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:38"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:38"]
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
    /*
 * Per V5 spec on definition of principal types
 */
    /* *<  Name type not known */
    /* *< Just the name of the principal
                                      as in DCE, or for users */
    /* *< Service and other unique instance (krbtgt) */
    /* *< Service with host name as instance
                                      (telnet, rcommands) */
    /* *< Service with host as remaining components */
    /* *< Unique ID */
    /* *< PKINIT */
    /* *< Name in form of SMTP email name */
    /* *< Windows 2000 UPN */
    /* *< Well-known (special) principal */
    /* *< First component of
                                                NT_WELLKNOWN principals */
    /* *< Windows 2000 UPN and SID */
    /* *< NT 4 style name */
    /* *< NT 4 style name and SID */
    /* * Constant version of krb5_principal_data */
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
        #[c2rust::src_loc = "4025:1"]
        pub fn krb5_build_principal(context: krb5_context,
                                    princ: *mut krb5_principal,
                                    rlen: libc::c_uint,
                                    realm: *const libc::c_char, _: ...)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:38"]
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_context, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
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
        #[c2rust::src_loc = "2219:1"]
        pub fn krb5_get_realm_domain(_: krb5_context, _: *const libc::c_char,
                                     _: *mut *mut libc::c_char)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:38"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:38"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:38"]
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
        #[c2rust::src_loc = "77:1"]
        pub fn profile_free_list(list: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn profile_iterator_free(iter_p: *mut *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn profile_get_values(profile: profile_t,
                                  names: *const *const libc::c_char,
                                  ret_values: *mut *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "107:1"]
        pub fn profile_iterator(iter_p: *mut *mut libc::c_void,
                                ret_name: *mut *mut libc::c_char,
                                ret_value: *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn profile_iterator_create(profile: profile_t,
                                       names: *const *const libc::c_char,
                                       flags: libc::c_int,
                                       ret_iter: *mut *mut libc::c_void)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn profile_get_string(profile: profile_t,
                                  name: *const libc::c_char,
                                  subname: *const libc::c_char,
                                  subsubname: *const libc::c_char,
                                  def_val: *const libc::c_char,
                                  ret_string: *mut *mut libc::c_char)
         -> libc::c_long;
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/ctype.h:40"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:38"]
pub mod k5_platform_h {
    use super::stddef_h::size_t;
    extern "C" {
        /* Make the interfaces to getpwnam and getpwuid consistent.
   Model the wrappers on the POSIX thread-safe versions, but
   use the unsafe system versions if the safe ones don't exist
   or we can't figure out their interfaces.  */
        /* int k5_getpwnam_r(const char *, blah blah) */
        /* POSIX */
        /* no getpwnam_r, or can't figure out #args or return type */
        /* int k5_getpwuid_r(uid_t, blah blah) */
        /* POSIX */
        /* no getpwuid_r, or can't figure out #args or return type */
        /* Ensure, if possible, that the indicated file descriptor won't be
   kept open if we exec another process (e.g., launching a ccapi
   server).  If we don't know how to do it... well, just go about our
   business.  Probably most callers won't check the return status
   anyways.  */
        /* Macros make the Sun compiler happier, and all variants of this do a
   single evaluation of the argument, and fcntl and fileno should
   produce reasonable error messages on type mismatches, on any system
   with F_SETFD.  */
        /* Since the original ANSI C spec left it undefined whether or
   how you could copy around a va_list, C 99 added va_copy.
   For old implementations, let's do our best to fake it.

   XXX Doesn't yet handle implementations with __va_copy (early draft)
   or GCC's __builtin_va_copy.  */
        /* Do nothing.  */
        /* Provide strlcpy/strlcat interfaces. */
        #[no_mangle]
        #[c2rust::src_loc = "887:1"]
        pub fn krb5int_strlcpy(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/string.h:38"]
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
        #[c2rust::src_loc = "132:14"]
        pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
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
#[c2rust::header_src = "/usr/include/stdlib.h:38"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::types_h::__int32_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _profile_t,
                       krb5_build_principal};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5_get_realm_domain};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_release_string,
                          profile_free_list, profile_iterator_free,
                          profile_get_values, profile_iterator,
                          profile_iterator_create, profile_get_string};
pub use self::ctype_h::{_ISupper, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, __ctype_b_loc, tolower};
use self::k5_platform_h::krb5int_strlcpy;
use self::string_h::{strlen, strchr, strcmp, strncat, strncpy, memcmp,
                     memcpy};
use self::stdlib_h::{free, malloc};
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
#[c2rust::src_loc = "49:8"]
pub struct krb_convert {
    pub v4_str: *mut libc::c_char,
    pub v5_str: *mut libc::c_char,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "len", ty = "libc::c_uint", bits = "8..=15")]
    pub flags_len: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
/*
 * Kadmin doesn't do realm conversion because it's currently
 * kadmin/REALM.NAME.  Zephyr doesn't because it's just zephyr/zephyr.
 *
 * "Realm conversion" is a bit of a misnomer; really, the v5 name is
 * using a FQDN or something that looks like it, where the v4 name is
 * just using the first label.  Sometimes that second principal name
 * component is a hostname, sometimes the realm name, sometimes it's
 * neither.
 *
 * This list should probably be more configurable, and more than
 * likely on a per-realm basis, so locally-defined services can be
 * added, or not.
 */
// Initialized in run_static_initializers
#[c2rust::src_loc = "72:33"]
static mut sconv_list: [krb_convert; 35] =
    [krb_convert{v4_str: 0 as *mut libc::c_char,
                 v5_str: 0 as *mut libc::c_char,
                 flags_len: [0; 2],
                 c2rust_padding: [0; 6],}; 35];
/*
 * char *strnchr(s, c, n)
 *   char *s;
 *   char c;
 *   unsigned int n;
 *
 * returns a pointer to the first occurrence of character c in the
 * string s, or a NULL pointer if c does not occur in in the string;
 * however, at most the first n characters will be considered.
 *
 * This falls in the "should have been in the ANSI C library"
 * category. :-)
 */
#[c2rust::src_loc = "133:1"]
unsafe extern "C" fn strnchr(mut s: *mut libc::c_char, mut c: libc::c_int,
                             mut n: libc::c_uint) -> *mut libc::c_char {
    if n < 1 as libc::c_int as libc::c_uint { return 0 as *mut libc::c_char }
    loop  {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0 && *s as libc::c_int != 0) { break ; }
        if *s as libc::c_int == c { return s }
        s = s.offset(1)
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "151:1"]
pub unsafe extern "C" fn krb5_524_conv_principal(mut context: krb5_context,
                                                 mut princ:
                                                     krb5_const_principal,
                                                 mut name: *mut libc::c_char,
                                                 mut inst: *mut libc::c_char,
                                                 mut realm: *mut libc::c_char)
 -> krb5_error_code {
    let mut p: *const krb_convert = 0 as *const krb_convert;
    let mut compo: *const krb5_data = 0 as *const krb5_data;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_realm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_prealm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp_realm_len: libc::c_uint = 0;
    let mut retval: libc::c_int = 0;
    if (*context).profile.is_null() {
        return -(1765328249 as libc::c_long) as krb5_error_code
    }
    *inst = '\u{0}' as i32 as libc::c_char;
    *name = *inst;
    match (*princ).length {
        2 => {
            /* Check if this principal is listed in the table */
            compo =
                &mut *(*princ).data.offset(0 as libc::c_int as isize) as
                    *mut krb5_data;
            p = sconv_list.as_ptr();
            while !(*p).v4_str.is_null() {
                if (*p).len() == (*compo).length &&
                       memcmp((*p).v5_str as *const libc::c_void,
                              (*compo).data as *const libc::c_void,
                              (*compo).length as libc::c_ulong) ==
                           0 as libc::c_int {
                    /*
                 * It is, so set the new name now, and chop off
                 * instance's domain name if requested.
                 */
                    if krb5int_strlcpy(name, (*p).v4_str,
                                       40 as libc::c_int as size_t) >=
                           40 as libc::c_int as libc::c_ulong {
                        return -(1765328207 as libc::c_long) as
                                   krb5_error_code
                    }
                    if (*p).flags() as libc::c_int & 0x1 as libc::c_int != 0 {
                        compo =
                            &mut *(*princ).data.offset(1 as libc::c_int as
                                                           isize) as
                                *mut krb5_data;
                        c =
                            strnchr((*compo).data, '.' as i32,
                                    (*compo).length);
                        if c.is_null() ||
                               c.wrapping_offset_from((*compo).data) as
                                   libc::c_long >=
                                   (40 as libc::c_int - 1 as libc::c_int) as
                                       libc::c_long {
                            return -(1765328207 as libc::c_long) as
                                       krb5_error_code
                        }
                        memcpy(inst as *mut libc::c_void,
                               (*compo).data as *const libc::c_void,
                               c.wrapping_offset_from((*compo).data) as
                                   libc::c_long as size_t);
                        *inst.offset(c.wrapping_offset_from((*compo).data) as
                                         libc::c_long as isize) =
                            '\u{0}' as i32 as libc::c_char
                    }
                    break ;
                } else { p = p.offset(1) }
            }
            /* If inst isn't set, the service isn't listed in the table, */
        /* so just copy it. */
            if *inst as libc::c_int == '\u{0}' as i32 {
                compo =
                    &mut *(*princ).data.offset(1 as libc::c_int as isize) as
                        *mut krb5_data;
                if (*compo).length >=
                       (40 as libc::c_int - 1 as libc::c_int) as libc::c_uint
                   {
                    return -(1765328207 as libc::c_long) as krb5_error_code
                }
                if (*compo).length > 0 as libc::c_int as libc::c_uint {
                    memcpy(inst as *mut libc::c_void,
                           (*compo).data as *const libc::c_void,
                           (*compo).length as libc::c_ulong);
                }
                *inst.offset((*compo).length as isize) =
                    '\u{0}' as i32 as libc::c_char
            }
        }
        1 => { }
        _ => { return -(1765328207 as libc::c_long) as krb5_error_code }
    }
    /* fall through */
    /* name may have been set above; otherwise, just copy it */
    if *name as libc::c_int == '\u{0}' as i32 {
        compo =
            &mut *(*princ).data.offset(0 as libc::c_int as isize) as
                *mut krb5_data;
        if (*compo).length >= 40 as libc::c_int as libc::c_uint {
            return -(1765328207 as libc::c_long) as krb5_error_code
        }
        if (*compo).length > 0 as libc::c_int as libc::c_uint {
            memcpy(name as *mut libc::c_void,
                   (*compo).data as *const libc::c_void,
                   (*compo).length as libc::c_ulong);
        }
        *name.offset((*compo).length as isize) =
            '\u{0}' as i32 as libc::c_char
    }
    compo = &(*princ).realm;
    tmp_prealm =
        malloc((*compo).length.wrapping_add(1 as libc::c_int as libc::c_uint)
                   as libc::c_ulong) as *mut libc::c_char;
    if tmp_prealm.is_null() { return 12 as libc::c_int }
    strncpy(tmp_prealm, (*compo).data, (*compo).length as libc::c_ulong);
    *tmp_prealm.offset((*compo).length as isize) =
        '\u{0}' as i32 as libc::c_char;
    /* Ask for v4_realm corresponding to
       krb5 principal realm from krb5.conf realms stanza */
    retval =
        profile_get_string((*context).profile,
                           b"realms\x00" as *const u8 as *const libc::c_char,
                           tmp_prealm,
                           b"v4_realm\x00" as *const u8 as
                               *const libc::c_char, 0 as *const libc::c_char,
                           &mut tmp_realm) as
            libc::c_int; /* V4 instances are limited to 40 characters */
    free(tmp_prealm as *mut libc::c_void);
    if retval != 0 {
        return retval
    } else {
        if tmp_realm.is_null() {
            if (*compo).length >
                   (40 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
                return -(1765328207 as libc::c_long) as krb5_error_code
            }
            strncpy(realm, (*compo).data, (*compo).length as libc::c_ulong);
            *realm.offset((*compo).length as isize) =
                '\u{0}' as i32 as libc::c_char
        } else {
            tmp_realm_len = strlen(tmp_realm) as libc::c_uint;
            if tmp_realm_len >
                   (40 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
                profile_release_string(tmp_realm);
                return -(1765328207 as libc::c_long) as krb5_error_code
            }
            strncpy(realm, tmp_realm, tmp_realm_len as libc::c_ulong);
            *realm.offset(tmp_realm_len as isize) =
                '\u{0}' as i32 as libc::c_char;
            profile_release_string(tmp_realm);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "254:1"]
pub unsafe extern "C" fn krb5_425_conv_principal(mut context: krb5_context,
                                                 mut name:
                                                     *const libc::c_char,
                                                 mut instance:
                                                     *const libc::c_char,
                                                 mut realm:
                                                     *const libc::c_char,
                                                 mut princ:
                                                     *mut krb5_principal)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut p: *const krb_convert = 0 as *const krb_convert;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut retval: krb5_error_code = 0;
    let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut full_name: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut names: [*const libc::c_char; 5] = [0 as *const libc::c_char; 5];
    let mut names2: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    let mut iterator: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut v4realms: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut realm_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dummy_value: *mut libc::c_char = 0 as *mut libc::c_char;
    /* First, convert the realm, since the v4 realm is not necessarily the same as the v5 realm
       To do that, iterate over all the realms in the config file, looking for a matching
       v4_realm line */
    names2[0 as libc::c_int as usize] =
        b"realms\x00" as *const u8 as *const libc::c_char;
    names2[1 as libc::c_int as usize] = 0 as *const libc::c_char;
    retval =
        profile_iterator_create((*context).profile, names2.as_mut_ptr(),
                                0x1 as libc::c_int | 0x2 as libc::c_int,
                                &mut iterator) as krb5_error_code;
    while retval == 0 as libc::c_int {
        retval =
            profile_iterator(&mut iterator, &mut realm_name, &mut dummy_value)
                as krb5_error_code;
        if retval == 0 as libc::c_int && !realm_name.is_null() {
            names[0 as libc::c_int as usize] =
                b"realms\x00" as *const u8 as *const libc::c_char;
            names[1 as libc::c_int as usize] = realm_name;
            names[2 as libc::c_int as usize] =
                b"v4_realm\x00" as *const u8 as *const libc::c_char;
            names[3 as libc::c_int as usize] = 0 as *const libc::c_char;
            retval =
                profile_get_values((*context).profile, names.as_mut_ptr(),
                                   &mut v4realms) as krb5_error_code;
            if retval == 0 as libc::c_int && !v4realms.is_null() &&
                   !(*v4realms.offset(0 as libc::c_int as isize)).is_null() &&
                   strcmp(*v4realms.offset(0 as libc::c_int as isize), realm)
                       == 0 as libc::c_int {
                realm = realm_name;
                break ;
            } else if retval as libc::c_long == -(1429577725 as libc::c_long)
             {
                /* If it's not found, just keep going */
                retval = 0 as libc::c_int
            }
        } else if retval == 0 as libc::c_int && realm_name.is_null() {
            break ;
        }
        if !v4realms.is_null() {
            profile_free_list(v4realms);
            v4realms = 0 as *mut *mut libc::c_char
        }
        if !realm_name.is_null() {
            profile_release_string(realm_name);
            realm_name = 0 as *mut libc::c_char
        }
        if !dummy_value.is_null() {
            profile_release_string(dummy_value);
            dummy_value = 0 as *mut libc::c_char
        }
    }
    if !instance.is_null() {
        if *instance.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\u{0}' as i32 {
            instance = 0 as *const libc::c_char;
            current_block = 2109935844126326212;
        } else {
            p = sconv_list.as_ptr();
            loop  {
                if (*p).v4_str.is_null() {
                    current_block = 2109935844126326212;
                    break ;
                }
                if strcmp((*p).v4_str, name) == 0 {
                    current_block = 721385680381463314;
                    break ;
                }
                p = p.offset(1)
            }
            match current_block {
                2109935844126326212 => { }
                _ => {
                    name = (*p).v5_str;
                    if (*p).flags() as libc::c_int & 0x1 as libc::c_int != 0
                           && strchr(instance, '.' as i32).is_null() {
                        names[0 as libc::c_int as usize] =
                            b"realms\x00" as *const u8 as *const libc::c_char;
                        names[1 as libc::c_int as usize] = realm;
                        names[2 as libc::c_int as usize] =
                            b"v4_instance_convert\x00" as *const u8 as
                                *const libc::c_char;
                        names[3 as libc::c_int as usize] = instance;
                        names[4 as libc::c_int as usize] =
                            0 as *const libc::c_char;
                        retval =
                            profile_get_values((*context).profile,
                                               names.as_mut_ptr(),
                                               &mut full_name) as
                                krb5_error_code;
                        if retval == 0 as libc::c_int && !full_name.is_null()
                               &&
                               !(*full_name.offset(0 as libc::c_int as
                                                       isize)).is_null() {
                            instance =
                                *full_name.offset(0 as libc::c_int as isize);
                            current_block = 2109935844126326212;
                        } else {
                            strncpy(buf.as_mut_ptr(), instance,
                                    ::std::mem::size_of::<[libc::c_char; 256]>()
                                        as libc::c_ulong);
                            buf[(::std::mem::size_of::<[libc::c_char; 256]>()
                                     as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            retval =
                                krb5_get_realm_domain(context, realm,
                                                      &mut domain);
                            if retval != 0 {
                                current_block = 10804146325835088476;
                            } else {
                                if !domain.is_null() {
                                    cp = domain;
                                    while *cp != 0 {
                                        if *(*__ctype_b_loc()).offset(*cp as
                                                                          libc::c_uchar
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                                               as libc::c_int &
                                               _ISupper as libc::c_int as
                                                   libc::c_ushort as
                                                   libc::c_int != 0 {
                                            *cp =
                                                tolower(*cp as libc::c_uchar
                                                            as libc::c_int) as
                                                    libc::c_char
                                        }
                                        cp = cp.offset(1)
                                    }
                                    strncat(buf.as_mut_ptr(),
                                            b".\x00" as *const u8 as
                                                *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 256]>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong).wrapping_sub(strlen(buf.as_mut_ptr())));
                                    strncat(buf.as_mut_ptr(), domain,
                                            (::std::mem::size_of::<[libc::c_char; 256]>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong).wrapping_sub(strlen(buf.as_mut_ptr())));
                                    free(domain as *mut libc::c_void);
                                }
                                instance = buf.as_mut_ptr();
                                current_block = 2109935844126326212;
                            }
                        }
                    } else { current_block = 2109935844126326212; }
                }
            }
        }
    } else { current_block = 2109935844126326212; }
    match current_block {
        2109935844126326212 => {
            retval =
                krb5_build_principal(context, princ,
                                     strlen(realm) as libc::c_uint, realm,
                                     name, instance, 0 as *mut libc::c_void)
        }
        _ => { }
    }
    if !iterator.is_null() { profile_iterator_free(&mut iterator); }
    if !full_name.is_null() { profile_free_list(full_name); }
    if !v4realms.is_null() { profile_free_list(v4realms); }
    if !realm_name.is_null() { profile_release_string(realm_name); }
    if !dummy_value.is_null() { profile_release_string(dummy_value); }
    return retval;
}
unsafe extern "C" fn run_static_initializers() {
    sconv_list =
        [{
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"kadmin\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"kadmin\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 7]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"rcmd\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"host\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 5]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"discuss\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"discuss\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 8]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"rvdsrv\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"rvdsrv\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 7]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"sample\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"sample\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 7]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"olc\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"olc\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"pop\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"pop\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"sis\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"sis\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"rfs\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"rfs\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"imap\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"imap\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 5]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"ftp\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"ftp\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"ecat\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"ecat\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 5]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"daemon\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"daemon\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 7]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"gnats\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"gnats\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 6]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"moira\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"moira\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 6]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"prms\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"prms\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 5]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"mandarin\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"mandarin\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 9]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"register\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"register\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 9]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"changepw\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"changepw\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 9]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"sms\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"sms\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"afpserver\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"afpserver\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 10]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"gdss\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"gdss\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 5]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"news\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"news\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 5]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"abs\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"abs\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"nfs\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"nfs\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"tftp\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"tftp\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 5]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"zephyr\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"zephyr\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 7]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"http\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"http\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 5]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"khttp\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"khttp\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 6]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"pgpsigner\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"pgpsigner\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 10]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"irc\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"irc\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"mandarin-agent\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"mandarin-agent\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 15]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"write\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"write\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 6]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str:
                                 b"palladium\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             v5_str:
                                 b"palladium\x00" as *const u8 as
                                     *const libc::c_char as
                                     *mut libc::c_char,};
             init.set_flags(0x1 as libc::c_int as libc::c_uint);
             init.set_len((::std::mem::size_of::<[libc::c_char; 10]>() as
                               libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                               libc::c_ulong)
                              as libc::c_uint);
             init
         },
         {
             let mut init =
                 krb_convert{flags_len: [0; 2],
                             c2rust_padding: [0; 6],
                             v4_str: 0 as *mut libc::c_char,
                             v5_str: 0 as *mut libc::c_char,};
             init.set_flags(0 as libc::c_int as libc::c_uint);
             init.set_len(0 as libc::c_int as libc::c_uint);
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
