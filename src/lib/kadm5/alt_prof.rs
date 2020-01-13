use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:32"]
pub mod krb5_h {
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
    /* Originally used to recognize AFS and default salts.  No longer used. */
    #[c2rust::src_loc = "225:1"]
    pub type krb5_pointer = *mut libc::c_void;
    /* *
 * Hook function for inspecting or overriding KDC replies.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  code            Status of KDC communication
 * @param [in]  realm           The realm the reply was received from
 * @param [in]  message         The message sent to the realm's KDC
 * @param [in]  reply           The reply received from the KDC
 * @param [out] new_reply_out   Optional replacement reply
 *
 * If @a code is zero, @a reply contains the reply received from the KDC.  The
 * hook function may return an error code to simulate an error, may synthesize
 * a different reply by setting @a new_reply_out, or may simply return
 * successfully to do nothing.
 *
 * If @a code is non-zero, KDC communication failed and @a reply should be
 * ignored.  The hook function may return @a code or a different error code, or
 * may synthesize a reply by setting @a new_reply_out and return successfully.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_reply_out, to ensure that it can be freed correctly by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
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
    /* *
 * Hook function for inspecting or modifying messages sent to KDCs.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  realm           The realm the message will be sent to
 * @param [in]  message         The original message to be sent to the KDC
 * @param [out] new_message_out Optional replacement message to be sent
 * @param [out] reply_out       Optional synthetic reply
 *
 * If the hook function returns an error code, the KDC communication will be
 * aborted and the error code will be returned to the library operation which
 * initiated the communication.
 *
 * If the hook function sets @a reply_out, @a message will not be sent to the
 * KDC, and the given reply will used instead.
 *
 * If the hook function sets @a new_message_out, the given message will be sent
 * to the KDC in place of @a message.
 *
 * If the hook function returns successfully without setting either output,
 * @a message will be sent to the KDC normally.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_message_out or @a reply_out, to ensure that it can be freed correctly
 * by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
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
    /* *
 * A wrapper for passing information to a @c krb5_trace_callback.
 *
 * Currently, it only contains the formatted message as determined
 * the the format string and arguments of the tracing macro, but it
 * may be extended to contain more fields in the future.
 */
    #[c2rust::src_loc = "8387:1"]
    pub type krb5_trace_info = _krb5_trace_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8387:16"]
    pub struct _krb5_trace_info {
        pub message: *const libc::c_char,
    }
    /*
 * Prompter enhancements
 */
/* * Prompt for password */
    /* * Prompt for new password (during password change) */
    /* * Prompt for new password again */
    /* * Prompt for preauthentication data (such as an OTP value) */
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
 * Free a string allocated by a krb5 function.
 *
 * @param [in] context          Library context
 * @param [in] val              String to be freed
 *
 * @version New in 1.10
 */
        #[no_mangle]
        #[c2rust::src_loc = "4778:1"]
        pub fn krb5_free_string(context: krb5_context,
                                val: *mut libc::c_char);
        /* *
 * Retrieve the default realm.
 *
 * @param [in]  context         Library context
 * @param [out] lrealm          Default realm name
 *
 * Retrieves the default realm to be used if no user-specified realm is
 * available.
 *
 * Use krb5_free_default_realm() to free @a lrealm when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4887:1"]
        pub fn krb5_get_default_realm(context: krb5_context,
                                      lrealm: *mut *mut libc::c_char)
         -> krb5_error_code;
        /* *
 * Canonicalize a hostname, possibly using name service.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Input hostname
 * @param [out] canonhost_out   Canonicalized hostname
 *
 * This function canonicalizes orig_hostname, possibly using name service
 * lookups if configuration permits.  Use krb5_free_string() to free @a
 * canonhost_out when it is no longer needed.
 *
 * @version New in 1.15
 */
        #[no_mangle]
        #[c2rust::src_loc = "4928:1"]
        pub fn krb5_expand_hostname(context: krb5_context,
                                    host: *const libc::c_char,
                                    canonhost_out: *mut *mut libc::c_char)
         -> krb5_error_code;
        /* str_conv.c */
/* *
 * Convert a string to an encryption type.
 *
 * @param [in]  string          String to convert to an encryption type
 * @param [out] enctypep        Encryption type
 *
 * @retval 0  Success; otherwise - EINVAL
 */
        #[no_mangle]
        #[c2rust::src_loc = "6266:1"]
        pub fn krb5_string_to_enctype(string: *mut libc::c_char,
                                      enctypep: *mut krb5_enctype)
         -> krb5_error_code;
        /* *
 * Convert a string to a timestamp.
 *
 * @param [in]  string          String to be converted
 * @param [out] timestampp      Pointer to timestamp
 *
 * @retval 0  Success; otherwise - EINVAL
 */
        #[no_mangle]
        #[c2rust::src_loc = "6299:1"]
        pub fn krb5_string_to_timestamp(string: *mut libc::c_char,
                                        timestampp: *mut krb5_timestamp)
         -> krb5_error_code;
        /* *
 * Convert a string to a delta time value.
 *
 * @param [in]  string          String to be converted
 * @param [out] deltatp         Delta time to be filled in
 *
 * @retval 0  Success; otherwise - KRB5_DELTAT_BADFORMAT
 */
        #[no_mangle]
        #[c2rust::src_loc = "6310:1"]
        pub fn krb5_string_to_deltat(string: *mut libc::c_char,
                                     deltatp: *mut krb5_deltat)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:32"]
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
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::kdb_log_h::_kdb_log_context;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
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
        #[c2rust::src_loc = "2164:1"]
        pub fn krb5_get_default_config_files(filenames:
                                                 *mut *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2166:1"]
        pub fn krb5_free_config_files(filenames: *mut *mut libc::c_char);
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:38"]
pub mod kdb_log_h {
    /* Log header magic # */
    /* Kerberos database version no. */
    /* # of updates in log */
    /* Timestamp of first update */
    /* Timestamp of last update */
    /* First serial # in the update log */
    /* Last serial # in the update log */
    /* State of update log */
    /* Block size of each element */
    /* Update entry magic # */
    /* Serial # of entry */
    /* Timestamp of update */
    /* Is the entry committed or not */
    /* Size of update entry */
    /* Address of kdb_incr_update_t */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:16"]
    pub struct _kdb_log_context {
        pub iproprole: iprop_role,
        pub ulog: *mut kdb_hlog_t,
        pub ulogentries: uint32_t,
        pub ulogfd: libc::c_int,
    }
    #[c2rust::src_loc = "81:1"]
    pub type kdb_hlog_t = kdb_hlog;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct kdb_hlog {
        pub kdb_hmagic: uint32_t,
        pub db_version_num: uint16_t,
        pub kdb_num: uint32_t,
        pub kdb_first_time: kdbe_time_t,
        pub kdb_last_time: kdbe_time_t,
        pub kdb_first_sno: kdb_sno_t,
        pub kdb_last_sno: kdb_sno_t,
        pub kdb_state: uint16_t,
        pub kdb_block: uint16_t,
    }
    use super::iprop_hdr_h::iprop_role;
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::iprop_h::{kdbe_time_t, kdb_sno_t};
    /* !_KDB_LOG_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:38"]
pub mod iprop_h {
    #[c2rust::src_loc = "22:1"]
    pub type kdb_sno_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:8"]
    pub struct kdbe_time_t {
        pub seconds: uint32_t,
        pub useconds: uint32_t,
    }
    use super::stdint_uintn_h::uint32_t;
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop_hdr.h:38"]
pub mod iprop_hdr_h {
    #[c2rust::src_loc = "32:1"]
    pub type iprop_role = libc::c_uint;
    #[c2rust::src_loc = "35:5"]
    pub const IPROP_REPLICA: iprop_role = 2;
    #[c2rust::src_loc = "34:5"]
    pub const IPROP_MASTER: iprop_role = 1;
    #[c2rust::src_loc = "33:5"]
    pub const IPROP_NULL: iprop_role = 0;
    /* !_IPROP_HDR_H */
    /*
 * Full resync dump versioning
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:32"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:32"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:32"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    /* path as C string */
    #[c2rust::src_loc = "43:1"]
    pub type const_profile_filespec_list_t = *const libc::c_char;
    use super::krb5_h::_profile_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "51:1"]
        pub fn profile_init_path(filelist: const_profile_filespec_list_t,
                                 ret_profile: *mut profile_t) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn profile_release(profile: profile_t);
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn profile_get_values(profile: profile_t,
                                  names: *const *const libc::c_char,
                                  ret_values: *mut *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn profile_free_list(list: *mut *mut libc::c_char);
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:32"]
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
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:34"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_enctype, krb5_int32};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:34"]
pub mod admin_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _kadm5_config_params {
        pub mask: libc::c_long,
        pub realm: *mut libc::c_char,
        pub kadmind_port: libc::c_int,
        pub kpasswd_port: libc::c_int,
        pub admin_server: *mut libc::c_char,
        pub dbname: *mut libc::c_char,
        pub acl_file: *mut libc::c_char,
        pub dict_file: *mut libc::c_char,
        pub mkey_from_kbd: libc::c_int,
        pub stash_file: *mut libc::c_char,
        pub mkey_name: *mut libc::c_char,
        pub enctype: krb5_enctype,
        pub max_life: krb5_deltat,
        pub max_rlife: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub flags: krb5_flags,
        pub keysalts: *mut krb5_key_salt_tuple,
        pub num_keysalts: krb5_int32,
        pub kvno: krb5_kvno,
        pub iprop_enabled: libc::c_int,
        pub iprop_ulogsize: uint32_t,
        pub iprop_poll_time: krb5_deltat,
        pub iprop_logfile: *mut libc::c_char,
        pub iprop_port: libc::c_int,
        pub iprop_resync_timeout: libc::c_int,
        pub kadmind_listen: *mut libc::c_char,
        pub kpasswd_listen: *mut libc::c_char,
        pub iprop_listen: *mut libc::c_char,
    }
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    use super::krb5_h::{krb5_enctype, krb5_deltat, krb5_timestamp, krb5_flags,
                        krb5_int32, krb5_kvno};
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src = "/usr/include/ctype.h:37"]
pub mod ctype_h {
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
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
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:32"]
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
        #[no_mangle]
        #[c2rust::src_loc = "888:1"]
        pub fn krb5int_strlcat(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/stdio.h:32"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "399:12"]
        pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn secure_getenv(__name: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/strings.h:32"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "116:12"]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "272:15"]
        pub fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_ulong;
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
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:35"]
pub mod adm_proto_h {
    use super::krb5_h::{krb5_flags, krb5_error_code, krb5_boolean,
                        krb5_int32};
    use super::kdb_h::krb5_key_salt_tuple;
    extern "C" {
        /* str_conv.c */
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn krb5_flagspec_to_mask(_: *const libc::c_char,
                                     _: *mut krb5_flags, _: *mut krb5_flags)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn krb5_string_to_keysalts(_: *const libc::c_char,
                                       _: *const libc::c_char,
                                       _: *const libc::c_char,
                                       _: krb5_boolean,
                                       _: *mut *mut krb5_key_salt_tuple,
                                       _: *mut krb5_int32) -> krb5_error_code;
    }
    /* KRB5_ADM_PROTO_H__ */
}
pub use self::types_h::{__uint16_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _profile_t,
                       krb5_free_string, krb5_get_default_realm,
                       krb5_expand_hostname, krb5_string_to_enctype,
                       krb5_string_to_timestamp, krb5_string_to_deltat};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_get_default_config_files,
                         krb5_free_config_files};
pub use self::kdb_log_h::{_kdb_log_context, kdb_hlog_t, kdb_hlog};
pub use self::iprop_h::{kdb_sno_t, kdbe_time_t};
pub use self::iprop_hdr_h::{iprop_role, IPROP_REPLICA, IPROP_MASTER,
                            IPROP_NULL};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, const_profile_filespec_list_t,
                          profile_init_path, profile_release,
                          profile_get_values, profile_free_list};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_init_dynamic,
                         k5_buf_add, k5_buf_status, k5_buf_free};
pub use self::kdb_h::{__krb5_key_salt_tuple, krb5_key_salt_tuple};
pub use self::admin_h::{_kadm5_config_params, kadm5_config_params};
pub use self::ctype_h::{_ISspace, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
use self::k5_platform_h::{krb5int_strlcpy, krb5int_strlcat};
use self::stdio_h::{sscanf, asprintf, snprintf};
use self::stdlib_h::{secure_getenv, free, calloc, atoi};
use self::strings_h::strcasecmp;
use self::string_h::{strlen, strcspn, strchr, strdup, memset, memmove,
                     memcpy};
use self::adm_proto_h::{krb5_flagspec_to_mask, krb5_string_to_keysalts};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kadm5/alt_prof.c */
/*
 * Copyright 1995,2001,2008,2009 by the Massachusetts Institute of Technology.
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
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/* Implement alternate profile file handling. */
#[c2rust::src_loc = "40:1"]
unsafe extern "C" fn copy_key_salt_tuple(mut ksalt: *mut krb5_key_salt_tuple,
                                         mut len: krb5_int32)
 -> *mut krb5_key_salt_tuple {
    let mut knew: *mut krb5_key_salt_tuple = 0 as *mut krb5_key_salt_tuple;
    knew =
        calloc(len as libc::c_ulong,
               ::std::mem::size_of::<krb5_key_salt_tuple>() as libc::c_ulong)
            as *mut krb5_key_salt_tuple;
    if knew.is_null() { return 0 as *mut krb5_key_salt_tuple }
    memcpy(knew as *mut libc::c_void, ksalt as *const libc::c_void,
           (len as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_key_salt_tuple>()
                                                as libc::c_ulong));
    return knew;
}
/* alt_prof.c */
/*
 * krb5_aprof_init()        - Initialize alternate profile context.
 *
 * Parameters:
 *        fname             - default file name of the profile.
 *        envname           - environment variable which can override fname
 *        acontextp         - Pointer to opaque context for alternate profile
 *
 * Returns:
 *        error codes from profile_init()
 */
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn krb5_aprof_init(mut fname: *mut libc::c_char,
                                         mut envname: *mut libc::c_char,
                                         mut acontextp: *mut krb5_pointer)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut profile: profile_t = 0 as *mut _profile_t;
    let mut kdc_config: *const libc::c_char = 0 as *const libc::c_char;
    let mut filenames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    ret = krb5_get_default_config_files(&mut filenames);
    if ret != 0 { return ret }
    if envname.is_null() ||
           { kdc_config = secure_getenv(envname); kdc_config.is_null() } {
        kdc_config = fname
    }
    k5_buf_init_dynamic(&mut buf);
    if !kdc_config.is_null() { k5_buf_add(&mut buf, kdc_config); }
    i = 0 as libc::c_int;
    while !(*filenames.offset(i as isize)).is_null() {
        if buf.len > 0 as libc::c_int as libc::c_ulong {
            k5_buf_add(&mut buf,
                       b":\x00" as *const u8 as *const libc::c_char);
        }
        k5_buf_add(&mut buf, *filenames.offset(i as isize));
        i += 1
    }
    krb5_free_config_files(filenames);
    if k5_buf_status(&mut buf) != 0 as libc::c_int {
        return 12 as libc::c_int
    }
    profile = 0 as *mut libc::c_void as profile_t;
    ret =
        profile_init_path(buf.data as const_profile_filespec_list_t,
                          &mut profile) as krb5_error_code;
    k5_buf_free(&mut buf);
    if ret != 0 { return ret }
    *acontextp = profile as krb5_pointer;
    return 0 as libc::c_int;
}
/*
 * krb5_aprof_getvals()     - Get values from alternate profile.
 *
 * Parameters:
 *        acontext          - opaque context for alternate profile.
 *        hierarchy         - hierarchy of value to retrieve.
 *        retdata           - Returned data values.
 *
 * Returns:
 *         error codes from profile_get_values()
 */
#[no_mangle]
#[c2rust::src_loc = "109:1"]
pub unsafe extern "C" fn krb5_aprof_getvals(mut acontext: krb5_pointer,
                                            mut hierarchy:
                                                *mut *const libc::c_char,
                                            mut retdata:
                                                *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    return profile_get_values(acontext as profile_t, hierarchy, retdata) as
               krb5_error_code;
}
/*
 * krb5_aprof_get_boolean()
 *
 * Parameters:
 *        acontext          - opaque context for alternate profile
 *        hierarchy         - hierarchy of value to retrieve
 *        retdata           - Returned data value
 * Returns:
 *        error codes
 */
#[c2rust::src_loc = "127:1"]
unsafe extern "C" fn string_to_boolean(mut string: *const libc::c_char,
                                       mut out: *mut krb5_boolean)
 -> krb5_error_code {
    static mut yes: [*const libc::c_char; 6] =
        [b"y\x00" as *const u8 as *const libc::c_char,
         b"yes\x00" as *const u8 as *const libc::c_char,
         b"true\x00" as *const u8 as *const libc::c_char,
         b"t\x00" as *const u8 as *const libc::c_char,
         b"1\x00" as *const u8 as *const libc::c_char,
         b"on\x00" as *const u8 as *const libc::c_char];
    static mut no: [*const libc::c_char; 7] =
        [b"n\x00" as *const u8 as *const libc::c_char,
         b"no\x00" as *const u8 as *const libc::c_char,
         b"false\x00" as *const u8 as *const libc::c_char,
         b"f\x00" as *const u8 as *const libc::c_char,
         b"nil\x00" as *const u8 as *const libc::c_char,
         b"0\x00" as *const u8 as *const libc::c_char,
         b"off\x00" as *const u8 as *const libc::c_char];
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*const libc::c_char; 6]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                   as libc::c_ulong) {
        if strcasecmp(string, yes[i as usize]) == 0 {
            *out = 1 as libc::c_int as krb5_boolean;
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*const libc::c_char; 7]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                   as libc::c_ulong) {
        if strcasecmp(string, no[i as usize]) == 0 {
            *out = 0 as libc::c_int as krb5_boolean;
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return -(1429577700 as libc::c_long) as krb5_error_code;
}
#[no_mangle]
#[c2rust::src_loc = "150:1"]
pub unsafe extern "C" fn krb5_aprof_get_boolean(mut acontext: krb5_pointer,
                                                mut hierarchy:
                                                    *mut *const libc::c_char,
                                                mut uselast: libc::c_int,
                                                mut retdata:
                                                    *mut krb5_boolean)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut values: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut valp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_int = 0;
    let mut val: krb5_boolean = 0;
    ret = krb5_aprof_getvals(acontext, hierarchy, &mut values);
    if ret != 0 { return ret }
    idx = 0 as libc::c_int;
    if uselast != 0 {
        while !(*values.offset(idx as isize)).is_null() { idx += 1 }
        idx -= 1
    }
    valp = *values.offset(idx as isize);
    ret = string_to_boolean(valp, &mut val);
    profile_free_list(values);
    if ret != 0 { return ret }
    *retdata = val;
    return 0 as libc::c_int;
}
/*
 * krb5_aprof_get_deltat()  - Get a delta time value from the alternate
 *                            profile.
 *
 * Parameters:
 *        acontext          - opaque context for alternate profile.
 *        hierarchy         - hierarchy of value to retrieve.
 *        uselast           - if true, use last value, otherwise use first
 *                            value found.
 *        deltatp           - returned delta time value.
 *
 * Returns:
 *        error codes from profile_get_values()
 *        error codes from krb5_string_to_deltat()
 */
