use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:27"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
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
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2727:16"]
    pub struct krb5_keytab_entry_st {
        pub magic: krb5_magic,
        pub principal: krb5_principal,
        pub timestamp: krb5_timestamp,
        pub vno: krb5_kvno,
        pub key: krb5_keyblock,
    }
    /* * A key table entry. */
    #[c2rust::src_loc = "2727:1"]
    pub type krb5_keytab_entry = krb5_keytab_entry_st;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    use super::stddef_h::size_t;
    extern "C" {
        /* *< Principal of this key */
        /* *< Time entry written to keytable */
        /* *< Key version number */
        /* *< The secret key */
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
        /*
 * end "keytab.h"
 */
        /*
 * begin "func-proto.h"
 */
        /* *< Use secure context configuration */
        /* *< Use KDC configuration if available */
        /* *
 * Create a krb5 library context.
 *
 * @param [out] context         Library context
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @warning Any program or module that needs the Kerberos code to not trust the
 * environment must use krb5_init_secure_context(), or clean out the
 * environment.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2922:1"]
        pub fn krb5_init_context(context: *mut krb5_context)
         -> krb5_error_code;
        /* *
 * Convert a krb5_principal structure to a string representation.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [out] name            String representation of principal name
 *
 * The resulting string representation uses the format and quoting conventions
 * described for krb5_parse_name().
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6341:1"]
        pub fn krb5_enctype_to_name(enctype: krb5_enctype,
                                    shortest: krb5_boolean,
                                    buffer: *mut libc::c_char, buflen: size_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6398:1"]
        pub fn krb5_timestamp_to_sfstring(timestamp: krb5_timestamp,
                                          buffer: *mut libc::c_char,
                                          buflen: size_t,
                                          pad: *mut libc::c_char)
         -> krb5_error_code;
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
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/ktutil/ktutil.h:28"]
pub mod ktutil_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:16"]
    pub struct _krb5_kt_list {
        pub next: *mut _krb5_kt_list,
        pub entry: *mut krb5_keytab_entry,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/ktutil/ktutil.h */
/*
 * Copyright 1995 by the Massachusetts Institute of Technology.
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
    #[c2rust::src_loc = "27:1"]
    pub type krb5_kt_list = *mut _krb5_kt_list;
    use super::krb5_h::{krb5_keytab_entry, krb5_context, krb5_error_code,
                        krb5_kvno};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "32:1"]
        pub fn ktutil_free_kt_list(_: krb5_context, _: krb5_kt_list)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "34:1"]
        pub fn ktutil_delete(_: krb5_context, _: *mut krb5_kt_list,
                             _: libc::c_int) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "36:1"]
        pub fn ktutil_add(_: krb5_context, _: *mut krb5_kt_list,
                          _: *mut libc::c_char, _: libc::c_int, _: krb5_kvno,
                          _: *mut libc::c_char, _: libc::c_int,
                          _: *mut libc::c_char) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "45:1"]
        pub fn ktutil_read_keytab(_: krb5_context, _: *mut libc::c_char,
                                  _: *mut krb5_kt_list) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn ktutil_write_keytab(_: krb5_context, _: krb5_kt_list,
                                   _: *mut libc::c_char) -> krb5_error_code;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/ss/ss.h:32"]
pub mod ss_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:27"]
    pub struct _ss_request_entry {
        pub command_names: *const *const libc::c_char,
        pub function: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _:
                                                      *const *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_void)
                                 -> ()>,
        pub info_string: *const libc::c_char,
        pub flags: libc::c_int,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see mit-sipb-copyright.h.
 */
    #[c2rust::src_loc = "22:1"]
    pub type ss_request_entry = _ss_request_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:27"]
    pub struct _ss_request_table {
        pub version: libc::c_int,
        pub requests: *const ss_request_entry,
    }
    #[c2rust::src_loc = "29:1"]
    pub type ss_request_table = _ss_request_table;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "58:1"]
        pub fn ss_perror(_: libc::c_int, _: libc::c_long,
                         _: *const libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "59:1"]
        pub fn ss_listen(_: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:1"]
        pub fn ss_create_invocation(_: *mut libc::c_char,
                                    _: *mut libc::c_char,
                                    _: *mut libc::c_char,
                                    _: *const ss_request_table,
                                    _: *mut libc::c_int) -> libc::c_int;
    }
    /* whatever */
    /* foo */
    /* NULL */
    /* 0 */
    /* _ss_h */
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/locale.h:30"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
pub use self::types_h::{__uint8_t, __int32_t, __off_t, __off64_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_kvno,
                       krb5_enctype, krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, krb5_keytab_entry_st, krb5_keytab_entry,
                       _profile_t, krb5_init_context, krb5_unparse_name,
                       krb5_enctype_to_name, krb5_timestamp_to_sfstring};
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
pub use self::com_err_h::{errcode_t, com_err};
pub use self::ktutil_h::{_krb5_kt_list, krb5_kt_list, ktutil_free_kt_list,
                         ktutil_delete, ktutil_add, ktutil_read_keytab,
                         ktutil_write_keytab};
