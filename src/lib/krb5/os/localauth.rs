use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
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
        #[c2rust::src_loc = "4887:1"]
        pub fn krb5_get_default_realm(context: krb5_context,
                                      lrealm: *mut *mut libc::c_char)
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
    #[inline]
    #[c2rust::src_loc = "2244:1"]
    pub unsafe extern "C" fn data_eq_string(mut d: krb5_data,
                                            mut s: *const libc::c_char)
     -> libc::c_int {
        return (d.length as libc::c_ulong == strlen(s) &&
                    (d.length == 0 as libc::c_int as libc::c_uint ||
                         memcmp(d.data as *const libc::c_void,
                                s as *const libc::c_void,
                                d.length as libc::c_ulong) == 0)) as
                   libc::c_int;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_context, krb5_error_code, krb5_data};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::localauth_module_handle;
    use super::k5_err_h::errinfo;
    use super::plugin_h::krb5_plugin_initvt_fn;
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
    use super::string_h::{memcpy, strlen, memcmp};
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1202:8"]
        pub type _kdb_log_context;
        #[c2rust::src_loc = "1207:8"]
        pub type k5_tls_vtable_st;
        #[c2rust::src_loc = "1206:8"]
        pub type hostrealm_module_handle;
        #[c2rust::src_loc = "1204:8"]
        pub type ccselect_module_handle;
        #[c2rust::src_loc = "1203:16"]
        pub type krb5_preauth_context_st;
        #[c2rust::src_loc = "1200:8"]
        pub type _kdb5_dal_handle;
        #[no_mangle]
        #[c2rust::src_loc = "1177:1"]
        pub fn k5_plugin_register(context: krb5_context,
                                  interface_id: libc::c_int,
                                  modname: *const libc::c_char,
                                  module: krb5_plugin_initvt_fn)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1168:1"]
        pub fn k5_plugin_load_all(context: krb5_context,
                                  interface_id: libc::c_int,
                                  modules: *mut *mut krb5_plugin_initvt_fn)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1173:1"]
        pub fn k5_plugin_free_modules(context: krb5_context,
                                      modules: *mut krb5_plugin_initvt_fn);
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/localauth_plugin.h:35"]
pub mod localauth_plugin_h {
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
 * Declarations for localauth plugin module implementors.
 *
 * The localauth pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * Localauth plugin modules should define a function named
 * localauth_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   localauth_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                            krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_localauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* An abstract type for localauth module data. */
    #[c2rust::src_loc = "67:1"]
    pub type krb5_localauth_moddata = *mut krb5_localauth_moddata_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:16"]
    pub struct krb5_localauth_vtable_st {
        pub name: *const libc::c_char,
        pub an2ln_types: *mut *const libc::c_char,
        pub init: krb5_localauth_init_fn,
        pub fini: krb5_localauth_fini_fn,
        pub userok: krb5_localauth_userok_fn,
        pub an2ln: krb5_localauth_an2ln_fn,
        pub free_string: krb5_localauth_free_string_fn,
    }
    /*
 * Optional (mandatory if an2ln is implemented): Release the memory returned by
 * an invocation of an2ln.
 */
    #[c2rust::src_loc = "122:1"]
    pub type krb5_localauth_free_string_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_localauth_moddata,
                                    _: *mut libc::c_char) -> ()>;
    /*
 * Optional (mandatory if an2ln_types is set): Determine the local account name
 * corresponding to aname.  Return 0 and set *lname_out if a mapping can be
 * determined; the contents of *lname_out will later be released with a call to
 * the module's free_string method.  Return KRB5_LNAME_NOTRANS if no mapping
 * can be determined.  Return any other error code for a serious failure to
 * process the request; this will halt the krb5_aname_to_localname operation.
 *
 * If the module's an2ln_types field is set, this method will only be invoked
 * when a profile "auth_to_local" value references one of the module's types.
 * type and residual will be set to the type and residual of the auth_to_local
 * value.
 *
 * If the module's an2ln_types field is not set but the an2ln method is
 * implemented, this method will be invoked independently of the profile's
 * auth_to_local settings, with type and residual set to NULL.  If multiple
 * modules are registered with an2ln methods but no an2ln_types field, the
 * order of invocation is not defined, but all such modules will be consulted
 * before the built-in mechanisms are tried.
 */
    #[c2rust::src_loc = "113:1"]
    pub type krb5_localauth_an2ln_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_localauth_moddata,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char,
                                    _: krb5_const_principal,
                                    _: *mut *mut libc::c_char)
                   -> krb5_error_code>;
    /*
 * Optional: Determine whether aname is authorized to log in as the local
 * account lname.  Return 0 if aname is authorized, EPERM if aname is
 * authoritatively not authorized, KRB5_PLUGIN_NO_HANDLE if the module cannot
 * determine whether aname is authorized, and any other error code for a
 * serious failure to process the request.  aname will be considered authorized
 * if at least one module returns 0 and all other modules return
 * KRB5_PLUGIN_NO_HANDLE.
 */
    #[c2rust::src_loc = "89:1"]
    pub type krb5_localauth_userok_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_localauth_moddata,
                                    _: krb5_const_principal,
                                    _: *const libc::c_char)
                   -> krb5_error_code>;
    /* Optional: Release resources used by module data. */
    #[c2rust::src_loc = "77:1"]
    pub type krb5_localauth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_localauth_moddata) -> ()>;
    /* ** Method type declarations ***/
    /* Optional: Initialize module data. */
    #[c2rust::src_loc = "72:1"]
    pub type krb5_localauth_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut krb5_localauth_moddata)
                   -> krb5_error_code>;
    /* localauth vtable for major version 1. */
    #[c2rust::src_loc = "127:1"]
    pub type krb5_localauth_vtable = *mut krb5_localauth_vtable_st;
    use super::krb5_h::{krb5_context, krb5_error_code, krb5_const_principal};
    extern "C" {
        #[c2rust::src_loc = "67:16"]
        pub type krb5_localauth_moddata_st;
    }
    /* Mandatory: name of module. */
    /* Optional: uppercase auth_to_local types */
    /* Minor version 1 ends here. */
    /* KRB5_LOCALAUTH_PLUGIN_H */
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
        #[c2rust::src_loc = "77:1"]
        pub fn profile_free_list(list: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn profile_get_values(profile: profile_t,
                                  names: *const *const libc::c_char,
                                  ret_values: *mut *mut *mut libc::c_char)
         -> libc::c_long;
    }
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:33"]
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
#[c2rust::header_src = "/usr/include/string.h:33"]
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
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:34"]
pub mod os_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    use super::plugin_h::{krb5_plugin_vtable_st, krb5_plugin_vtable};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "211:1"]
        pub fn localauth_rule_initvt(context: krb5_context,
                                     maj_ver: libc::c_int,
                                     min_ver: libc::c_int,
                                     vtable: krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn localauth_k5login_initvt(context: krb5_context,
                                        maj_ver: libc::c_int,
                                        min_ver: libc::c_int,
                                        vtable: krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "216:1"]
        pub fn localauth_an2ln_initvt(context: krb5_context,
                                      maj_ver: libc::c_int,
                                      min_ver: libc::c_int,
                                      vtable: krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "209:1"]
        pub fn localauth_names_initvt(context: krb5_context,
                                      maj_ver: libc::c_int,
                                      min_ver: libc::c_int,
                                      vtable: krb5_plugin_vtable)
         -> krb5_error_code;
    }
    /* KRB5_LIBOS_INT_PROTO__ */
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
                       krb5_get_default_realm};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, k5alloc, k5calloc, k5memdup0,
                         data_eq_string, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, k5_plugin_register,
                         k5_plugin_load_all, k5_plugin_free_modules};
