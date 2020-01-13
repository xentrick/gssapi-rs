use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:62"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:62"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:62"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:62"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:62"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:62"]
pub mod k5_thread_h {
    /* Thread-specific data; implemented in a support file, because we'll
   need to keep track of some global data for cleanup purposes.

   Note that the callback function type is such that the C library
   routine free() is a valid callback.  */
    #[c2rust::src_loc = "401:9"]
    pub type k5_key_t = libc::c_uint;
    #[c2rust::src_loc = "410:5"]
    pub const K5_KEY_MAX: k5_key_t = 5;
    #[c2rust::src_loc = "406:5"]
    pub const K5_KEY_GSS_SPNEGO_STATUS: k5_key_t = 4;
    #[c2rust::src_loc = "405:5"]
    pub const K5_KEY_GSS_KRB5_ERROR_MESSAGE: k5_key_t = 3;
    #[c2rust::src_loc = "404:5"]
    pub const K5_KEY_GSS_KRB5_CCACHE_NAME: k5_key_t = 2;
    #[c2rust::src_loc = "403:5"]
    pub const K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME: k5_key_t = 1;
    #[c2rust::src_loc = "402:5"]
    pub const K5_KEY_COM_ERR: k5_key_t = 0;
    extern "C" {
        /* rename shorthand symbols for export */
        #[no_mangle]
        #[c2rust::src_loc = "417:1"]
        pub fn krb5int_key_register(_: k5_key_t,
                                    _:
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut libc::c_void)
                                                   -> ()>) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn krb5int_getspecific(_: k5_key_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "419:1"]
        pub fn krb5int_setspecific(_: k5_key_t, _: *mut libc::c_void)
         -> libc::c_int;
    }
    /* multiple inclusion? */
    /* In time, many of the definitions above should move into the support
   library, and this file should be greatly simplified.  For type
   definitions, that'll take some work, since other data structures
   incorporate mutexes directly, and our mutex type is dependent on
   configuration options and system attributes.  For most functions,
   though, it should be relatively easy.

   For now, plugins should use the exported functions, and not the
   above macros, and use krb5int_mutex_alloc for allocations.  */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:62"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
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
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:62"]
pub mod k5_int_h {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:62"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:62"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:62"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:62"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:62"]
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
    /* K5_BUF_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:64"]
pub mod gssapi_h {
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set_desc = gss_OID_set_desc_struct;
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "122:16"]
    pub struct gss_channel_bindings_struct {
        pub initiator_addrtype: OM_uint32,
        pub initiator_address: gss_buffer_desc,
        pub acceptor_addrtype: OM_uint32,
        pub acceptor_address: gss_buffer_desc,
        pub application_data: gss_buffer_desc,
    }
    #[c2rust::src_loc = "122:1"]
    pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
    #[c2rust::src_loc = "134:1"]
    pub type gss_qop_t = OM_uint32;
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    /* mech_set */
    /* XXXX these are not part of the GSSAPI C bindings!  (but should be) */
    /* XXXX This is a necessary evil until the spec is fixed */
    /*
 * RFC 5587
 */
    #[c2rust::src_loc = "845:1"]
    pub type gss_const_buffer_t = *const gss_buffer_desc;
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
    #[c2rust::src_loc = "851:1"]
    pub type gss_const_OID_set = *const gss_OID_set_desc;
    use super::stdint_uintn_h::uint32_t;
    use super::mglueP_h::{gss_name_struct, gss_cred_id_struct};
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        /* cred_handle */
        #[no_mangle]
        #[c2rust::src_loc = "437:1"]
        pub fn gss_init_sec_context(_: *mut OM_uint32, _: gss_cred_id_t,
                                    _: *mut gss_ctx_id_t, _: gss_name_t,
                                    _: gss_OID, _: OM_uint32, _: OM_uint32,
                                    _: gss_channel_bindings_t,
                                    _: gss_buffer_t, _: *mut gss_OID,
                                    _: gss_buffer_t, _: *mut OM_uint32,
                                    _: *mut OM_uint32) -> OM_uint32;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn gss_accept_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_cred_id_t, _: gss_buffer_t,
                                      _: gss_channel_bindings_t,
                                      _: *mut gss_name_t, _: *mut gss_OID,
                                      _: gss_buffer_t, _: *mut OM_uint32,
                                      _: *mut OM_uint32,
                                      _: *mut gss_cred_id_t) -> OM_uint32;
        /* delegated_cred_handle */
        #[no_mangle]
        #[c2rust::src_loc = "467:1"]
        pub fn gss_process_context_token(_: *mut OM_uint32, _: gss_ctx_id_t,
                                         _: gss_buffer_t) -> OM_uint32;
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* output_token */
        #[no_mangle]
        #[c2rust::src_loc = "481:1"]
        pub fn gss_context_time(_: *mut OM_uint32, _: gss_ctx_id_t,
                                _: *mut OM_uint32) -> OM_uint32;
        /* time_rec */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "489:1"]
        pub fn gss_get_mic(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_qop_t,
                           _: gss_buffer_t, _: gss_buffer_t) -> OM_uint32;
        /* message_token */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn gss_verify_mic(_: *mut OM_uint32, _: gss_ctx_id_t,
                              _: gss_buffer_t, _: gss_buffer_t,
                              _: *mut gss_qop_t) -> OM_uint32;
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "508:1"]
        pub fn gss_wrap(_: *mut OM_uint32, _: gss_ctx_id_t, _: libc::c_int,
                        _: gss_qop_t, _: gss_buffer_t, _: *mut libc::c_int,
                        _: gss_buffer_t) -> OM_uint32;
        /* output_message_buffer */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "520:1"]
        pub fn gss_unwrap(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_buffer_t,
                          _: gss_buffer_t, _: *mut libc::c_int,
                          _: *mut gss_qop_t) -> OM_uint32;
        /* qop_state */
        #[no_mangle]
        #[c2rust::src_loc = "530:1"]
        pub fn gss_display_status(_: *mut OM_uint32, _: OM_uint32,
                                  _: libc::c_int, _: gss_OID,
                                  _: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* mech_set */
        #[no_mangle]
        #[c2rust::src_loc = "546:1"]
        pub fn gss_compare_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_name_t, _: *mut libc::c_int)
         -> OM_uint32;
        /* name_equal */
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
        /* output_name_type */
        #[no_mangle]
        #[c2rust::src_loc = "562:1"]
        pub fn gss_import_name(_: *mut OM_uint32, _: gss_buffer_t, _: gss_OID,
                               _: *mut gss_name_t) -> OM_uint32;
        /* output_name */
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* buffer */
        #[no_mangle]
        #[c2rust::src_loc = "579:1"]
        pub fn gss_release_oid_set(_: *mut OM_uint32, _: *mut gss_OID_set)
         -> OM_uint32;
        /* set */
        #[no_mangle]
        #[c2rust::src_loc = "584:1"]
        pub fn gss_inquire_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                                _: *mut gss_name_t, _: *mut OM_uint32,
                                _: *mut gss_cred_usage_t, _: *mut gss_OID_set)
         -> OM_uint32;
        /* mechanisms */
        /* Last argument new for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "594:1"]
        pub fn gss_inquire_context(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: *mut gss_name_t, _: *mut gss_name_t,
                                   _: *mut OM_uint32, _: *mut gss_OID,
                                   _: *mut OM_uint32, _: *mut libc::c_int,
                                   _: *mut libc::c_int) -> OM_uint32;
        /* open */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "607:1"]
        pub fn gss_wrap_size_limit(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: libc::c_int, _: gss_qop_t, _: OM_uint32,
                                   _: *mut OM_uint32) -> OM_uint32;
        /* acceptor_time_rec */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "648:1"]
        pub fn gss_inquire_cred_by_mech(_: *mut OM_uint32, _: gss_cred_id_t,
                                        _: gss_OID, _: *mut gss_name_t,
                                        _: *mut OM_uint32, _: *mut OM_uint32,
                                        _: *mut gss_cred_usage_t)
         -> OM_uint32;
        /* cred_usage */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "659:1"]
        pub fn gss_export_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* interprocess_token */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "666:1"]
        pub fn gss_import_sec_context(_: *mut OM_uint32, _: gss_buffer_t,
                                      _: *mut gss_ctx_id_t) -> OM_uint32;
        /* oid */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "679:1"]
        pub fn gss_create_empty_oid_set(_: *mut OM_uint32,
                                        _: *mut gss_OID_set) -> OM_uint32;
        /* oid_set */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "685:1"]
        pub fn gss_add_oid_set_member(_: *mut OM_uint32, _: gss_OID,
                                      _: *mut gss_OID_set) -> OM_uint32;
        /* oid_set */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "692:1"]
        pub fn gss_test_oid_set_member(_: *mut OM_uint32, _: gss_OID,
                                       _: gss_OID_set, _: *mut libc::c_int)
         -> OM_uint32;
        /* exported_name */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "777:1"]
        pub fn gss_duplicate_name(_: *mut OM_uint32, _: gss_name_t,
                                  _: *mut gss_name_t) -> OM_uint32;
        /* output_name */
        /* RFC 4401 */
        #[no_mangle]
        #[c2rust::src_loc = "796:1"]
        pub fn gss_pseudo_random(_: *mut OM_uint32, _: gss_ctx_id_t,
                                 _: libc::c_int, _: gss_buffer_t, _: ssize_t,
                                 _: gss_buffer_t) -> OM_uint32;
        /* cred_usage_stored */
        #[no_mangle]
        #[c2rust::src_loc = "816:1"]
        pub fn gss_set_neg_mechs(_: *mut OM_uint32, _: gss_cred_id_t,
                                 _: gss_OID_set) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "853:1"]
        pub fn gss_indicate_mechs_by_attrs(_: *mut OM_uint32,
                                           _: gss_const_OID_set,
                                           _: gss_const_OID_set,
                                           _: gss_const_OID_set,
                                           _: *mut gss_OID_set) -> OM_uint32;
        /* mechs */
        #[no_mangle]
        #[c2rust::src_loc = "861:1"]
        pub fn gss_inquire_attrs_for_mech(_: *mut OM_uint32, _: gss_const_OID,
                                          _: *mut gss_OID_set,
                                          _: *mut gss_OID_set) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "879:33"]
        pub static mut GSS_C_MA_MECH_NEGO: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "882:33"]
        pub static mut GSS_C_MA_DEPRECATED: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "883:33"]
        pub static mut GSS_C_MA_NOT_DFLT_MECH: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "884:33"]
        pub static mut GSS_C_MA_ITOK_FRAMED: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "392:27"]
        pub static mut GSS_C_NT_HOSTBASED_SERVICE: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "360:27"]
        pub static mut GSS_C_NT_STRING_UID_NAME: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "348:27"]
        pub static mut GSS_C_NT_MACHINE_UID_NAME: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "336:27"]
        pub static mut GSS_C_NT_USER_NAME: gss_OID;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/mechglue/mglueP.h:64"]
pub mod mglueP_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:16"]
    pub struct gss_name_struct {
        pub loopback: *mut gss_name_struct,
        pub name_type: gss_OID,
        pub external_name: gss_buffer_t,
        pub mech_type: gss_OID,
        pub mech_name: gss_name_t,
    }
    /* lib/gssapi/mechglue/mglueP.h */
    /*
 * Copyright (c) 1995, by Sun Microsystems, Inc.
 * All rights reserved.
 */
    /* This header contains the private mechglue definitions. */
    /*
 * Array of context IDs typed by mechanism OID
 */
    /*
 * Generic GSSAPI names.  A name can either be a generic name, or a
 * mechanism specific name....
 */
    /*
	 * These last two fields are only filled in for mechanism
	 * names.
	 */
    /*
 * Structure for holding list of mechanism-specific name types
 */
    /*
 * Set of Credentials typed on mechanism OID
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:16"]
    pub struct gss_cred_id_struct {
        pub loopback: *mut gss_cred_id_struct,
        pub count: libc::c_int,
        pub mechs_array: gss_OID,
        pub cred_array: *mut gss_cred_id_t,
    }
    /*
 * This is the definition of the mechs_array struct, which is used to
 * define the mechs array table. This table is used to indirectly
 * access mechanism specific versions of the gssapi routines through
 * the routines in the glue module (gssd_mech_glue.c)
 *
 * This contants all of the functions defined in gssapi.h except for
 * gss_release_buffer() and gss_release_oid_set(), which I am
 * assuming, for now, to be equal across mechanisms.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:16"]
    pub struct gss_config {
        pub mech_type: gss_OID_desc,
        pub context: *mut libc::c_void,
        pub gss_acquire_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: OM_uint32,
                                                          _: gss_OID_set,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut gss_cred_id_t,
                                                          _: *mut gss_OID_set,
                                                          _: *mut OM_uint32)
                                         -> OM_uint32>,
        pub gss_release_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _:
                                                              *mut gss_cred_id_t)
                                         -> OM_uint32>,
        pub gss_init_sec_context: Option<unsafe extern "C" fn(_:
                                                                  *mut OM_uint32,
                                                              _:
                                                                  gss_cred_id_t,
                                                              _:
                                                                  *mut gss_ctx_id_t,
                                                              _: gss_name_t,
                                                              _: gss_OID,
                                                              _: OM_uint32,
                                                              _: OM_uint32,
                                                              _:
                                                                  gss_channel_bindings_t,
                                                              _: gss_buffer_t,
                                                              _: *mut gss_OID,
                                                              _: gss_buffer_t,
                                                              _:
                                                                  *mut OM_uint32,
                                                              _:
                                                                  *mut OM_uint32)
                                             -> OM_uint32>,
        pub gss_accept_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _:
                                                                    gss_cred_id_t,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    gss_channel_bindings_t,
                                                                _:
                                                                    *mut gss_name_t,
                                                                _:
                                                                    *mut gss_OID,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_cred_id_t)
                                               -> OM_uint32>,
        pub gss_process_context_token: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_ctx_id_t,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32>,
        pub gss_delete_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gss_context_time: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_ctx_id_t,
                                                          _: *mut OM_uint32)
                                         -> OM_uint32>,
        pub gss_get_mic: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                     _: gss_ctx_id_t,
                                                     _: gss_qop_t,
                                                     _: gss_buffer_t,
                                                     _: gss_buffer_t)
                                    -> OM_uint32>,
        pub gss_verify_mic: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                        _: gss_ctx_id_t,
                                                        _: gss_buffer_t,
                                                        _: gss_buffer_t,
                                                        _: *mut gss_qop_t)
                                       -> OM_uint32>,
        pub gss_wrap: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                  _: gss_ctx_id_t,
                                                  _: libc::c_int,
                                                  _: gss_qop_t,
                                                  _: gss_buffer_t,
                                                  _: *mut libc::c_int,
                                                  _: gss_buffer_t)
                                 -> OM_uint32>,
        pub gss_unwrap: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                    _: gss_ctx_id_t,
                                                    _: gss_buffer_t,
                                                    _: gss_buffer_t,
                                                    _: *mut libc::c_int,
                                                    _: *mut gss_qop_t)
                                   -> OM_uint32>,
        pub gss_display_status: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: OM_uint32,
                                                            _: libc::c_int,
                                                            _: gss_OID,
                                                            _: *mut OM_uint32,
                                                            _: gss_buffer_t)
                                           -> OM_uint32>,
        pub gss_indicate_mechs: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _:
                                                                *mut gss_OID_set)
                                           -> OM_uint32>,
        pub gss_compare_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: gss_name_t,
                                                          _: *mut libc::c_int)
                                         -> OM_uint32>,
        pub gss_display_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: gss_buffer_t,
                                                          _: *mut gss_OID)
                                         -> OM_uint32>,
        pub gss_import_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_buffer_t,
                                                         _: gss_OID,
                                                         _: *mut gss_name_t)
                                        -> OM_uint32>,
        pub gss_release_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: *mut gss_name_t)
                                         -> OM_uint32>,
        pub gss_inquire_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_cred_id_t,
                                                          _: *mut gss_name_t,
                                                          _: *mut OM_uint32,
                                                          _: *mut libc::c_int,
                                                          _: *mut gss_OID_set)
                                         -> OM_uint32>,
        pub gss_add_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                      _: gss_cred_id_t,
                                                      _: gss_name_t,
                                                      _: gss_OID,
                                                      _: gss_cred_usage_t,
                                                      _: OM_uint32,
                                                      _: OM_uint32,
                                                      _: *mut gss_cred_id_t,
                                                      _: *mut gss_OID_set,
                                                      _: *mut OM_uint32,
                                                      _: *mut OM_uint32)
                                     -> OM_uint32>,
        pub gss_export_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gss_import_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut gss_ctx_id_t)
                                               -> OM_uint32>,
        pub gss_inquire_cred_by_mech: Option<unsafe extern "C" fn(_:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      gss_cred_id_t,
                                                                  _: gss_OID,
                                                                  _:
                                                                      *mut gss_name_t,
                                                                  _:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      *mut gss_cred_usage_t)
                                                 -> OM_uint32>,
        pub gss_inquire_names_for_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32>,
        pub gss_inquire_context: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _: *mut gss_OID,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32>,
        pub gss_internal_release_oid: Option<unsafe extern "C" fn(_:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      *mut gss_OID)
                                                 -> OM_uint32>,
        pub gss_wrap_size_limit: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _: OM_uint32,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32>,
        pub gss_localname: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                       _: gss_name_t,
                                                       _: gss_const_OID,
                                                       _: gss_buffer_t)
                                      -> OM_uint32>,
        pub gssspi_authorize_localname: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_const_buffer_t,
                                                                    _:
                                                                        gss_const_OID)
                                                   -> OM_uint32>,
        pub gss_export_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_name_t,
                                                         _: gss_buffer_t)
                                        -> OM_uint32>,
        pub gss_duplicate_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: gss_name_t,
                                                            _:
                                                                *mut gss_name_t)
                                           -> OM_uint32>,
        pub gss_store_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                        _: gss_cred_id_t,
                                                        _: gss_cred_usage_t,
                                                        _: gss_OID,
                                                        _: OM_uint32,
                                                        _: OM_uint32,
                                                        _: *mut gss_OID_set,
                                                        _:
                                                            *mut gss_cred_usage_t)
                                       -> OM_uint32>,
        pub gss_inquire_sec_context_by_oid: Option<unsafe extern "C" fn(_:
                                                                            *mut OM_uint32,
                                                                        _:
                                                                            gss_ctx_id_t,
                                                                        _:
                                                                            gss_OID,
                                                                        _:
                                                                            *mut gss_buffer_set_t)
                                                       -> OM_uint32>,
        pub gss_inquire_cred_by_oid: Option<unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_cred_id_t,
                                                                 _: gss_OID,
                                                                 _:
                                                                     *mut gss_buffer_set_t)
                                                -> OM_uint32>,
        pub gss_set_sec_context_option: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_ctx_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32>,
        pub gssspi_set_cred_option: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_cred_id_t,
                                                                _: gss_OID,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gssspi_mech_invoke: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: gss_OID,
                                                            _: gss_OID,
                                                            _: gss_buffer_t)
                                           -> OM_uint32>,
        pub gss_wrap_aead: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                       _: gss_ctx_id_t,
                                                       _: libc::c_int,
                                                       _: gss_qop_t,
                                                       _: gss_buffer_t,
                                                       _: gss_buffer_t,
                                                       _: *mut libc::c_int,
                                                       _: gss_buffer_t)
                                      -> OM_uint32>,
        pub gss_unwrap_aead: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_ctx_id_t,
                                                         _: gss_buffer_t,
                                                         _: gss_buffer_t,
                                                         _: gss_buffer_t,
                                                         _: *mut libc::c_int,
                                                         _: *mut gss_qop_t)
                                        -> OM_uint32>,
        pub gss_wrap_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                      _: gss_ctx_id_t,
                                                      _: libc::c_int,
                                                      _: gss_qop_t,
                                                      _: *mut libc::c_int,
                                                      _:
                                                          *mut gss_iov_buffer_desc,
                                                      _: libc::c_int)
                                     -> OM_uint32>,
        pub gss_unwrap_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                        _: gss_ctx_id_t,
                                                        _: *mut libc::c_int,
                                                        _: *mut gss_qop_t,
                                                        _:
                                                            *mut gss_iov_buffer_desc,
                                                        _: libc::c_int)
                                       -> OM_uint32>,
        pub gss_wrap_iov_length: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32>,
        pub gss_complete_auth_token: Option<unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_ctx_id_t,
                                                                 _:
                                                                     gss_buffer_t)
                                                -> OM_uint32>,
        pub gss_acquire_cred_impersonate_name: Option<unsafe extern "C" fn(_:
                                                                               *mut OM_uint32,
                                                                           _:
                                                                               gss_cred_id_t,
                                                                           _:
                                                                               gss_name_t,
                                                                           _:
                                                                               OM_uint32,
                                                                           _:
                                                                               gss_OID_set,
                                                                           _:
                                                                               gss_cred_usage_t,
                                                                           _:
                                                                               *mut gss_cred_id_t,
                                                                           _:
                                                                               *mut gss_OID_set,
                                                                           _:
                                                                               *mut OM_uint32)
                                                          -> OM_uint32>,
        pub gss_add_cred_impersonate_name: Option<unsafe extern "C" fn(_:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           gss_cred_id_t,
                                                                       _:
                                                                           gss_cred_id_t,
                                                                       _:
                                                                           gss_name_t,
                                                                       _:
                                                                           gss_OID,
                                                                       _:
                                                                           gss_cred_usage_t,
                                                                       _:
                                                                           OM_uint32,
                                                                       _:
                                                                           OM_uint32,
                                                                       _:
                                                                           *mut gss_cred_id_t,
                                                                       _:
                                                                           *mut gss_OID_set,
                                                                       _:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           *mut OM_uint32)
                                                      -> OM_uint32>,
        pub gss_display_name_ext: Option<unsafe extern "C" fn(_:
                                                                  *mut OM_uint32,
                                                              _: gss_name_t,
                                                              _: gss_OID,
                                                              _: gss_buffer_t)
                                             -> OM_uint32>,
        pub gss_inquire_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: *mut libc::c_int,
                                                          _: *mut gss_OID,
                                                          _:
                                                              *mut gss_buffer_set_t)
                                         -> OM_uint32>,
        pub gss_get_name_attribute: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _: gss_name_t,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> OM_uint32>,
        pub gss_set_name_attribute: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _: gss_name_t,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gss_delete_name_attribute: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_name_t,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32>,
        pub gss_export_name_composite: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_name_t,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32>,
        pub gss_map_name_to_any: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_any_t)
                                            -> OM_uint32>,
        pub gss_release_any_name_mapping: Option<unsafe extern "C" fn(_:
                                                                          *mut OM_uint32,
                                                                      _:
                                                                          gss_name_t,
                                                                      _:
                                                                          gss_buffer_t,
                                                                      _:
                                                                          *mut gss_any_t)
                                                     -> OM_uint32>,
        pub gss_pseudo_random: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                           _: gss_ctx_id_t,
                                                           _: libc::c_int,
                                                           _: gss_buffer_t,
                                                           _: ssize_t,
                                                           _: gss_buffer_t)
                                          -> OM_uint32>,
        pub gss_set_neg_mechs: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                           _: gss_cred_id_t,
                                                           _: gss_OID_set)
                                          -> OM_uint32>,
        pub gss_inquire_saslname_for_mech: Option<unsafe extern "C" fn(_:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           gss_OID,
                                                                       _:
                                                                           gss_buffer_t,
                                                                       _:
                                                                           gss_buffer_t,
                                                                       _:
                                                                           gss_buffer_t)
                                                      -> OM_uint32>,
        pub gss_inquire_mech_for_saslname: Option<unsafe extern "C" fn(_:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           gss_buffer_t,
                                                                       _:
                                                                           *mut gss_OID)
                                                      -> OM_uint32>,
        pub gss_inquire_attrs_for_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_const_OID,
                                                                    _:
                                                                        *mut gss_OID_set,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32>,
        pub gss_acquire_cred_from: Option<unsafe extern "C" fn(_:
                                                                   *mut OM_uint32,
                                                               _: gss_name_t,
                                                               _: OM_uint32,
                                                               _: gss_OID_set,
                                                               _:
                                                                   gss_cred_usage_t,
                                                               _:
                                                                   gss_const_key_value_set_t,
                                                               _:
                                                                   *mut gss_cred_id_t,
                                                               _:
                                                                   *mut gss_OID_set,
                                                               _:
                                                                   *mut OM_uint32)
                                              -> OM_uint32>,
        pub gss_store_cred_into: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _: gss_OID,
                                                             _: OM_uint32,
                                                             _: OM_uint32,
                                                             _:
                                                                 gss_const_key_value_set_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut gss_cred_usage_t)
                                            -> OM_uint32>,
        pub gssspi_acquire_cred_with_password: Option<unsafe extern "C" fn(_:
                                                                               *mut OM_uint32,
                                                                           _:
                                                                               gss_name_t,
                                                                           _:
                                                                               gss_buffer_t,
                                                                           _:
                                                                               OM_uint32,
                                                                           _:
                                                                               gss_OID_set,
                                                                           _:
                                                                               libc::c_int,
                                                                           _:
                                                                               *mut gss_cred_id_t,
                                                                           _:
                                                                               *mut gss_OID_set,
                                                                           _:
                                                                               *mut OM_uint32)
                                                          -> OM_uint32>,
        pub gss_export_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_cred_id_t,
                                                         _: gss_buffer_t)
                                        -> OM_uint32>,
        pub gss_import_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_buffer_t,
                                                         _:
                                                             *mut gss_cred_id_t)
                                        -> OM_uint32>,
        pub gssspi_import_sec_context_by_mech: Option<unsafe extern "C" fn(_:
                                                                               *mut OM_uint32,
                                                                           _:
                                                                               gss_OID,
                                                                           _:
                                                                               gss_buffer_t,
                                                                           _:
                                                                               *mut gss_ctx_id_t)
                                                          -> OM_uint32>,
        pub gssspi_import_name_by_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_name_t)
                                                   -> OM_uint32>,
        pub gssspi_import_cred_by_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_cred_id_t)
                                                   -> OM_uint32>,
        pub gss_get_mic_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_ctx_id_t,
                                                         _: gss_qop_t,
                                                         _:
                                                             *mut gss_iov_buffer_desc,
                                                         _: libc::c_int)
                                        -> OM_uint32>,
        pub gss_verify_mic_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: gss_ctx_id_t,
                                                            _: *mut gss_qop_t,
                                                            _:
                                                                *mut gss_iov_buffer_desc,
                                                            _: libc::c_int)
                                           -> OM_uint32>,
        pub gss_get_mic_iov_length: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    gss_ctx_id_t,
                                                                _: gss_qop_t,
                                                                _:
                                                                    *mut gss_iov_buffer_desc,
                                                                _:
                                                                    libc::c_int)
                                               -> OM_uint32>,
        pub gssspi_query_meta_data: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    gss_const_OID,
                                                                _:
                                                                    gss_cred_id_t,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _: gss_name_t,
                                                                _: OM_uint32,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gssspi_exchange_meta_data: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_const_OID,
                                                                   _:
                                                                       gss_cred_id_t,
                                                                   _:
                                                                       *mut gss_ctx_id_t,
                                                                   _:
                                                                       gss_name_t,
                                                                   _:
                                                                       OM_uint32,
                                                                   _:
                                                                       gss_const_buffer_t)
                                                  -> OM_uint32>,
        pub gssspi_query_mechanism_info: Option<unsafe extern "C" fn(_:
                                                                         *mut OM_uint32,
                                                                     _:
                                                                         gss_const_OID,
                                                                     _:
                                                                         *mut libc::c_uchar)
                                                    -> OM_uint32>,
    }
    #[c2rust::src_loc = "95:1"]
    pub type gss_mechanism = *mut gss_config;
    /*
 * In the user space we use a wrapper structure to encompass the
 * mechanism entry points.  The wrapper contain the mechanism
 * entry points and other data which is only relevant to the gss-api
 * layer.  In the kernel we use only the gss_config strucutre because
 * the kernal does not cantain any of the extra gss-api specific data.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "745:16"]
    pub struct gss_mech_config {
        pub kmodName: *mut libc::c_char,
        pub uLibName: *mut libc::c_char,
        pub mechNameStr: *mut libc::c_char,
        pub optionStr: *mut libc::c_char,
        pub dl_handle: *mut libc::c_void,
        pub mech_type: gss_OID,
        pub mech: gss_mechanism,
        pub priority: libc::c_int,
        pub freeMech: libc::c_int,
        pub is_interposer: libc::c_int,
        pub int_mech_type: gss_OID,
        pub int_mech: gss_mechanism,
        pub next: *mut gss_mech_config,
    }
    #[c2rust::src_loc = "745:1"]
    pub type gss_mech_info = *mut gss_mech_config;
    use super::gssapi_h::{gss_OID, gss_buffer_t, gss_name_t, gss_cred_id_t,
                          gss_OID_desc, OM_uint32, gss_OID_set, gss_ctx_id_t,
                          gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                          gss_const_OID, gss_const_buffer_t,
                          gss_OID_desc_struct, gss_buffer_desc_struct,
                          gss_OID_set_desc};
    use super::gssapi_ext_h::{gss_buffer_set_t, gss_iov_buffer_desc,
                              gss_any_t, gss_const_key_value_set_t};
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "769:1"]
        pub fn gssint_get_mechanism(_: gss_const_OID) -> gss_mechanism;
        #[no_mangle]
        #[c2rust::src_loc = "770:1"]
        pub fn gssint_get_mech_type(_: gss_OID, _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "783:1"]
        pub fn gssint_register_mechinfo(template: gss_mech_info)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "809:1"]
        pub fn gssint_copy_oid_set(_: *mut OM_uint32,
                                   _: *const gss_OID_set_desc,
                                   _: *mut gss_OID_set) -> OM_uint32;
        /* next element in the list */
        /*
 * Sun extensions to GSS-API v2
 */
        #[no_mangle]
        #[c2rust::src_loc = "827:1"]
        pub fn gssint_get_der_length(_: *mut *mut libc::c_uchar,
                                     _: libc::c_uint, _: *mut libc::c_uint)
         -> libc::c_int;
        /* der routines defined in libgss */
        #[no_mangle]
        #[c2rust::src_loc = "834:1"]
        pub fn gssint_der_length_size(_: libc::c_uint) -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "837:1"]
        pub fn gssint_put_der_length(_: libc::c_uint,
                                     _: *mut *mut libc::c_uchar,
                                     _: libc::c_uint) -> libc::c_int;
    }
    /* _GSS_MECHGLUEP_H */
    /* Use this to map an errno value or com_err error code being
   generated within the mechglue code (e.g., by calling generic oid
   ops).  Any errno or com_err values produced by mech operations
   should be processed with map_error.  This means they'll be stored
   separately even if the mech uses com_err, because we can't assume
   that it will use com_err.  */
    /* Use this to map an error code that was returned from a mech
   operation; the mech will be asked to produce the associated error
   messages.

   Remember that if the minor status code cannot be returned to the
   caller (e.g., if it's stuffed in an automatic variable and then
   ignored), then we don't care about producing a mapping.  */
    /* qop_state */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:64"]