pub use self::ss_h::{_ss_request_entry, ss_request_entry, _ss_request_table,
                     ss_request_table, ss_perror, ss_listen,
                     ss_create_invocation};
use self::stdlib_h::{atoi, free, exit};
use self::stdio_h::{stderr, fprintf, printf};
use self::libintl_h::dgettext;
use self::string_h::{strlen, strncmp};
use self::locale_h::setlocale;
extern "C" {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/ktutil/ktutil.c - SS user interface for ktutil */
/*
 * Copyright 1995, 1996, 2008 by the Massachusetts Institute of Technology.
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
    #[no_mangle]
    #[c2rust::src_loc = "38:25"]
    pub static ktutil_cmds: ss_request_table;
}
#[no_mangle]
#[c2rust::src_loc = "39:14"]
pub static mut kcontext: krb5_context =
    0 as *const _krb5_context as *mut _krb5_context;
#[no_mangle]
#[c2rust::src_loc = "40:14"]
pub static mut ktlist: krb5_kt_list =
    0 as *const _krb5_kt_list as krb5_kt_list;
#[c2rust::src_loc = "42:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut retval: krb5_error_code = 0;
    let mut sci_idx: libc::c_int = 0;
    setlocale(6 as libc::c_int, b"\x00" as *const u8 as *const libc::c_char);
    retval = krb5_init_context(&mut kcontext);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing krb5\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    sci_idx =
        ss_create_invocation(b"ktutil\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                             b"5.0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char,
                             0 as *mut libc::c_void as *mut libc::c_char,
                             &ktutil_cmds, &mut retval);
    if retval != 0 {
        ss_perror(sci_idx, retval as libc::c_long,
                  dgettext(b"mit-krb5\x00" as *const u8 as
                               *const libc::c_char,
                           b"creating invocation\x00" as *const u8 as
                               *const libc::c_char));
        exit(1 as libc::c_int);
    }
    retval = ss_listen(sci_idx);
    ktutil_free_kt_list(kcontext, ktlist);
    exit(0 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn ktutil_clear_list(mut argc: libc::c_int,
                                           mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    if argc != 1 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s: invalid arguments\n\x00" as *const u8 as
                             *const libc::c_char),
                *argv.offset(0 as libc::c_int as isize));
        return
    }
    retval = ktutil_free_kt_list(kcontext, ktlist);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while freeing ktlist\x00" as *const u8 as
                             *const libc::c_char));
    }
    ktlist = 0 as krb5_kt_list;
}
#[no_mangle]
#[c2rust::src_loc = "82:1"]
pub unsafe extern "C" fn ktutil_read_v5(mut argc: libc::c_int,
                                        mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    if argc != 2 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s: must specify keytab to read\n\x00" as *const u8
                             as *const libc::c_char),
                *argv.offset(0 as libc::c_int as isize));
        return
    }
    retval =
        ktutil_read_keytab(kcontext, *argv.offset(1 as libc::c_int as isize),
                           &mut ktlist);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while reading keytab \"%s\"\x00" as *const u8 as
                             *const libc::c_char),
                *argv.offset(1 as libc::c_int as isize));
    };
}
#[no_mangle]
#[c2rust::src_loc = "97:1"]
pub unsafe extern "C" fn ktutil_read_v4(mut argc: libc::c_int,
                                        mut argv: *mut *mut libc::c_char) {
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"%s: reading srvtabs is no longer supported\n\x00" as
                         *const u8 as *const libc::c_char),
            *argv.offset(0 as libc::c_int as isize));
}
#[no_mangle]
#[c2rust::src_loc = "105:1"]
pub unsafe extern "C" fn ktutil_write_v5(mut argc: libc::c_int,
                                         mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    if argc != 2 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s: must specify keytab to write\n\x00" as
                             *const u8 as *const libc::c_char),
                *argv.offset(0 as libc::c_int as isize));
        return
    }
    retval =
        ktutil_write_keytab(kcontext, ktlist,
                            *argv.offset(1 as libc::c_int as isize));
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while writing keytab \"%s\"\x00" as *const u8 as
                             *const libc::c_char),
                *argv.offset(1 as libc::c_int as isize));
    };
}
#[no_mangle]
#[c2rust::src_loc = "120:1"]
pub unsafe extern "C" fn ktutil_write_v4(mut argc: libc::c_int,
                                         mut argv: *mut *mut libc::c_char) {
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"%s: writing srvtabs is no longer supported\n\x00" as
                         *const u8 as *const libc::c_char),
            *argv.offset(0 as libc::c_int as isize));
}
#[no_mangle]
#[c2rust::src_loc = "128:1"]
pub unsafe extern "C" fn ktutil_add_entry(mut argc: libc::c_int,
                                          mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    let mut princ: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut enctype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut kvno: krb5_kvno = 0 as libc::c_int as krb5_kvno;
    let mut use_pass: libc::c_int = 0 as libc::c_int;
    let mut use_key: libc::c_int = 0 as libc::c_int;
    let mut use_kvno: libc::c_int = 0 as libc::c_int;
    let mut fetch: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut salt: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 1 as libc::c_int;
    while i < argc {
        if strlen(*argv.offset(i as isize)) ==
               2 as libc::c_int as libc::c_ulong &&
               strncmp(*argv.offset(i as isize),
                       b"-p\x00" as *const u8 as *const libc::c_char,
                       2 as libc::c_int as libc::c_ulong) == 0 {
            i += 1;
            princ = *argv.offset(i as isize)
        } else if strlen(*argv.offset(i as isize)) ==
                      2 as libc::c_int as libc::c_ulong &&
                      strncmp(*argv.offset(i as isize),
                              b"-k\x00" as *const u8 as *const libc::c_char,
                              2 as libc::c_int as libc::c_ulong) == 0 {
            i += 1;
            kvno = atoi(*argv.offset(i as isize)) as krb5_kvno;
            use_kvno += 1
        } else if strlen(*argv.offset(i as isize)) ==
                      2 as libc::c_int as libc::c_ulong &&
                      strncmp(*argv.offset(i as isize),
                              b"-e\x00" as *const u8 as *const libc::c_char,
                              2 as libc::c_int as libc::c_ulong) == 0 {
            i += 1;
            enctype = *argv.offset(i as isize)
        } else if strlen(*argv.offset(i as isize)) ==
                      9 as libc::c_int as libc::c_ulong &&
                      strncmp(*argv.offset(i as isize),
                              b"-password\x00" as *const u8 as
                                  *const libc::c_char,
                              9 as libc::c_int as libc::c_ulong) == 0 {
            use_pass += 1
        } else if strlen(*argv.offset(i as isize)) ==
                      4 as libc::c_int as libc::c_ulong &&
                      strncmp(*argv.offset(i as isize),
                              b"-key\x00" as *const u8 as *const libc::c_char,
                              4 as libc::c_int as libc::c_ulong) == 0 {
            use_key += 1
        } else if strlen(*argv.offset(i as isize)) ==
                      2 as libc::c_int as libc::c_ulong &&
                      strncmp(*argv.offset(i as isize),
                              b"-s\x00" as *const u8 as *const libc::c_char,
                              2 as libc::c_int as libc::c_ulong) == 0 {
            i += 1;
            salt = *argv.offset(i as isize)
        } else if strlen(*argv.offset(i as isize)) ==
                      2 as libc::c_int as libc::c_ulong &&
                      strncmp(*argv.offset(i as isize),
                              b"-f\x00" as *const u8 as *const libc::c_char,
                              2 as libc::c_int as libc::c_ulong) == 0 {
            fetch += 1
        }
        i += 1
    }
    if princ.is_null() || use_pass + use_key != 1 as libc::c_int ||
           use_kvno == 0 || fetch != 0 && !salt.is_null() {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"usage: %s (-key | -password) -p principal -k kvno [-e enctype] [-f|-s salt]\n\x00"
                             as *const u8 as *const libc::c_char),
                *argv.offset(0 as libc::c_int as isize));
        return
    }
    if fetch == 0 && enctype.is_null() {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"enctype must be specified if not using -f\n\x00" as
                             *const u8 as *const libc::c_char));
        return
    }
    retval =
        ktutil_add(kcontext, &mut ktlist, princ, fetch, kvno, enctype,
                   use_pass, salt);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while adding new entry\x00" as *const u8 as
                             *const libc::c_char));
    };
}
#[no_mangle]
#[c2rust::src_loc = "186:1"]
pub unsafe extern "C" fn ktutil_delete_entry(mut argc: libc::c_int,
                                             mut argv:
                                                 *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    if argc != 2 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s: must specify entry to delete\n\x00" as
                             *const u8 as *const libc::c_char),
                *argv.offset(0 as libc::c_int as isize));
        return
    }
    retval =
        ktutil_delete(kcontext, &mut ktlist,
                      atoi(*argv.offset(1 as libc::c_int as isize)));
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while deleting entry %d\x00" as *const u8 as
                             *const libc::c_char),
                atoi(*argv.offset(1 as libc::c_int as isize)));
    };
}
#[no_mangle]
#[c2rust::src_loc = "201:1"]
pub unsafe extern "C" fn ktutil_list(mut argc: libc::c_int,
                                     mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    let mut lp: krb5_kt_list = 0 as *mut _krb5_kt_list;
    let mut show_time: libc::c_int = 0 as libc::c_int;
    let mut show_keys: libc::c_int = 0 as libc::c_int;
    let mut show_enctype: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_uint = 0;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 1 as libc::c_int;
    while i < argc {
        if strlen(*argv.offset(i as isize)) ==
               2 as libc::c_int as libc::c_ulong &&
               strncmp(*argv.offset(i as isize),
                       b"-t\x00" as *const u8 as *const libc::c_char,
                       2 as libc::c_int as libc::c_ulong) == 0 {
            show_time += 1
        } else if strlen(*argv.offset(i as isize)) ==
                      2 as libc::c_int as libc::c_ulong &&
                      strncmp(*argv.offset(i as isize),
                              b"-k\x00" as *const u8 as *const libc::c_char,
                              2 as libc::c_int as libc::c_ulong) == 0 {
            show_keys += 1
        } else if strlen(*argv.offset(i as isize)) ==
                      2 as libc::c_int as libc::c_ulong &&
                      strncmp(*argv.offset(i as isize),
                              b"-e\x00" as *const u8 as *const libc::c_char,
                              2 as libc::c_int as libc::c_ulong) == 0 {
            show_enctype += 1
        } else {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"%s: usage: %s [-t] [-k] [-e]\n\x00" as
                                 *const u8 as *const libc::c_char),
                    *argv.offset(0 as libc::c_int as isize),
                    *argv.offset(0 as libc::c_int as isize));
            return
        }
        i += 1
    }
    /* XXX Translating would disturb table alignment; skip for now. */
    if show_time != 0 {
        printf(b"slot KVNO Timestamp         Principal\n\x00" as *const u8 as
                   *const libc::c_char);
        printf(b"---- ---- ----------------- ---------------------------------------------------\n\x00"
                   as *const u8 as *const libc::c_char);
    } else {
        printf(b"slot KVNO Principal\n\x00" as *const u8 as
                   *const libc::c_char);
        printf(b"---- ---- ---------------------------------------------------------------------\n\x00"
                   as *const u8 as *const libc::c_char);
    }
    i = 1 as libc::c_int;
    lp = ktlist;
    while !lp.is_null() {
        retval =
            krb5_unparse_name(kcontext,
                              (*(*lp).entry).principal as
                                  krb5_const_principal, &mut pname);
        if retval != 0 {
            com_err(*argv.offset(0 as libc::c_int as isize),
                    retval as errcode_t,
                    b"while unparsing principal name\x00" as *const u8 as
                        *const libc::c_char);
            return
        }
        printf(b"%4d %4d \x00" as *const u8 as *const libc::c_char, i,
               (*(*lp).entry).vno);
        if show_time != 0 {
            let mut fmtbuf: [libc::c_char; 18] = [0; 18];
            let mut fill: libc::c_char = 0;
            let mut tstamp: time_t = 0;
            tstamp = (*(*lp).entry).timestamp as time_t;
            (*(*lp).entry).timestamp = tstamp as krb5_timestamp;
            fill = ' ' as i32 as libc::c_char;
            if krb5_timestamp_to_sfstring((*(*lp).entry).timestamp,
                                          fmtbuf.as_mut_ptr(),
                                          ::std::mem::size_of::<[libc::c_char; 18]>()
                                              as libc::c_ulong, &mut fill) ==
                   0 {
                printf(b"%s \x00" as *const u8 as *const libc::c_char,
                       fmtbuf.as_mut_ptr());
            }
        }
        printf(b"%40s\x00" as *const u8 as *const libc::c_char, pname);
        if show_enctype != 0 {
            static mut buf: [libc::c_char; 256] = [0; 256];
            retval =
                krb5_enctype_to_name((*(*lp).entry).key.enctype,
                                     0 as libc::c_int as krb5_boolean,
                                     buf.as_mut_ptr(),
                                     ::std::mem::size_of::<[libc::c_char; 256]>()
                                         as libc::c_ulong);
            if retval != 0 {
                com_err(*argv.offset(0 as libc::c_int as isize),
                        retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"While converting enctype to string\x00" as
                                     *const u8 as *const libc::c_char));
                return
            }
            printf(b" (%s) \x00" as *const u8 as *const libc::c_char,
                   buf.as_mut_ptr());
        }
        if show_keys != 0 {
            printf(b" (0x\x00" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int as libc::c_uint;
            while j < (*(*lp).entry).key.length {
                printf(b"%02x\x00" as *const u8 as *const libc::c_char,
                       *(*(*lp).entry).key.contents.offset(j as isize) as
                           libc::c_int);
                j = j.wrapping_add(1)
            }
            printf(b")\x00" as *const u8 as *const libc::c_char);
        }
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        free(pname as *mut libc::c_void);
        i += 1;
        lp = (*lp).next
    };
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
