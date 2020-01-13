use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:28"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:28"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:28"]
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
    #[c2rust::src_loc = "702:1"]
    pub type krb5_os_context = *mut _krb5_os_context;
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32};
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
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:28"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:28"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:28"]
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
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn secure_getenv(__name: *const libc::c_char)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:30"]
pub mod os_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "163:1"]
        pub fn k5_expand_path_tokens(context: krb5_context,
                                     path_in: *const libc::c_char,
                                     path_out: *mut *mut libc::c_char)
         -> krb5_error_code;
    }
    /* KRB5_LIBOS_INT_PROTO__ */
}
pub use self::types_h::__int32_t;
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _profile_t};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_os_context, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_release_string,
                          profile_get_string};
use self::stdlib_h::{free, secure_getenv};
use self::string_h::strdup;
use self::os_proto_h::k5_expand_path_tokens;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/os/ccdefname.c - Return default credential cache name */
/*
 * Copyright 1990, 2007 by the Massachusetts Institute of Technology.
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
/* not _WIN32 */
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn get_from_os(mut context: krb5_context) {
    k5_expand_path_tokens(context,
                          b"FILE:/tmp/krb5cc_%{uid}\x00" as *const u8 as
                              *const libc::c_char,
                          &mut (*context).os_context.default_ccname);
}
/* not _WIN32 */
#[c2rust::src_loc = "243:1"]
unsafe extern "C" fn set_for_os(mut name: *const libc::c_char) {
    /* No implementation at present. */
}
/*
 * Set the default ccache name for all processes for the current user
 * (and the current context)
 */
#[no_mangle]
#[c2rust::src_loc = "254:1"]
pub unsafe extern "C" fn krb5int_cc_user_set_default_name(mut context:
                                                              krb5_context,
                                                          mut name:
                                                              *const libc::c_char)
 -> krb5_error_code {
    let mut err: krb5_error_code = 0;
    err = krb5_cc_set_default_name(context, name);
    if err != 0 { return err }
    set_for_os(name);
    return 0 as libc::c_int;
}
/* *
 * Set the default credential cache name.
 *
 * @param [in] context          Library context
 * @param [in] name             Default credential cache name or NULL
 *
 * Set the default credential cache name to @a name for future operations using
 * @a context.  If @a name is NULL, clear any previous application-set default
 * name and forget any cached value of the default name for @a context.
 *
 * Calls to this function invalidate the result of any previous calls to
 * krb5_cc_default_name() using @a context.
 *
 * @retval
 *  0  Success
 * @retval
 *  KV5M_CONTEXT          Bad magic number for @c _krb5_context structure
 * @return
 * Kerberos error codes
 */
