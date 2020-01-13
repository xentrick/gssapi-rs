use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
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
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
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
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "8023:1"]
        pub fn krb5_get_error_message(ctx: krb5_context,
                                      code: krb5_error_code)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "8032:1"]
        pub fn krb5_free_error_message(ctx: krb5_context,
                                       msg: *const libc::c_char);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
pub mod k5_int_h {
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
    /* private, in kdb5.h */
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
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
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
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    /* Like k5memdup, but add a final null byte. */
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
        #[no_mangle]
        #[c2rust::src_loc = "1778:1"]
        pub fn k5_parse_host_string(address: *const libc::c_char,
                                    default_port: libc::c_int,
                                    host_out: *mut *mut libc::c_char,
                                    port_out: *mut libc::c_int)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
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
    use super::k5_err_h::errinfo;
    extern "C" {
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
        #[no_mangle]
        #[c2rust::src_loc = "103:1"]
        pub fn krb5int_open_plugin_dirs(_: *const *const libc::c_char,
                                        _: *const *const libc::c_char,
                                        _: *mut plugin_dir_handle,
                                        _: *mut errinfo) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn krb5int_get_plugin_dir_data(_: *mut plugin_dir_handle,
                                           _: *const libc::c_char,
                                           _: *mut *mut *mut libc::c_void,
                                           _: *mut errinfo) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn krb5int_free_plugin_dir_data(_: *mut *mut libc::c_void);
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
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn profile_release_string(str: *mut libc::c_char);
    }
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
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:27"]
pub mod socket_type_h {
    #[c2rust::src_loc = "24:1"]
    pub type __socket_type = libc::c_uint;
    #[c2rust::src_loc = "52:3"]
    pub const SOCK_NONBLOCK: __socket_type = 2048;
    #[c2rust::src_loc = "49:3"]
    pub const SOCK_CLOEXEC: __socket_type = 524288;
    #[c2rust::src_loc = "41:3"]
    pub const SOCK_PACKET: __socket_type = 10;
    #[c2rust::src_loc = "39:3"]
    pub const SOCK_DCCP: __socket_type = 6;
    #[c2rust::src_loc = "36:3"]
    pub const SOCK_SEQPACKET: __socket_type = 5;
    #[c2rust::src_loc = "34:3"]
    pub const SOCK_RDM: __socket_type = 4;
    #[c2rust::src_loc = "32:3"]
    pub const SOCK_RAW: __socket_type = 3;
    #[c2rust::src_loc = "29:3"]
    pub const SOCK_DGRAM: __socket_type = 2;
    #[c2rust::src_loc = "26:3"]
    pub const SOCK_STREAM: __socket_type = 1;
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:27"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:27"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:8"]
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        pub __ss_padding: [libc::c_char; 118],
        pub __ss_align: libc::c_ulong,
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:27"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "253:8"]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct in6_addr {
        pub __in6_u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [uint8_t; 16],
        pub __u6_addr16: [uint16_t; 8],
        pub __u6_addr32: [uint32_t; 4],
    }
    #[c2rust::src_loc = "119:1"]
    pub type in_port_t = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "238:8"]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
}
#[c2rust::header_src = "/usr/include/netdb.h:27"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "255:8"]
    pub struct servent {
        pub s_name: *mut libc::c_char,
        pub s_aliases: *mut *mut libc::c_char,
        pub s_port: libc::c_int,
        pub s_proto: *mut libc::c_char,
    }
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "288:1"]
        pub fn getservbyname(__name: *const libc::c_char,
                             __proto: *const libc::c_char) -> *mut servent;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:29"]
pub mod os_proto_h {
    /* HAVE_NETINET_IN_H */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:8"]
    pub struct srv_dns_entry {
        pub next: *mut srv_dns_entry,
        pub priority: libc::c_int,
        pub weight: libc::c_int,
        pub port: libc::c_ushort,
        pub host: *mut libc::c_char,
    }
    /* A single server hostname or address. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:8"]
    pub struct server_entry {
        pub hostname: *mut libc::c_char,
        pub port: libc::c_int,
        pub transport: k5_transport,
        pub uri_path: *mut libc::c_char,
        pub family: libc::c_int,
        pub master: libc::c_int,
        pub addrlen: size_t,
        pub addr: sockaddr_storage,
    }
    #[c2rust::src_loc = "41:9"]
    pub type k5_transport = libc::c_uint;
    #[c2rust::src_loc = "45:5"]
    pub const HTTPS: k5_transport = 3;
    #[c2rust::src_loc = "44:5"]
    pub const UDP: k5_transport = 2;
    #[c2rust::src_loc = "43:5"]
    pub const TCP: k5_transport = 1;
    #[c2rust::src_loc = "42:5"]
    pub const TCP_OR_UDP: k5_transport = 0;
    /* A list of server hostnames/addresses. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:8"]
    pub struct serverlist {
        pub servers: *mut server_entry,
        pub nservers: size_t,
    }
    use super::stddef_h::size_t;
    use super::socket_h::sockaddr_storage;
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_data, krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "125:1"]
        pub fn krb5int_make_srv_query_realm(context: krb5_context,
                                            realm: *const krb5_data,
                                            service: *const libc::c_char,
                                            protocol: *const libc::c_char,
                                            answers: *mut *mut srv_dns_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn krb5int_free_srv_dns_data(_: *mut srv_dns_entry);
        #[no_mangle]
        #[c2rust::src_loc = "132:1"]
        pub fn k5_make_uri_query(context: krb5_context,
                                 realm: *const krb5_data,
                                 service: *const libc::c_char,
                                 answers: *mut *mut srv_dns_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "143:1"]
        pub fn _krb5_conf_boolean(_: *const libc::c_char) -> libc::c_int;
    }
    /* KRB5_LIBOS_INT_PROTO__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/locate_plugin.h:29"]
pub mod locate_plugin_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:16"]
    pub struct krb5plugin_service_locate_ftable {
        pub minor_version: libc::c_int,
        pub init: Option<unsafe extern "C" fn(_: krb5_context,
                                              _: *mut *mut libc::c_void)
                             -> krb5_error_code>,
        pub fini: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub lookup: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                _: locate_service_type,
                                                _: *const libc::c_char,
                                                _: libc::c_int,
                                                _: libc::c_int,
                                                _:
                                                    Option<unsafe extern "C" fn(_:
                                                                                    *mut libc::c_void,
                                                                                _:
                                                                                    libc::c_int,
                                                                                _:
                                                                                    *mut sockaddr)
                                                               ->
                                                                   libc::c_int>,
                                                _: *mut libc::c_void)
                               -> krb5_error_code>,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2006 Massachusetts Institute of Technology.
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
 * Service location plugin definitions for Kerberos 5.
 */
    #[c2rust::src_loc = "35:1"]
    pub type locate_service_type = libc::c_uint;
    #[c2rust::src_loc = "40:5"]
    pub const locate_service_kpasswd: locate_service_type = 5;
    #[c2rust::src_loc = "39:5"]
    pub const locate_service_krb524: locate_service_type = 4;
    #[c2rust::src_loc = "38:5"]
    pub const locate_service_kadmin: locate_service_type = 3;
    #[c2rust::src_loc = "37:5"]
    pub const locate_service_master_kdc: locate_service_type = 2;
    #[c2rust::src_loc = "36:5"]
    pub const locate_service_kdc: locate_service_type = 1;
    use super::krb5_h::{krb5_error_code, krb5_context};
    use super::socket_h::sockaddr;
    /* extern krb5plugin_service_locate_ftable service_locator; */
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
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
#[c2rust::header_src = "/usr/include/libintl.h:27"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/strings.h:27"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "120:12"]
        pub fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char,
                           _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
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
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:27"]
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
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _profile_t,
                       krb5_set_error_message, krb5_get_error_message,
                       krb5_free_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, k5calloc, k5alloc, k5memdup0,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, k5_parse_host_string};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle,
                            krb5int_open_plugin_dirs,
                            krb5int_get_plugin_dir_data,
                            krb5int_free_plugin_dir_data};