pub use self::k5_err_h::errinfo;
pub use self::localauth_plugin_h::{krb5_localauth_moddata,
                                   krb5_localauth_vtable_st,
                                   krb5_localauth_free_string_fn,
                                   krb5_localauth_an2ln_fn,
                                   krb5_localauth_userok_fn,
                                   krb5_localauth_fini_fn,
                                   krb5_localauth_init_fn,
                                   krb5_localauth_vtable,
                                   krb5_localauth_moddata_st};
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_free_list, profile_get_values};
pub use self::plugin_h::{krb5_plugin_initvt_fn, krb5_plugin_vtable,
                         krb5_plugin_vtable_st};
use self::stdlib_h::{calloc, free};
use self::k5_platform_h::krb5int_strlcpy;
use self::string_h::{strlen, strchr, strdup, strcmp, memcmp, memcpy};
use self::k5_trace_h::krb5int_trace;
use self::os_proto_h::{localauth_rule_initvt, localauth_k5login_initvt,
                       localauth_an2ln_initvt, localauth_names_initvt};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/os/localauth.c - Authorize access to local accounts */
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
#[c2rust::src_loc = "37:8"]
pub struct localauth_module_handle {
    pub vt: krb5_localauth_vtable_st,
    pub data: krb5_localauth_moddata,
}
/* Release a list of localauth module handles. */
#[c2rust::src_loc = "50:1"]
unsafe extern "C" fn free_handles(mut context: krb5_context,
                                  mut handles:
                                      *mut *mut localauth_module_handle) {
    let mut h: *mut localauth_module_handle =
        0 as *mut localauth_module_handle;
    let mut hp: *mut *mut localauth_module_handle =
        0 as *mut *mut localauth_module_handle;
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
/* Find a localauth module whose an2ln_types contains a match for type. */
#[c2rust::src_loc = "67:1"]
unsafe extern "C" fn find_typed_module(mut handles:
                                           *mut *mut localauth_module_handle,
                                       mut type_0: *const libc::c_char)
 -> *mut localauth_module_handle {
    let mut hp: *mut *mut localauth_module_handle =
        0 as *mut *mut localauth_module_handle;
    let mut h: *mut localauth_module_handle =
        0 as *mut localauth_module_handle;
    let mut tp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    hp = handles;
    while !(*hp).is_null() {
        h = *hp;
        tp = (*h).vt.an2ln_types;
        while !tp.is_null() && !(*tp).is_null() {
            if strcmp(*tp, type_0) == 0 as libc::c_int { return h }
            tp = tp.offset(1)
        }
        hp = hp.offset(1)
    }
    return 0 as *mut localauth_module_handle;
}
/* Generate a trace log and return 1 if the an2ln_types of handle conflicts
 * with any of the handles in list.  Return 0 otherwise. */
#[c2rust::src_loc = "85:1"]
unsafe extern "C" fn check_conflict(mut context: krb5_context,
                                    mut list:
                                        *mut *mut localauth_module_handle,
                                    mut handle: *mut localauth_module_handle)
 -> libc::c_int {
    let mut conflict: *mut localauth_module_handle =
        0 as *mut localauth_module_handle;
    let mut tp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    tp = (*handle).vt.an2ln_types;
    while !tp.is_null() && !(*tp).is_null() {
        conflict = find_typed_module(list, *tp);
        if !conflict.is_null() {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"Ignoring localauth module {str} because it conflicts with an2ln type {str} from module {str}\x00"
                                  as *const u8 as *const libc::c_char,
                              (*conflict).vt.name, *tp, (*handle).vt.name);
            }
            return 1 as libc::c_int
        }
        tp = tp.offset(1)
    }
    return 0 as libc::c_int;
}
/* Get the registered localauth modules including all built-in modules, in the
 * proper order. */
