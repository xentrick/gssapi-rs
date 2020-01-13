use ::libc;
use ::c2rust_bitfields;
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
    /* *
 * Unparse an encoded PAC into a new handle.
 *
 * @param [in]  context         Library context
 * @param [in]  ptr             PAC buffer
 * @param [in]  len             Length of @a ptr
 * @param [out] pac             PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Verify a PAC.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 *
 * This function validates @a pac against the supplied @a server, @a privsvr,
 * @a principal and @a authtime.  If @a principal is NULL, the principal and
 * authtime are not verified.  If @a server or @a privsvr is NULL, the
 * corresponding checksum is not verified.
 *
 * If successful, @a pac is marked as verified.
 *
 * @note A checksum mismatch can occur if the PAC was copied from a cross-realm
 * TGT by an ignorant KDC; also macOS Server Open Directory (as of 10.6)
 * generates PACs with no server checksum at all.  One should consider not
 * failing the whole authentication because of this reason, but, instead,
 * treating the ticket as if it did not contain a PAC or marking the PAC
 * information as non-verified.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Verify a PAC, possibly from a specified realm.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 * @param [in] with_realm       If true, expect the realm of @a principal
 *
 * This function is similar to krb5_pac_verify(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field is
 * expected to include the realm of @a principal as well as the name.  This
 * flag is necessary to verify PACs in cross-realm S4U2Self referral TGTs.
 *
 * @version New in 1.17
 */
    /* *
 * Sign a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Expected principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [out] data            Signed PAC encoding
 *
 * This function signs @a pac using the keys @a server_key and @a privsvr_key
 * and returns the signed encoding in @a data.  @a pac is modified to include
 * the server and KDC checksum buffers.  Use krb5_free_data_contents() to free
 * @a data when it is no longer needed.
 *
 * @version New in 1.10
 */
    /* *
 * Sign a PAC, possibly with a specified realm.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [in]  with_realm      If true, include the realm of @a principal
 * @param [out] data            Signed PAC encoding
 *
 * This function is similar to krb5_pac_sign(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field of the
 * signed PAC will include the realm of @a principal as well as the name.  This
 * flag is necessary to generate PACs for cross-realm S4U2Self referrals.
 *
 * @version New in 1.17
 */
    /*
 * Read client information from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] authtime_out    Authentication timestamp (NULL if not needed)
 * @param [out] princname_out   Client account name
 *
 * Read the PAC_CLIENT_INFO buffer in @a pac.  Place the client account name as
 * a string in @a princname_out.  If @a authtime_out is not NULL, place the
 * initial authentication timestamp in @a authtime_out.
 *
 * @retval 0 on success, ENOENT if no PAC_CLIENT_INFO buffer is present in @a
 * pac, ERANGE if the buffer contains invalid lengths.
 *
 * @version New in 1.18
 */
    /* *
 * Allow the appplication to override the profile's allow_weak_crypto setting.
 *
 * @param [in] context          Library context
 * @param [in] enable           Boolean flag
 *
 * This function allows an application to override the allow_weak_crypto
 * setting.  It is primarily for use by aklog.
 *
 * @retval 0  (always)
 */
    /* *
 * A wrapper for passing information to a @c krb5_trace_callback.
 *
 * Currently, it only contains the formatted message as determined
 * the the format string and arguments of the tracing macro, but it
 * may be extended to contain more fields in the future.
 */
    /* *
 * Specify a callback function for trace events.
 *
 * @param [in] context          Library context
 * @param [in] fn               Callback function
 * @param [in] cb_data          Callback data
 *
 * Specify a callback for trace events occurring in krb5 operations performed
 * within @a context.  @a fn will be invoked with @a context as the first
 * argument, @a cb_data as the last argument, and a pointer to a
 * krb5_trace_info as the second argument.  If the trace callback is reset via
 * this function or @a context is destroyed, @a fn will be invoked with a NULL
 * second argument so it can clean up @a cb_data.  Supply a NULL value for @a
 * fn to disable trace callbacks within @a context.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @return Returns KRB5_TRACE_NOSUPP if tracing is not supported in the library
 * (unless @a fn is NULL).
 */
    /* *
 * Specify a file name for directing trace events.
 *
 * @param [in] context          Library context
 * @param [in] filename         File name
 *
 * Open @a filename for appending (creating it, if necessary) and set up a
 * callback to write trace events to it.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @retval KRB5_TRACE_NOSUPP Tracing is not supported in the library.
 */
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:27"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:27"]
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
        /* Add sprintf-style formatted data to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn k5_buf_add_fmt(buf: *mut k5buf, fmt: *const libc::c_char,
                              _: ...);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src = "/usr/include/arpa/nameser.h:34"]
pub mod nameser_h {
    #[c2rust::src_loc = "93:1"]
    pub type ns_msg = __ns_msg;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "93:16"]
    pub struct __ns_msg {
        pub _msg: *const libc::c_uchar,
        pub _eom: *const libc::c_uchar,
        pub _id: uint16_t,
        pub _flags: uint16_t,
        pub _counts: [uint16_t; 4],
        pub _sections: [*const libc::c_uchar; 4],
        pub _sect: ns_sect,
        pub _rrnum: libc::c_int,
        pub _msg_ptr: *const libc::c_uchar,
    }
    #[c2rust::src_loc = "77:1"]
    pub type ns_sect = __ns_sect;
    #[c2rust::src_loc = "77:9"]
    pub type __ns_sect = libc::c_uint;
    #[c2rust::src_loc = "85:2"]
    pub const ns_s_max: __ns_sect = 4;
    #[c2rust::src_loc = "84:2"]
    pub const ns_s_ar: __ns_sect = 3;
    #[c2rust::src_loc = "83:2"]
    pub const ns_s_ud: __ns_sect = 2;
    #[c2rust::src_loc = "82:2"]
    pub const ns_s_ns: __ns_sect = 2;
    #[c2rust::src_loc = "81:2"]
    pub const ns_s_pr: __ns_sect = 1;
    #[c2rust::src_loc = "80:2"]
    pub const ns_s_an: __ns_sect = 1;
    #[c2rust::src_loc = "79:2"]
    pub const ns_s_zn: __ns_sect = 0;
    #[c2rust::src_loc = "78:2"]
    pub const ns_s_qd: __ns_sect = 0;
    #[c2rust::src_loc = "117:1"]
    pub type ns_rr = __ns_rr;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct __ns_rr {
        pub name: [libc::c_char; 1025],
        pub type_0: uint16_t,
        pub rr_class: uint16_t,
        pub ttl: uint32_t,
        pub rdlength: uint16_t,
        pub rdata: *const libc::c_uchar,
    }
    #[c2rust::src_loc = "231:1"]
    pub type ns_type = __ns_type;
    #[c2rust::src_loc = "231:9"]
    pub type __ns_type = libc::c_uint;
    #[c2rust::src_loc = "320:5"]
    pub const ns_t_max: __ns_type = 65536;
    #[c2rust::src_loc = "318:5"]
    pub const ns_t_dlv: __ns_type = 32769;
    #[c2rust::src_loc = "317:5"]
    pub const ns_t_ta: __ns_type = 32768;
    #[c2rust::src_loc = "316:5"]
    pub const ns_t_avc: __ns_type = 258;
    #[c2rust::src_loc = "315:5"]
    pub const ns_t_caa: __ns_type = 257;
    #[c2rust::src_loc = "314:5"]
    pub const ns_t_uri: __ns_type = 256;
    #[c2rust::src_loc = "313:5"]
    pub const ns_t_any: __ns_type = 255;
    #[c2rust::src_loc = "312:5"]
    pub const ns_t_maila: __ns_type = 254;
    #[c2rust::src_loc = "311:5"]
    pub const ns_t_mailb: __ns_type = 253;
    #[c2rust::src_loc = "310:5"]
    pub const ns_t_axfr: __ns_type = 252;
    #[c2rust::src_loc = "309:5"]
    pub const ns_t_ixfr: __ns_type = 251;
    #[c2rust::src_loc = "308:5"]
    pub const ns_t_tsig: __ns_type = 250;
    #[c2rust::src_loc = "307:5"]
    pub const ns_t_tkey: __ns_type = 249;
    #[c2rust::src_loc = "306:5"]
    pub const ns_t_eui64: __ns_type = 109;
    #[c2rust::src_loc = "305:5"]
    pub const ns_t_eui48: __ns_type = 108;
    #[c2rust::src_loc = "304:5"]
    pub const ns_t_lp: __ns_type = 107;
    #[c2rust::src_loc = "303:5"]
    pub const ns_t_l64: __ns_type = 106;
    #[c2rust::src_loc = "302:5"]
    pub const ns_t_l32: __ns_type = 105;
    #[c2rust::src_loc = "301:5"]
    pub const ns_t_nid: __ns_type = 104;
    #[c2rust::src_loc = "300:5"]
    pub const ns_t_unspec: __ns_type = 103;
    #[c2rust::src_loc = "299:5"]
    pub const ns_t_gid: __ns_type = 102;
    #[c2rust::src_loc = "298:5"]
    pub const ns_t_uid: __ns_type = 101;
    #[c2rust::src_loc = "297:5"]
    pub const ns_t_uinfo: __ns_type = 100;
    #[c2rust::src_loc = "296:5"]
    pub const ns_t_spf: __ns_type = 99;
    #[c2rust::src_loc = "295:5"]
    pub const ns_t_csync: __ns_type = 62;
    #[c2rust::src_loc = "294:5"]
    pub const ns_t_openpgpkey: __ns_type = 61;
    #[c2rust::src_loc = "293:5"]
    pub const ns_t_cdnskey: __ns_type = 60;
    #[c2rust::src_loc = "292:5"]
    pub const ns_t_cds: __ns_type = 59;
    #[c2rust::src_loc = "291:5"]
    pub const ns_t_talink: __ns_type = 58;
    #[c2rust::src_loc = "290:5"]
    pub const ns_t_rkey: __ns_type = 57;
    #[c2rust::src_loc = "289:5"]
    pub const ns_t_ninfo: __ns_type = 56;
    #[c2rust::src_loc = "288:5"]
    pub const ns_t_hip: __ns_type = 55;
    #[c2rust::src_loc = "287:5"]
    pub const ns_t_smimea: __ns_type = 53;
    #[c2rust::src_loc = "286:5"]
    pub const ns_t_tlsa: __ns_type = 52;
    #[c2rust::src_loc = "285:5"]
    pub const ns_t_nsec3param: __ns_type = 51;
    #[c2rust::src_loc = "284:5"]
    pub const ns_t_nsec3: __ns_type = 50;
    #[c2rust::src_loc = "283:5"]
    pub const ns_t_dhcid: __ns_type = 49;
    #[c2rust::src_loc = "282:5"]
    pub const ns_t_dnskey: __ns_type = 48;
    #[c2rust::src_loc = "281:5"]
    pub const ns_t_nsec: __ns_type = 47;
    #[c2rust::src_loc = "280:5"]
    pub const ns_t_rrsig: __ns_type = 46;
    #[c2rust::src_loc = "279:5"]
    pub const ns_t_ipseckey: __ns_type = 45;
    #[c2rust::src_loc = "278:5"]
    pub const ns_t_sshfp: __ns_type = 44;
    #[c2rust::src_loc = "277:5"]
    pub const ns_t_ds: __ns_type = 43;
    #[c2rust::src_loc = "276:5"]
    pub const ns_t_apl: __ns_type = 42;
    #[c2rust::src_loc = "275:5"]
    pub const ns_t_opt: __ns_type = 41;
    #[c2rust::src_loc = "274:5"]
    pub const ns_t_sink: __ns_type = 40;
    #[c2rust::src_loc = "273:5"]
    pub const ns_t_dname: __ns_type = 39;
    #[c2rust::src_loc = "272:5"]
    pub const ns_t_a6: __ns_type = 38;
    #[c2rust::src_loc = "271:5"]
    pub const ns_t_cert: __ns_type = 37;
    #[c2rust::src_loc = "270:5"]
    pub const ns_t_kx: __ns_type = 36;
    #[c2rust::src_loc = "269:5"]
    pub const ns_t_naptr: __ns_type = 35;
    #[c2rust::src_loc = "268:5"]
    pub const ns_t_atma: __ns_type = 34;
    #[c2rust::src_loc = "267:5"]
    pub const ns_t_srv: __ns_type = 33;
    #[c2rust::src_loc = "266:5"]
    pub const ns_t_nimloc: __ns_type = 32;
    #[c2rust::src_loc = "265:5"]
    pub const ns_t_eid: __ns_type = 31;
    #[c2rust::src_loc = "264:5"]
    pub const ns_t_nxt: __ns_type = 30;
    #[c2rust::src_loc = "263:5"]
    pub const ns_t_loc: __ns_type = 29;
    #[c2rust::src_loc = "262:5"]
    pub const ns_t_aaaa: __ns_type = 28;
    #[c2rust::src_loc = "261:5"]
    pub const ns_t_gpos: __ns_type = 27;
    #[c2rust::src_loc = "260:5"]
    pub const ns_t_px: __ns_type = 26;
    #[c2rust::src_loc = "259:5"]
    pub const ns_t_key: __ns_type = 25;
    #[c2rust::src_loc = "258:5"]
    pub const ns_t_sig: __ns_type = 24;
    #[c2rust::src_loc = "257:5"]
    pub const ns_t_nsap_ptr: __ns_type = 23;
    #[c2rust::src_loc = "256:5"]
    pub const ns_t_nsap: __ns_type = 22;
    #[c2rust::src_loc = "255:5"]
    pub const ns_t_rt: __ns_type = 21;
    #[c2rust::src_loc = "254:5"]
    pub const ns_t_isdn: __ns_type = 20;
    #[c2rust::src_loc = "253:5"]
    pub const ns_t_x25: __ns_type = 19;
    #[c2rust::src_loc = "252:5"]
    pub const ns_t_afsdb: __ns_type = 18;
    #[c2rust::src_loc = "251:5"]
    pub const ns_t_rp: __ns_type = 17;
    #[c2rust::src_loc = "250:5"]
    pub const ns_t_txt: __ns_type = 16;
    #[c2rust::src_loc = "249:5"]
    pub const ns_t_mx: __ns_type = 15;
    #[c2rust::src_loc = "248:5"]
    pub const ns_t_minfo: __ns_type = 14;
    #[c2rust::src_loc = "247:5"]
    pub const ns_t_hinfo: __ns_type = 13;
    #[c2rust::src_loc = "246:5"]
    pub const ns_t_ptr: __ns_type = 12;
    #[c2rust::src_loc = "245:5"]
    pub const ns_t_wks: __ns_type = 11;
    #[c2rust::src_loc = "244:5"]
    pub const ns_t_null: __ns_type = 10;
    #[c2rust::src_loc = "243:5"]
    pub const ns_t_mr: __ns_type = 9;
    #[c2rust::src_loc = "242:5"]
    pub const ns_t_mg: __ns_type = 8;
    #[c2rust::src_loc = "241:5"]
    pub const ns_t_mb: __ns_type = 7;
    #[c2rust::src_loc = "240:5"]
    pub const ns_t_soa: __ns_type = 6;
    #[c2rust::src_loc = "239:5"]
    pub const ns_t_cname: __ns_type = 5;
    #[c2rust::src_loc = "238:5"]
    pub const ns_t_mf: __ns_type = 4;
    #[c2rust::src_loc = "237:5"]
    pub const ns_t_md: __ns_type = 3;
    #[c2rust::src_loc = "236:5"]
    pub const ns_t_ns: __ns_type = 2;
    #[c2rust::src_loc = "235:5"]
    pub const ns_t_a: __ns_type = 1;
    #[c2rust::src_loc = "233:5"]
    pub const ns_t_invalid: __ns_type = 0;
    #[c2rust::src_loc = "326:1"]
    pub type ns_class = __ns_class;
    #[c2rust::src_loc = "326:9"]
    pub type __ns_class = libc::c_uint;
    #[c2rust::src_loc = "335:2"]
    pub const ns_c_max: __ns_class = 65536;
    #[c2rust::src_loc = "334:2"]
    pub const ns_c_any: __ns_class = 255;
    #[c2rust::src_loc = "333:2"]
    pub const ns_c_none: __ns_class = 254;
    #[c2rust::src_loc = "331:2"]
    pub const ns_c_hs: __ns_class = 4;
    #[c2rust::src_loc = "330:2"]
    pub const ns_c_chaos: __ns_class = 3;
    #[c2rust::src_loc = "329:2"]
    pub const ns_c_2: __ns_class = 2;
    #[c2rust::src_loc = "328:2"]
    pub const ns_c_in: __ns_class = 1;
    #[c2rust::src_loc = "327:2"]
    pub const ns_c_invalid: __ns_class = 0;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "401:1"]
        pub fn ns_parserr(_: *mut ns_msg, _: ns_sect, _: libc::c_int,
                          _: *mut ns_rr) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "398:1"]
        pub fn ns_initparse(_: *const libc::c_uchar, _: libc::c_int,
                            _: *mut ns_msg) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "422:1"]
        pub fn ns_name_uncompress(_: *const libc::c_uchar,
                                  _: *const libc::c_uchar,
                                  _: *const libc::c_uchar,
                                  _: *mut libc::c_char, _: size_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/res_state.h:34"]
pub mod res_state_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "13:8"]
    pub struct __res_state {
        pub retrans: libc::c_int,
        pub retry: libc::c_int,
        pub options: libc::c_ulong,
        pub nscount: libc::c_int,
        pub nsaddr_list: [sockaddr_in; 3],
        pub id: libc::c_ushort,
        pub dnsrch: [*mut libc::c_char; 7],
        pub defdname: [libc::c_char; 256],
        pub pfcode: libc::c_ulong,
        #[bitfield(name = "ndots", ty = "libc::c_uint", bits = "0..=3")]
        #[bitfield(name = "nsort", ty = "libc::c_uint", bits = "4..=7")]
        #[bitfield(name = "ipv6_unavail", ty = "libc::c_uint", bits =
                   "8..=8")]
        #[bitfield(name = "unused", ty = "libc::c_uint", bits = "9..=31")]
        pub ndots_nsort_ipv6_unavail_unused: [u8; 4],
        pub sort_list: [C2RustUnnamed_2; 10],
        pub __glibc_unused_qhook: *mut libc::c_void,
        pub __glibc_unused_rhook: *mut libc::c_void,
        pub res_h_errno: libc::c_int,
        pub _vcsock: libc::c_int,
        pub _flags: libc::c_uint,
        pub _u: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:2"]
    pub union C2RustUnnamed_0 {
        pub pad: [libc::c_char; 52],
        pub _ext: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:3"]
    pub struct C2RustUnnamed_1 {
        pub nscount: uint16_t,
        pub nsmap: [uint16_t; 3],
        pub nssocks: [libc::c_int; 3],
        pub nscount6: uint16_t,
        pub nsinit: uint16_t,
        pub nsaddrs: [*mut sockaddr_in6; 3],
        pub __glibc_reserved: [libc::c_uint; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:2"]
    pub struct C2RustUnnamed_2 {
        pub addr: in_addr,
        pub mask: uint32_t,
    }
    #[c2rust::src_loc = "59:1"]
    pub type res_state = *mut __res_state;
    use super::in_h::{sockaddr_in, sockaddr_in6, in_addr};
    use super::stdint_uintn_h::{uint16_t, uint32_t};
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
#[c2rust::header_src = "/usr/include/resolv.h:34"]
pub mod resolv_h {
    use super::res_state_h::{__res_state, res_state};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "280:1"]
        pub fn __res_ninit(_: res_state) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "286:1"]
        pub fn __res_nsearch(_: res_state, _: *const libc::c_char,
                             _: libc::c_int, _: libc::c_int,
                             _: *mut libc::c_uchar, _: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "296:1"]
        pub fn __res_nclose(_: res_state);
    }
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _profile_t};
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
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_init_dynamic,
                         k5_buf_add, k5_buf_add_fmt};
pub use self::nameser_h::{ns_msg, __ns_msg, ns_sect, __ns_sect, ns_s_max,
                          ns_s_ar, ns_s_ud, ns_s_ns, ns_s_pr, ns_s_an,
                          ns_s_zn, ns_s_qd, ns_rr, __ns_rr, ns_type,
                          __ns_type, ns_t_max, ns_t_dlv, ns_t_ta, ns_t_avc,
                          ns_t_caa, ns_t_uri, ns_t_any, ns_t_maila,
                          ns_t_mailb, ns_t_axfr, ns_t_ixfr, ns_t_tsig,
                          ns_t_tkey, ns_t_eui64, ns_t_eui48, ns_t_lp,
                          ns_t_l64, ns_t_l32, ns_t_nid, ns_t_unspec, ns_t_gid,
                          ns_t_uid, ns_t_uinfo, ns_t_spf, ns_t_csync,
                          ns_t_openpgpkey, ns_t_cdnskey, ns_t_cds,
                          ns_t_talink, ns_t_rkey, ns_t_ninfo, ns_t_hip,
                          ns_t_smimea, ns_t_tlsa, ns_t_nsec3param, ns_t_nsec3,
                          ns_t_dhcid, ns_t_dnskey, ns_t_nsec, ns_t_rrsig,
                          ns_t_ipseckey, ns_t_sshfp, ns_t_ds, ns_t_apl,
                          ns_t_opt, ns_t_sink, ns_t_dname, ns_t_a6, ns_t_cert,
                          ns_t_kx, ns_t_naptr, ns_t_atma, ns_t_srv,
                          ns_t_nimloc, ns_t_eid, ns_t_nxt, ns_t_loc,
                          ns_t_aaaa, ns_t_gpos, ns_t_px, ns_t_key, ns_t_sig,
                          ns_t_nsap_ptr, ns_t_nsap, ns_t_rt, ns_t_isdn,
                          ns_t_x25, ns_t_afsdb, ns_t_rp, ns_t_txt, ns_t_mx,
                          ns_t_minfo, ns_t_hinfo, ns_t_ptr, ns_t_wks,
                          ns_t_null, ns_t_mr, ns_t_mg, ns_t_mb, ns_t_soa,
                          ns_t_cname, ns_t_mf, ns_t_md, ns_t_ns, ns_t_a,
                          ns_t_invalid, ns_class, __ns_class, ns_c_max,
                          ns_c_any, ns_c_none, ns_c_hs, ns_c_chaos, ns_c_2,
                          ns_c_in, ns_c_invalid, ns_parserr, ns_initparse,
                          ns_name_uncompress};
pub use self::res_state_h::{__res_state, C2RustUnnamed_0, C2RustUnnamed_1,
                            C2RustUnnamed_2, res_state};
use self::string_h::{strdup, strncpy, memset};
use self::stdlib_h::{free, realloc, malloc};
use self::k5_trace_h::krb5int_trace;
use self::resolv_h::{__res_ninit, __res_nsearch, __res_nclose};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/os/dnsglue.c */
/*
 * Copyright 2004, 2009 by the Massachusetts Institute of Technology.
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
 * Only use res_ninit() if there's also a res_ndestroy(), to avoid
 * memory leaks (Linux & Solaris) and outright corruption (AIX 4.x,
 * 5.x).  While we're at it, make sure res_nsearch() is there too.
 *
 * In any case, it is probable that platforms having broken
 * res_ninit() will have thread safety hacks for res_init() and _res.
 */
