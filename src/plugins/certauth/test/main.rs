use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
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
        #[c2rust::src_loc = "3547:1"]
        pub fn krb5_unparse_name_flags(context: krb5_context,
                                       principal: krb5_const_principal,
                                       flags: libc::c_int,
                                       name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:33"]
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
    use super::stddef_h::size_t;
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
        /* Return ENOMEM if buf is in an error state, 0 otherwise. */
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn k5_buf_status(buf: *mut k5buf) -> libc::c_int;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/certauth_plugin.h:34"]
pub mod certauth_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/krb5/certauth_plugin.h - certauth plugin header. */
/*
 * Copyright (C) 2017 by Red Hat, Inc.
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
 * Declarations for certauth plugin module implementors.
 *
 * The certauth pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * certauth plugin modules should define a function named
 * certauth_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   certauth_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                           krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_certauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* Abstract module data type. */
    #[c2rust::src_loc = "68:1"]
    pub type krb5_certauth_moddata = *mut krb5_certauth_moddata_st;
    /*
 * Optional: Initialize module data.
 */
    #[c2rust::src_loc = "77:1"]
    pub type krb5_certauth_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut krb5_certauth_moddata)
                   -> krb5_error_code>;
    /*
 * Optional: Clean up the module data.
 */
    #[c2rust::src_loc = "84:1"]
    pub type krb5_certauth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_certauth_moddata)
                   -> ()>;
    /*
 * Mandatory:
 * Return 0 if the DER-encoded cert is authorized for PKINIT authentication by
 * princ; otherwise return one of the following error codes:
 * - KRB5KDC_ERR_CLIENT_NAME_MISMATCH - incorrect SAN value
 * - KRB5KDC_ERR_INCONSISTENT_KEY_PURPOSE - incorrect EKU
 * - KRB5KDC_ERR_CERTIFICATE_MISMATCH - other extension error
 * - KRB5_PLUGIN_NO_HANDLE - the module has no opinion about cert
 *
 * - opts is used by built-in modules to receive internal data, and must be
 *   ignored by other modules.
 * - db_entry receives the client principal database entry, and can be ignored
 *   by modules that do not link with libkdb5.
 * - *authinds_out optionally returns a null-terminated list of authentication
 *   indicator strings upon KRB5_PLUGIN_NO_HANDLE or accepted authorization.
 */
    #[c2rust::src_loc = "103:1"]
    pub type krb5_certauth_authorize_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_certauth_moddata,
                                    _: *const uint8_t, _: size_t,
                                    _: krb5_const_principal,
                                    _: *const libc::c_void,
                                    _: *const _krb5_db_entry_new,
                                    _: *mut *mut *mut libc::c_char)
                   -> krb5_error_code>;
    /*
 * Free indicators allocated by a module.  Mandatory if authorize returns
 * authentication indicators.
 */
    #[c2rust::src_loc = "115:1"]
    pub type krb5_certauth_free_indicator_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_certauth_moddata,
                                    _: *mut *mut libc::c_char) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "120:16"]
    pub struct krb5_certauth_vtable_st {
        pub name: *const libc::c_char,
        pub init: krb5_certauth_init_fn,
        pub fini: krb5_certauth_fini_fn,
        pub authorize: krb5_certauth_authorize_fn,
        pub free_ind: krb5_certauth_free_indicator_fn,
    }
    #[c2rust::src_loc = "120:1"]
    pub type krb5_certauth_vtable = *mut krb5_certauth_vtable_st;
    use super::krb5_certauth_moddata_st;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_const_principal};
    use super::stdint_uintn_h::uint8_t;
    use super::stddef_h::size_t;
    extern "C" {
        /* A module can optionally include <kdb.h> to inspect the client principal
 * entry when authorizing a request. */
        #[c2rust::src_loc = "72:8"]
        pub type _krb5_db_entry_new;
    }
    /* KRB5_CERTAUTH_PLUGIN_H */
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
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "90:14"]
        pub fn memchr(_: *const libc::c_void, _: libc::c_int,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:33"]
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
pub use self::types_h::{__uint8_t, __int32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_const_principal,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _profile_t, krb5_unparse_name_flags,
                       krb5_free_unparsed_name};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_init_dynamic,
                         k5_buf_add, k5_buf_add_len, k5_buf_status,
                         k5_buf_free};