pub mod gssapi_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:16"]
    pub struct gss_buffer_set_desc_struct {
        pub count: size_t,
        pub elements: *mut gss_buffer_desc,
    }
    /* acceptor_time_rec */
    /*
 * GGF extensions
 */
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "248:16"]
    pub struct gss_iov_buffer_desc_struct {
        pub type_0: OM_uint32,
        pub buffer: gss_buffer_desc,
    }
    #[c2rust::src_loc = "248:1"]
    pub type gss_iov_buffer_desc = gss_iov_buffer_desc_struct;
    #[c2rust::src_loc = "488:1"]
    pub type gss_any_t = *mut gss_any;
    /* Credential store extensions */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "530:8"]
    pub struct gss_key_value_element_struct {
        pub key: *const libc::c_char,
        pub value: *const libc::c_char,
    }
    #[c2rust::src_loc = "534:1"]
    pub type gss_key_value_element_desc = gss_key_value_element_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "536:8"]
    pub struct gss_key_value_set_struct {
        pub count: OM_uint32,
        pub elements: *mut gss_key_value_element_desc,
    }
    #[c2rust::src_loc = "540:1"]
    pub type gss_key_value_set_desc = gss_key_value_set_struct;
    #[c2rust::src_loc = "541:1"]
    pub type gss_const_key_value_set_t = *const gss_key_value_set_desc;
    use super::stddef_h::size_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32, gss_name_t,
                          gss_buffer_desc_struct, gss_buffer_t,
                          gss_OID_set_desc_struct, gss_OID_set,
                          gss_cred_usage_t, gss_cred_id_t, gss_ctx_id_struct,
                          gss_ctx_id_t, gss_OID_desc_struct, gss_OID,
                          gss_qop_t, gss_OID_desc, gss_const_OID};
    use super::mglueP_h::{gss_name_struct, gss_cred_id_struct};
    extern "C" {
        #[c2rust::src_loc = "488:16"]
        pub type gss_any;
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn gss_acquire_cred_with_password(_: *mut OM_uint32,
                                              _: gss_name_t, _: gss_buffer_t,
                                              _: OM_uint32, _: gss_OID_set,
                                              _: gss_cred_usage_t,
                                              _: *mut gss_cred_id_t,
                                              _: *mut gss_OID_set,
                                              _: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn gss_release_buffer_set(_: *mut OM_uint32,
                                      _: *mut gss_buffer_set_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "154:1"]
        pub fn gss_inquire_sec_context_by_oid(_: *mut OM_uint32,
                                              _: gss_ctx_id_t, _: gss_OID,
                                              _: *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "160:1"]
        pub fn gss_inquire_cred_by_oid(_: *mut OM_uint32, _: gss_cred_id_t,
                                       _: gss_OID, _: *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "166:1"]
        pub fn gss_set_sec_context_option(_: *mut OM_uint32,
                                          _: *mut gss_ctx_id_t, _: gss_OID,
                                          _: gss_buffer_t) -> OM_uint32;
        /*
 * Export import cred extensions from GGF, but using Heimdal's signatures
 */
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn gss_export_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                               _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "180:1"]
        pub fn gss_import_cred(_: *mut OM_uint32, _: gss_buffer_t,
                               _: *mut gss_cred_id_t) -> OM_uint32;
        /*
 * Heimdal extension
 */
        #[no_mangle]
        #[c2rust::src_loc = "188:1"]
        pub fn gss_set_cred_option(_: *mut OM_uint32, _: *mut gss_cred_id_t,
                                   _: gss_OID, _: gss_buffer_t) -> OM_uint32;
        /*
 * AEAD extensions
 */
        #[no_mangle]
        #[c2rust::src_loc = "207:1"]
        pub fn gss_wrap_aead(_: *mut OM_uint32, _: gss_ctx_id_t,
                             _: libc::c_int, _: gss_qop_t, _: gss_buffer_t,
                             _: gss_buffer_t, _: *mut libc::c_int,
                             _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn gss_unwrap_aead(_: *mut OM_uint32, _: gss_ctx_id_t,
                               _: gss_buffer_t, _: gss_buffer_t,
                               _: gss_buffer_t, _: *mut libc::c_int,
                               _: *mut gss_qop_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "243:1"]
        pub fn gss_complete_auth_token(minor_status: *mut OM_uint32,
                                       context_handle: gss_ctx_id_t,
                                       input_message_buffer: gss_buffer_t)
         -> OM_uint32;
        /* Packet data */
        /* Mechanism header */
        /* Mechanism specific parameters */
        /* Mechanism trailer */
        /* Padding */
        /* Complete wrap token */
        /* Sign only packet data */
        /* MIC token destination */
        /* indicates GSS should allocate */
        /* indicates caller should free */
        /*
 * Sign and optionally encrypt a sequence of buffers. The buffers
 * shall be ordered HEADER | DATA | PADDING | TRAILER. Suitable
 * space for the header, padding and trailer should be provided
 * by calling gss_wrap_iov_length(), or the ALLOCATE flag should
 * be set on those buffers.
 *
 * Encryption is in-place. SIGN_ONLY buffers are untouched. Only
 * a single PADDING buffer should be provided. The order of the
 * buffers in memory does not matter. Buffers in the IOV should
 * be arranged in the order above, and in the case of multiple
 * DATA buffers the sender and receiver should agree on the
 * order.
 *
 * With GSS_C_DCE_STYLE it is acceptable to not provide PADDING
 * and TRAILER, but the caller must guarantee the plaintext data
 * being encrypted is correctly padded, otherwise an error will
 * be returned.
 *
 * While applications that have knowledge of the underlying
 * cryptosystem may request a specific configuration of data
 * buffers, the only generally supported configurations are:
 *
 *  HEADER | DATA | PADDING | TRAILER
 *
 * which will emit GSS_Wrap() compatible tokens, and:
 *
 *  HEADER | SIGN_ONLY | DATA | PADDING | TRAILER
 *
 * for AEAD.
 *
 * The typical (special cased) usage for DCE is as follows:
 *
 *  SIGN_ONLY_1 | DATA | SIGN_ONLY_2 | HEADER
 */
        #[no_mangle]
        #[c2rust::src_loc = "307:1"]
        pub fn gss_wrap_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                            _: libc::c_int, _: gss_qop_t, _: *mut libc::c_int,
                            _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
        /* iov_count */
        /*
 * Verify and optionally decrypt a sequence of buffers. To process
 * a GSS-API message without separate buffer, pass STREAM | DATA.
 * Upon return DATA will contain the decrypted or integrity
 * protected message. Only a single DATA buffer may be provided
 * with this usage. DATA by default will point into STREAM, but if
 * the ALLOCATE flag is set a copy will be returned.
 *
 * Otherwise, decryption is in-place. SIGN_ONLY buffers are
 * untouched.
 */
        #[no_mangle]
        #[c2rust::src_loc = "328:1"]
        pub fn gss_unwrap_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                              _: *mut libc::c_int, _: *mut gss_qop_t,
                              _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
        /* iov_count */
        /*
 * Query HEADER, PADDING and TRAILER buffer lengths. DATA buffers
 * should be provided so the correct padding length can be determined.
 */
        #[no_mangle]
        #[c2rust::src_loc = "341:1"]
        pub fn gss_wrap_iov_length(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: libc::c_int, _: gss_qop_t,
                                   _: *mut libc::c_int,
                                   _: *mut gss_iov_buffer_desc,
                                   _: libc::c_int) -> OM_uint32;
        /* iov_count */
        /*
 * Produce a GSSAPI MIC token for a sequence of buffers.  All SIGN_ONLY and
 * DATA buffers will be signed, in the order they appear.  One MIC_TOKEN buffer
 * must be included for the result.  Suitable space should be provided for the
 * MIC_TOKEN buffer by calling gss_get_mic_iov_length, or the ALLOCATE flag
 * should be set on that buffer.  If the ALLOCATE flag is used, use
 * gss_release_iov_buffer to free the allocated buffer within the iov list when
 * it is no longer needed.
 */
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn gss_get_mic_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                               _: gss_qop_t, _: *mut gss_iov_buffer_desc,
                               _: libc::c_int) -> OM_uint32;
        /* iov_count */
        /*
 * Query the MIC_TOKEN buffer length within the iov list.
 */
        #[no_mangle]
        #[c2rust::src_loc = "371:1"]
        pub fn gss_get_mic_iov_length(_: *mut OM_uint32, _: gss_ctx_id_t,
                                      _: gss_qop_t,
                                      _: *mut gss_iov_buffer_desc,
                                      _: libc::c_int) -> OM_uint32;
        /* iov_count */
        /*
 * Verify the MIC_TOKEN buffer within the iov list against the SIGN_ONLY and
 * DATA buffers in the order they appear.  Return values are the same as for
 * gss_verify_mic.
 */
        #[no_mangle]
        #[c2rust::src_loc = "383:1"]
        pub fn gss_verify_mic_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                                  _: *mut gss_qop_t,
                                  _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
        /* iov_count */
        /*
 * Protocol transition
 */
        #[no_mangle]
        #[c2rust::src_loc = "403:1"]
        pub fn gss_acquire_cred_impersonate_name(_: *mut OM_uint32,
                                                 _: gss_cred_id_t,
                                                 _: gss_name_t, _: OM_uint32,
                                                 _: gss_OID_set,
                                                 _: gss_cred_usage_t,
                                                 _: *mut gss_cred_id_t,
                                                 _: *mut gss_OID_set,
                                                 _: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "436:1"]
        pub fn gss_display_name_ext(_: *mut OM_uint32, _: gss_name_t,
                                    _: gss_OID, _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "444:1"]
        pub fn gss_inquire_name(_: *mut OM_uint32, _: gss_name_t,
                                _: *mut libc::c_int, _: *mut gss_OID,
                                _: *mut gss_buffer_set_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn gss_get_name_attribute(_: *mut OM_uint32, _: gss_name_t,
                                      _: gss_buffer_t, _: *mut libc::c_int,
                                      _: *mut libc::c_int, _: gss_buffer_t,
                                      _: gss_buffer_t, _: *mut libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "465:1"]
        pub fn gss_set_name_attribute(_: *mut OM_uint32, _: gss_name_t,
                                      _: libc::c_int, _: gss_buffer_t,
                                      _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_name_attribute(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "481:1"]
        pub fn gss_export_name_composite(_: *mut OM_uint32, _: gss_name_t,
                                         _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "490:1"]
        pub fn gss_map_name_to_any(_: *mut OM_uint32, _: gss_name_t,
                                   _: libc::c_int, _: gss_buffer_t,
                                   _: *mut gss_any_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn gss_release_any_name_mapping(_: *mut OM_uint32, _: gss_name_t,
                                            _: gss_buffer_t,
                                            _: *mut gss_any_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn gss_oid_equal(_: gss_const_OID, _: gss_const_OID)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "545:1"]
        pub fn gss_acquire_cred_from(_: *mut OM_uint32, _: gss_name_t,
                                     _: OM_uint32, _: gss_OID_set,
                                     _: gss_cred_usage_t,
                                     _: gss_const_key_value_set_t,
                                     _: *mut gss_cred_id_t,
                                     _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "618:1"]
        pub fn gssspi_query_mechanism_info(minor_status: *mut OM_uint32,
                                           mech_oid: gss_const_OID,
                                           auth_scheme: *mut libc::c_uchar)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "624:33"]
        pub static mut GSS_C_MA_NEGOEX_AND_SPNEGO: gss_const_OID;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/spnego/gssapiP_spnego.h:65"]
pub mod gssapiP_spnego_h {
    /* Structure for context handle */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:8"]
    pub struct spnego_ctx_st {
        pub magic_num: OM_uint32,
        pub DER_mechTypes: gss_buffer_desc,
        pub mech_set: gss_OID_set,
        pub internal_mech: gss_OID,
        pub ctx_handle: gss_ctx_id_t,
        pub mic_reqd: libc::c_int,
        pub mic_sent: libc::c_int,
        pub mic_rcvd: libc::c_int,
        pub firstpass: libc::c_int,
        pub mech_complete: libc::c_int,
        pub nego_done: libc::c_int,
        pub initiate: libc::c_int,
        pub opened: libc::c_int,
        pub ctx_flags: OM_uint32,
        pub internal_name: gss_name_t,
        pub actual_mech: gss_OID,
        pub deleg_cred: gss_cred_id_t,
        pub negoex_step: libc::c_int,
        pub negoex_transcript: k5buf,
        pub negoex_seqnum: uint32_t,
        pub negoex_conv_id: conversation_id,
        pub negoex_mechs: negoex_mech_list,
        pub kctx: krb5_context,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "133:16"]
    pub struct negoex_mech_list {
        pub tqh_first: *mut negoex_auth_mech,
        pub tqh_last: *mut *mut negoex_auth_mech,
    }
    /*
 * Copyright 2003 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* #pragma ident	"@(#)gssapiP_spnego.h	1.3	03/09/18 SMI" */
    #[c2rust::src_loc = "15:1"]
    pub type spnego_gss_ctx_id_t = *mut spnego_ctx_st;
    #[c2rust::src_loc = "81:9"]
    pub type send_token_flag = libc::c_uint;
    #[c2rust::src_loc = "82:14"]
    pub const ERROR_TOKEN_SEND: send_token_flag = 4;
    #[c2rust::src_loc = "82:3"]
    pub const CHECK_MIC: send_token_flag = 3;
    #[c2rust::src_loc = "81:47"]
    pub const CONT_TOKEN_SEND: send_token_flag = 2;
    #[c2rust::src_loc = "81:30"]
    pub const INIT_TOKEN_SEND: send_token_flag = 1;
    #[c2rust::src_loc = "81:15"]
    pub const NO_TOKEN_SEND: send_token_flag = 0;
    #[c2rust::src_loc = "93:1"]
    pub type spnego_token_t = *mut libc::c_void;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "104:9"]
    pub struct C2RustUnnamed_0 {
        pub mcred: gss_cred_id_t,
        pub neg_mechs: gss_OID_set,
        pub no_ask_integ: libc::c_int,
    }
    /* Structure for credential */
    #[c2rust::src_loc = "104:1"]
    pub type spnego_gss_cred_id_t = *mut C2RustUnnamed_0;
    use super::gssapi_h::{OM_uint32, gss_buffer_desc, gss_OID_set, gss_OID,
                          gss_ctx_id_t, gss_name_t, gss_cred_id_t};
    use super::k5_buf_h::k5buf;
    use super::stdint_uintn_h::uint32_t;
    use super::gssapiP_negoex_h::{conversation_id, negoex_auth_mech};
    use super::krb5_h::krb5_context;
    /* mechglue union of obtainable creds */
    /* app-specified list of allowable mechs */
    /* do not request integ from mechs */
    /* _GSSAPIP_SPNEGO_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/spnego/gssapiP_negoex.h:65"]
pub mod gssapiP_negoex_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "115:8"]
    pub struct negoex_auth_mech {
        pub links: C2RustUnnamed,
        pub oid: gss_OID,
        pub scheme: auth_scheme,
        pub mech_context: gss_ctx_id_t,
        pub metadata: gss_buffer_desc,
        pub key: krb5_keyblock,
        pub verify_key: krb5_keyblock,
        pub complete: libc::c_int,
        pub sent_checksum: libc::c_int,
        pub verified_checksum: libc::c_int,
    }
    #[c2rust::src_loc = "56:1"]
    pub type auth_scheme = [uint8_t; 16];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:5"]
    pub struct C2RustUnnamed {
        pub tqe_next: *mut negoex_auth_mech,
        pub tqe_prev: *mut *mut negoex_auth_mech,
    }
    #[c2rust::src_loc = "57:1"]
    pub type conversation_id = [uint8_t; 16];
    use super::gssapi_h::{gss_OID, gss_ctx_id_t, gss_buffer_desc, OM_uint32,
                          gss_OID_desc, gss_const_OID, gss_cred_id_t,
                          gss_name_t, gss_buffer_desc_struct, gss_buffer_t};
    use super::krb5_h::krb5_keyblock;
    use super::stdint_uintn_h::uint8_t;
    use super::gssapiP_spnego_h::{spnego_ctx_st, spnego_gss_ctx_id_t};
    use super::mglueP_h::{gss_cred_id_struct, gss_name_struct};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "173:1"]
        pub fn negoex_release_context(ctx: spnego_gss_ctx_id_t);
        #[no_mangle]
        #[c2rust::src_loc = "176:1"]
        pub fn negoex_add_auth_mech(minor: *mut OM_uint32,
                                    ctx: spnego_gss_ctx_id_t,
                                    oid: gss_const_OID, scheme: *mut uint8_t)
         -> OM_uint32;
        /* negoex_ctx.c */
        #[no_mangle]
        #[c2rust::src_loc = "201:1"]
        pub fn negoex_init(minor: *mut OM_uint32, ctx: spnego_gss_ctx_id_t,
                           cred: gss_cred_id_t, target_name: gss_name_t,
                           req_flags: OM_uint32, time_req: OM_uint32,
                           input_token: gss_buffer_t,
                           output_token: gss_buffer_t,
                           time_rec: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "207:1"]
        pub fn negoex_accept(minor: *mut OM_uint32, ctx: spnego_gss_ctx_id_t,
                             cred: gss_cred_id_t, input_token: gss_buffer_t,
                             output_token: gss_buffer_t,
                             time_rec: *mut OM_uint32) -> OM_uint32;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:62"]
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
#[c2rust::header_src = "/usr/include/string.h:62"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:62"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:62"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:64"]
pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* To the extent possible under law, Painless Security, LLC has waived
 * all copyright and related or neighboring rights to GSS-API Memory
 * Management Header. This work is published from: United States.
 */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]
    #[c2rust::src_loc = "93:1"]
    pub unsafe extern "C" fn gssalloc_free(mut value: *mut libc::c_void) {
        free(value);
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn gssalloc_malloc(mut size: size_t)
     -> *mut libc::c_void {
        return malloc(size);
    }
    /* not _WIN32 or DEBUG_GSSALLOC */
    #[inline]
    #[c2rust::src_loc = "119:1"]
    pub unsafe extern "C" fn gssalloc_strdup(mut str: *const libc::c_char)
     -> *mut libc::c_char {
        let mut size: size_t =
            strlen(str).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut copy: *mut libc::c_char =
            gssalloc_malloc(size) as *mut libc::c_char;
        if !copy.is_null() {
            memcpy(copy as *mut libc::c_void, str as *const libc::c_void,
                   size);
            *copy.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                             as isize) = '\u{0}' as i32 as libc::c_char
        }
        return copy;
    }
    use super::stdlib_h::{free, malloc};
    use super::stddef_h::size_t;
    use super::string_h::{strlen, memcpy};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:64"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{gss_buffer_desc_struct, gss_buffer_t, OM_uint32,
                          gss_OID, gss_OID_desc, gss_OID_set_desc_struct,
                          gss_OID_set, gss_OID_set_desc};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "152:1"]
        pub fn gssint_g_make_string_buffer(str: *const libc::c_char,
                                           buffer: gss_buffer_t)
         -> libc::c_int;
        /* set */
        #[no_mangle]
        #[c2rust::src_loc = "201:1"]
        pub fn generic_gss_release_oid(_: *mut OM_uint32, _: *mut gss_OID)
         -> OM_uint32;
        /* set */
        #[no_mangle]
        #[c2rust::src_loc = "206:1"]
        pub fn generic_gss_copy_oid(_: *mut OM_uint32, _: *const gss_OID_desc,
                                    _: *mut gss_OID) -> OM_uint32;
        /* oid_set */
        #[no_mangle]
        #[c2rust::src_loc = "223:1"]
        pub fn generic_gss_test_oid_set_member(_: *mut OM_uint32,
                                               _: *const gss_OID_desc,
                                               _: gss_OID_set,
                                               _: *mut libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn gssint_mecherrmap_map(minor: OM_uint32,
                                     oid: *const gss_OID_desc) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "263:1"]
        pub fn gssint_mecherrmap_map_errcode(errcode: OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "316:1"]
        pub fn generic_gss_copy_oid_set(_: *mut OM_uint32,
                                        _: *const gss_OID_set_desc,
                                        _: *mut gss_OID_set) -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __ssize_t};
pub use self::sys_types_h::ssize_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::k5_thread_h::{k5_key_t, K5_KEY_MAX, K5_KEY_GSS_SPNEGO_STATUS,
                            K5_KEY_GSS_KRB5_ERROR_MESSAGE,
                            K5_KEY_GSS_KRB5_CCACHE_NAME,
                            K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME,
                            K5_KEY_COM_ERR, krb5int_key_register,
                            krb5int_getspecific, krb5int_setspecific};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_enctype,
                       krb5_flags, krb5_deltat, krb5_error_code, krb5_magic,
                       _krb5_data, krb5_data, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _profile_t};
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
pub use self::com_err_h::{errcode_t, error_message};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf};
pub use self::gssapi_h::{OM_uint32, gss_uint32, gss_name_t, gss_OID,
                         gss_OID_desc_struct, gss_buffer_t,
                         gss_buffer_desc_struct, gss_cred_id_t, gss_ctx_id_t,
                         gss_OID_desc, gss_OID_set_desc_struct,
                         gss_OID_set_desc, gss_OID_set, gss_buffer_desc,
                         gss_channel_bindings_struct, gss_channel_bindings_t,
                         gss_qop_t, gss_cred_usage_t, gss_const_buffer_t,
                         gss_const_OID, gss_const_OID_set, gss_ctx_id_struct,
                         gss_release_cred, gss_init_sec_context,
                         gss_accept_sec_context, gss_process_context_token,
                         gss_delete_sec_context, gss_context_time,
                         gss_get_mic, gss_verify_mic, gss_wrap, gss_unwrap,
                         gss_display_status, gss_compare_name,
                         gss_display_name, gss_import_name, gss_release_name,
                         gss_release_buffer, gss_release_oid_set,
                         gss_inquire_cred, gss_inquire_context,
                         gss_wrap_size_limit, gss_inquire_cred_by_mech,
                         gss_export_sec_context, gss_import_sec_context,
                         gss_create_empty_oid_set, gss_add_oid_set_member,
                         gss_test_oid_set_member, gss_duplicate_name,
                         gss_pseudo_random, gss_set_neg_mechs,
                         gss_indicate_mechs_by_attrs,
                         gss_inquire_attrs_for_mech, GSS_C_MA_MECH_NEGO,
                         GSS_C_MA_DEPRECATED, GSS_C_MA_NOT_DFLT_MECH,
                         GSS_C_MA_ITOK_FRAMED, GSS_C_NT_HOSTBASED_SERVICE,
                         GSS_C_NT_STRING_UID_NAME, GSS_C_NT_MACHINE_UID_NAME,
                         GSS_C_NT_USER_NAME};
pub use self::mglueP_h::{gss_name_struct, gss_cred_id_struct, gss_config,
                         gss_mechanism, gss_mech_config, gss_mech_info,
                         gssint_get_mechanism, gssint_get_mech_type,
                         gssint_register_mechinfo, gssint_copy_oid_set,
                         gssint_get_der_length, gssint_der_length_size,
                         gssint_put_der_length};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_t,
                             gss_iov_buffer_desc_struct, gss_iov_buffer_desc,
                             gss_any_t, gss_key_value_element_struct,
                             gss_key_value_element_desc,
                             gss_key_value_set_struct, gss_key_value_set_desc,
                             gss_const_key_value_set_t, gss_any,
                             gss_acquire_cred_with_password,
                             gss_release_buffer_set,
                             gss_inquire_sec_context_by_oid,
                             gss_inquire_cred_by_oid,
                             gss_set_sec_context_option, gss_export_cred,
                             gss_import_cred, gss_set_cred_option,
                             gss_wrap_aead, gss_unwrap_aead,
                             gss_complete_auth_token, gss_wrap_iov,
                             gss_unwrap_iov, gss_wrap_iov_length,
                             gss_get_mic_iov, gss_get_mic_iov_length,
                             gss_verify_mic_iov,
                             gss_acquire_cred_impersonate_name,
                             gss_display_name_ext, gss_inquire_name,
                             gss_get_name_attribute, gss_set_name_attribute,
                             gss_delete_name_attribute,
                             gss_export_name_composite, gss_map_name_to_any,
                             gss_release_any_name_mapping, gss_oid_equal,
                             gss_acquire_cred_from,
                             gssspi_query_mechanism_info,
                             GSS_C_MA_NEGOEX_AND_SPNEGO};
pub use self::gssapiP_spnego_h::{spnego_ctx_st, negoex_mech_list,
                                 spnego_gss_ctx_id_t, send_token_flag,
                                 ERROR_TOKEN_SEND, CHECK_MIC, CONT_TOKEN_SEND,
                                 INIT_TOKEN_SEND, NO_TOKEN_SEND,
                                 spnego_token_t, C2RustUnnamed_0,
                                 spnego_gss_cred_id_t};
pub use self::gssapiP_negoex_h::{negoex_auth_mech, auth_scheme, C2RustUnnamed,
                                 conversation_id, negoex_release_context,
                                 negoex_add_auth_mech, negoex_init,
                                 negoex_accept};
use self::assert_h::__assert_fail;
use self::string_h::{memcpy, memmove, memset, memcmp, strlen};
use self::stdlib_h::{malloc, calloc, free};
use self::libintl_h::dgettext;
pub use self::gssapi_alloc_h::{gssalloc_free, gssalloc_malloc,
                               gssalloc_strdup};
use self::gssapiP_generic_h::{gssint_g_make_string_buffer,
                              generic_gss_release_oid, generic_gss_copy_oid,
                              generic_gss_test_oid_set_member,
                              gssint_mecherrmap_map,
                              gssint_mecherrmap_map_errcode,
                              generic_gss_copy_oid_set};
/*
 * Copyright (C) 2006,2008 by the Massachusetts Institute of Technology.
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
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 *
 * A module that implements the spnego security mechanism.
 * It is used to negotiate the security mechanism between
 * peers using the GSS-API.  SPNEGO is specified in RFC 4178.
 *
 */
/*
 * Copyright (c) 2006-2008, Novell, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *   * The copyright holder's name is not used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
/* #pragma ident	"@(#)spnego_mech.c	1.7	04/09/28 SMI" */
#[c2rust::src_loc = "74:1"]
pub type gss_OID_const = *const gss_OID_desc;
/*  LEAN_CLIENT */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1817:8"]
pub struct C2RustUnnamed_1 {
    pub status: OM_uint32,
    pub msg: *const libc::c_char,
}
/* SPNEGO oid structure */
#[c2rust::src_loc = "178:27"]
static mut spnego_oids: [gss_OID_desc; 1] =
    [{
         let mut init =
             gss_OID_desc_struct{length: 6 as libc::c_int as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\x02\x00" as *const u8
                                         as *const libc::c_char as
                                         *mut libc::c_void,};
         init
     }];
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "182:28"]
pub static mut gss_mech_spnego: *const gss_OID_desc =
    0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[c2rust::src_loc = "183:31"]
static mut spnego_oidsets: [gss_OID_set_desc; 1] =
    [gss_OID_set_desc{count: 0, elements: 0 as *mut gss_OID_desc_struct,}; 1];
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "186:32"]
pub static mut gss_mech_set_spnego: *const gss_OID_set_desc =
    0 as *const gss_OID_set_desc;
#[c2rust::src_loc = "188:21"]
static mut negoex_mech: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 10 as libc::c_int as OM_uint32,
                                elements:
                                    b"+\x06\x01\x04\x01\x827\x02\x02\x1e\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
/*
 * The Mech OID for SPNEGO:
 * { iso(1) org(3) dod(6) internet(1) security(5)
 *  mechanism(5) spnego(2) }
 */