/*
 * Opaque handle
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "51:8"]
pub struct krb5int_dns_state {
    pub nclass: libc::c_int,
    pub ntype: libc::c_int,
    pub ansp: *mut libc::c_void,
    pub anslen: libc::c_int,
    pub ansmax: libc::c_int,
    pub cur_ans: libc::c_int,
    pub msg: ns_msg,
}
/*
 * Define macros to use the best available DNS search functions.  INIT_HANDLE()
 * returns true if handle initialization is successful, false if it is not.
 * SEARCH() returns the length of the response or -1 on error.
 * PRIMARY_DOMAIN() returns the first search domain in allocated memory.
 * DECLARE_HANDLE() must be used last in the declaration list since it may
 * evaluate to nothing.
 */
/* Use res_ninit, res_nsearch, and res_ndestroy or res_nclose. */
/*
 * krb5int_dns_init()
 *
 * Initialize an opaque handle.  Do name lookup and initial parsing of
 * reply, skipping question section.  Prepare to iterate over answer
 * section.  Returns -1 on error, 0 on success.
 */
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn krb5int_dns_init(mut dsp:
                                              *mut *mut krb5int_dns_state,
                                          mut host: *mut libc::c_char,
                                          mut nclass: libc::c_int,
                                          mut ntype: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut ds: *mut krb5int_dns_state = 0 as *mut krb5int_dns_state;
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut nextincr: size_t = 0;
    let mut maxincr: size_t = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut h: __res_state =
        __res_state{retrans: 0,
                    retry: 0,
                    options: 0,
                    nscount: 0,
                    nsaddr_list:
                        [sockaddr_in{sin_family: 0,
                                     sin_port: 0,
                                     sin_addr: in_addr{s_addr: 0,},
                                     sin_zero: [0; 8],}; 3],
                    id: 0,
                    dnsrch: [0 as *mut libc::c_char; 7],
                    defdname: [0; 256],
                    pfcode: 0,
                    ndots_nsort_ipv6_unavail_unused: [0; 4],
                    sort_list:
                        [C2RustUnnamed_2{addr: in_addr{s_addr: 0,}, mask: 0,};
                            10],
                    __glibc_unused_qhook: 0 as *mut libc::c_void,
                    __glibc_unused_rhook: 0 as *mut libc::c_void,
                    res_h_errno: 0,
                    _vcsock: 0,
                    _flags: 0,
                    _u: C2RustUnnamed_0{pad: [0; 52],},};
    ds =
        malloc(::std::mem::size_of::<krb5int_dns_state>() as libc::c_ulong) as
            *mut krb5int_dns_state;
    *dsp = ds;
    if ds.is_null() { return -(1 as libc::c_int) }
    ret = -(1 as libc::c_int);
    (*ds).nclass = nclass;
    (*ds).ntype = ntype;
    (*ds).ansp = 0 as *mut libc::c_void;
    (*ds).anslen = 0 as libc::c_int;
    (*ds).ansmax = 0 as libc::c_int;
    nextincr = 4096 as libc::c_int as size_t;
    maxincr = 2147483647 as libc::c_int as size_t;
    (*ds).cur_ans = 0 as libc::c_int;
    memset(&mut h as *mut __res_state as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<__res_state>() as libc::c_ulong);
    if !(__res_ninit(&mut h) == 0 as libc::c_int) {
        return -(1 as libc::c_int)
    }
    loop  {
        p =
            if (*ds).ansp.is_null() {
                malloc(nextincr)
            } else { realloc((*ds).ansp, nextincr) } as *mut libc::c_uchar;
        if p.is_null() {
            ret = -(1 as libc::c_int);
            current_block = 12405928152146757229;
            break ;
        } else {
            (*ds).ansp = p as *mut libc::c_void;
            (*ds).ansmax = nextincr as libc::c_int;
            len =
                __res_nsearch(&mut h, host, (*ds).nclass, (*ds).ntype,
                              (*ds).ansp as *mut libc::c_uchar, (*ds).ansmax);
            if len as size_t > maxincr {
                ret = -(1 as libc::c_int);
                current_block = 12405928152146757229;
                break ;
            } else {
                while nextincr < len as size_t {
                    nextincr =
                        (nextincr as
                             libc::c_ulong).wrapping_mul(2 as libc::c_int as
                                                             libc::c_ulong) as
                            size_t as size_t
                }
                if len < 0 as libc::c_int || nextincr > maxincr {
                    ret = -(1 as libc::c_int);
                    current_block = 12405928152146757229;
                    break ;
                } else if !(len > (*ds).ansmax) {
                    current_block = 14359455889292382949;
                    break ;
                }
            }
        }
    }
    match current_block {
        14359455889292382949 => {
            (*ds).anslen = len;
            ret =
                ns_initparse((*ds).ansp as *const libc::c_uchar, (*ds).anslen,
                             &mut (*ds).msg);
            if !(ret < 0 as libc::c_int) { ret = 0 as libc::c_int }
        }
        _ => { }
    }
    __res_nclose(&mut h);
    if ret < 0 as libc::c_int {
        if !(*ds).ansp.is_null() {
            free((*ds).ansp);
            (*ds).ansp = 0 as *mut libc::c_void
        }
    }
    return ret;
}
/*
 * krb5int_dns_nextans - get next matching answer record
 *
 * Sets pp to NULL if no more records.  Returns -1 on error, 0 on
 * success.
 */