#[no_mangle]
#[c2rust::src_loc = "192:1"]
pub unsafe extern "C" fn krb5_aprof_get_deltat(mut acontext: krb5_pointer,
                                               mut hierarchy:
                                                   *mut *const libc::c_char,
                                               mut uselast: krb5_boolean,
                                               mut deltatp: *mut krb5_deltat)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut values: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut valp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idx: libc::c_int = 0;
    ret = krb5_aprof_getvals(acontext, hierarchy, &mut values);
    if ret != 0 { return ret }
    idx = 0 as libc::c_int;
    if uselast != 0 {
        idx = 0 as libc::c_int;
        while !(*values.offset(idx as isize)).is_null() { idx += 1 }
        idx -= 1
    }
    valp = *values.offset(idx as isize);
    ret = krb5_string_to_deltat(valp, deltatp);
    profile_free_list(values);
    return ret;
}
/*
 * krb5_aprof_get_string()  - Get a string value from the alternate profile.
 *
 * Parameters:
 *        acontext          - opaque context for alternate profile.
 *        hierarchy         - hierarchy of value to retrieve.
 *        uselast           - if true, use last value, otherwise use first
 *                            value found.
 *        stringp           - returned string value.
 *
 * Returns:
 *         error codes from profile_get_values()
 */