#[c2rust::src_loc = "201:26"]
static mut spnego_mechanism: gss_config =
    unsafe {
        {
            let mut init =
                gss_config{mech_type:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               6 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"+\x06\x01\x05\x05\x02\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },
                           context:
                               0 as *const libc::c_void as *mut libc::c_void,
                           gss_acquire_cred:
                               Some(spnego_gss_acquire_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: OM_uint32,
                                                             _: gss_OID_set,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_release_cred:
                               Some(spnego_gss_release_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_id_t)
                                            -> OM_uint32),
                           gss_init_sec_context:
                               Some(spnego_gss_init_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_name_t,
                                                             _: gss_OID,
                                                             _: OM_uint32,
                                                             _: OM_uint32,
                                                             _:
                                                                 gss_channel_bindings_t,
                                                             _: gss_buffer_t,
                                                             _: *mut gss_OID,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_accept_sec_context:
                               Some(spnego_gss_accept_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_cred_id_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 gss_channel_bindings_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _: *mut gss_OID,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_id_t)
                                            -> OM_uint32),
                           gss_process_context_token: None,
                           gss_delete_sec_context:
                               Some(spnego_gss_delete_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_context_time:
                               Some(spnego_gss_context_time as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_get_mic:
                               Some(spnego_gss_get_mic as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_qop_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_verify_mic:
                               Some(spnego_gss_verify_mic as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_qop_t)
                                            -> OM_uint32),
                           gss_wrap:
                               Some(spnego_gss_wrap as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_unwrap:
                               Some(spnego_gss_unwrap as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_qop_t)
                                            -> OM_uint32),
                           gss_display_status:
                               Some(spnego_gss_display_status as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: OM_uint32,
                                                             _: libc::c_int,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_indicate_mechs: None,
                           gss_compare_name:
                               Some(spnego_gss_compare_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_name_t,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32),
                           gss_display_name:
                               Some(spnego_gss_display_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _: *mut gss_OID)
                                            -> OM_uint32),
                           gss_import_name:
                               Some(spnego_gss_import_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_name_t)
                                            -> OM_uint32),
                           gss_release_name:
                               Some(spnego_gss_release_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_name_t)
                                            -> OM_uint32),
                           gss_inquire_cred:
                               Some(spnego_gss_inquire_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_OID_set)
                                            -> OM_uint32),
                           gss_add_cred: None,
                           gss_export_sec_context:
                               Some(spnego_gss_export_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_import_sec_context:
                               Some(spnego_gss_import_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_ctx_id_t)
                                            -> OM_uint32),
                           gss_inquire_cred_by_mech: None,
                           gss_inquire_names_for_mech:
                               Some(spnego_gss_inquire_names_for_mech as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_OID_set)
                                            -> OM_uint32),
                           gss_inquire_context:
                               Some(spnego_gss_inquire_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _: *mut gss_OID,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32),
                           gss_internal_release_oid: None,
                           gss_wrap_size_limit:
                               Some(spnego_gss_wrap_size_limit as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _: OM_uint32,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_localname: None,
                           gssspi_authorize_localname: None,
                           gss_export_name: None,
                           gss_duplicate_name:
                               Some(spnego_gss_duplicate_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _:
                                                                 *mut gss_name_t)
                                            -> OM_uint32),
                           gss_store_cred: None,
                           gss_inquire_sec_context_by_oid:
                               Some(spnego_gss_inquire_sec_context_by_oid as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_buffer_set_t)
                                            -> OM_uint32),
                           gss_inquire_cred_by_oid:
                               Some(spnego_gss_inquire_cred_by_oid as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_buffer_set_t)
                                            -> OM_uint32),
                           gss_set_sec_context_option:
                               Some(spnego_gss_set_sec_context_option as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_OID,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gssspi_set_cred_option:
                               Some(spnego_gss_set_cred_option as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _: gss_OID,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gssspi_mech_invoke: None,
                           gss_wrap_aead:
                               Some(spnego_gss_wrap_aead as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_unwrap_aead:
                               Some(spnego_gss_unwrap_aead as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_qop_t)
                                            -> OM_uint32),
                           gss_wrap_iov:
                               Some(spnego_gss_wrap_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_unwrap_iov:
                               Some(spnego_gss_unwrap_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_wrap_iov_length:
                               Some(spnego_gss_wrap_iov_length as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_complete_auth_token:
                               Some(spnego_gss_complete_auth_token as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_acquire_cred_impersonate_name:
                               Some(spnego_gss_acquire_cred_impersonate_name
                                        as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _: gss_name_t,
                                                             _: OM_uint32,
                                                             _: gss_OID_set,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_add_cred_impersonate_name: None,
                           gss_display_name_ext:
                               Some(spnego_gss_display_name_ext as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_OID,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_inquire_name:
                               Some(spnego_gss_inquire_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: *mut gss_OID,
                                                             _:
                                                                 *mut gss_buffer_set_t)
                                            -> OM_uint32),
                           gss_get_name_attribute:
                               Some(spnego_gss_get_name_attribute as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32),
                           gss_set_name_attribute:
                               Some(spnego_gss_set_name_attribute as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_delete_name_attribute:
                               Some(spnego_gss_delete_name_attribute as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_export_name_composite:
                               Some(spnego_gss_export_name_composite as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_map_name_to_any:
                               Some(spnego_gss_map_name_to_any as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_any_t)
                                            -> OM_uint32),
                           gss_release_any_name_mapping:
                               Some(spnego_gss_release_any_name_mapping as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_any_t)
                                            -> OM_uint32),
                           gss_pseudo_random:
                               Some(spnego_gss_pseudo_random as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _: ssize_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_set_neg_mechs:
                               Some(spnego_gss_set_neg_mechs as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _: gss_OID_set)
                                            -> OM_uint32),
                           gss_inquire_saslname_for_mech:
                               Some(spnego_gss_inquire_saslname_for_mech as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_OID,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_inquire_mech_for_saslname:
                               Some(spnego_gss_inquire_mech_for_saslname as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _: *mut gss_OID)
                                            -> OM_uint32),
                           gss_inquire_attrs_for_mech:
                               Some(spnego_gss_inquire_attrs_for_mech as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_const_OID,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut gss_OID_set)
                                            -> OM_uint32),
                           gss_acquire_cred_from:
                               Some(spnego_gss_acquire_cred_from as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: OM_uint32,
                                                             _: gss_OID_set,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _:
                                                                 gss_const_key_value_set_t,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_store_cred_into: None,
                           gssspi_acquire_cred_with_password:
                               Some(spnego_gss_acquire_cred_with_password as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _: OM_uint32,
                                                             _: gss_OID_set,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_export_cred:
                               Some(spnego_gss_export_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_import_cred:
                               Some(spnego_gss_import_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_cred_id_t)
                                            -> OM_uint32),
                           gssspi_import_sec_context_by_mech: None,
                           gssspi_import_name_by_mech: None,
                           gssspi_import_cred_by_mech: None,
                           gss_get_mic_iov:
                               Some(spnego_gss_get_mic_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_verify_mic_iov:
                               Some(spnego_gss_verify_mic_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_get_mic_iov_length:
                               Some(spnego_gss_get_mic_iov_length as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gssspi_query_meta_data: None,
                           gssspi_exchange_meta_data: None,
                           gssspi_query_mechanism_info: None,};
            init
        }
    };
#[c2rust::src_loc = "287:1"]
unsafe extern "C" fn gss_spnegomechglue_init() -> libc::c_int {
    let mut mech_spnego: gss_mech_config =
        gss_mech_config{kmodName: 0 as *mut libc::c_char,
                        uLibName: 0 as *mut libc::c_char,
                        mechNameStr: 0 as *mut libc::c_char,
                        optionStr: 0 as *mut libc::c_char,
                        dl_handle: 0 as *mut libc::c_void,
                        mech_type: 0 as *mut gss_OID_desc_struct,
                        mech: 0 as *mut gss_config,
                        priority: 0,
                        freeMech: 0,
                        is_interposer: 0,
                        int_mech_type: 0 as *mut gss_OID_desc_struct,
                        int_mech: 0 as *mut gss_config,
                        next: 0 as *mut gss_mech_config,};
    memset(&mut mech_spnego as *mut gss_mech_config as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<gss_mech_config>() as libc::c_ulong);
    mech_spnego.mech = &mut spnego_mechanism;
    mech_spnego.mechNameStr =
        b"spnego\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    mech_spnego.mech_type = 0 as gss_OID;
    return gssint_register_mechinfo(&mut mech_spnego);
}
/* _GSS_STATIC_LINK */
#[no_mangle]
#[c2rust::src_loc = "310:1"]
pub unsafe extern "C" fn gss_spnegoint_lib_init() -> libc::c_int {
    let mut err: libc::c_int = 0;
    err = krb5int_key_register(K5_KEY_GSS_SPNEGO_STATUS, None);
    if err != 0 { return err }
    return gss_spnegomechglue_init();
}
#[no_mangle]
#[c2rust::src_loc = "325:1"]
pub unsafe extern "C" fn gss_spnegoint_lib_fini() { }
#[c2rust::src_loc = "329:1"]
unsafe extern "C" fn create_spnego_cred(mut minor_status: *mut OM_uint32,
                                        mut mcred: gss_cred_id_t,
                                        mut cred_out:
                                            *mut spnego_gss_cred_id_t)
 -> OM_uint32 {
    let mut spcred: spnego_gss_cred_id_t = 0 as *mut C2RustUnnamed_0;
    *cred_out = 0 as spnego_gss_cred_id_t;
    spcred =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong) as
            spnego_gss_cred_id_t;
    if spcred.is_null() {
        *minor_status = 12 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    (*spcred).mcred = mcred;
    *cred_out = spcred;
    return 0 as libc::c_int as OM_uint32;
}
/* SPNEGO oid declarations */
/* DEBUG */
/*
 * declarations of internal name mechanism functions
 */
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "347:1"]
pub unsafe extern "C" fn spnego_gss_acquire_cred(mut minor_status:
                                                     *mut OM_uint32,
                                                 mut desired_name: gss_name_t,
                                                 mut time_req: OM_uint32,
                                                 mut desired_mechs:
                                                     gss_OID_set,
                                                 mut cred_usage:
                                                     gss_cred_usage_t,
                                                 mut output_cred_handle:
                                                     *mut gss_cred_id_t,
                                                 mut actual_mechs:
                                                     *mut gss_OID_set,
                                                 mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    return spnego_gss_acquire_cred_from(minor_status, desired_name, time_req,
                                        desired_mechs, cred_usage,
                                        0 as gss_const_key_value_set_t,
                                        output_cred_handle, actual_mechs,
                                        time_rec);
}
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "364:1"]
pub unsafe extern "C" fn spnego_gss_acquire_cred_from(mut minor_status:
                                                          *mut OM_uint32,
                                                      desired_name:
                                                          gss_name_t,
                                                      mut time_req: OM_uint32,
                                                      desired_mechs:
                                                          gss_OID_set,
                                                      mut cred_usage:
                                                          gss_cred_usage_t,
                                                      mut cred_store:
                                                          gss_const_key_value_set_t,
                                                      mut output_cred_handle:
                                                          *mut gss_cred_id_t,
                                                      mut actual_mechs:
                                                          *mut gss_OID_set,
                                                      mut time_rec:
                                                          *mut OM_uint32)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut amechs: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut mcred: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut spcred: spnego_gss_cred_id_t = 0 as spnego_gss_cred_id_t;
    if !actual_mechs.is_null() { *actual_mechs = 0 as gss_OID_set }
    if !time_rec.is_null() { *time_rec = 0 as libc::c_int as OM_uint32 }
    /* We will obtain a mechglue credential and wrap it in a
	 * spnego_gss_cred_id_rec structure.  Allocate the wrapper. */
    status = create_spnego_cred(minor_status, mcred, &mut spcred);
    if status != 0 as libc::c_int as libc::c_uint { return status }
    /*
	 * Always use get_available_mechs to collect a list of
	 * mechs for which creds are available.
	 */
    status =
        get_available_mechs(minor_status, desired_name, cred_usage,
                            cred_store, &mut mcred, &mut amechs, time_rec);
    if !actual_mechs.is_null() && !amechs.is_null() {
        gssint_copy_oid_set(&mut tmpmin, amechs as *const gss_OID_set_desc,
                            actual_mechs);
    }
    gss_release_oid_set(&mut tmpmin, &mut amechs);
    if status == 0 as libc::c_int as libc::c_uint {
        (*spcred).mcred = mcred;
        *output_cred_handle = spcred as gss_cred_id_t
    } else {
        free(spcred as *mut libc::c_void);
        *output_cred_handle = 0 as gss_cred_id_t
    }
    return status;
}
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "419:1"]
pub unsafe extern "C" fn spnego_gss_release_cred(mut minor_status:
                                                     *mut OM_uint32,
                                                 mut cred_handle:
                                                     *mut gss_cred_id_t)
 -> OM_uint32 {
    let mut spcred: spnego_gss_cred_id_t = 0 as spnego_gss_cred_id_t;
    if minor_status.is_null() || cred_handle.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    if (*cred_handle).is_null() { return 0 as libc::c_int as OM_uint32 }
    spcred = *cred_handle as spnego_gss_cred_id_t;
    *cred_handle = 0 as gss_cred_id_t;
    gss_release_oid_set(minor_status, &mut (*spcred).neg_mechs);
    gss_release_cred(minor_status, &mut (*spcred).mcred);
    free(spcred as *mut libc::c_void);
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "445:1"]
unsafe extern "C" fn create_spnego_ctx(mut initiate: libc::c_int)
 -> spnego_gss_ctx_id_t {
    let mut spnego_ctx: spnego_gss_ctx_id_t = 0 as spnego_gss_ctx_id_t;
    spnego_ctx =
        malloc(::std::mem::size_of::<spnego_ctx_st>() as libc::c_ulong) as
            spnego_gss_ctx_id_t;
    if spnego_ctx.is_null() { return 0 as spnego_gss_ctx_id_t }
    (*spnego_ctx).magic_num = 0xfed as libc::c_int as OM_uint32;
    (*spnego_ctx).ctx_handle = 0 as gss_ctx_id_t;
    (*spnego_ctx).mech_set = 0 as gss_OID_set;
    (*spnego_ctx).internal_mech = 0 as gss_OID;
    (*spnego_ctx).DER_mechTypes.length = 0 as libc::c_int as size_t;
    (*spnego_ctx).DER_mechTypes.value = 0 as *mut libc::c_void;
    (*spnego_ctx).mic_reqd = 0 as libc::c_int;
    (*spnego_ctx).mic_sent = 0 as libc::c_int;
    (*spnego_ctx).mic_rcvd = 0 as libc::c_int;
    (*spnego_ctx).mech_complete = 0 as libc::c_int;
    (*spnego_ctx).nego_done = 0 as libc::c_int;
    (*spnego_ctx).opened = 0 as libc::c_int;
    (*spnego_ctx).initiate = initiate;
    (*spnego_ctx).internal_name = 0 as gss_name_t;
    (*spnego_ctx).actual_mech = 0 as gss_OID;
    (*spnego_ctx).deleg_cred = 0 as gss_cred_id_t;
    (*spnego_ctx).negoex_step = 0 as libc::c_int;
    memset(&mut (*spnego_ctx).negoex_transcript as *mut k5buf as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<k5buf>() as libc::c_ulong);
    (*spnego_ctx).negoex_seqnum = 0 as libc::c_int as uint32_t;
    (*spnego_ctx).negoex_mechs.tqh_first = 0 as *mut negoex_auth_mech;
    (*spnego_ctx).negoex_mechs.tqh_last =
        &mut (*spnego_ctx).negoex_mechs.tqh_first;
    (*spnego_ctx).kctx = 0 as krb5_context;
    memset((*spnego_ctx).negoex_conv_id.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int, 16 as libc::c_int as libc::c_ulong);
    return spnego_ctx;
}
/* iso(1) org(3) dod(6) internet(1) private(4) enterprises(1) samba(7165)
 * gssntlmssp(655) controls(1) spnego_req_mechlistMIC(2) */
#[c2rust::src_loc = "483:27"]
static mut spnego_req_mechlistMIC_oid: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                elements:
                                    b"+\x06\x01\x04\x01\xb7}\x85\x0f\x01\x02\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
/*
 * Return nonzero if the mechanism has reason to believe that a mechlistMIC
 * exchange will be required.  Microsoft servers erroneously require SPNEGO
 * mechlistMIC if they see an internal MIC within an NTLMSSP Authenticate
 * message, even if NTLMSSP was the preferred mechanism.
 */
#[c2rust::src_loc = "492:1"]
unsafe extern "C" fn mech_requires_mechlistMIC(mut sc: spnego_gss_ctx_id_t)
 -> libc::c_int {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut ctx: gss_ctx_id_t = (*sc).ctx_handle;
    let mut oid: gss_OID =
        &spnego_req_mechlistMIC_oid as *const gss_OID_desc as gss_OID;
    let mut bufs: gss_buffer_set_t = 0 as *mut gss_buffer_set_desc_struct;
    let mut result: libc::c_int = 0;
    major = gss_inquire_sec_context_by_oid(&mut minor, ctx, oid, &mut bufs);
    if major != 0 as libc::c_int as libc::c_uint { return 0 as libc::c_int }
    /* Report true if the mech returns a single buffer containing a single
	 * byte with value 1. */
    result =
        (!bufs.is_null() && (*bufs).count == 1 as libc::c_int as libc::c_ulong
             &&
             (*(*bufs).elements.offset(0 as libc::c_int as isize)).length ==
                 1 as libc::c_int as libc::c_ulong &&
             memcmp((*(*bufs).elements.offset(0 as libc::c_int as
                                                  isize)).value,
                    b"\x01\x00" as *const u8 as *const libc::c_char as
                        *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong) == 0 as libc::c_int) as
            libc::c_int;
    gss_release_buffer_set(&mut minor, &mut bufs);
    return result;
}
/* iso(1) org(3) dod(6) internet(1) private(4) enterprises(1) Microsoft(311)
 * security(2) mechanisms(2) NTLM(10) */
#[c2rust::src_loc = "516:27"]
static mut gss_mech_ntlmssp_oid: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 10 as libc::c_int as OM_uint32,
                                elements:
                                    b"+\x06\x01\x04\x01\x827\x02\x02\n\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
/* iso(1) org(3) dod(6) internet(1) private(4) enterprises(1) samba(7165)
 * gssntlmssp(655) controls(1) ntlmssp_reset_crypto(3) */
#[c2rust::src_loc = "521:27"]
static mut ntlmssp_reset_crypto_oid: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                elements:
                                    b"+\x06\x01\x04\x01\xb7}\x85\x0f\x01\x03\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
/*
 * MS-SPNG section 3.3.5.1 warns that the NTLM mechanism requires special
 * handling of the crypto state to interop with Windows.  If the mechanism for
 * sc is SPNEGO, invoke a mechanism-specific operation on the context to reset
 * the RC4 state after producing or verifying a MIC.  Ignore a result of
 * GSS_S_UNAVAILABLE for compatibility with older versions of the mechanism
 * that do not support this functionality.
 */
#[c2rust::src_loc = "532:1"]
unsafe extern "C" fn ntlmssp_reset_crypto_state(mut minor_status:
                                                    *mut OM_uint32,
                                                mut sc: spnego_gss_ctx_id_t,
                                                mut verify: OM_uint32)
 -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut value: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    if !((*(*sc).internal_mech).length == gss_mech_ntlmssp_oid.length &&
             memcmp((*(*sc).internal_mech).elements,
                    gss_mech_ntlmssp_oid.elements,
                    (*(*sc).internal_mech).length as libc::c_ulong) ==
                 0 as libc::c_int) {
        return 0 as libc::c_int as OM_uint32
    }
    value.length = ::std::mem::size_of::<OM_uint32>() as libc::c_ulong;
    value.value = &mut verify as *mut OM_uint32 as *mut libc::c_void;
    major =
        gss_set_sec_context_option(&mut minor, &mut (*sc).ctx_handle,
                                   &ntlmssp_reset_crypto_oid as
                                       *const gss_OID_desc as gss_OID,
                                   &mut value);
    if major == (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        return 0 as libc::c_int as OM_uint32
    }
    *minor_status = minor;
    return major;
}
/*
 * Both initiator and acceptor call here to verify and/or create mechListMIC,
 * and to consistency-check the MIC state.  handle_mic is invoked only if the
 * negotiated mech has completed and supports MICs.
 */
#[c2rust::src_loc = "558:1"]
unsafe extern "C" fn handle_mic(mut minor_status: *mut OM_uint32,
                                mut mic_in: gss_buffer_t,
                                mut send_mechtok: libc::c_int,
                                mut sc: spnego_gss_ctx_id_t,
                                mut mic_out: *mut gss_buffer_t,
                                mut negState: *mut OM_uint32,
                                mut tokflag: *mut send_token_flag)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    *mic_out = 0 as gss_buffer_t;
    if !mic_in.is_null() {
        if (*sc).mic_rcvd != 0 {
            /* Reject MIC if we've already received a MIC. */
            *negState = 2 as libc::c_int as OM_uint32;
            *tokflag = ERROR_TOKEN_SEND;
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    } else if (*sc).mic_reqd != 0 && send_mechtok == 0 {
        /*
		 * If the peer sends the final mechanism token, it
		 * must send the MIC with that token if the
		 * negotiation requires MICs.
		 */
        *negState = 2 as libc::c_int as OM_uint32;
        *tokflag = ERROR_TOKEN_SEND;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret = process_mic(minor_status, mic_in, sc, mic_out, negState, tokflag);
    if ret != 0 as libc::c_int as libc::c_uint { return ret }
    if (*sc).mic_reqd != 0 {
        if (*sc).mic_sent != 0 || (*sc).mic_rcvd != 0 {
        } else {
            __assert_fail(b"sc->mic_sent || sc->mic_rcvd\x00" as *const u8 as
                              *const libc::c_char,
                          b"spnego_mech.c\x00" as *const u8 as
                              *const libc::c_char,
                          591 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 122],
                                                    &[libc::c_char; 122]>(b"OM_uint32 handle_mic(OM_uint32 *, gss_buffer_t, int, spnego_gss_ctx_id_t, gss_buffer_t *, OM_uint32 *, send_token_flag *)\x00")).as_ptr());
        }
    }
    if (*sc).mic_sent != 0 && (*sc).mic_rcvd != 0 {
        ret = 0 as libc::c_int as OM_uint32;
        *negState = 0 as libc::c_int as OM_uint32;
        if (*mic_out).is_null() {
            /*
			 * We sent a MIC on the previous pass; we
			 * shouldn't be sending a mechanism token.
			 */
            if send_mechtok == 0 {
            } else {
                __assert_fail(b"!send_mechtok\x00" as *const u8 as
                                  *const libc::c_char,
                              b"spnego_mech.c\x00" as *const u8 as
                                  *const libc::c_char,
                              601 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 122],
                                                        &[libc::c_char; 122]>(b"OM_uint32 handle_mic(OM_uint32 *, gss_buffer_t, int, spnego_gss_ctx_id_t, gss_buffer_t *, OM_uint32 *, send_token_flag *)\x00")).as_ptr());
            }
            *tokflag = NO_TOKEN_SEND
        } else { *tokflag = CONT_TOKEN_SEND }
    } else if (*sc).mic_reqd != 0 {
        *negState = 1 as libc::c_int as OM_uint32;
        ret =
            ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int) as
                OM_uint32
    } else if *negState == 0 as libc::c_int as libc::c_uint {
        ret = 0 as libc::c_int as OM_uint32
    } else {
        ret =
            ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int) as
                OM_uint32
    }
    return ret;
}
/*
 * Perform the actual verification and/or generation of mechListMIC.
 */
#[c2rust::src_loc = "620:1"]
unsafe extern "C" fn process_mic(mut minor_status: *mut OM_uint32,
                                 mut mic_in: gss_buffer_t,
                                 mut sc: spnego_gss_ctx_id_t,
                                 mut mic_out: *mut gss_buffer_t,
                                 mut negState: *mut OM_uint32,
                                 mut tokflag: *mut send_token_flag)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut qop_state: gss_qop_t = 0;
    let mut tmpmic: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    if !mic_in.is_null() {
        ret =
            gss_verify_mic(minor_status, (*sc).ctx_handle,
                           &mut (*sc).DER_mechTypes, mic_in, &mut qop_state);
        if ret == 0 as libc::c_int as libc::c_uint {
            ret =
                ntlmssp_reset_crypto_state(minor_status, sc,
                                           1 as libc::c_int as OM_uint32)
        }
        if ret != 0 as libc::c_int as libc::c_uint {
            *negState = 2 as libc::c_int as OM_uint32;
            *tokflag = ERROR_TOKEN_SEND;
            return ret
        }
        /* If we got a MIC, we must send a MIC. */
        (*sc).mic_reqd = 1 as libc::c_int;
        (*sc).mic_rcvd = 1 as libc::c_int
    }
    if (*sc).mic_reqd != 0 && (*sc).mic_sent == 0 {
        ret =
            gss_get_mic(minor_status, (*sc).ctx_handle,
                        0 as libc::c_int as gss_qop_t,
                        &mut (*sc).DER_mechTypes, &mut tmpmic);
        if ret == 0 as libc::c_int as libc::c_uint {
            ret =
                ntlmssp_reset_crypto_state(minor_status, sc,
                                           0 as libc::c_int as OM_uint32)
        }
        if ret != 0 as libc::c_int as libc::c_uint {
            gss_release_buffer(&mut tmpmin, &mut tmpmic);
            *tokflag = NO_TOKEN_SEND;
            return ret
        }
        *mic_out =
            malloc(::std::mem::size_of::<gss_buffer_desc>() as libc::c_ulong)
                as gss_buffer_t;
        if (*mic_out).is_null() {
            gss_release_buffer(&mut tmpmin, &mut tmpmic);
            *tokflag = NO_TOKEN_SEND;
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        **mic_out = tmpmic;
        (*sc).mic_sent = 1 as libc::c_int
    }
    return 0 as libc::c_int as OM_uint32;
}
/* Create a new SPNEGO context handle for the initial call to
 * spnego_gss_init_sec_context().  */