#[no_mangle]
#[c2rust::src_loc = "266:1"]
pub unsafe extern "C" fn krb5_cc_set_default_name(mut context: krb5_context,
                                                  mut name:
                                                      *const libc::c_char)
 -> krb5_error_code {
    let mut os_ctx: krb5_os_context = 0 as *mut _krb5_os_context;
    let mut new_ccname: *mut libc::c_char = 0 as *mut libc::c_char;
    if context.is_null() ||
           (*context).magic as libc::c_long != -(1760647388 as libc::c_long) {
        return -(1760647388 as libc::c_long) as krb5_error_code
    }
    if !name.is_null() {
        new_ccname = strdup(name);
        if new_ccname.is_null() { return 12 as libc::c_int }
    }
    /* Free the old ccname and store the new one. */
    os_ctx = &mut (*context).os_context;
    free((*os_ctx).default_ccname as *mut libc::c_void);
    (*os_ctx).default_ccname = new_ccname;
    return 0 as libc::c_int;
}
/* *
 * Initialize an empty @c krb5_keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  length          Length of keyblock (or 0)
 * @param [out] out             New keyblock structure
 *
 * Initialize a new keyblock and allocate storage for the contents of the key.
 * It is legal to pass in a length of 0, in which case contents are left
 * unallocated.  Use krb5_free_keyblock() to free @a out when it is no longer
 * needed.
 *
 * @note If @a length is set to 0, contents are left unallocated.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Copy a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Keyblock to be copied
 * @param [out] to              Copy of keyblock @a from
 *
 * This function creates a new keyblock with the same contents as @a from.  Use
 * krb5_free_keyblock() to free @a to when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Copy the contents of a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Key to be copied
 * @param [out] to              Output key
 *
 * This function copies the contents of @a from to @a to.  Use
 * krb5_free_keyblock_contents() to free @a to when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Copy a krb5_creds structure.
 *
 * @param [in]  context         Library context
 * @param [in]  incred          Credentials structure to be copied
 * @param [out] outcred         Copy of @a incred
 *
 * This function creates a new credential with the contents of @a incred.  Use
 * krb5_free_creds() to free @a outcred when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Copy a krb5_data object.
 *
 * @param [in]  context           Library context
 * @param [in]  indata            Data object to be copied
 * @param [out] outdata           Copy of @a indata
 *
 * This function creates a new krb5_data object with the contents of @a indata.
 * Use krb5_free_data() to free @a outdata when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Copy a principal.
 *
 * @param [in]  context         Library context
 * @param [in]  inprinc         Principal to be copied
 * @param [out] outprinc        Copy of @a inprinc
 *
 * This function creates a new principal structure with the contents of @a
 * inprinc.  Use krb5_free_principal() to free @a outprinc when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Copy an array of addresses.
 *
 * @param [in]  context         Library context
 * @param [in]  inaddr          Array of addresses to be copied
 * @param [out] outaddr         Copy of array of addresses
 *
 * This function creates a new address array containing a copy of @a inaddr.
 * Use krb5_free_addresses() to free @a outaddr when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Copy a krb5_ticket structure.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Ticket to be copied
 * @param [out] pto             Copy of ticket
 *
 * This function creates a new krb5_ticket structure containing the contents of
 * @a from.  Use krb5_free_ticket() to free @a pto when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Copy an authorization data list.
 *
 * @param [in]  context         Library context
 * @param [in]  in_authdat      List of @a krb5_authdata structures
 * @param [out] out             New array of @a krb5_authdata structures
 *
 * This function creates a new authorization data list containing a copy of @a
 * in_authdat, which must be null-terminated.  Use krb5_free_authdata() to free
 * @a out when it is no longer needed.
 *
 * @note The last array entry in @a in_authdat must be a NULL pointer.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Find authorization data elements.
 *
 * @param [in]  context         Library context
 * @param [in]  ticket_authdata Authorization data list from ticket
 * @param [in]  ap_req_authdata Authorization data list from AP request
 * @param [in]  ad_type         Authorization data type to find
 * @param [out] results         List of matching entries
 *
 * This function searches @a ticket_authdata and @a ap_req_authdata for
 * elements of type @a ad_type.  Either input list may be NULL, in which case
 * it will not be searched; otherwise, the input lists must be terminated by
 * NULL entries.  This function will search inside AD-IF-RELEVANT containers if
 * found in either list.  Use krb5_free_authdata() to free @a results when it
 * is no longer needed.
 *
 * @version New in 1.10
 */