#[no_mangle]
#[c2rust::src_loc = "229:1"]
pub unsafe extern "C" fn krb5_aprof_get_string(mut acontext: krb5_pointer,
                                               mut hierarchy:
                                                   *mut *const libc::c_char,
                                               mut uselast: krb5_boolean,
                                               mut stringp:
                                                   *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut values: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut lastidx: libc::c_int = 0;
    ret = krb5_aprof_getvals(acontext, hierarchy, &mut values);
    if ret != 0 { return ret }
    lastidx = 0 as libc::c_int;
    while !(*values.offset(lastidx as isize)).is_null() { lastidx += 1 }
    lastidx -= 1;
    /* Excise the entry we want from the null-terminated list,
     * and free up the rest. */
    if uselast != 0 {
        *stringp = *values.offset(lastidx as isize);
        let ref mut fresh0 = *values.offset(lastidx as isize);
        *fresh0 = 0 as *mut libc::c_char
    } else {
        *stringp = *values.offset(0 as libc::c_int as isize);
        let ref mut fresh1 = *values.offset(0 as libc::c_int as isize);
        *fresh1 = *values.offset(lastidx as isize);
        let ref mut fresh2 = *values.offset(lastidx as isize);
        *fresh2 = 0 as *mut libc::c_char
    }
    profile_free_list(values);
    return 0 as libc::c_int;
}
/*
 * krb5_aprof_get_string_all() - When the attr identified by "hierarchy" is
 *                               specified multiple times, concatenate all of
 *                               its string values from the alternate profile,
 *                               separated with spaces.
 *
 * Parameters:
 *        acontext             - opaque context for alternate profile.
 *        hierarchy            - hierarchy of value to retrieve.
 *        stringp              - Returned string value.
 *
 * Returns:
 *        error codes from profile_get_values() or ENOMEM
 *        Caller is responsible for deallocating stringp buffer
 */