#[c2rust::src_loc = "671:1"]
unsafe extern "C" fn init_ctx_new(mut minor_status: *mut OM_uint32,
                                  mut spcred: spnego_gss_cred_id_t,
                                  mut tokflag: *mut send_token_flag,
                                  mut sc_out: *mut spnego_gss_ctx_id_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = 0 as spnego_gss_ctx_id_t;
    *sc_out = 0 as spnego_gss_ctx_id_t;
    sc = create_spnego_ctx(1 as libc::c_int);
    if sc.is_null() {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* determine negotiation mech set */
    ret = get_negotiable_mechs(minor_status, sc, spcred, 1 as libc::c_int);
    if !(ret != 0 as libc::c_int as libc::c_uint) {
        /* Set an initial internal mech to make the first context token. */
        (*sc).internal_mech =
            &mut *(*(*sc).mech_set).elements.offset(0 as libc::c_int as isize)
                as *mut gss_OID_desc_struct;
        if put_mech_set((*sc).mech_set, &mut (*sc).DER_mechTypes) <
               0 as libc::c_int {
            ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        } else {
            (*sc).ctx_handle = 0 as gss_ctx_id_t;
            *sc_out = sc;
            sc = 0 as spnego_gss_ctx_id_t;
            *tokflag = INIT_TOKEN_SEND;
            ret = 0 as libc::c_int as OM_uint32
        }
    }
    release_spnego_ctx(&mut sc);
    return ret;
}
/*
 * Called by second and later calls to spnego_gss_init_sec_context()
 * to decode reply and update state.
 */
#[c2rust::src_loc = "714:1"]
unsafe extern "C" fn init_ctx_cont(mut minor_status: *mut OM_uint32,
                                   mut sc: spnego_gss_ctx_id_t,
                                   mut buf: gss_buffer_t,
                                   mut responseToken: *mut gss_buffer_t,
                                   mut mechListMIC: *mut gss_buffer_t,
                                   mut acc_negState: *mut OM_uint32,
                                   mut tokflag: *mut send_token_flag)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut supportedMech: gss_OID = 0 as gss_OID;
    *acc_negState = 0xffffffff as libc::c_ulong as OM_uint32;
    *tokflag = ERROR_TOKEN_SEND;
    ptr = (*buf).value as *mut libc::c_uchar;
    ret =
        get_negTokenResp(minor_status, ptr, (*buf).length as libc::c_uint,
                         acc_negState, &mut supportedMech, responseToken,
                         mechListMIC);
    if !(ret != 0 as libc::c_int as libc::c_uint) {
        /* Bail out now on a reject with no error token.  If we have an error
	 * token, keep going and get a better error status from the mech. */
        if *acc_negState == 2 as libc::c_int as libc::c_uint &&
               (*responseToken).is_null() {
            if (*sc).nego_done == 0 {
                /* RFC 4178 says to return GSS_S_BAD_MECH on a
			 * mechanism negotiation failure. */
                *minor_status = 0x20000004 as libc::c_int as OM_uint32;
                *minor_status = gssint_mecherrmap_map_errcode(*minor_status);
                ret = (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
            } else {
                ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
            }
            *tokflag = NO_TOKEN_SEND
        } else if (*sc).nego_done == 0 {
            ret =
                init_ctx_nego(minor_status, sc, *acc_negState, supportedMech,
                              responseToken, mechListMIC, tokflag)
        } else if (*sc).mech_complete == 0 && (*responseToken).is_null() ||
                      (*sc).mech_complete != 0 && !(*responseToken).is_null()
         {
            /*
	 * nego_done is false for the first call to init_ctx_cont()
	 */
            /* Missing or spurious token from acceptor. */
            ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        } else if (*sc).mech_complete == 0 ||
                      (*sc).mic_reqd != 0 &&
                          (*sc).ctx_flags & 32 as libc::c_int as libc::c_uint
                              != 0 {
            /* Not obviously done; we may decide we're done later in
		 * init_ctx_call_init or handle_mic. */
            *tokflag = CONT_TOKEN_SEND;
            ret = 0 as libc::c_int as OM_uint32
        } else {
            /* mech finished on last pass and no MIC required, so done. */
            *tokflag = NO_TOKEN_SEND;
            ret = 0 as libc::c_int as OM_uint32
        }
    }
    if !supportedMech.is_null() {
        generic_gss_release_oid(&mut tmpmin, &mut supportedMech);
    }
    return ret;
}
/*
 * Consistency checking and mechanism negotiation handling for second
 * call of spnego_gss_init_sec_context().  Call init_ctx_reselect() to
 * update internal state if acceptor has counter-proposed.
 */
#[c2rust::src_loc = "782:1"]
unsafe extern "C" fn init_ctx_nego(mut minor_status: *mut OM_uint32,
                                   mut sc: spnego_gss_ctx_id_t,
                                   mut acc_negState: OM_uint32,
                                   mut supportedMech: gss_OID,
                                   mut responseToken: *mut gss_buffer_t,
                                   mut mechListMIC: *mut gss_buffer_t,
                                   mut tokflag: *mut send_token_flag)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    *tokflag = ERROR_TOKEN_SEND;
    ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    /*
	 * According to RFC 4178, both supportedMech and negState must be
	 * present in the first acceptor token.  However, some Java
	 * implementations include only a responseToken in the first
	 * NegTokenResp.  In this case we can use sc->internal_mech as the
	 * negotiated mechanism.  (We do not currently look at acc_negState
	 * when continuing with the optimistic mechanism.)
	 */
    if supportedMech.is_null() { supportedMech = (*sc).internal_mech }
    /*
	 * If the mechanism we sent is not the mechanism returned from
	 * the server, we need to handle the server's counter
	 * proposal.  There is a bug in SAMBA servers that always send
	 * the old Kerberos mech OID, even though we sent the new one.
	 * So we will treat all the Kerberos mech OIDS as the same.
         */
    if !(is_kerb_mech(supportedMech) != 0 &&
             is_kerb_mech((*sc).internal_mech) != 0) &&
           !((*supportedMech).length == (*(*sc).internal_mech).length &&
                 memcmp((*supportedMech).elements,
                        (*(*sc).internal_mech).elements,
                        (*supportedMech).length as libc::c_ulong) ==
                     0 as libc::c_int) {
        ret =
            init_ctx_reselect(minor_status, sc, acc_negState, supportedMech,
                              responseToken, mechListMIC, tokflag)
    } else if (*responseToken).is_null() {
        if (*sc).mech_complete != 0 {
            /*
			 * Mech completed on first call to its
			 * init_sec_context().  Acceptor sends no mech
			 * token.
			 */
            *tokflag = NO_TOKEN_SEND;
            ret = 0 as libc::c_int as OM_uint32
        } else {
            /*
			 * Reject missing mech token when optimistic
			 * mech selected.
			 */
            *minor_status = 0x20000005 as libc::c_int as OM_uint32;
            *minor_status = gssint_mecherrmap_map_errcode(*minor_status);
            ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    } else if (**responseToken).length == 0 as libc::c_int as libc::c_ulong &&
                  (*sc).mech_complete != 0 {
        /* Handle old IIS servers returning empty token instead of
		 * null tokens in the non-mutual auth case. */
        *tokflag = NO_TOKEN_SEND;
        ret = 0 as libc::c_int as OM_uint32
    } else if (*sc).mech_complete != 0 {
        /* Reject spurious mech token. */
        ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    } else { *tokflag = CONT_TOKEN_SEND; ret = 0 as libc::c_int as OM_uint32 }
    (*sc).nego_done = 1 as libc::c_int;
    return ret;
}
/*
 * Handle acceptor's counter-proposal of an alternative mechanism.
 */
#[c2rust::src_loc = "855:1"]
unsafe extern "C" fn init_ctx_reselect(mut minor_status: *mut OM_uint32,
                                       mut sc: spnego_gss_ctx_id_t,
                                       mut acc_negState: OM_uint32,
                                       mut supportedMech: gss_OID,
                                       mut responseToken: *mut gss_buffer_t,
                                       mut mechListMIC: *mut gss_buffer_t,
                                       mut tokflag: *mut send_token_flag)
 -> OM_uint32 {
    let mut tmpmin: OM_uint32 = 0;
    let mut i: size_t = 0;
    gss_delete_sec_context(&mut tmpmin, &mut (*sc).ctx_handle,
                           0 as gss_buffer_t);
    /* Find supportedMech in sc->mech_set. */
    i = 0 as libc::c_int as size_t;
    while i < (*(*sc).mech_set).count {
        if (*supportedMech).length ==
               (*(*(*sc).mech_set).elements.offset(i as isize)).length &&
               memcmp((*supportedMech).elements,
                      (*(*(*sc).mech_set).elements.offset(i as
                                                              isize)).elements,
                      (*supportedMech).length as libc::c_ulong) ==
                   0 as libc::c_int {
            break ;
        }
        i = i.wrapping_add(1)
    }
    if i == (*(*sc).mech_set).count {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    (*sc).internal_mech =
        &mut *(*(*sc).mech_set).elements.offset(i as isize) as
            *mut gss_OID_desc_struct;
    /*
	 * A server conforming to RFC4178 MUST set REQUEST_MIC here, but
	 * Windows Server 2003 and earlier implement (roughly) RFC2478 instead,
	 * and send ACCEPT_INCOMPLETE.  Tolerate that only if we are falling
	 * back to NTLMSSP.
	 */
    if acc_negState == 1 as libc::c_int as libc::c_uint {
        if !((*supportedMech).length == gss_mech_ntlmssp_oid.length &&
                 memcmp((*supportedMech).elements,
                        gss_mech_ntlmssp_oid.elements,
                        (*supportedMech).length as libc::c_ulong) ==
                     0 as libc::c_int) {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    } else if acc_negState != 3 as libc::c_int as libc::c_uint {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    (*sc).mech_complete = 0 as libc::c_int;
    (*sc).mic_reqd =
        (acc_negState == 3 as libc::c_int as libc::c_uint) as libc::c_int;
    *tokflag = CONT_TOKEN_SEND;
    return 0 as libc::c_int as OM_uint32;
}
/*
 * Wrap call to mechanism gss_init_sec_context() and update state
 * accordingly.
 */
#[c2rust::src_loc = "899:1"]
unsafe extern "C" fn init_ctx_call_init(mut minor_status: *mut OM_uint32,
                                        mut sc: spnego_gss_ctx_id_t,
                                        mut spcred: spnego_gss_cred_id_t,
                                        mut acc_negState: OM_uint32,
                                        mut target_name: gss_name_t,
                                        mut req_flags: OM_uint32,
                                        mut time_req: OM_uint32,
                                        mut mechtok_in: gss_buffer_t,
                                        mut mechtok_out: gss_buffer_t,
                                        mut time_rec: *mut OM_uint32,
                                        mut send_token: *mut send_token_flag)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut tmpret: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut mech_req_flags: OM_uint32 = 0;
    let mut mcred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    mcred =
        if spcred.is_null() { 0 as gss_cred_id_t } else { (*spcred).mcred };
    mech_req_flags = req_flags;
    if spcred.is_null() || (*spcred).no_ask_integ == 0 {
        mech_req_flags |= 32 as libc::c_int as libc::c_uint
    }
    if gss_oid_equal((*sc).internal_mech as gss_const_OID,
                     &mut negoex_mech as *mut gss_OID_desc as gss_const_OID)
           != 0 {
        ret =
            negoex_init(minor_status, sc, mcred, target_name, mech_req_flags,
                        time_req, mechtok_in, mechtok_out, time_rec)
    } else {
        ret =
            gss_init_sec_context(minor_status, mcred, &mut (*sc).ctx_handle,
                                 target_name, (*sc).internal_mech,
                                 mech_req_flags, time_req,
                                 0 as gss_channel_bindings_t, mechtok_in,
                                 &mut (*sc).actual_mech, mechtok_out,
                                 &mut (*sc).ctx_flags, time_rec)
    }
    /* Bail out if the acceptor gave us an error token but the mech didn't
	 * see it as an error. */
    if acc_negState == 2 as libc::c_int as libc::c_uint &&
           ret &
               ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                    (0o377 as libc::c_ulong as OM_uint32) <<
                        16 as libc::c_int) == 0 {
        ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    } else {
        if ret == 0 as libc::c_int as libc::c_uint {
            (*sc).mech_complete = 1 as libc::c_int;
            /*
		 * Microsoft SPNEGO implementations expect an even number of
		 * token exchanges.  So if we're sending a final token, ask for
		 * a zero-length token back from the server.  Also ask for a
		 * token back if this is the first token or if a MIC exchange
		 * is required.
		 */
            if *send_token as libc::c_uint ==
                   CONT_TOKEN_SEND as libc::c_int as libc::c_uint &&
                   (*mechtok_out).length == 0 as libc::c_int as libc::c_ulong
                   &&
                   ((*sc).mic_reqd == 0 ||
                        (*sc).ctx_flags & 32 as libc::c_int as libc::c_uint ==
                            0) {
                *send_token = NO_TOKEN_SEND
            }
            return 0 as libc::c_int as OM_uint32
        }
        if ret ==
               ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int) as
                   libc::c_uint {
            return 0 as libc::c_int as OM_uint32
        }
        if *send_token as libc::c_uint !=
               INIT_TOKEN_SEND as libc::c_int as libc::c_uint {
            *send_token = ERROR_TOKEN_SEND;
            return ret
        }
        /*
	 * Since this is the first token, we can fall back to later mechanisms
	 * in the list.  Since the mechanism list is expected to be short, we
	 * can do this with recursion.  If all mechanisms produce errors, the
	 * caller should get the error from the first mech in the list.
	 */
        gssalloc_free((*(*(*sc).mech_set).elements).elements);
        (*(*sc).mech_set).count = (*(*sc).mech_set).count.wrapping_sub(1);
        memmove((*(*sc).mech_set).elements as *mut libc::c_void,
                (*(*sc).mech_set).elements.offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                (*(*sc).mech_set).count.wrapping_mul(::std::mem::size_of::<gss_OID_desc_struct>()
                                                         as libc::c_ulong));
        if !((*(*sc).mech_set).count == 0 as libc::c_int as libc::c_ulong) {
            gss_release_buffer(&mut tmpmin, &mut (*sc).DER_mechTypes);
            if !(put_mech_set((*sc).mech_set, &mut (*sc).DER_mechTypes) <
                     0 as libc::c_int) {
                gss_delete_sec_context(&mut tmpmin, &mut (*sc).ctx_handle,
                                       0 as gss_buffer_t);
                tmpret =
                    init_ctx_call_init(&mut tmpmin, sc, spcred, acc_negState,
                                       target_name, req_flags, time_req,
                                       mechtok_in, mechtok_out, time_rec,
                                       send_token);
                if !(tmpret != 0 as libc::c_int as libc::c_uint &&
                         tmpret !=
                             ((1 as libc::c_int) <<
                                  0 as libc::c_int + 0 as libc::c_int) as
                                 libc::c_uint) {
                    *minor_status = tmpmin;
                    return tmpret
                }
            }
        }
    }
    /* Don't output token on error from first call. */
    *send_token = NO_TOKEN_SEND;
    return ret;
}
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "998:1"]
pub unsafe extern "C" fn spnego_gss_init_sec_context(mut minor_status:
                                                         *mut OM_uint32,
                                                     mut claimant_cred_handle:
                                                         gss_cred_id_t,
                                                     mut context_handle:
                                                         *mut gss_ctx_id_t,
                                                     mut target_name:
                                                         gss_name_t,
                                                     mut mech_type: gss_OID,
                                                     mut req_flags: OM_uint32,
                                                     mut time_req: OM_uint32,
                                                     mut input_chan_bindings:
                                                         gss_channel_bindings_t,
                                                     mut input_token:
                                                         gss_buffer_t,
                                                     mut actual_mech:
                                                         *mut gss_OID,
                                                     mut output_token:
                                                         gss_buffer_t,
                                                     mut ret_flags:
                                                         *mut OM_uint32,
                                                     mut time_rec:
                                                         *mut OM_uint32)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut send_token: send_token_flag = NO_TOKEN_SEND;
    let mut tmpmin: OM_uint32 = 0;
    let mut ret: OM_uint32 = 0;
    let mut negState: OM_uint32 = 0xffffffff as libc::c_ulong as OM_uint32;
    let mut acc_negState: OM_uint32 = 0;
    let mut mechtok_in: gss_buffer_t = 0 as *mut gss_buffer_desc_struct;
    let mut mechListMIC_in: gss_buffer_t = 0 as *mut gss_buffer_desc_struct;
    let mut mechListMIC_out: gss_buffer_t = 0 as *mut gss_buffer_desc_struct;
    let mut mechtok_out: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut spcred: spnego_gss_cred_id_t = 0 as spnego_gss_cred_id_t;
    let mut spnego_ctx: spnego_gss_ctx_id_t = 0 as spnego_gss_ctx_id_t;
    mechListMIC_in = 0 as gss_buffer_t;
    mechListMIC_out = mechListMIC_in;
    mechtok_in = mechListMIC_out;
    /*
	 * This function works in three steps:
	 *
	 *   1. Perform mechanism negotiation.
	 *   2. Invoke the negotiated or optimistic mech's gss_init_sec_context
	 *      function and examine the results.
	 *   3. Process or generate MICs if necessary.
	 *
	 * The three steps share responsibility for determining when the
	 * exchange is complete.  If the selected mech completed in a previous
	 * call and no MIC exchange is expected, then step 1 will decide.  If
	 * the selected mech completes in this call and no MIC exchange is
	 * expected, then step 2 will decide.  If a MIC exchange is expected,
	 * then step 3 will decide.  If an error occurs in any step, the
	 * exchange will be aborted, possibly with an error token.
	 *
	 * negState determines the state of the negotiation, and is
	 * communicated to the acceptor if a continuing token is sent.
	 * send_token is used to indicate what type of token, if any, should be
	 * generated.
	 */
    /* Validate arguments. */
    if !minor_status.is_null() {
        *minor_status = 0 as libc::c_int as OM_uint32
    }
    if !output_token.is_null() {
        (*output_token).length = 0 as libc::c_int as size_t;
        (*output_token).value = 0 as *mut libc::c_void
    }
    if minor_status.is_null() || output_token.is_null() ||
           context_handle.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if !actual_mech.is_null() { *actual_mech = 0 as gss_OID }
    if !time_rec.is_null() { *time_rec = 0 as libc::c_int as OM_uint32 }
    /* Step 1: perform mechanism negotiation. */
    spcred = claimant_cred_handle as spnego_gss_cred_id_t;
    spnego_ctx = *context_handle as spnego_gss_ctx_id_t;
    if spnego_ctx.is_null() {
        ret =
            init_ctx_new(minor_status, spcred, &mut send_token,
                         &mut spnego_ctx);
        if ret != 0 as libc::c_int as libc::c_uint {
            current_block = 16927928461156034833;
        } else {
            *context_handle = spnego_ctx as gss_ctx_id_t;
            acc_negState = 0xffffffff as libc::c_ulong as OM_uint32;
            current_block = 9828876828309294594;
        }
    } else {
        ret =
            init_ctx_cont(minor_status, spnego_ctx, input_token,
                          &mut mechtok_in, &mut mechListMIC_in,
                          &mut acc_negState, &mut send_token);
        if ret != 0 as libc::c_int as libc::c_uint {
            current_block = 16927928461156034833;
        } else { current_block = 9828876828309294594; }
    }
    match current_block {
        9828876828309294594 =>
        /* Step 2: invoke the selected or optimistic mechanism's
	 * gss_init_sec_context function, if it didn't complete previously. */
        {
            if (*spnego_ctx).mech_complete == 0 {
                ret =
                    init_ctx_call_init(minor_status, spnego_ctx, spcred,
                                       acc_negState, target_name, req_flags,
                                       time_req, mechtok_in, &mut mechtok_out,
                                       time_rec, &mut send_token);
                if ret != 0 as libc::c_int as libc::c_uint {
                    current_block = 16927928461156034833;
                } else {
                    /* Give the mechanism a chance to force a mechlistMIC. */
                    if mech_requires_mechlistMIC(spnego_ctx) != 0 {
                        (*spnego_ctx).mic_reqd = 1 as libc::c_int
                    }
                    current_block = 8693738493027456495;
                }
            } else { current_block = 8693738493027456495; }
            match current_block {
                16927928461156034833 => { }
                _ => {
                    /* Step 3: process or generate the MIC, if the negotiated mech is
	 * complete and supports MICs.  Also decide the outgoing negState. */
                    negState = 1 as libc::c_int as OM_uint32;
                    if (*spnego_ctx).mech_complete != 0 &&
                           (*spnego_ctx).ctx_flags &
                               32 as libc::c_int as libc::c_uint != 0 {
                        ret =
                            handle_mic(minor_status, mechListMIC_in,
                                       (mechtok_out.length !=
                                            0 as libc::c_int as libc::c_ulong)
                                           as libc::c_int, spnego_ctx,
                                       &mut mechListMIC_out, &mut negState,
                                       &mut send_token);
                        if ret != 0 as libc::c_int as libc::c_uint &&
                               ret !=
                                   ((1 as libc::c_int) <<
                                        0 as libc::c_int + 0 as libc::c_int)
                                       as libc::c_uint {
                            current_block = 16927928461156034833;
                        } else { current_block = 7333393191927787629; }
                    } else { current_block = 7333393191927787629; }
                    match current_block {
                        16927928461156034833 => { }
                        _ => {
                            if !ret_flags.is_null() {
                                *ret_flags =
                                    (*spnego_ctx).ctx_flags &
                                        !(128 as libc::c_int) as libc::c_uint
                            }
                            ret =
                                if send_token as libc::c_uint ==
                                       NO_TOKEN_SEND as libc::c_int as
                                           libc::c_uint ||
                                       negState ==
                                           0 as libc::c_int as libc::c_uint {
                                    0 as libc::c_int
                                } else {
                                    ((1 as libc::c_int)) <<
                                        0 as libc::c_int + 0 as libc::c_int
                                } as OM_uint32
                        }
                    }
                }
            }
        }
        _ => { }
    }
    if send_token as libc::c_uint ==
           INIT_TOKEN_SEND as libc::c_int as libc::c_uint {
        if make_spnego_tokenInit_msg(spnego_ctx, 0 as libc::c_int,
                                     mechListMIC_out, req_flags,
                                     &mut mechtok_out, send_token,
                                     output_token) < 0 as libc::c_int {
            ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    } else if send_token as libc::c_uint !=
                  NO_TOKEN_SEND as libc::c_int as libc::c_uint {
        if send_token as libc::c_uint ==
               ERROR_TOKEN_SEND as libc::c_int as libc::c_uint {
            negState = 2 as libc::c_int as OM_uint32
        }
        if make_spnego_tokenTarg_msg(negState, 0 as gss_OID, &mut mechtok_out,
                                     mechListMIC_out, send_token,
                                     output_token) < 0 as libc::c_int {
            ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    gss_release_buffer(&mut tmpmin, &mut mechtok_out);
    if ret == 0 as libc::c_int as libc::c_uint {
        (*spnego_ctx).opened = 1 as libc::c_int;
        if !actual_mech.is_null() { *actual_mech = (*spnego_ctx).actual_mech }
        /* Get an updated lifetime if we didn't call into the mech. */
        if !time_rec.is_null() &&
               *time_rec == 0 as libc::c_int as libc::c_uint {
            gss_context_time(&mut tmpmin, (*spnego_ctx).ctx_handle, time_rec);
        }
    } else if ret !=
                  ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int)
                      as libc::c_uint {
        if !spnego_ctx.is_null() {
            gss_delete_sec_context(&mut tmpmin, &mut (*spnego_ctx).ctx_handle,
                                   0 as gss_buffer_t);
            release_spnego_ctx(&mut spnego_ctx);
        }
        *context_handle = 0 as gss_ctx_id_t
    }
    if !mechtok_in.is_null() {
        gss_release_buffer(&mut tmpmin, mechtok_in);
        free(mechtok_in as *mut libc::c_void);
    }
    if !mechListMIC_in.is_null() {
        gss_release_buffer(&mut tmpmin, mechListMIC_in);
        free(mechListMIC_in as *mut libc::c_void);
    }
    if !mechListMIC_out.is_null() {
        gss_release_buffer(&mut tmpmin, mechListMIC_out);
        free(mechListMIC_out as *mut libc::c_void);
    }
    return ret;
}
/* init_sec_context */
/* We don't want to import KRB5 headers here */
#[c2rust::src_loc = "1174:27"]
static mut gss_mech_krb5_oid: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 9 as libc::c_int as OM_uint32,
                                elements:
                                    b"*\x86H\x86\xf7\x12\x01\x02\x02\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
#[c2rust::src_loc = "1176:27"]
static mut gss_mech_krb5_wrong_oid: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 9 as libc::c_int as OM_uint32,
                                elements:
                                    b"*\x86H\x82\xf7\x12\x01\x02\x02\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
/*
 * verify that the input token length is not 0. If it is, just return.
 * If the token length is greater than 0, der encode as a sequence
 * and place in buf_out, advancing buf_out.
 */
#[c2rust::src_loc = "1185:1"]
unsafe extern "C" fn put_neg_hints(mut buf_out: *mut *mut libc::c_uchar,
                                   mut input_token: gss_buffer_t,
                                   mut buflen: libc::c_uint) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    /* if token length is 0, we do not want to send */
    if (*input_token).length == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    if (*input_token).length > buflen as libc::c_ulong {
        return -(1 as libc::c_int)
    }
    let fresh0 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh0 = 0x30 as libc::c_int as libc::c_uchar;
    ret =
        gssint_put_der_length((*input_token).length as libc::c_uint, buf_out,
                              (*input_token).length as libc::c_uint);
    if ret != 0 { return ret }
    memcpy(*buf_out as *mut libc::c_void, (*input_token).value,
           (*input_token).length);
    *buf_out = (*buf_out).offset((*input_token).length as isize);
    return 0 as libc::c_int;
}
/*
 * NegHints ::= SEQUENCE {
 *    hintName       [0]  GeneralString      OPTIONAL,
 *    hintAddress    [1]  OCTET STRING       OPTIONAL
 * }
 */
/* Encode the dummy hintname string (as specified in [MS-SPNG]) into a
 * DER-encoded [0] tagged GeneralString, and place the result in *outbuf. */
#[c2rust::src_loc = "1218:1"]
unsafe extern "C" fn make_NegHints(mut minor_status: *mut OM_uint32,
                                   mut outbuf: *mut gss_buffer_t)
 -> libc::c_int {
    let mut major_status: OM_uint32 = 0;
    let mut tlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut hintNameSize: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut t: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut hintname: *const libc::c_char =
        b"not_defined_in_RFC4178@please_ignore\x00" as *const u8 as
            *const libc::c_char;
    let hintname_len: size_t = strlen(hintname);
    *outbuf = 0 as gss_buffer_t;
    major_status = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    /* Length of DER encoded GeneralString */
    tlen =
        ((1 as libc::c_int as
              libc::c_uint).wrapping_add(gssint_der_length_size(hintname_len
                                                                    as
                                                                    libc::c_uint))
             as libc::c_ulong).wrapping_add(hintname_len) as libc::c_uint;
    hintNameSize = tlen;
    /* Length of DER encoded hintName */
    tlen =
        tlen.wrapping_add((1 as libc::c_int as
                               libc::c_uint).wrapping_add(gssint_der_length_size(hintNameSize))); /* hintName identifier */
    t =
        gssalloc_malloc(tlen as size_t) as
            *mut libc::c_uchar; /* don't free */
    if t.is_null() {
        *minor_status = 12 as libc::c_int as OM_uint32
    } else {
        ptr = t;
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        *fresh1 = (0xa0 as libc::c_int | 0 as libc::c_int) as libc::c_uchar;
        if !(gssint_put_der_length(hintNameSize, &mut ptr,
                                   tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                         as libc::c_long as
                                                         libc::c_int as
                                                         libc::c_uint)) != 0)
           {
            let fresh2 = ptr;
            ptr = ptr.offset(1);
            *fresh2 = 0x1b as libc::c_int as libc::c_uchar;
            if !(gssint_put_der_length(hintname_len as libc::c_uint, &mut ptr,
                                       tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                             as libc::c_long
                                                             as libc::c_int as
                                                             libc::c_uint)) !=
                     0) {
                memcpy(ptr as *mut libc::c_void,
                       hintname as *const libc::c_void, hintname_len);
                ptr = ptr.offset(hintname_len as isize);
                *outbuf =
                    malloc(::std::mem::size_of::<gss_buffer_desc>() as
                               libc::c_ulong) as gss_buffer_t;
                if (*outbuf).is_null() {
                    *minor_status = 12 as libc::c_int as OM_uint32
                } else {
                    (**outbuf).value = t as *mut libc::c_void;
                    (**outbuf).length =
                        ptr.wrapping_offset_from(t) as libc::c_long as size_t;
                    t = 0 as *mut libc::c_uchar;
                    *minor_status = 0 as libc::c_int as OM_uint32;
                    major_status = 0 as libc::c_int as OM_uint32
                }
            }
        }
    }
    if !t.is_null() { free(t as *mut libc::c_void); }
    return major_status as libc::c_int;
}
/*
 * Create a new SPNEGO context handle for the initial call to
 * spnego_gss_accept_sec_context() when the request is empty.  For empty
 * requests, we implement the Microsoft NegHints extension to SPNEGO for
 * compatibility with some versions of Samba.  See:
 * https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-spng/8e71cf53-e867-4b79-b5b5-38c92be3d472
 */
#[c2rust::src_loc = "1287:1"]
unsafe extern "C" fn acc_ctx_hints(mut minor_status: *mut OM_uint32,
                                   mut spcred: spnego_gss_cred_id_t,
                                   mut mechListMIC: *mut gss_buffer_t,
                                   mut negState: *mut OM_uint32,
                                   mut return_token: *mut send_token_flag,
                                   mut sc_out: *mut spnego_gss_ctx_id_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = 0 as spnego_gss_ctx_id_t;
    *mechListMIC = 0 as gss_buffer_t;
    *return_token = NO_TOKEN_SEND;
    *negState = 2 as libc::c_int as OM_uint32;
    *minor_status = 0 as libc::c_int as OM_uint32;
    *sc_out = 0 as spnego_gss_ctx_id_t;
    ret = make_NegHints(minor_status, mechListMIC) as OM_uint32;
    if !(ret != 0 as libc::c_int as libc::c_uint) {
        sc = create_spnego_ctx(0 as libc::c_int);
        if sc.is_null() {
            ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        } else {
            ret =
                get_negotiable_mechs(minor_status, sc, spcred,
                                     2 as libc::c_int);
            if !(ret != 0 as libc::c_int as libc::c_uint) {
                if put_mech_set((*sc).mech_set, &mut (*sc).DER_mechTypes) <
                       0 as libc::c_int {
                    ret =
                        (13 as libc::c_ulong as OM_uint32) <<
                            16 as libc::c_int
                } else {
                    (*sc).internal_mech = 0 as gss_OID;
                    *negState = 1 as libc::c_int as OM_uint32;
                    *return_token = INIT_TOKEN_SEND;
                    (*sc).firstpass = 1 as libc::c_int;
                    *sc_out = sc;
                    sc = 0 as spnego_gss_ctx_id_t;
                    ret = 0 as libc::c_int as OM_uint32
                }
            }
        }
    }
    release_spnego_ctx(&mut sc);
    return ret;
}
/*
 * Create a new SPNEGO context handle for the initial call to
 * spnego_gss_accept_sec_context().  Set negState to REJECT if the token is
 * defective, else ACCEPT_INCOMPLETE or REQUEST_MIC, depending on whether
 * the initiator's preferred mechanism is supported.
 */