#[c2rust::src_loc = "105:1"]
unsafe extern "C" fn get_modules(mut context: krb5_context,
                                 mut modules_out:
                                     *mut *mut krb5_plugin_initvt_fn)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let intf: libc::c_int = 5 as libc::c_int;
    *modules_out = 0 as *mut krb5_plugin_initvt_fn;
    /* Register built-in modules. */
    ret =
        k5_plugin_register(context, intf,
                           b"default\x00" as *const u8 as *const libc::c_char,
                           Some(localauth_default_initvt as
                                    unsafe extern "C" fn(_: krb5_context,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             krb5_plugin_vtable)
                                        -> krb5_error_code));
    if ret != 0 { return ret }
    ret =
        k5_plugin_register(context, intf,
                           b"rule\x00" as *const u8 as *const libc::c_char,
                           Some(localauth_rule_initvt as
                                    unsafe extern "C" fn(_: krb5_context,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             krb5_plugin_vtable)
                                        -> krb5_error_code));
    if ret != 0 { return ret }
    ret =
        k5_plugin_register(context, intf,
                           b"names\x00" as *const u8 as *const libc::c_char,
                           Some(localauth_names_initvt as
                                    unsafe extern "C" fn(_: krb5_context,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             krb5_plugin_vtable)
                                        -> krb5_error_code));
    if ret != 0 { return ret }
    ret =
        k5_plugin_register(context, intf,
                           b"auth_to_local\x00" as *const u8 as
                               *const libc::c_char,
                           Some(localauth_auth_to_local_initvt as
                                    unsafe extern "C" fn(_: krb5_context,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             krb5_plugin_vtable)
                                        -> krb5_error_code));
    if ret != 0 { return ret }
    ret =
        k5_plugin_register(context, intf,
                           b"k5login\x00" as *const u8 as *const libc::c_char,
                           Some(localauth_k5login_initvt as
                                    unsafe extern "C" fn(_: krb5_context,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             krb5_plugin_vtable)
                                        -> krb5_error_code));
    if ret != 0 { return ret }
    ret =
        k5_plugin_register(context, intf,
                           b"an2ln\x00" as *const u8 as *const libc::c_char,
                           Some(localauth_an2ln_initvt as
                                    unsafe extern "C" fn(_: krb5_context,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             krb5_plugin_vtable)
                                        -> krb5_error_code));
    if ret != 0 { return ret }
    ret = k5_plugin_load_all(context, intf, modules_out);
    if ret != 0 { return ret }
    return 0 as libc::c_int;
}
/* Initialize context->localauth_handles with a list of module handles. */
#[c2rust::src_loc = "144:1"]
unsafe extern "C" fn load_localauth_modules(mut context: krb5_context)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut list: *mut *mut localauth_module_handle =
        0 as *mut *mut localauth_module_handle;
    let mut handle: *mut localauth_module_handle =
        0 as *mut localauth_module_handle;
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
            k5calloc(count.wrapping_add(1 as libc::c_int as libc::c_ulong),
                     ::std::mem::size_of::<*mut localauth_module_handle>() as
                         libc::c_ulong, &mut ret) as
                *mut *mut localauth_module_handle;
        if !list.is_null() {
            /* Initialize each module, ignoring ones that fail. */
            count = 0 as libc::c_int as size_t;
            mod_0 = modules;
            loop  {
                if !(*mod_0).is_some() {
                    current_block = 9520865839495247062;
                    break ;
                }
                handle =
                    k5alloc(::std::mem::size_of::<localauth_module_handle>()
                                as libc::c_ulong, &mut ret) as
                        *mut localauth_module_handle;
                if handle.is_null() {
                    current_block = 2262972324549050694;
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
                                                                     *mut krb5_localauth_vtable_st
                                                                     as
                                                                     krb5_plugin_vtable);
                if ret != 0 as libc::c_int {
                    if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"localauth module failed to init vtable: {kerr}\x00"
                                          as *const u8 as *const libc::c_char,
                                      ret);
                    }
                    free(handle as *mut libc::c_void);
                } else if !(check_conflict(context, list, handle) != 0) {
                    (*handle).data = 0 as krb5_localauth_moddata;
                    if (*handle).vt.init.is_some() {
                        ret =
                            (*handle).vt.init.expect("non-null function pointer")(context,
                                                                                  &mut (*handle).data);
                        if ret != 0 as libc::c_int {
                            if (*context).trace_callback.is_some() {
                                krb5int_trace(context,
                                              b"localauth module {str} failed to init: {kerr}\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              (*handle).vt.name, ret);
                            }
                            free(handle as *mut libc::c_void);
                            current_block = 13536709405535804910;
                        } else { current_block = 3275366147856559585; }
                    } else { current_block = 3275366147856559585; }
                    match current_block {
                        13536709405535804910 => { }
                        _ => {
                            let fresh0 = count;
                            count = count.wrapping_add(1);
                            let ref mut fresh1 =
                                *list.offset(fresh0 as isize);
                            *fresh1 = handle;
                            let ref mut fresh2 = *list.offset(count as isize);
                            *fresh2 = 0 as *mut localauth_module_handle
                        }
                    }
                }
                mod_0 = mod_0.offset(1)
            }
            match current_block {
                2262972324549050694 => { }
                _ => {
                    let ref mut fresh3 = *list.offset(count as isize);
                    *fresh3 = 0 as *mut localauth_module_handle;
                    ret = 0 as libc::c_int;
                    (*context).localauth_handles = list;
                    list = 0 as *mut *mut localauth_module_handle
                }
            }
        }
    }
    k5_plugin_free_modules(context, modules);
    free_handles(context, list);
    return ret;
}
/* Invoke a module's userok method, if it has one. */
#[c2rust::src_loc = "203:1"]
unsafe extern "C" fn userok(mut context: krb5_context,
                            mut h: *mut localauth_module_handle,
                            mut aname: krb5_const_principal,
                            mut lname: *const libc::c_char)
 -> krb5_error_code {
    if (*h).vt.userok.is_none() {
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    return (*h).vt.userok.expect("non-null function pointer")(context,
                                                              (*h).data,
                                                              aname, lname);
}
/* Invoke a module's an2ln method, if it has one. */
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn an2ln(mut context: krb5_context,
                           mut h: *mut localauth_module_handle,
                           mut type_0: *const libc::c_char,
                           mut residual: *const libc::c_char,
                           mut aname: krb5_const_principal,
                           mut lname_out: *mut *mut libc::c_char)
 -> krb5_error_code {
    if (*h).vt.an2ln.is_none() {
        return -(1765328208 as libc::c_long) as krb5_error_code
    }
    return (*h).vt.an2ln.expect("non-null function pointer")(context,
                                                             (*h).data,
                                                             type_0, residual,
                                                             aname,
                                                             lname_out);
}
/* Invoke a module's free_string method. */
#[c2rust::src_loc = "224:1"]
unsafe extern "C" fn free_lname(mut context: krb5_context,
                                mut h: *mut localauth_module_handle,
                                mut str: *mut libc::c_char) {
    (*h).vt.free_string.expect("non-null function pointer")(context,
                                                            (*h).data, str);
}
/* Parse a TYPE or TYPE:residual string into its components. */
#[c2rust::src_loc = "231:1"]
unsafe extern "C" fn parse_mapping_value(mut value: *const libc::c_char,
                                         mut type_out: *mut *mut libc::c_char,
                                         mut residual_out:
                                             *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut residual: *mut libc::c_char = 0 as *mut libc::c_char;
    *type_out = 0 as *mut libc::c_char;
    *residual_out = 0 as *mut libc::c_char;
    p = strchr(value, ':' as i32);
    if p.is_null() {
        type_0 = strdup(value);
        if type_0.is_null() { return 12 as libc::c_int }
        residual = 0 as *mut libc::c_char
    } else {
        type_0 =
            k5memdup0(value as *const libc::c_void,
                      p.wrapping_offset_from(value) as libc::c_long as size_t,
                      &mut ret) as *mut libc::c_char;
        if type_0.is_null() { return ret }
        residual = strdup(p.offset(1 as libc::c_int as isize));
        if residual.is_null() {
            free(type_0 as *mut libc::c_void);
            return 12 as libc::c_int
        }
    }
    *type_out = type_0;
    *residual_out = residual;
    return 0 as libc::c_int;
}
/* Apply the default an2ln method, which translates name@defaultrealm or
 * name/defaultrealm@defaultrealm to name. */
#[c2rust::src_loc = "263:1"]
unsafe extern "C" fn an2ln_default(mut context: krb5_context,
                                   mut data: krb5_localauth_moddata,
                                   mut type_0: *const libc::c_char,
                                   mut residual: *const libc::c_char,
                                   mut aname: krb5_const_principal,
                                   mut lname_out: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut def_realm: *mut libc::c_char = 0 as *mut libc::c_char;
    *lname_out = 0 as *mut libc::c_char;
    ret = krb5_get_default_realm(context, &mut def_realm);
    if ret != 0 { return -(1765328208 as libc::c_long) as krb5_error_code }
    if data_eq_string((*aname).realm, def_realm) == 0 {
        ret = -(1765328208 as libc::c_long) as krb5_error_code
    } else if (*aname).length == 2 as libc::c_int {
        /* Allow a second component if it is the local realm. */
        if data_eq_string(*(*aname).data.offset(1 as libc::c_int as isize),
                          def_realm) == 0 {
            ret = -(1765328208 as libc::c_long) as krb5_error_code
        }
    } else if (*aname).length != 1 as libc::c_int {
        ret = -(1765328208 as libc::c_long) as krb5_error_code
    }
    free(def_realm as *mut libc::c_void);
    if ret != 0 { return ret }
    *lname_out =
        k5memdup0((*(*aname).data.offset(0 as libc::c_int as isize)).data as
                      *const libc::c_void,
                  (*(*aname).data.offset(0 as libc::c_int as isize)).length as
                      size_t, &mut ret) as *mut libc::c_char;
    return ret;
}
/*
 * Perform aname-to-lname translation using the auth_to_local values in the
 * default realm's profile section.  If no values exist, fall back to the
 * default method.
 */
#[c2rust::src_loc = "299:1"]
unsafe extern "C" fn an2ln_auth_to_local(mut context: krb5_context,
                                         mut data: krb5_localauth_moddata,
                                         mut type_arg: *const libc::c_char,
                                         mut residual_arg:
                                             *const libc::c_char,
                                         mut aname: krb5_const_principal,
                                         mut lname_out:
                                             *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut h: *mut localauth_module_handle =
        0 as *mut localauth_module_handle;
    let mut realm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mapping_values: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut residual: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hierarchy: [*const libc::c_char; 4] =
        [0 as *const libc::c_char; 4];
    let mut i: size_t = 0;
    *lname_out = 0 as *mut libc::c_char;
    /* Fetch the profile values for realms-><defaultrealm>->auth_to_local. */
    ret = krb5_get_default_realm(context, &mut realm);
    if ret != 0 { return -(1765328208 as libc::c_long) as krb5_error_code }
    hierarchy[0 as libc::c_int as usize] =
        b"realms\x00" as *const u8 as *const libc::c_char;
    hierarchy[1 as libc::c_int as usize] = realm;
    hierarchy[2 as libc::c_int as usize] =
        b"auth_to_local\x00" as *const u8 as *const libc::c_char;
    hierarchy[3 as libc::c_int as usize] = 0 as *const libc::c_char;
    ret =
        profile_get_values((*context).profile, hierarchy.as_mut_ptr(),
                           &mut mapping_values) as krb5_error_code;
    if ret != 0 {
        /* Use default method if there are no auth_to_local values. */
        ret =
            an2ln_default(context, data, 0 as *const libc::c_char,
                          0 as *const libc::c_char, aname, lname_out)
    } else {
        ret = -(1765328208 as libc::c_long) as krb5_error_code;
        i = 0 as libc::c_int as size_t;
        while !(*mapping_values.offset(i as isize)).is_null() &&
                  ret as libc::c_long == -(1765328208 as libc::c_long) {
            ret =
                parse_mapping_value(*mapping_values.offset(i as isize),
                                    &mut type_0, &mut residual);
            if ret != 0 { break ; }
            h = find_typed_module((*context).localauth_handles, type_0);
            if !h.is_null() {
                ret = an2ln(context, h, type_0, residual, aname, &mut lname);
                if ret == 0 as libc::c_int {
                    *lname_out = strdup(lname);
                    if (*lname_out).is_null() { ret = 12 as libc::c_int }
                    free_lname(context, h, lname);
                }
            } else { ret = -(1765328248 as libc::c_long) as krb5_error_code }
            free(type_0 as *mut libc::c_void);
            free(residual as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
    }
    free(realm as *mut libc::c_void);
    profile_free_list(mapping_values);
    return ret;
}
#[c2rust::src_loc = "354:1"]
unsafe extern "C" fn freestr(mut context: krb5_context,
                             mut data: krb5_localauth_moddata,
                             mut str: *mut libc::c_char) {
    free(str as *mut libc::c_void);
}
#[c2rust::src_loc = "360:1"]
unsafe extern "C" fn localauth_auth_to_local_initvt(mut context: krb5_context,
                                                    mut maj_ver: libc::c_int,
                                                    mut min_ver: libc::c_int,
                                                    mut vtable:
                                                        krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_localauth_vtable = vtable as krb5_localauth_vtable;
    (*vt).name = b"auth_to_local\x00" as *const u8 as *const libc::c_char;
    (*vt).an2ln =
        Some(an2ln_auth_to_local as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_localauth_moddata,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char,
                                      _: krb5_const_principal,
                                      _: *mut *mut libc::c_char)
                     -> krb5_error_code);
    (*vt).free_string =
        Some(freestr as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_localauth_moddata,
                                      _: *mut libc::c_char) -> ());
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "372:1"]
unsafe extern "C" fn localauth_default_initvt(mut context: krb5_context,
                                              mut maj_ver: libc::c_int,
                                              mut min_ver: libc::c_int,
                                              mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_localauth_vtable = vtable as krb5_localauth_vtable;
    static mut types: [*const libc::c_char; 2] =
        [b"DEFAULT\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char];
    (*vt).name = b"default\x00" as *const u8 as *const libc::c_char;
    (*vt).an2ln_types = types.as_mut_ptr();
    (*vt).an2ln =
        Some(an2ln_default as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_localauth_moddata,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char,
                                      _: krb5_const_principal,
                                      _: *mut *mut libc::c_char)
                     -> krb5_error_code);
    (*vt).free_string =
        Some(freestr as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_localauth_moddata,
                                      _: *mut libc::c_char) -> ());
    return 0 as libc::c_int;
}
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
/* *
 * Determine if a principal is authorized to log in as a local user.
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal name
 * @param [in] luser            Local username
 *
 * Determine whether @a principal is authorized to log in as a local user @a
 * luser.
 *
 * @retval
 * TRUE Principal is authorized to log in as user; FALSE otherwise.
 */