#[no_mangle]
#[c2rust::src_loc = "274:1"]
pub unsafe extern "C" fn krb5_aprof_get_string_all(mut acontext: krb5_pointer,
                                                   mut hierarchy:
                                                       *mut *const libc::c_char,
                                                   mut stringp:
                                                       *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut values: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut buf_size: size_t = 0 as libc::c_int as size_t;
    ret = krb5_aprof_getvals(acontext, hierarchy, &mut values);
    if ret != 0 { return ret }
    buf_size =
        strlen(*values.offset(0 as libc::c_int as
                                  isize)).wrapping_add(3 as libc::c_int as
                                                           libc::c_ulong);
    idx = 1 as libc::c_int;
    while !(*values.offset(idx as isize)).is_null() {
        buf_size =
            (buf_size as
                 libc::c_ulong).wrapping_add(strlen(*values.offset(idx as
                                                                       isize)).wrapping_add(3
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong))
                as size_t as size_t;
        idx += 1
    }
    *stringp =
        calloc(1 as libc::c_int as libc::c_ulong, buf_size) as
            *mut libc::c_char;
    if (*stringp).is_null() {
        profile_free_list(values);
        return 12 as libc::c_int
    }
    krb5int_strlcpy(*stringp, *values.offset(0 as libc::c_int as isize),
                    buf_size);
    idx = 1 as libc::c_int;
    while !(*values.offset(idx as isize)).is_null() {
        krb5int_strlcat(*stringp,
                        b" \x00" as *const u8 as *const libc::c_char,
                        buf_size);
        krb5int_strlcat(*stringp, *values.offset(idx as isize), buf_size);
        idx += 1
    }
    profile_free_list(values);
    return 0 as libc::c_int;
}
/*
 * krb5_aprof_get_int32()   - Get a 32-bit integer value from the alternate
 *                            profile.
 *
 * Parameters:
 *        acontext          - opaque context for alternate profile.
 *        hierarchy         - hierarchy of value to retrieve.
 *        uselast           - if true, use last value, otherwise use first
 *                            value found.
 *        intp              - returned 32-bit integer value.
 *
 * Returns:
 *        error codes from profile_get_values()
 *        EINVAL            - value is not an integer
 */
#[no_mangle]
#[c2rust::src_loc = "322:1"]
pub unsafe extern "C" fn krb5_aprof_get_int32(mut acontext: krb5_pointer,
                                              mut hierarchy:
                                                  *mut *const libc::c_char,
                                              mut uselast: krb5_boolean,
                                              mut intp: *mut krb5_int32)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut values: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut idx: libc::c_int = 0;
    ret = krb5_aprof_getvals(acontext, hierarchy, &mut values);
    if ret != 0 { return ret }
    idx = 0 as libc::c_int;
    if uselast != 0 {
        idx = 0 as libc::c_int;
        while !(*values.offset(idx as isize)).is_null() { idx += 1 }
        idx -= 1
    }
    if sscanf(*values.offset(idx as isize),
              b"%d\x00" as *const u8 as *const libc::c_char, intp) !=
           1 as libc::c_int {
        ret = 22 as libc::c_int
    }
    profile_free_list(values);
    return ret;
}
/*
 * krb5_aprof_finish()      - Finish alternate profile context.
 *
 * Parameter:
 *        acontext          - opaque context for alternate profile.
 *
 * Returns:
 *        0 on success, something else on failure.
 */
#[no_mangle]
#[c2rust::src_loc = "356:1"]
pub unsafe extern "C" fn krb5_aprof_finish(mut acontext: krb5_pointer)
 -> krb5_error_code {
    profile_release(acontext as profile_t);
    return 0 as libc::c_int;
}
/*
 * Returns nonzero if it found something to copy; the caller may still need to
 * check the output field or mask to see if the copy (allocation) was
 * successful.  Returns zero if nothing was found to copy, and thus the caller
 * may want to apply some default heuristic.  If the default action is just to
 * use a fixed, compiled-in string, supply it as the default value here and
 * ignore the return value.
 */