#[c2rust::src_loc = "1343:1"]
unsafe extern "C" fn acc_ctx_new(mut minor_status: *mut OM_uint32,
                                 mut buf: gss_buffer_t,
                                 mut spcred: spnego_gss_cred_id_t,
                                 mut mechToken: *mut gss_buffer_t,
                                 mut mechListMIC: *mut gss_buffer_t,
                                 mut negState: *mut OM_uint32,
                                 mut return_token: *mut send_token_flag,
                                 mut sc_out: *mut spnego_gss_ctx_id_t)
 -> OM_uint32 {
    let mut tmpmin: OM_uint32 = 0;
    let mut ret: OM_uint32 = 0;
    let mut req_flags: OM_uint32 = 0;
    let mut mechTypes: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut der_mechTypes: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut mech_wanted: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut sc: spnego_gss_ctx_id_t = 0 as spnego_gss_ctx_id_t;
    ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    der_mechTypes.length = 0 as libc::c_int as size_t;
    der_mechTypes.value = 0 as *mut libc::c_void;
    *mechListMIC = 0 as gss_buffer_t;
    *mechToken = *mechListMIC;
    mechTypes = 0 as gss_OID_set;
    *return_token = ERROR_TOKEN_SEND;
    *negState = 2 as libc::c_int as OM_uint32;
    *minor_status = 0 as libc::c_int as OM_uint32;
    ret =
        get_negTokenInit(minor_status, buf, &mut der_mechTypes,
                         &mut mechTypes, &mut req_flags, mechToken,
                         mechListMIC);
    if !(ret != 0 as libc::c_int as libc::c_uint) {
        sc = create_spnego_ctx(0 as libc::c_int);
        if sc.is_null() {
            ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
            *return_token = NO_TOKEN_SEND
        } else {
            ret =
                get_negotiable_mechs(minor_status, sc, spcred,
                                     1 as libc::c_int);
            if ret != 0 as libc::c_int as libc::c_uint {
                *return_token = NO_TOKEN_SEND
            } else {
                /*
	 * Select the best match between the list of mechs
	 * that the initiator requested and the list that
	 * the acceptor will support.
	 */
                mech_wanted = negotiate_mech(sc, mechTypes, negState);
                if *negState == 2 as libc::c_int as libc::c_uint {
                    ret =
                        (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
                } else {
                    (*sc).internal_mech = mech_wanted;
                    (*sc).DER_mechTypes = der_mechTypes;
                    der_mechTypes.length = 0 as libc::c_int as size_t;
                    der_mechTypes.value = 0 as *mut libc::c_void;
                    if *negState == 3 as libc::c_int as libc::c_uint {
                        (*sc).mic_reqd = 1 as libc::c_int
                    }
                    *return_token = INIT_TOKEN_SEND;
                    (*sc).firstpass = 1 as libc::c_int;
                    *sc_out = sc;
                    sc = 0 as spnego_gss_ctx_id_t;
                    ret = 0 as libc::c_int as OM_uint32
                }
            }
        }
    }
    release_spnego_ctx(&mut sc);
    gss_release_oid_set(&mut tmpmin, &mut mechTypes);
    if der_mechTypes.length != 0 as libc::c_int as libc::c_ulong {
        gss_release_buffer(&mut tmpmin, &mut der_mechTypes);
    }
    return ret;
}
#[c2rust::src_loc = "1421:1"]
unsafe extern "C" fn acc_ctx_cont(mut minstat: *mut OM_uint32,
                                  mut buf: gss_buffer_t,
                                  mut sc: spnego_gss_ctx_id_t,
                                  mut responseToken: *mut gss_buffer_t,
                                  mut mechListMIC: *mut gss_buffer_t,
                                  mut negState: *mut OM_uint32,
                                  mut return_token: *mut send_token_flag)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut supportedMech: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut len: libc::c_uint = 0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bufstart: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    *negState = 2 as libc::c_int as OM_uint32;
    *minstat = 0 as libc::c_int as OM_uint32;
    supportedMech = 0 as gss_OID;
    *return_token = ERROR_TOKEN_SEND;
    *mechListMIC = 0 as gss_buffer_t;
    *responseToken = *mechListMIC;
    bufstart = (*buf).value as *mut libc::c_uchar;
    ptr = bufstart;
    if (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart) as
                                      libc::c_long as libc::c_ulong) ==
           0 as libc::c_int as libc::c_ulong ||
           (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart) as
                                          libc::c_long as libc::c_ulong) >
               2147483647 as libc::c_int as libc::c_ulong {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /*
	 * Attempt to work with old Sun SPNEGO.
	 */
    if *ptr as libc::c_int == 0x60 as libc::c_int {
        ret =
            g_verify_token_header(gss_mech_spnego, &mut len, &mut ptr,
                                  0 as libc::c_int,
                                  (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart)
                                                                 as
                                                                 libc::c_long
                                                                 as
                                                                 libc::c_ulong)
                                      as libc::c_uint) as OM_uint32;
        if ret != 0 {
            *minstat = ret;
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    if *ptr as libc::c_int != 0xa0 as libc::c_int | 0x1 as libc::c_int {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        get_negTokenResp(minstat, ptr,
                         (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart)
                                                        as libc::c_long as
                                                        libc::c_ulong) as
                             libc::c_uint, negState, &mut supportedMech,
                         responseToken, mechListMIC);
    if !(ret != 0 as libc::c_int as libc::c_uint) {
        if (*responseToken).is_null() && (*mechListMIC).is_null() {
            ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        } else if !supportedMech.is_null() {
            ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        } else {
            (*sc).firstpass = 0 as libc::c_int;
            *negState = 1 as libc::c_int as OM_uint32;
            *return_token = CONT_TOKEN_SEND
        }
    }
    if !supportedMech.is_null() {
        generic_gss_release_oid(&mut tmpmin, &mut supportedMech);
    }
    return ret;
}
/*
 * Verify that mech OID is either exactly the same as the negotiated
 * mech OID, or is a mech OID supported by the negotiated mech.  MS
 * implementations can list a most preferred mech using an incorrect
 * krb5 OID while emitting a krb5 initiator mech token having the
 * correct krb5 mech OID.
 */
#[c2rust::src_loc = "1495:1"]
unsafe extern "C" fn acc_ctx_vfy_oid(mut minor_status: *mut OM_uint32,
                                     mut sc: spnego_gss_ctx_id_t,
                                     mut mechoid: gss_OID,
                                     mut negState: *mut OM_uint32,
                                     mut tokflag: *mut send_token_flag)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut mech: gss_mechanism = 0 as gss_mechanism;
    let mut mech_set: gss_OID_set = 0 as gss_OID_set;
    let mut present: libc::c_int = 0 as libc::c_int;
    if (*(*sc).internal_mech).length == (*mechoid).length &&
           memcmp((*(*sc).internal_mech).elements, (*mechoid).elements,
                  (*(*sc).internal_mech).length as libc::c_ulong) ==
               0 as libc::c_int {
        return 0 as libc::c_int as OM_uint32
    }
    mech = gssint_get_mechanism((*sc).internal_mech as gss_const_OID);
    if mech.is_null() || (*mech).gss_indicate_mechs.is_none() {
        *minor_status = 0x20000004 as libc::c_int as OM_uint32;
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status);
        *negState = 2 as libc::c_int as OM_uint32;
        *tokflag = ERROR_TOKEN_SEND;
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        (*mech).gss_indicate_mechs.expect("non-null function pointer")(minor_status,
                                                                       &mut mech_set);
    if ret != 0 as libc::c_int as libc::c_uint {
        *tokflag = NO_TOKEN_SEND;
        *minor_status =
            gssint_mecherrmap_map(*minor_status, &mut (*mech).mech_type)
    } else {
        ret =
            gss_test_oid_set_member(minor_status, mechoid, mech_set,
                                    &mut present);
        if !(ret != 0 as libc::c_int as libc::c_uint) {
            if present == 0 {
                *minor_status = 0x20000004 as libc::c_int as OM_uint32;
                *minor_status = gssint_mecherrmap_map_errcode(*minor_status);
                *negState = 2 as libc::c_int as OM_uint32;
                *tokflag = ERROR_TOKEN_SEND;
                ret = (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
            }
        }
    }
    gss_release_oid_set(&mut tmpmin, &mut mech_set);
    return ret;
}
/*
 * Wrap call to gss_accept_sec_context() and update state
 * accordingly.
 */
#[c2rust::src_loc = "1542:1"]
unsafe extern "C" fn acc_ctx_call_acc(mut minor_status: *mut OM_uint32,
                                      mut sc: spnego_gss_ctx_id_t,
                                      mut spcred: spnego_gss_cred_id_t,
                                      mut mechtok_in: gss_buffer_t,
                                      mut mechtok_out: gss_buffer_t,
                                      mut time_rec: *mut OM_uint32,
                                      mut negState: *mut OM_uint32,
                                      mut tokflag: *mut send_token_flag)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut mechoid: gss_OID_desc =
        gss_OID_desc{length: 0,
                     elements:
                         0 as *const libc::c_void as *mut libc::c_void,};
    let mut mcred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut negoex: libc::c_int =
        gss_oid_equal((*sc).internal_mech as gss_const_OID,
                      &mut negoex_mech as *mut gss_OID_desc as gss_const_OID);
    if (*sc).ctx_handle.is_null() && negoex == 0 {
        /*
		 * mechoid is an alias; don't free it.
		 */
        ret = gssint_get_mech_type(&mut mechoid, mechtok_in);
        if ret != 0 as libc::c_int as libc::c_uint {
            *tokflag = NO_TOKEN_SEND;
            return ret
        }
        ret =
            acc_ctx_vfy_oid(minor_status, sc, &mut mechoid, negState,
                            tokflag);
        if ret != 0 as libc::c_int as libc::c_uint { return ret }
    }
    mcred =
        if spcred.is_null() { 0 as gss_cred_id_t } else { (*spcred).mcred };
    gss_release_name(&mut tmpmin, &mut (*sc).internal_name);
    gss_release_cred(&mut tmpmin, &mut (*sc).deleg_cred);
    if negoex != 0 {
        ret =
            negoex_accept(minor_status, sc, mcred, mechtok_in, mechtok_out,
                          time_rec)
    } else {
        ret =
            gss_accept_sec_context(minor_status, &mut (*sc).ctx_handle, mcred,
                                   mechtok_in, 0 as gss_channel_bindings_t,
                                   &mut (*sc).internal_name,
                                   &mut (*sc).actual_mech, mechtok_out,
                                   &mut (*sc).ctx_flags, time_rec,
                                   &mut (*sc).deleg_cred)
    }
    if ret == 0 as libc::c_int as libc::c_uint {
        (*sc).mech_complete = 1 as libc::c_int;
        if (*sc).mic_reqd == 0 ||
               (*sc).ctx_flags & 32 as libc::c_int as libc::c_uint == 0 {
            /* No MIC exchange required, so we're done. */
            *negState = 0 as libc::c_int as OM_uint32;
            ret = 0 as libc::c_int as OM_uint32
        } else {
            /* handle_mic will decide if we're done. */
            ret =
                ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int) as
                    OM_uint32
        }
    } else if ret !=
                  ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int)
                      as libc::c_uint {
        *negState = 2 as libc::c_int as OM_uint32;
        *tokflag = ERROR_TOKEN_SEND
    }
    return ret;
}
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "1617:1"]
pub unsafe extern "C" fn spnego_gss_accept_sec_context(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut context_handle:
                                                           *mut gss_ctx_id_t,
                                                       mut verifier_cred_handle:
                                                           gss_cred_id_t,
                                                       mut input_token:
                                                           gss_buffer_t,
                                                       mut input_chan_bindings:
                                                           gss_channel_bindings_t,
                                                       mut src_name:
                                                           *mut gss_name_t,
                                                       mut mech_type:
                                                           *mut gss_OID,
                                                       mut output_token:
                                                           gss_buffer_t,
                                                       mut ret_flags:
                                                           *mut OM_uint32,
                                                       mut time_rec:
                                                           *mut OM_uint32,
                                                       mut delegated_cred_handle:
                                                           *mut gss_cred_id_t)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut ret: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut negState: OM_uint32 = 0;
    let mut return_token: send_token_flag = NO_TOKEN_SEND;
    let mut mechtok_in: gss_buffer_t = 0 as *mut gss_buffer_desc_struct;
    let mut mic_in: gss_buffer_t = 0 as *mut gss_buffer_desc_struct;
    let mut mic_out: gss_buffer_t = 0 as *mut gss_buffer_desc_struct;
    let mut mechtok_out: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut sc: spnego_gss_ctx_id_t = 0 as spnego_gss_ctx_id_t;
    let mut spcred: spnego_gss_cred_id_t = 0 as spnego_gss_cred_id_t;
    let mut sendTokenInit: libc::c_int = 0 as libc::c_int;
    let mut tmpret: libc::c_int = 0;
    mic_out = 0 as gss_buffer_t;
    mic_in = mic_out;
    mechtok_in = mic_in;
    /*
	 * This function works in three steps:
	 *
	 *   1. Perform mechanism negotiation.
	 *   2. Invoke the negotiated mech's gss_accept_sec_context function
	 *      and examine the results.
	 *   3. Process or generate MICs if necessary.
	 *
	 * Step one determines whether the negotiation requires a MIC exchange,
	 * while steps two and three share responsibility for determining when
	 * the exchange is complete.  If the selected mech completes in this
	 * call and no MIC exchange is expected, then step 2 will decide.  If a
	 * MIC exchange is expected, then step 3 will decide.  If an error
	 * occurs in any step, the exchange will be aborted, possibly with an
	 * error token.
	 *
	 * negState determines the state of the negotiation, and is
	 * communicated to the acceptor if a continuing token is sent.
	 * return_token is used to indicate what type of token, if any, should
	 * be generated.
	 */
    /* Validate arguments. */
    if !minor_status.is_null() {
        *minor_status = 0 as libc::c_int as OM_uint32
    }
    if !output_token.is_null() {
        (*output_token).length = 0 as libc::c_int as size_t;
        (*output_token).value = 0 as *mut libc::c_void
    }
    if !src_name.is_null() { *src_name = 0 as gss_name_t }
    if !mech_type.is_null() { *mech_type = 0 as gss_OID }
    if !time_rec.is_null() { *time_rec = 0 as libc::c_int as OM_uint32 }
    if !ret_flags.is_null() { *ret_flags = 0 as libc::c_int as OM_uint32 }
    if !delegated_cred_handle.is_null() {
        *delegated_cred_handle = 0 as gss_cred_id_t
    }
    if minor_status.is_null() || output_token.is_null() ||
           context_handle.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if input_token.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    /* Step 1: Perform mechanism negotiation. */
    sc = *context_handle as spnego_gss_ctx_id_t;
    spcred = verifier_cred_handle as spnego_gss_cred_id_t;
    if sc.is_null() &&
           (*input_token).length == 0 as libc::c_int as libc::c_ulong {
        /* Process a request for NegHints. */
        ret =
            acc_ctx_hints(minor_status, spcred, &mut mic_out, &mut negState,
                          &mut return_token, &mut sc);
        if ret != 0 as libc::c_int as libc::c_uint {
            current_block = 15072255976349127597;
        } else {
            *context_handle = sc as gss_ctx_id_t;
            sendTokenInit = 1 as libc::c_int;
            ret =
                ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int) as
                    OM_uint32;
            current_block = 3222590281903869779;
        }
    } else if sc.is_null() || (*sc).internal_mech.is_null() {
        if !sc.is_null() {
            /* Discard the context from the NegHints request. */
            release_spnego_ctx(&mut sc);
            *context_handle = 0 as gss_ctx_id_t
        }
        /* Process an initial token; can set negState to
		 * REQUEST_MIC. */
        ret =
            acc_ctx_new(minor_status, input_token, spcred, &mut mechtok_in,
                        &mut mic_in, &mut negState, &mut return_token,
                        &mut sc);
        if ret != 0 as libc::c_int as libc::c_uint {
            current_block = 15072255976349127597;
        } else {
            *context_handle = sc as gss_ctx_id_t;
            ret =
                ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int) as
                    OM_uint32;
            current_block = 3222590281903869779;
        }
    } else {
        /* Process a response token.  Can set negState to
		 * ACCEPT_INCOMPLETE. */
        ret =
            acc_ctx_cont(minor_status, input_token, sc, &mut mechtok_in,
                         &mut mic_in, &mut negState, &mut return_token);
        if ret != 0 as libc::c_int as libc::c_uint {
            current_block = 15072255976349127597;
        } else {
            ret =
                ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int) as
                    OM_uint32;
            current_block = 3222590281903869779;
        }
    }
    match current_block {
        3222590281903869779 => {
            /* Step 2: invoke the negotiated mechanism's gss_accept_sec_context
	 * function. */
	/*
	 * Handle mechtok_in and mic_in only if they are
	 * present in input_token.  If neither is present, whether
	 * this is an error depends on whether this is the first
	 * round-trip.  RET is set to a default value according to
	 * whether it is the first round-trip.
	 */
            if negState != 3 as libc::c_int as libc::c_uint &&
                   !mechtok_in.is_null() {
                ret =
                    acc_ctx_call_acc(minor_status, sc, spcred, mechtok_in,
                                     &mut mechtok_out, time_rec,
                                     &mut negState, &mut return_token)
            }
            /* Step 3: process or generate the MIC, if the negotiated mech is
	 * complete and supports MICs. */
            if !(ret != 0 as libc::c_int as libc::c_uint &&
                     ret !=
                         ((1 as libc::c_int) <<
                              0 as libc::c_int + 0 as libc::c_int) as
                             libc::c_uint) && (*sc).mech_complete != 0 &&
                   (*sc).ctx_flags & 32 as libc::c_int as libc::c_uint != 0 {
                ret =
                    handle_mic(minor_status, mic_in,
                               (mechtok_out.length !=
                                    0 as libc::c_int as libc::c_ulong) as
                                   libc::c_int, sc, &mut mic_out,
                               &mut negState, &mut return_token)
            }
            if !(ret != 0 as libc::c_int as libc::c_uint &&
                     ret !=
                         ((1 as libc::c_int) <<
                              0 as libc::c_int + 0 as libc::c_int) as
                             libc::c_uint) && !ret_flags.is_null() {
                *ret_flags =
                    (*sc).ctx_flags & !(128 as libc::c_int) as libc::c_uint
            }
        }
        _ => { }
    }
    if return_token as libc::c_uint ==
           INIT_TOKEN_SEND as libc::c_int as libc::c_uint &&
           sendTokenInit != 0 {
        if !sc.is_null() {
        } else {
            __assert_fail(b"sc != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"spnego_mech.c\x00" as *const u8 as
                              *const libc::c_char,
                          1757 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 204],
                                                    &[libc::c_char; 204]>(b"OM_uint32 spnego_gss_accept_sec_context(OM_uint32 *, gss_ctx_id_t *, gss_cred_id_t, gss_buffer_t, gss_channel_bindings_t, gss_name_t *, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *, gss_cred_id_t *)\x00")).as_ptr());
        }
        tmpret =
            make_spnego_tokenInit_msg(sc, 1 as libc::c_int, mic_out,
                                      0 as libc::c_int as OM_uint32,
                                      0 as gss_buffer_t, return_token,
                                      output_token);
        if tmpret < 0 as libc::c_int {
            ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    } else if return_token as libc::c_uint !=
                  NO_TOKEN_SEND as libc::c_int as libc::c_uint &&
                  return_token as libc::c_uint !=
                      CHECK_MIC as libc::c_int as libc::c_uint {
        tmpret =
            make_spnego_tokenTarg_msg(negState,
                                      if !sc.is_null() {
                                          (*sc).internal_mech
                                      } else { 0 as gss_OID },
                                      &mut mechtok_out, mic_out, return_token,
                                      output_token);
        if tmpret < 0 as libc::c_int {
            ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    if ret == 0 as libc::c_int as libc::c_uint {
        (*sc).opened = 1 as libc::c_int;
        if !(*sc).internal_name.is_null() && !src_name.is_null() {
            *src_name = (*sc).internal_name;
            (*sc).internal_name = 0 as gss_name_t
        }
        if !mech_type.is_null() { *mech_type = (*sc).actual_mech }
        /* Get an updated lifetime if we didn't call into the mech. */
        if !time_rec.is_null() &&
               *time_rec == 0 as libc::c_int as libc::c_uint {
            gss_context_time(&mut tmpmin, (*sc).ctx_handle, time_rec);
        }
        if !delegated_cred_handle.is_null() {
            *delegated_cred_handle = (*sc).deleg_cred;
            (*sc).deleg_cred = 0 as gss_cred_id_t
        }
    } else if ret !=
                  ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int)
                      as libc::c_uint {
        if !sc.is_null() {
            gss_delete_sec_context(&mut tmpmin, &mut (*sc).ctx_handle,
                                   0 as gss_buffer_t);
            release_spnego_ctx(&mut sc);
        }
        *context_handle = 0 as gss_ctx_id_t
    }
    gss_release_buffer(&mut tmpmin, &mut mechtok_out);
    if !mechtok_in.is_null() {
        gss_release_buffer(&mut tmpmin, mechtok_in);
        free(mechtok_in as *mut libc::c_void);
    }
    if !mic_in.is_null() {
        gss_release_buffer(&mut tmpmin, mic_in);
        free(mic_in as *mut libc::c_void);
    }
    if !mic_out.is_null() {
        gss_release_buffer(&mut tmpmin, mic_out);
        free(mic_out as *mut libc::c_void);
    }
    return ret;
}
#[c2rust::src_loc = "1820:3"]
static mut msg_table: [C2RustUnnamed_1; 19] =
    [{
         let mut init =
             C2RustUnnamed_1{status: 0x20000001 as libc::c_int as OM_uint32,
                             msg:
                                 b"SPNEGO cannot find mechanisms to negotiate\x00"
                                     as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000002 as libc::c_int as OM_uint32,
                             msg:
                                 b"SPNEGO failed to acquire creds\x00" as
                                     *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000003 as libc::c_int as OM_uint32,
                             msg:
                                 b"SPNEGO acceptor did not select a mechanism\x00"
                                     as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000004 as libc::c_int as OM_uint32,
                             msg:
                                 b"SPNEGO failed to negotiate a mechanism\x00"
                                     as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000005 as libc::c_int as OM_uint32,
                             msg:
                                 b"SPNEGO acceptor did not return a valid token\x00"
                                     as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000006 as libc::c_int as OM_uint32,
                             msg:
                                 b"Invalid NegoEx signature\x00" as *const u8
                                     as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000007 as libc::c_int as OM_uint32,
                             msg:
                                 b"Invalid NegoEx message type\x00" as
                                     *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000008 as libc::c_int as OM_uint32,
                             msg:
                                 b"Invalid NegoEx message size\x00" as
                                     *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000009 as libc::c_int as OM_uint32,
                             msg:
                                 b"Invalid NegoEx conversation ID\x00" as
                                     *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000010 as libc::c_int as OM_uint32,
                             msg:
                                 b"NegoEx authentication scheme not found\x00"
                                     as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000011 as libc::c_int as OM_uint32,
                             msg:
                                 b"Missing NegoEx negotiate message\x00" as
                                     *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000012 as libc::c_int as OM_uint32,
                             msg:
                                 b"Missing NegoEx authentication protocol request message\x00"
                                     as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000013 as libc::c_int as OM_uint32,
                             msg:
                                 b"No mutually supported NegoEx authentication schemes\x00"
                                     as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000014 as libc::c_int as OM_uint32,
                             msg:
                                 b"No NegoEx verify key\x00" as *const u8 as
                                     *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000015 as libc::c_int as OM_uint32,
                             msg:
                                 b"Unknown NegoEx checksum scheme\x00" as
                                     *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000016 as libc::c_int as OM_uint32,
                             msg:
                                 b"Invalid NegoEx checksum\x00" as *const u8
                                     as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000017 as libc::c_int as OM_uint32,
                             msg:
                                 b"Unsupported critical NegoEx extension\x00"
                                     as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000018 as libc::c_int as OM_uint32,
                             msg:
                                 b"Unsupported NegoEx version\x00" as
                                     *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_1{status: 0x20000019 as libc::c_int as OM_uint32,
                             msg:
                                 b"NegoEx message out of sequence\x00" as
                                     *const u8 as *const libc::c_char,};
         init
     }];
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "1862:1"]
pub unsafe extern "C" fn spnego_gss_display_status(mut minor_status:
                                                       *mut OM_uint32,
                                                   mut status_value:
                                                       OM_uint32,
                                                   mut status_type:
                                                       libc::c_int,
                                                   mut mech_type: gss_OID,
                                                   mut message_context:
                                                       *mut OM_uint32,
                                                   mut status_string:
                                                       gss_buffer_t)
 -> OM_uint32 {
    let mut maj: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0;
    let mut ret: libc::c_int = 0;
    *message_context = 0 as libc::c_int as OM_uint32;
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[C2RustUnnamed_1; 19]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_1>()
                                                   as libc::c_ulong) {
        if status_value == msg_table[i as usize].status {
            msg =
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         msg_table[i as usize].msg);
            *status_string = make_err_msg(msg);
            return 0 as libc::c_int as OM_uint32
        }
        i = i.wrapping_add(1)
    }
    /* Not one of our minor codes; might be from a mech.  Call back
	 * to gss_display_status, but first check for recursion. */
    if !krb5int_getspecific(K5_KEY_GSS_SPNEGO_STATUS).is_null() {
        /* Perhaps we returned a com_err code like ENOMEM. */
        let mut err: *const libc::c_char =
            error_message(status_value as errcode_t);
        *status_string = make_err_msg(err);
        return 0 as libc::c_int as OM_uint32
    }
    /* Set a non-null pointer value; doesn't matter which one. */
    ret =
        krb5int_setspecific(K5_KEY_GSS_SPNEGO_STATUS,
                            &mut ret as *mut libc::c_int as
                                *mut libc::c_void);
    if ret != 0 as libc::c_int {
        *minor_status = ret as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    maj =
        gss_display_status(minor_status, status_value, status_type, mech_type,
                           message_context, status_string);
    /* This is unlikely to fail; not much we can do if it does. */
    krb5int_setspecific(K5_KEY_GSS_SPNEGO_STATUS, 0 as *mut libc::c_void);
    return maj;
}
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "1911:1"]
pub unsafe extern "C" fn spnego_gss_import_name(mut minor_status:
                                                    *mut OM_uint32,
                                                mut input_name_buffer:
                                                    gss_buffer_t,
                                                mut input_name_type: gss_OID,
                                                mut output_name:
                                                    *mut gss_name_t)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    status =
        gss_import_name(minor_status, input_name_buffer, input_name_type,
                        output_name);
    return status;
}
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "1930:1"]
pub unsafe extern "C" fn spnego_gss_release_name(mut minor_status:
                                                     *mut OM_uint32,
                                                 mut input_name:
                                                     *mut gss_name_t)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    status = gss_release_name(minor_status, input_name);
    return status;
}
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "1946:1"]
pub unsafe extern "C" fn spnego_gss_duplicate_name(mut minor_status:
                                                       *mut OM_uint32,
                                                   input_name: gss_name_t,
                                                   mut output_name:
                                                       *mut gss_name_t)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    status = gss_duplicate_name(minor_status, input_name, output_name);
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "1962:1"]
pub unsafe extern "C" fn spnego_gss_inquire_cred(mut minor_status:
                                                     *mut OM_uint32,
                                                 mut cred_handle:
                                                     gss_cred_id_t,
                                                 mut name: *mut gss_name_t,
                                                 mut lifetime: *mut OM_uint32,
                                                 mut cred_usage:
                                                     *mut libc::c_int,
                                                 mut mechanisms:
                                                     *mut gss_OID_set)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut spcred: spnego_gss_cred_id_t = 0 as spnego_gss_cred_id_t;
    let mut creds: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut tmp_minor_status: OM_uint32 = 0;
    let mut initiator_lifetime: OM_uint32 = 0;
    let mut acceptor_lifetime: OM_uint32 = 0;
    /*
	 * To avoid infinite recursion, if GSS_C_NO_CREDENTIAL is
	 * supplied we call gss_inquire_cred_by_mech() on the
	 * first non-SPNEGO mechanism.
	 */
    spcred = cred_handle as spnego_gss_cred_id_t;
    if spcred.is_null() {
        status =
            get_available_mechs(minor_status, 0 as gss_name_t,
                                0 as libc::c_int,
                                0 as gss_const_key_value_set_t, &mut creds,
                                mechanisms, 0 as *mut OM_uint32);
        if status != 0 as libc::c_int as libc::c_uint { return status }
        if (**mechanisms).count == 0 as libc::c_int as libc::c_ulong {
            gss_release_cred(&mut tmp_minor_status, &mut creds);
            gss_release_oid_set(&mut tmp_minor_status, mechanisms);
            return (10 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        if !(**mechanisms).elements.is_null() {
        } else {
            __assert_fail(b"(*mechanisms)->elements != NULL\x00" as *const u8
                              as *const libc::c_char,
                          b"spnego_mech.c\x00" as *const u8 as
                              *const libc::c_char,
                          2004 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 111],
                                                    &[libc::c_char; 111]>(b"OM_uint32 spnego_gss_inquire_cred(OM_uint32 *, gss_cred_id_t, gss_name_t *, OM_uint32 *, int *, gss_OID_set *)\x00")).as_ptr());
        }
        status =
            gss_inquire_cred_by_mech(minor_status, creds,
                                     &mut *(**mechanisms).elements.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize),
                                     name, &mut initiator_lifetime,
                                     &mut acceptor_lifetime, cred_usage);
        if status != 0 as libc::c_int as libc::c_uint {
            gss_release_cred(&mut tmp_minor_status, &mut creds);
            return status
        }
        if !lifetime.is_null() {
            *lifetime =
                if *cred_usage == 2 as libc::c_int {
                    acceptor_lifetime
                } else { initiator_lifetime }
        }
        gss_release_cred(&mut tmp_minor_status, &mut creds);
    } else {
        status =
            gss_inquire_cred(minor_status, (*spcred).mcred, name, lifetime,
                             cred_usage, mechanisms)
    }
    return status;
}
/* LEAN_CLIENT */
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "2036:1"]
pub unsafe extern "C" fn spnego_gss_compare_name(mut minor_status:
                                                     *mut OM_uint32,
                                                 name1: gss_name_t,
                                                 name2: gss_name_t,
                                                 mut name_equal:
                                                     *mut libc::c_int)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0 as libc::c_int as OM_uint32;
    status = gss_compare_name(minor_status, name1, name2, name_equal);
    return status;
}
/*ARGSUSED*/
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "2054:1"]
pub unsafe extern "C" fn spnego_gss_display_name(mut minor_status:
                                                     *mut OM_uint32,
                                                 mut input_name: gss_name_t,
                                                 mut output_name_buffer:
                                                     gss_buffer_t,
                                                 mut output_name_type:
                                                     *mut gss_OID)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0 as libc::c_int as OM_uint32;
    status =
        gss_display_name(minor_status, input_name, output_name_buffer,
                         output_name_type);
    return status;
}
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "2073:1"]
pub unsafe extern "C" fn spnego_gss_inquire_names_for_mech(mut minor_status:
                                                               *mut OM_uint32,
                                                           mut mechanism:
                                                               gss_OID,
                                                           mut name_types:
                                                               *mut gss_OID_set)
 -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    /*
	 * We only know how to handle our own mechanism.
	 */
    if !mechanism.is_null() &&
           !((*gss_mech_spnego).length == (*mechanism).length &&
                 memcmp((*gss_mech_spnego).elements, (*mechanism).elements,
                        (*gss_mech_spnego).length as libc::c_ulong) ==
                     0 as libc::c_int) {
        *minor_status = 0 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    major = gss_create_empty_oid_set(minor_status, name_types);
    if major == 0 as libc::c_int as libc::c_uint {
        /* Now add our members. */
        major =
            gss_add_oid_set_member(minor_status, GSS_C_NT_USER_NAME,
                                   name_types);
        if major == 0 as libc::c_int as libc::c_uint &&
               {
                   major =
                       gss_add_oid_set_member(minor_status,
                                              GSS_C_NT_MACHINE_UID_NAME,
                                              name_types);
                   (major) == 0 as libc::c_int as libc::c_uint
               } &&
               {
                   major =
                       gss_add_oid_set_member(minor_status,
                                              GSS_C_NT_STRING_UID_NAME,
                                              name_types);
                   (major) == 0 as libc::c_int as libc::c_uint
               } {
            major =
                gss_add_oid_set_member(minor_status,
                                       GSS_C_NT_HOSTBASED_SERVICE, name_types)
        }
        if major != 0 as libc::c_int as libc::c_uint {
            gss_release_oid_set(&mut minor, name_types);
        }
    }
    return major;
}
#[no_mangle]
#[c2rust::src_loc = "2116:1"]
pub unsafe extern "C" fn spnego_gss_unwrap(mut minor_status: *mut OM_uint32,
                                           mut context_handle: gss_ctx_id_t,
                                           mut input_message_buffer:
                                               gss_buffer_t,
                                           mut output_message_buffer:
                                               gss_buffer_t,
                                           mut conf_state: *mut libc::c_int,
                                           mut qop_state: *mut gss_qop_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_unwrap(minor_status, (*sc).ctx_handle, input_message_buffer,
                   output_message_buffer, conf_state, qop_state);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2141:1"]
pub unsafe extern "C" fn spnego_gss_wrap(mut minor_status: *mut OM_uint32,
                                         mut context_handle: gss_ctx_id_t,
                                         mut conf_req_flag: libc::c_int,
                                         mut qop_req: gss_qop_t,
                                         mut input_message_buffer:
                                             gss_buffer_t,
                                         mut conf_state: *mut libc::c_int,
                                         mut output_message_buffer:
                                             gss_buffer_t) -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_wrap(minor_status, (*sc).ctx_handle, conf_req_flag, qop_req,
                 input_message_buffer, conf_state, output_message_buffer);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2168:1"]
pub unsafe extern "C" fn spnego_gss_process_context_token(mut minor_status:
                                                              *mut OM_uint32,
                                                          context_handle:
                                                              gss_ctx_id_t,
                                                          token_buffer:
                                                              gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    /* SPNEGO doesn't have its own context tokens. */
    if (*sc).opened == 0 {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_process_context_token(minor_status, (*sc).ctx_handle,
                                  token_buffer);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2188:1"]
pub unsafe extern "C" fn spnego_gss_delete_sec_context(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut context_handle:
                                                           *mut gss_ctx_id_t,
                                                       mut output_token:
                                                           gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut ctx: *mut spnego_gss_ctx_id_t =
        context_handle as *mut spnego_gss_ctx_id_t;
    *minor_status = 0 as libc::c_int as OM_uint32;
    if context_handle.is_null() {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if (*ctx).is_null() { return 0 as libc::c_int as OM_uint32 }
    gss_delete_sec_context(minor_status, &mut (**ctx).ctx_handle,
                           output_token);
    release_spnego_ctx(ctx);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2213:1"]
pub unsafe extern "C" fn spnego_gss_context_time(mut minor_status:
                                                     *mut OM_uint32,
                                                 context_handle: gss_ctx_id_t,
                                                 mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret = gss_context_time(minor_status, (*sc).ctx_handle, time_rec);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2231:1"]
pub unsafe extern "C" fn spnego_gss_export_sec_context(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut context_handle:
                                                           *mut gss_ctx_id_t,
                                                       mut interprocess_token:
                                                           gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t =
        *(context_handle as *mut spnego_gss_ctx_id_t);
    /* We don't currently support exporting partially established
	 * contexts. */
    if (*sc).opened == 0 {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_export_sec_context(minor_status, &mut (*sc).ctx_handle,
                               interprocess_token);
    if (*sc).ctx_handle.is_null() {
        release_spnego_ctx(&mut sc);
        *context_handle = 0 as gss_ctx_id_t
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2255:1"]
pub unsafe extern "C" fn spnego_gss_import_sec_context(mut minor_status:
                                                           *mut OM_uint32,
                                                       interprocess_token:
                                                           gss_buffer_t,
                                                       mut context_handle:
                                                           *mut gss_ctx_id_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut mctx: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut sc: spnego_gss_ctx_id_t = 0 as *mut spnego_ctx_st;
    let mut initiate: libc::c_int = 0;
    let mut opened: libc::c_int = 0;
    ret = gss_import_sec_context(minor_status, interprocess_token, &mut mctx);
    if ret != 0 as libc::c_int as libc::c_uint { return ret }
    ret =
        gss_inquire_context(&mut tmpmin, mctx, 0 as *mut gss_name_t,
                            0 as *mut gss_name_t, 0 as *mut OM_uint32,
                            0 as *mut gss_OID, 0 as *mut OM_uint32,
                            &mut initiate, &mut opened);
    if ret != 0 as libc::c_int as libc::c_uint || opened == 0 {
        /* We don't currently support importing partially established
		 * contexts. */
        gss_delete_sec_context(&mut tmpmin, &mut mctx, 0 as gss_buffer_t);
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    sc = create_spnego_ctx(initiate);
    if sc.is_null() {
        gss_delete_sec_context(&mut tmpmin, &mut mctx, 0 as gss_buffer_t);
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    (*sc).ctx_handle = mctx;
    (*sc).opened = 1 as libc::c_int;
    *context_handle = sc as gss_ctx_id_t;
    return 0 as libc::c_int as OM_uint32;
}
/* LEAN_CLIENT */
/* LEAN_CLIENT */
#[no_mangle]
#[c2rust::src_loc = "2291:1"]
pub unsafe extern "C" fn spnego_gss_inquire_context(mut minor_status:
                                                        *mut OM_uint32,
                                                    context_handle:
                                                        gss_ctx_id_t,
                                                    mut src_name:
                                                        *mut gss_name_t,
                                                    mut targ_name:
                                                        *mut gss_name_t,
                                                    mut lifetime_rec:
                                                        *mut OM_uint32,
                                                    mut mech_type:
                                                        *mut gss_OID,
                                                    mut ctx_flags:
                                                        *mut OM_uint32,
                                                    mut locally_initiated:
                                                        *mut libc::c_int,
                                                    mut opened:
                                                        *mut libc::c_int)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if !src_name.is_null() { *src_name = 0 as gss_name_t }
    if !targ_name.is_null() { *targ_name = 0 as gss_name_t }
    if !lifetime_rec.is_null() {
        *lifetime_rec = 0 as libc::c_int as OM_uint32
    }
    if !mech_type.is_null() { *mech_type = gss_mech_spnego as gss_OID }
    if !ctx_flags.is_null() { *ctx_flags = 0 as libc::c_int as OM_uint32 }
    if !locally_initiated.is_null() { *locally_initiated = (*sc).initiate }
    if !opened.is_null() { *opened = (*sc).opened }
    if !(*sc).ctx_handle.is_null() {
        ret =
            gss_inquire_context(minor_status, (*sc).ctx_handle, src_name,
                                targ_name, lifetime_rec, mech_type, ctx_flags,
                                0 as *mut libc::c_int, 0 as *mut libc::c_int)
    }
    if (*sc).opened == 0 {
        /*
		 * We are still doing SPNEGO negotiation, so report SPNEGO as
		 * the OID.  After negotiation is complete we will report the
		 * underlying mechanism OID.
		 */
        if !mech_type.is_null() { *mech_type = gss_mech_spnego as gss_OID }
        /*
		 * Remove flags we don't support with partially-established
		 * contexts.  (Change this to keep GSS_C_TRANS_FLAG if we add
		 * support for exporting partial SPNEGO contexts.)
		 */
        if !ctx_flags.is_null() {
            *ctx_flags &= !(128 as libc::c_int) as libc::c_uint;
            *ctx_flags &= !(256 as libc::c_int) as libc::c_uint
        }
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2350:1"]
pub unsafe extern "C" fn spnego_gss_wrap_size_limit(mut minor_status:
                                                        *mut OM_uint32,
                                                    context_handle:
                                                        gss_ctx_id_t,
                                                    mut conf_req_flag:
                                                        libc::c_int,
                                                    mut qop_req: gss_qop_t,
                                                    mut req_output_size:
                                                        OM_uint32,
                                                    mut max_input_size:
                                                        *mut OM_uint32)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_wrap_size_limit(minor_status, (*sc).ctx_handle, conf_req_flag,
                            qop_req, req_output_size, max_input_size);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2374:1"]
pub unsafe extern "C" fn spnego_gss_get_mic(mut minor_status: *mut OM_uint32,
                                            context_handle: gss_ctx_id_t,
                                            mut qop_req: gss_qop_t,
                                            message_buffer: gss_buffer_t,
                                            mut message_token: gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_get_mic(minor_status, (*sc).ctx_handle, qop_req, message_buffer,
                    message_token);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2396:1"]
pub unsafe extern "C" fn spnego_gss_verify_mic(mut minor_status:
                                                   *mut OM_uint32,
                                               context_handle: gss_ctx_id_t,
                                               msg_buffer: gss_buffer_t,
                                               token_buffer: gss_buffer_t,
                                               mut qop_state: *mut gss_qop_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_verify_mic(minor_status, (*sc).ctx_handle, msg_buffer,
                       token_buffer, qop_state);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2418:1"]
pub unsafe extern "C" fn spnego_gss_inquire_sec_context_by_oid(mut minor_status:
                                                                   *mut OM_uint32,
                                                               context_handle:
                                                                   gss_ctx_id_t,
                                                               desired_object:
                                                                   gss_OID,
                                                               mut data_set:
                                                                   *mut gss_buffer_set_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    /* There are no SPNEGO-specific OIDs for this function. */
    if (*sc).ctx_handle.is_null() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_inquire_sec_context_by_oid(minor_status, (*sc).ctx_handle,
                                       desired_object, data_set);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2439:1"]
pub unsafe extern "C" fn spnego_gss_inquire_cred_by_oid(mut minor_status:
                                                            *mut OM_uint32,
                                                        cred_handle:
                                                            gss_cred_id_t,
                                                        desired_object:
                                                            gss_OID,
                                                        mut data_set:
                                                            *mut gss_buffer_set_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut spcred: spnego_gss_cred_id_t =
        cred_handle as spnego_gss_cred_id_t;
    let mut mcred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    mcred =
        if spcred.is_null() { 0 as gss_cred_id_t } else { (*spcred).mcred };
    ret =
        gss_inquire_cred_by_oid(minor_status, mcred, desired_object,
                                data_set);
    return ret;
}
#[c2rust::src_loc = "2460:27"]
static mut no_ci_flags_oid: [gss_OID_desc; 1] =
    [{
         let mut init =
             gss_OID_desc_struct{length: 6 as libc::c_int as OM_uint32,
                                 elements:
                                     b"*\x85p+\r\x1d\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_void,};
         init
     }];
#[no_mangle]
#[c2rust::src_loc = "2464:1"]
pub unsafe extern "C" fn spnego_gss_set_cred_option(mut minor_status:
                                                        *mut OM_uint32,
                                                    mut cred_handle:
                                                        *mut gss_cred_id_t,
                                                    desired_object: gss_OID,
                                                    value: gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut tmp_minor_status: OM_uint32 = 0;
    let mut spcred: spnego_gss_cred_id_t =
        *cred_handle as spnego_gss_cred_id_t;
    let mut mcred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    mcred =
        if spcred.is_null() { 0 as gss_cred_id_t } else { (*spcred).mcred };
    ret =
        gss_set_cred_option(minor_status, &mut mcred, desired_object, value);
    if ret == 0 as libc::c_int as libc::c_uint && spcred.is_null() {
        /*
		 * If the mechanism allocated a new credential handle, then
		 * we need to wrap it up in an SPNEGO credential handle.
		 */
        ret = create_spnego_cred(minor_status, mcred, &mut spcred);
        if ret != 0 as libc::c_int as libc::c_uint {
            gss_release_cred(&mut tmp_minor_status, &mut mcred);
            return ret
        }
        *cred_handle = spcred as gss_cred_id_t
    }
    if ret != 0 as libc::c_int as libc::c_uint { return ret }
    /* Recognize KRB5_NO_CI_FLAGS_X_OID and avoid asking for integrity. */
    if (*desired_object).length == (*no_ci_flags_oid.as_ptr()).length &&
           memcmp((*desired_object).elements,
                  (*no_ci_flags_oid.as_ptr()).elements,
                  (*desired_object).length as libc::c_ulong) ==
               0 as libc::c_int {
        (*spcred).no_ask_integ = 1 as libc::c_int
    }
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "2505:1"]
pub unsafe extern "C" fn spnego_gss_set_sec_context_option(mut minor_status:
                                                               *mut OM_uint32,
                                                           mut context_handle:
                                                               *mut gss_ctx_id_t,
                                                           desired_object:
                                                               gss_OID,
                                                           value:
                                                               gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = *context_handle as spnego_gss_ctx_id_t;
    /* There are no SPNEGO-specific OIDs for this function, and we cannot
	 * construct an empty SPNEGO context with it. */
    if sc.is_null() || (*sc).ctx_handle.is_null() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_set_sec_context_option(minor_status, &mut (*sc).ctx_handle,
                                   desired_object, value);
    return ret;
}
/* _GSS_STATIC_LINK */
#[no_mangle]
#[c2rust::src_loc = "2527:1"]
pub unsafe extern "C" fn spnego_gss_wrap_aead(mut minor_status:
                                                  *mut OM_uint32,
                                              mut context_handle:
                                                  gss_ctx_id_t,
                                              mut conf_req_flag: libc::c_int,
                                              mut qop_req: gss_qop_t,
                                              mut input_assoc_buffer:
                                                  gss_buffer_t,
                                              mut input_payload_buffer:
                                                  gss_buffer_t,
                                              mut conf_state:
                                                  *mut libc::c_int,
                                              mut output_message_buffer:
                                                  gss_buffer_t) -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_wrap_aead(minor_status, (*sc).ctx_handle, conf_req_flag, qop_req,
                      input_assoc_buffer, input_payload_buffer, conf_state,
                      output_message_buffer);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2555:1"]
pub unsafe extern "C" fn spnego_gss_unwrap_aead(mut minor_status:
                                                    *mut OM_uint32,
                                                mut context_handle:
                                                    gss_ctx_id_t,
                                                mut input_message_buffer:
                                                    gss_buffer_t,
                                                mut input_assoc_buffer:
                                                    gss_buffer_t,
                                                mut output_payload_buffer:
                                                    gss_buffer_t,
                                                mut conf_state:
                                                    *mut libc::c_int,
                                                mut qop_state: *mut gss_qop_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_unwrap_aead(minor_status, (*sc).ctx_handle, input_message_buffer,
                        input_assoc_buffer, output_payload_buffer, conf_state,
                        qop_state);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2580:1"]
pub unsafe extern "C" fn spnego_gss_wrap_iov(mut minor_status: *mut OM_uint32,
                                             mut context_handle: gss_ctx_id_t,
                                             mut conf_req_flag: libc::c_int,
                                             mut qop_req: gss_qop_t,
                                             mut conf_state: *mut libc::c_int,
                                             mut iov:
                                                 *mut gss_iov_buffer_desc,
                                             mut iov_count: libc::c_int)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_wrap_iov(minor_status, (*sc).ctx_handle, conf_req_flag, qop_req,
                     conf_state, iov, iov_count);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2605:1"]
pub unsafe extern "C" fn spnego_gss_unwrap_iov(mut minor_status:
                                                   *mut OM_uint32,
                                               mut context_handle:
                                                   gss_ctx_id_t,
                                               mut conf_state:
                                                   *mut libc::c_int,
                                               mut qop_state: *mut gss_qop_t,
                                               mut iov:
                                                   *mut gss_iov_buffer_desc,
                                               mut iov_count: libc::c_int)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_unwrap_iov(minor_status, (*sc).ctx_handle, conf_state, qop_state,
                       iov, iov_count);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2628:1"]
pub unsafe extern "C" fn spnego_gss_wrap_iov_length(mut minor_status:
                                                        *mut OM_uint32,
                                                    mut context_handle:
                                                        gss_ctx_id_t,
                                                    mut conf_req_flag:
                                                        libc::c_int,
                                                    mut qop_req: gss_qop_t,
                                                    mut conf_state:
                                                        *mut libc::c_int,
                                                    mut iov:
                                                        *mut gss_iov_buffer_desc,
                                                    mut iov_count:
                                                        libc::c_int)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_wrap_iov_length(minor_status, (*sc).ctx_handle, conf_req_flag,
                            qop_req, conf_state, iov, iov_count);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2654:1"]
pub unsafe extern "C" fn spnego_gss_complete_auth_token(mut minor_status:
                                                            *mut OM_uint32,
                                                        context_handle:
                                                            gss_ctx_id_t,
                                                        mut input_message_buffer:
                                                            gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_complete_auth_token(minor_status, (*sc).ctx_handle,
                                input_message_buffer);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2672:1"]
pub unsafe extern "C" fn spnego_gss_acquire_cred_impersonate_name(mut minor_status:
                                                                      *mut OM_uint32,
                                                                  impersonator_cred_handle:
                                                                      gss_cred_id_t,
                                                                  desired_name:
                                                                      gss_name_t,
                                                                  mut time_req:
                                                                      OM_uint32,
                                                                  mut desired_mechs:
                                                                      gss_OID_set,
                                                                  mut cred_usage:
                                                                      gss_cred_usage_t,
                                                                  mut output_cred_handle:
                                                                      *mut gss_cred_id_t,
                                                                  mut actual_mechs:
                                                                      *mut gss_OID_set,
                                                                  mut time_rec:
                                                                      *mut OM_uint32)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut amechs: gss_OID_set = 0 as gss_OID_set;
    let mut imp_spcred: spnego_gss_cred_id_t = 0 as spnego_gss_cred_id_t;
    let mut out_spcred: spnego_gss_cred_id_t = 0 as spnego_gss_cred_id_t;
    let mut imp_mcred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut out_mcred: gss_cred_id_t = 0 as gss_cred_id_t;
    if !actual_mechs.is_null() { *actual_mechs = 0 as gss_OID_set }
    if !time_rec.is_null() { *time_rec = 0 as libc::c_int as OM_uint32 }
    imp_spcred = impersonator_cred_handle as spnego_gss_cred_id_t;
    imp_mcred =
        if !imp_spcred.is_null() {
            (*imp_spcred).mcred
        } else { 0 as gss_cred_id_t };
    status =
        gss_inquire_cred(minor_status, imp_mcred, 0 as *mut gss_name_t,
                         0 as *mut OM_uint32, 0 as *mut gss_cred_usage_t,
                         &mut amechs);
    if status != 0 as libc::c_int as libc::c_uint { return status }
    status =
        gss_acquire_cred_impersonate_name(minor_status, imp_mcred,
                                          desired_name, time_req, amechs,
                                          cred_usage, &mut out_mcred,
                                          actual_mechs, time_rec);
    if !(status != 0 as libc::c_int as libc::c_uint) {
        status = create_spnego_cred(minor_status, out_mcred, &mut out_spcred);
        if !(status != 0 as libc::c_int as libc::c_uint) {
            out_mcred = 0 as gss_cred_id_t;
            *output_cred_handle = out_spcred as gss_cred_id_t
        }
    }
    gss_release_oid_set(&mut tmpmin, &mut amechs);
    gss_release_cred(&mut tmpmin, &mut out_mcred);
    return status;
}
/* time_rec */
#[no_mangle]
#[c2rust::src_loc = "2726:1"]
pub unsafe extern "C" fn spnego_gss_acquire_cred_with_password(mut minor_status:
                                                                   *mut OM_uint32,
                                                               desired_name:
                                                                   gss_name_t,
                                                               password:
                                                                   gss_buffer_t,
                                                               mut time_req:
                                                                   OM_uint32,
                                                               desired_mechs:
                                                                   gss_OID_set,
                                                               mut cred_usage:
                                                                   gss_cred_usage_t,
                                                               mut output_cred_handle:
                                                                   *mut gss_cred_id_t,
                                                               mut actual_mechs:
                                                                   *mut gss_OID_set,
                                                               mut time_rec:
                                                                   *mut OM_uint32)
 -> OM_uint32 {
    let mut status: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut amechs: gss_OID_set = 0 as gss_OID_set;
    let mut mcred: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut spcred: spnego_gss_cred_id_t = 0 as spnego_gss_cred_id_t;
    if !actual_mechs.is_null() { *actual_mechs = 0 as gss_OID_set }
    if !time_rec.is_null() { *time_rec = 0 as libc::c_int as OM_uint32 }
    status =
        get_available_mechs(minor_status, desired_name, cred_usage,
                            0 as gss_const_key_value_set_t,
                            0 as *mut gss_cred_id_t, &mut amechs,
                            0 as *mut OM_uint32);
    if !(status != 0 as libc::c_int as libc::c_uint) {
        status =
            gss_acquire_cred_with_password(minor_status, desired_name,
                                           password, time_req, amechs,
                                           cred_usage, &mut mcred,
                                           actual_mechs, time_rec);
        if !(status != 0 as libc::c_int as libc::c_uint) {
            status = create_spnego_cred(minor_status, mcred, &mut spcred);
            if !(status != 0 as libc::c_int as libc::c_uint) {
                mcred = 0 as gss_cred_id_t;
                *output_cred_handle = spcred as gss_cred_id_t
            }
        }
    }
    gss_release_oid_set(&mut tmpmin, &mut amechs);
    gss_release_cred(&mut tmpmin, &mut mcred);
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "2779:1"]
pub unsafe extern "C" fn spnego_gss_display_name_ext(mut minor_status:
                                                         *mut OM_uint32,
                                                     mut name: gss_name_t,
                                                     mut display_as_name_type:
                                                         gss_OID,
                                                     mut display_name:
                                                         gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    ret =
        gss_display_name_ext(minor_status, name, display_as_name_type,
                             display_name);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2794:1"]
pub unsafe extern "C" fn spnego_gss_inquire_name(mut minor_status:
                                                     *mut OM_uint32,
                                                 mut name: gss_name_t,
                                                 mut name_is_MN:
                                                     *mut libc::c_int,
                                                 mut MN_mech: *mut gss_OID,
                                                 mut attrs:
                                                     *mut gss_buffer_set_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    ret = gss_inquire_name(minor_status, name, name_is_MN, MN_mech, attrs);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2810:1"]
pub unsafe extern "C" fn spnego_gss_get_name_attribute(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut name: gss_name_t,
                                                       mut attr: gss_buffer_t,
                                                       mut authenticated:
                                                           *mut libc::c_int,
                                                       mut complete:
                                                           *mut libc::c_int,
                                                       mut value:
                                                           gss_buffer_t,
                                                       mut display_value:
                                                           gss_buffer_t,
                                                       mut more:
                                                           *mut libc::c_int)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    ret =
        gss_get_name_attribute(minor_status, name, attr, authenticated,
                               complete, value, display_value, more);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2832:1"]
pub unsafe extern "C" fn spnego_gss_set_name_attribute(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut name: gss_name_t,
                                                       mut complete:
                                                           libc::c_int,
                                                       mut attr: gss_buffer_t,
                                                       mut value:
                                                           gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    ret = gss_set_name_attribute(minor_status, name, complete, attr, value);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2848:1"]
pub unsafe extern "C" fn spnego_gss_delete_name_attribute(mut minor_status:
                                                              *mut OM_uint32,
                                                          mut name:
                                                              gss_name_t,
                                                          mut attr:
                                                              gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    ret = gss_delete_name_attribute(minor_status, name, attr);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2860:1"]
pub unsafe extern "C" fn spnego_gss_export_name_composite(mut minor_status:
                                                              *mut OM_uint32,
                                                          mut name:
                                                              gss_name_t,
                                                          mut exp_composite_name:
                                                              gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    ret = gss_export_name_composite(minor_status, name, exp_composite_name);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2872:1"]
pub unsafe extern "C" fn spnego_gss_map_name_to_any(mut minor_status:
                                                        *mut OM_uint32,
                                                    mut name: gss_name_t,
                                                    mut authenticated:
                                                        libc::c_int,
                                                    mut type_id: gss_buffer_t,
                                                    mut output:
                                                        *mut gss_any_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    ret =
        gss_map_name_to_any(minor_status, name, authenticated, type_id,
                            output);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2888:1"]
pub unsafe extern "C" fn spnego_gss_release_any_name_mapping(mut minor_status:
                                                                 *mut OM_uint32,
                                                             mut name:
                                                                 gss_name_t,
                                                             mut type_id:
                                                                 gss_buffer_t,
                                                             mut input:
                                                                 *mut gss_any_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    ret = gss_release_any_name_mapping(minor_status, name, type_id, input);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2902:1"]
pub unsafe extern "C" fn spnego_gss_pseudo_random(mut minor_status:
                                                      *mut OM_uint32,
                                                  mut context: gss_ctx_id_t,
                                                  mut prf_key: libc::c_int,
                                                  prf_in: gss_buffer_t,
                                                  mut desired_output_len:
                                                      ssize_t,
                                                  mut prf_out: gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut sc: spnego_gss_ctx_id_t = context as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    ret =
        gss_pseudo_random(minor_status, (*sc).ctx_handle, prf_key, prf_in,
                          desired_output_len, prf_out);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2925:1"]
pub unsafe extern "C" fn spnego_gss_set_neg_mechs(mut minor_status:
                                                      *mut OM_uint32,
                                                  mut cred_handle:
                                                      gss_cred_id_t,
                                                  mech_list: gss_OID_set)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut spcred: spnego_gss_cred_id_t =
        cred_handle as spnego_gss_cred_id_t;
    /* Store mech_list in spcred for use in negotiation logic. */
    gss_release_oid_set(minor_status, &mut (*spcred).neg_mechs);
    ret =
        generic_gss_copy_oid_set(minor_status,
                                 mech_list as *const gss_OID_set_desc,
                                 &mut (*spcred).neg_mechs);
    if ret == 0 as libc::c_int as libc::c_uint {
        gss_set_neg_mechs(minor_status, (*spcred).mcred, (*spcred).neg_mechs);
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2949:1"]
pub unsafe extern "C" fn spnego_gss_inquire_mech_for_saslname(mut minor_status:
                                                                  *mut OM_uint32,
                                                              sasl_mech_name:
                                                                  gss_buffer_t,
                                                              mut mech_type:
                                                                  *mut gss_OID)
 -> OM_uint32 {
    if (*sasl_mech_name).length ==
           (::std::mem::size_of::<[libc::c_char; 7]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
           &&
           memcmp((*sasl_mech_name).value,
                  b"SPNEGO\x00" as *const u8 as *const libc::c_char as
                      *const libc::c_void,
                  (::std::mem::size_of::<[libc::c_char; 7]>() as
                       libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong)) ==
               0 as libc::c_int {
        if !mech_type.is_null() { *mech_type = gss_mech_spnego as gss_OID }
        return 0 as libc::c_int as OM_uint32
    }
    return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "2965:1"]
pub unsafe extern "C" fn spnego_gss_inquire_saslname_for_mech(mut minor_status:
                                                                  *mut OM_uint32,
                                                              desired_mech:
                                                                  gss_OID,
                                                              mut sasl_mech_name:
                                                                  gss_buffer_t,
                                                              mut mech_name:
                                                                  gss_buffer_t,
                                                              mut mech_description:
                                                                  gss_buffer_t)
 -> OM_uint32 {
    *minor_status = 0 as libc::c_int as OM_uint32;
    if !((*desired_mech).length == (*gss_mech_spnego).length &&
             memcmp((*desired_mech).elements, (*gss_mech_spnego).elements,
                    (*desired_mech).length as libc::c_ulong) ==
                 0 as libc::c_int) {
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if gssint_g_make_string_buffer(b"SPNEGO\x00" as *const u8 as
                                       *const libc::c_char, sasl_mech_name) ==
           0 ||
           gssint_g_make_string_buffer(b"spnego\x00" as *const u8 as
                                           *const libc::c_char, mech_name) ==
               0 ||
           gssint_g_make_string_buffer(b"Simple and Protected GSS-API Negotiation Mechanism\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       mech_description) == 0 {
        *minor_status = 12 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    } else { return 0 as libc::c_int as OM_uint32 };
}
#[no_mangle]
#[c2rust::src_loc = "2990:1"]
pub unsafe extern "C" fn spnego_gss_inquire_attrs_for_mech(mut minor_status:
                                                               *mut OM_uint32,
                                                           mut mech:
                                                               gss_const_OID,
                                                           mut mech_attrs:
                                                               *mut gss_OID_set,
                                                           mut known_mech_attrs:
                                                               *mut gss_OID_set)
 -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    let mut tmpMinor: OM_uint32 = 0;
    /* known_mech_attrs is handled by mechglue */
    *minor_status = 0 as libc::c_int as OM_uint32;
    if mech_attrs.is_null() { return 0 as libc::c_int as OM_uint32 }
    major = gss_create_empty_oid_set(minor_status, mech_attrs);
    if !(major &
             ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                  (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int)
             != 0) {
        major =
            gss_add_oid_set_member(minor_status,
                                   GSS_C_MA_MECH_NEGO as gss_OID, mech_attrs);
        if !(major &
                 ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                      (0o377 as libc::c_ulong as OM_uint32) <<
                          16 as libc::c_int) != 0) {
            major =
                gss_add_oid_set_member(minor_status,
                                       GSS_C_MA_ITOK_FRAMED as gss_OID,
                                       mech_attrs);
            (major &
                 ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                      (0o377 as libc::c_ulong as OM_uint32) <<
                          16 as libc::c_int)) != 0;
        }
    }
    if major &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        gss_release_oid_set(&mut tmpMinor, mech_attrs);
    }
    return major;
}
#[no_mangle]
#[c2rust::src_loc = "3025:1"]
pub unsafe extern "C" fn spnego_gss_export_cred(mut minor_status:
                                                    *mut OM_uint32,
                                                mut cred_handle:
                                                    gss_cred_id_t,
                                                mut token: gss_buffer_t)
 -> OM_uint32 {
    let mut spcred: spnego_gss_cred_id_t =
        cred_handle as spnego_gss_cred_id_t;
    return gss_export_cred(minor_status, (*spcred).mcred, token);
}
#[no_mangle]
#[c2rust::src_loc = "3035:1"]
pub unsafe extern "C" fn spnego_gss_import_cred(mut minor_status:
                                                    *mut OM_uint32,
                                                mut token: gss_buffer_t,
                                                mut cred_handle:
                                                    *mut gss_cred_id_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut spcred: spnego_gss_cred_id_t = 0 as *mut C2RustUnnamed_0;
    let mut mcred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    ret = gss_import_cred(minor_status, token, &mut mcred);
    if ret &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        return ret
    }
    ret = create_spnego_cred(minor_status, mcred, &mut spcred);
    if ret &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        return ret
    }
    *cred_handle = spcred as gss_cred_id_t;
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "3056:1"]
pub unsafe extern "C" fn spnego_gss_get_mic_iov(mut minor_status:
                                                    *mut OM_uint32,
                                                mut context_handle:
                                                    gss_ctx_id_t,
                                                mut qop_req: gss_qop_t,
                                                mut iov:
                                                    *mut gss_iov_buffer_desc,
                                                mut iov_count: libc::c_int)
 -> OM_uint32 {
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return gss_get_mic_iov(minor_status, (*sc).ctx_handle, qop_req, iov,
                           iov_count);
}
#[no_mangle]
#[c2rust::src_loc = "3070:1"]
pub unsafe extern "C" fn spnego_gss_verify_mic_iov(mut minor_status:
                                                       *mut OM_uint32,
                                                   mut context_handle:
                                                       gss_ctx_id_t,
                                                   mut qop_state:
                                                       *mut gss_qop_t,
                                                   mut iov:
                                                       *mut gss_iov_buffer_desc,
                                                   mut iov_count: libc::c_int)
 -> OM_uint32 {
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return gss_verify_mic_iov(minor_status, (*sc).ctx_handle, qop_state, iov,
                              iov_count);
}
#[no_mangle]
#[c2rust::src_loc = "3084:1"]
pub unsafe extern "C" fn spnego_gss_get_mic_iov_length(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut context_handle:
                                                           gss_ctx_id_t,
                                                       mut qop_req: gss_qop_t,
                                                       mut iov:
                                                           *mut gss_iov_buffer_desc,
                                                       mut iov_count:
                                                           libc::c_int)
 -> OM_uint32 {
    let mut sc: spnego_gss_ctx_id_t = context_handle as spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return gss_get_mic_iov_length(minor_status, (*sc).ctx_handle, qop_req,
                                  iov, iov_count);
}
/*
 * We will release everything but the ctx_handle so that it
 * can be passed back to init/accept context. This routine should
 * not be called until after the ctx_handle memory is assigned to
 * the supplied context handle from init/accept context.
 */
#[c2rust::src_loc = "3104:1"]
unsafe extern "C" fn release_spnego_ctx(mut ctx: *mut spnego_gss_ctx_id_t) {
    let mut context: spnego_gss_ctx_id_t = 0 as *mut spnego_ctx_st;
    let mut minor_stat: OM_uint32 = 0;
    context = *ctx;
    if !context.is_null() {
        gss_release_buffer(&mut minor_stat, &mut (*context).DER_mechTypes);
        gss_release_oid_set(&mut minor_stat, &mut (*context).mech_set);
        gss_release_name(&mut minor_stat, &mut (*context).internal_name);
        gss_release_cred(&mut minor_stat, &mut (*context).deleg_cred);
        negoex_release_context(context);
        free(context as *mut libc::c_void);
        *ctx = 0 as spnego_gss_ctx_id_t
    };
}
/*
 * Can't use gss_indicate_mechs by itself to get available mechs for
 * SPNEGO because it will also return the SPNEGO mech and we do not
 * want to consider SPNEGO as an available security mech for
 * negotiation. For this reason, get_available_mechs will return
 * all available, non-deprecated mechs except SPNEGO and NegoEx-
 * only mechanisms.
 *
 * Note that gss_acquire_cred_from(GSS_C_NO_OID_SET) will filter
 * out hidden (GSS_C_MA_NOT_INDICATED) mechanisms such as NegoEx, so
 * calling gss_indicate_mechs_by_attrs() also works around that.
 *
 * If a ptr to a creds list is given, this function will attempt
 * to acquire creds for the creds given and trim the list of
 * returned mechanisms to only those for which creds are valid.
 *
 */
#[c2rust::src_loc = "3144:1"]
unsafe extern "C" fn get_available_mechs(mut minor_status: *mut OM_uint32,
                                         mut name: gss_name_t,
                                         mut usage: gss_cred_usage_t,
                                         mut cred_store:
                                             gss_const_key_value_set_t,
                                         mut creds: *mut gss_cred_id_t,
                                         mut rmechs: *mut gss_OID_set,
                                         mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    let mut major_status: OM_uint32 =
        0 as libc::c_int as OM_uint32; /* Exclude ourselves */
    let mut tmpmin: OM_uint32 = 0;
    let mut mechs: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut goodmechs: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut except_attrs: gss_OID_set_desc =
        gss_OID_set_desc{count: 0, elements: 0 as *mut gss_OID_desc_struct,};
    let mut attr_oids: [gss_OID_desc; 3] =
        [gss_OID_desc{length: 0,
                      elements:
                          0 as *const libc::c_void as *mut libc::c_void,}; 3];
    *rmechs = 0 as gss_OID_set;
    attr_oids[0 as libc::c_int as usize] = *GSS_C_MA_DEPRECATED;
    attr_oids[1 as libc::c_int as usize] = *GSS_C_MA_NOT_DFLT_MECH;
    attr_oids[2 as libc::c_int as usize] = *GSS_C_MA_MECH_NEGO;
    except_attrs.count =
        (::std::mem::size_of::<[gss_OID_desc; 3]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<gss_OID_desc>()
                                             as libc::c_ulong);
    except_attrs.elements = attr_oids.as_mut_ptr();
    major_status =
        gss_indicate_mechs_by_attrs(minor_status,
                                    0 as gss_OID_set as gss_const_OID_set,
                                    &mut except_attrs as *mut gss_OID_set_desc
                                        as gss_const_OID_set,
                                    0 as gss_OID_set as gss_const_OID_set,
                                    &mut mechs);
    /*
	 * If the caller wanted a list of creds returned,
	 * trim the list of mechanisms down to only those
	 * for which the creds are valid.
	 */
    if (*mechs).count > 0 as libc::c_int as libc::c_ulong &&
           major_status == 0 as libc::c_int as libc::c_uint &&
           !creds.is_null() {
        major_status =
            gss_acquire_cred_from(minor_status, name,
                                  0xffffffff as libc::c_ulong as OM_uint32,
                                  mechs, usage, cred_store, creds,
                                  &mut goodmechs, time_rec);
        /*
		 * Drop the old list in favor of the new
		 * "trimmed" list.
		 */
        if major_status == 0 as libc::c_int as libc::c_uint {
            gss_release_oid_set(&mut tmpmin, &mut mechs);
            mechs = goodmechs
        }
    }
    if (*mechs).count > 0 as libc::c_int as libc::c_ulong &&
           major_status == 0 as libc::c_int as libc::c_uint {
        *rmechs = mechs
    } else {
        gss_release_oid_set(&mut tmpmin, &mut mechs);
        *minor_status = 0x20000001 as libc::c_int as OM_uint32;
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status);
        if major_status == 0 as libc::c_int as libc::c_uint {
            major_status =
                (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    return major_status;
}
/* Return true if mech asserts the GSS_C_MA_NEGOEX_AND_SPNEGO attribute. */
#[c2rust::src_loc = "3204:1"]
unsafe extern "C" fn negoex_and_spnego(mut mech: gss_OID) -> libc::c_int {
    let mut ret: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut attrs: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut present: libc::c_int = 0;
    ret =
        gss_inquire_attrs_for_mech(&mut minor, mech as gss_const_OID,
                                   &mut attrs, 0 as *mut gss_OID_set);
    if ret != 0 as libc::c_int as libc::c_uint || attrs.is_null() {
        return 0 as libc::c_int
    }
    generic_gss_test_oid_set_member(&mut minor, GSS_C_MA_NEGOEX_AND_SPNEGO,
                                    attrs, &mut present);
    gss_release_oid_set(&mut minor, &mut attrs);
    return present;
}
/*
 * Fill sc->mech_set with the SPNEGO-negotiable mechanism OIDs, and
 * sc->negoex_mechs with an entry for each NegoEx-negotiable mechanism.  Take
 * into account the mech set provided with gss_set_neg_mechs() if it exists.
 */
#[c2rust::src_loc = "3227:1"]
unsafe extern "C" fn get_negotiable_mechs(mut minor_status: *mut OM_uint32,
                                          mut sc: spnego_gss_ctx_id_t,
                                          mut spcred: spnego_gss_cred_id_t,
                                          mut usage: gss_cred_usage_t)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut ret: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut creds: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut cred_mechs: gss_OID_set = 0 as gss_OID_set;
    let mut mechs: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut i: libc::c_uint = 0;
    let mut present: libc::c_int = 0;
    let mut added_negoex: libc::c_int = 0 as libc::c_int;
    let mut scheme: auth_scheme = [0; 16];
    if !spcred.is_null() {
        /* Get the list of mechs in the mechglue cred. */
        ret =
            gss_inquire_cred(minor_status, (*spcred).mcred,
                             0 as *mut gss_name_t, 0 as *mut OM_uint32,
                             0 as *mut gss_cred_usage_t, &mut cred_mechs);
        if ret != 0 as libc::c_int as libc::c_uint { return ret }
    } else {
        /* Start with the list of available mechs. */
        ret =
            get_available_mechs(minor_status, 0 as gss_name_t, usage,
                                0 as gss_const_key_value_set_t, &mut creds,
                                &mut cred_mechs, 0 as *mut OM_uint32);
        if ret != 0 as libc::c_int as libc::c_uint { return ret }
        gss_release_cred(&mut tmpmin, &mut creds);
    }
    /* If gss_set_neg_mechs() was called, use that to determine the
	 * iteration order.  Otherwise iterate over the credential mechs. */
    mechs =
        if !spcred.is_null() && !(*spcred).neg_mechs.is_null() {
            (*spcred).neg_mechs
        } else { cred_mechs };
    ret = gss_create_empty_oid_set(minor_status, &mut (*sc).mech_set);
    if !(ret != 0 as libc::c_int as libc::c_uint) {
        i = 0 as libc::c_int as libc::c_uint;
        loop  {
            if !((i as libc::c_ulong) < (*mechs).count) {
                current_block = 17500079516916021833;
                break ;
            }
            if mechs != cred_mechs {
                /* Intersect neg_mechs with cred_mechs. */
                gss_test_oid_set_member(&mut tmpmin,
                                        &mut *(*mechs).elements.offset(i as
                                                                           isize),
                                        cred_mechs, &mut present);
                if present == 0 {
                    current_block = 13056961889198038528;
                } else { current_block = 26972500619410423; }
            } else { current_block = 26972500619410423; }
            match current_block {
                26972500619410423 => {
                    /* Query the auth scheme to see if this is a NegoEx mech. */
                    ret =
                        gssspi_query_mechanism_info(&mut tmpmin,
                                                    &mut *(*mechs).elements.offset(i
                                                                                       as
                                                                                       isize)
                                                        as
                                                        *mut gss_OID_desc_struct
                                                        as gss_const_OID,
                                                    scheme.as_mut_ptr());
                    if ret == 0 as libc::c_int as libc::c_uint {
                        /* Add an entry for this mech to the NegoEx list. */
                        ret =
                            negoex_add_auth_mech(minor_status, sc,
                                                 &mut *(*mechs).elements.offset(i
                                                                                    as
                                                                                    isize)
                                                     as
                                                     *mut gss_OID_desc_struct
                                                     as gss_const_OID,
                                                 scheme.as_mut_ptr());
                        if ret != 0 as libc::c_int as libc::c_uint {
                            current_block = 4395529121510314304;
                            break ;
                        }
                        /* Add the NegoEx OID to the SPNEGO list at the
			 * position of the first NegoEx mechanism. */
                        if added_negoex == 0 {
                            ret =
                                gss_add_oid_set_member(minor_status,
                                                       &mut negoex_mech,
                                                       &mut (*sc).mech_set);
                            if ret != 0 as libc::c_int as libc::c_uint {
                                current_block = 4395529121510314304;
                                break ;
                            }
                            added_negoex = 1 as libc::c_int
                        }
                        /* Skip this mech in the SPNEGO list unless it asks for
			 * direct SPNEGO negotiation. */
                        if negoex_and_spnego(&mut *(*mechs).elements.offset(i
                                                                                as
                                                                                isize))
                               == 0 {
                            current_block = 13056961889198038528;
                        } else { current_block = 17788412896529399552; }
                    } else { current_block = 17788412896529399552; }
                    match current_block {
                        13056961889198038528 => { }
                        _ => {
                            /* Add this mech to the SPNEGO list. */
                            ret =
                                gss_add_oid_set_member(minor_status,
                                                       &mut *(*mechs).elements.offset(i
                                                                                          as
                                                                                          isize),
                                                       &mut (*sc).mech_set);
                            if ret != 0 as libc::c_int as libc::c_uint {
                                current_block = 4395529121510314304;
                                break ;
                            }
                        }
                    }
                }
                _ => { }
            }
            i = i.wrapping_add(1)
        }
        match current_block {
            4395529121510314304 => { }
            _ => { *minor_status = 0 as libc::c_int as OM_uint32 }
        }
    }
    if ret != 0 as libc::c_int as libc::c_uint ||
           (*(*sc).mech_set).count == 0 as libc::c_int as libc::c_ulong {
        *minor_status = 0x20000001 as libc::c_int as OM_uint32;
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status);
        ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    gss_release_oid_set(&mut tmpmin, &mut cred_mechs);
    return ret;
}
/* following are token creation and reading routines */
/*
 * If buff_in is not pointing to a MECH_OID, then return NULL and do not
 * advance the buffer, otherwise, decode the mech_oid from the buffer and
 * place in gss_OID.
 */
#[c2rust::src_loc = "3327:1"]
unsafe extern "C" fn get_mech_oid(mut minor_status: *mut OM_uint32,
                                  mut buff_in: *mut *mut libc::c_uchar,
                                  mut length: size_t) -> gss_OID {
    let mut status: OM_uint32 = 0;
    let mut toid: gss_OID_desc =
        gss_OID_desc{length: 0,
                     elements:
                         0 as *const libc::c_void as *mut libc::c_void,};
    let mut mech_out: gss_OID = 0 as gss_OID;
    let mut start: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if length < 1 as libc::c_int as libc::c_ulong ||
           **buff_in as libc::c_int != 0x6 as libc::c_int {
        return 0 as gss_OID
    }
    start = *buff_in;
    end = start.offset(length as isize);
    *buff_in = (*buff_in).offset(1);
    let fresh3 = *buff_in;
    *buff_in = (*buff_in).offset(1);
    toid.length = *fresh3 as OM_uint32;
    if (*buff_in).offset(toid.length as isize) > end { return 0 as gss_OID }
    toid.elements = *buff_in as *mut libc::c_void;
    *buff_in = (*buff_in).offset(toid.length as isize);
    status = generic_gss_copy_oid(minor_status, &mut toid, &mut mech_out);
    if status != 0 as libc::c_int as libc::c_uint {
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status);
        mech_out = 0 as gss_OID
    }
    return mech_out;
}
/*
 * der encode the given mechanism oid into buf_out, advancing the
 * buffer pointer.
 */
#[c2rust::src_loc = "3365:1"]
unsafe extern "C" fn put_mech_oid(mut buf_out: *mut *mut libc::c_uchar,
                                  mut mech: gss_OID_const,
                                  mut buflen: libc::c_uint) -> libc::c_int {
    if buflen < (*mech).length.wrapping_add(2 as libc::c_int as libc::c_uint)
       {
        return -(1 as libc::c_int)
    }
    let fresh4 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh4 = 0x6 as libc::c_int as libc::c_uchar;
    let fresh5 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh5 = (*mech).length as libc::c_uchar;
    memcpy(*buf_out as *mut libc::c_void, (*mech).elements,
           (*mech).length as libc::c_ulong);
    *buf_out = (*buf_out).offset((*mech).length as isize);
    return 0 as libc::c_int;
}
/*
 * verify that buff_in points to an octet string, if it does not,
 * return NULL and don't advance the pointer. If it is an octet string
 * decode buff_in into a gss_buffer_t and return it, advancing the
 * buffer pointer.
 */
#[c2rust::src_loc = "3383:1"]
unsafe extern "C" fn get_input_token(mut buff_in: *mut *mut libc::c_uchar,
                                     mut buff_length: libc::c_uint)
 -> gss_buffer_t {
    let mut input_token: gss_buffer_t = 0 as *mut gss_buffer_desc_struct;
    let mut len: libc::c_uint = 0;
    if g_get_tag_and_length(buff_in, 0x4 as libc::c_int, buff_length,
                            &mut len) < 0 as libc::c_int {
        return 0 as gss_buffer_t
    }
    input_token =
        malloc(::std::mem::size_of::<gss_buffer_desc>() as libc::c_ulong) as
            gss_buffer_t;
    if input_token.is_null() { return 0 as gss_buffer_t }
    (*input_token).length = len as size_t;
    if (*input_token).length > 0 as libc::c_int as libc::c_ulong {
        (*input_token).value = gssalloc_malloc((*input_token).length);
        if (*input_token).value.is_null() {
            free(input_token as *mut libc::c_void);
            return 0 as gss_buffer_t
        }
        memcpy((*input_token).value, *buff_in as *const libc::c_void,
               (*input_token).length);
    } else { (*input_token).value = 0 as *mut libc::c_void }
    *buff_in = (*buff_in).offset((*input_token).length as isize);
    return input_token;
}
/*
 * verify that the input token length is not 0. If it is, just return.
 * If the token length is greater than 0, der encode as an octet string
 * and place in buf_out, advancing buf_out.
 */
#[c2rust::src_loc = "3418:1"]
unsafe extern "C" fn put_input_token(mut buf_out: *mut *mut libc::c_uchar,
                                     mut input_token: gss_buffer_t,
                                     mut buflen: libc::c_uint)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    /* if token length is 0, we do not want to send */
    if (*input_token).length == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    if (*input_token).length > buflen as libc::c_ulong {
        return -(1 as libc::c_int)
    }
    let fresh6 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh6 = 0x4 as libc::c_int as libc::c_uchar;
    ret =
        gssint_put_der_length((*input_token).length as libc::c_uint, buf_out,
                              (*input_token).length as libc::c_uint);
    if ret != 0 { return ret }
    memcpy(*buf_out as *mut libc::c_void, (*input_token).value,
           (*input_token).length);
    *buf_out = (*buf_out).offset((*input_token).length as isize);
    return 0 as libc::c_int;
}
/*
 * verify that buff_in points to a sequence of der encoding. The mech
 * set is the only sequence of encoded object in the token, so if it is
 * a sequence of encoding, decode the mechset into a gss_OID_set and
 * return it, advancing the buffer pointer.
 */
#[c2rust::src_loc = "3445:1"]
unsafe extern "C" fn get_mech_set(mut minor_status: *mut OM_uint32,
                                  mut buff_in: *mut *mut libc::c_uchar,
                                  mut buff_length: libc::c_uint)
 -> gss_OID_set {
    let mut returned_mechSet: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut major_status: OM_uint32 = 0;
    let mut length: libc::c_int = 0;
    let mut bytes: libc::c_uint = 0;
    let mut set_length: OM_uint32 = 0;
    let mut start: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    if **buff_in as libc::c_int != 0x30 as libc::c_int {
        return 0 as gss_OID_set
    }
    start = *buff_in;
    *buff_in = (*buff_in).offset(1);
    length = gssint_get_der_length(buff_in, buff_length, &mut bytes);
    if length < 0 as libc::c_int ||
           buff_length.wrapping_sub(bytes) < length as libc::c_uint {
        return 0 as gss_OID_set
    }
    major_status =
        gss_create_empty_oid_set(minor_status, &mut returned_mechSet);
    if major_status != 0 as libc::c_int as libc::c_uint {
        return 0 as gss_OID_set
    }
    set_length = 0 as libc::c_int as OM_uint32;
    i = 0 as libc::c_int;
    while set_length < length as libc::c_uint {
        let mut temp: *mut gss_OID_desc =
            get_mech_oid(minor_status, buff_in,
                         (buff_length as libc::c_long -
                              (*buff_in).wrapping_offset_from(start) as
                                  libc::c_long) as size_t);
        if temp.is_null() { break ; }
        major_status =
            gss_add_oid_set_member(minor_status, temp, &mut returned_mechSet);
        if major_status == 0 as libc::c_int as libc::c_uint {
            set_length =
                (set_length as
                     libc::c_uint).wrapping_add((*(*returned_mechSet).elements.offset(i
                                                                                          as
                                                                                          isize)).length.wrapping_add(2
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_uint))
                    as OM_uint32 as OM_uint32;
            if generic_gss_release_oid(minor_status, &mut temp) != 0 {
                *minor_status = gssint_mecherrmap_map_errcode(*minor_status)
            }
        }
        i += 1
    }
    return returned_mechSet;
}
/*
 * Encode mechSet into buf.
 */
#[c2rust::src_loc = "3493:1"]
unsafe extern "C" fn put_mech_set(mut mechSet: gss_OID_set,
                                  mut buf: gss_buffer_t) -> libc::c_int {
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_uint = 0;
    let mut tlen: libc::c_uint = 0;
    let mut ilen: libc::c_uint = 0;
    ilen = 0 as libc::c_int as libc::c_uint;
    tlen = ilen;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*mechSet).count {
        /*
		 * 0x06 [DER LEN] [OID]
		 */
        ilen =
            ilen.wrapping_add((1 as libc::c_int as
                                   libc::c_uint).wrapping_add(gssint_der_length_size((*(*mechSet).elements.offset(i
                                                                                                                      as
                                                                                                                      isize)).length)).wrapping_add((*(*mechSet).elements.offset(i
                                                                                                                                                                                     as
                                                                                                                                                                                     isize)).length));
        i = i.wrapping_add(1)
    }
    /*
	 * 0x30 [DER LEN]
	 */
    tlen =
        (1 as libc::c_int as
             libc::c_uint).wrapping_add(gssint_der_length_size(ilen)).wrapping_add(ilen);
    ptr = gssalloc_malloc(tlen as size_t) as *mut libc::c_uchar;
    if ptr.is_null() { return -(1 as libc::c_int) }
    (*buf).value = ptr as *mut libc::c_void;
    (*buf).length = tlen as size_t;
    let fresh7 = ptr;
    ptr = ptr.offset(1);
    *fresh7 = 0x30 as libc::c_int as libc::c_uchar;
    if gssint_put_der_length(ilen, &mut ptr,
                             (*buf).length.wrapping_sub(((*buf).value as
                                                             *mut libc::c_uchar).wrapping_offset_from(ptr)
                                                            as libc::c_long as
                                                            libc::c_ulong) as
                                 libc::c_uint) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*mechSet).count {
        if put_mech_oid(&mut ptr,
                        &mut *(*mechSet).elements.offset(i as isize) as
                            *mut gss_OID_desc_struct as gss_OID_const,
                        (*buf).length.wrapping_sub(((*buf).value as
                                                        *mut libc::c_uchar).wrapping_offset_from(ptr)
                                                       as libc::c_long as
                                                       libc::c_ulong) as
                            libc::c_uint) < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
/*
 * Verify that buff_in is pointing to a BIT_STRING with the correct
 * length and padding for the req_flags. If it is, decode req_flags
 * and return them, otherwise, return NULL.
 */
#[c2rust::src_loc = "3538:1"]
unsafe extern "C" fn get_req_flags(mut buff_in: *mut *mut libc::c_uchar,
                                   mut bodysize: OM_uint32,
                                   mut req_flags: *mut OM_uint32)
 -> OM_uint32 {
    let mut len: libc::c_uint = 0;
    if **buff_in as libc::c_int != 0xa0 as libc::c_int | 0x1 as libc::c_int {
        return 0 as libc::c_int as OM_uint32
    }
    if g_get_tag_and_length(buff_in, 0xa0 as libc::c_int | 0x1 as libc::c_int,
                            bodysize, &mut len) < 0 as libc::c_int {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    let fresh8 = *buff_in;
    *buff_in = (*buff_in).offset(1);
    if *fresh8 as libc::c_int != 0x3 as libc::c_int {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    let fresh9 = *buff_in;
    *buff_in = (*buff_in).offset(1);
    if *fresh9 as libc::c_int != 0x2 as libc::c_int {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    let fresh10 = *buff_in;
    *buff_in = (*buff_in).offset(1);
    if *fresh10 as libc::c_int != 0x1 as libc::c_int {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    let fresh11 = *buff_in;
    *buff_in = (*buff_in).offset(1);
    *req_flags = (*fresh11 as libc::c_int >> 1 as libc::c_int) as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "3564:1"]
unsafe extern "C" fn get_negTokenInit(mut minor_status: *mut OM_uint32,
                                      mut buf: gss_buffer_t,
                                      mut der_mechSet: gss_buffer_t,
                                      mut mechSet: *mut gss_OID_set,
                                      mut req_flags: *mut OM_uint32,
                                      mut mechtok: *mut gss_buffer_t,
                                      mut mechListMIC: *mut gss_buffer_t)
 -> OM_uint32 {
    let mut err: OM_uint32 = 0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bufstart: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: libc::c_uint = 0;
    let mut tmpbuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    *minor_status = 0 as libc::c_int as OM_uint32;
    (*der_mechSet).length = 0 as libc::c_int as size_t;
    (*der_mechSet).value = 0 as *mut libc::c_void;
    *mechSet = 0 as gss_OID_set;
    *req_flags = 0 as libc::c_int as OM_uint32;
    *mechListMIC = 0 as gss_buffer_t;
    *mechtok = *mechListMIC;
    bufstart = (*buf).value as *mut libc::c_uchar;
    ptr = bufstart;
    if (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart) as
                                      libc::c_long as libc::c_ulong) >
           2147483647 as libc::c_int as libc::c_ulong {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    err =
        g_verify_token_header(gss_mech_spnego, &mut len, &mut ptr,
                              0 as libc::c_int,
                              (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart)
                                                             as libc::c_long
                                                             as libc::c_ulong)
                                  as libc::c_uint) as OM_uint32;
    if err != 0 {
        *minor_status = err;
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status);
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    *minor_status =
        g_verify_neg_token_init(&mut ptr,
                                (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart)
                                                               as libc::c_long
                                                               as
                                                               libc::c_ulong)
                                    as libc::c_uint) as OM_uint32;
    if *minor_status != 0 {
        *minor_status = gssint_mecherrmap_map_errcode(*minor_status);
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* alias into input_token */
    tmpbuf.value = ptr as *mut libc::c_void;
    tmpbuf.length =
        (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart) as
                                       libc::c_long as libc::c_ulong);
    *mechSet =
        get_mech_set(minor_status, &mut ptr,
                     (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart)
                                                    as libc::c_long as
                                                    libc::c_ulong) as
                         libc::c_uint);
    if (*mechSet).is_null() {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    tmpbuf.length =
        ptr.wrapping_offset_from(tmpbuf.value as *mut libc::c_uchar) as
            libc::c_long as size_t;
    (*der_mechSet).value = gssalloc_malloc(tmpbuf.length);
    if (*der_mechSet).value.is_null() {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    memcpy((*der_mechSet).value, tmpbuf.value, tmpbuf.length);
    (*der_mechSet).length = tmpbuf.length;
    err =
        get_req_flags(&mut ptr,
                      (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart)
                                                     as libc::c_long as
                                                     libc::c_ulong) as
                          OM_uint32, req_flags);
    if err != 0 as libc::c_int as libc::c_uint { return err }
    if g_get_tag_and_length(&mut ptr,
                            0xa0 as libc::c_int | 0x2 as libc::c_int,
                            (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart)
                                                           as libc::c_long as
                                                           libc::c_ulong) as
                                libc::c_uint, &mut len) >= 0 as libc::c_int {
        *mechtok = get_input_token(&mut ptr, len);
        if (*mechtok).is_null() {
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    if g_get_tag_and_length(&mut ptr,
                            0xa0 as libc::c_int | 0x3 as libc::c_int,
                            (*buf).length.wrapping_sub(ptr.wrapping_offset_from(bufstart)
                                                           as libc::c_long as
                                                           libc::c_ulong) as
                                libc::c_uint, &mut len) >= 0 as libc::c_int {
        *mechListMIC = get_input_token(&mut ptr, len);
        if (*mechListMIC).is_null() {
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "3639:1"]
unsafe extern "C" fn get_negTokenResp(mut minor_status: *mut OM_uint32,
                                      mut buf: *mut libc::c_uchar,
                                      mut buflen: libc::c_uint,
                                      mut negState: *mut OM_uint32,
                                      mut supportedMech: *mut gss_OID,
                                      mut responseToken: *mut gss_buffer_t,
                                      mut mechListMIC: *mut gss_buffer_t)
 -> OM_uint32 {
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bufstart: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: libc::c_uint = 0;
    let mut tmplen: libc::c_int = 0;
    let mut tag: libc::c_uint = 0;
    let mut bytes: libc::c_uint = 0;
    *negState = 0xffffffff as libc::c_ulong as OM_uint32;
    *supportedMech = 0 as gss_OID;
    *mechListMIC = 0 as gss_buffer_t;
    *responseToken = *mechListMIC;
    bufstart = buf;
    ptr = bufstart;
    if g_get_tag_and_length(&mut ptr,
                            0xa0 as libc::c_int | 0x1 as libc::c_int,
                            (buflen as libc::c_long -
                                 ptr.wrapping_offset_from(bufstart) as
                                     libc::c_long) as libc::c_uint, &mut len)
           < 0 as libc::c_int {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    let fresh12 = ptr;
    ptr = ptr.offset(1);
    if *fresh12 as libc::c_int == 0x30 as libc::c_int {
        tmplen =
            gssint_get_der_length(&mut ptr,
                                  (buflen as libc::c_long -
                                       ptr.wrapping_offset_from(bufstart) as
                                           libc::c_long) as libc::c_uint,
                                  &mut bytes);
        if tmplen < 0 as libc::c_int ||
               (buflen as libc::c_long -
                    ptr.wrapping_offset_from(bufstart) as libc::c_long) <
                   tmplen as libc::c_uint as libc::c_long {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    if (buflen as libc::c_long -
            ptr.wrapping_offset_from(bufstart) as libc::c_long) <
           1 as libc::c_int as libc::c_long {
        tag = 0 as libc::c_int as libc::c_uint
    } else {
        let fresh13 = ptr;
        ptr = ptr.offset(1);
        tag = *fresh13 as libc::c_uint
    }
    if tag == 0xa0 as libc::c_int as libc::c_uint {
        tmplen =
            gssint_get_der_length(&mut ptr,
                                  (buflen as libc::c_long -
                                       ptr.wrapping_offset_from(bufstart) as
                                           libc::c_long) as libc::c_uint,
                                  &mut bytes);
        if tmplen < 0 as libc::c_int ||
               (buflen as libc::c_long -
                    ptr.wrapping_offset_from(bufstart) as libc::c_long) <
                   tmplen as libc::c_uint as libc::c_long {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        if g_get_tag_and_length(&mut ptr, 0xa as libc::c_int,
                                (buflen as libc::c_long -
                                     ptr.wrapping_offset_from(bufstart) as
                                         libc::c_long) as libc::c_uint,
                                &mut len) < 0 as libc::c_int {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        if len != 1 as libc::c_int as libc::c_uint {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        if (buflen as libc::c_long -
                ptr.wrapping_offset_from(bufstart) as libc::c_long) <
               1 as libc::c_int as libc::c_long {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        let fresh14 = ptr;
        ptr = ptr.offset(1);
        *negState = *fresh14 as OM_uint32;
        if (buflen as libc::c_long -
                ptr.wrapping_offset_from(bufstart) as libc::c_long) <
               1 as libc::c_int as libc::c_long {
            tag = 0 as libc::c_int as libc::c_uint
        } else {
            let fresh15 = ptr;
            ptr = ptr.offset(1);
            tag = *fresh15 as libc::c_uint
        }
    }
    if tag == (0xa0 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        tmplen =
            gssint_get_der_length(&mut ptr,
                                  (buflen as libc::c_long -
                                       ptr.wrapping_offset_from(bufstart) as
                                           libc::c_long) as libc::c_uint,
                                  &mut bytes);
        if tmplen < 0 as libc::c_int ||
               (buflen as libc::c_long -
                    ptr.wrapping_offset_from(bufstart) as libc::c_long) <
                   tmplen as libc::c_uint as libc::c_long {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        *supportedMech =
            get_mech_oid(minor_status, &mut ptr,
                         (buflen as libc::c_long -
                              ptr.wrapping_offset_from(bufstart) as
                                  libc::c_long) as size_t);
        if (*supportedMech).is_null() {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        if (buflen as libc::c_long -
                ptr.wrapping_offset_from(bufstart) as libc::c_long) <
               1 as libc::c_int as libc::c_long {
            tag = 0 as libc::c_int as libc::c_uint
        } else {
            let fresh16 = ptr;
            ptr = ptr.offset(1);
            tag = *fresh16 as libc::c_uint
        }
    }
    if tag == (0xa0 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        tmplen =
            gssint_get_der_length(&mut ptr,
                                  (buflen as libc::c_long -
                                       ptr.wrapping_offset_from(bufstart) as
                                           libc::c_long) as libc::c_uint,
                                  &mut bytes);
        if tmplen < 0 as libc::c_int ||
               (buflen as libc::c_long -
                    ptr.wrapping_offset_from(bufstart) as libc::c_long) <
                   tmplen as libc::c_uint as libc::c_long {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        *responseToken =
            get_input_token(&mut ptr,
                            (buflen as libc::c_long -
                                 ptr.wrapping_offset_from(bufstart) as
                                     libc::c_long) as libc::c_uint);
        if (*responseToken).is_null() {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        if (buflen as libc::c_long -
                ptr.wrapping_offset_from(bufstart) as libc::c_long) <
               1 as libc::c_int as libc::c_long {
            tag = 0 as libc::c_int as libc::c_uint
        } else {
            let fresh17 = ptr;
            ptr = ptr.offset(1);
            tag = *fresh17 as libc::c_uint
        }
    }
    if tag == (0xa0 as libc::c_int | 0x3 as libc::c_int) as libc::c_uint {
        tmplen =
            gssint_get_der_length(&mut ptr,
                                  (buflen as libc::c_long -
                                       ptr.wrapping_offset_from(bufstart) as
                                           libc::c_long) as libc::c_uint,
                                  &mut bytes);
        if tmplen < 0 as libc::c_int ||
               (buflen as libc::c_long -
                    ptr.wrapping_offset_from(bufstart) as libc::c_long) <
                   tmplen as libc::c_uint as libc::c_long {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        *mechListMIC =
            get_input_token(&mut ptr,
                            (buflen as libc::c_long -
                                 ptr.wrapping_offset_from(bufstart) as
                                     libc::c_long) as libc::c_uint);
        if (*mechListMIC).is_null() {
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        /* Handle Windows 2000 duplicate response token */
        if !(*responseToken).is_null() &&
               (**responseToken).length == (**mechListMIC).length &&
               memcmp((**responseToken).value, (**mechListMIC).value,
                      (**responseToken).length) == 0 {
            let mut tmpmin: OM_uint32 = 0;
            gss_release_buffer(&mut tmpmin, *mechListMIC);
            free(*mechListMIC as *mut libc::c_void);
            *mechListMIC = 0 as gss_buffer_t
        }
    }
    return 0 as libc::c_int as OM_uint32;
}
/*
 * der encode the passed negResults as an ENUMERATED type and
 * place it in buf_out, advancing the buffer.
 */
#[c2rust::src_loc = "3749:1"]
unsafe extern "C" fn put_negResult(mut buf_out: *mut *mut libc::c_uchar,
                                   mut negResult: OM_uint32,
                                   mut buflen: libc::c_uint) -> libc::c_int {
    if buflen < 3 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    let fresh18 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh18 = 0xa as libc::c_int as libc::c_uchar;
    let fresh19 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh19 = 1 as libc::c_int as libc::c_uchar;
    let fresh20 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh20 = negResult as libc::c_uchar;
    return 0 as libc::c_int;
}
/*
 * This routine compares the recieved mechset to the mechset that
 * this server can support. It looks sequentially through the mechset
 * and the first one that matches what the server can support is
 * chosen as the negotiated mechanism. If one is found, negResult
 * is set to ACCEPT_INCOMPLETE if it's the first mech, REQUEST_MIC if
 * it's not the first mech, otherwise we return NULL and negResult
 * is set to REJECT. The returned pointer is an alias into
 * received->elements and should not be freed.
 *
 * NOTE: There is currently no way to specify a preference order of
 * mechanisms supported by the acceptor.
 */
#[c2rust::src_loc = "3774:1"]
unsafe extern "C" fn negotiate_mech(mut ctx: spnego_gss_ctx_id_t,
                                    mut received: gss_OID_set,
                                    mut negResult: *mut OM_uint32)
 -> gss_OID {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut wrong_krb5_oid: libc::c_int = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*received).count {
        let mut mech_oid: gss_OID =
            &mut *(*received).elements.offset(i as isize) as
                *mut gss_OID_desc_struct;
        /* Accept wrong mechanism OID from MS clients */
        wrong_krb5_oid = 0 as libc::c_int;
        if (*mech_oid).length == gss_mech_krb5_wrong_oid.length &&
               memcmp((*mech_oid).elements, gss_mech_krb5_wrong_oid.elements,
                      (*mech_oid).length as libc::c_ulong) == 0 as libc::c_int
           {
            mech_oid = &gss_mech_krb5_oid as *const gss_OID_desc as gss_OID;
            wrong_krb5_oid = 1 as libc::c_int
        }
        j = 0 as libc::c_int as size_t;
        while j < (*(*ctx).mech_set).count {
            if (*mech_oid).length ==
                   (*(*(*ctx).mech_set).elements.offset(j as isize)).length &&
                   memcmp((*mech_oid).elements,
                          (*(*(*ctx).mech_set).elements.offset(j as
                                                                   isize)).elements,
                          (*mech_oid).length as libc::c_ulong) ==
                       0 as libc::c_int {
                *negResult =
                    if i == 0 as libc::c_int as libc::c_ulong {
                        1 as libc::c_int
                    } else { 3 as libc::c_int } as OM_uint32;
                return if wrong_krb5_oid != 0 {
                           &gss_mech_krb5_wrong_oid as *const gss_OID_desc as
                               gss_OID
                       } else {
                           &mut *(*(*ctx).mech_set).elements.offset(j as
                                                                        isize)
                               as *mut gss_OID_desc_struct
                       }
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    *negResult = 2 as libc::c_int as OM_uint32;
    return 0 as gss_OID;
}
/* private routines for spnego_mechanism */
/*
 * the next two routines make a token buffer suitable for
 * spnego_gss_display_status. These currently take the string
 * in name and place it in the token. Eventually, if
 * spnego_gss_display_status returns valid error messages,
 * these routines will be changes to return the error string.
 */
#[c2rust::src_loc = "3813:1"]
unsafe extern "C" fn make_spnego_token(mut name: *const libc::c_char)
 -> spnego_token_t {
    return gssalloc_strdup(name) as spnego_token_t;
}
#[c2rust::src_loc = "3819:1"]
unsafe extern "C" fn make_err_msg(mut name: *const libc::c_char)
 -> gss_buffer_desc {
    let mut buffer: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    if name.is_null() {
        buffer.length = 0 as libc::c_int as size_t;
        buffer.value = 0 as *mut libc::c_void
    } else {
        buffer.length =
            strlen(name).wrapping_add(1 as libc::c_int as libc::c_ulong);
        buffer.value = make_spnego_token(name)
    }
    return buffer;
}
/*
 * Create the client side spnego token passed back to gss_init_sec_context
 * and eventually up to the application program and over to the server.
 *
 * Use DER rules, definite length method per RFC 2478
 */
#[c2rust::src_loc = "3841:1"]
unsafe extern "C" fn make_spnego_tokenInit_msg(mut spnego_ctx:
                                                   spnego_gss_ctx_id_t,
                                               mut negHintsCompat:
                                                   libc::c_int,
                                               mut mechListMIC: gss_buffer_t,
                                               mut req_flags: OM_uint32,
                                               mut data: gss_buffer_t,
                                               mut sendtoken: send_token_flag,
                                               mut outbuf: gss_buffer_t)
 -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut tlen: libc::c_uint = 0;
    let mut dataLen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut negTokenInitSize: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut negTokenInitSeqSize: libc::c_uint =
        0 as libc::c_int as libc::c_uint;
    let mut negTokenInitContSize: libc::c_uint =
        0 as libc::c_int as libc::c_uint;
    let mut rspTokenSize: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut mechListTokenSize: libc::c_uint =
        0 as libc::c_int as libc::c_uint;
    let mut micTokenSize: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut t: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if outbuf.is_null() { return -(1 as libc::c_int) }
    (*outbuf).length = 0 as libc::c_int as size_t;
    (*outbuf).value = 0 as *mut libc::c_void;
    /* calculate the data length */
    /*
	 * 0xa0 [DER LEN] [mechTypes]
	 */
    mechListTokenSize =
        ((1 as libc::c_int as
              libc::c_uint).wrapping_add(gssint_der_length_size((*spnego_ctx).DER_mechTypes.length
                                                                    as
                                                                    libc::c_uint))
             as
             libc::c_ulong).wrapping_add((*spnego_ctx).DER_mechTypes.length)
            as libc::c_uint;
    dataLen = dataLen.wrapping_add(mechListTokenSize);
    /*
	 * If a token from gss_init_sec_context exists,
	 * add the length of the token + the ASN.1 overhead
	 */
    if !data.is_null() {
        /*
		 * Encoded in final output as:
		 * 0xa2 [DER LEN] 0x04 [DER LEN] [DATA]
		 * -----s--------|--------s2----------
		 */
        rspTokenSize =
            ((1 as libc::c_int as
                  libc::c_uint).wrapping_add(gssint_der_length_size((*data).length
                                                                        as
                                                                        libc::c_uint))
                 as libc::c_ulong).wrapping_add((*data).length) as
                libc::c_uint;
        dataLen =
            dataLen.wrapping_add((1 as libc::c_int as
                                      libc::c_uint).wrapping_add(gssint_der_length_size(rspTokenSize)).wrapping_add(rspTokenSize))
    }
    if !mechListMIC.is_null() {
        /*
		 * Encoded in final output as:
		 * 0xa3 [DER LEN] 0x04 [DER LEN] [DATA]
		 *	--s--     -----tlen------------
		 */
        micTokenSize =
            ((1 as libc::c_int as
                  libc::c_uint).wrapping_add(gssint_der_length_size((*mechListMIC).length
                                                                        as
                                                                        libc::c_uint))
                 as libc::c_ulong).wrapping_add((*mechListMIC).length) as
                libc::c_uint;
        dataLen =
            dataLen.wrapping_add((1 as libc::c_int as
                                      libc::c_uint).wrapping_add(gssint_der_length_size(micTokenSize)).wrapping_add(micTokenSize))
    }
    /*
	 * Add size of DER encoding
	 * [ SEQUENCE { MechTypeList | ReqFLags | Token | mechListMIC } ]
	 *   0x30 [DER_LEN] [data]
	 *
	 */
    negTokenInitContSize = dataLen;
    negTokenInitSeqSize =
        (1 as libc::c_int as
             libc::c_uint).wrapping_add(gssint_der_length_size(dataLen)).wrapping_add(dataLen);
    dataLen = negTokenInitSeqSize;
    /*
	 * negTokenInitSize indicates the bytes needed to
	 * hold the ASN.1 encoding of the entire NegTokenInit
	 * SEQUENCE.
	 * 0xa0 [DER_LEN] + data
	 *
	 */
    negTokenInitSize =
        (1 as libc::c_int as
             libc::c_uint).wrapping_add(gssint_der_length_size(negTokenInitSeqSize)).wrapping_add(negTokenInitSeqSize);
    tlen = g_token_size(gss_mech_spnego, negTokenInitSize) as libc::c_uint;
    t = gssalloc_malloc(tlen as size_t) as *mut libc::c_uchar;
    if t.is_null() { return -(1 as libc::c_int) }
    ptr = t;
    /* create the message */
    ret =
        g_make_token_header(gss_mech_spnego, negTokenInitSize, &mut ptr,
                            tlen); /* NegotiationToken identifier */
    if !(ret != 0) {
        let fresh21 = ptr; /* MechTypeList identifier */
        ptr = ptr.offset(1);
        *fresh21 = 0xa0 as libc::c_int as libc::c_uchar;
        ret = gssint_put_der_length(negTokenInitSeqSize, &mut ptr, tlen);
        if !(ret != 0) {
            let fresh22 = ptr;
            ptr = ptr.offset(1);
            *fresh22 = 0x30 as libc::c_int as libc::c_uchar;
            ret =
                gssint_put_der_length(negTokenInitContSize, &mut ptr,
                                      tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                            as libc::c_long as
                                                            libc::c_int as
                                                            libc::c_uint));
            if !(ret != 0) {
                let fresh23 = ptr;
                ptr = ptr.offset(1);
                *fresh23 =
                    (0xa0 as libc::c_int | 0 as libc::c_int) as libc::c_uchar;
                ret =
                    gssint_put_der_length((*spnego_ctx).DER_mechTypes.length
                                              as libc::c_uint, &mut ptr,
                                          tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                as
                                                                libc::c_long
                                                                as libc::c_int
                                                                as
                                                                libc::c_uint));
                if !(ret != 0) {
                    /* We already encoded the MechSetList */
                    memcpy(ptr as *mut libc::c_void,
                           (*spnego_ctx).DER_mechTypes.value,
                           (*spnego_ctx).DER_mechTypes.length);
                    ptr =
                        ptr.offset((*spnego_ctx).DER_mechTypes.length as
                                       isize);
                    if !data.is_null() {
                        let fresh24 = ptr;
                        ptr = ptr.offset(1);
                        *fresh24 =
                            (0xa0 as libc::c_int | 0x2 as libc::c_int) as
                                libc::c_uchar;
                        ret =
                            gssint_put_der_length(rspTokenSize, &mut ptr,
                                                  tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                        as
                                                                        libc::c_long
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint));
                        if ret != 0 {
                            current_block = 4996352559381616237;
                        } else {
                            ret =
                                put_input_token(&mut ptr, data,
                                                tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                      as
                                                                      libc::c_long
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint));
                            if ret != 0 {
                                current_block = 4996352559381616237;
                            } else { current_block = 980989089337379490; }
                        }
                    } else { current_block = 980989089337379490; }
                    match current_block {
                        4996352559381616237 => { }
                        _ => {
                            if !mechListMIC.is_null() {
                                let fresh25 = ptr;
                                ptr = ptr.offset(1);
                                *fresh25 =
                                    (0xa0 as libc::c_int | 0x3 as libc::c_int)
                                        as libc::c_uchar;
                                ret =
                                    gssint_put_der_length(micTokenSize,
                                                          &mut ptr,
                                                          tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                                as
                                                                                libc::c_long
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint));
                                if !(ret != 0) {
                                    if negHintsCompat != 0 {
                                        ret =
                                            put_neg_hints(&mut ptr,
                                                          mechListMIC,
                                                          tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                                as
                                                                                libc::c_long
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint));
                                        (ret) != 0;
                                    } else {
                                        ret =
                                            put_input_token(&mut ptr,
                                                            mechListMIC,
                                                            tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                                  as
                                                                                  libc::c_long
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint));
                                        (ret) != 0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ret != 0 as libc::c_int {
        if !t.is_null() { free(t as *mut libc::c_void); }
        t = 0 as *mut libc::c_uchar;
        tlen = 0 as libc::c_int as libc::c_uint
    }
    (*outbuf).length = tlen as size_t;
    (*outbuf).value = t as *mut libc::c_void;
    return ret;
}
/*
 * create the server side spnego token passed back to
 * gss_accept_sec_context and eventually up to the application program
 * and over to the client.
 */
#[c2rust::src_loc = "4007:1"]
unsafe extern "C" fn make_spnego_tokenTarg_msg(mut status: OM_uint32,
                                               mut mech_wanted: gss_OID,
                                               mut data: gss_buffer_t,
                                               mut mechListMIC: gss_buffer_t,
                                               mut sendtoken: send_token_flag,
                                               mut outbuf: gss_buffer_t)
 -> libc::c_int {
    let mut current_block: u64;
    let mut tlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ret: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut NegTokenTargSize: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut NegTokenSize: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut rspTokenSize: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut micTokenSize: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dataLen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut t: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if outbuf.is_null() {
        return ((9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) as
                   libc::c_int
    }
    if sendtoken as libc::c_uint ==
           INIT_TOKEN_SEND as libc::c_int as libc::c_uint &&
           mech_wanted.is_null() {
        return ((9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) as
                   libc::c_int
    }
    (*outbuf).length = 0 as libc::c_int as size_t;
    (*outbuf).value = 0 as *mut libc::c_void;
    /*
	 * ASN.1 encoding of the negResult
	 * ENUMERATED type is 3 bytes
	 *  ENUMERATED TAG, Length, Value,
	 * Plus 2 bytes for the CONTEXT id and length.
	 */
    dataLen = 5 as libc::c_int as libc::c_uint;
    /*
	 * calculate data length
	 *
	 * If this is the initial token, include length of
	 * mech_type and the negotiation result fields.
	 */
    if sendtoken as libc::c_uint ==
           INIT_TOKEN_SEND as libc::c_int as libc::c_uint {
        let mut mechlistTokenSize: libc::c_int = 0;
        /*
		 * 1 byte for the CONTEXT ID(0xa0),
		 * 1 byte for the OID ID(0x06)
		 * 1 byte for OID Length field
		 * Plus the rest... (OID Length, OID value)
		 */
        mechlistTokenSize =
            (3 as libc::c_int as
                 libc::c_uint).wrapping_add((*mech_wanted).length).wrapping_add(gssint_der_length_size((*mech_wanted).length))
                as libc::c_int;
        dataLen = dataLen.wrapping_add(mechlistTokenSize as libc::c_uint)
    }
    if !data.is_null() && (*data).length > 0 as libc::c_int as libc::c_ulong {
        /* Length of the inner token */
        rspTokenSize =
            ((1 as libc::c_int as
                  libc::c_uint).wrapping_add(gssint_der_length_size((*data).length
                                                                        as
                                                                        libc::c_uint))
                 as libc::c_ulong).wrapping_add((*data).length) as
                libc::c_uint;
        dataLen = dataLen.wrapping_add(rspTokenSize);
        /* Length of the outer token */
        dataLen =
            dataLen.wrapping_add((1 as libc::c_int as
                                      libc::c_uint).wrapping_add(gssint_der_length_size(rspTokenSize)))
    }
    if !mechListMIC.is_null() {
        /* Length of the inner token */
        micTokenSize =
            ((1 as libc::c_int as
                  libc::c_uint).wrapping_add(gssint_der_length_size((*mechListMIC).length
                                                                        as
                                                                        libc::c_uint))
                 as libc::c_ulong).wrapping_add((*mechListMIC).length) as
                libc::c_uint;
        dataLen = dataLen.wrapping_add(micTokenSize);
        /* Length of the outer token */
        dataLen =
            dataLen.wrapping_add((1 as libc::c_int as
                                      libc::c_uint).wrapping_add(gssint_der_length_size(micTokenSize)))
    }
    /*
	 * Add size of DER encoded:
	 * NegTokenTarg [ SEQUENCE ] of
	 *    NegResult[0] ENUMERATED {
	 *	accept_completed(0),
	 *	accept_incomplete(1),
	 *	reject(2) }
	 *    supportedMech [1] MechType OPTIONAL,
	 *    responseToken [2] OCTET STRING OPTIONAL,
	 *    mechListMIC   [3] OCTET STRING OPTIONAL
	 *
	 * size = data->length + MechListMic + SupportedMech len +
	 *	Result Length + ASN.1 overhead
	 */
    NegTokenTargSize = dataLen;
    dataLen =
        dataLen.wrapping_add((1 as libc::c_int as
                                  libc::c_uint).wrapping_add(gssint_der_length_size(NegTokenTargSize)));
    /*
	 * NegotiationToken [ CHOICE ]{
	 *    negTokenInit  [0]	 NegTokenInit,
	 *    negTokenTarg  [1]	 NegTokenTarg }
	 */
    NegTokenSize = dataLen;
    dataLen =
        dataLen.wrapping_add((1 as libc::c_int as
                                  libc::c_uint).wrapping_add(gssint_der_length_size(NegTokenSize)));
    tlen = dataLen;
    t = gssalloc_malloc(tlen as size_t) as *mut libc::c_uchar;
    if t.is_null() {
        ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    } else {
        ptr = t;
        /*
	 * Indicate that we are sending CHOICE 1
	 * (NegTokenTarg)
	 */
        let fresh26 = ptr;
        ptr = ptr.offset(1);
        *fresh26 =
            (0xa0 as libc::c_int | 0x1 as libc::c_int) as libc::c_uchar;
        if gssint_put_der_length(NegTokenSize, &mut ptr, dataLen) <
               0 as libc::c_int {
            ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        } else {
            let fresh27 = ptr;
            ptr = ptr.offset(1);
            *fresh27 = 0x30 as libc::c_int as libc::c_uchar;
            if gssint_put_der_length(NegTokenTargSize, &mut ptr,
                                     tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                           as libc::c_long as
                                                           libc::c_int as
                                                           libc::c_uint)) <
                   0 as libc::c_int {
                ret = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
            } else {
                /*
	 * First field of the NegTokenTarg SEQUENCE
	 * is the ENUMERATED NegResult.
	 */
                let fresh28 = ptr;
                ptr = ptr.offset(1);
                *fresh28 = 0xa0 as libc::c_int as libc::c_uchar;
                if gssint_put_der_length(3 as libc::c_int as libc::c_uint,
                                         &mut ptr,
                                         tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                               as libc::c_long
                                                               as libc::c_int
                                                               as
                                                               libc::c_uint))
                       < 0 as libc::c_int {
                    ret =
                        (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
                } else if put_negResult(&mut ptr, status,
                                        tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                              as libc::c_long
                                                              as libc::c_int
                                                              as
                                                              libc::c_uint)) <
                              0 as libc::c_int {
                    ret =
                        (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
                } else {
                    if sendtoken as libc::c_uint ==
                           INIT_TOKEN_SEND as libc::c_int as libc::c_uint {
                        /*
		 * Next, is the Supported MechType
		 */
                        let fresh29 = ptr;
                        ptr = ptr.offset(1);
                        *fresh29 =
                            (0xa0 as libc::c_int | 0x1 as libc::c_int) as
                                libc::c_uchar;
                        if gssint_put_der_length((*mech_wanted).length.wrapping_add(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint),
                                                 &mut ptr,
                                                 tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                       as
                                                                       libc::c_long
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint))
                               < 0 as libc::c_int {
                            ret =
                                (9 as libc::c_ulong as OM_uint32) <<
                                    16 as libc::c_int;
                            current_block = 8037724726832295043;
                        } else if put_mech_oid(&mut ptr,
                                               mech_wanted as gss_OID_const,
                                               tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                     as
                                                                     libc::c_long
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
                                      < 0 as libc::c_int {
                            ret =
                                (9 as libc::c_ulong as OM_uint32) <<
                                    16 as libc::c_int;
                            current_block = 8037724726832295043;
                        } else { current_block = 2516253395664191498; }
                    } else { current_block = 2516253395664191498; }
                    match current_block {
                        8037724726832295043 => { }
                        _ => {
                            if !data.is_null() &&
                                   (*data).length >
                                       0 as libc::c_int as libc::c_ulong {
                                let fresh30 = ptr;
                                ptr = ptr.offset(1);
                                *fresh30 =
                                    (0xa0 as libc::c_int | 0x2 as libc::c_int)
                                        as libc::c_uchar;
                                if gssint_put_der_length(rspTokenSize,
                                                         &mut ptr,
                                                         tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                               as
                                                                               libc::c_long
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint))
                                       < 0 as libc::c_int {
                                    ret =
                                        (9 as libc::c_ulong as OM_uint32) <<
                                            16 as libc::c_int;
                                    current_block = 8037724726832295043;
                                } else if put_input_token(&mut ptr, data,
                                                          tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                                as
                                                                                libc::c_long
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint))
                                              < 0 as libc::c_int {
                                    ret =
                                        (9 as libc::c_ulong as OM_uint32) <<
                                            16 as libc::c_int;
                                    current_block = 8037724726832295043;
                                } else {
                                    current_block = 9512719473022792396;
                                }
                            } else { current_block = 9512719473022792396; }
                            match current_block {
                                8037724726832295043 => { }
                                _ => {
                                    if !mechListMIC.is_null() {
                                        let fresh31 = ptr;
                                        ptr = ptr.offset(1);
                                        *fresh31 =
                                            (0xa0 as libc::c_int |
                                                 0x3 as libc::c_int) as
                                                libc::c_uchar;
                                        if gssint_put_der_length(micTokenSize,
                                                                 &mut ptr,
                                                                 tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                                       as
                                                                                       libc::c_long
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint))
                                               < 0 as libc::c_int {
                                            ret =
                                                (9 as libc::c_ulong as
                                                     OM_uint32) <<
                                                    16 as libc::c_int;
                                            current_block =
                                                8037724726832295043;
                                        } else if put_input_token(&mut ptr,
                                                                  mechListMIC,
                                                                  tlen.wrapping_sub(ptr.wrapping_offset_from(t)
                                                                                        as
                                                                                        libc::c_long
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint))
                                                      < 0 as libc::c_int {
                                            ret =
                                                (9 as libc::c_ulong as
                                                     OM_uint32) <<
                                                    16 as libc::c_int;
                                            current_block =
                                                8037724726832295043;
                                        } else {
                                            current_block =
                                                13660591889533726445;
                                        }
                                    } else {
                                        current_block = 13660591889533726445;
                                    }
                                    match current_block {
                                        8037724726832295043 => { }
                                        _ => {
                                            ret =
                                                0 as libc::c_int as
                                                    libc::c_uint
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
    if ret != 0 as libc::c_int as libc::c_uint {
        if !t.is_null() { free(t as *mut libc::c_void); }
    } else {
        (*outbuf).length =
            ptr.wrapping_offset_from(t) as libc::c_long as size_t;
        (*outbuf).value = t as *mut libc::c_void
    }
    return ret as libc::c_int;
}
/* determine size of token */
#[c2rust::src_loc = "4201:1"]
unsafe extern "C" fn g_token_size(mut mech: gss_OID_const,
                                  mut body_size: libc::c_uint)
 -> libc::c_int {
    let mut hdrsize: libc::c_int = 0;
    /*
	 * Initialize the header size to the
	 * MECH_OID byte + the bytes needed to indicate the
	 * length of the OID + the OID itself.
	 *
	 * 0x06 [MECHLENFIELD] MECHDATA
	 */
    hdrsize =
        (1 as libc::c_int as
             libc::c_uint).wrapping_add(gssint_der_length_size((*mech).length)).wrapping_add((*mech).length)
            as libc::c_int;
    /*
	 * Now add the bytes needed for the initial header
	 * token bytes:
	 * 0x60 + [DER_LEN] + HDRSIZE
	 */
    hdrsize =
        (hdrsize as
             libc::c_uint).wrapping_add((1 as libc::c_int as
                                             libc::c_uint).wrapping_add(gssint_der_length_size(body_size.wrapping_add(hdrsize
                                                                                                                          as
                                                                                                                          libc::c_uint))))
            as libc::c_int as libc::c_int;
    return (hdrsize as libc::c_uint).wrapping_add(body_size) as libc::c_int;
}
/*
 * generate token header.
 *
 * Use DER Definite Length method per RFC2478
 * Use of indefinite length encoding will not be compatible
 * with Microsoft or others that actually follow the spec.
 */
#[c2rust::src_loc = "4232:1"]
unsafe extern "C" fn g_make_token_header(mut mech: gss_OID_const,
                                         mut body_size: libc::c_uint,
                                         mut buf: *mut *mut libc::c_uchar,
                                         mut totallen: libc::c_uint)
 -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut hdrsize: libc::c_uint = 0;
    let mut p: *mut libc::c_uchar = *buf;
    hdrsize =
        (1 as libc::c_int as
             libc::c_uint).wrapping_add(gssint_der_length_size((*mech).length)).wrapping_add((*mech).length);
    let fresh32 = *buf;
    *buf = (*buf).offset(1);
    *fresh32 = 0x60 as libc::c_int as libc::c_uchar;
    ret =
        gssint_put_der_length(hdrsize.wrapping_add(body_size), buf, totallen);
    if ret != 0 { return ret }
    let fresh33 = *buf;
    *buf = (*buf).offset(1);
    *fresh33 = 0x6 as libc::c_int as libc::c_uchar;
    ret =
        gssint_put_der_length((*mech).length, buf,
                              totallen.wrapping_sub(p.wrapping_offset_from(*buf)
                                                        as libc::c_long as
                                                        libc::c_int as
                                                        libc::c_uint));
    if ret != 0 { return ret }
    memcpy(*buf as *mut libc::c_void, (*mech).elements,
           (*mech).length as libc::c_ulong);
    *buf = (*buf).offset((*mech).length as isize);
    return 0 as libc::c_int;
}
/*
 * NOTE: This checks that the length returned by
 * gssint_get_der_length() is not greater than the number of octets
 * remaining, even though gssint_get_der_length() already checks, in
 * theory.
 */
#[c2rust::src_loc = "4262:1"]
unsafe extern "C" fn g_get_tag_and_length(mut buf: *mut *mut libc::c_uchar,
                                          mut tag: libc::c_int,
                                          mut buflen: libc::c_uint,
                                          mut outlen: *mut libc::c_uint)
 -> libc::c_int {
    let mut ptr: *mut libc::c_uchar = *buf; /* pessimists, assume failure ! */
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut encoded_len: libc::c_uint = 0;
    let mut tmplen: libc::c_int = 0 as libc::c_int;
    *outlen = 0 as libc::c_int as libc::c_uint;
    if buflen > 1 as libc::c_int as libc::c_uint && *ptr as libc::c_int == tag
       {
        ptr = ptr.offset(1);
        tmplen =
            gssint_get_der_length(&mut ptr,
                                  buflen.wrapping_sub(1 as libc::c_int as
                                                          libc::c_uint),
                                  &mut encoded_len);
        if tmplen < 0 as libc::c_int {
            ret = -(1 as libc::c_int)
        } else if tmplen as libc::c_uint as libc::c_long >
                      buflen as libc::c_long -
                          ptr.wrapping_offset_from(*buf) as libc::c_long {
            ret = -(1 as libc::c_int)
        } else { ret = 0 as libc::c_int }
    }
    *outlen = tmplen as libc::c_uint;
    *buf = ptr;
    return ret;
}
#[c2rust::src_loc = "4288:1"]
unsafe extern "C" fn g_verify_neg_token_init(mut buf_in:
                                                 *mut *mut libc::c_uchar,
                                             mut cur_size: libc::c_uint)
 -> libc::c_int {
    let mut buf: *mut libc::c_uchar = *buf_in;
    let mut endptr: *mut libc::c_uchar = buf.offset(cur_size as isize);
    let mut seqsize: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut bytes: libc::c_uint = 0;
    /*
	 * Verify this is a NegotiationToken type token
	 * - check for a0(context specific identifier)
	 * - get length and verify that enoughd ata exists
	 */
    if g_get_tag_and_length(&mut buf, 0xa0 as libc::c_int, cur_size,
                            &mut bytes) < 0 as libc::c_int {
        return -(2045022964 as libc::c_long) as libc::c_int
    } /* should indicate bytes remaining */
    cur_size = bytes;
    /*
	 * Verify the next piece, it should identify this as
	 * a strucure of type NegTokenInit.
	 */
    let fresh34 = buf;
    buf = buf.offset(1);
    if *fresh34 as libc::c_int == 0x30 as libc::c_int {
        seqsize = gssint_get_der_length(&mut buf, cur_size, &mut bytes);
        if seqsize < 0 as libc::c_int {
            return -(2045022964 as libc::c_long) as libc::c_int
        }
        /*
		 * Make sure we have the entire buffer as described
		 */
        if seqsize as libc::c_long >
               endptr.wrapping_offset_from(buf) as libc::c_long {
            return -(2045022964 as libc::c_long) as libc::c_int
        }
    } else {
        return -(2045022964 as libc::c_long) as libc::c_int
    } /* should indicate bytes remaining */
    cur_size = seqsize as libc::c_uint;
    /*
	 * Verify that the first blob is a sequence of mechTypes
	 */
    let fresh35 = buf;
    buf = buf.offset(1);
    if *fresh35 as libc::c_int == 0xa0 as libc::c_int {
        seqsize = gssint_get_der_length(&mut buf, cur_size, &mut bytes);
        if seqsize < 0 as libc::c_int {
            return -(2045022964 as libc::c_long) as libc::c_int
        }
        /*
		 * Make sure we have the entire buffer as described
		 */
        if seqsize as libc::c_long >
               endptr.wrapping_offset_from(buf) as libc::c_long {
            return -(2045022964 as libc::c_long) as libc::c_int
        }
    } else { return -(2045022964 as libc::c_long) as libc::c_int }
    /*
	 * At this point, *buf should be at the beginning of the
	 * DER encoded list of mech types that are to be negotiated.
	 */
    *buf_in = buf;
    return ret;
}
/* verify token header. */
#[c2rust::src_loc = "4351:1"]
unsafe extern "C" fn g_verify_token_header(mut mech: gss_OID_const,
                                           mut body_size: *mut libc::c_uint,
                                           mut buf_in:
                                               *mut *mut libc::c_uchar,
                                           mut tok_type: libc::c_int,
                                           mut toksize: libc::c_uint)
 -> libc::c_int {
    let mut buf: *mut libc::c_uchar = *buf_in;
    let mut seqsize: libc::c_int = 0;
    let mut toid: gss_OID_desc =
        gss_OID_desc{length: 0,
                     elements:
                         0 as *const libc::c_void as *mut libc::c_void,};
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut bytes: libc::c_uint = 0;
    let fresh36 = toksize;
    toksize = toksize.wrapping_sub(1);
    if fresh36 < 1 as libc::c_int as libc::c_uint {
        return -(2045022964 as libc::c_long) as libc::c_int
    }
    let fresh37 = buf;
    buf = buf.offset(1);
    if *fresh37 as libc::c_int != 0x60 as libc::c_int {
        return -(2045022964 as libc::c_long) as libc::c_int
    }
    seqsize = gssint_get_der_length(&mut buf, toksize, &mut bytes);
    if seqsize < 0 as libc::c_int {
        return -(2045022964 as libc::c_long) as libc::c_int
    }
    if (seqsize as libc::c_uint).wrapping_add(bytes) != toksize {
        return -(2045022964 as libc::c_long) as libc::c_int
    }
    let fresh38 = toksize;
    toksize = toksize.wrapping_sub(1);
    if fresh38 < 1 as libc::c_int as libc::c_uint {
        return -(2045022964 as libc::c_long) as libc::c_int
    }
    let fresh39 = buf;
    buf = buf.offset(1);
    if *fresh39 as libc::c_int != 0x6 as libc::c_int {
        return -(2045022964 as libc::c_long) as libc::c_int
    }
    let fresh40 = toksize;
    toksize = toksize.wrapping_sub(1);
    if fresh40 < 1 as libc::c_int as libc::c_uint {
        return -(2045022964 as libc::c_long) as libc::c_int
    }
    let fresh41 = buf;
    buf = buf.offset(1);
    toid.length = *fresh41 as OM_uint32;
    if toksize < toid.length {
        return -(2045022964 as libc::c_long) as libc::c_int
    } else { toksize = toksize.wrapping_sub(toid.length) }
    toid.elements = buf as *mut libc::c_void;
    buf = buf.offset(toid.length as isize);
    if !(toid.length == (*mech).length &&
             memcmp(toid.elements, (*mech).elements,
                    toid.length as libc::c_ulong) == 0 as libc::c_int) {
        ret = -(2045022965 as libc::c_long) as libc::c_int
    }
    /*
	 * G_WRONG_MECH is not returned immediately because it's more important
	 * to return G_BAD_TOK_HEADER if the token header is in fact bad
	 */
    if toksize < 2 as libc::c_int as libc::c_uint {
        return -(2045022964 as libc::c_long) as libc::c_int
    } else {
        toksize = toksize.wrapping_sub(2 as libc::c_int as libc::c_uint)
    }
    if ret == 0 { *buf_in = buf; *body_size = toksize }
    return ret;
}
/*
 * Return non-zero if the oid is one of the kerberos mech oids,
 * otherwise return zero.
 *
 * N.B. There are 3 oids that represent the kerberos mech:
 * RFC-specified GSS_MECH_KRB5_OID,
 * Old pre-RFC   GSS_MECH_KRB5_OLD_OID,
 * Incorrect MS  GSS_MECH_KRB5_WRONG_OID
 */
#[c2rust::src_loc = "4426:1"]
unsafe extern "C" fn is_kerb_mech(mut oid: gss_OID) -> libc::c_int {
    let mut answer: libc::c_int = 0 as libc::c_int;
    let mut minor: OM_uint32 = 0;
    extern "C" {
        #[no_mangle]
        pub static gss_mech_set_krb5_both: *const gss_OID_set_desc;
    }
    gss_test_oid_set_member(&mut minor, oid,
                            gss_mech_set_krb5_both as gss_OID_set,
                            &mut answer);
    return answer;
}
unsafe extern "C" fn run_static_initializers() {
    gss_mech_spnego = spnego_oids.as_ptr().offset(0 as libc::c_int as isize);
    spnego_oidsets =
        [{
             let mut init =
                 gss_OID_set_desc_struct{count: 1 as libc::c_int as size_t,
                                         elements:
                                             (spnego_oids.as_ptr() as
                                                  gss_OID).offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),};
             init
         }];
    gss_mech_set_spnego =
        spnego_oidsets.as_ptr().offset(0 as libc::c_int as isize)
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