#[no_mangle]
#[c2rust::src_loc = "386:1"]
pub unsafe extern "C" fn krb5_kuserok(mut context: krb5_context,
                                      mut aname: krb5_principal,
                                      mut lname: *const libc::c_char)
 -> krb5_boolean {
    let mut ret: krb5_error_code = 0;
    let mut hp: *mut *mut localauth_module_handle =
        0 as *mut *mut localauth_module_handle;
    let mut accepted: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    if (*context).localauth_handles.is_null() &&
           load_localauth_modules(context) != 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    /* If any module denies access, return false immediately.  Otherwise,
     * consult all modules and return true if one of them allows access. */
    hp = (*context).localauth_handles;
    while !(*hp).is_null() {
        ret = userok(context, *hp, aname as krb5_const_principal, lname);
        if ret == 0 as libc::c_int {
            accepted = 1 as libc::c_int as krb5_boolean
        } else if ret as libc::c_long != -(1765328135 as libc::c_long) {
            return 0 as libc::c_int as krb5_boolean
        }
        hp = hp.offset(1)
    }
    return accepted;
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
#[no_mangle]
#[c2rust::src_loc = "408:1"]
pub unsafe extern "C" fn krb5_aname_to_localname(mut context: krb5_context,
                                                 mut aname:
                                                     krb5_const_principal,
                                                 mut lnsize: libc::c_int,
                                                 mut lname_out:
                                                     *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut hp: *mut *mut localauth_module_handle =
        0 as *mut *mut localauth_module_handle;
    let mut lname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: size_t = 0;
    if (*context).localauth_handles.is_null() {
        ret = load_localauth_modules(context);
        if ret != 0 { return ret }
    }
    hp = (*context).localauth_handles;
    while !(*hp).is_null() {
        if (**hp).vt.an2ln_types.is_null() {
            ret =
                an2ln(context, *hp, 0 as *const libc::c_char,
                      0 as *const libc::c_char, aname, &mut lname);
            if ret == 0 as libc::c_int {
                sz = krb5int_strlcpy(lname_out, lname, lnsize as size_t);
                free_lname(context, *hp, lname);
                return if sz >= lnsize as size_t {
                           -(1765328247 as libc::c_long)
                       } else { 0 as libc::c_int as libc::c_long } as
                           krb5_error_code
            } else {
                if ret as libc::c_long != -(1765328208 as libc::c_long) {
                    return ret
                }
            }
        }
        hp = hp.offset(1)
    }
    return -(1765328208 as libc::c_long) as krb5_error_code;
}
#[no_mangle]
#[c2rust::src_loc = "438:1"]
pub unsafe extern "C" fn k5_localauth_free_context(mut context:
                                                       krb5_context) {
    free_handles(context, (*context).localauth_handles);
    (*context).localauth_handles = 0 as *mut *mut localauth_module_handle;
}
