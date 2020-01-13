use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
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
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:33"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "644:5"]
    pub struct C2RustUnnamed {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "657:5"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "691:12"]
    pub struct C2RustUnnamed_1 {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "703:12"]
    pub struct C2RustUnnamed_2 {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "639:1"]
    pub unsafe extern "C" fn store_16_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = val as uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "652:1"]
    pub unsafe extern "C" fn store_32_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_0)).i = val;
    }
    #[inline]
    #[c2rust::src_loc = "686:1"]
    pub unsafe extern "C" fn load_16_le(mut cvp: *const libc::c_void)
     -> libc::c_ushort {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return (*(p as *const C2RustUnnamed_1)).i;
    }
    #[inline]
    #[c2rust::src_loc = "698:1"]
    pub unsafe extern "C" fn load_32_le(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return (*(p as *const C2RustUnnamed_2)).i;
    }
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:33"]
pub mod krb5_h {
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
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
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
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
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
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
 * Convert a string principal name to a krb5_principal structure.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [out] principal_out   New principal
 *
 * Convert a string representation of a principal name to a krb5_principal
 * structure.
 *
 * A string representation of a Kerberos name consists of one or more principal
 * name components, separated by slashes, optionally followed by the \@
 * character and a realm name.  If the realm name is not specified, the local
 * realm is used.
 *
 * To use the slash and \@ symbols as part of a component (quoted) instead of
 * using them as a component separator or as a realm prefix), put a backslash
 * (\) character in front of the symbol.  Similarly, newline, tab, backspace,
 * and NULL characters can be included in a component by using @c n, @c t, @c b
 * or @c 0, respectively.
 *
 * @note The realm in a Kerberos @a name cannot contain slash, colon,
 * or NULL characters.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
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
    #[c2rust::src_loc = "2315:1"]
    pub unsafe extern "C" fn k5memdup(mut in_0: *const libc::c_void,
                                      mut len: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = k5alloc(len, code);
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:33"]
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
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn k5_buf_add_uint16_le(mut buf: *mut k5buf,
                                                  mut val: uint16_t) {
        let mut p: *mut libc::c_void =
            k5_buf_get_space(buf, 2 as libc::c_int as size_t);
        if !p.is_null() { store_16_le(val as libc::c_uint, p); };
    }
    #[inline]
    #[c2rust::src_loc = "138:1"]
    pub unsafe extern "C" fn k5_buf_add_uint32_le(mut buf: *mut k5buf,
                                                  mut val: uint32_t) {
        let mut p: *mut libc::c_void =
            k5_buf_get_space(buf, 4 as libc::c_int as size_t);
        if !p.is_null() { store_32_le(val, p); };
    }
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::k5_platform_h::{store_16_le, store_32_le};
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
        /* Add a C string to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn k5_buf_add(buf: *mut k5buf, data: *const libc::c_char);
        /* Add a counted series of bytes to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_buf_add_len(buf: *mut k5buf, data: *const libc::c_void,
                              len: size_t);
        /* Extend the length of buf by len and return a pointer to the reserved space,
 * to be filled in by the caller.  Return NULL on error. */
        #[no_mangle]
        #[c2rust::src_loc = "93:1"]
        pub fn k5_buf_get_space(buf: *mut k5buf, len: size_t)
         -> *mut libc::c_void;
        /* Return ENOMEM if buf is in an error state, 0 otherwise. */
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn k5_buf_status(buf: *mut k5buf) -> libc::c_int;
    }
    /* K5_BUF_H */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:35"]