#[c2rust::src_loc = "371:1"]
unsafe extern "C" fn get_string_param(mut param_out: *mut *mut libc::c_char,
                                      mut param_in: *mut libc::c_char,
                                      mut mask_out: *mut libc::c_long,
                                      mut mask_in: libc::c_long,
                                      mut mask_bit: libc::c_long,
                                      mut aprofile: krb5_pointer,
                                      mut hierarchy: *mut *const libc::c_char,
                                      mut config_name: *const libc::c_char,
                                      mut default_value: *const libc::c_char)
 -> libc::c_int {
    let mut svalue: *mut libc::c_char = 0 as *mut libc::c_char;
    let ref mut fresh3 = *hierarchy.offset(2 as libc::c_int as isize);
    *fresh3 = config_name;
    if mask_in & mask_bit != 0 {
        *param_out = strdup(param_in);
        if !(*param_out).is_null() { *mask_out |= mask_bit }
        return 1 as libc::c_int
    } else if !aprofile.is_null() &&
                  krb5_aprof_get_string(aprofile, hierarchy,
                                        1 as libc::c_int as krb5_boolean,
                                        &mut svalue) == 0 {
        *param_out = svalue;
        *mask_out |= mask_bit;
        return 1 as libc::c_int
    } else if !default_value.is_null() {
        *param_out = strdup(default_value);
        if !(*param_out).is_null() { *mask_out |= mask_bit }
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/*
 * Similar, for (host-order) port number, if not already set in the output
 * field; default_value == 0 means no default.
 */
#[c2rust::src_loc = "403:1"]
unsafe extern "C" fn get_port_param(mut param_out: *mut libc::c_int,
                                    mut param_in: libc::c_int,
                                    mut mask_out: *mut libc::c_long,
                                    mut mask_in: libc::c_long,
                                    mut mask_bit: libc::c_long,
                                    mut aprofile: krb5_pointer,
                                    mut hierarchy: *mut *const libc::c_char,
                                    mut config_name: *const libc::c_char,
                                    mut default_value: libc::c_int) {
    let mut ivalue: krb5_int32 = 0;
    if *mask_out & mask_bit != 0 { return }
    let ref mut fresh4 = *hierarchy.offset(2 as libc::c_int as isize);
    *fresh4 = config_name;
    if mask_in & mask_bit != 0 {
        *mask_out |= mask_bit;
        *param_out = param_in
    } else if !aprofile.is_null() &&
                  krb5_aprof_get_int32(aprofile, hierarchy,
                                       1 as libc::c_int as krb5_boolean,
                                       &mut ivalue) == 0 {
        *param_out = ivalue;
        *mask_out |= mask_bit
    } else if default_value != 0 {
        *param_out = default_value;
        *mask_out |= mask_bit
    };
}
/*
 * Similar, for delta_t; default is required.
 */
#[c2rust::src_loc = "429:1"]
unsafe extern "C" fn get_deltat_param(mut param_out: *mut krb5_deltat,
                                      mut param_in: krb5_deltat,
                                      mut mask_out: *mut libc::c_long,
                                      mut mask_in: libc::c_long,
                                      mut mask_bit: libc::c_long,
                                      mut aprofile: krb5_pointer,
                                      mut hierarchy: *mut *const libc::c_char,
                                      mut config_name: *const libc::c_char,
                                      mut default_value: krb5_deltat) {
    let mut dtvalue: krb5_deltat = 0;
    let ref mut fresh5 = *hierarchy.offset(2 as libc::c_int as isize);
    *fresh5 = config_name;
    if mask_in & mask_bit != 0 {
        *mask_out |= mask_bit;
        *param_out = param_in
    } else if !aprofile.is_null() &&
                  krb5_aprof_get_deltat(aprofile, hierarchy,
                                        1 as libc::c_int as krb5_boolean,
                                        &mut dtvalue) == 0 {
        *param_out = dtvalue;
        *mask_out |= mask_bit
    } else { *param_out = default_value; *mask_out |= mask_bit };
}
/*
 * Parse out the port number from an admin_server setting.  Modify server to
 * contain just the hostname or address.  If a port is given, set *port, and
 * set the appropriate bit in *mask.
 */
#[c2rust::src_loc = "456:1"]
unsafe extern "C" fn parse_admin_server_port(mut server: *mut libc::c_char,
                                             mut port: *mut libc::c_int,
                                             mut mask: *mut libc::c_long) {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut portstr: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Allow the name or addr to be enclosed in brackets, for IPv6 addrs. */
    if *server as libc::c_int == '[' as i32 &&
           {
               end =
                   strchr(server.offset(1 as libc::c_int as isize),
                          ']' as i32);
               !end.is_null()
           } {
        portstr =
            if *end.offset(1 as libc::c_int as isize) as libc::c_int ==
                   ':' as i32 {
                end.offset(2 as libc::c_int as isize)
            } else { 0 as *mut libc::c_char };
        /* Shift the bracketed name or address back into server. */
        memmove(server as *mut libc::c_void,
                server.offset(1 as libc::c_int as isize) as
                    *const libc::c_void,
                end.wrapping_offset_from(server.offset(1 as libc::c_int as
                                                           isize)) as
                    libc::c_long as libc::c_ulong);
        *end.offset(-(1 as libc::c_int as isize)) =
            '\u{0}' as i32 as libc::c_char
    } else {
        /* Terminate the name at the colon, if any. */
        end =
            server.offset(strcspn(server,
                                  b":\x00" as *const u8 as
                                      *const libc::c_char) as isize);
        portstr =
            if *end as libc::c_int == ':' as i32 {
                end.offset(1 as libc::c_int as isize)
            } else { 0 as *mut libc::c_char };
        *end = '\u{0}' as i32 as libc::c_char
    }
    /* If we found a port string, parse it and set the appropriate bit. */
    if !portstr.is_null() {
        *port = atoi(portstr);
        *mask |= 0x4000 as libc::c_int as libc::c_long
    };
}
/*
 * Function: kadm5_get_config_params
 *
 * Purpose: Merge configuration parameters provided by the caller with values
 * specified in configuration files and with default values.
 *
 * Arguments:
 *
 *        context     (r) krb5_context to use
 *        profile     (r) profile file to use
 *        envname     (r) envname that contains a profile name to
 *                        override profile
 *        params_in   (r) params structure containing user-supplied
 *                        values, or NULL
 *        params_out  (w) params structure to be filled in
 *
 * Effects:
 *
 * The fields and mask of params_out are filled in with values obtained from
 * params_in, the specified profile, and default values.  Only and all fields
 * specified in params_out->mask are set.  The context of params_out must be
 * freed with kadm5_free_config_params.
 *
 * params_in and params_out may be the same pointer.  However, all pointers in
 * params_in for which the mask is set will be re-assigned to newly copied
 * versions, overwriting the old pointer value.
 */
#[no_mangle]
#[c2rust::src_loc = "508:1"]
pub unsafe extern "C" fn kadm5_get_config_params(mut context: krb5_context,
                                                 mut use_kdc_config:
                                                     libc::c_int,
                                                 mut params_in:
                                                     *mut kadm5_config_params,
                                                 mut params_out:
                                                     *mut kadm5_config_params)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut envname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lrealm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut svalue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aprofile: krb5_pointer = 0 as krb5_pointer;
    let mut hierarchy: [*const libc::c_char; 4] =
        [0 as *const libc::c_char; 4];
    let mut ivalue: krb5_int32 = 0;
    let mut params: kadm5_config_params =
        kadm5_config_params{mask: 0,
                            realm: 0 as *mut libc::c_char,
                            kadmind_port: 0,
                            kpasswd_port: 0,
                            admin_server: 0 as *mut libc::c_char,
                            dbname: 0 as *mut libc::c_char,
                            acl_file: 0 as *mut libc::c_char,
                            dict_file: 0 as *mut libc::c_char,
                            mkey_from_kbd: 0,
                            stash_file: 0 as *mut libc::c_char,
                            mkey_name: 0 as *mut libc::c_char,
                            enctype: 0,
                            max_life: 0,
                            max_rlife: 0,
                            expiration: 0,
                            flags: 0,
                            keysalts: 0 as *mut krb5_key_salt_tuple,
                            num_keysalts: 0,
                            kvno: 0,
                            iprop_enabled: 0,
                            iprop_ulogsize: 0,
                            iprop_poll_time: 0,
                            iprop_logfile: 0 as *mut libc::c_char,
                            iprop_port: 0,
                            iprop_resync_timeout: 0,
                            kadmind_listen: 0 as *mut libc::c_char,
                            kpasswd_listen: 0 as *mut libc::c_char,
                            iprop_listen: 0 as *mut libc::c_char,};
    let mut empty_params: kadm5_config_params =
        kadm5_config_params{mask: 0,
                            realm: 0 as *mut libc::c_char,
                            kadmind_port: 0,
                            kpasswd_port: 0,
                            admin_server: 0 as *mut libc::c_char,
                            dbname: 0 as *mut libc::c_char,
                            acl_file: 0 as *mut libc::c_char,
                            dict_file: 0 as *mut libc::c_char,
                            mkey_from_kbd: 0,
                            stash_file: 0 as *mut libc::c_char,
                            mkey_name: 0 as *mut libc::c_char,
                            enctype: 0,
                            max_life: 0,
                            max_rlife: 0,
                            expiration: 0,
                            flags: 0,
                            keysalts: 0 as *mut krb5_key_salt_tuple,
                            num_keysalts: 0,
                            kvno: 0,
                            iprop_enabled: 0,
                            iprop_ulogsize: 0,
                            iprop_poll_time: 0,
                            iprop_logfile: 0 as *mut libc::c_char,
                            iprop_port: 0,
                            iprop_resync_timeout: 0,
                            kadmind_listen: 0 as *mut libc::c_char,
                            kpasswd_listen: 0 as *mut libc::c_char,
                            iprop_listen: 0 as *mut libc::c_char,};
    let mut bvalue: krb5_boolean = 0;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    memset(&mut params as *mut kadm5_config_params as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_config_params>() as libc::c_ulong);
    memset(&mut empty_params as *mut kadm5_config_params as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_config_params>() as libc::c_ulong);
    if params_in.is_null() { params_in = &mut empty_params }
    if (*params_in).mask & 0x1 as libc::c_int as libc::c_long != 0 {
        params.realm = strdup((*params_in).realm);
        lrealm = params.realm;
        if params.realm.is_null() {
            ret = 12 as libc::c_int;
            current_block = 15046855868199870974;
        } else {
            params.mask |= 0x1 as libc::c_int as libc::c_long;
            current_block = 17833034027772472439;
        }
    } else {
        ret = krb5_get_default_realm(context, &mut lrealm);
        if ret != 0 {
            current_block = 15046855868199870974;
        } else {
            params.realm = lrealm;
            params.mask |= 0x1 as libc::c_int as libc::c_long;
            current_block = 17833034027772472439;
        }
    }
    match current_block {
        17833034027772472439 => {
            if (*params_in).mask & 0x20000000 as libc::c_int as libc::c_long
                   != 0 {
                params.kvno = (*params_in).kvno;
                params.mask |= 0x20000000 as libc::c_int as libc::c_long
            }
            /*
     * XXX These defaults should to work on both client and
     * server.  kadm5_get_config_params can be implemented as a
     * wrapper function in each library that provides correct
     * defaults for NULL values.
     */
            if use_kdc_config != 0 {
                filename =
                    b"/usr/local/var/krb5kdc/kdc.conf\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char;
                envname =
                    b"KRB5_KDC_PROFILE\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char
            } else {
                filename =
                    b"/etc/krb5.conf:/usr/local/etc/krb5.conf\x00" as
                        *const u8 as *const libc::c_char as *mut libc::c_char;
                envname =
                    b"KRB5_CONFIG\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            }
            if (*context).profile_secure == 1 as libc::c_int as libc::c_uint {
                envname = 0 as *mut libc::c_char
            }
            ret = krb5_aprof_init(filename, envname, &mut aprofile);
            if !(ret != 0) {
                /* Initialize realm parameters. */
                hierarchy[0 as libc::c_int as usize] =
                    b"realms\x00" as *const u8 as *const libc::c_char;
                hierarchy[1 as libc::c_int as usize] = lrealm;
                hierarchy[3 as libc::c_int as usize] =
                    0 as *const libc::c_char;
                /* Get the value for the admin server. */
                get_string_param(&mut params.admin_server,
                                 (*params_in).admin_server, &mut params.mask,
                                 (*params_in).mask,
                                 0x10000 as libc::c_int as libc::c_long,
                                 aprofile, hierarchy.as_mut_ptr(),
                                 b"admin_server\x00" as *const u8 as
                                     *const libc::c_char,
                                 0 as *const libc::c_char);
                if params.mask & 0x10000 as libc::c_int as libc::c_long != 0 {
                    parse_admin_server_port(params.admin_server,
                                            &mut params.kadmind_port,
                                            &mut params.mask);
                }
                /* Get the value for the database. */
                get_string_param(&mut params.dbname, (*params_in).dbname,
                                 &mut params.mask, (*params_in).mask,
                                 0x2 as libc::c_int as libc::c_long, aprofile,
                                 hierarchy.as_mut_ptr(),
                                 b"database_name\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"/usr/local/var/krb5kdc/principal\x00" as
                                     *const u8 as *const libc::c_char);
                /* Get the name of the acl file. */
                get_string_param(&mut params.acl_file, (*params_in).acl_file,
                                 &mut params.mask, (*params_in).mask,
                                 0x2000 as libc::c_int as libc::c_long,
                                 aprofile, hierarchy.as_mut_ptr(),
                                 b"acl_file\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"/usr/local/var/krb5kdc/kadm5.acl\x00" as
                                     *const u8 as *const libc::c_char);
                /* Get the name of the dict file. */
                get_string_param(&mut params.dict_file,
                                 (*params_in).dict_file, &mut params.mask,
                                 (*params_in).mask,
                                 0x20000 as libc::c_int as libc::c_long,
                                 aprofile, hierarchy.as_mut_ptr(),
                                 b"dict_file\x00" as *const u8 as
                                     *const libc::c_char,
                                 0 as *const libc::c_char);
                /* Get the kadmind listen addresses. */
                get_string_param(&mut params.kadmind_listen,
                                 (*params_in).kadmind_listen,
                                 &mut params.mask, (*params_in).mask,
                                 0x1000 as libc::c_int as libc::c_long,
                                 aprofile, hierarchy.as_mut_ptr(),
                                 b"kadmind_listen\x00" as *const u8 as
                                     *const libc::c_char,
                                 0 as *const libc::c_char);
                get_string_param(&mut params.kpasswd_listen,
                                 (*params_in).kpasswd_listen,
                                 &mut params.mask, (*params_in).mask,
                                 0x800000 as libc::c_int as libc::c_long,
                                 aprofile, hierarchy.as_mut_ptr(),
                                 b"kpasswd_listen\x00" as *const u8 as
                                     *const libc::c_char,
                                 0 as *const libc::c_char);
                get_string_param(&mut params.iprop_listen,
                                 (*params_in).iprop_listen, &mut params.mask,
                                 (*params_in).mask,
                                 0x80000000 as libc::c_uint as libc::c_long,
                                 aprofile, hierarchy.as_mut_ptr(),
                                 b"iprop_listen\x00" as *const u8 as
                                     *const libc::c_char,
                                 0 as *const libc::c_char);
                /* Get the value for the kadmind port. */
                get_port_param(&mut params.kadmind_port,
                               (*params_in).kadmind_port, &mut params.mask,
                               (*params_in).mask,
                               0x4000 as libc::c_int as libc::c_long,
                               aprofile, hierarchy.as_mut_ptr(),
                               b"kadmind_port\x00" as *const u8 as
                                   *const libc::c_char, 749 as libc::c_int);
                /* Get the value for the kpasswd port. */
                get_port_param(&mut params.kpasswd_port,
                               (*params_in).kpasswd_port, &mut params.mask,
                               (*params_in).mask,
                               0x80000 as libc::c_int as libc::c_long,
                               aprofile, hierarchy.as_mut_ptr(),
                               b"kpasswd_port\x00" as *const u8 as
                                   *const libc::c_char, 464 as libc::c_int);
                /* Get the value for the master key name. */
                get_string_param(&mut params.mkey_name,
                                 (*params_in).mkey_name, &mut params.mask,
                                 (*params_in).mask,
                                 0x4 as libc::c_int as libc::c_long, aprofile,
                                 hierarchy.as_mut_ptr(),
                                 b"master_key_name\x00" as *const u8 as
                                     *const libc::c_char,
                                 0 as *const libc::c_char);
                /* Get the value for the master key type. */
                hierarchy[2 as libc::c_int as usize] =
                    b"master_key_type\x00" as *const u8 as
                        *const libc::c_char;
                if (*params_in).mask & 0x200 as libc::c_int as libc::c_long !=
                       0 {
                    params.mask |= 0x200 as libc::c_int as libc::c_long;
                    params.enctype = (*params_in).enctype
                } else if !aprofile.is_null() &&
                              krb5_aprof_get_string(aprofile,
                                                    hierarchy.as_mut_ptr(),
                                                    1 as libc::c_int as
                                                        krb5_boolean,
                                                    &mut svalue) == 0 {
                    if krb5_string_to_enctype(svalue, &mut params.enctype) ==
                           0 {
                        params.mask |= 0x200 as libc::c_int as libc::c_long;
                        free(svalue as *mut libc::c_void);
                    }
                } else {
                    params.mask |= 0x200 as libc::c_int as libc::c_long;
                    params.enctype = 0x12 as libc::c_int
                }
                /* Get the value for mkey_from_kbd. */
                if (*params_in).mask & 0x40000 as libc::c_int as libc::c_long
                       != 0 {
                    params.mask |= 0x40000 as libc::c_int as libc::c_long;
                    params.mkey_from_kbd = (*params_in).mkey_from_kbd
                }
                /* Get the value for the stashfile. */
                get_string_param(&mut params.stash_file,
                                 (*params_in).stash_file, &mut params.mask,
                                 (*params_in).mask,
                                 0x100 as libc::c_int as libc::c_long,
                                 aprofile, hierarchy.as_mut_ptr(),
                                 b"key_stash_file\x00" as *const u8 as
                                     *const libc::c_char,
                                 0 as *const libc::c_char);
                /* Get the value for maximum ticket lifetime. */
                get_deltat_param(&mut params.max_life, (*params_in).max_life,
                                 &mut params.mask, (*params_in).mask,
                                 0x8 as libc::c_int as libc::c_long, aprofile,
                                 hierarchy.as_mut_ptr(),
                                 b"max_life\x00" as *const u8 as
                                     *const libc::c_char,
                                 24 as libc::c_int * 60 as libc::c_int *
                                     60 as libc::c_int);
                /* 1 day */
                /* Get the value for maximum renewable ticket lifetime. */
                get_deltat_param(&mut params.max_rlife,
                                 (*params_in).max_rlife, &mut params.mask,
                                 (*params_in).mask,
                                 0x10 as libc::c_int as libc::c_long,
                                 aprofile, hierarchy.as_mut_ptr(),
                                 b"max_renewable_life\x00" as *const u8 as
                                     *const libc::c_char, 0 as libc::c_int);
                /* Get the value for the default principal expiration */
                hierarchy[2 as libc::c_int as usize] =
                    b"default_principal_expiration\x00" as *const u8 as
                        *const libc::c_char;
                if (*params_in).mask & 0x20 as libc::c_int as libc::c_long !=
                       0 {
                    params.mask |= 0x20 as libc::c_int as libc::c_long;
                    params.expiration = (*params_in).expiration
                } else if !aprofile.is_null() &&
                              krb5_aprof_get_string(aprofile,
                                                    hierarchy.as_mut_ptr(),
                                                    1 as libc::c_int as
                                                        krb5_boolean,
                                                    &mut svalue) == 0 {
                    if krb5_string_to_timestamp(svalue,
                                                &mut params.expiration) == 0 {
                        params.mask |= 0x20 as libc::c_int as libc::c_long;
                        free(svalue as *mut libc::c_void);
                    }
                } else {
                    params.mask |= 0x20 as libc::c_int as libc::c_long;
                    params.expiration = 0 as libc::c_int
                }
                /* Get the value for the default principal flags */
                hierarchy[2 as libc::c_int as usize] =
                    b"default_principal_flags\x00" as *const u8 as
                        *const libc::c_char;
                if (*params_in).mask & 0x40 as libc::c_int as libc::c_long !=
                       0 {
                    params.mask |= 0x40 as libc::c_int as libc::c_long;
                    params.flags = (*params_in).flags
                } else if !aprofile.is_null() &&
                              krb5_aprof_get_string(aprofile,
                                                    hierarchy.as_mut_ptr(),
                                                    1 as libc::c_int as
                                                        krb5_boolean,
                                                    &mut svalue) == 0 {
                    sp = svalue;
                    params.flags = 0 as libc::c_int;
                    while !sp.is_null() {
                        ep = strchr(sp, ',' as i32);
                        if !ep.is_null() ||
                               { ep = strchr(sp, ' ' as i32); !ep.is_null() }
                               ||
                               { ep = strchr(sp, '\t' as i32); !ep.is_null() }
                           {
                            /* Fill in trailing whitespace of sp. */
                            tp = ep.offset(-(1 as libc::c_int as isize));
                            while *(*__ctype_b_loc()).offset(*tp as
                                                                 libc::c_uchar
                                                                 as
                                                                 libc::c_int
                                                                 as isize) as
                                      libc::c_int &
                                      _ISspace as libc::c_int as
                                          libc::c_ushort as libc::c_int != 0
                                      && tp > sp {
                                *tp = '\u{0}' as i32 as libc::c_char;
                                tp = tp.offset(-1)
                            }
                            *ep = '\u{0}' as i32 as libc::c_char;
                            ep = ep.offset(1);
                            /* Skip over trailing whitespace of ep. */
                            while *(*__ctype_b_loc()).offset(*ep as
                                                                 libc::c_uchar
                                                                 as
                                                                 libc::c_int
                                                                 as isize) as
                                      libc::c_int &
                                      _ISspace as libc::c_int as
                                          libc::c_ushort as libc::c_int != 0
                                      && *ep as libc::c_int != '\u{0}' as i32
                                  {
                                ep = ep.offset(1)
                            }
                        }
                        /* Convert this flag. */
                        if krb5_flagspec_to_mask(sp, &mut params.flags,
                                                 &mut params.flags) != 0 {
                            break ;
                        }
                        sp = ep
                    }
                    if sp.is_null() {
                        params.mask |= 0x40 as libc::c_int as libc::c_long
                    }
                    free(svalue as *mut libc::c_void);
                } else {
                    params.mask |= 0x40 as libc::c_int as libc::c_long;
                    params.flags = 0 as libc::c_int
                }
                /* Get the value for the supported enctype/salttype matrix. */
                hierarchy[2 as libc::c_int as usize] =
                    b"supported_enctypes\x00" as *const u8 as
                        *const libc::c_char;
                if (*params_in).mask & 0x8000 as libc::c_int as libc::c_long
                       != 0 {
                    if !(*params_in).keysalts.is_null() {
                        params.keysalts =
                            copy_key_salt_tuple((*params_in).keysalts,
                                                (*params_in).num_keysalts);
                        if !params.keysalts.is_null() {
                            params.mask |=
                                0x8000 as libc::c_int as libc::c_long;
                            params.num_keysalts = (*params_in).num_keysalts
                        }
                    } else {
                        params.mask |= 0x8000 as libc::c_int as libc::c_long;
                        params.keysalts = 0 as *mut krb5_key_salt_tuple;
                        params.num_keysalts = (*params_in).num_keysalts
                    }
                    current_block = 17336970397495664729;
                } else {
                    svalue = 0 as *mut libc::c_char;
                    if !aprofile.is_null() {
                        krb5_aprof_get_string(aprofile,
                                              hierarchy.as_mut_ptr(),
                                              1 as libc::c_int as
                                                  krb5_boolean, &mut svalue);
                    }
                    if svalue.is_null() {
                        svalue =
                            strdup(b"aes256-cts-hmac-sha1-96:normal aes128-cts-hmac-sha1-96:normal\x00"
                                       as *const u8 as *const libc::c_char)
                    }
                    if svalue.is_null() {
                        ret = 12 as libc::c_int;
                        current_block = 15046855868199870974;
                    } else {
                        params.keysalts = 0 as *mut krb5_key_salt_tuple;
                        params.num_keysalts = 0 as libc::c_int;
                        krb5_string_to_keysalts(svalue,
                                                0 as *const libc::c_char,
                                                0 as *const libc::c_char,
                                                0 as libc::c_int as
                                                    krb5_boolean,
                                                &mut params.keysalts,
                                                &mut params.num_keysalts);
                        if params.num_keysalts != 0 {
                            params.mask |=
                                0x8000 as libc::c_int as libc::c_long
                        }
                        free(svalue as *mut libc::c_void);
                        current_block = 17336970397495664729;
                    }
                }
                match current_block {
                    15046855868199870974 => { }
                    _ => {
                        hierarchy[2 as libc::c_int as usize] =
                            b"iprop_enable\x00" as *const u8 as
                                *const libc::c_char;
                        params.iprop_enabled = 0 as libc::c_int;
                        params.mask |=
                            0x1000000 as libc::c_int as libc::c_long;
                        if (*params_in).mask &
                               0x1000000 as libc::c_int as libc::c_long != 0 {
                            params.mask |=
                                0x1000000 as libc::c_int as libc::c_long;
                            params.iprop_enabled = (*params_in).iprop_enabled
                        } else if !aprofile.is_null() &&
                                      krb5_aprof_get_boolean(aprofile,
                                                             hierarchy.as_mut_ptr(),
                                                             1 as libc::c_int,
                                                             &mut bvalue) == 0
                         {
                            params.iprop_enabled = bvalue as libc::c_int;
                            params.mask |=
                                0x1000000 as libc::c_int as libc::c_long
                        }
                        if get_string_param(&mut params.iprop_logfile,
                                            (*params_in).iprop_logfile,
                                            &mut params.mask,
                                            (*params_in).mask,
                                            0x8000000 as libc::c_int as
                                                libc::c_long, aprofile,
                                            hierarchy.as_mut_ptr(),
                                            b"iprop_logfile\x00" as *const u8
                                                as *const libc::c_char,
                                            0 as *const libc::c_char) == 0 {
                            if params.mask &
                                   0x2 as libc::c_int as libc::c_long != 0 {
                                if asprintf(&mut params.iprop_logfile as
                                                *mut *mut libc::c_char,
                                            b"%s.ulog\x00" as *const u8 as
                                                *const libc::c_char,
                                            params.dbname) >= 0 as libc::c_int
                                   {
                                    params.mask |=
                                        0x8000000 as libc::c_int as
                                            libc::c_long
                                }
                            }
                        }
                        get_port_param(&mut params.iprop_port,
                                       (*params_in).iprop_port,
                                       &mut params.mask, (*params_in).mask,
                                       0x10000000 as libc::c_int as
                                           libc::c_long, aprofile,
                                       hierarchy.as_mut_ptr(),
                                       b"iprop_port\x00" as *const u8 as
                                           *const libc::c_char,
                                       0 as libc::c_int);
                        /* 5 min for large KDBs */
                        get_deltat_param(&mut params.iprop_resync_timeout,
                                         (*params_in).iprop_resync_timeout,
                                         &mut params.mask, (*params_in).mask,
                                         0x40000000 as libc::c_int as
                                             libc::c_long, aprofile,
                                         hierarchy.as_mut_ptr(),
                                         b"iprop_resync_timeout\x00" as
                                             *const u8 as *const libc::c_char,
                                         60 as libc::c_int *
                                             5 as libc::c_int);
                        hierarchy[2 as libc::c_int as usize] =
                            b"iprop_master_ulogsize\x00" as *const u8 as
                                *const libc::c_char;
                        params.iprop_ulogsize =
                            1000 as libc::c_int as uint32_t;
                        params.mask |=
                            0x2000000 as libc::c_int as libc::c_long;
                        if (*params_in).mask &
                               0x2000000 as libc::c_int as libc::c_long != 0 {
                            params.mask |=
                                0x2000000 as libc::c_int as libc::c_long;
                            params.iprop_ulogsize =
                                (*params_in).iprop_ulogsize
                        } else if !aprofile.is_null() &&
                                      krb5_aprof_get_int32(aprofile,
                                                           hierarchy.as_mut_ptr(),
                                                           1 as libc::c_int as
                                                               krb5_boolean,
                                                           &mut ivalue) == 0 {
                            if ivalue <= 0 as libc::c_int {
                                params.iprop_ulogsize =
                                    1000 as libc::c_int as uint32_t
                            } else {
                                params.iprop_ulogsize = ivalue as uint32_t
                            }
                            params.mask |=
                                0x2000000 as libc::c_int as libc::c_long
                        }
                        get_deltat_param(&mut params.iprop_poll_time,
                                         (*params_in).iprop_poll_time,
                                         &mut params.mask, (*params_in).mask,
                                         0x4000000 as libc::c_int as
                                             libc::c_long, aprofile,
                                         hierarchy.as_mut_ptr(),
                                         b"iprop_replica_poll\x00" as
                                             *const u8 as *const libc::c_char,
                                         -(1 as libc::c_int));
                        if params.iprop_poll_time == -(1 as libc::c_int) {
                            get_deltat_param(&mut params.iprop_poll_time,
                                             (*params_in).iprop_poll_time,
                                             &mut params.mask,
                                             (*params_in).mask,
                                             0x4000000 as libc::c_int as
                                                 libc::c_long, aprofile,
                                             hierarchy.as_mut_ptr(),
                                             b"iprop_slave_poll\x00" as
                                                 *const u8 as
                                                 *const libc::c_char,
                                             2 as libc::c_int *
                                                 60 as libc::c_int);
                        }
                        *params_out = params
                    }
                }
            }
        }
        _ => { }
    }
    krb5_aprof_finish(aprofile);
    if ret != 0 {
        kadm5_free_config_params(context, &mut params);
        (*params_out).mask = 0 as libc::c_int as libc::c_long
    }
    return ret;
}
/*
 * kadm5_free_config_params()        - Free data allocated by above.
 */
#[no_mangle]
#[c2rust::src_loc = "827:1"]
pub unsafe extern "C" fn kadm5_free_config_params(mut context: krb5_context,
                                                  mut params:
                                                      *mut kadm5_config_params)
 -> krb5_error_code {
    if params.is_null() { return 0 as libc::c_int }
    free((*params).dbname as *mut libc::c_void);
    free((*params).mkey_name as *mut libc::c_void);
    free((*params).stash_file as *mut libc::c_void);
    free((*params).keysalts as *mut libc::c_void);
    free((*params).admin_server as *mut libc::c_void);
    free((*params).dict_file as *mut libc::c_void);
    free((*params).acl_file as *mut libc::c_void);
    free((*params).realm as *mut libc::c_void);
    free((*params).iprop_logfile as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kadm5/admin.h */
/*
 * Copyright 2001, 2008 by the Massachusetts Institute of Technology.
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
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
/*
 * This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
/*
 * Successful return code
 */
/*
 * Field masks
 */
/* kadm5_principal_ent_t */
/* version 2 masks */
/* Novell */
/* all but KEY_DATA, TL_DATA, LOAD */
/* kadm5_policy_ent_t */
/* kadm5_config_params */
/*#define KADM5_CONFIG_ADMIN_KEYTAB       0x00000080*/
/*
 * permission bits
 */
/*
 * API versioning constants
 */
/* version 2 fields */
/* no longer used */
/* version 3 fields */
/* version 4 fields */
/*
 * Data structure returned by kadm5_get_config_params()
 */
/* Novell */ /* ABI change? */
/* Deprecated except for db2 backwards compatibility.  Don't add
       new uses except as fallbacks for parameters that should be
       specified in the database module section of the config
       file.  */
/*    char *            iprop_server;*/
/*
 * functions
 */
#[no_mangle]
#[c2rust::src_loc = "844:1"]
pub unsafe extern "C" fn kadm5_get_admin_service_name(mut ctx: krb5_context,
                                                      mut realm_in:
                                                          *mut libc::c_char,
                                                      mut admin_name:
                                                          *mut libc::c_char,
                                                      mut maxlen: size_t)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut params_in: kadm5_config_params =
        kadm5_config_params{mask: 0,
                            realm: 0 as *mut libc::c_char,
                            kadmind_port: 0,
                            kpasswd_port: 0,
                            admin_server: 0 as *mut libc::c_char,
                            dbname: 0 as *mut libc::c_char,
                            acl_file: 0 as *mut libc::c_char,
                            dict_file: 0 as *mut libc::c_char,
                            mkey_from_kbd: 0,
                            stash_file: 0 as *mut libc::c_char,
                            mkey_name: 0 as *mut libc::c_char,
                            enctype: 0,
                            max_life: 0,
                            max_rlife: 0,
                            expiration: 0,
                            flags: 0,
                            keysalts: 0 as *mut krb5_key_salt_tuple,
                            num_keysalts: 0,
                            kvno: 0,
                            iprop_enabled: 0,
                            iprop_ulogsize: 0,
                            iprop_poll_time: 0,
                            iprop_logfile: 0 as *mut libc::c_char,
                            iprop_port: 0,
                            iprop_resync_timeout: 0,
                            kadmind_listen: 0 as *mut libc::c_char,
                            kpasswd_listen: 0 as *mut libc::c_char,
                            iprop_listen: 0 as *mut libc::c_char,};
    let mut params_out: kadm5_config_params =
        kadm5_config_params{mask: 0,
                            realm: 0 as *mut libc::c_char,
                            kadmind_port: 0,
                            kpasswd_port: 0,
                            admin_server: 0 as *mut libc::c_char,
                            dbname: 0 as *mut libc::c_char,
                            acl_file: 0 as *mut libc::c_char,
                            dict_file: 0 as *mut libc::c_char,
                            mkey_from_kbd: 0,
                            stash_file: 0 as *mut libc::c_char,
                            mkey_name: 0 as *mut libc::c_char,
                            enctype: 0,
                            max_life: 0,
                            max_rlife: 0,
                            expiration: 0,
                            flags: 0,
                            keysalts: 0 as *mut krb5_key_salt_tuple,
                            num_keysalts: 0,
                            kvno: 0,
                            iprop_enabled: 0,
                            iprop_ulogsize: 0,
                            iprop_poll_time: 0,
                            iprop_logfile: 0 as *mut libc::c_char,
                            iprop_port: 0,
                            iprop_resync_timeout: 0,
                            kadmind_listen: 0 as *mut libc::c_char,
                            kpasswd_listen: 0 as *mut libc::c_char,
                            iprop_listen: 0 as *mut libc::c_char,};
    let mut canonhost: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(&mut params_in as *mut kadm5_config_params as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_config_params>() as libc::c_ulong);
    memset(&mut params_out as *mut kadm5_config_params as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_config_params>() as libc::c_ulong);
    params_in.mask |= 0x1 as libc::c_int as libc::c_long;
    params_in.realm = realm_in;
    ret =
        kadm5_get_config_params(ctx, 0 as libc::c_int, &mut params_in,
                                &mut params_out);
    if ret != 0 { return ret }
    if params_out.mask & 0x10000 as libc::c_int as libc::c_long == 0 {
        ret = 43787574 as libc::c_long as krb5_error_code
    } else {
        ret =
            krb5_expand_hostname(ctx, params_out.admin_server,
                                 &mut canonhost);
        if !(ret != 0) {
            if strlen(canonhost).wrapping_add(::std::mem::size_of::<[libc::c_char; 8]>()
                                                  as libc::c_ulong) > maxlen {
                ret = 12 as libc::c_int
            } else {
                snprintf(admin_name, maxlen,
                         b"kadmin/%s\x00" as *const u8 as *const libc::c_char,
                         canonhost);
            }
        }
    }
    krb5_free_string(ctx, canonhost);
    kadm5_free_config_params(ctx, &mut params_out);
    return ret;
}