pub use self::profile_h::{profile_t, profile_get_values, profile_free_list,
                          profile_get_string, profile_get_boolean,
                          profile_release_string};
pub use self::com_err_h::{errcode_t, error_message};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t};
pub use self::netdb_h::{servent, getservbyname};
pub use self::os_proto_h::{srv_dns_entry, server_entry, k5_transport, HTTPS,
                           UDP, TCP, TCP_OR_UDP, serverlist,
                           krb5int_make_srv_query_realm,
                           krb5int_free_srv_dns_data, k5_make_uri_query,
                           _krb5_conf_boolean};
pub use self::locate_plugin_h::{krb5plugin_service_locate_ftable,
                                locate_service_type, locate_service_kpasswd,
                                locate_service_krb524, locate_service_kadmin,
                                locate_service_master_kdc,
                                locate_service_kdc};
use self::stdlib_h::{calloc, realloc, free};
use self::libintl_h::dgettext;
use self::strings_h::strncasecmp;
use self::string_h::{strchr, strdup, strncmp, strcmp, memcmp, memset, memcpy};
use self::k5_trace_h::krb5int_trace;
extern "C" {
    #[c2rust::src_loc = "388:12"]
    pub type krb5plugin_service_locate_result;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "353:8"]
pub struct module_callback_data {
    pub out_of_mem: libc::c_int,
    pub list: *mut serverlist,
}
#[c2rust::src_loc = "41:1"]
unsafe extern "C" fn maybe_use_dns(mut context: krb5_context,
                                   mut name: *const libc::c_char,
                                   mut defalt: libc::c_int) -> libc::c_int {
    let mut code: krb5_error_code = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut use_dns: libc::c_int = 0 as libc::c_int;
    code =
        profile_get_string((*context).profile,
                           b"libdefaults\x00" as *const u8 as
                               *const libc::c_char, name,
                           0 as *const libc::c_char, 0 as *const libc::c_char,
                           &mut value) as krb5_error_code;
    if value.is_null() && code == 0 as libc::c_int {
        code =
            profile_get_string((*context).profile,
                               b"libdefaults\x00" as *const u8 as
                                   *const libc::c_char,
                               b"dns_fallback\x00" as *const u8 as
                                   *const libc::c_char,
                               0 as *const libc::c_char,
                               0 as *const libc::c_char, &mut value) as
                krb5_error_code
    }
    if code != 0 { return defalt }
    if value.is_null() { return defalt }
    use_dns = _krb5_conf_boolean(value);
    profile_release_string(value);
    return use_dns;
}
#[c2rust::src_loc = "65:1"]
unsafe extern "C" fn use_dns_uri(mut ctx: krb5_context) -> krb5_boolean {
    let mut ret: krb5_error_code = 0;
    let mut use_0: libc::c_int = 0;
    ret =
        profile_get_boolean((*ctx).profile,
                            b"libdefaults\x00" as *const u8 as
                                *const libc::c_char,
                            b"dns_uri_lookup\x00" as *const u8 as
                                *const libc::c_char, 0 as *const libc::c_char,
                            1 as libc::c_int, &mut use_0) as krb5_error_code;
    return if ret != 0 { 1 as libc::c_int } else { use_0 } as krb5_boolean;
}
#[no_mangle]
#[c2rust::src_loc = "77:1"]
pub unsafe extern "C" fn _krb5_use_dns_kdc(mut context: krb5_context)
 -> libc::c_int {
    return maybe_use_dns(context,
                         b"dns_lookup_kdc\x00" as *const u8 as
                             *const libc::c_char, 1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "84:1"]
pub unsafe extern "C" fn _krb5_use_dns_realm(mut context: krb5_context)
 -> libc::c_int {
    return maybe_use_dns(context,
                         b"dns_lookup_realm\x00" as *const u8 as
                             *const libc::c_char, 0 as libc::c_int);
}
/* KRB5_DNS_LOOKUP */
/* Free up everything pointed to by the serverlist structure, but don't
 * free the structure itself. */
#[no_mangle]
#[c2rust::src_loc = "95:1"]
pub unsafe extern "C" fn k5_free_serverlist(mut list: *mut serverlist) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*list).nservers {
        free((*(*list).servers.offset(i as isize)).hostname as
                 *mut libc::c_void);
        free((*(*list).servers.offset(i as isize)).uri_path as
                 *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free((*list).servers as *mut libc::c_void);
    (*list).servers = 0 as *mut server_entry;
    (*list).nservers = 0 as libc::c_int as size_t;
}
#[inline]
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn Tprintf(mut fmt: *const libc::c_char, mut args: ...) { }
/* Make room for a new server entry in list and return a pointer to the new
 * entry.  (Do not increment list->nservers.) */
#[c2rust::src_loc = "123:1"]
unsafe extern "C" fn new_server_entry(mut list: *mut serverlist)
 -> *mut server_entry {
    let mut newservers: *mut server_entry = 0 as *mut server_entry;
    let mut entry: *mut server_entry = 0 as *mut server_entry;
    let mut newspace: size_t =
        (*list).nservers.wrapping_add(1 as libc::c_int as
                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<server_entry>()
                                                                          as
                                                                          libc::c_ulong);
    newservers =
        realloc((*list).servers as *mut libc::c_void, newspace) as
            *mut server_entry;
    if newservers.is_null() { return 0 as *mut server_entry }
    (*list).servers = newservers;
    entry =
        &mut *newservers.offset((*list).nservers as isize) as
            *mut server_entry;
    memset(entry as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<server_entry>() as libc::c_ulong);
    (*entry).master = -(1 as libc::c_int);
    return entry;
}
/* Add an address entry to list. */
#[c2rust::src_loc = "140:1"]
unsafe extern "C" fn add_addr_to_list(mut list: *mut serverlist,
                                      mut transport: k5_transport,
                                      mut family: libc::c_int,
                                      mut addrlen: size_t,
                                      mut addr: *mut sockaddr)
 -> libc::c_int {
    let mut entry: *mut server_entry = 0 as *mut server_entry;
    entry = new_server_entry(list);
    if entry.is_null() { return 12 as libc::c_int }
    (*entry).transport = transport;
    (*entry).family = family;
    (*entry).hostname = 0 as *mut libc::c_char;
    (*entry).uri_path = 0 as *mut libc::c_char;
    (*entry).addrlen = addrlen;
    memcpy(&mut (*entry).addr as *mut sockaddr_storage as *mut libc::c_void,
           addr as *const libc::c_void, addrlen);
    (*list).nservers = (*list).nservers.wrapping_add(1);
    return 0 as libc::c_int;
}
/* Add a hostname entry to list. */
#[c2rust::src_loc = "160:1"]
unsafe extern "C" fn add_host_to_list(mut list: *mut serverlist,
                                      mut hostname: *const libc::c_char,
                                      mut port: libc::c_int,
                                      mut transport: k5_transport,
                                      mut family: libc::c_int,
                                      mut uri_path: *const libc::c_char,
                                      mut master: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut entry: *mut server_entry = 0 as *mut server_entry;
    entry = new_server_entry(list);
    if entry.is_null() { return 12 as libc::c_int }
    (*entry).transport = transport;
    (*entry).family = family;
    (*entry).hostname = strdup(hostname);
    if !(*entry).hostname.is_null() {
        if !uri_path.is_null() {
            (*entry).uri_path = strdup(uri_path);
            if (*entry).uri_path.is_null() {
                current_block = 4179030124418297737;
            } else { current_block = 11812396948646013369; }
        } else { current_block = 11812396948646013369; }
        match current_block {
            4179030124418297737 => { }
            _ => {
                (*entry).port = port;
                (*entry).master = master;
                (*list).nservers = (*list).nservers.wrapping_add(1);
                return 0 as libc::c_int
            }
        }
    }
    free((*entry).hostname as *mut libc::c_void);
    (*entry).hostname = 0 as *mut libc::c_char;
    return 12 as libc::c_int;
}
#[c2rust::src_loc = "190:1"]
unsafe extern "C" fn parse_uri_if_https(mut host_or_uri: *const libc::c_char,
                                        mut transport: *mut k5_transport,
                                        mut host: *mut *const libc::c_char,
                                        mut uri_path:
                                            *mut *const libc::c_char) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if strncmp(host_or_uri,
               b"https://\x00" as *const u8 as *const libc::c_char,
               8 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        *transport = HTTPS;
        *host = host_or_uri.offset(8 as libc::c_int as isize);
        cp = strchr(*host, '/' as i32);
        if !cp.is_null() {
            *cp = '\u{0}' as i32 as libc::c_char;
            *uri_path = cp.offset(1 as libc::c_int as isize)
        }
    };
}
/* Return true if server is identical to an entry in list. */
#[c2rust::src_loc = "209:1"]
unsafe extern "C" fn server_list_contains(mut list: *mut serverlist,
                                          mut server: *mut server_entry)
 -> krb5_boolean {
    let mut ent: *mut server_entry = 0 as *mut server_entry;
    ent = (*list).servers;
    while ent < (*list).servers.offset((*list).nservers as isize) {
        if !(*server).hostname.is_null() && !(*ent).hostname.is_null() &&
               strcmp((*server).hostname, (*ent).hostname) == 0 as libc::c_int
           {
            return 1 as libc::c_int as krb5_boolean
        }
        if (*server).hostname.is_null() && (*ent).hostname.is_null() &&
               (*server).addrlen == (*ent).addrlen &&
               memcmp(&mut (*server).addr as *mut sockaddr_storage as
                          *const libc::c_void,
                      &mut (*ent).addr as *mut sockaddr_storage as
                          *const libc::c_void, (*server).addrlen) ==
                   0 as libc::c_int {
            return 1 as libc::c_int as krb5_boolean
        }
        ent = ent.offset(1)
    }
    return 0 as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "226:1"]