pub mod kdb_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1990, 1991, 2016 by the Massachusetts Institute of Technology.
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
    /* KDC Database interface definitions */
    /* This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
    /* This version will be incremented when incompatible changes are made to the
 * KDB API, and will be kept in sync with the libkdb major version. */
    /* Salt types */
    /* #define KRB5_KDB_SALTTYPE_V4            1 */
    /* #define KRB5_KDB_SALTTYPE_AFS3          5 */
    /* Attributes */
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_t = *mut _osa_policy_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _osa_policy_ent_t {
        pub version: libc::c_int,
        pub name: *mut libc::c_char,
        pub pw_min_life: krb5_ui_4,
        pub pw_max_life: krb5_ui_4,
        pub pw_min_length: krb5_ui_4,
        pub pw_min_classes: krb5_ui_4,
        pub pw_history_num: krb5_ui_4,
        pub policy_refcnt: krb5_ui_4,
        pub pw_max_fail: krb5_ui_4,
        pub pw_failcnt_interval: krb5_ui_4,
        pub pw_lockout_duration: krb5_ui_4,
        pub attributes: krb5_ui_4,
        pub max_life: krb5_ui_4,
        pub max_renewable_life: krb5_ui_4,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    /* NOT saved */
    /* no longer used */
    /* Only valid if version > 1 */
    /* pwdMaxFailure */
    /* pwdFailureCountInterval */
    /* pwdLockoutDuration */
    /* Only valid if version > 2 */
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
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_rec = _osa_policy_ent_t;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_ui_4,
                        krb5_magic, krb5_flags, krb5_deltat, krb5_timestamp,
                        krb5_kvno, krb5_principal, krb5_context};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn krb5_db_free_principal(kcontext: krb5_context,
                                      entry: *mut krb5_db_entry);
        #[no_mangle]
        #[c2rust::src_loc = "837:1"]
        pub fn krb5_db_free_policy(kcontext: krb5_context,
                                   policy: osa_policy_ent_t);
    }
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-input.h:34"]
pub mod k5_input_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-input.h - k5input helper functions */
/*
 * Copyright (C) 2014 by the Massachusetts Institute of Technology.
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
 * The k5input module defines helpers for safely consuming a fixed-sized block
 * of memory.  If an overrun or allocation failure occurs at any step,
 * subsequent functions will return default values until the error is detected
 * by looking at the status field.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct k5input {
        pub ptr: *const libc::c_uchar,
        pub len: size_t,
        pub status: int32_t,
    }
    #[inline]
    #[c2rust::src_loc = "51:1"]
    pub unsafe extern "C" fn k5_input_init(mut in_0: *mut k5input,
                                           mut ptr: *const libc::c_void,
                                           mut len: size_t) {
        (*in_0).ptr = ptr as *const libc::c_uchar;
        (*in_0).len = len;
        (*in_0).status = 0 as libc::c_int;
    }
    /* Only set the status value of in if it hasn't already been set, so status
 * reflects the first thing to go wrong. */
    #[inline]
    #[c2rust::src_loc = "61:1"]
    pub unsafe extern "C" fn k5_input_set_status(mut in_0: *mut k5input,
                                                 mut status: int32_t) {
        if (*in_0).status == 0 { (*in_0).status = status };
    }
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn k5_input_get_bytes(mut in_0: *mut k5input,
                                                mut len: size_t)
     -> *const libc::c_uchar {
        if (*in_0).len < len { k5_input_set_status(in_0, 22 as libc::c_int); }
        if (*in_0).status != 0 { return 0 as *const libc::c_uchar }
        (*in_0).len =
            ((*in_0).len as libc::c_ulong).wrapping_sub(len) as size_t as
                size_t;
        (*in_0).ptr = (*in_0).ptr.offset(len as isize);
        return (*in_0).ptr.offset(-(len as isize));
    }
    #[inline]
    #[c2rust::src_loc = "94:1"]
    pub unsafe extern "C" fn k5_input_get_uint16_le(mut in_0: *mut k5input)
     -> uint16_t {
        let mut ptr: *const libc::c_uchar =
            k5_input_get_bytes(in_0, 2 as libc::c_int as size_t);
        return if ptr.is_null() {
                   0 as libc::c_int
               } else {
                   load_16_le(ptr as *const libc::c_void) as libc::c_int
               } as uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "115:1"]
    pub unsafe extern "C" fn k5_input_get_uint32_le(mut in_0: *mut k5input)
     -> uint32_t {
        let mut ptr: *const libc::c_uchar =
            k5_input_get_bytes(in_0, 4 as libc::c_int as size_t);
        return if ptr.is_null() {
                   0 as libc::c_int as libc::c_uint
               } else { load_32_le(ptr as *const libc::c_void) };
    }
    use super::stddef_h::size_t;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::k5_platform_h::{load_16_le, load_32_le};
    /* K5_BUF_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
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
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, C2RustUnnamed_1,
                              C2RustUnnamed_2, store_16_le, store_32_le,
                              load_16_le, load_32_le};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal, krb5_post_recv_fn,
                       krb5_context, krb5_pre_send_fn, krb5_trace_callback,
                       krb5_trace_info, _krb5_trace_info, krb5_prompt_type,
                       _profile_t, krb5_parse_name};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, k5memdup0, k5memdup, k5alloc,
                         k5calloc, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf,
                         k5_buf_add_uint16_le, k5_buf_add_uint32_le,
                         k5_buf_init_dynamic, k5_buf_add, k5_buf_add_len,
                         k5_buf_get_space, k5_buf_status};
pub use self::kdb_h::{krb5_tl_data, _krb5_tl_data, osa_policy_ent_t,
                      _osa_policy_ent_t, krb5_db_entry, _krb5_db_entry_new,
                      krb5_key_data, _krb5_key_data, osa_policy_ent_rec,
                      krb5_db_free_principal, krb5_db_free_policy};
pub use self::k5_input_h::{k5input, k5_input_init, k5_input_set_status,
                           k5_input_get_bytes, k5_input_get_uint16_le,
                           k5_input_get_uint32_le};
use self::stdlib_h::{calloc, free};
use self::string_h::{strlen, memcpy};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kdb/kdb_xdr.c */
/*
 * Copyright (C) 2018 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "38:1"]
unsafe extern "C" fn put_tl_data(mut buf: *mut k5buf,
                                 mut tl: *const krb5_tl_data) {
    while !tl.is_null() {
        k5_buf_add_uint16_le(buf, (*tl).tl_data_type as uint16_t);
        k5_buf_add_uint16_le(buf, (*tl).tl_data_length);
        k5_buf_add_len(buf, (*tl).tl_data_contents as *const libc::c_void,
                       (*tl).tl_data_length as size_t);
        tl = (*tl).tl_data_next
    };
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/kdb/lmdb/klmdb-int.h - internal declarations for LMDB KDB module */
/*
 * Copyright (C) 2018 by the Massachusetts Institute of Technology.
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
/* Length of a principal lockout record (three 32-bit fields) */
#[no_mangle]
#[c2rust::src_loc = "48:1"]
pub unsafe extern "C" fn klmdb_encode_princ(mut context: krb5_context,
                                            mut entry: *const krb5_db_entry,
                                            mut enc_out: *mut *mut uint8_t,
                                            mut len_out: *mut size_t)
 -> krb5_error_code {
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut kd: *const krb5_key_data = 0 as *const krb5_key_data;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    *enc_out = 0 as *mut uint8_t;
    *len_out = 0 as libc::c_int as size_t;
    k5_buf_init_dynamic(&mut buf);
    k5_buf_add_uint32_le(&mut buf, (*entry).attributes as uint32_t);
    k5_buf_add_uint32_le(&mut buf, (*entry).max_life as uint32_t);
    k5_buf_add_uint32_le(&mut buf, (*entry).max_renewable_life as uint32_t);
    k5_buf_add_uint32_le(&mut buf, (*entry).expiration as uint32_t);
    k5_buf_add_uint32_le(&mut buf, (*entry).pw_expiration as uint32_t);
    k5_buf_add_uint16_le(&mut buf, (*entry).n_tl_data as uint16_t);
    k5_buf_add_uint16_le(&mut buf, (*entry).n_key_data as uint16_t);
    put_tl_data(&mut buf, (*entry).tl_data);
    i = 0 as libc::c_int;
    while i < (*entry).n_key_data as libc::c_int {
        kd = &mut *(*entry).key_data.offset(i as isize) as *mut krb5_key_data;
        k5_buf_add_uint16_le(&mut buf, (*kd).key_data_ver as uint16_t);
        k5_buf_add_uint16_le(&mut buf, (*kd).key_data_kvno);
        j = 0 as libc::c_int;
        while j < (*kd).key_data_ver as libc::c_int {
            k5_buf_add_uint16_le(&mut buf,
                                 (*kd).key_data_type[j as usize] as uint16_t);
            k5_buf_add_uint16_le(&mut buf, (*kd).key_data_length[j as usize]);
            if (*kd).key_data_length[j as usize] as libc::c_int >
                   0 as libc::c_int {
                k5_buf_add_len(&mut buf,
                               (*kd).key_data_contents[j as usize] as
                                   *const libc::c_void,
                               (*kd).key_data_length[j as usize] as size_t);
            }
            j += 1
        }
        i += 1
    }
    if k5_buf_status(&mut buf) != 0 as libc::c_int {
        return 12 as libc::c_int
    }
    *enc_out = buf.data as *mut uint8_t;
    *len_out = buf.len;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "91:1"]