pub use self::certauth_plugin_h::{krb5_certauth_moddata,
                                  krb5_certauth_init_fn,
                                  krb5_certauth_fini_fn,
                                  krb5_certauth_authorize_fn,
                                  krb5_certauth_free_indicator_fn,
                                  krb5_certauth_vtable_st,
                                  krb5_certauth_vtable, _krb5_db_entry_new};
use self::stdlib_h::{calloc, free};
use self::string_h::{strdup, memchr, memcmp, strlen};
use self::assert_h::__assert_fail;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/certauth/main.c - certauth plugin test modules. */
/*
 * Copyright (C) 2017 by Red Hat, Inc.
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
#[c2rust::src_loc = "36:8"]
pub struct krb5_certauth_moddata_st {
    pub initialized: libc::c_int,
}
/* Test module 1 returns OK with an indicator. */
#[c2rust::src_loc = "41:1"]
unsafe extern "C" fn test1_authorize(mut context: krb5_context,
                                     mut moddata: krb5_certauth_moddata,
                                     mut cert: *const uint8_t,
                                     mut cert_len: size_t,
                                     mut princ: krb5_const_principal,
                                     mut opts: *const libc::c_void,
                                     mut db_entry: *const _krb5_db_entry_new,
                                     mut authinds_out:
                                         *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ais: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    ais =
        calloc(2 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) as
            *mut *mut libc::c_char;
    if !ais.is_null() {
    } else {
        __assert_fail(b"ais != NULL\x00" as *const u8 as *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      51 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 175],
                                                &[libc::c_char; 175]>(b"krb5_error_code test1_authorize(krb5_context, krb5_certauth_moddata, const uint8_t *, size_t, krb5_const_principal, const void *, const struct _krb5_db_entry_new *, char ***)\x00")).as_ptr());
    }
    let ref mut fresh0 = *ais.offset(0 as libc::c_int as isize);
    *fresh0 = strdup(b"test1\x00" as *const u8 as *const libc::c_char);
    if !(*ais.offset(0 as libc::c_int as isize)).is_null() {
    } else {
        __assert_fail(b"ais[0] != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      53 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 175],
                                                &[libc::c_char; 175]>(b"krb5_error_code test1_authorize(krb5_context, krb5_certauth_moddata, const uint8_t *, size_t, krb5_const_principal, const void *, const struct _krb5_db_entry_new *, char ***)\x00")).as_ptr());
    }
    *authinds_out = ais;
    return -(1765328135 as libc::c_long) as krb5_error_code;
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn test_free_ind(mut context: krb5_context,
                                   mut moddata: krb5_certauth_moddata,
                                   mut authinds: *mut *mut libc::c_char) {
    let mut i: size_t = 0;
    if authinds.is_null() { return }
    i = 0 as libc::c_int as size_t;
    while !(*authinds.offset(i as isize)).is_null() {
        free(*authinds.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free(authinds as *mut libc::c_void);
}
/* A basic moddata test. */
#[c2rust::src_loc = "72:1"]
unsafe extern "C" fn test2_init(mut context: krb5_context,
                                mut moddata_out: *mut krb5_certauth_moddata)
 -> krb5_error_code {
    let mut mod_0: krb5_certauth_moddata = 0 as *mut krb5_certauth_moddata_st;
    mod_0 =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_certauth_moddata_st>() as
                   libc::c_ulong) as krb5_certauth_moddata;
    if !mod_0.is_null() {
    } else {
        __assert_fail(b"mod != NULL\x00" as *const u8 as *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      78 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[libc::c_char; 66]>(b"krb5_error_code test2_init(krb5_context, krb5_certauth_moddata *)\x00")).as_ptr());
    }
    (*mod_0).initialized = 1 as libc::c_int;
    *moddata_out = mod_0;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "84:1"]
unsafe extern "C" fn test2_fini(mut context: krb5_context,
                                mut moddata: krb5_certauth_moddata) {
    free(moddata as *mut libc::c_void);
}
/* Return true if cert appears to contain the CN name, based on a search of the
 * DER encoding. */