unsafe extern "C" fn locate_srv_conf_1(mut context: krb5_context,
                                       mut realm: *const krb5_data,
                                       mut name: *const libc::c_char,
                                       mut serverlist: *mut serverlist,
                                       mut transport: k5_transport,
                                       mut udpport: libc::c_int)
 -> krb5_error_code {
    let mut realm_srv_names: [*const libc::c_char; 4] =
        [0 as *const libc::c_char; 4];
    let mut hostlist: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut realmstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostspec: *const libc::c_char = 0 as *const libc::c_char;
    let mut code: krb5_error_code = 0;
    let mut i: libc::c_int = 0;
    let mut default_port: libc::c_int = 0;
    Tprintf(b"looking in krb5.conf for realm %s entry %s; ports %d,%d\n\x00"
                as *const u8 as *const libc::c_char, (*realm).data, name,
            udpport);
    realmstr =
        k5memdup0((*realm).data as *const libc::c_void,
                  (*realm).length as size_t, &mut code) as *mut libc::c_char;
    if !realmstr.is_null() {
        realm_srv_names[0 as libc::c_int as usize] =
            b"realms\x00" as *const u8 as *const libc::c_char;
        realm_srv_names[1 as libc::c_int as usize] = realmstr;
        realm_srv_names[2 as libc::c_int as usize] = name;
        realm_srv_names[3 as libc::c_int as usize] = 0 as *const libc::c_char;
        code =
            profile_get_values((*context).profile,
                               realm_srv_names.as_mut_ptr(), &mut hostlist) as
                krb5_error_code;
        if code != 0 {
            Tprintf(b"config file lookup failed: %s\n\x00" as *const u8 as
                        *const libc::c_char,
                    error_message(code as errcode_t));
            if code as libc::c_long == -(1429577726 as libc::c_long) ||
                   code as libc::c_long == -(1429577725 as libc::c_long) {
                code = 0 as libc::c_int
            }
        } else {
            i = 0 as libc::c_int;
            while !(*hostlist.offset(i as isize)).is_null() {
                let mut port_num: libc::c_int = 0;
                let mut this_transport: k5_transport = transport;
                let mut uri_path: *const libc::c_char =
                    0 as *const libc::c_char;
                hostspec = *hostlist.offset(i as isize);
                Tprintf(b"entry %d is \'%s\'\n\x00" as *const u8 as
                            *const libc::c_char, i, hostspec);
                parse_uri_if_https(hostspec, &mut this_transport,
                                   &mut hostspec, &mut uri_path);
                default_port =
                    if this_transport as libc::c_uint ==
                           HTTPS as libc::c_int as libc::c_uint {
                        443 as libc::c_int
                    } else { udpport };
                code =
                    k5_parse_host_string(hostspec, default_port, &mut host,
                                         &mut port_num);
                if code == 0 as libc::c_int && host.is_null() {
                    code = 22 as libc::c_int
                }
                if code != 0 { break ; }
                code =
                    add_host_to_list(serverlist, host, port_num,
                                     this_transport, 0 as libc::c_int,
                                     uri_path, -(1 as libc::c_int));
                if code != 0 { break ; }
                free(host as *mut libc::c_void);
                host = 0 as *mut libc::c_char;
                i += 1
            }
        }
    }
    free(realmstr as *mut libc::c_void);
    free(host as *mut libc::c_void);
    profile_free_list(hostlist);
    return code;
}
#[c2rust::src_loc = "306:1"]
unsafe extern "C" fn locate_srv_dns_1(mut context: krb5_context,
                                      mut realm: *const krb5_data,
                                      mut service: *const libc::c_char,
                                      mut protocol: *const libc::c_char,
                                      mut serverlist: *mut serverlist)
 -> krb5_error_code {
    let mut head: *mut srv_dns_entry = 0 as *mut srv_dns_entry;
    let mut entry: *mut srv_dns_entry = 0 as *mut srv_dns_entry;
    let mut code: krb5_error_code = 0 as libc::c_int;
    let mut transport: k5_transport = TCP_OR_UDP;
    code =
        krb5int_make_srv_query_realm(context, realm, service, protocol,
                                     &mut head);
    if code != 0 { return 0 as libc::c_int }
    if head.is_null() { return 0 as libc::c_int }
    /* Check for the "." case indicating no support.  */
    if (*head).next.is_null() &&
           *(*head).host.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\u{0}' as i32 {
        code = -(1765328139 as libc::c_long) as krb5_error_code
    } else {
        entry = head;
        while !entry.is_null() {
            transport =
                if strcmp(protocol,
                          b"_tcp\x00" as *const u8 as *const libc::c_char) ==
                       0 as libc::c_int {
                    TCP as libc::c_int
                } else { UDP as libc::c_int } as k5_transport;
            code =
                add_host_to_list(serverlist, (*entry).host,
                                 (*entry).port as libc::c_int, transport,
                                 0 as libc::c_int, 0 as *const libc::c_char,
                                 -(1 as libc::c_int));
            if code != 0 { break ; }
            entry = (*entry).next
        }
    }
    krb5int_free_srv_dns_data(head);
    return code;
}
#[c2rust::src_loc = "350:20"]
static mut objdirs: [*const libc::c_char; 2] =
    [b"/usr/local/lib/krb5/plugins/libkrb5\x00" as *const u8 as
         *const libc::c_char, 0 as *const libc::c_char];