/* *
 * Merge two authorization data lists into a new list.
 *
 * @param [in]  context         Library context
 * @param [in]  inauthdat1      First list of @a krb5_authdata structures
 * @param [in]  inauthdat2      Second list of @a krb5_authdata structures
 * @param [out] outauthdat      Merged list of @a krb5_authdata structures
 *
 * Merge two authdata arrays, such as the array from a ticket
 * and authenticator.
 * Use krb5_free_authdata() to free @a outauthdat when it is no longer needed.
 *
 * @note The last array entry in @a inauthdat1 and @a inauthdat2
 * must be a NULL pointer.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Copy a krb5_authenticator structure.
 *
 * @param [in]  context         Library context
 * @param [in]  authfrom        krb5_authenticator structure to be copied
 * @param [out] authto          Copy of krb5_authenticator structure
 *
 * This function creates a new krb5_authenticator structure with the content of
 * @a authfrom.  Use krb5_free_authenticator() to free @a authto when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Copy a krb5_checksum structure.
 *
 * @param [in]  context         Library context
 * @param [in]  ckfrom          Checksum to be copied
 * @param [out] ckto            Copy of krb5_checksum structure
 *
 * This function creates a new krb5_checksum structure with the contents of @a
 * ckfrom.  Use krb5_free_checksum() to free @a ckto when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Generate a replay cache object for server use and open it.
 *
 * @param [in]  context         Library context
 * @param [in]  piece           Unused (replay cache identifier)
 * @param [out] rcptr           Handle to an open rcache
 *
 * This function creates a handle to the default replay cache.  Use
 * krb5_rc_close() to close @a rcptr when it is no longer needed.
 *
 * @version Prior to release 1.18, this function creates a handle to a
 * different replay cache for each unique value of @a piece.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Build a principal name using length-counted strings.
 *
 * @param [in]  context  Library context
 * @param [out] princ    Principal name
 * @param [in]  rlen     Realm name length
 * @param [in]  realm    Realm name
 * @param [in]  ...      List of unsigned int/char * components, followed by 0
 *
 * This function creates a principal from a length-counted string and a
 * variable-length list of length-counted components.  The list of components
 * ends with the first 0 length argument (so it is not possible to specify an
 * empty component with this function).  Call krb5_free_principal() to free
 * allocated memory for principal when it is no longer needed.
 *
 * @code
 * Example of how to build principal WELLKNOWN/ANONYMOUS@R
 *     krb5_build_principal_ext(context, &principal, strlen("R"), "R",
 *         (unsigned int)strlen(KRB5_WELLKNOWN_NAMESTR),
 *         KRB5_WELLKNOWN_NAMESTR,
 *         (unsigned int)strlen(KRB5_ANONYMOUS_PRINCSTR),
 *         KRB5_ANONYMOUS_PRINCSTR, 0);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
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
/* * @deprecated Replaced by krb5_build_principal_alloc_va(). */
/* *
 * Build a principal name, using a precomputed variable argument list
 *
 * @param [in]  context         Library context
 * @param [out] princ           Principal structure
 * @param [in]  rlen            Realm name length
 * @param [in]  realm           Realm name
 * @param [in]  ap              List of char * components, ending with NULL
 *
 * Similar to krb5_build_principal(), this function builds a principal name,
 * but its name components are specified as a va_list.
 *
 * Use krb5_free_principal() to deallocate @a princ when it is no longer
 * needed.
 *
 * @code
 * Function usage example:
 *   va_list ap;
 *   va_start(ap, realm);
 *   krb5_build_principal_alloc_va(context, princ, rlen, realm, ap);
 *   va_end(ap);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Convert a Kerberos V4 principal to a Kerberos V5 principal.
 *
 * @param [in]  context         Library context
 * @param [in]  name            V4 name
 * @param [in]  instance        V4 instance
 * @param [in]  realm           Realm
 * @param [out] princ           V5 principal
 *
 * This function builds a @a princ from V4 specification based on given input
 * @a name.instance\@realm.
 *
 * Use krb5_free_principal() to free @a princ when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Convert a Kerberos V5 principal to a Kerberos V4 principal.
 *
 * @param [in]  context         Library context
 * @param [in]  princ           V5 Principal
 * @param [out] name            V4 principal's name to be filled in
 * @param [out] inst            V4 principal's instance name to be filled in
 * @param [out] realm           Principal's realm name to be filled in
 *
 * This function separates a V5 principal @a princ into @a name, @a instance,
 * and @a realm.
 *
 * @retval
 *  0  Success
 * @retval
 *  KRB5_INVALID_PRINCIPAL   Invalid principal name
 * @retval
 *  KRB5_CONFIG_CANTOPEN     Can't open or find Kerberos configuration file
 * @return
 * Kerberos error codes
 */
/* *
 *@deprecated
 */
/* *
 * Convert a Kerberos V5 credentials to a Kerberos V4 credentials
 *
 * @note Not implemented
 *
 * @retval KRB524_KRB4_DISABLED (always)
 */
/* libkt.spec */
/* *
 * Get a handle for a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Name of the key table
 * @param [out] ktid            Key table handle
 *
 * Resolve the key table name @a name and set @a ktid to a handle identifying
 * the key table.  Use krb5_kt_close() to free @a ktid when it is no longer
 * needed.
 *
 * @a name must be of the form @c type:residual, where @a type must be a type
 * known to the library and @a residual portion should be specific to the
 * particular keytab type.  If no @a type is given, the default is @c FILE.
 *
 * If @a name is of type @c FILE, the keytab file is not opened by this call.
 *
 * @code
 *  Example: krb5_kt_resolve(context, "FILE:/tmp/filename", &ktid);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Duplicate keytab handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Key table handle to be duplicated
 * @param [out] out             Key table handle
 *
 * Create a new handle referring to the same key table as @a in.  The new
 * handle and @a in can be closed independently.
 *
 * @version New in 1.12
 */
/* *
 * Get the default key table name.
 *
 * @param [in]     context      Library context
 * @param [out]    name         Default key table name
 * @param [in]     name_size    Space available in @a name
 *
 * Fill @a name with the name of the default key table for @a context.
 *
 * @sa MAX_KEYTAB_NAME_LEN
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_CONFIG_NOTENUFSPACE Buffer is too short
 * @return
 * Kerberos error codes
 */