#[no_mangle]
#[c2rust::src_loc = "203:1"]
pub unsafe extern "C" fn krb5int_dns_nextans(mut ds: *mut krb5int_dns_state,
                                             mut pp:
                                                 *mut *const libc::c_uchar,
                                             mut lenp: *mut libc::c_int)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut rr: ns_rr =
        ns_rr{name: [0; 1025],
              type_0: 0,
              rr_class: 0,
              ttl: 0,
              rdlength: 0,
              rdata: 0 as *const libc::c_uchar,};
    *pp = 0 as *const libc::c_uchar;
    *lenp = 0 as libc::c_int;
    while (*ds).cur_ans <
              (*ds).msg._counts[ns_s_an as libc::c_int as usize] as
                  libc::c_int + 0 as libc::c_int {
        len = ns_parserr(&mut (*ds).msg, ns_s_an, (*ds).cur_ans, &mut rr);
        if len < 0 as libc::c_int { return -(1 as libc::c_int) }
        (*ds).cur_ans += 1;
        if (*ds).nclass ==
               (rr.rr_class as libc::c_int + 0 as libc::c_int) as ns_class as
                   libc::c_int &&
               (*ds).ntype ==
                   (rr.type_0 as libc::c_int + 0 as libc::c_int) as ns_type as
                       libc::c_int {
            *pp = rr.rdata.offset(0 as libc::c_int as isize);
            *lenp = rr.rdlength as libc::c_int + 0 as libc::c_int;
            return 0 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/*
 * krb5int_dns_expand - wrapper for dn_expand()
 */
#[no_mangle]
#[c2rust::src_loc = "231:1"]
pub unsafe extern "C" fn krb5int_dns_expand(mut ds: *mut krb5int_dns_state,
                                            mut p: *const libc::c_uchar,
                                            mut buf: *mut libc::c_char,
                                            mut len: libc::c_int)
 -> libc::c_int {
    return ns_name_uncompress((*ds).ansp as *const libc::c_uchar,
                              ((*ds).ansp as
                                   *mut libc::c_uchar).offset((*ds).anslen as
                                                                  isize), p,
                              buf, len as size_t);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/os/dnsglue.h */
/*
 * Copyright 2004 by the Massachusetts Institute of Technology.
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
 * Glue layer for DNS resolver, to make parsing of replies easier
 * whether we are using BIND 4, 8, or 9.  This header is not used on
 * Windows.
 */
/*
 * BIND 4 doesn't have the ns_initparse() API, so we need to do some
 * manual parsing via the HEADER struct.  BIND 8 does have
 * ns_initparse(), but has enums for the various protocol constants
 * rather than the BIND 4 macros.  BIND 9 (at least on macOS 10.3)
 * appears to disable res_nsearch() if BIND_8_COMPAT is defined
 * (which is necessary to obtain the HEADER struct).
 *
 * We use ns_initparse() if available at all, and never define
 * BIND_8_COMPAT.  If there is no ns_initparse(), we do manual parsing
 * by using the HEADER struct.
 */
/* for MAXHOSTNAMELEN */
/*
 * Solaris 7 has ns_rr_cl rather than ns_rr_class.
 */
/*
 * Some BIND 8 / BIND 9 implementations disable the BIND 4 style
 * constants.
 */
/* !HAVE_RES_NSEARCH */
/* HAVE_RES_NSEARCH */
/*
 * INCR_OK
 *
 * Given moving pointer PTR offset from BASE, return true if adding
 * INCR to PTR doesn't move it PTR than MAX bytes from BASE.
 */
/*
 * SAFE_GETUINT16
 *
 * Given PTR offset from BASE, if at least INCR bytes are safe to
 * read, get network byte order uint16 into S, and increment PTR.  On
 * failure, goto LABEL.
 */
/*
 * Free stuff.
 */
#[no_mangle]
#[c2rust::src_loc = "250:1"]
pub unsafe extern "C" fn krb5int_dns_fini(mut ds: *mut krb5int_dns_state) {
    if ds.is_null() { return }
    if !(*ds).ansp.is_null() { free((*ds).ansp); }
    free(ds as *mut libc::c_void);
}
/*
 * Compat routines for BIND 4
 */
/* !HAVE_NS_INITPARSE */
/* not _WIN32 */
/* Construct a DNS label of the form "prefix[.name.]".  name may be NULL. */
#[c2rust::src_loc = "367:1"]
unsafe extern "C" fn txt_lookup_name(mut prefix: *const libc::c_char,
                                     mut name: *const libc::c_char)
 -> *mut libc::c_char {
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    k5_buf_init_dynamic(&mut buf);
    if name.is_null() ||
           *name.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\u{0}' as i32 {
        k5_buf_add(&mut buf, prefix);
    } else {
        k5_buf_add_fmt(&mut buf as *mut k5buf,
                       b"%s.%s\x00" as *const u8 as *const libc::c_char,
                       prefix, name);
        /*
         * Realm names don't (normally) end with ".", but if the query doesn't
         * end with "." and doesn't get an answer as is, the resolv code will
         * try appending the local domain.  Since the realm names are
         * absolutes, let's stop that.
         *
         * But only if a name has been specified.  If we are performing a
         * search on the prefix alone then the intention is to allow the local
         * domain or domain search lists to be expanded.
         */
        if buf.len > 0 as libc::c_int as libc::c_ulong &&
               *(buf.data as
                     *mut libc::c_char).offset(buf.len.wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                                   as isize) as libc::c_int !=
                   '.' as i32 {
            k5_buf_add(&mut buf,
                       b".\x00" as *const u8 as *const libc::c_char);
        }
    }
    return buf.data as *mut libc::c_char;
}
/*
 * Try to look up a TXT record pointing to a Kerberos realm
 */
/* _WIN32 */
#[no_mangle]
#[c2rust::src_loc = "448:1"]
pub unsafe extern "C" fn k5_try_realm_txt_rr(mut context: krb5_context,
                                             mut prefix: *const libc::c_char,
                                             mut name: *const libc::c_char,
                                             mut realm:
                                                 *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328167 as libc::c_long) as krb5_error_code;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut base: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut txtname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut rdlen: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ds: *mut krb5int_dns_state = 0 as *mut krb5int_dns_state;
    /*
     * Form our query, and send it via DNS
     */
    txtname = txt_lookup_name(prefix, name);
    if txtname.is_null() { return 12 as libc::c_int }
    ret =
        krb5int_dns_init(&mut ds, txtname, ns_c_in as libc::c_int,
                         ns_t_txt as libc::c_int);
    if ret < 0 as libc::c_int {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"TXT record {str} not found\x00" as *const u8 as
                              *const libc::c_char, txtname);
        }
    } else {
        ret = krb5int_dns_nextans(ds, &mut base, &mut rdlen);
        if !(ret < 0 as libc::c_int || base.is_null()) {
            p = base;
            if 1 as libc::c_int as libc::c_long <=
                   rdlen as libc::c_long -
                       p.wrapping_offset_from(base) as libc::c_long {
                let fresh0 = p;
                p = p.offset(1);
                len = *fresh0 as libc::c_int;
                *realm =
                    malloc((len as
                                size_t).wrapping_add(1 as libc::c_int as
                                                         libc::c_ulong)) as
                        *mut libc::c_char;
                if (*realm).is_null() {
                    retval = 12 as libc::c_int
                } else {
                    strncpy(*realm, p as *const libc::c_char, len as size_t);
                    *(*realm).offset(len as isize) =
                        '\u{0}' as i32 as libc::c_char;
                    /* Avoid a common error. */
                    if *(*realm).offset((len - 1 as libc::c_int) as isize) as
                           libc::c_int == '.' as i32 {
                        *(*realm).offset((len - 1 as libc::c_int) as isize) =
                            '\u{0}' as i32 as libc::c_char
                    }
                    retval = 0 as libc::c_int;
                    if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"TXT record {str} found: {str}\x00" as
                                          *const u8 as *const libc::c_char,
                                      txtname, *realm);
                    }
                }
            }
        }
    }
    krb5int_dns_fini(ds);
    free(txtname as *mut libc::c_void);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "498:1"]