#[c2rust::src_loc = "92:1"]
unsafe extern "C" fn has_cn(mut context: krb5_context,
                            mut cert: *const uint8_t, mut cert_len: size_t,
                            mut name: *const libc::c_char) -> krb5_boolean {
    let mut match_0: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut name_len: uint8_t = 0;
    let mut cntag: [uint8_t; 5] =
        *::std::mem::transmute::<&[u8; 5],
                                 &mut [uint8_t; 5]>(b"\x06\x03U\x04\x03");
    let mut c: *const uint8_t = 0 as *const uint8_t;
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut c_left: size_t = 0;
    /* Construct a DER search string of the CN AttributeType encoding followed
     * by a UTF8String encoding containing name as the AttributeValue. */
    k5_buf_init_dynamic(&mut buf);
    k5_buf_add_len(&mut buf, cntag.as_mut_ptr() as *const libc::c_void,
                   ::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong);
    k5_buf_add(&mut buf, b"\x0c\x00" as *const u8 as *const libc::c_char);
    if strlen(name) < 128 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"strlen(name) < 128\x00" as *const u8 as
                          *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      107 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"krb5_boolean has_cn(krb5_context, const uint8_t *, size_t, const char *)\x00")).as_ptr());
    }
    name_len = strlen(name) as uint8_t;
    k5_buf_add_len(&mut buf,
                   &mut name_len as *mut uint8_t as *const libc::c_void,
                   1 as libc::c_int as size_t);
    k5_buf_add_len(&mut buf, name as *const libc::c_void, name_len as size_t);
    if k5_buf_status(&mut buf) == 0 as libc::c_int {
    } else {
        __assert_fail(b"k5_buf_status(&buf) == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      111 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"krb5_boolean has_cn(krb5_context, const uint8_t *, size_t, const char *)\x00")).as_ptr());
    }
    /* Check for the CN needle in the certificate haystack. */
    c_left = cert_len;
    c =
        memchr(cert as *const libc::c_void,
               *cntag.as_mut_ptr() as libc::c_int, c_left) as *const uint8_t;
    while !c.is_null() {
        c_left =
            cert_len.wrapping_sub(c.wrapping_offset_from(cert) as libc::c_long
                                      as libc::c_ulong);
        if buf.len > c_left { break ; }
        if memcmp(c as *const libc::c_void, buf.data, buf.len) ==
               0 as libc::c_int {
            match_0 = 1 as libc::c_int as krb5_boolean;
            break ;
        } else {
            if c_left >= 1 as libc::c_int as libc::c_ulong {
            } else {
                __assert_fail(b"c_left >= 1\x00" as *const u8 as
                                  *const libc::c_char,
                              b"main.c\x00" as *const u8 as
                                  *const libc::c_char,
                              124 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 73],
                                                        &[libc::c_char; 73]>(b"krb5_boolean has_cn(krb5_context, const uint8_t *, size_t, const char *)\x00")).as_ptr());
            }
            c =
                memchr(c.offset(1 as libc::c_int as isize) as
                           *const libc::c_void,
                       *cntag.as_mut_ptr() as libc::c_int,
                       c_left.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                    as *const uint8_t
        }
    }
    k5_buf_free(&mut buf);
    return match_0;
}
/*
 * Test module 2 returns OK if princ matches the CN part of the subject name,
 * and returns indicators of the module name and princ.
 */