/* *
 * Resolve the default key table.
 *
 * @param [in]  context         Library context
 * @param [out] id              Key table handle
 *
 * Set @a id to a handle to the default key table.  The key table is not
 * opened.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Resolve the default client key table.
 *
 * @param [in]     context      Library context
 * @param [out]    keytab_out   Key table handle
 *
 * Fill @a keytab_out with a handle to the default client key table.
 *
 * @version New in 1.11
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Free the contents of a key table entry.
 *
 * @param [in] context          Library context
 * @param [in] entry            Key table entry whose contents are to be freed
 *
 * @note The pointer is not freed.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
/* * @deprecated Use krb5_free_keytab_entry_contents instead. */
/* remove and add are functions, so that they can return NOWRITE
   if not a writable keytab */
/* *
 * Remove an entry from a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to remove from key table
 *
 * @retval
 * 0 Success
 * @retval
 *  KRB5_KT_NOWRITE     Key table is not writable
 * @return
 * Kerberos error codes
 */
/* *
 * Add a new entry to a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to be added
 *
 * @retval
 * 0  Success
 * @retval
 *  ENOMEM    Insufficient memory
 * @retval
 *  KRB5_KT_NOWRITE  Key table is not writeable
 * @return
 * Kerberos error codes
 */
/* *
 * Convert a principal name into the default salt for that principal.
 *
 * @param [in]  context         Library context
 * @param [in]  pr              Principal name
 * @param [out] ret             Default salt for @a pr to be filled in
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* librc.spec--see rcache.h */
/* libcc.spec */
/* *
 * Resolve a credential cache name.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Credential cache name to be resolved
 * @param [out] cache           Credential cache handle
 *
 * Fills in @a cache with a @a cache handle that corresponds to the name in @a
 * name.  @a name should be of the form @c type:residual, and @a type must be a
 * type known to the library.  If the @a name does not contain a colon,
 * interpret it as a file name.
 *
 * @code
 * Example: krb5_cc_resolve(context, "MEMORY:C_", &cache);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
/* *
 * Duplicate ccache handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Credential cache handle to be duplicated
 * @param [out] out             Credential cache handle
 *
 * Create a new handle referring to the same cache as @a in.
 * The new handle and @a in can be closed independently.
 */
/* *
 * Return the name of the default credential cache.
 *
 * @param [in] context          Library context
 *
 * Return a pointer to the default credential cache name for @a context, as
 * determined by a prior call to krb5_cc_set_default_name(), by the KRB5CCNAME
 * environment variable, by the default_ccache_name profile variable, or by the
 * operating system or build-time default value.  The returned value must not
 * be modified or freed by the caller.  The returned value becomes invalid when
 * @a context is destroyed krb5_free_context() or if a subsequent call to
 * krb5_cc_set_default_name() is made on @a context.
 *
 * The default credential cache name is cached in @a context between calls to
 * this function, so if the value of KRB5CCNAME changes in the process
 * environment after the first call to this function on, that change will not
 * be reflected in later calls with the same context.  The caller can invoke
 * krb5_cc_set_default_name() with a NULL value of @a name to clear the cached
 * value and force the default name to be recomputed.
 *
 * @return
 * Name of default credential cache for the current user.
 */
#[no_mangle]
#[c2rust::src_loc = "289:1"]
pub unsafe extern "C" fn krb5_cc_default_name(mut context: krb5_context)
 -> *const libc::c_char {
    let mut os_ctx: krb5_os_context = 0 as *mut _krb5_os_context;
    let mut profstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut envstr: *mut libc::c_char = 0 as *mut libc::c_char;
    if context.is_null() ||
           (*context).magic as libc::c_long != -(1760647388 as libc::c_long) {
        return 0 as *const libc::c_char
    }
    os_ctx = &mut (*context).os_context;
    if !(*os_ctx).default_ccname.is_null() { return (*os_ctx).default_ccname }
    /* Try the environment variable first. */
    envstr =
        secure_getenv(b"KRB5CCNAME\x00" as *const u8 as *const libc::c_char);
    if !envstr.is_null() {
        (*os_ctx).default_ccname = strdup(envstr);
        return (*os_ctx).default_ccname
    }
    if profile_get_string((*context).profile,
                          b"libdefaults\x00" as *const u8 as
                              *const libc::c_char,
                          b"default_ccache_name\x00" as *const u8 as
                              *const libc::c_char, 0 as *const libc::c_char,
                          0 as *const libc::c_char, &mut profstr) ==
           0 as libc::c_int as libc::c_long && !profstr.is_null() {
        k5_expand_path_tokens(context, profstr,
                              &mut (*os_ctx).default_ccname);
        profile_release_string(profstr);
        return (*os_ctx).default_ccname
    }
    /* Fall back on the default ccache name for the OS. */
    get_from_os(context);
    return (*os_ctx).default_ccname;
}