pub unsafe extern "C" fn klmdb_encode_princ_lockout(mut context: krb5_context,
                                                    mut entry:
                                                        *const krb5_db_entry,
                                                    mut buf: *mut uint8_t) {
    store_32_le((*entry).last_success as libc::c_uint,
                buf as *mut libc::c_void);
    store_32_le((*entry).last_failed as libc::c_uint,
                buf.offset(4 as libc::c_int as isize) as *mut libc::c_void);
    store_32_le((*entry).fail_auth_count,
                buf.offset(8 as libc::c_int as isize) as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "100:1"]
pub unsafe extern "C" fn klmdb_encode_policy(mut context: krb5_context,
                                             mut pol:
                                                 *const osa_policy_ent_rec,
                                             mut enc_out: *mut *mut uint8_t,
                                             mut len_out: *mut size_t)
 -> krb5_error_code {
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    *enc_out = 0 as *mut uint8_t;
    *len_out = 0 as libc::c_int as size_t;
    k5_buf_init_dynamic(&mut buf);
    k5_buf_add_uint32_le(&mut buf, (*pol).pw_min_life);
    k5_buf_add_uint32_le(&mut buf, (*pol).pw_max_life);
    k5_buf_add_uint32_le(&mut buf, (*pol).pw_min_length);
    k5_buf_add_uint32_le(&mut buf, (*pol).pw_min_classes);
    k5_buf_add_uint32_le(&mut buf, (*pol).pw_history_num);
    k5_buf_add_uint32_le(&mut buf, (*pol).pw_max_fail);
    k5_buf_add_uint32_le(&mut buf, (*pol).pw_failcnt_interval);
    k5_buf_add_uint32_le(&mut buf, (*pol).pw_lockout_duration);
    k5_buf_add_uint32_le(&mut buf, (*pol).attributes);
    k5_buf_add_uint32_le(&mut buf, (*pol).max_life);
    k5_buf_add_uint32_le(&mut buf, (*pol).max_renewable_life);
    if (*pol).allowed_keysalts.is_null() {
        k5_buf_add_uint32_le(&mut buf, 0 as libc::c_int as uint32_t);
    } else {
        k5_buf_add_uint32_le(&mut buf,
                             strlen((*pol).allowed_keysalts) as uint32_t);
        k5_buf_add(&mut buf, (*pol).allowed_keysalts);
    }
    k5_buf_add_uint16_le(&mut buf, (*pol).n_tl_data as uint16_t);
    put_tl_data(&mut buf, (*pol).tl_data);
    if k5_buf_status(&mut buf) != 0 as libc::c_int {
        return 12 as libc::c_int
    }
    *enc_out = buf.data as *mut uint8_t;
    *len_out = buf.len;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "140:1"]
unsafe extern "C" fn get_tl_data(mut in_0: *mut k5input, mut count: size_t,
                                 mut tl: *mut *mut krb5_tl_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut contents: *const uint8_t = 0 as *const uint8_t;
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < count {
        *tl =
            k5alloc(::std::mem::size_of::<krb5_tl_data>() as libc::c_ulong,
                    &mut ret) as *mut krb5_tl_data;
        if (*tl).is_null() { return ret }
        (**tl).tl_data_type = k5_input_get_uint16_le(in_0) as krb5_int16;
        (**tl).tl_data_length = k5_input_get_uint16_le(in_0);
        len = (**tl).tl_data_length as size_t;
        contents = k5_input_get_bytes(in_0, len);
        if contents.is_null() {
            return -(1780008439 as libc::c_long) as krb5_error_code
        }
        (**tl).tl_data_contents =
            k5memdup(contents as *const libc::c_void, len, &mut ret) as
                *mut krb5_octet;
        if (**tl).tl_data_contents.is_null() { return ret }
        tl = &mut (**tl).tl_data_next;
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "165:1"]
pub unsafe extern "C" fn klmdb_decode_princ(mut context: krb5_context,
                                            mut key: *const libc::c_void,
                                            mut key_len: size_t,
                                            mut enc: *const libc::c_void,
                                            mut enc_len: size_t,
                                            mut entry_out:
                                                *mut *mut krb5_db_entry)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut in_0: k5input =
        k5input{ptr: 0 as *const libc::c_uchar, len: 0, status: 0,};
    let mut entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut princname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut contents: *const uint8_t = 0 as *const uint8_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut kd: *mut krb5_key_data = 0 as *mut krb5_key_data;
    *entry_out = 0 as *mut krb5_db_entry;
    entry =
        k5alloc(::std::mem::size_of::<krb5_db_entry>() as libc::c_ulong,
                &mut ret) as *mut krb5_db_entry;
    if !entry.is_null() {
        princname = k5memdup0(key, key_len, &mut ret) as *mut libc::c_char;
        if !princname.is_null() {
            ret = krb5_parse_name(context, princname, &mut (*entry).princ);
            if !(ret != 0) {
                k5_input_init(&mut in_0, enc, enc_len);
                (*entry).attributes =
                    k5_input_get_uint32_le(&mut in_0) as krb5_flags;
                (*entry).max_life =
                    k5_input_get_uint32_le(&mut in_0) as krb5_deltat;
                (*entry).max_renewable_life =
                    k5_input_get_uint32_le(&mut in_0) as krb5_deltat;
                (*entry).expiration =
                    k5_input_get_uint32_le(&mut in_0) as krb5_timestamp;
                (*entry).pw_expiration =
                    k5_input_get_uint32_le(&mut in_0) as krb5_timestamp;
                (*entry).n_tl_data =
                    k5_input_get_uint16_le(&mut in_0) as krb5_int16;
                (*entry).n_key_data =
                    k5_input_get_uint16_le(&mut in_0) as krb5_int16;
                if ((*entry).n_tl_data as libc::c_int) < 0 as libc::c_int ||
                       ((*entry).n_key_data as libc::c_int) < 0 as libc::c_int
                   {
                    ret = -(1780008439 as libc::c_long) as krb5_error_code
                } else {
                    ret =
                        get_tl_data(&mut in_0, (*entry).n_tl_data as size_t,
                                    &mut (*entry).tl_data);
                    if !(ret != 0) {
                        if (*entry).n_key_data as libc::c_int >
                               0 as libc::c_int {
                            (*entry).key_data =
                                k5calloc((*entry).n_key_data as size_t,
                                         ::std::mem::size_of::<krb5_key_data>()
                                             as libc::c_ulong, &mut ret) as
                                    *mut krb5_key_data;
                            if (*entry).key_data.is_null() {
                                current_block = 5071510063984272789;
                            } else { current_block = 4068382217303356765; }
                        } else { current_block = 4068382217303356765; }
                        match current_block {
                            5071510063984272789 => { }
                            _ => {
                                i = 0 as libc::c_int;
                                's_134:
                                    loop  {
                                        if !(i <
                                                 (*entry).n_key_data as
                                                     libc::c_int) {
                                            current_block =
                                                14447253356787937536;
                                            break ;
                                        }
                                        kd =
                                            &mut *(*entry).key_data.offset(i
                                                                               as
                                                                               isize)
                                                as *mut krb5_key_data;
                                        (*kd).key_data_ver =
                                            k5_input_get_uint16_le(&mut in_0)
                                                as krb5_int16;
                                        (*kd).key_data_kvno =
                                            k5_input_get_uint16_le(&mut in_0);
                                        if ((*kd).key_data_ver as libc::c_int)
                                               < 0 as libc::c_int &&
                                               (*kd).key_data_ver as
                                                   libc::c_int >
                                                   2 as libc::c_int {
                                            ret =
                                                -(1780008422 as libc::c_long)
                                                    as krb5_error_code;
                                            current_block =
                                                5071510063984272789;
                                            break ;
                                        } else {
                                            j = 0 as libc::c_int;
                                            while j <
                                                      (*kd).key_data_ver as
                                                          libc::c_int {
                                                (*kd).key_data_type[j as
                                                                        usize]
                                                    =
                                                    k5_input_get_uint16_le(&mut in_0)
                                                        as krb5_int16;
                                                (*kd).key_data_length[j as
                                                                          usize]
                                                    =
                                                    k5_input_get_uint16_le(&mut in_0);
                                                len =
                                                    (*kd).key_data_length[j as
                                                                              usize]
                                                        as size_t;
                                                contents =
                                                    k5_input_get_bytes(&mut in_0,
                                                                       len);
                                                if contents.is_null() {
                                                    ret =
                                                        -(1780008439 as
                                                              libc::c_long) as
                                                            krb5_error_code;
                                                    current_block =
                                                        5071510063984272789;
                                                    break 's_134 ;
                                                } else {
                                                    if len >
                                                           0 as libc::c_int as
                                                               libc::c_ulong {
                                                        (*kd).key_data_contents[j
                                                                                    as
                                                                                    usize]
                                                            =
                                                            k5memdup(contents
                                                                         as
                                                                         *const libc::c_void,
                                                                     len,
                                                                     &mut ret)
                                                                as
                                                                *mut krb5_octet;
                                                        if (*kd).key_data_contents[j
                                                                                       as
                                                                                       usize].is_null()
                                                           {
                                                            current_block =
                                                                5071510063984272789;
                                                            break 's_134 ;
                                                        }
                                                    }
                                                    j += 1
                                                }
                                            }
                                            i += 1
                                        }
                                    }
                                match current_block {
                                    5071510063984272789 => { }
                                    _ => {
                                        ret = in_0.status;
                                        if !(ret != 0) {
                                            (*entry).len =
                                                38 as libc::c_int as
                                                    krb5_ui_2;
                                            *entry_out = entry;
                                            entry = 0 as *mut krb5_db_entry
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
    free(princname as *mut libc::c_void);
    krb5_db_free_principal(context, entry);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "253:1"]
pub unsafe extern "C" fn klmdb_decode_princ_lockout(mut context: krb5_context,
                                                    mut entry:
                                                        *mut krb5_db_entry,
                                                    mut buf: *const uint8_t) {
    (*entry).last_success =
        load_32_le(buf as *const libc::c_void) as krb5_timestamp;
    (*entry).last_failed =
        load_32_le(buf.offset(4 as libc::c_int as isize) as
                       *const libc::c_void) as krb5_timestamp;
    (*entry).fail_auth_count =
        load_32_le(buf.offset(8 as libc::c_int as isize) as
                       *const libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "262:1"]
pub unsafe extern "C" fn klmdb_decode_policy(mut context: krb5_context,
                                             mut key: *const libc::c_void,
                                             mut key_len: size_t,
                                             mut enc: *const libc::c_void,
                                             mut enc_len: size_t,
                                             mut pol_out:
                                                 *mut osa_policy_ent_t)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut pol: osa_policy_ent_t = 0 as osa_policy_ent_t;
    let mut in_0: k5input =
        k5input{ptr: 0 as *const libc::c_uchar, len: 0, status: 0,};
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    *pol_out = 0 as osa_policy_ent_t;
    pol =
        k5alloc(::std::mem::size_of::<_osa_policy_ent_t>() as libc::c_ulong,
                &mut ret) as osa_policy_ent_t;
    if !pol.is_null() {
        (*pol).name = k5memdup0(key, key_len, &mut ret) as *mut libc::c_char;
        if !(*pol).name.is_null() {
            k5_input_init(&mut in_0, enc, enc_len);
            (*pol).pw_min_life = k5_input_get_uint32_le(&mut in_0);
            (*pol).pw_max_life = k5_input_get_uint32_le(&mut in_0);
            (*pol).pw_min_length = k5_input_get_uint32_le(&mut in_0);
            (*pol).pw_min_classes = k5_input_get_uint32_le(&mut in_0);
            (*pol).pw_history_num = k5_input_get_uint32_le(&mut in_0);
            (*pol).pw_max_fail = k5_input_get_uint32_le(&mut in_0);
            (*pol).pw_failcnt_interval = k5_input_get_uint32_le(&mut in_0);
            (*pol).pw_lockout_duration = k5_input_get_uint32_le(&mut in_0);
            (*pol).attributes = k5_input_get_uint32_le(&mut in_0);
            (*pol).max_life = k5_input_get_uint32_le(&mut in_0);
            (*pol).max_renewable_life = k5_input_get_uint32_le(&mut in_0);
            len = k5_input_get_uint32_le(&mut in_0) as size_t;
            if len > 0 as libc::c_int as libc::c_ulong {
                str = k5_input_get_bytes(&mut in_0, len) as *mut libc::c_char;
                if str.is_null() {
                    ret = -(1780008439 as libc::c_long) as krb5_error_code;
                    current_block = 8121321173029633297;
                } else {
                    (*pol).allowed_keysalts =
                        k5memdup0(str as *const libc::c_void, len, &mut ret)
                            as *mut libc::c_char;
                    if (*pol).allowed_keysalts.is_null() {
                        current_block = 8121321173029633297;
                    } else { current_block = 15768484401365413375; }
                }
            } else { current_block = 15768484401365413375; }
            match current_block {
                8121321173029633297 => { }
                _ => {
                    (*pol).n_tl_data =
                        k5_input_get_uint16_le(&mut in_0) as krb5_int16;
                    ret =
                        get_tl_data(&mut in_0, (*pol).n_tl_data as size_t,
                                    &mut (*pol).tl_data);
                    if !(ret != 0) {
                        ret = in_0.status;
                        if !(ret != 0) {
                            *pol_out = pol;
                            return 0 as libc::c_int
                        }
                    }
                }
            }
        }
    }
    krb5_db_free_policy(context, pol);
    return ret;
}