#[c2rust::src_loc = "358:1"]
unsafe extern "C" fn module_callback(mut cbdata: *mut libc::c_void,
                                     mut socktype: libc::c_int,
                                     mut sa: *mut sockaddr) -> libc::c_int {
    let mut d: *mut module_callback_data =
        cbdata as *mut module_callback_data;
    let mut addrlen: size_t = 0;
    let mut transport: k5_transport = TCP_OR_UDP;
    if socktype != SOCK_STREAM as libc::c_int &&
           socktype != SOCK_DGRAM as libc::c_int {
        return 0 as libc::c_int
    }
    if (*sa).sa_family as libc::c_int == 2 as libc::c_int {
        addrlen = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
    } else if (*sa).sa_family as libc::c_int == 10 as libc::c_int {
        addrlen = ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong
    } else { return 0 as libc::c_int }
    transport =
        if socktype == SOCK_STREAM as libc::c_int {
            TCP as libc::c_int
        } else { UDP as libc::c_int } as k5_transport;
    if add_addr_to_list((*d).list, transport, (*sa).sa_family as libc::c_int,
                        addrlen, sa) != 0 as libc::c_int {
        /* Assumes only error is ENOMEM.  */
        (*d).out_of_mem = 1 as libc::c_int; /* NUL-terminated realm */
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "383:1"]
unsafe extern "C" fn module_locate_server(mut ctx: krb5_context,
                                          mut realm: *const krb5_data,
                                          mut serverlist: *mut serverlist,
                                          mut svc: locate_service_type,
                                          mut transport: k5_transport)
 -> krb5_error_code {
    let mut res: *mut krb5plugin_service_locate_result =
        0 as *mut krb5plugin_service_locate_result;
    let mut code: krb5_error_code = 0;
    let mut vtbl: *mut krb5plugin_service_locate_ftable =
        0 as *mut krb5plugin_service_locate_ftable;
    let mut ptrs: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut realmz: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut socktype: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut cbdata: module_callback_data =
        {
            let mut init =
                module_callback_data{out_of_mem: 0 as libc::c_int,
                                     list: 0 as *mut serverlist,};
            init
        };
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    Tprintf(b"in module_locate_server\n\x00" as *const u8 as
                *const libc::c_char);
    cbdata.list = serverlist;
    if (*ctx).libkrb5_plugins.files.is_null() {
        code =
            krb5int_open_plugin_dirs(objdirs.as_mut_ptr(),
                                     0 as *const *const libc::c_char,
                                     &mut (*ctx).libkrb5_plugins,
                                     &mut (*ctx).err) as krb5_error_code;
        if code != 0 {
            return -(1765328135 as libc::c_long) as krb5_error_code
        }
    }
    code =
        krb5int_get_plugin_dir_data(&mut (*ctx).libkrb5_plugins,
                                    b"service_locator\x00" as *const u8 as
                                        *const libc::c_char, &mut ptrs,
                                    &mut (*ctx).err) as krb5_error_code;
    if code != 0 {
        msg = krb5_get_error_message(ctx, code);
        Tprintf(b"error looking up plugin symbols: %s\n\x00" as *const u8 as
                    *const libc::c_char, msg);
        krb5_free_error_message(ctx, msg);
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    if (*realm).length >=
           (2147483647 as libc::c_int as
                libc::c_uint).wrapping_mul(2 as
                                               libc::c_uint).wrapping_add(1 as
                                                                              libc::c_uint)
       {
        krb5int_free_plugin_dir_data(ptrs);
        return 12 as libc::c_int
    }
    realmz =
        k5memdup0((*realm).data as *const libc::c_void,
                  (*realm).length as size_t, &mut code) as *mut libc::c_char;
    if realmz.is_null() { krb5int_free_plugin_dir_data(ptrs); return code }
    i = 0 as libc::c_int;
    while !(*ptrs.offset(i as isize)).is_null() {
        let mut blob: *mut libc::c_void = 0 as *mut libc::c_void;
        vtbl =
            *ptrs.offset(i as isize) as *mut krb5plugin_service_locate_ftable;
        Tprintf(b"element %d is %p\n\x00" as *const u8 as *const libc::c_char,
                i, *ptrs.offset(i as isize));
        /* For now, don't keep the plugin data alive.  For long-lived
         * contexts, it may be desirable to change that later. */
        code =
            (*vtbl).init.expect("non-null function pointer")(ctx, &mut blob);
        if !(code != 0) {
            socktype =
                if transport as libc::c_uint ==
                       TCP as libc::c_int as libc::c_uint {
                    SOCK_STREAM as libc::c_int
                } else { SOCK_DGRAM as libc::c_int };
            code =
                (*vtbl).lookup.expect("non-null function pointer")(blob, svc,
                                                                   realmz,
                                                                   socktype,
                                                                   0 as
                                                                       libc::c_int,
                                                                   Some(module_callback
                                                                            as
                                                                            unsafe extern "C" fn(_:
                                                                                                     *mut libc::c_void,
                                                                                                 _:
                                                                                                     libc::c_int,
                                                                                                 _:
                                                                                                     *mut sockaddr)
                                                                                ->
                                                                                    libc::c_int),
                                                                   &mut cbdata
                                                                       as
                                                                       *mut module_callback_data
                                                                       as
                                                                       *mut libc::c_void);
            /* Also ask for TCP addresses if we got UDP addresses and want both. */
            if code == 0 as libc::c_int &&
                   transport as libc::c_uint ==
                       TCP_OR_UDP as libc::c_int as libc::c_uint {
                code =
                    (*vtbl).lookup.expect("non-null function pointer")(blob,
                                                                       svc,
                                                                       realmz,
                                                                       SOCK_STREAM
                                                                           as
                                                                           libc::c_int,
                                                                       0 as
                                                                           libc::c_int,
                                                                       Some(module_callback
                                                                                as
                                                                                unsafe extern "C" fn(_:
                                                                                                         *mut libc::c_void,
                                                                                                     _:
                                                                                                         libc::c_int,
                                                                                                     _:
                                                                                                         *mut sockaddr)
                                                                                    ->
                                                                                        libc::c_int),
                                                                       &mut cbdata
                                                                           as
                                                                           *mut module_callback_data
                                                                           as
                                                                           *mut libc::c_void);
                if code as libc::c_long == -(1765328135 as libc::c_long) {
                    code = 0 as libc::c_int
                }
            }
            (*vtbl).fini.expect("non-null function pointer")(blob);
            if code as libc::c_long == -(1765328135 as libc::c_long) {
                /* Module passes, keep going.  */
            /* XXX */
                Tprintf(b"plugin doesn\'t handle this realm (KRB5_PLUGIN_NO_HANDLE)\n\x00"
                            as *const u8 as *const libc::c_char);
            } else {
                if code != 0 as libc::c_int {
                    /* Module encountered an actual error.  */
                    Tprintf(b"plugin lookup routine returned error %d: %s\n\x00"
                                as *const u8 as *const libc::c_char, code,
                            error_message(code as errcode_t));
                    free(realmz as *mut libc::c_void);
                    krb5int_free_plugin_dir_data(ptrs);
                    return code
                }
                break ;
            }
        }
        i += 1
    }
    if (*ptrs.offset(i as isize)).is_null() {
        Tprintf(b"ran off end of plugin list\n\x00" as *const u8 as
                    *const libc::c_char);
        free(realmz as *mut libc::c_void);
        krb5int_free_plugin_dir_data(ptrs);
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    Tprintf(b"stopped with plugin #%d, res=%p\n\x00" as *const u8 as
                *const libc::c_char, i, res);
    /* Got something back, yippee.  */
    Tprintf(b"now have %lu addrs in list %p\n\x00" as *const u8 as
                *const libc::c_char, (*serverlist).nservers, serverlist);
    free(realmz as *mut libc::c_void);
    krb5int_free_plugin_dir_data(ptrs);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "481:1"]
unsafe extern "C" fn prof_locate_server(mut context: krb5_context,
                                        mut realm: *const krb5_data,
                                        mut serverlist: *mut serverlist,
                                        mut svc: locate_service_type,
                                        mut transport: k5_transport)
 -> krb5_error_code {
    let mut profname: *const libc::c_char = 0 as *const libc::c_char;
    let mut dflport: libc::c_int = 0 as libc::c_int;
    let mut serv: *mut servent = 0 as *mut servent;
    let mut current_block_10: u64;
    match svc as libc::c_uint {
        1 => {
            profname = b"kdc\x00" as *const u8 as *const libc::c_char;
            current_block_10 = 3094981545124401488;
            /* XXX */
        }
        2 => {
            profname = b"master_kdc\x00" as *const u8 as *const libc::c_char;
            current_block_10 = 3094981545124401488;
        }
        3 => {
            profname =
                b"admin_server\x00" as *const u8 as *const libc::c_char;
            dflport = 749 as libc::c_int;
            current_block_10 = 1054647088692577877;
        }
        4 => {
            profname =
                b"krb524_server\x00" as *const u8 as *const libc::c_char;
            serv =
                getservbyname(b"krb524\x00" as *const u8 as
                                  *const libc::c_char,
                              b"udp\x00" as *const u8 as *const libc::c_char);
            dflport =
                if !serv.is_null() {
                    (*serv).s_port
                } else { 4444 as libc::c_int };
            current_block_10 = 1054647088692577877;
        }
        5 => {
            profname =
                b"kpasswd_server\x00" as *const u8 as *const libc::c_char;
            dflport = 464 as libc::c_int;
            current_block_10 = 1054647088692577877;
        }
        _ => { return 16 as libc::c_int }
    }
    match current_block_10 {
        3094981545124401488 =>
        /* We used to use /etc/services for these, but enough systems have old,
         * crufty, wrong settings that this is probably better. */
        {
            dflport = 88 as libc::c_int
        }
        _ => { }
    }
    return locate_srv_conf_1(context, realm, profname, serverlist, transport,
                             dflport);
}
/*
 * Parse the initial part of the URI, first confirming the scheme name.  Get
 * the transport, flags (indicating master status), and host.  The host is
 * either an address or hostname with an optional port, or an HTTPS URL.
 * The format is krb5srv:flags:udp|tcp|kkdcp:host
 *
 * Return a NULL *host_out if there are any problems parsing the URI.
 */
#[c2rust::src_loc = "532:1"]
unsafe extern "C" fn parse_uri_fields(mut uri: *const libc::c_char,
                                      mut transport_out: *mut k5_transport,
                                      mut host_out: *mut *const libc::c_char,
                                      mut master_out: *mut libc::c_int) {
    let mut transport: k5_transport = TCP_OR_UDP;
    let mut master: libc::c_int = 0 as libc::c_int;
    *transport_out = TCP_OR_UDP;
    *host_out = 0 as *const libc::c_char;
    *master_out = -(1 as libc::c_int);
    /* Confirm the scheme name. */
    if strncasecmp(uri, b"krb5srv\x00" as *const u8 as *const libc::c_char,
                   7 as libc::c_int as libc::c_ulong) != 0 as libc::c_int {
        return
    }
    uri = uri.offset(7 as libc::c_int as isize);
    if *uri as libc::c_int != ':' as i32 { return }
    uri = uri.offset(1);
    if *uri as libc::c_int == '\u{0}' as i32 { return }
    /* Check the flags field for supported flags. */
    while *uri as libc::c_int != ':' as i32 &&
              *uri as libc::c_int != '\u{0}' as i32 {
        if *uri as libc::c_int == 'm' as i32 ||
               *uri as libc::c_int == 'M' as i32 {
            master = 1 as libc::c_int
        }
        uri = uri.offset(1)
    }
    if *uri as libc::c_int != ':' as i32 { return }
    /* Look for the transport type. */
    uri = uri.offset(1);
    if strncasecmp(uri, b"udp\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
        transport = UDP;
        uri = uri.offset(3 as libc::c_int as isize)
    } else if strncasecmp(uri, b"tcp\x00" as *const u8 as *const libc::c_char,
                          3 as libc::c_int as libc::c_ulong) ==
                  0 as libc::c_int {
        transport = TCP;
        uri = uri.offset(3 as libc::c_int as isize)
    } else if strncasecmp(uri,
                          b"kkdcp\x00" as *const u8 as *const libc::c_char,
                          5 as libc::c_int as libc::c_ulong) ==
                  0 as libc::c_int {
        /* Currently the only MS-KKDCP transport type is HTTPS. */
        transport = HTTPS;
        uri = uri.offset(5 as libc::c_int as isize)
    } else { return }
    if *uri as libc::c_int != ':' as i32 { return }
    /* The rest of the URI is the host (with optional port) or URI. */
    *host_out = uri.offset(1 as libc::c_int as isize);
    *transport_out = transport;
    *master_out = master;
}
/*
 * Collect a list of servers from DNS URI records, for the requested service
 * and transport type.  Problematic entries are skipped.
 */
#[c2rust::src_loc = "593:1"]
unsafe extern "C" fn locate_uri(mut context: krb5_context,
                                mut realm: *const krb5_data,
                                mut req_service: *const libc::c_char,
                                mut serverlist: *mut serverlist,
                                mut req_transport: k5_transport,
                                mut default_port: libc::c_int,
                                mut master_only: krb5_boolean)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut transport: k5_transport = TCP_OR_UDP;
    let mut host_trans: k5_transport = TCP_OR_UDP;
    let mut answers: *mut srv_dns_entry = 0 as *mut srv_dns_entry;
    let mut entry: *mut srv_dns_entry = 0 as *mut srv_dns_entry;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host_field: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut port: libc::c_int = 0;
    let mut def_port: libc::c_int = 0;
    let mut master: libc::c_int = 0;
    ret = k5_make_uri_query(context, realm, req_service, &mut answers);
    if ret != 0 || answers.is_null() { return ret }
    let mut current_block_13: u64;
    entry = answers;
    while !entry.is_null() {
        def_port = default_port;
        path = 0 as *const libc::c_char;
        parse_uri_fields((*entry).host, &mut transport, &mut host_field,
                         &mut master);
        if !host_field.is_null() {
            /* TCP_OR_UDP allows entries of any transport type; otherwise
         * we're asking for a match. */
            if !(req_transport as libc::c_uint !=
                     TCP_OR_UDP as libc::c_int as libc::c_uint &&
                     req_transport as libc::c_uint !=
                         transport as libc::c_uint) {
                /* Process a MS-KKDCP target. */
                if transport as libc::c_uint ==
                       HTTPS as libc::c_int as libc::c_uint {
                    host_trans = TCP_OR_UDP;
                    def_port = 443 as libc::c_int;
                    parse_uri_if_https(host_field, &mut host_trans,
                                       &mut host_field, &mut path);
                    if host_trans as libc::c_uint !=
                           HTTPS as libc::c_int as libc::c_uint {
                        current_block_13 = 4906268039856690917;
                    } else { current_block_13 = 8457315219000651999; }
                } else { current_block_13 = 8457315219000651999; }
                match current_block_13 {
                    4906268039856690917 => { }
                    _ => {
                        ret =
                            k5_parse_host_string(host_field, def_port,
                                                 &mut host, &mut port);
                        if ret == 12 as libc::c_int { break ; }
                        if ret != 0 || host.is_null() {
                            ret = 0 as libc::c_int
                        } else {
                            ret =
                                add_host_to_list(serverlist, host, port,
                                                 transport, 0 as libc::c_int,
                                                 path, master);
                            free(host as *mut libc::c_void);
                            if ret != 0 { break ; }
                        }
                    }
                }
            }
        }
        entry = (*entry).next
    }
    krb5int_free_srv_dns_data(answers);
    return ret;
}
#[c2rust::src_loc = "652:1"]
unsafe extern "C" fn dns_locate_server_uri(mut context: krb5_context,
                                           mut realm: *const krb5_data,
                                           mut serverlist: *mut serverlist,
                                           mut svc: locate_service_type,
                                           mut transport: k5_transport)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut svcname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut def_port: libc::c_int = 0;
    let mut find_master: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    if _krb5_use_dns_kdc(context) == 0 || use_dns_uri(context) == 0 {
        return 0 as libc::c_int
    }
    let mut current_block_9: u64;
    match svc as libc::c_uint {
        2 => {
            find_master = 1 as libc::c_int as krb5_boolean;
            current_block_9 = 1201219362439546566;
        }
        1 => { current_block_9 = 1201219362439546566; }
        3 => {
            svcname =
                b"_kerberos-adm\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            def_port = 749 as libc::c_int;
            current_block_9 = 12599329904712511516;
        }
        5 => {
            svcname =
                b"_kpasswd\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            def_port = 464 as libc::c_int;
            current_block_9 = 12599329904712511516;
        }
        _ => { return 0 as libc::c_int }
    }
    match current_block_9 {
        1201219362439546566 =>
        /* Fall through */
        {
            svcname =
                b"_kerberos\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            def_port = 88 as libc::c_int
        }
        _ => { }
    }
    ret =
        locate_uri(context, realm, svcname, serverlist, transport, def_port,
                   find_master);
    if (*serverlist).nservers == 0 as libc::c_int as libc::c_ulong {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"No URI records found\x00" as *const u8 as
                              *const libc::c_char);
        }
    }
    return ret;
}
#[c2rust::src_loc = "694:1"]
unsafe extern "C" fn dns_locate_server_srv(mut context: krb5_context,
                                           mut realm: *const krb5_data,
                                           mut serverlist: *mut serverlist,
                                           mut svc: locate_service_type,
                                           mut transport: k5_transport)
 -> krb5_error_code {
    let mut dnsname: *const libc::c_char = 0 as *const libc::c_char;
    let mut use_dns: libc::c_int = _krb5_use_dns_kdc(context);
    let mut code: krb5_error_code = 0;
    if use_dns == 0 { return 0 as libc::c_int }
    match svc as libc::c_uint {
        1 => {
            dnsname = b"_kerberos\x00" as *const u8 as *const libc::c_char
        }
        2 => {
            dnsname =
                b"_kerberos-master\x00" as *const u8 as *const libc::c_char
        }
        3 => {
            dnsname = b"_kerberos-adm\x00" as *const u8 as *const libc::c_char
        }
        4 => { dnsname = b"_krb524\x00" as *const u8 as *const libc::c_char }
        5 => { dnsname = b"_kpasswd\x00" as *const u8 as *const libc::c_char }
        _ => { return 0 as libc::c_int }
    }
    code = 0 as libc::c_int;
    if transport as libc::c_uint == UDP as libc::c_int as libc::c_uint ||
           transport as libc::c_uint ==
               TCP_OR_UDP as libc::c_int as libc::c_uint {
        code =
            locate_srv_dns_1(context, realm, dnsname,
                             b"_udp\x00" as *const u8 as *const libc::c_char,
                             serverlist)
    }
    if (transport as libc::c_uint == TCP as libc::c_int as libc::c_uint ||
            transport as libc::c_uint ==
                TCP_OR_UDP as libc::c_int as libc::c_uint) &&
           code == 0 as libc::c_int {
        code =
            locate_srv_dns_1(context, realm, dnsname,
                             b"_tcp\x00" as *const u8 as *const libc::c_char,
                             serverlist)
    }
    if (*serverlist).nservers == 0 as libc::c_int as libc::c_ulong {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"No SRV records found\x00" as *const u8 as
                              *const libc::c_char);
        }
    }
    return code;
}
/* KRB5_DNS_LOOKUP */
/*
 * Try all of the server location methods in sequence.  transport must be
 * TCP_OR_UDP, TCP, or UDP.  It is applied to hostname entries in the profile
 * and affects whether we query modules or DNS for UDP or TCP or both, but does
 * not restrict a method from returning entries of other transports.
 */