pub unsafe extern "C" fn k5_primary_domain() -> *mut libc::c_char {
    let mut domain: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut h: __res_state =
        __res_state{retrans: 0,
                    retry: 0,
                    options: 0,
                    nscount: 0,
                    nsaddr_list:
                        [sockaddr_in{sin_family: 0,
                                     sin_port: 0,
                                     sin_addr: in_addr{s_addr: 0,},
                                     sin_zero: [0; 8],}; 3],
                    id: 0,
                    dnsrch: [0 as *mut libc::c_char; 7],
                    defdname: [0; 256],
                    pfcode: 0,
                    ndots_nsort_ipv6_unavail_unused: [0; 4],
                    sort_list:
                        [C2RustUnnamed_2{addr: in_addr{s_addr: 0,}, mask: 0,};
                            10],
                    __glibc_unused_qhook: 0 as *mut libc::c_void,
                    __glibc_unused_rhook: 0 as *mut libc::c_void,
                    res_h_errno: 0,
                    _vcsock: 0,
                    _flags: 0,
                    _u: C2RustUnnamed_0{pad: [0; 52],},};
    memset(&mut h as *mut __res_state as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<__res_state>() as libc::c_ulong);
    if !(__res_ninit(&mut h) == 0 as libc::c_int) {
        return 0 as *mut libc::c_char
    }
    domain = strdup(h.dnsrch[0 as libc::c_int as usize]);
    __res_nclose(&mut h);
    return domain;
}
/* KRB5_DNS_LOOKUP */
/* not _WIN32 */