#[c2rust::src_loc = "136:1"]
unsafe extern "C" fn test2_authorize(mut context: krb5_context,
                                     mut moddata: krb5_certauth_moddata,
                                     mut cert: *const uint8_t,
                                     mut cert_len: size_t,
                                     mut princ: krb5_const_principal,
                                     mut opts: *const libc::c_void,
                                     mut db_entry: *const _krb5_db_entry_new,
                                     mut authinds_out:
                                         *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ais: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    *authinds_out = 0 as *mut *mut libc::c_char;
    if !moddata.is_null() && (*moddata).initialized != 0 {
    } else {
        __assert_fail(b"moddata != NULL && moddata->initialized\x00" as
                          *const u8 as *const libc::c_char,
                      b"main.c\x00" as *const u8 as *const libc::c_char,
                      148 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 175],
                                                &[libc::c_char; 175]>(b"krb5_error_code test2_authorize(krb5_context, krb5_certauth_moddata, const uint8_t *, size_t, krb5_const_principal, const void *, const struct _krb5_db_entry_new *, char ***)\x00")).as_ptr());
    }
    ret =
        krb5_unparse_name_flags(context, princ, 0x2 as libc::c_int,
                                &mut name);
    if !(ret != 0) {
        if has_cn(context, cert, cert_len, name) == 0 {
            ret = -(1765328318 as libc::c_long) as krb5_error_code
        } else {
            /* Create an indicator list with the module name and CN. */
            ais =
                calloc(3 as libc::c_int as libc::c_ulong,
                       ::std::mem::size_of::<*mut libc::c_char>() as
                           libc::c_ulong) as *mut *mut libc::c_char;
            if !ais.is_null() {
            } else {
                __assert_fail(b"ais != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"main.c\x00" as *const u8 as
                                  *const libc::c_char,
                              162 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 175],
                                                        &[libc::c_char; 175]>(b"krb5_error_code test2_authorize(krb5_context, krb5_certauth_moddata, const uint8_t *, size_t, krb5_const_principal, const void *, const struct _krb5_db_entry_new *, char ***)\x00")).as_ptr());
            }
            let ref mut fresh1 = *ais.offset(0 as libc::c_int as isize);
            *fresh1 =
                strdup(b"test2\x00" as *const u8 as *const libc::c_char);
            let ref mut fresh2 = *ais.offset(1 as libc::c_int as isize);
            *fresh2 = strdup(name);
            if !(*ais.offset(0 as libc::c_int as isize)).is_null() &&
                   !(*ais.offset(1 as libc::c_int as isize)).is_null() {
            } else {
                __assert_fail(b"ais[0] != NULL && ais[1] != NULL\x00" as
                                  *const u8 as *const libc::c_char,
                              b"main.c\x00" as *const u8 as
                                  *const libc::c_char,
                              165 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 175],
                                                        &[libc::c_char; 175]>(b"krb5_error_code test2_authorize(krb5_context, krb5_certauth_moddata, const uint8_t *, size_t, krb5_const_principal, const void *, const struct _krb5_db_entry_new *, char ***)\x00")).as_ptr());
            }
            *authinds_out = ais;
            ais = 0 as *mut *mut libc::c_char
        }
    }
    krb5_free_unparsed_name(context, name);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "178:1"]
pub unsafe extern "C" fn certauth_test1_initvt(mut context: krb5_context,
                                               mut maj_ver: libc::c_int,
                                               mut min_ver: libc::c_int,
                                               mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_certauth_vtable = 0 as *mut krb5_certauth_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_certauth_vtable;
    (*vt).name = b"test1\x00" as *const u8 as *const libc::c_char;
    (*vt).authorize =
        Some(test1_authorize as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_certauth_moddata,
                                      _: *const uint8_t, _: size_t,
                                      _: krb5_const_principal,
                                      _: *const libc::c_void,
                                      _: *const _krb5_db_entry_new,
                                      _: *mut *mut *mut libc::c_char)
                     -> krb5_error_code);
    (*vt).free_ind =
        Some(test_free_ind as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_certauth_moddata,
                                      _: *mut *mut libc::c_char) -> ());
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "196:1"]
pub unsafe extern "C" fn certauth_test2_initvt(mut context: krb5_context,
                                               mut maj_ver: libc::c_int,
                                               mut min_ver: libc::c_int,
                                               mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_certauth_vtable = 0 as *mut krb5_certauth_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_certauth_vtable;
    (*vt).name = b"test2\x00" as *const u8 as *const libc::c_char;
    (*vt).authorize =
        Some(test2_authorize as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_certauth_moddata,
                                      _: *const uint8_t, _: size_t,
                                      _: krb5_const_principal,
                                      _: *const libc::c_void,
                                      _: *const _krb5_db_entry_new,
                                      _: *mut *mut *mut libc::c_char)
                     -> krb5_error_code);
    (*vt).init =
        Some(test2_init as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: *mut krb5_certauth_moddata)
                     -> krb5_error_code);
    (*vt).fini =
        Some(test2_fini as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_certauth_moddata) -> ());
    (*vt).free_ind =
        Some(test_free_ind as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_certauth_moddata,
                                      _: *mut *mut libc::c_char) -> ());
    return 0 as libc::c_int;
}