#[c2rust::src_loc = "746:1"]
unsafe extern "C" fn locate_server(mut context: krb5_context,
                                   mut realm: *const krb5_data,
                                   mut serverlist: *mut serverlist,
                                   mut svc: locate_service_type,
                                   mut transport: k5_transport)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut list: serverlist =
        {
            let mut init =
                serverlist{servers: 0 as *mut server_entry,
                           nservers: 0 as libc::c_int as size_t,};
            init
        };
    *serverlist = list;
    /* Try modules.  If a module returns 0 but leaves the list empty, return an
     * empty list. */
    ret = module_locate_server(context, realm, &mut list, svc, transport);
    if !(ret as libc::c_long != -(1765328135 as libc::c_long)) {
        /* Try the profile.  Fall back to DNS if it returns an empty list. */
        ret = prof_locate_server(context, realm, &mut list, svc, transport);
        if !(ret != 0) {
            if list.nservers == 0 as libc::c_int as libc::c_ulong {
                ret =
                    dns_locate_server_uri(context, realm, &mut list, svc,
                                          transport);
                if ret != 0 {
                    current_block = 16130812644014403787;
                } else { current_block = 13183875560443969876; }
            } else { current_block = 13183875560443969876; }
            match current_block {
                16130812644014403787 => { }
                _ => {
                    if list.nservers == 0 as libc::c_int as libc::c_ulong {
                        ret =
                            dns_locate_server_srv(context, realm, &mut list,
                                                  svc, transport)
                    }
                }
            }
        }
    }
    if ret != 0 { k5_free_serverlist(&mut list); return ret }
    *serverlist = list;
    return 0 as libc::c_int;
}
/*
 * Wrapper function for the various backends
 */
#[no_mangle]
#[c2rust::src_loc = "791:1"]
pub unsafe extern "C" fn k5_locate_server(mut context: krb5_context,
                                          mut realm: *const krb5_data,
                                          mut serverlist: *mut serverlist,
                                          mut svc: locate_service_type,
                                          mut no_udp: krb5_boolean)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut transport: k5_transport =
        if no_udp != 0 {
            TCP as libc::c_int
        } else { TCP_OR_UDP as libc::c_int } as k5_transport;
    memset(serverlist as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<serverlist>() as libc::c_ulong);
    if realm.is_null() || (*realm).data.is_null() ||
           *(*realm).data.offset(0 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int {
        krb5_set_error_message(context,
                               -(1765328164 as libc::c_long) as
                                   krb5_error_code,
                               b"Cannot find KDC for invalid realm name \"\"\x00"
                                   as *const u8 as *const libc::c_char);
        return -(1765328164 as libc::c_long) as krb5_error_code
    }
    ret = locate_server(context, realm, serverlist, svc, transport);
    if ret != 0 { return ret }
    if (*serverlist).nservers == 0 as libc::c_int as libc::c_ulong {
        k5_free_serverlist(serverlist);
        krb5_set_error_message(context,
                               -(1765328230 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Cannot find KDC for realm \"%.*s\"\x00"
                                            as *const u8 as
                                            *const libc::c_char),
                               (*realm).length, (*realm).data);
        return -(1765328230 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "820:1"]
pub unsafe extern "C" fn k5_locate_kdc(mut context: krb5_context,
                                       mut realm: *const krb5_data,
                                       mut serverlist: *mut serverlist,
                                       mut get_masters: krb5_boolean,
                                       mut no_udp: krb5_boolean)
 -> krb5_error_code {
    let mut stype: locate_service_type = 0 as locate_service_type;
    stype =
        if get_masters != 0 {
            locate_service_master_kdc as libc::c_int
        } else { locate_service_kdc as libc::c_int } as locate_service_type;
    return k5_locate_server(context, realm, serverlist, stype, no_udp);
}
#[no_mangle]
#[c2rust::src_loc = "831:1"]
pub unsafe extern "C" fn k5_kdc_is_master(mut context: krb5_context,
                                          mut realm: *const krb5_data,
                                          mut server: *mut server_entry)
 -> krb5_boolean {
    let mut list: serverlist =
        serverlist{servers: 0 as *mut server_entry, nservers: 0,};
    let mut found: krb5_boolean = 0;
    if (*server).master != -(1 as libc::c_int) {
        return (*server).master as krb5_boolean
    }
    if locate_server(context, realm, &mut list, locate_service_master_kdc,
                     (*server).transport) != 0 as libc::c_int {
        return 0 as libc::c_int as krb5_boolean
    }
    found = server_list_contains(&mut list, server);
    k5_free_serverlist(&mut list);
    return found;
}
