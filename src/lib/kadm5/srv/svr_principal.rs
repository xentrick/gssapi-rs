use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:7"]
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:7"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:7"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:7"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:7"]
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
    /*
 * Prompter enhancements
 */
/* * Prompt for password */
    /* * Prompt for new password (during password change) */
    /* * Prompt for new password again */
    /* * Prompt for preauthentication data (such as an OTP value) */
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
 * Compare two encryption types.
 *
 * @param [in]  context         Library context
 * @param [in]  e1              First encryption type
 * @param [in]  e2              Second encryption type
 * @param [out] similar         @c TRUE if types are similar, @c FALSE if not
 *
 * This function determines whether two encryption types use the same kind of
 * keys.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "882:1"]
        pub fn krb5_c_enctype_compare(context: krb5_context, e1: krb5_enctype,
                                      e2: krb5_enctype,
                                      similar: *mut krb5_boolean)
         -> krb5_error_code;
        /* *
 * Compare two principals.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
        #[no_mangle]
        #[c2rust::src_loc = "3664:1"]
        pub fn krb5_principal_compare(context: krb5_context,
                                      princ1: krb5_const_principal,
                                      princ2: krb5_const_principal)
         -> krb5_boolean;
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
        #[no_mangle]
        #[c2rust::src_loc = "3813:1"]
        pub fn krb5_copy_principal(context: krb5_context,
                                   inprinc: krb5_const_principal,
                                   outprinc: *mut krb5_principal)
         -> krb5_error_code;
        /* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
        /* *
 * Retrieve the current time with context specific time offset adjustment.
 *
 * @param [in]  context         Library context
 * @param [out] timeret         Timestamp to fill in
 *
 * This function retrieves the system time of day with the context specific
 * time offset adjustment.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
        /* *
 * Clear the extended error message in a context.
 *
 * @param [in] ctx              Library context
 *
 * This function unsets the extended error message in a context, to ensure that
 * it is not mistakenly applied to another occurrence of the same error code.
 */
        #[no_mangle]
        #[c2rust::src_loc = "8043:1"]
        pub fn krb5_clear_error_message(ctx: krb5_context);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:7"]
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
    #[inline]
    #[c2rust::src_loc = "2354:1"]
    pub unsafe extern "C" fn ts_incr(mut ts: krb5_timestamp,
                                     mut delta: krb5_deltat)
     -> krb5_timestamp {
        return (ts as uint32_t).wrapping_add(delta as uint32_t) as
                   krb5_timestamp;
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
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_timestamp, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    use super::stdlib_h::{calloc, free};
    use super::string_h::explicit_bzero;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:7"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:7"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:7"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:9"]
pub mod kdb_h {
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
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
    #[c2rust::src_loc = "156:16"]
    pub struct krb5_string_attr_st {
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    /* NOT saved */
    /* String attributes (currently stored inside tl-data) map C string keys to
 * values.  They can be set via kadmin and consumed by KDC plugins. */
    #[c2rust::src_loc = "156:1"]
    pub type krb5_string_attr = krb5_string_attr_st;
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
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /* # of array elements */
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
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
    /* Length, data */
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
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_data,
                        krb5_magic, krb5_ui_4, krb5_flags, krb5_deltat,
                        krb5_timestamp, krb5_kvno, krb5_principal,
                        krb5_enctype, krb5_int32, krb5_context,
                        krb5_principal_data, krb5_error_code, krb5_keyblock,
                        krb5_boolean};
    use super::k5_int_h::_krb5_context;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn krb5_db_free_principal(kcontext: krb5_context,
                                      entry: *mut krb5_db_entry);
        #[no_mangle]
        #[c2rust::src_loc = "379:1"]
        pub fn krb5_db_rename_principal(kcontext: krb5_context,
                                        source: krb5_principal,
                                        target: krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "413:1"]
        pub fn krb5_db_fetch_mkey_list(context: krb5_context,
                                       mname: krb5_principal,
                                       mkey: *const krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn krb5_dbe_find_enctype(kcontext: krb5_context,
                                     dbentp: *mut krb5_db_entry,
                                     ktype: krb5_int32, stype: krb5_int32,
                                     kvno: krb5_int32,
                                     kdatap: *mut *mut krb5_key_data)
         -> krb5_error_code;
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
        /* *
 * Decrypts the key given in @@a key_data. If @a mkey is specified, that
 * master key is used. If @a mkey is NULL, then all master keys are tried.
 */
        #[no_mangle]
        #[c2rust::src_loc = "446:1"]
        pub fn krb5_dbe_decrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         key_data: *const krb5_key_data,
                                         dbkey: *mut krb5_keyblock,
                                         keysalt: *mut krb5_keysalt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn krb5_dbe_encrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         dbkey: *const krb5_keyblock,
                                         keysalt: *const krb5_keysalt,
                                         keyver: libc::c_int,
                                         key_data: *mut krb5_key_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "472:1"]
        pub fn krb5_dbe_find_mkey(context: krb5_context,
                                  entry: *mut krb5_db_entry,
                                  mkey: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        /* Set *mkvno to mkvno in entry tl_data, or minimum value from mkey_list. */
        #[no_mangle]
        #[c2rust::src_loc = "487:1"]
        pub fn krb5_dbe_get_mkvno(context: krb5_context,
                                  entry: *mut krb5_db_entry,
                                  mkvno: *mut krb5_kvno) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "492:1"]
        pub fn krb5_dbe_lookup_mod_princ_data(context: krb5_context,
                                              entry: *mut krb5_db_entry,
                                              mod_time: *mut krb5_timestamp,
                                              mod_princ: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "502:1"]
        pub fn krb5_dbe_update_mkvno(context: krb5_context,
                                     entry: *mut krb5_db_entry,
                                     mkvno: krb5_kvno) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn krb5_dbe_update_last_pwd_change(context: krb5_context,
                                               entry: *mut krb5_db_entry,
                                               stamp: krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn krb5_dbe_lookup_last_pwd_change(context: krb5_context,
                                               entry: *mut krb5_db_entry,
                                               stamp: *mut krb5_timestamp)
         -> krb5_error_code;
        /* Retrieve the set of string attributes in entry, in no particular order.
 * Free *strings_out with krb5_dbe_free_strings when done. */
        #[no_mangle]
        #[c2rust::src_loc = "576:1"]
        pub fn krb5_dbe_get_strings(context: krb5_context,
                                    entry: *mut krb5_db_entry,
                                    strings_out: *mut *mut krb5_string_attr,
                                    count_out: *mut libc::c_int)
         -> krb5_error_code;
        /* Change or add a string attribute in entry, or delete it if value is NULL. */
        #[no_mangle]
        #[c2rust::src_loc = "587:1"]
        pub fn krb5_dbe_set_string(context: krb5_context,
                                   entry: *mut krb5_db_entry,
                                   key: *const libc::c_char,
                                   value: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "602:1"]
        pub fn krb5_dbe_update_tl_data(context: krb5_context,
                                       entry: *mut krb5_db_entry,
                                       new_tl_data: *mut krb5_tl_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "620:1"]
        pub fn krb5_dbe_cpw(kcontext: krb5_context,
                            master_key: *mut krb5_keyblock,
                            ks_tuple: *mut krb5_key_salt_tuple,
                            ks_tuple_count: libc::c_int,
                            passwd: *mut libc::c_char, new_kvno: libc::c_int,
                            keepold: krb5_boolean,
                            db_entry: *mut krb5_db_entry) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "638:1"]
        pub fn krb5_dbe_crk(context: krb5_context,
                            master_key: *mut krb5_keyblock,
                            ks_tuple: *mut krb5_key_salt_tuple,
                            ks_tuple_count: libc::c_int,
                            keepold: krb5_boolean,
                            db_entry: *mut krb5_db_entry) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "654:1"]
        pub fn krb5_db_get_key_data_kvno(context: krb5_context,
                                         count: libc::c_int,
                                         data: *mut krb5_key_data)
         -> libc::c_int;
        /* *
 * Sort an array of @a krb5_key_data keys in descending order by their kvno.
 * Key data order within a kvno is preserved.
 *
 * @param key_data
 *     The @a krb5_key_data array to sort.  This is sorted in place so the
 *     array will be modified.
 * @param key_data_length
 *     The length of @a key_data.
 */
        #[no_mangle]
        #[c2rust::src_loc = "747:1"]
        pub fn krb5_dbe_sort_key_data(key_data: *mut krb5_key_data,
                                      key_data_length: size_t);
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:9"]
pub mod admin_h {
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
    #[c2rust::src_loc = "70:1"]
    pub type kadm5_policy_t = *mut libc::c_char;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "190:16"]
    pub struct _kadm5_principal_ent_t {
        pub principal: krb5_principal,
        pub princ_expire_time: krb5_timestamp,
        pub last_pwd_change: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub max_life: krb5_deltat,
        pub mod_name: krb5_principal,
        pub mod_date: krb5_timestamp,
        pub attributes: krb5_flags,
        pub kvno: krb5_kvno,
        pub mkvno: krb5_kvno,
        pub policy: *mut libc::c_char,
        pub aux_attributes: libc::c_long,
        pub max_renewable_life: krb5_deltat,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_key_data: krb5_int16,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    #[c2rust::src_loc = "190:1"]
    pub type kadm5_principal_ent_t = *mut _kadm5_principal_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _kadm5_policy_ent_t {
        pub policy: *mut libc::c_char,
        pub pw_min_life: libc::c_long,
        pub pw_max_life: libc::c_long,
        pub pw_min_length: libc::c_long,
        pub pw_min_classes: libc::c_long,
        pub pw_history_num: libc::c_long,
        pub policy_refcnt: libc::c_long,
        pub pw_max_fail: krb5_kvno,
        pub pw_failcnt_interval: krb5_deltat,
        pub pw_lockout_duration: krb5_deltat,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    #[c2rust::src_loc = "215:1"]
    pub type kadm5_policy_ent_rec = _kadm5_policy_ent_t;
    #[c2rust::src_loc = "215:1"]
    pub type kadm5_policy_ent_t = *mut _kadm5_policy_ent_t;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "284:16"]
    pub struct _kadm5_key_data {
        pub kvno: krb5_kvno,
        pub key: krb5_keyblock,
        pub salt: krb5_keysalt,
    }
    #[c2rust::src_loc = "284:1"]
    pub type kadm5_key_data = _kadm5_key_data;
    use super::krb5_h::{krb5_principal, krb5_timestamp, krb5_deltat,
                        krb5_flags, krb5_kvno, krb5_int16, krb5_enctype,
                        krb5_int32, krb5_keyblock, krb5_context};
    use super::kdb_h::{krb5_tl_data, krb5_key_data, krb5_key_salt_tuple,
                       krb5_keysalt};
    use super::stdint_uintn_h::uint32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "493:1"]
        pub fn kadm5_free_kadm5_key_data(context: krb5_context,
                                         n_key_data: libc::c_int,
                                         key_data: *mut kadm5_key_data)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "447:1"]
        pub fn kadm5_free_policy_ent(server_handle: *mut libc::c_void,
                                     ent: kadm5_policy_ent_t) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "431:1"]
        pub fn kadm5_get_policy(server_handle: *mut libc::c_void,
                                policy: kadm5_policy_t,
                                ent: kadm5_policy_ent_t) -> kadm5_ret_t;
    }
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/server_internal.h:11"]
pub mod server_internal_h {
    #[c2rust::src_loc = "57:1"]
    pub type osa_pw_hist_t = *mut _osa_pw_hist_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:16"]
    pub struct _osa_pw_hist_t {
        pub n_key_data: libc::c_int,
        pub key_data: *mut krb5_key_data,
    }
    #[c2rust::src_loc = "57:1"]
    pub type osa_pw_hist_ent = _osa_pw_hist_t;
    #[c2rust::src_loc = "62:1"]
    pub type osa_princ_ent_rec = _osa_princ_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:16"]
    pub struct _osa_princ_ent_t {
        pub version: libc::c_int,
        pub policy: *mut libc::c_char,
        pub aux_attributes: libc::c_long,
        pub old_key_len: libc::c_uint,
        pub old_key_next: libc::c_uint,
        pub admin_history_kvno: krb5_kvno,
        pub old_keys: *mut osa_pw_hist_ent,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:16"]
    pub struct _kadm5_server_handle_t {
        pub magic_number: krb5_ui_4,
        pub struct_version: krb5_ui_4,
        pub api_version: krb5_ui_4,
        pub context: krb5_context,
        pub current_caller: krb5_principal,
        pub params: kadm5_config_params,
        pub lhandle: *mut _kadm5_server_handle_t,
        pub db_args: *mut *mut libc::c_char,
        pub qual_handles: *mut pwqual_handle,
        pub hook_handles: *mut kadm5_hook_handle,
    }
    #[c2rust::src_loc = "40:1"]
    pub type kadm5_hook_handle = *mut kadm5_hook_handle_st;
    /* A pwqual_handle represents a password quality plugin module. */
    #[c2rust::src_loc = "38:1"]
    pub type pwqual_handle = *mut pwqual_handle_st;
    #[c2rust::src_loc = "42:1"]
    pub type kadm5_server_handle_t = *mut _kadm5_server_handle_t;
    #[c2rust::src_loc = "62:1"]
    pub type osa_princ_ent_t = *mut _osa_princ_ent_t;
    use super::kdb_h::{krb5_key_data, krb5_db_entry, krb5_key_salt_tuple};
    use super::krb5_h::{krb5_kvno, krb5_ui_4, krb5_context, krb5_principal,
                        krb5_principal_data, krb5_keyblock, krb5_error_code,
                        krb5_boolean};
    use super::admin_h::{kadm5_config_params, _kadm5_policy_ent_t,
                         kadm5_policy_ent_t, kadm5_ret_t,
                         _kadm5_principal_ent_t, kadm5_principal_ent_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type kadm5_hook_handle_st;
        #[c2rust::src_loc = "38:16"]
        pub type pwqual_handle_st;
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn passwd_check(handle: kadm5_server_handle_t,
                            pass: *const libc::c_char,
                            policy: kadm5_policy_ent_t,
                            principal: krb5_principal) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn kdb_get_active_mkey(handle: kadm5_server_handle_t,
                                   act_kvno_out: *mut krb5_kvno,
                                   act_mkey_out: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn kdb_get_hist_key(handle: kadm5_server_handle_t,
                                keyblocks_out: *mut *mut krb5_keyblock,
                                kvno_out: *mut krb5_kvno) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "87:1"]
        pub fn kdb_free_keyblocks(handle: kadm5_server_handle_t,
                                  keyblocks: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn kdb_get_entry(handle: kadm5_server_handle_t,
                             principal: krb5_principal,
                             kdb: *mut *mut krb5_db_entry,
                             adb: *mut osa_princ_ent_rec) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "92:1"]
        pub fn kdb_free_entry(handle: kadm5_server_handle_t,
                              kdb: *mut krb5_db_entry,
                              adb: *mut osa_princ_ent_rec) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn kdb_put_entry(handle: kadm5_server_handle_t,
                             kdb: *mut krb5_db_entry,
                             adb: *mut osa_princ_ent_rec) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "96:1"]
        pub fn kdb_delete_entry(handle: kadm5_server_handle_t,
                                name: krb5_principal) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn krb5_free_key_data_contents(context: krb5_context,
                                           key: *mut krb5_key_data)
         -> kadm5_ret_t;
        /* ** initvt functions for built-in password quality modules ***/
        /* The dict module checks passwords against the realm's dictionary. */
        /* The empty module rejects empty passwords (even with no password policy). */
        /* The hesiod module checks passwords against GECOS fields from Hesiod passwd
 * information (only if the tree was built with Hesiod support). */
        /* The princ module checks passwords against principal components. */
        /* * @{
 * @name kadm5_hook plugin support
 */
        /* * Load all kadm5_hook plugins. */
        /* * Free handles allocated by k5_kadm5_hook_load(). */
        /* * Call the chpass entry point on every kadm5_hook in @a handles. */
        /* * Call the create entry point for kadm5_hook_plugins. */
        /* * Call modify kadm5_hook entry point. */
        #[no_mangle]
        #[c2rust::src_loc = "245:1"]
        pub fn k5_kadm5_hook_modify(context: krb5_context,
                                    handles: *mut kadm5_hook_handle,
                                    stage: libc::c_int,
                                    princ: kadm5_principal_ent_t,
                                    mask: libc::c_long) -> kadm5_ret_t;
        /* * Call remove kadm5_hook entry point. */
        /* * Call rename kadm5_hook entry point. */
        #[no_mangle]
        #[c2rust::src_loc = "259:1"]
        pub fn k5_kadm5_hook_rename(context: krb5_context,
                                    handles: *mut kadm5_hook_handle,
                                    stage: libc::c_int,
                                    oprinc: krb5_principal,
                                    nprinc: krb5_principal) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "225:1"]
        pub fn k5_kadm5_hook_chpass(context: krb5_context,
                                    handles: *mut kadm5_hook_handle,
                                    stage: libc::c_int, princ: krb5_principal,
                                    keepold: krb5_boolean,
                                    n_ks_tuple: libc::c_int,
                                    ks_tuple: *mut krb5_key_salt_tuple,
                                    newpass: *const libc::c_char)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "235:1"]
        pub fn k5_kadm5_hook_create(context: krb5_context,
                                    handles: *mut kadm5_hook_handle,
                                    stage: libc::c_int,
                                    princ: kadm5_principal_ent_t,
                                    mask: libc::c_long,
                                    n_ks_tuple: libc::c_int,
                                    ks_tuple: *mut krb5_key_salt_tuple,
                                    newpass: *const libc::c_char)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "252:1"]
        pub fn k5_kadm5_hook_remove(context: krb5_context,
                                    handles: *mut kadm5_hook_handle,
                                    stage: libc::c_int, princ: krb5_principal)
         -> kadm5_ret_t;
    }
    /* __KADM5_SERVER_INTERNAL_H__ */
    /* * @}*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/kadm5_hook_plugin.h:17"]
pub mod kadm5_hook_plugin_h {
    /* * In this stage, plugin failures are logged but otherwise ignored.*/
    #[c2rust::src_loc = "86:5"]
    pub const KADM5_HOOK_STAGE_POSTCOMMIT: kadm5_hook_stage = 1;
    /* * In this stage, any plugin failure prevents following plugins from
     *         running and aborts the operation.*/
    #[c2rust::src_loc = "84:5"]
    pub const KADM5_HOOK_STAGE_PRECOMMIT: kadm5_hook_stage = 0;
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
    /* *
 * @file krb5/krb5_kadm5_hook_plugin.h
 * Provide a plugin interface for kadm5 operations. This interface
 * permits a plugin to intercept principal modification, creation and
 * change password operations. Operations run at two stages: a
 * precommit stage that runs before the operation is committed to the
 * database and a postcommit operation that runs after the database
 * is updated; see #kadm5_hook_stage for details on semantics.
 *
 * This interface is based on a proposed extension to Heimdal by Russ
 * Allbery; it is likely that Heimdal will adopt an approach based on
 * stacked kdb modules rather than this interface. For MIT, writing a
 * plugin to this interface is significantly easier than stacking kdb
 * modules. Also, the kadm5 interface is significantly more stable
 * than the kdb interface, so this approach is more desirable than
 * stacked kdb modules.
 *
 * This interface depends on kadm5/admin.h. As such, the interface
 * does not provide strong guarantees of ABI stability.
 *
 * The kadm5_hook interface currently has only one supported major version,
 * which is 1.  Major version 1 has a current minor version number of 2.
 *
 * kadm5_hook plugins should:
 * kadm5_hook_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   kadm5_hook_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                         krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to kadm5_hook_vftable_1
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* *
 * Whether the operation is being run before or after the database
 * update.
 */
    #[c2rust::src_loc = "81:1"]
    pub type kadm5_hook_stage = libc::c_uint;
    /*H_KRB5_KADM5_HOOK_PLUGIN*/
}
#[c2rust::header_src = "/usr/include/stdlib.h:7"]
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
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:7"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
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
#[c2rust::header_src = "/usr/include/assert.h:7"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin_internal.h:11"]
pub mod admin_internal_h {
    use super::krb5_h::{krb5_boolean, krb5_int32, krb5_error_code};
    use super::kdb_h::krb5_key_salt_tuple;
    extern "C" {
        /*
 * _KADM5_CHECK_HANDLE calls the function _kadm5_check_handle and
 * returns any non-zero error code that function returns.
 * _kadm5_check_handle, in client_handle.c and server_handle.c, exists
 * in both the server- and client- side libraries.  In each library,
 * it calls CHECK_HANDLE, which is defined by the appropriate
 * _internal.h header file to call GENERIC_CHECK_HANDLE as well as
 * CLIENT_CHECK_HANDLE and SERVER_CHECK_HANDLE.
 *
 * _KADM5_CHECK_HANDLE should be used by a function that needs to
 * check the handle but wants to be the same code in both the client
 * and server library; it makes a function call to the right handle
 * checker.  Code that only exists in one library can call the
 * CHECK_HANDLE macro, which inlines the test instead of making
 * another function call.
 *
 * Got that?
 */
        /* this is needed by the alt_prof code I stole.  The functions
   maybe shouldn't be named krb5_*, but they are. */
        #[no_mangle]
        #[c2rust::src_loc = "72:1"]
        pub fn krb5_string_to_keysalts(string: *const libc::c_char,
                                       tupleseps: *const libc::c_char,
                                       ksaltseps: *const libc::c_char,
                                       dups: krb5_boolean,
                                       ksaltp: *mut *mut krb5_key_salt_tuple,
                                       nksaltp: *mut krb5_int32)
         -> krb5_error_code;
    }
    /* __KADM5_ADMIN_INTERNAL_H__ */
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _profile_t, krb5_c_enctype_compare,
                       krb5_principal_compare, krb5_copy_principal,
                       krb5_free_principal, krb5_free_keyblock_contents,
                       krb5_timeofday, krb5_clear_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, ts_incr, k5calloc, zapfree,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::kdb_h::{_krb5_key_data, _krb5_tl_data, krb5_tl_data,
                      krb5_string_attr_st, krb5_string_attr, krb5_key_data,
                      _krb5_keysalt, krb5_keysalt, _krb5_db_entry_new,
                      krb5_db_entry, __krb5_key_salt_tuple,
                      krb5_key_salt_tuple, krb5_db_free_principal,
                      krb5_db_rename_principal, krb5_db_fetch_mkey_list,
                      krb5_dbe_find_enctype, krb5_dbe_decrypt_key_data,
                      krb5_dbe_encrypt_key_data, krb5_dbe_find_mkey,
                      krb5_dbe_get_mkvno, krb5_dbe_lookup_mod_princ_data,
                      krb5_dbe_update_mkvno, krb5_dbe_update_last_pwd_change,
                      krb5_dbe_lookup_last_pwd_change, krb5_dbe_get_strings,
                      krb5_dbe_set_string, krb5_dbe_update_tl_data,
                      krb5_dbe_cpw, krb5_dbe_crk, krb5_db_get_key_data_kvno,
                      krb5_dbe_sort_key_data};
pub use self::admin_h::{kadm5_ret_t, kadm5_policy_t, _kadm5_principal_ent_t,
                        kadm5_principal_ent_t, _kadm5_policy_ent_t,
                        kadm5_policy_ent_rec, kadm5_policy_ent_t,
                        _kadm5_config_params, kadm5_config_params,
                        _kadm5_key_data, kadm5_key_data,
                        kadm5_free_kadm5_key_data, kadm5_free_policy_ent,
                        kadm5_get_policy};
pub use self::server_internal_h::{osa_pw_hist_t, _osa_pw_hist_t,
                                  osa_pw_hist_ent, osa_princ_ent_rec,
                                  _osa_princ_ent_t, _kadm5_server_handle_t,
                                  kadm5_hook_handle, pwqual_handle,
                                  kadm5_server_handle_t, osa_princ_ent_t,
                                  kadm5_hook_handle_st, pwqual_handle_st,
                                  passwd_check, kdb_get_active_mkey,
                                  kdb_get_hist_key, kdb_free_keyblocks,
                                  kdb_get_entry, kdb_free_entry,
                                  kdb_put_entry, kdb_delete_entry,
                                  krb5_free_key_data_contents,
                                  k5_kadm5_hook_modify, k5_kadm5_hook_rename,
                                  k5_kadm5_hook_chpass, k5_kadm5_hook_create,
                                  k5_kadm5_hook_remove};
pub use self::kadm5_hook_plugin_h::{KADM5_HOOK_STAGE_POSTCOMMIT,
                                    KADM5_HOOK_STAGE_PRECOMMIT,
                                    kadm5_hook_stage};
use self::stdlib_h::{free, realloc, calloc, malloc};
use self::string_h::{explicit_bzero, strdup, memcmp, memset, memcpy};
use self::assert_h::__assert_fail;
use self::admin_internal_h::krb5_string_to_keysalts;
extern "C" {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
    #[no_mangle]
    #[c2rust::src_loc = "25:29"]
    pub static mut master_princ: krb5_principal;
    #[no_mangle]
    #[c2rust::src_loc = "26:29"]
    pub static mut hist_princ: krb5_principal;
    #[no_mangle]
    #[c2rust::src_loc = "27:29"]
    pub static mut master_keyblock: krb5_keyblock;
}
/* XXX this ought to be in libkrb5.a, but isn't */
/*
 * XXX Functions that ought to be in libkrb5.a, but aren't.
 */
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn krb5_copy_key_data_contents(mut context:
                                                         krb5_context,
                                                     mut from:
                                                         *mut krb5_key_data,
                                                     mut to:
                                                         *mut krb5_key_data)
 -> kadm5_ret_t {
    let mut i: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    *to = *from;
    idx =
        if (*from).key_data_ver as libc::c_int == 1 as libc::c_int {
            1 as libc::c_int
        } else { 2 as libc::c_int };
    i = 0 as libc::c_int;
    while i < idx {
        if (*from).key_data_length[i as usize] != 0 {
            (*to).key_data_contents[i as usize] =
                malloc((*from).key_data_length[i as usize] as libc::c_ulong)
                    as *mut krb5_octet;
            if (*to).key_data_contents[i as usize].is_null() {
                i = 0 as libc::c_int;
                while i < idx {
                    zapfree((*to).key_data_contents[i as usize] as
                                *mut libc::c_void,
                            (*to).key_data_length[i as usize] as size_t);
                    i += 1
                }
                return 12 as libc::c_int as kadm5_ret_t
            }
            memcpy((*to).key_data_contents[i as usize] as *mut libc::c_void,
                   (*from).key_data_contents[i as usize] as
                       *const libc::c_void,
                   (*from).key_data_length[i as usize] as libc::c_ulong);
        }
        i += 1
    }
    return 0 as libc::c_int as kadm5_ret_t;
}
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn dup_tl_data(mut tl: *mut krb5_tl_data)
 -> *mut krb5_tl_data {
    let mut n: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    n =
        malloc(::std::mem::size_of::<krb5_tl_data>() as libc::c_ulong) as
            *mut krb5_tl_data;
    if n.is_null() { return 0 as *mut krb5_tl_data }
    (*n).tl_data_contents =
        malloc((*tl).tl_data_length as libc::c_ulong) as *mut krb5_octet;
    if (*n).tl_data_contents.is_null() {
        free(n as *mut libc::c_void);
        return 0 as *mut krb5_tl_data
    }
    memcpy((*n).tl_data_contents as *mut libc::c_void,
           (*tl).tl_data_contents as *const libc::c_void,
           (*tl).tl_data_length as libc::c_ulong);
    (*n).tl_data_type = (*tl).tl_data_type;
    (*n).tl_data_length = (*tl).tl_data_length;
    (*n).tl_data_next = 0 as *mut _krb5_tl_data;
    return n;
}
/* This is in lib/kdb/kdb_cpw.c, but is static */
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn cleanup_key_data(mut context: krb5_context,
                                      mut count: libc::c_int,
                                      mut data: *mut krb5_key_data) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < count {
        krb5_free_key_data_contents(context, &mut *data.offset(i as isize));
        i += 1
    }
    free(data as *mut libc::c_void);
}
/* Check whether a ks_tuple is present in an array of ks_tuples. */
#[c2rust::src_loc = "95:1"]
unsafe extern "C" fn ks_tuple_present(mut n_ks_tuple: libc::c_int,
                                      mut ks_tuple: *mut krb5_key_salt_tuple,
                                      mut looking_for:
                                          *mut krb5_key_salt_tuple)
 -> krb5_boolean {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_ks_tuple {
        if (*ks_tuple.offset(i as isize)).ks_enctype ==
               (*looking_for).ks_enctype &&
               (*ks_tuple.offset(i as isize)).ks_salttype ==
                   (*looking_for).ks_salttype {
            return 1 as libc::c_int as krb5_boolean
        }
        i += 1
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* Fetch a policy if it exists; set *have_pol_out appropriately.  Return
 * success whether or not the policy exists. */
#[c2rust::src_loc = "111:1"]
unsafe extern "C" fn get_policy(mut handle: kadm5_server_handle_t,
                                mut name: *const libc::c_char,
                                mut policy_out: kadm5_policy_ent_t,
                                mut have_pol_out: *mut krb5_boolean)
 -> kadm5_ret_t {
    let mut ret: kadm5_ret_t = 0;
    *have_pol_out = 0 as libc::c_int as krb5_boolean;
    if name.is_null() { return 0 as libc::c_int as kadm5_ret_t }
    ret =
        kadm5_get_policy((*handle).lhandle as *mut libc::c_void,
                         name as *mut libc::c_char, policy_out);
    if ret == 0 as libc::c_int as libc::c_long {
        *have_pol_out = 1 as libc::c_int as krb5_boolean
    }
    return if ret == 43787533 as libc::c_long {
               0 as libc::c_int as libc::c_long
           } else { ret };
}
/*
 * Apply the -allowedkeysalts policy (see kadmin(1)'s addpol/modpol
 * commands).  We use the allowed key/salt tuple list as a default if
 * no ks tuples as provided by the caller.  We reject lists that include
 * key/salts outside the policy.  We re-order the requested ks tuples
 * (which may be a subset of the policy) to reflect the policy order.
 */
#[c2rust::src_loc = "133:1"]
unsafe extern "C" fn apply_keysalt_policy(mut handle: kadm5_server_handle_t,
                                          mut policy: *const libc::c_char,
                                          mut n_ks_tuple: libc::c_int,
                                          mut ks_tuple:
                                              *mut krb5_key_salt_tuple,
                                          mut new_n_kstp: *mut libc::c_int,
                                          mut new_kstp:
                                              *mut *mut krb5_key_salt_tuple)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut ret: kadm5_ret_t = 0;
    let mut polent: kadm5_policy_ent_rec =
        kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                             pw_min_life: 0,
                             pw_max_life: 0,
                             pw_min_length: 0,
                             pw_min_classes: 0,
                             pw_history_num: 0,
                             policy_refcnt: 0,
                             pw_max_fail: 0,
                             pw_failcnt_interval: 0,
                             pw_lockout_duration: 0,
                             attributes: 0,
                             max_life: 0,
                             max_renewable_life: 0,
                             allowed_keysalts: 0 as *mut libc::c_char,
                             n_tl_data: 0,
                             tl_data: 0 as *mut krb5_tl_data,};
    let mut have_polent: krb5_boolean = 0;
    let mut ak_n_ks_tuple: libc::c_int = 0 as libc::c_int;
    let mut new_n_ks_tuple: libc::c_int = 0 as libc::c_int;
    let mut ak_ks_tuple: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    let mut new_ks_tuple: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    let mut subset: *mut krb5_key_salt_tuple = 0 as *mut krb5_key_salt_tuple;
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    if !new_n_kstp.is_null() {
        *new_n_kstp = 0 as libc::c_int;
        *new_kstp = 0 as *mut krb5_key_salt_tuple
    }
    memset(&mut polent as *mut kadm5_policy_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_policy_ent_rec>() as libc::c_ulong);
    ret = get_policy(handle, policy, &mut polent, &mut have_polent);
    if !(ret != 0) {
        if polent.allowed_keysalts.is_null() {
            /* Requested keysalts allowed or default to supported_enctypes. */
            if n_ks_tuple == 0 as libc::c_int {
                /* Default to supported_enctypes. */
                n_ks_tuple = (*handle).params.num_keysalts;
                ks_tuple = (*handle).params.keysalts
            }
            /* Dup the requested or defaulted keysalt tuples. */
            new_ks_tuple =
                malloc((n_ks_tuple as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_key_salt_tuple>()
                                                            as libc::c_ulong))
                    as *mut krb5_key_salt_tuple;
            if new_ks_tuple.is_null() {
                ret = 12 as libc::c_int as kadm5_ret_t
            } else {
                memcpy(new_ks_tuple as *mut libc::c_void,
                       ks_tuple as *const libc::c_void,
                       (n_ks_tuple as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_key_salt_tuple>()
                                                            as
                                                            libc::c_ulong));
                new_n_ks_tuple = n_ks_tuple;
                ret = 0 as libc::c_int as kadm5_ret_t
            }
        } else {
            ret =
                krb5_string_to_keysalts(polent.allowed_keysalts,
                                        b",\x00" as *const u8 as
                                            *const libc::c_char,
                                        0 as *const libc::c_char,
                                        0 as libc::c_int as krb5_boolean,
                                        &mut ak_ks_tuple, &mut ak_n_ks_tuple)
                    as kadm5_ret_t;
            /*
     * Malformed policy?  Shouldn't happen, but it's remotely possible
     * someday, so we don't assert, just bail.
     */
            if !(ret != 0) {
                /* Check that the requested ks_tuples are within policy, if we have one. */
                i = 0 as libc::c_int;
                loop  {
                    if !(i < n_ks_tuple) {
                        current_block = 11932355480408055363;
                        break ;
                    }
                    if ks_tuple_present(ak_n_ks_tuple, ak_ks_tuple,
                                        &mut *ks_tuple.offset(i as isize)) ==
                           0 {
                        ret = 43787578 as libc::c_long;
                        current_block = 8020739526534111833;
                        break ;
                    } else { i += 1 }
                }
                match current_block {
                    8020739526534111833 => { }
                    _ =>
                    /* Have policy but no ks_tuple input?  Output the policy. */
                    {
                        if n_ks_tuple == 0 as libc::c_int {
                            new_n_ks_tuple = ak_n_ks_tuple;
                            new_ks_tuple = ak_ks_tuple;
                            ak_ks_tuple = 0 as *mut krb5_key_salt_tuple
                        } else {
                            /*
     * Now filter the policy ks tuples by the requested ones so as to
     * preserve in the requested sub-set the relative ordering from the
     * policy.  We could optimize this (if (n_ks_tuple == ak_n_ks_tuple)
     * then skip this), but we don't bother.
     */
                            subset =
                                calloc(n_ks_tuple as libc::c_ulong,
                                       ::std::mem::size_of::<krb5_key_salt_tuple>()
                                           as libc::c_ulong) as
                                    *mut krb5_key_salt_tuple;
                            if subset.is_null() {
                                ret = 12 as libc::c_int as kadm5_ret_t
                            } else {
                                m = 0 as libc::c_int;
                                i = 0 as libc::c_int;
                                while i < ak_n_ks_tuple && m < n_ks_tuple {
                                    if ks_tuple_present(n_ks_tuple, ks_tuple,
                                                        &mut *ak_ks_tuple.offset(i
                                                                                     as
                                                                                     isize))
                                           != 0 {
                                        let fresh0 = m;
                                        m = m + 1;
                                        *subset.offset(fresh0 as isize) =
                                            *ak_ks_tuple.offset(i as isize)
                                    }
                                    i += 1
                                }
                                new_ks_tuple = subset;
                                new_n_ks_tuple = m;
                                ret = 0 as libc::c_int as kadm5_ret_t
                            }
                        }
                    }
                }
            }
        }
    }
    if have_polent != 0 {
        kadm5_free_policy_ent((*handle).lhandle as *mut libc::c_void,
                              &mut polent);
    }
    free(ak_ks_tuple as *mut libc::c_void);
    if !new_n_kstp.is_null() {
        *new_n_kstp = new_n_ks_tuple;
        *new_kstp = new_ks_tuple
    } else { free(new_ks_tuple as *mut libc::c_void); }
    return ret;
}
/*
 * Set *passptr to NULL if the request looks like the first part of a krb5 1.6
 * addprinc -randkey operation.  The krb5 1.6 dummy password for these requests
 * was invalid UTF-8, which runs afoul of the arcfour string-to-key.
 */
#[c2rust::src_loc = "245:1"]
unsafe extern "C" fn check_1_6_dummy(mut entry: kadm5_principal_ent_t,
                                     mut mask: libc::c_long,
                                     mut n_ks_tuple: libc::c_int,
                                     mut ks_tuple: *mut krb5_key_salt_tuple,
                                     mut passptr: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut password: *mut libc::c_char = *passptr;
    /* Old-style randkey operations disallowed tickets to start. */
    if password.is_null() || mask & 0x10 as libc::c_int as libc::c_long == 0
           || (*entry).attributes & 0x40 as libc::c_int == 0 {
        return
    }
    /* The 1.6 dummy password was the octets 1..255. */
    i = 0 as libc::c_int;
    while *password.offset(i as isize) as libc::c_uchar as libc::c_int ==
              i + 1 as libc::c_int {
        i += 1
    }
    if *password.offset(i as isize) as libc::c_int != '\u{0}' as i32 ||
           i != 255 as libc::c_int {
        return
    }
    /* This will make the caller use a random password instead. */
    *passptr = 0 as *mut libc::c_char;
}
/* Return the number of keys with the newest kvno.  Assumes that all key data
 * with the newest kvno are at the front of the key data array. */
#[c2rust::src_loc = "268:1"]
unsafe extern "C" fn count_new_keys(mut n_key_data: libc::c_int,
                                    mut key_data: *mut krb5_key_data)
 -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = 1 as libc::c_int;
    while n < n_key_data {
        if (*key_data.offset((n - 1 as libc::c_int) as isize)).key_data_kvno
               as libc::c_int !=
               (*key_data.offset(n as isize)).key_data_kvno as libc::c_int {
            return n
        }
        n += 1
    }
    return n_key_data;
}
#[no_mangle]
#[c2rust::src_loc = "280:1"]
pub unsafe extern "C" fn kadm5_create_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut entry:
                                                    kadm5_principal_ent_t,
                                                mut mask: libc::c_long,
                                                mut password:
                                                    *mut libc::c_char)
 -> kadm5_ret_t {
    return kadm5_create_principal_3(server_handle, entry, mask,
                                    0 as libc::c_int,
                                    0 as *mut krb5_key_salt_tuple, password);
}
#[no_mangle]
#[c2rust::src_loc = "289:1"]
pub unsafe extern "C" fn kadm5_create_principal_3(mut server_handle:
                                                      *mut libc::c_void,
                                                  mut entry:
                                                      kadm5_principal_ent_t,
                                                  mut mask: libc::c_long,
                                                  mut n_ks_tuple: libc::c_int,
                                                  mut ks_tuple:
                                                      *mut krb5_key_salt_tuple,
                                                  mut password:
                                                      *mut libc::c_char)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut adb: osa_princ_ent_rec =
        osa_princ_ent_rec{version: 0,
                          policy: 0 as *mut libc::c_char,
                          aux_attributes: 0,
                          old_key_len: 0,
                          old_key_next: 0,
                          admin_history_kvno: 0,
                          old_keys: 0 as *mut osa_pw_hist_ent,};
    let mut polent: kadm5_policy_ent_rec =
        kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                             pw_min_life: 0,
                             pw_max_life: 0,
                             pw_min_length: 0,
                             pw_min_classes: 0,
                             pw_history_num: 0,
                             policy_refcnt: 0,
                             pw_max_fail: 0,
                             pw_failcnt_interval: 0,
                             pw_lockout_duration: 0,
                             attributes: 0,
                             max_life: 0,
                             max_renewable_life: 0,
                             allowed_keysalts: 0 as *mut libc::c_char,
                             n_tl_data: 0,
                             tl_data: 0 as *mut krb5_tl_data,};
    let mut have_polent: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut now: krb5_timestamp = 0;
    let mut tl_data_tail: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut ret: libc::c_uint = 0;
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut act_mkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut act_kvno: krb5_kvno = 0;
    let mut new_n_ks_tuple: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut new_ks_tuple: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    check_1_6_dummy(entry, mask, n_ks_tuple, ks_tuple, &mut password);
    /*
     * Argument sanity checking, and opening up the DB
     */
    if entry.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    if mask & 0x1 as libc::c_int as libc::c_long == 0 ||
           mask & 0x80 as libc::c_int as libc::c_long != 0 ||
           mask & 0x40 as libc::c_int as libc::c_long != 0 ||
           mask & 0x8 as libc::c_int as libc::c_long != 0 ||
           mask & 0x200 as libc::c_int as libc::c_long != 0 ||
           mask & 0x400 as libc::c_int as libc::c_long != 0 ||
           mask & 0x4000 as libc::c_int as libc::c_long != 0 ||
           mask & 0x8000 as libc::c_int as libc::c_long != 0 ||
           mask & 0x10000 as libc::c_int as libc::c_long != 0 {
        return 43787534 as libc::c_long
    }
    if mask & 0x20000 as libc::c_int as libc::c_long != 0 &&
           (*entry).n_key_data as libc::c_int != 0 as libc::c_int {
        return 43787534 as libc::c_long
    }
    if mask & 0x800 as libc::c_int as libc::c_long != 0 &&
           (*entry).policy.is_null() {
        return 43787534 as libc::c_long
    }
    if mask & 0x800 as libc::c_int as libc::c_long != 0 &&
           mask & 0x1000 as libc::c_int as libc::c_long != 0 {
        return 43787534 as libc::c_long
    }
    if mask &
           !(0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int |
                 0x8 as libc::c_int | 0x10 as libc::c_int |
                 0x20 as libc::c_int | 0x40 as libc::c_int |
                 0x80 as libc::c_int | 0x100 as libc::c_int |
                 0x200 as libc::c_int | 0x400 as libc::c_int |
                 0x1000 as libc::c_int | 0x800 as libc::c_int |
                 0x2000 as libc::c_int | 0x40000 as libc::c_int |
                 0x20000 as libc::c_int | 0x10000 as libc::c_int) as
               libc::c_long != 0 {
        return 43787534 as libc::c_long
    }
    if mask & 0x40000 as libc::c_int as libc::c_long != 0 {
        tl_data_tail = (*entry).tl_data;
        while !tl_data_tail.is_null() {
            if ((*tl_data_tail).tl_data_type as libc::c_int) <
                   256 as libc::c_int {
                return 43787567 as libc::c_long
            }
            tl_data_tail = (*tl_data_tail).tl_data_next
        }
    }
    /*
     * Check to see if the principal exists
     */
    ret =
        kdb_get_entry(handle, (*entry).principal, &mut kdb, &mut adb) as
            libc::c_uint;
    match ret {
        43787532 => { }
        0 => {
            kdb_free_entry(handle, kdb, &mut adb);
            return 43787527 as libc::c_long
        }
        _ => { return ret as kadm5_ret_t }
    }
    kdb =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_db_entry>() as libc::c_ulong) as
            *mut krb5_db_entry;
    if kdb.is_null() { return 12 as libc::c_int as kadm5_ret_t }
    memset(&mut adb as *mut osa_princ_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<osa_princ_ent_rec>() as libc::c_ulong);
    /*
     * If a policy was specified, load it.
     * If we can not find the one specified return an error
     */
    if mask & 0x800 as libc::c_int as libc::c_long != 0 {
        ret =
            get_policy(handle, (*entry).policy, &mut polent, &mut have_polent)
                as libc::c_uint;
        if ret != 0 {
            current_block = 6279170885573032865;
        } else { current_block = 5235537862154438448; }
    } else { current_block = 5235537862154438448; }
    match current_block {
        5235537862154438448 => {
            if !password.is_null() {
                ret =
                    passwd_check(handle, password,
                                 if have_polent != 0 {
                                     &mut polent
                                 } else { 0 as *mut kadm5_policy_ent_rec },
                                 (*entry).principal) as libc::c_uint;
                if ret != 0 {
                    current_block = 6279170885573032865;
                } else { current_block = 7252614138838059896; }
            } else { current_block = 7252614138838059896; }
            match current_block {
                6279170885573032865 => { }
                _ =>
                /*
     * Start populating the various DB fields, using the
     * "defaults" for fields that were not specified by the
     * mask.
     */
                {
                    ret =
                        krb5_timeofday((*handle).context, &mut now) as
                            libc::c_uint; /* gag me with a chainsaw */
                    if !(ret != 0) {
                        (*kdb).magic =
                            0xdbdbdbdb as libc::c_uint as krb5_magic;
                        (*kdb).len = 38 as libc::c_int as krb5_ui_2;
                        if mask & 0x10 as libc::c_int as libc::c_long != 0 {
                            (*kdb).attributes = (*entry).attributes
                        } else { (*kdb).attributes = (*handle).params.flags }
                        if mask & 0x20 as libc::c_int as libc::c_long != 0 {
                            (*kdb).max_life = (*entry).max_life
                        } else { (*kdb).max_life = (*handle).params.max_life }
                        if mask & 0x2000 as libc::c_int as libc::c_long != 0 {
                            (*kdb).max_renewable_life =
                                (*entry).max_renewable_life
                        } else {
                            (*kdb).max_renewable_life =
                                (*handle).params.max_rlife
                        }
                        if mask & 0x2 as libc::c_int as libc::c_long != 0 {
                            (*kdb).expiration = (*entry).princ_expire_time
                        } else {
                            (*kdb).expiration = (*handle).params.expiration
                        }
                        (*kdb).pw_expiration = 0 as libc::c_int;
                        if have_polent != 0 {
                            if polent.pw_max_life != 0 {
                                (*kdb).pw_expiration =
                                    ts_incr(now,
                                            polent.pw_max_life as krb5_deltat)
                            } else { (*kdb).pw_expiration = 0 as libc::c_int }
                        }
                        if mask & 0x4 as libc::c_int as libc::c_long != 0 {
                            (*kdb).pw_expiration = (*entry).pw_expiration
                        }
                        (*kdb).last_success = 0 as libc::c_int;
                        (*kdb).last_failed = 0 as libc::c_int;
                        (*kdb).fail_auth_count =
                            0 as libc::c_int as krb5_kvno;
                        /* this is kind of gross, but in order to free the tl data, I need
       to free the entire kdb entry, and that will try to free the
       principal. */
                        ret =
                            krb5_copy_principal((*handle).context,
                                                (*entry).principal as
                                                    krb5_const_principal,
                                                &mut (*kdb).princ) as
                                libc::c_uint;
                        if !(ret != 0) {
                            ret =
                                krb5_dbe_update_last_pwd_change((*handle).context,
                                                                kdb, now) as
                                    libc::c_uint;
                            if !(ret != 0) {
                                if mask &
                                       0x40000 as libc::c_int as libc::c_long
                                       != 0 {
                                    /* splice entry->tl_data onto the front of kdb->tl_data */
                                    tl_data_tail = (*entry).tl_data;
                                    loop  {
                                        if tl_data_tail.is_null() {
                                            current_block =
                                                8723848109087415604;
                                            break ;
                                        }
                                        ret =
                                            krb5_dbe_update_tl_data((*handle).context,
                                                                    kdb,
                                                                    tl_data_tail)
                                                as libc::c_uint;
                                        if ret != 0 {
                                            current_block =
                                                6279170885573032865;
                                            break ;
                                        }
                                        tl_data_tail =
                                            (*tl_data_tail).tl_data_next
                                    }
                                } else {
                                    current_block = 8723848109087415604;
                                }
                                match current_block {
                                    6279170885573032865 => { }
                                    _ => {
                                        /*
     * We need to have setup the TL data, so we have strings, so we can
     * check enctype policy, which is why we check/initialize ks_tuple
     * this late.
     */
                                        ret =
                                            apply_keysalt_policy(handle,
                                                                 (*entry).policy,
                                                                 n_ks_tuple,
                                                                 ks_tuple,
                                                                 &mut new_n_ks_tuple,
                                                                 &mut new_ks_tuple)
                                                as libc::c_uint;
                                        if !(ret != 0) {
                                            /* initialize the keys */
                                            ret =
                                                kdb_get_active_mkey(handle,
                                                                    &mut act_kvno,
                                                                    &mut act_mkey)
                                                    as libc::c_uint;
                                            if !(ret != 0) {
                                                if mask &
                                                       0x20000 as libc::c_int
                                                           as libc::c_long !=
                                                       0 {
                                                    /* The client requested no keys for this principal. */
                                                    if (*entry).n_key_data as
                                                           libc::c_int ==
                                                           0 as libc::c_int {
                                                    } else {
                                                        __assert_fail(b"entry->n_key_data == 0\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      b"svr_principal.c\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      461 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint,
                                                                      (*::std::mem::transmute::<&[u8; 110],
                                                                                                &[libc::c_char; 110]>(b"kadm5_ret_t kadm5_create_principal_3(void *, kadm5_principal_ent_t, long, int, krb5_key_salt_tuple *, char *)\x00")).as_ptr());
                                                    }
                                                } else if !password.is_null()
                                                 {
                                                    ret =
                                                        krb5_dbe_cpw((*handle).context,
                                                                     act_mkey,
                                                                     new_ks_tuple,
                                                                     new_n_ks_tuple,
                                                                     password,
                                                                     if mask &
                                                                            0x100
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_long
                                                                            !=
                                                                            0
                                                                        {
                                                                         (*entry).kvno
                                                                     } else {
                                                                         1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint
                                                                     } as
                                                                         libc::c_int,
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         krb5_boolean,
                                                                     kdb) as
                                                            libc::c_uint
                                                } else {
                                                    /* Null password means create with random key (new in 1.8). */
                                                    ret =
                                                        krb5_dbe_crk((*handle).context,
                                                                     &mut master_keyblock,
                                                                     new_ks_tuple,
                                                                     new_n_ks_tuple,
                                                                     0 as
                                                                         libc::c_int
                                                                         as
                                                                         krb5_boolean,
                                                                     kdb) as
                                                            libc::c_uint;
                                                    if mask &
                                                           0x100 as
                                                               libc::c_int as
                                                               libc::c_long !=
                                                           0 {
                                                        i = 0 as libc::c_int;
                                                        while i <
                                                                  (*kdb).n_key_data
                                                                      as
                                                                      libc::c_int
                                                              {
                                                            (*(*kdb).key_data.offset(i
                                                                                         as
                                                                                         isize)).key_data_kvno
                                                                =
                                                                (*entry).kvno
                                                                    as
                                                                    krb5_ui_2;
                                                            i += 1
                                                        }
                                                    }
                                                }
                                                if !(ret != 0) {
                                                    /* Record the master key VNO used to encrypt this entry's keys */
                                                    ret =
                                                        krb5_dbe_update_mkvno((*handle).context,
                                                                              kdb,
                                                                              act_kvno)
                                                            as libc::c_uint;
                                                    if !(ret != 0) {
                                                        ret =
                                                            k5_kadm5_hook_create((*handle).context,
                                                                                 (*handle).hook_handles,
                                                                                 KADM5_HOOK_STAGE_PRECOMMIT
                                                                                     as
                                                                                     libc::c_int,
                                                                                 entry,
                                                                                 mask,
                                                                                 new_n_ks_tuple,
                                                                                 new_ks_tuple,
                                                                                 password)
                                                                as
                                                                libc::c_uint;
                                                        if !(ret != 0) {
                                                            /* populate the admin-server-specific fields.  In the OV server,
       this used to be in a separate database.  Since there's already
       marshalling code for the admin fields, to keep things simple,
       I'm going to keep it, and make all the admin stuff occupy a
       single tl_data record, */
                                                            adb.admin_history_kvno
                                                                =
                                                                2 as
                                                                    libc::c_int
                                                                    as
                                                                    krb5_kvno;
                                                            if mask &
                                                                   0x800 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_long
                                                                   != 0 {
                                                                adb.aux_attributes
                                                                    =
                                                                    0x800 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_long;
                                                                /* this does *not* need to be strdup'ed, because adb is xdr */
        /* encoded in osa_adb_create_princ, and not ever freed */
                                                                adb.policy =
                                                                    (*entry).policy
                                                            }
                                                            /* In all cases key and the principal data is set, let the database provider know */
                                                            (*kdb).mask =
                                                                (mask |
                                                                     0x20000
                                                                         as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_long
                                                                     |
                                                                     0x1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_long)
                                                                    as
                                                                    krb5_ui_4;
                                                            /* store the new db entry */
                                                            ret =
                                                                kdb_put_entry(handle,
                                                                              kdb,
                                                                              &mut adb)
                                                                    as
                                                                    libc::c_uint;
                                                            k5_kadm5_hook_create((*handle).context,
                                                                                 (*handle).hook_handles,
                                                                                 KADM5_HOOK_STAGE_POSTCOMMIT
                                                                                     as
                                                                                     libc::c_int,
                                                                                 entry,
                                                                                 mask,
                                                                                 new_n_ks_tuple,
                                                                                 new_ks_tuple,
                                                                                 password);
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
                }
            }
        }
        _ => { }
    }
    free(new_ks_tuple as *mut libc::c_void);
    krb5_db_free_principal((*handle).context, kdb);
    if have_polent != 0 {
        kadm5_free_policy_ent((*handle).lhandle as *mut libc::c_void,
                              &mut polent);
    }
    return ret as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "525:1"]
pub unsafe extern "C" fn kadm5_delete_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut principal: krb5_principal)
 -> kadm5_ret_t {
    let mut ret: libc::c_uint = 0;
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut adb: osa_princ_ent_rec =
        osa_princ_ent_rec{version: 0,
                          policy: 0 as *mut libc::c_char,
                          aux_attributes: 0,
                          old_key_len: 0,
                          old_key_next: 0,
                          admin_history_kvno: 0,
                          old_keys: 0 as *mut osa_pw_hist_ent,};
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    if principal.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    ret =
        kdb_get_entry(handle, principal, &mut kdb, &mut adb) as libc::c_uint;
    if ret != 0 { return ret as kadm5_ret_t }
    ret =
        k5_kadm5_hook_remove((*handle).context, (*handle).hook_handles,
                             KADM5_HOOK_STAGE_PRECOMMIT as libc::c_int,
                             principal) as libc::c_uint;
    if ret != 0 {
        kdb_free_entry(handle, kdb, &mut adb);
        return ret as kadm5_ret_t
    }
    ret = kdb_delete_entry(handle, principal) as libc::c_uint;
    kdb_free_entry(handle, kdb, &mut adb);
    if ret == 0 as libc::c_int as libc::c_uint {
        k5_kadm5_hook_remove((*handle).context, (*handle).hook_handles,
                             KADM5_HOOK_STAGE_POSTCOMMIT as libc::c_int,
                             principal);
    }
    return ret as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "561:1"]
pub unsafe extern "C" fn kadm5_modify_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut entry:
                                                    kadm5_principal_ent_t,
                                                mut mask: libc::c_long)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut ret2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pol: kadm5_policy_ent_rec =
        kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                             pw_min_life: 0,
                             pw_max_life: 0,
                             pw_min_length: 0,
                             pw_min_classes: 0,
                             pw_history_num: 0,
                             policy_refcnt: 0,
                             pw_max_fail: 0,
                             pw_failcnt_interval: 0,
                             pw_lockout_duration: 0,
                             attributes: 0,
                             max_life: 0,
                             max_renewable_life: 0,
                             allowed_keysalts: 0 as *mut libc::c_char,
                             n_tl_data: 0,
                             tl_data: 0 as *mut krb5_tl_data,};
    let mut have_pol: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut tl_data_orig: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut adb: osa_princ_ent_rec =
        osa_princ_ent_rec{version: 0,
                          policy: 0 as *mut libc::c_char,
                          aux_attributes: 0,
                          old_key_len: 0,
                          old_key_next: 0,
                          admin_history_kvno: 0,
                          old_keys: 0 as *mut osa_pw_hist_ent,};
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    if entry.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    if mask & 0x1 as libc::c_int as libc::c_long != 0 ||
           mask & 0x8 as libc::c_int as libc::c_long != 0 ||
           mask & 0x40 as libc::c_int as libc::c_long != 0 ||
           mask & 0x80 as libc::c_int as libc::c_long != 0 ||
           mask & 0x200 as libc::c_int as libc::c_long != 0 ||
           mask & 0x400 as libc::c_int as libc::c_long != 0 ||
           mask & 0x20000 as libc::c_int as libc::c_long != 0 ||
           mask & 0x4000 as libc::c_int as libc::c_long != 0 ||
           mask & 0x8000 as libc::c_int as libc::c_long != 0 {
        return 43787534 as libc::c_long
    }
    if mask &
           !(0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int |
                 0x8 as libc::c_int | 0x10 as libc::c_int |
                 0x20 as libc::c_int | 0x40 as libc::c_int |
                 0x80 as libc::c_int | 0x100 as libc::c_int |
                 0x200 as libc::c_int | 0x400 as libc::c_int |
                 0x1000 as libc::c_int | 0x800 as libc::c_int |
                 0x2000 as libc::c_int | 0x40000 as libc::c_int |
                 0x20000 as libc::c_int | 0x10000 as libc::c_int) as
               libc::c_long != 0 {
        return 43787534 as libc::c_long
    }
    if mask & 0x800 as libc::c_int as libc::c_long != 0 &&
           (*entry).policy.is_null() {
        return 43787534 as libc::c_long
    }
    if mask & 0x800 as libc::c_int as libc::c_long != 0 &&
           mask & 0x1000 as libc::c_int as libc::c_long != 0 {
        return 43787534 as libc::c_long
    }
    if mask & 0x40000 as libc::c_int as libc::c_long != 0 {
        tl_data_orig = (*entry).tl_data;
        while !tl_data_orig.is_null() {
            if ((*tl_data_orig).tl_data_type as libc::c_int) <
                   256 as libc::c_int {
                return 43787567 as libc::c_long
            }
            tl_data_orig = (*tl_data_orig).tl_data_next
        }
    }
    ret = kdb_get_entry(handle, (*entry).principal, &mut kdb, &mut adb);
    if ret != 0 { return ret as kadm5_ret_t }
    /*
     * This is pretty much the same as create ...
     */
    if mask & 0x800 as libc::c_int as libc::c_long != 0 {
        ret =
            get_policy(handle, (*entry).policy, &mut pol, &mut have_pol) as
                libc::c_int;
        if ret != 0 {
            current_block = 6585862331005205431;
        } else {
            /* set us up to use the new policy */
            adb.aux_attributes |= 0x800 as libc::c_int as libc::c_long;
            if !adb.policy.is_null() {
                free(adb.policy as *mut libc::c_void);
            }
            adb.policy = strdup((*entry).policy);
            current_block = 13619784596304402172;
        }
    } else { current_block = 13619784596304402172; }
    match current_block {
        13619784596304402172 => {
            if have_pol != 0 {
                /* set pw_max_life based on new policy */
                if pol.pw_max_life != 0 {
                    ret =
                        krb5_dbe_lookup_last_pwd_change((*handle).context,
                                                        kdb,
                                                        &mut (*kdb).pw_expiration);
                    if ret != 0 {
                        current_block = 6585862331005205431;
                    } else {
                        (*kdb).pw_expiration =
                            ts_incr((*kdb).pw_expiration,
                                    pol.pw_max_life as krb5_deltat);
                        current_block = 5235537862154438448;
                    }
                } else {
                    (*kdb).pw_expiration = 0 as libc::c_int;
                    current_block = 5235537862154438448;
                }
            } else { current_block = 5235537862154438448; }
            match current_block {
                6585862331005205431 => { }
                _ => {
                    if mask & 0x1000 as libc::c_int as libc::c_long != 0 &&
                           adb.aux_attributes &
                               0x800 as libc::c_int as libc::c_long != 0 {
                        free(adb.policy as *mut libc::c_void);
                        adb.policy = 0 as *mut libc::c_char;
                        adb.aux_attributes &=
                            !(0x800 as libc::c_int) as libc::c_long;
                        (*kdb).pw_expiration = 0 as libc::c_int
                    }
                    if mask & 0x10 as libc::c_int as libc::c_long != 0 {
                        (*kdb).attributes = (*entry).attributes
                    }
                    if mask & 0x20 as libc::c_int as libc::c_long != 0 {
                        (*kdb).max_life = (*entry).max_life
                    }
                    if mask & 0x2 as libc::c_int as libc::c_long != 0 {
                        (*kdb).expiration = (*entry).princ_expire_time
                    }
                    if mask & 0x4 as libc::c_int as libc::c_long != 0 {
                        (*kdb).pw_expiration = (*entry).pw_expiration
                    }
                    if mask & 0x2000 as libc::c_int as libc::c_long != 0 {
                        (*kdb).max_renewable_life =
                            (*entry).max_renewable_life
                    }
                    if mask & 0x100 as libc::c_int as libc::c_long != 0 {
                        i = 0 as libc::c_int;
                        while i < (*kdb).n_key_data as libc::c_int {
                            (*(*kdb).key_data.offset(i as
                                                         isize)).key_data_kvno
                                = (*entry).kvno as krb5_ui_2;
                            i += 1
                        }
                    }
                    if mask & 0x40000 as libc::c_int as libc::c_long != 0 {
                        let mut tl: *mut krb5_tl_data =
                            0 as *mut krb5_tl_data;
                        /* may have to change the version number of the API. Updates the list with the given tl_data rather than over-writing */
                        tl = (*entry).tl_data;
                        loop  {
                            if tl.is_null() {
                                current_block = 14294131666767243020;
                                break ;
                            }
                            ret =
                                krb5_dbe_update_tl_data((*handle).context,
                                                        kdb, tl);
                            if ret != 0 {
                                current_block = 6585862331005205431;
                                break ;
                            }
                            tl = (*tl).tl_data_next
                        }
                    } else { current_block = 14294131666767243020; }
                    match current_block {
                        6585862331005205431 => { }
                        _ =>
                        /*
     * Setting entry->fail_auth_count to 0 can be used to manually unlock
     * an account. It is not possible to set fail_auth_count to any other
     * value using kadmin.
     */
                        {
                            if mask & 0x10000 as libc::c_int as libc::c_long
                                   != 0 {
                                if (*entry).fail_auth_count !=
                                       0 as libc::c_int as libc::c_uint {
                                    ret =
                                        43787563 as libc::c_long as
                                            libc::c_int;
                                    current_block = 6585862331005205431;
                                } else {
                                    (*kdb).fail_auth_count =
                                        0 as libc::c_int as krb5_kvno;
                                    current_block = 2750570471926810434;
                                }
                            } else { current_block = 2750570471926810434; }
                            match current_block {
                                6585862331005205431 => { }
                                _ => {
                                    /* let the mask propagate to the database provider */
                                    (*kdb).mask = mask as krb5_ui_4;
                                    ret =
                                        k5_kadm5_hook_modify((*handle).context,
                                                             (*handle).hook_handles,
                                                             KADM5_HOOK_STAGE_PRECOMMIT
                                                                 as
                                                                 libc::c_int,
                                                             entry, mask) as
                                            libc::c_int;
                                    if !(ret != 0) {
                                        ret =
                                            kdb_put_entry(handle, kdb,
                                                          &mut adb);
                                        if !(ret != 0) {
                                            k5_kadm5_hook_modify((*handle).context,
                                                                 (*handle).hook_handles,
                                                                 KADM5_HOOK_STAGE_POSTCOMMIT
                                                                     as
                                                                     libc::c_int,
                                                                 entry, mask);
                                            ret = 0 as libc::c_int
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    if have_pol != 0 {
        ret2 =
            kadm5_free_policy_ent((*handle).lhandle as *mut libc::c_void,
                                  &mut pol) as libc::c_int;
        ret = if ret != 0 { ret } else { ret2 }
    }
    kdb_free_entry(handle, kdb, &mut adb);
    return ret as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "708:1"]
pub unsafe extern "C" fn kadm5_rename_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut source: krb5_principal,
                                                mut target: krb5_principal)
 -> kadm5_ret_t {
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut adb: osa_princ_ent_rec =
        osa_princ_ent_rec{version: 0,
                          policy: 0 as *mut libc::c_char,
                          aux_attributes: 0,
                          old_key_len: 0,
                          old_key_next: 0,
                          admin_history_kvno: 0,
                          old_keys: 0 as *mut osa_pw_hist_ent,};
    let mut ret: krb5_error_code = 0;
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    if source.is_null() || target.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    ret = kdb_get_entry(handle, target, &mut kdb, &mut adb);
    if ret == 0 as libc::c_int {
        kdb_free_entry(handle, kdb, &mut adb);
        return 43787527 as libc::c_long
    }
    ret =
        k5_kadm5_hook_rename((*handle).context, (*handle).hook_handles,
                             KADM5_HOOK_STAGE_PRECOMMIT as libc::c_int,
                             source, target) as krb5_error_code;
    if ret != 0 { return ret as kadm5_ret_t }
    ret = krb5_db_rename_principal((*handle).context, source, target);
    if ret != 0 { return ret as kadm5_ret_t }
    /* Update the principal mod data. */
    ret = kdb_get_entry(handle, target, &mut kdb, &mut adb);
    if ret != 0 { return ret as kadm5_ret_t }
    (*kdb).mask = 0 as libc::c_int as krb5_ui_4;
    ret = kdb_put_entry(handle, kdb, &mut adb);
    kdb_free_entry(handle, kdb, &mut adb);
    if ret != 0 { return ret as kadm5_ret_t }
    k5_kadm5_hook_rename((*handle).context, (*handle).hook_handles,
                         KADM5_HOOK_STAGE_POSTCOMMIT as libc::c_int, source,
                         target);
    return 0 as libc::c_int as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "753:1"]
pub unsafe extern "C" fn kadm5_get_principal(mut server_handle:
                                                 *mut libc::c_void,
                                             mut principal: krb5_principal,
                                             mut entry: kadm5_principal_ent_t,
                                             mut in_mask: libc::c_long)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut adb: osa_princ_ent_rec =
        osa_princ_ent_rec{version: 0,
                          policy: 0 as *mut libc::c_char,
                          aux_attributes: 0,
                          old_key_len: 0,
                          old_key_next: 0,
                          admin_history_kvno: 0,
                          old_keys: 0 as *mut osa_pw_hist_ent,};
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut mask: libc::c_long = 0;
    let mut i: libc::c_int = 0;
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    /*
     * In version 1, all the defined fields are always returned.
     * entry is a pointer to a kadm5_principal_ent_t_v1 that should be
     * filled with allocated memory.
     */
    mask = in_mask;
    memset(entry as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<_kadm5_principal_ent_t>() as libc::c_ulong);
    if principal.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    ret = kdb_get_entry(handle, principal, &mut kdb, &mut adb);
    if ret != 0 { return ret as kadm5_ret_t }
    if mask & 0x800 as libc::c_int as libc::c_long != 0 &&
           !adb.policy.is_null() &&
           adb.aux_attributes & 0x800 as libc::c_int as libc::c_long != 0 {
        (*entry).policy = strdup(adb.policy);
        if (*entry).policy.is_null() {
            ret = 12 as libc::c_int;
            current_block = 14364226823821194100;
        } else { current_block = 17500079516916021833; }
    } else { current_block = 17500079516916021833; }
    match current_block {
        17500079516916021833 => {
            if mask & 0x400 as libc::c_int as libc::c_long != 0 {
                (*entry).aux_attributes = adb.aux_attributes
            }
            if !(mask & 0x1 as libc::c_int as libc::c_long != 0 &&
                     {
                         ret =
                             krb5_copy_principal((*handle).context,
                                                 (*kdb).princ as
                                                     krb5_const_principal,
                                                 &mut (*entry).principal);
                         (ret) != 0
                     }) {
                if mask & 0x2 as libc::c_int as libc::c_long != 0 {
                    (*entry).princ_expire_time = (*kdb).expiration
                }
                if !(mask & 0x8 as libc::c_int as libc::c_long != 0 &&
                         {
                             ret =
                                 krb5_dbe_lookup_last_pwd_change((*handle).context,
                                                                 kdb,
                                                                 &mut (*entry).last_pwd_change);
                             (ret) != 0
                         }) {
                    if mask & 0x4 as libc::c_int as libc::c_long != 0 {
                        (*entry).pw_expiration = (*kdb).pw_expiration
                    }
                    if mask & 0x20 as libc::c_int as libc::c_long != 0 {
                        (*entry).max_life = (*kdb).max_life
                    }
                    /* this is a little non-sensical because the function returns two */
    /* values that must be checked separately against the mask */
                    if mask & 0x80 as libc::c_int as libc::c_long != 0 ||
                           mask & 0x40 as libc::c_int as libc::c_long != 0 {
                        ret =
                            krb5_dbe_lookup_mod_princ_data((*handle).context,
                                                           kdb,
                                                           &mut (*entry).mod_date,
                                                           &mut (*entry).mod_name);
                        if ret != 0 {
                            current_block = 14364226823821194100;
                        } else {
                            if mask & 0x40 as libc::c_int as libc::c_long == 0
                               {
                                (*entry).mod_date = 0 as libc::c_int
                            }
                            if mask & 0x80 as libc::c_int as libc::c_long == 0
                               {
                                krb5_free_principal((*handle).context,
                                                    (*entry).mod_name);
                                (*entry).mod_name = 0 as krb5_principal
                            }
                            current_block = 2290177392965769716;
                        }
                    } else { current_block = 2290177392965769716; }
                    match current_block {
                        14364226823821194100 => { }
                        _ => {
                            if mask & 0x10 as libc::c_int as libc::c_long != 0
                               {
                                (*entry).attributes = (*kdb).attributes
                            }
                            if mask & 0x100 as libc::c_int as libc::c_long !=
                                   0 {
                                (*entry).kvno = 0 as libc::c_int as krb5_kvno;
                                i = 0 as libc::c_int;
                                while i < (*kdb).n_key_data as libc::c_int {
                                    if (*(*kdb).key_data.offset(i as
                                                                    isize)).key_data_kvno
                                           as krb5_kvno > (*entry).kvno {
                                        (*entry).kvno =
                                            (*(*kdb).key_data.offset(i as
                                                                         isize)).key_data_kvno
                                                as krb5_kvno
                                    }
                                    i += 1
                                }
                            }
                            if mask & 0x200 as libc::c_int as libc::c_long !=
                                   0 {
                                ret =
                                    krb5_dbe_get_mkvno((*handle).context, kdb,
                                                       &mut (*entry).mkvno);
                                if ret != 0 {
                                    current_block = 14364226823821194100;
                                } else {
                                    current_block = 3580086814630675314;
                                }
                            } else { current_block = 3580086814630675314; }
                            match current_block {
                                14364226823821194100 => { }
                                _ => {
                                    if mask &
                                           0x2000 as libc::c_int as
                                               libc::c_long != 0 {
                                        (*entry).max_renewable_life =
                                            (*kdb).max_renewable_life
                                    }
                                    if mask &
                                           0x4000 as libc::c_int as
                                               libc::c_long != 0 {
                                        (*entry).last_success =
                                            (*kdb).last_success
                                    }
                                    if mask &
                                           0x8000 as libc::c_int as
                                               libc::c_long != 0 {
                                        (*entry).last_failed =
                                            (*kdb).last_failed
                                    }
                                    if mask &
                                           0x10000 as libc::c_int as
                                               libc::c_long != 0 {
                                        (*entry).fail_auth_count =
                                            (*kdb).fail_auth_count
                                    }
                                    if mask &
                                           0x40000 as libc::c_int as
                                               libc::c_long != 0 {
                                        let mut tl: *mut krb5_tl_data =
                                            0 as *mut krb5_tl_data;
                                        let mut tl2: *mut krb5_tl_data =
                                            0 as *mut krb5_tl_data;
                                        (*entry).tl_data =
                                            0 as *mut krb5_tl_data;
                                        tl = (*kdb).tl_data;
                                        loop  {
                                            if tl.is_null() {
                                                current_block =
                                                    14294131666767243020;
                                                break ;
                                            }
                                            if (*tl).tl_data_type as
                                                   libc::c_int >
                                                   255 as libc::c_int {
                                                tl2 = dup_tl_data(tl);
                                                if tl2.is_null() {
                                                    ret = 12 as libc::c_int;
                                                    current_block =
                                                        14364226823821194100;
                                                    break ;
                                                } else {
                                                    (*tl2).tl_data_next =
                                                        (*entry).tl_data;
                                                    (*entry).tl_data = tl2;
                                                    (*entry).n_tl_data += 1
                                                }
                                            }
                                            tl = (*tl).tl_data_next
                                        }
                                    } else {
                                        current_block = 14294131666767243020;
                                    }
                                    match current_block {
                                        14364226823821194100 => { }
                                        _ => {
                                            if mask &
                                                   0x20000 as libc::c_int as
                                                       libc::c_long != 0 {
                                                (*entry).n_key_data =
                                                    (*kdb).n_key_data;
                                                if (*entry).n_key_data != 0 {
                                                    (*entry).key_data =
                                                        k5calloc((*entry).n_key_data
                                                                     as
                                                                     size_t,
                                                                 ::std::mem::size_of::<krb5_key_data>()
                                                                     as
                                                                     libc::c_ulong,
                                                                 &mut ret) as
                                                            *mut krb5_key_data;
                                                    if (*entry).key_data.is_null()
                                                       {
                                                        current_block =
                                                            14364226823821194100;
                                                    } else {
                                                        current_block =
                                                            9505035279996566320;
                                                    }
                                                } else {
                                                    (*entry).key_data =
                                                        0 as
                                                            *mut krb5_key_data;
                                                    current_block =
                                                        9505035279996566320;
                                                }
                                                match current_block {
                                                    14364226823821194100 => {
                                                    }
                                                    _ => {
                                                        i = 0 as libc::c_int;
                                                        while i <
                                                                  (*entry).n_key_data
                                                                      as
                                                                      libc::c_int
                                                              {
                                                            ret =
                                                                krb5_copy_key_data_contents((*handle).context,
                                                                                            &mut *(*kdb).key_data.offset(i
                                                                                                                             as
                                                                                                                             isize),
                                                                                            &mut *(*entry).key_data.offset(i
                                                                                                                               as
                                                                                                                               isize))
                                                                    as
                                                                    krb5_error_code;
                                                            i += 1
                                                        }
                                                        if ret != 0 {
                                                            current_block =
                                                                14364226823821194100;
                                                        } else {
                                                            current_block =
                                                                17395932908762866334;
                                                        }
                                                    }
                                                }
                                            } else {
                                                current_block =
                                                    17395932908762866334;
                                            }
                                            match current_block {
                                                14364226823821194100 => { }
                                                _ => {
                                                    ret = 0 as libc::c_int
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
        }
        _ => { }
    }
    if ret != 0 && !(*entry).principal.is_null() {
        krb5_free_principal((*handle).context, (*entry).principal);
        (*entry).principal = 0 as krb5_principal
    }
    kdb_free_entry(handle, kdb, &mut adb);
    return ret as kadm5_ret_t;
}
/*
 * Function: check_pw_reuse
 *
 * Purpose: Check if a key appears in a list of keys, in order to
 * enforce password history.
 *
 * Arguments:
 *
 *      context                 (r) the krb5 context
 *      hist_keyblock           (r) the key that hist_key_data is
 *                              encrypted in
 *      n_new_key_data          (r) length of new_key_data
 *      new_key_data            (r) keys to check against
 *                              pw_hist_data, encrypted in hist_keyblock
 *      n_pw_hist_data          (r) length of pw_hist_data
 *      pw_hist_data            (r) passwords to check new_key_data against
 *
 * Effects:
 * For each new_key in new_key_data:
 *      decrypt new_key with the master_keyblock
 *      for each password in pw_hist_data:
 *              for each hist_key in password:
 *                      decrypt hist_key with hist_keyblock
 *                      compare the new_key and hist_key
 *
 * Returns krb5 errors, KADM5_PASS_RESUSE if a key in
 * new_key_data is the same as a key in pw_hist_data, or 0.
 */
#[c2rust::src_loc = "933:1"]
unsafe extern "C" fn check_pw_reuse(mut context: krb5_context,
                                    mut hist_keyblocks: *mut krb5_keyblock,
                                    mut n_new_key_data: libc::c_int,
                                    mut new_key_data: *mut krb5_key_data,
                                    mut n_pw_hist_data: libc::c_uint,
                                    mut pw_hist_data: *mut osa_pw_hist_ent)
 -> kadm5_ret_t {
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut z: libc::c_uint = 0;
    let mut newkey: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *const krb5_octet as *mut krb5_octet,};
    let mut histkey: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *const krb5_octet as *mut krb5_octet,};
    let mut kb: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut key_data: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut ret: krb5_error_code = 0;
    if n_new_key_data >= 0 as libc::c_int {
    } else {
        __assert_fail(b"n_new_key_data >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"svr_principal.c\x00" as *const u8 as
                          *const libc::c_char,
                      944 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 113],
                                                &[libc::c_char; 113]>(b"kadm5_ret_t check_pw_reuse(krb5_context, krb5_keyblock *, int, krb5_key_data *, unsigned int, osa_pw_hist_ent *)\x00")).as_ptr());
    }
    x = 0 as libc::c_int as libc::c_uint;
    while x < n_new_key_data as libc::c_uint {
        /* Check only entries with the most recent kvno. */
        if (*new_key_data.offset(x as isize)).key_data_kvno as libc::c_int !=
               (*new_key_data.offset(0 as libc::c_int as isize)).key_data_kvno
                   as libc::c_int {
            break ;
        }
        ret =
            krb5_dbe_decrypt_key_data(context, 0 as *const krb5_keyblock,
                                      &mut *new_key_data.offset(x as isize),
                                      &mut newkey, 0 as *mut krb5_keysalt);
        if ret != 0 { return ret as kadm5_ret_t }
        y = 0 as libc::c_int as libc::c_uint;
        while y < n_pw_hist_data {
            z = 0 as libc::c_int as libc::c_uint;
            while z <
                      (*pw_hist_data.offset(y as isize)).n_key_data as
                          libc::c_uint {
                kb = hist_keyblocks;
                while (*kb).enctype != 0 as libc::c_int {
                    key_data =
                        &mut *(*pw_hist_data.offset(y as
                                                        isize)).key_data.offset(z
                                                                                    as
                                                                                    isize)
                            as *mut krb5_key_data;
                    ret =
                        krb5_dbe_decrypt_key_data(context, kb, key_data,
                                                  &mut histkey,
                                                  0 as *mut krb5_keysalt);
                    if !(ret != 0) {
                        if newkey.length == histkey.length &&
                               newkey.enctype == histkey.enctype &&
                               memcmp(newkey.contents as *const libc::c_void,
                                      histkey.contents as *const libc::c_void,
                                      histkey.length as libc::c_ulong) ==
                                   0 as libc::c_int {
                            krb5_free_keyblock_contents(context,
                                                        &mut histkey);
                            krb5_free_keyblock_contents(context, &mut newkey);
                            return 43787545 as libc::c_long
                        }
                        krb5_free_keyblock_contents(context, &mut histkey);
                    }
                    kb = kb.offset(1)
                }
                z = z.wrapping_add(1)
            }
            y = y.wrapping_add(1)
        }
        krb5_free_keyblock_contents(context, &mut newkey);
        x = x.wrapping_add(1)
    }
    return 0 as libc::c_int as kadm5_ret_t;
}
#[c2rust::src_loc = "979:1"]
unsafe extern "C" fn free_history_entry(mut context: krb5_context,
                                        mut hist: *mut osa_pw_hist_ent) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*hist).n_key_data {
        krb5_free_key_data_contents(context,
                                    &mut *(*hist).key_data.offset(i as
                                                                      isize));
        i += 1
    }
    free((*hist).key_data as *mut libc::c_void);
}
/*
 * Function: create_history_entry
 *
 * Purpose: Creates a password history entry from an array of
 * key_data.
 *
 * Arguments:
 *
 *      context         (r) krb5_context to use
 *      mkey            (r) master keyblock to decrypt key data with
 *      hist_key        (r) history keyblock to encrypt key data with
 *      n_key_data      (r) number of elements in key_data
 *      key_data        (r) keys to add to the history entry
 *      hist_out        (w) history entry to fill in
 *
 * Effects:
 *
 * hist->key_data is allocated to store n_key_data key_datas.  Each
 * element of key_data is decrypted with master_keyblock, re-encrypted
 * in hist_key, and added to hist->key_data.  hist->n_key_data is
 * set to n_key_data.
 */
#[c2rust::src_loc = "1011:1"]
unsafe extern "C" fn create_history_entry(mut context: krb5_context,
                                          mut hist_key: *mut krb5_keyblock,
                                          mut n_key_data: libc::c_int,
                                          mut key_data: *mut krb5_key_data,
                                          mut hist_out: *mut osa_pw_hist_ent)
 -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *const krb5_octet as *mut krb5_octet,};
    let mut salt: krb5_keysalt =
        krb5_keysalt{type_0: 0,
                     data:
                         krb5_data{magic: 0,
                                   length: 0,
                                   data: 0 as *mut libc::c_char,},};
    let mut kvno: krb5_ui_2 = 0;
    let mut hist: osa_pw_hist_ent =
        osa_pw_hist_ent{n_key_data: 0, key_data: 0 as *mut krb5_key_data,};
    (*hist_out).key_data = 0 as *mut krb5_key_data;
    (*hist_out).n_key_data = 0 as libc::c_int;
    if n_key_data < 0 as libc::c_int { return 22 as libc::c_int }
    memset(&mut key as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    memset(&mut hist as *mut osa_pw_hist_ent as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<osa_pw_hist_ent>() as libc::c_ulong);
    if !(n_key_data == 0 as libc::c_int) {
        hist.key_data =
            k5calloc(n_key_data as size_t,
                     ::std::mem::size_of::<krb5_key_data>() as libc::c_ulong,
                     &mut ret) as *mut krb5_key_data;
        if !hist.key_data.is_null() {
            /* We only want to store the most recent kvno, and key_data should already
     * be sorted in descending order by kvno. */
            kvno =
                (*key_data.offset(0 as libc::c_int as isize)).key_data_kvno;
            i = 0 as libc::c_int;
            loop  {
                if !(i < n_key_data) {
                    current_block = 5783071609795492627;
                    break ;
                }
                if ((*key_data.offset(i as isize)).key_data_kvno as
                        libc::c_int) < kvno as libc::c_int {
                    current_block = 5783071609795492627;
                    break ;
                    /* krb5_free_keysalt(context, &salt); */
                }
                ret =
                    krb5_dbe_decrypt_key_data(context,
                                              0 as *const krb5_keyblock,
                                              &mut *key_data.offset(i as
                                                                        isize),
                                              &mut key, &mut salt);
                if ret != 0 { current_block = 12992199543102285143; break ; }
                ret =
                    krb5_dbe_encrypt_key_data(context, hist_key, &mut key,
                                              &mut salt,
                                              (*key_data.offset(i as
                                                                    isize)).key_data_kvno
                                                  as libc::c_int,
                                              &mut *hist.key_data.offset(hist.n_key_data
                                                                             as
                                                                             isize));
                if ret != 0 { current_block = 12992199543102285143; break ; }
                hist.n_key_data += 1;
                krb5_free_keyblock_contents(context, &mut key);
                i += 1
            }
            match current_block {
                12992199543102285143 => { }
                _ => {
                    *hist_out = hist;
                    hist.n_key_data = 0 as libc::c_int;
                    hist.key_data = 0 as *mut krb5_key_data
                }
            }
        }
    }
    krb5_free_keyblock_contents(context, &mut key);
    free_history_entry(context, &mut hist);
    return ret;
}
/*
 * Function: add_to_history
 *
 * Purpose: Adds a password to a principal's password history.
 *
 * Arguments:
 *
 *      context         (r) krb5_context to use
 *      hist_kvno       (r) kvno of current history key
 *      adb             (r/w) admin principal entry to add keys to
 *      pol             (r) adb's policy
 *      pw              (r) keys for the password to add to adb's key history
 *
 * Effects:
 *
 * add_to_history adds a single password to adb's password history.
 * pw contains n_key_data keys in its key_data, in storage should be
 * allocated but not freed by the caller (XXX blech!).
 *
 * This function maintains adb->old_keys as a circular queue.  It
 * starts empty, and grows each time this function is called until it
 * is pol->pw_history_num items long.  adb->old_key_len holds the
 * number of allocated entries in the array, and must therefore be [0,
 * pol->pw_history_num).  adb->old_key_next is the index into the
 * array where the next element should be written, and must be [0,
 * adb->old_key_len).
 */
#[c2rust::src_loc = "1099:1"]
unsafe extern "C" fn add_to_history(mut context: krb5_context,
                                    mut hist_kvno: krb5_kvno,
                                    mut adb: osa_princ_ent_t,
                                    mut pol: kadm5_policy_ent_t,
                                    mut pw: *mut osa_pw_hist_ent)
 -> kadm5_ret_t {
    let mut histp: *mut osa_pw_hist_ent = 0 as *mut osa_pw_hist_ent;
    let mut nhist: uint32_t = 0;
    let mut i: libc::c_uint = 0;
    let mut knext: libc::c_uint = 0;
    let mut nkeys: libc::c_uint = 0;
    nhist = (*pol).pw_history_num as uint32_t;
    /* A history of 1 means just check the current password */
    if nhist <= 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as kadm5_ret_t
    }
    if (*adb).admin_history_kvno != hist_kvno {
        /* The history key has changed since the last password change, so we
         * have to reset the password history. */
        free((*adb).old_keys as *mut libc::c_void);
        (*adb).old_keys = 0 as *mut osa_pw_hist_ent;
        (*adb).old_key_len = 0 as libc::c_int as libc::c_uint;
        (*adb).old_key_next = 0 as libc::c_int as libc::c_uint;
        (*adb).admin_history_kvno = hist_kvno
    }
    nkeys = (*adb).old_key_len;
    knext = (*adb).old_key_next;
    /* resize the adb->old_keys array if necessary */
    if nkeys.wrapping_add(1 as libc::c_int as libc::c_uint) < nhist {
        if (*adb).old_keys.is_null() {
            (*adb).old_keys =
                malloc((nkeys.wrapping_add(1 as libc::c_int as libc::c_uint)
                            as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<osa_pw_hist_ent>()
                                                            as libc::c_ulong))
                    as *mut osa_pw_hist_ent
        } else {
            (*adb).old_keys =
                realloc((*adb).old_keys as *mut libc::c_void,
                        (nkeys.wrapping_add(1 as libc::c_int as libc::c_uint)
                             as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<osa_pw_hist_ent>()
                                                             as
                                                             libc::c_ulong))
                    as *mut osa_pw_hist_ent
        }
        if (*adb).old_keys.is_null() {
            return 12 as libc::c_int as kadm5_ret_t
        }
        memset(&mut *(*adb).old_keys.offset(nkeys as isize) as
                   *mut osa_pw_hist_ent as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<osa_pw_hist_ent>() as libc::c_ulong);
        (*adb).old_key_len = (*adb).old_key_len.wrapping_add(1);
        nkeys = (*adb).old_key_len;
        /*
         * To avoid losing old keys, shift forward each entry after
         * knext.
         */
        i = nkeys.wrapping_sub(1 as libc::c_int as libc::c_uint);
        while i > knext {
            *(*adb).old_keys.offset(i as isize) =
                *(*adb).old_keys.offset(i.wrapping_sub(1 as libc::c_int as
                                                           libc::c_uint) as
                                            isize);
            i = i.wrapping_sub(1)
        }
        memset(&mut *(*adb).old_keys.offset(knext as isize) as
                   *mut osa_pw_hist_ent as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<osa_pw_hist_ent>() as libc::c_ulong);
    } else if nkeys.wrapping_add(1 as libc::c_int as libc::c_uint) > nhist {
        /*
         * The policy must have changed!  Shrink the array.
         * Can't simply realloc() down, since it might be wrapped.
         * To understand the arithmetic below, note that we are
         * copying into new positions 0 .. N-1 from old positions
         * old_key_next-N .. old_key_next-1, modulo old_key_len,
         * where N = pw_history_num - 1 is the length of the
         * shortened list.        Matt Crawford, FNAL
         */
        /*
         * M = adb->old_key_len, N = pol->pw_history_num - 1
         *
         * tmp[0] .. tmp[N-1] = old[(knext-N)%M] .. old[(knext-1)%M]
         */
        let mut j: libc::c_int = 0;
        let mut tmp: osa_pw_hist_t = 0 as *mut _osa_pw_hist_t;
        tmp =
            malloc((nhist.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<osa_pw_hist_ent>()
                                                        as libc::c_ulong)) as
                *mut osa_pw_hist_ent;
        if tmp.is_null() { return 12 as libc::c_int as kadm5_ret_t }
        i = 0 as libc::c_int as libc::c_uint;
        while i < nhist.wrapping_sub(1 as libc::c_int as libc::c_uint) {
            /*
             * Add nkeys once before taking remainder to avoid
             * negative values.
             */
            j =
                i.wrapping_add(nkeys).wrapping_add(knext).wrapping_sub(nhist.wrapping_sub(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint)).wrapping_rem(nkeys)
                    as libc::c_int;
            *tmp.offset(i as isize) = *(*adb).old_keys.offset(j as isize);
            i = i.wrapping_add(1)
        }
        /* Now free the ones we don't keep (the oldest ones) */
        i = 0 as libc::c_int as libc::c_uint;
        while i <
                  nkeys.wrapping_sub(nhist.wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint)) {
            j =
                i.wrapping_add(nkeys).wrapping_add(knext).wrapping_rem(nkeys)
                    as libc::c_int;
            histp =
                &mut *(*adb).old_keys.offset(j as isize) as
                    *mut osa_pw_hist_ent;
            j = 0 as libc::c_int;
            while j < (*histp).n_key_data {
                krb5_free_key_data_contents(context,
                                            &mut *(*histp).key_data.offset(j
                                                                               as
                                                                               isize));
                j += 1
            }
            free((*histp).key_data as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        free((*adb).old_keys as *mut libc::c_void);
        (*adb).old_keys = tmp;
        (*adb).old_key_len =
            nhist.wrapping_sub(1 as libc::c_int as libc::c_uint);
        nkeys = (*adb).old_key_len;
        (*adb).old_key_next = 0 as libc::c_int as libc::c_uint;
        knext = (*adb).old_key_next
    }
    /*
     * If nhist decreased since the last password change, and nkeys+1
     * is less than the previous nhist, it is possible for knext to
     * index into unallocated space.  This condition would not be
     * caught by the resizing code above.
     */
    if knext.wrapping_add(1 as libc::c_int as libc::c_uint) > nkeys {
        (*adb).old_key_next = 0 as libc::c_int as libc::c_uint;
        knext = (*adb).old_key_next
    }
    /* free the old pw history entry if it contains data */
    histp =
        &mut *(*adb).old_keys.offset(knext as isize) as *mut osa_pw_hist_ent;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*histp).n_key_data as libc::c_uint {
        krb5_free_key_data_contents(context,
                                    &mut *(*histp).key_data.offset(i as
                                                                       isize));
        i = i.wrapping_add(1)
    }
    free((*histp).key_data as *mut libc::c_void);
    /* store the new entry */
    *(*adb).old_keys.offset(knext as isize) = *pw;
    /* update the next pointer */
    (*adb).old_key_next = (*adb).old_key_next.wrapping_add(1);
    if (*adb).old_key_next ==
           nhist.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        (*adb).old_key_next = 0 as libc::c_int as libc::c_uint
    }
    return 0 as libc::c_int as kadm5_ret_t;
}
/* FIXME: don't use global variable for this */
#[no_mangle]
#[c2rust::src_loc = "1219:14"]
pub static mut use_password_server: krb5_boolean =
    0 as libc::c_int as krb5_boolean;
#[no_mangle]
#[c2rust::src_loc = "1231:1"]
pub unsafe extern "C" fn kadm5_set_use_password_server() {
    use_password_server = 1 as libc::c_int as krb5_boolean;
}
#[no_mangle]
#[c2rust::src_loc = "1321:1"]
pub unsafe extern "C" fn kadm5_chpass_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut principal: krb5_principal,
                                                mut password:
                                                    *mut libc::c_char)
 -> kadm5_ret_t {
    return kadm5_chpass_principal_3(server_handle, principal,
                                    0 as libc::c_int as krb5_boolean,
                                    0 as libc::c_int,
                                    0 as *mut krb5_key_salt_tuple, password);
}
#[no_mangle]
#[c2rust::src_loc = "1330:1"]
pub unsafe extern "C" fn kadm5_chpass_principal_3(mut server_handle:
                                                      *mut libc::c_void,
                                                  mut principal:
                                                      krb5_principal,
                                                  mut keepold: krb5_boolean,
                                                  mut n_ks_tuple: libc::c_int,
                                                  mut ks_tuple:
                                                      *mut krb5_key_salt_tuple,
                                                  mut password:
                                                      *mut libc::c_char)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut now: krb5_timestamp = 0;
    let mut pol: kadm5_policy_ent_rec =
        kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                             pw_min_life: 0,
                             pw_max_life: 0,
                             pw_min_length: 0,
                             pw_min_classes: 0,
                             pw_history_num: 0,
                             policy_refcnt: 0,
                             pw_max_fail: 0,
                             pw_failcnt_interval: 0,
                             pw_lockout_duration: 0,
                             attributes: 0,
                             max_life: 0,
                             max_renewable_life: 0,
                             allowed_keysalts: 0 as *mut libc::c_char,
                             n_tl_data: 0,
                             tl_data: 0 as *mut krb5_tl_data,};
    let mut adb: osa_princ_ent_rec =
        osa_princ_ent_rec{version: 0,
                          policy: 0 as *mut libc::c_char,
                          aux_attributes: 0,
                          old_key_len: 0,
                          old_key_next: 0,
                          admin_history_kvno: 0,
                          old_keys: 0 as *mut osa_pw_hist_ent,};
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut ret: libc::c_int = 0;
    let mut ret2: libc::c_int = 0;
    let mut hist_added: libc::c_int = 0;
    let mut have_pol: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut hist: osa_pw_hist_ent =
        osa_pw_hist_ent{n_key_data: 0, key_data: 0 as *mut krb5_key_data,};
    let mut act_mkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut hist_keyblocks: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut act_kvno: krb5_kvno = 0;
    let mut hist_kvno: krb5_kvno = 0;
    let mut new_n_ks_tuple: libc::c_int = 0 as libc::c_int;
    let mut new_ks_tuple: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    hist_added = 0 as libc::c_int;
    memset(&mut hist as *mut osa_pw_hist_ent as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<osa_pw_hist_ent>() as libc::c_ulong);
    if principal.is_null() || password.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    if krb5_principal_compare((*handle).context,
                              principal as krb5_const_principal,
                              hist_princ as krb5_const_principal) ==
           1 as libc::c_int as libc::c_uint {
        return 43787550 as libc::c_long
    }
    ret = kdb_get_entry(handle, principal, &mut kdb, &mut adb);
    if ret != 0 { return ret as kadm5_ret_t }
    ret =
        apply_keysalt_policy(handle, adb.policy, n_ks_tuple, ks_tuple,
                             &mut new_n_ks_tuple, &mut new_ks_tuple) as
            libc::c_int;
    if !(ret != 0) {
        if adb.aux_attributes & 0x800 as libc::c_int as libc::c_long != 0 {
            ret =
                get_policy(handle, adb.policy, &mut pol, &mut have_pol) as
                    libc::c_int;
            if ret != 0 {
                current_block = 5166191686646229151;
            } else { current_block = 11743904203796629665; }
        } else { current_block = 11743904203796629665; }
        match current_block {
            5166191686646229151 => { }
            _ => {
                if have_pol != 0 {
                    /* Create a password history entry before we change kdb's key_data. */
                    ret =
                        kdb_get_hist_key(handle, &mut hist_keyblocks,
                                         &mut hist_kvno);
                    if ret != 0 {
                        current_block = 5166191686646229151;
                    } else {
                        ret =
                            create_history_entry((*handle).context,
                                                 &mut *hist_keyblocks.offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                                                 (*kdb).n_key_data as
                                                     libc::c_int,
                                                 (*kdb).key_data, &mut hist);
                        if ret != 0 {
                            current_block = 5166191686646229151;
                        } else { current_block = 16799951812150840583; }
                    }
                } else { current_block = 16799951812150840583; }
                match current_block {
                    5166191686646229151 => { }
                    _ => {
                        ret =
                            passwd_check(handle, password,
                                         if have_pol != 0 {
                                             &mut pol
                                         } else {
                                             0 as *mut kadm5_policy_ent_rec
                                         }, principal) as libc::c_int;
                        if !(ret != 0) {
                            ret =
                                kdb_get_active_mkey(handle, &mut act_kvno,
                                                    &mut act_mkey);
                            if !(ret != 0) {
                                ret =
                                    krb5_dbe_cpw((*handle).context, act_mkey,
                                                 new_ks_tuple, new_n_ks_tuple,
                                                 password, 0 as libc::c_int,
                                                 keepold, kdb);
                                if !(ret != 0) {
                                    ret =
                                        krb5_dbe_update_mkvno((*handle).context,
                                                              kdb, act_kvno);
                                    if !(ret != 0) {
                                        (*kdb).attributes &=
                                            !(0x200 as libc::c_int);
                                        ret =
                                            krb5_timeofday((*handle).context,
                                                           &mut now);
                                        if !(ret != 0) {
                                            if adb.aux_attributes &
                                                   0x800 as libc::c_int as
                                                       libc::c_long != 0 {
                                                /* the policy was loaded before */
                                                ret =
                                                    check_pw_reuse((*handle).context,
                                                                   hist_keyblocks,
                                                                   (*kdb).n_key_data
                                                                       as
                                                                       libc::c_int,
                                                                   (*kdb).key_data,
                                                                   1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint,
                                                                   &mut hist)
                                                        as libc::c_int;
                                                if ret != 0 {
                                                    current_block =
                                                        5166191686646229151;
                                                } else {
                                                    if pol.pw_history_num >
                                                           1 as libc::c_int as
                                                               libc::c_long {
                                                        /* If hist_kvno has changed since the last password change, we
             * can't check the history. */
                                                        if adb.admin_history_kvno
                                                               == hist_kvno {
                                                            ret =
                                                                check_pw_reuse((*handle).context,
                                                                               hist_keyblocks,
                                                                               (*kdb).n_key_data
                                                                                   as
                                                                                   libc::c_int,
                                                                               (*kdb).key_data,
                                                                               adb.old_key_len,
                                                                               adb.old_keys)
                                                                    as
                                                                    libc::c_int;
                                                            if ret != 0 {
                                                                current_block
                                                                    =
                                                                    5166191686646229151;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    12930649117290160518;
                                                            }
                                                        } else {
                                                            current_block =
                                                                12930649117290160518;
                                                        }
                                                        match current_block {
                                                            5166191686646229151
                                                            => {
                                                            }
                                                            _ =>
                                                            /* Don't save empty history. */
                                                            {
                                                                if hist.n_key_data
                                                                       >
                                                                       0 as
                                                                           libc::c_int
                                                                   {
                                                                    ret =
                                                                        add_to_history((*handle).context,
                                                                                       hist_kvno,
                                                                                       &mut adb,
                                                                                       &mut pol,
                                                                                       &mut hist)
                                                                            as
                                                                            libc::c_int;
                                                                    if ret !=
                                                                           0 {
                                                                        current_block
                                                                            =
                                                                            5166191686646229151;
                                                                    } else {
                                                                        hist_added
                                                                            =
                                                                            1
                                                                                as
                                                                                libc::c_int;
                                                                        current_block
                                                                            =
                                                                            16779030619667747692;
                                                                    }
                                                                } else {
                                                                    current_block
                                                                        =
                                                                        16779030619667747692;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        current_block =
                                                            16779030619667747692;
                                                    }
                                                    match current_block {
                                                        5166191686646229151 =>
                                                        {
                                                        }
                                                        _ => {
                                                            if pol.pw_max_life
                                                                   != 0 {
                                                                (*kdb).pw_expiration
                                                                    =
                                                                    ts_incr(now,
                                                                            pol.pw_max_life
                                                                                as
                                                                                krb5_deltat)
                                                            } else {
                                                                (*kdb).pw_expiration
                                                                    =
                                                                    0 as
                                                                        libc::c_int
                                                            }
                                                            current_block =
                                                                7158658067966855297;
                                                        }
                                                    }
                                                }
                                            } else {
                                                (*kdb).pw_expiration =
                                                    0 as libc::c_int;
                                                current_block =
                                                    7158658067966855297;
                                            }
                                            match current_block {
                                                5166191686646229151 => { }
                                                _ => {
                                                    ret =
                                                        krb5_dbe_update_last_pwd_change((*handle).context,
                                                                                        kdb,
                                                                                        now);
                                                    if !(ret != 0) {
                                                        /* unlock principal on this KDC */
                                                        (*kdb).fail_auth_count
                                                            =
                                                            0 as libc::c_int
                                                                as krb5_kvno;
                                                        /* key data and attributes changed, let the database provider know */
                                                        (*kdb).mask =
                                                            (0x20000 as
                                                                 libc::c_int |
                                                                 0x10 as
                                                                     libc::c_int
                                                                 |
                                                                 0x10000 as
                                                                     libc::c_int)
                                                                as krb5_ui_4;
                                                        /* | KADM5_CPW_FUNCTION */
                                                        if hist_added != 0 {
                                                            (*kdb).mask |=
                                                                0x400000 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint
                                                        }
                                                        ret =
                                                            k5_kadm5_hook_chpass((*handle).context,
                                                                                 (*handle).hook_handles,
                                                                                 KADM5_HOOK_STAGE_PRECOMMIT
                                                                                     as
                                                                                     libc::c_int,
                                                                                 principal,
                                                                                 keepold,
                                                                                 new_n_ks_tuple,
                                                                                 new_ks_tuple,
                                                                                 password)
                                                                as
                                                                libc::c_int;
                                                        if !(ret != 0) {
                                                            ret =
                                                                kdb_put_entry(handle,
                                                                              kdb,
                                                                              &mut adb);
                                                            if !(ret != 0) {
                                                                k5_kadm5_hook_chpass((*handle).context,
                                                                                     (*handle).hook_handles,
                                                                                     KADM5_HOOK_STAGE_POSTCOMMIT
                                                                                         as
                                                                                         libc::c_int,
                                                                                     principal,
                                                                                     keepold,
                                                                                     new_n_ks_tuple,
                                                                                     new_ks_tuple,
                                                                                     password);
                                                                ret =
                                                                    0 as
                                                                        libc::c_int
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
                    }
                }
            }
        }
    }
    free(new_ks_tuple as *mut libc::c_void);
    if hist_added == 0 && !hist.key_data.is_null() {
        free_history_entry((*handle).context, &mut hist);
    }
    kdb_free_entry(handle, kdb, &mut adb);
    kdb_free_keyblocks(handle, hist_keyblocks);
    if have_pol != 0 &&
           {
               ret2 =
                   kadm5_free_policy_ent((*handle).lhandle as
                                             *mut libc::c_void, &mut pol) as
                       libc::c_int;
               (ret2) != 0
           } && ret == 0 {
        ret = ret2
    }
    return ret as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "1519:1"]
pub unsafe extern "C" fn kadm5_randkey_principal(mut server_handle:
                                                     *mut libc::c_void,
                                                 mut principal:
                                                     krb5_principal,
                                                 mut keyblocks:
                                                     *mut *mut krb5_keyblock,
                                                 mut n_keys: *mut libc::c_int)
 -> kadm5_ret_t {
    return kadm5_randkey_principal_3(server_handle, principal,
                                     0 as libc::c_int as krb5_boolean,
                                     0 as libc::c_int,
                                     0 as *mut krb5_key_salt_tuple, keyblocks,
                                     n_keys);
}
#[no_mangle]
#[c2rust::src_loc = "1530:1"]
pub unsafe extern "C" fn kadm5_randkey_principal_3(mut server_handle:
                                                       *mut libc::c_void,
                                                   mut principal:
                                                       krb5_principal,
                                                   mut keepold: krb5_boolean,
                                                   mut n_ks_tuple:
                                                       libc::c_int,
                                                   mut ks_tuple:
                                                       *mut krb5_key_salt_tuple,
                                                   mut keyblocks:
                                                       *mut *mut krb5_keyblock,
                                                   mut n_keys:
                                                       *mut libc::c_int)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut adb: osa_princ_ent_rec =
        osa_princ_ent_rec{version: 0,
                          policy: 0 as *mut libc::c_char,
                          aux_attributes: 0,
                          old_key_len: 0,
                          old_key_next: 0,
                          admin_history_kvno: 0,
                          old_keys: 0 as *mut osa_pw_hist_ent,};
    let mut now: krb5_timestamp = 0;
    let mut pol: kadm5_policy_ent_rec =
        kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                             pw_min_life: 0,
                             pw_max_life: 0,
                             pw_min_length: 0,
                             pw_min_classes: 0,
                             pw_history_num: 0,
                             policy_refcnt: 0,
                             pw_max_fail: 0,
                             pw_failcnt_interval: 0,
                             pw_lockout_duration: 0,
                             attributes: 0,
                             max_life: 0,
                             max_renewable_life: 0,
                             allowed_keysalts: 0 as *mut libc::c_char,
                             n_tl_data: 0,
                             tl_data: 0 as *mut krb5_tl_data,};
    let mut ret: libc::c_int = 0;
    let mut n_new_keys: libc::c_int = 0;
    let mut have_pol: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut act_mkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut act_kvno: krb5_kvno = 0;
    let mut new_n_ks_tuple: libc::c_int = 0 as libc::c_int;
    let mut new_ks_tuple: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    if !keyblocks.is_null() { *keyblocks = 0 as *mut krb5_keyblock }
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    if principal.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    ret = kdb_get_entry(handle, principal, &mut kdb, &mut adb);
    if ret != 0 { return ret as kadm5_ret_t }
    ret =
        apply_keysalt_policy(handle, adb.policy, n_ks_tuple, ks_tuple,
                             &mut new_n_ks_tuple, &mut new_ks_tuple) as
            libc::c_int;
    if !(ret != 0) {
        if krb5_principal_compare((*handle).context,
                                  principal as krb5_const_principal,
                                  hist_princ as krb5_const_principal) != 0 {
            /* If changing the history entry, the new entry must have exactly one
         * key. */
            if keepold != 0 {
                ret = 43787550 as libc::c_long as libc::c_int;
                current_block = 1847334758264289692;
            } else {
                new_n_ks_tuple = 1 as libc::c_int;
                current_block = 13131896068329595644;
            }
        } else { current_block = 13131896068329595644; }
        match current_block {
            1847334758264289692 => { }
            _ => {
                ret =
                    kdb_get_active_mkey(handle, &mut act_kvno, &mut act_mkey);
                if !(ret != 0) {
                    ret =
                        krb5_dbe_crk((*handle).context, act_mkey,
                                     new_ks_tuple, new_n_ks_tuple, keepold,
                                     kdb);
                    if !(ret != 0) {
                        ret =
                            krb5_dbe_update_mkvno((*handle).context, kdb,
                                                  act_kvno);
                        if !(ret != 0) {
                            (*kdb).attributes &= !(0x200 as libc::c_int);
                            ret = krb5_timeofday((*handle).context, &mut now);
                            if !(ret != 0) {
                                if adb.aux_attributes &
                                       0x800 as libc::c_int as libc::c_long !=
                                       0 {
                                    ret =
                                        get_policy(handle, adb.policy,
                                                   &mut pol, &mut have_pol) as
                                            libc::c_int;
                                    if ret != 0 {
                                        current_block = 1847334758264289692;
                                    } else {
                                        current_block = 10095721787123848864;
                                    }
                                } else {
                                    current_block = 10095721787123848864;
                                }
                                match current_block {
                                    1847334758264289692 => { }
                                    _ => {
                                        if have_pol != 0 {
                                            if pol.pw_max_life != 0 {
                                                (*kdb).pw_expiration =
                                                    ts_incr(now,
                                                            pol.pw_max_life as
                                                                krb5_deltat)
                                            } else {
                                                (*kdb).pw_expiration =
                                                    0 as libc::c_int
                                            }
                                        } else {
                                            (*kdb).pw_expiration =
                                                0 as libc::c_int
                                        }
                                        ret =
                                            krb5_dbe_update_last_pwd_change((*handle).context,
                                                                            kdb,
                                                                            now);
                                        if !(ret != 0) {
                                            /* unlock principal on this KDC */
                                            (*kdb).fail_auth_count =
                                                0 as libc::c_int as krb5_kvno;
                                            if !keyblocks.is_null() {
                                                /* Return only the new keys added by krb5_dbe_crk. */
                                                n_new_keys =
                                                    count_new_keys((*kdb).n_key_data
                                                                       as
                                                                       libc::c_int,
                                                                   (*kdb).key_data);
                                                ret =
                                                    decrypt_key_data((*handle).context,
                                                                     n_new_keys,
                                                                     (*kdb).key_data,
                                                                     keyblocks,
                                                                     n_keys);
                                                if ret != 0 {
                                                    current_block =
                                                        1847334758264289692;
                                                } else {
                                                    current_block =
                                                        307447392441238883;
                                                }
                                            } else {
                                                current_block =
                                                    307447392441238883;
                                            }
                                            match current_block {
                                                1847334758264289692 => { }
                                                _ => {
                                                    /* key data changed, let the database provider know */
                                                    (*kdb).mask =
                                                        (0x20000 as
                                                             libc::c_int |
                                                             0x10000 as
                                                                 libc::c_int)
                                                            as krb5_ui_4;
                                                    /* | KADM5_RANDKEY_USED */
                                                    ret =
                                                        k5_kadm5_hook_chpass((*handle).context,
                                                                             (*handle).hook_handles,
                                                                             KADM5_HOOK_STAGE_PRECOMMIT
                                                                                 as
                                                                                 libc::c_int,
                                                                             principal,
                                                                             keepold,
                                                                             new_n_ks_tuple,
                                                                             new_ks_tuple,
                                                                             0
                                                                                 as
                                                                                 *const libc::c_char)
                                                            as libc::c_int;
                                                    if !(ret != 0) {
                                                        ret =
                                                            kdb_put_entry(handle,
                                                                          kdb,
                                                                          &mut adb);
                                                        if !(ret != 0) {
                                                            k5_kadm5_hook_chpass((*handle).context,
                                                                                 (*handle).hook_handles,
                                                                                 KADM5_HOOK_STAGE_POSTCOMMIT
                                                                                     as
                                                                                     libc::c_int,
                                                                                 principal,
                                                                                 keepold,
                                                                                 new_n_ks_tuple,
                                                                                 new_ks_tuple,
                                                                                 0
                                                                                     as
                                                                                     *const libc::c_char);
                                                            ret =
                                                                0 as
                                                                    libc::c_int
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
                }
            }
        }
    }
    free(new_ks_tuple as *mut libc::c_void);
    kdb_free_entry(handle, kdb, &mut adb);
    if have_pol != 0 {
        kadm5_free_policy_ent((*handle).lhandle as *mut libc::c_void,
                              &mut pol);
    }
    return ret as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "1652:1"]
pub unsafe extern "C" fn kadm5_setkey_principal(mut server_handle:
                                                    *mut libc::c_void,
                                                mut principal: krb5_principal,
                                                mut keyblocks:
                                                    *mut krb5_keyblock,
                                                mut n_keys: libc::c_int)
 -> kadm5_ret_t {
    return kadm5_setkey_principal_3(server_handle, principal,
                                    0 as libc::c_int as krb5_boolean,
                                    0 as libc::c_int,
                                    0 as *mut krb5_key_salt_tuple, keyblocks,
                                    n_keys);
}
#[no_mangle]
#[c2rust::src_loc = "1664:1"]
pub unsafe extern "C" fn kadm5_setkey_principal_3(mut server_handle:
                                                      *mut libc::c_void,
                                                  mut principal:
                                                      krb5_principal,
                                                  mut keepold: krb5_boolean,
                                                  mut n_ks_tuple: libc::c_int,
                                                  mut ks_tuple:
                                                      *mut krb5_key_salt_tuple,
                                                  mut keyblocks:
                                                      *mut krb5_keyblock,
                                                  mut n_keys: libc::c_int)
 -> kadm5_ret_t {
    let mut key_data: *mut kadm5_key_data = 0 as *mut kadm5_key_data;
    let mut ret: kadm5_ret_t = 0;
    let mut i: libc::c_int = 0;
    if keyblocks.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    if n_ks_tuple != 0 {
        if n_ks_tuple != n_keys { return 43787573 as libc::c_long }
        i = 0 as libc::c_int;
        while i < n_ks_tuple {
            if (*ks_tuple.offset(i as isize)).ks_enctype !=
                   (*keyblocks.offset(i as isize)).enctype {
                return 43787573 as libc::c_long
            }
            i += 1
        }
    }
    key_data =
        calloc(n_keys as libc::c_ulong,
               ::std::mem::size_of::<kadm5_key_data>() as libc::c_ulong) as
            *mut kadm5_key_data;
    if key_data.is_null() { return 12 as libc::c_int as kadm5_ret_t }
    i = 0 as libc::c_int;
    while i < n_keys {
        (*key_data.offset(i as isize)).key = *keyblocks.offset(i as isize);
        (*key_data.offset(i as isize)).salt.type_0 =
            if n_ks_tuple != 0 {
                (*ks_tuple.offset(i as isize)).ks_salttype
            } else { 0 as libc::c_int } as krb5_int16;
        i += 1
    }
    ret =
        kadm5_setkey_principal_4(server_handle, principal, keepold, key_data,
                                 n_keys);
    free(key_data as *mut libc::c_void);
    return ret;
}
/* Create a key/salt list from a key_data array. */
#[c2rust::src_loc = "1705:1"]
unsafe extern "C" fn make_ks_from_key_data(mut context: krb5_context,
                                           mut key_data: *mut kadm5_key_data,
                                           mut n_key_data: libc::c_int,
                                           mut out:
                                               *mut *mut krb5_key_salt_tuple)
 -> kadm5_ret_t {
    let mut i: libc::c_int = 0;
    let mut ks: *mut krb5_key_salt_tuple = 0 as *mut krb5_key_salt_tuple;
    *out = 0 as *mut krb5_key_salt_tuple;
    ks =
        calloc(n_key_data as libc::c_ulong,
               ::std::mem::size_of::<krb5_key_salt_tuple>() as libc::c_ulong)
            as *mut krb5_key_salt_tuple;
    if ks.is_null() { return 12 as libc::c_int as kadm5_ret_t }
    i = 0 as libc::c_int;
    while i < n_key_data {
        (*ks.offset(i as isize)).ks_enctype =
            (*key_data.offset(i as isize)).key.enctype;
        (*ks.offset(i as isize)).ks_salttype =
            (*key_data.offset(i as isize)).salt.type_0 as krb5_int32;
        i += 1
    }
    *out = ks;
    return 0 as libc::c_int as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "1726:1"]
pub unsafe extern "C" fn kadm5_setkey_principal_4(mut server_handle:
                                                      *mut libc::c_void,
                                                  mut principal:
                                                      krb5_principal,
                                                  mut keepold: krb5_boolean,
                                                  mut key_data:
                                                      *mut kadm5_key_data,
                                                  mut n_key_data: libc::c_int)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut adb: osa_princ_ent_rec =
        osa_princ_ent_rec{version: 0,
                          policy: 0 as *mut libc::c_char,
                          aux_attributes: 0,
                          old_key_len: 0,
                          old_key_next: 0,
                          admin_history_kvno: 0,
                          old_keys: 0 as *mut osa_pw_hist_ent,};
    let mut now: krb5_timestamp = 0;
    let mut pol: kadm5_policy_ent_rec =
        kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                             pw_min_life: 0,
                             pw_max_life: 0,
                             pw_min_length: 0,
                             pw_min_classes: 0,
                             pw_history_num: 0,
                             policy_refcnt: 0,
                             pw_max_fail: 0,
                             pw_failcnt_interval: 0,
                             pw_lockout_duration: 0,
                             attributes: 0,
                             max_life: 0,
                             max_renewable_life: 0,
                             allowed_keysalts: 0 as *mut libc::c_char,
                             n_tl_data: 0,
                             tl_data: 0 as *mut krb5_tl_data,};
    let mut new_key_data: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut n_new_key_data: libc::c_int = 0 as libc::c_int;
    let mut kvno: krb5_kvno = 0;
    let mut similar: krb5_boolean = 0;
    let mut have_pol: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut act_mkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut ks_from_keys: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    if principal.is_null() || key_data.is_null() ||
           n_key_data == 0 as libc::c_int {
        return 22 as libc::c_int as kadm5_ret_t
    }
    /* hist_princ will be NULL when initializing the database. */
    if !hist_princ.is_null() &&
           krb5_principal_compare((*handle).context,
                                  principal as krb5_const_principal,
                                  hist_princ as krb5_const_principal) != 0 {
        return 43787550 as libc::c_long
    }
    /* For now, all keys must have the same kvno. */
    kvno = (*key_data.offset(0 as libc::c_int as isize)).kvno;
    i = 1 as libc::c_int;
    while i < n_key_data {
        if (*key_data.offset(i as isize)).kvno != kvno {
            return 43787579 as libc::c_long
        }
        i += 1
    }
    ret = kdb_get_entry(handle, principal, &mut kdb, &mut adb);
    if ret != 0 { return ret as kadm5_ret_t }
    if kvno == 0 as libc::c_int as libc::c_uint {
        /* Pick the next kvno. */
        i = 0 as libc::c_int;
        while i < (*kdb).n_key_data as libc::c_int {
            if (*(*kdb).key_data.offset(i as isize)).key_data_kvno as
                   libc::c_uint > kvno {
                kvno =
                    (*(*kdb).key_data.offset(i as isize)).key_data_kvno as
                        krb5_kvno
            }
            i += 1
        }
        kvno = kvno.wrapping_add(1);
        current_block = 13826291924415791078;
    } else if keepold != 0 {
        /* Check that the kvno does collide with existing keys. */
        i = 0 as libc::c_int;
        loop  {
            if !(i < (*kdb).n_key_data as libc::c_int) {
                current_block = 13826291924415791078;
                break ;
            }
            if (*(*kdb).key_data.offset(i as isize)).key_data_kvno as
                   libc::c_uint == kvno {
                ret = 43787579 as libc::c_long as libc::c_int;
                current_block = 16456185935704989789;
                break ;
            } else { i += 1 }
        }
    } else { current_block = 13826291924415791078; }
    match current_block {
        13826291924415791078 => {
            ret =
                make_ks_from_key_data((*handle).context, key_data, n_key_data,
                                      &mut ks_from_keys) as libc::c_int;
            if !(ret != 0) {
                ret =
                    apply_keysalt_policy(handle, adb.policy, n_key_data,
                                         ks_from_keys, 0 as *mut libc::c_int,
                                         0 as *mut *mut krb5_key_salt_tuple)
                        as libc::c_int;
                free(ks_from_keys as *mut libc::c_void);
                if !(ret != 0) {
                    i = 0 as libc::c_int;
                    's_293:
                        loop  {
                            if !(i < n_key_data) {
                                current_block = 2606304779496145856;
                                break ;
                            }
                            j = i + 1 as libc::c_int;
                            while j < n_key_data {
                                ret =
                                    krb5_c_enctype_compare((*handle).context,
                                                           (*key_data.offset(i
                                                                                 as
                                                                                 isize)).key.enctype,
                                                           (*key_data.offset(j
                                                                                 as
                                                                                 isize)).key.enctype,
                                                           &mut similar);
                                if ret != 0 {
                                    current_block = 16456185935704989789;
                                    break 's_293 ;
                                }
                                if similar != 0 {
                                    if (*key_data.offset(i as
                                                             isize)).salt.type_0
                                           as libc::c_int ==
                                           (*key_data.offset(j as
                                                                 isize)).salt.type_0
                                               as libc::c_int {
                                        ret =
                                            43787571 as libc::c_long as
                                                libc::c_int;
                                        current_block = 16456185935704989789;
                                        break 's_293 ;
                                    }
                                }
                                j += 1
                            }
                            i += 1
                        }
                    match current_block {
                        16456185935704989789 => { }
                        _ => {
                            n_new_key_data =
                                n_key_data +
                                    (if keepold != 0 {
                                         (*kdb).n_key_data as libc::c_int
                                     } else { 0 as libc::c_int });
                            new_key_data =
                                calloc(n_new_key_data as libc::c_ulong,
                                       ::std::mem::size_of::<krb5_key_data>()
                                           as libc::c_ulong) as
                                    *mut krb5_key_data;
                            if new_key_data.is_null() {
                                n_new_key_data = 0 as libc::c_int;
                                ret = 12 as libc::c_int
                            } else {
                                n_new_key_data = 0 as libc::c_int;
                                i = 0 as libc::c_int;
                                loop  {
                                    if !(i < n_key_data) {
                                        current_block = 3879520548144599102;
                                        break ;
                                    }
                                    ret =
                                        kdb_get_active_mkey(handle,
                                                            0 as
                                                                *mut krb5_kvno,
                                                            &mut act_mkey);
                                    if ret != 0 {
                                        current_block = 16456185935704989789;
                                        break ;
                                    }
                                    ret =
                                        krb5_dbe_encrypt_key_data((*handle).context,
                                                                  act_mkey,
                                                                  &mut (*key_data.offset(i
                                                                                             as
                                                                                             isize)).key,
                                                                  &mut (*key_data.offset(i
                                                                                             as
                                                                                             isize)).salt,
                                                                  kvno as
                                                                      libc::c_int,
                                                                  &mut *new_key_data.offset(i
                                                                                                as
                                                                                                isize));
                                    if ret != 0 {
                                        current_block = 16456185935704989789;
                                        break ;
                                    }
                                    n_new_key_data += 1;
                                    i += 1
                                }
                                match current_block {
                                    16456185935704989789 => { }
                                    _ => {
                                        /* Copy old key data if necessary. */
                                        if keepold != 0 {
                                            memcpy(new_key_data.offset(n_new_key_data
                                                                           as
                                                                           isize)
                                                       as *mut libc::c_void,
                                                   (*kdb).key_data as
                                                       *const libc::c_void,
                                                   ((*kdb).n_key_data as
                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_key_data>()
                                                                                        as
                                                                                        libc::c_ulong));
                                            memset((*kdb).key_data as
                                                       *mut libc::c_void,
                                                   0 as libc::c_int,
                                                   ((*kdb).n_key_data as
                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_key_data>()
                                                                                        as
                                                                                        libc::c_ulong));
                                            /*
         * Sort the keys to maintain the defined kvno order.  We only need to
         * sort if we keep old keys, as otherwise we allow only a single kvno
         * to be specified.
         */
                                            krb5_dbe_sort_key_data(new_key_data,
                                                                   n_new_key_data
                                                                       as
                                                                       size_t);
                                        }
                                        /* Replace kdb->key_data with the new keys. */
                                        cleanup_key_data((*handle).context,
                                                         (*kdb).n_key_data as
                                                             libc::c_int,
                                                         (*kdb).key_data);
                                        (*kdb).key_data = new_key_data;
                                        (*kdb).n_key_data =
                                            n_new_key_data as krb5_int16;
                                        new_key_data =
                                            0 as *mut krb5_key_data;
                                        n_new_key_data = 0 as libc::c_int;
                                        (*kdb).attributes &=
                                            !(0x200 as libc::c_int);
                                        ret =
                                            krb5_timeofday((*handle).context,
                                                           &mut now);
                                        if !(ret != 0) {
                                            if adb.aux_attributes &
                                                   0x800 as libc::c_int as
                                                       libc::c_long != 0 {
                                                ret =
                                                    get_policy(handle,
                                                               adb.policy,
                                                               &mut pol,
                                                               &mut have_pol)
                                                        as libc::c_int;
                                                if ret != 0 {
                                                    current_block =
                                                        16456185935704989789;
                                                } else {
                                                    current_block =
                                                        6712462580143783635;
                                                }
                                            } else {
                                                current_block =
                                                    6712462580143783635;
                                            }
                                            match current_block {
                                                16456185935704989789 => { }
                                                _ => {
                                                    if have_pol != 0 {
                                                        if pol.pw_max_life !=
                                                               0 {
                                                            (*kdb).pw_expiration
                                                                =
                                                                ts_incr(now,
                                                                        pol.pw_max_life
                                                                            as
                                                                            krb5_deltat)
                                                        } else {
                                                            (*kdb).pw_expiration
                                                                =
                                                                0 as
                                                                    libc::c_int
                                                        }
                                                    } else {
                                                        (*kdb).pw_expiration =
                                                            0 as libc::c_int
                                                    }
                                                    ret =
                                                        krb5_dbe_update_last_pwd_change((*handle).context,
                                                                                        kdb,
                                                                                        now);
                                                    if !(ret != 0) {
                                                        /* Unlock principal on this KDC. */
                                                        (*kdb).fail_auth_count
                                                            =
                                                            0 as libc::c_int
                                                                as krb5_kvno;
                                                        /* key data changed, let the database provider know */
                                                        (*kdb).mask =
                                                            (0x20000 as
                                                                 libc::c_int |
                                                                 0x10000 as
                                                                     libc::c_int)
                                                                as krb5_ui_4;
                                                        ret =
                                                            kdb_put_entry(handle,
                                                                          kdb,
                                                                          &mut adb);
                                                        if !(ret != 0) {
                                                            ret =
                                                                0 as
                                                                    libc::c_int
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
                }
            }
        }
        _ => { }
    }
    cleanup_key_data((*handle).context, n_new_key_data, new_key_data);
    kdb_free_entry(handle, kdb, &mut adb);
    if have_pol != 0 {
        kadm5_free_policy_ent((*handle).lhandle as *mut libc::c_void,
                              &mut pol);
    }
    return ret as kadm5_ret_t;
}
/*
 * Return the list of keys like kadm5_randkey_principal,
 * but don't modify the principal.
 */
#[no_mangle]
#[c2rust::src_loc = "1904:1"]
pub unsafe extern "C" fn kadm5_get_principal_keys(mut server_handle:
                                                      *mut libc::c_void,
                                                  mut principal:
                                                      krb5_principal,
                                                  mut kvno: krb5_kvno,
                                                  mut key_data_out:
                                                      *mut *mut kadm5_key_data,
                                                  mut n_key_data_out:
                                                      *mut libc::c_int)
 -> kadm5_ret_t 
 /* OUT */
 {
    let mut current_block: u64;
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut adb: osa_princ_ent_rec =
        osa_princ_ent_rec{version: 0,
                          policy: 0 as *mut libc::c_char,
                          aux_attributes: 0,
                          old_key_len: 0,
                          old_key_next: 0,
                          admin_history_kvno: 0,
                          old_keys: 0 as *mut osa_pw_hist_ent,};
    let mut ret: kadm5_ret_t = 0;
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut key_data: *mut kadm5_key_data = 0 as *mut kadm5_key_data;
    let mut i: libc::c_int = 0;
    let mut nkeys: libc::c_int = 0 as libc::c_int;
    if principal.is_null() || key_data_out.is_null() ||
           n_key_data_out.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    ret = kdb_get_entry(handle, principal, &mut kdb, &mut adb) as kadm5_ret_t;
    if ret != 0 { return ret }
    key_data =
        calloc((*kdb).n_key_data as libc::c_ulong,
               ::std::mem::size_of::<kadm5_key_data>() as libc::c_ulong) as
            *mut kadm5_key_data;
    if key_data.is_null() {
        ret = 12 as libc::c_int as kadm5_ret_t
    } else {
        i = 0 as libc::c_int;
        nkeys = 0 as libc::c_int;
        loop  {
            if !(i < (*kdb).n_key_data as libc::c_int) {
                current_block = 15597372965620363352;
                break ;
            }
            if !(kvno != 0 as libc::c_int as libc::c_uint &&
                     kvno !=
                         (*(*kdb).key_data.offset(i as isize)).key_data_kvno
                             as libc::c_uint) {
                (*key_data.offset(nkeys as isize)).kvno =
                    (*(*kdb).key_data.offset(i as isize)).key_data_kvno as
                        krb5_kvno;
                ret =
                    krb5_dbe_decrypt_key_data((*handle).context,
                                              0 as *const krb5_keyblock,
                                              &mut *(*kdb).key_data.offset(i
                                                                               as
                                                                               isize),
                                              &mut (*key_data.offset(nkeys as
                                                                         isize)).key,
                                              &mut (*key_data.offset(nkeys as
                                                                         isize)).salt)
                        as kadm5_ret_t;
                if ret != 0 { current_block = 13820746548497889489; break ; }
                nkeys += 1
            }
            i += 1
        }
        match current_block {
            13820746548497889489 => { }
            _ => {
                *n_key_data_out = nkeys;
                *key_data_out = key_data;
                key_data = 0 as *mut kadm5_key_data;
                nkeys = 0 as libc::c_int;
                ret = 0 as libc::c_int as kadm5_ret_t
            }
        }
    }
    kdb_free_entry(handle, kdb, &mut adb);
    kadm5_free_kadm5_key_data((*handle).context, nkeys, key_data);
    return ret;
}
/*
 * Allocate an array of n_key_data krb5_keyblocks, fill in each
 * element with the results of decrypting the nth key in key_data,
 * and if n_keys is not NULL fill it in with the
 * number of keys decrypted.
 */
#[c2rust::src_loc = "1966:1"]
unsafe extern "C" fn decrypt_key_data(mut context: krb5_context,
                                      mut n_key_data: libc::c_int,
                                      mut key_data: *mut krb5_key_data,
                                      mut keyblocks: *mut *mut krb5_keyblock,
                                      mut n_keys: *mut libc::c_int)
 -> libc::c_int {
    let mut keys: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    keys =
        malloc((n_key_data as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_keyblock>()
                                                    as libc::c_ulong)) as
            *mut krb5_keyblock;
    if keys.is_null() { return 12 as libc::c_int }
    memset(keys as *mut libc::c_void, 0 as libc::c_int,
           (n_key_data as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_keyblock>()
                                                as libc::c_ulong));
    i = 0 as libc::c_int;
    while i < n_key_data {
        ret =
            krb5_dbe_decrypt_key_data(context, 0 as *const krb5_keyblock,
                                      &mut *key_data.offset(i as isize),
                                      &mut *keys.offset(i as isize),
                                      0 as *mut krb5_keysalt);
        if ret != 0 {
            while i >= 0 as libc::c_int {
                krb5_free_keyblock_contents(context,
                                            &mut *keys.offset(i as isize));
                i -= 1
            }
            free(keys as *mut libc::c_void);
            return ret
        }
        i += 1
    }
    *keyblocks = keys;
    if !n_keys.is_null() { *n_keys = n_key_data }
    return 0 as libc::c_int;
}
/*
 * Function: kadm5_decrypt_key
 *
 * Purpose: Retrieves and decrypts a principal key.
 *
 * Arguments:
 *
 *      server_handle   (r) kadm5 handle
 *      entry           (r) principal retrieved with kadm5_get_principal
 *      ktype           (r) enctype to search for, or -1 to ignore
 *      stype           (r) salt type to search for, or -1 to ignore
 *      kvno            (r) kvno to search for, -1 for max, 0 for max
 *                      only if it also matches ktype and stype
 *      keyblock        (w) keyblock to fill in
 *      keysalt         (w) keysalt to fill in, or NULL
 *      kvnop           (w) kvno to fill in, or NULL
 *
 * Effects: Searches the key_data array of entry, which must have been
 * retrived with kadm5_get_principal with the KADM5_KEY_DATA mask, to
 * find a key with a specified enctype, salt type, and kvno in a
 * principal entry.  If not found, return ENOENT.  Otherwise, decrypt
 * it with the master key, and return the key in keyblock, the salt
 * in salttype, and the key version number in kvno.
 *
 * If ktype or stype is -1, it is ignored for the search.  If kvno is
 * -1, ktype and stype are ignored and the key with the max kvno is
 * returned.  If kvno is 0, only the key with the max kvno is returned
 * and only if it matches the ktype and stype; otherwise, ENOENT is
 * returned.
 */
#[no_mangle]
#[c2rust::src_loc = "2026:1"]
pub unsafe extern "C" fn kadm5_decrypt_key(mut server_handle:
                                               *mut libc::c_void,
                                           mut entry: kadm5_principal_ent_t,
                                           mut ktype: krb5_int32,
                                           mut stype: krb5_int32,
                                           mut kvno: krb5_int32,
                                           mut keyblock: *mut krb5_keyblock,
                                           mut keysalt: *mut krb5_keysalt,
                                           mut kvnop: *mut libc::c_int)
 -> kadm5_ret_t {
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut dbent: krb5_db_entry =
        krb5_db_entry{magic: 0,
                      len: 0,
                      mask: 0,
                      attributes: 0,
                      max_life: 0,
                      max_renewable_life: 0,
                      expiration: 0,
                      pw_expiration: 0,
                      last_success: 0,
                      last_failed: 0,
                      fail_auth_count: 0,
                      n_tl_data: 0,
                      n_key_data: 0,
                      e_length: 0,
                      e_data: 0 as *mut krb5_octet,
                      princ: 0 as *mut krb5_principal_data,
                      tl_data: 0 as *mut krb5_tl_data,
                      key_data: 0 as *mut krb5_key_data,};
    let mut key_data: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut mkey_ptr: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut ret: libc::c_int = 0;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    if (*entry).n_key_data as libc::c_int == 0 as libc::c_int ||
           (*entry).key_data.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    /* find_enctype only uses these two fields */
    dbent.n_key_data = (*entry).n_key_data;
    dbent.key_data = (*entry).key_data;
    ret =
        krb5_dbe_find_enctype((*handle).context, &mut dbent, ktype, stype,
                              kvno, &mut key_data);
    if ret != 0 { return ret as kadm5_ret_t }
    /* find_mkey only uses this field */
    dbent.tl_data = (*entry).tl_data;
    ret = krb5_dbe_find_mkey((*handle).context, &mut dbent, &mut mkey_ptr);
    if ret != 0 {
        /* try refreshing master key list */
        /* XXX it would nice if we had the mkvno here for optimization */
        if krb5_db_fetch_mkey_list((*handle).context, master_princ,
                                   &mut master_keyblock) == 0 as libc::c_int {
            ret =
                krb5_dbe_find_mkey((*handle).context, &mut dbent,
                                   &mut mkey_ptr);
            if ret != 0 { return ret as kadm5_ret_t }
        } else { return ret as kadm5_ret_t }
    }
    ret =
        krb5_dbe_decrypt_key_data((*handle).context,
                                  0 as *const krb5_keyblock, key_data,
                                  keyblock, keysalt);
    if ret != 0 { return ret as kadm5_ret_t }
    /*
     * Coerce the enctype of the output keyblock in case we got an
     * inexact match on the enctype; this behavior will go away when
     * the key storage architecture gets redesigned for 1.3.
     */
    if ktype != -(1 as libc::c_int) { (*keyblock).enctype = ktype }
    if !kvnop.is_null() { *kvnop = (*key_data).key_data_kvno as libc::c_int }
    return 0 as libc::c_int as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "2084:1"]
pub unsafe extern "C" fn kadm5_purgekeys(mut server_handle: *mut libc::c_void,
                                         mut principal: krb5_principal,
                                         mut keepkvno: libc::c_int)
 -> kadm5_ret_t {
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut ret: kadm5_ret_t = 0;
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut adb: osa_princ_ent_rec =
        osa_princ_ent_rec{version: 0,
                          policy: 0 as *mut libc::c_char,
                          aux_attributes: 0,
                          old_key_len: 0,
                          old_key_next: 0,
                          admin_history_kvno: 0,
                          old_keys: 0 as *mut osa_pw_hist_ent,};
    let mut old_keydata: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut n_old_keydata: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    if principal.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    ret = kdb_get_entry(handle, principal, &mut kdb, &mut adb) as kadm5_ret_t;
    if ret != 0 { return ret }
    if keepkvno <= 0 as libc::c_int {
        keepkvno =
            krb5_db_get_key_data_kvno((*handle).context,
                                      (*kdb).n_key_data as libc::c_int,
                                      (*kdb).key_data)
    }
    old_keydata = (*kdb).key_data;
    n_old_keydata = (*kdb).n_key_data as libc::c_int;
    (*kdb).n_key_data = 0 as libc::c_int as krb5_int16;
    /* Allocate one extra key_data to avoid allocating 0 bytes. */
    (*kdb).key_data =
        calloc(n_old_keydata as libc::c_ulong,
               ::std::mem::size_of::<krb5_key_data>() as libc::c_ulong) as
            *mut krb5_key_data;
    if (*kdb).key_data.is_null() {
        ret = 12 as libc::c_int as kadm5_ret_t
    } else {
        memset((*kdb).key_data as *mut libc::c_void, 0 as libc::c_int,
               (n_old_keydata as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_key_data>()
                                                    as libc::c_ulong));
        i = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while i < n_old_keydata {
            if !(((*old_keydata.offset(i as isize)).key_data_kvno as
                      libc::c_int) < keepkvno) {
                /* Alias the key_data_contents pointers; we null them out in the
         * source array immediately after. */
                *(*kdb).key_data.offset(j as isize) =
                    *old_keydata.offset(i as isize);
                k = 0 as libc::c_int;
                while k <
                          (*old_keydata.offset(i as isize)).key_data_ver as
                              libc::c_int {
                    let ref mut fresh1 =
                        (*old_keydata.offset(i as
                                                 isize)).key_data_contents[k
                                                                               as
                                                                               usize];
                    *fresh1 = 0 as *mut krb5_octet;
                    k += 1
                }
                j += 1
            }
            i += 1
        }
        (*kdb).n_key_data = j as krb5_int16;
        cleanup_key_data((*handle).context, n_old_keydata, old_keydata);
        (*kdb).mask = 0x20000 as libc::c_int as krb5_ui_4;
        ret = kdb_put_entry(handle, kdb, &mut adb) as kadm5_ret_t;
        (ret) != 0;
    }
    kdb_free_entry(handle, kdb, &mut adb);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2146:1"]
pub unsafe extern "C" fn kadm5_get_strings(mut server_handle:
                                               *mut libc::c_void,
                                           mut principal: krb5_principal,
                                           mut strings_out:
                                               *mut *mut krb5_string_attr,
                                           mut count_out: *mut libc::c_int)
 -> kadm5_ret_t {
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut ret: kadm5_ret_t = 0;
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    *strings_out = 0 as *mut krb5_string_attr;
    *count_out = 0 as libc::c_int;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    if principal.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    ret =
        kdb_get_entry(handle, principal, &mut kdb,
                      0 as *mut osa_princ_ent_rec) as kadm5_ret_t;
    if ret != 0 { return ret }
    ret =
        krb5_dbe_get_strings((*handle).context, kdb, strings_out, count_out)
            as kadm5_ret_t;
    kdb_free_entry(handle, kdb, 0 as *mut osa_princ_ent_rec);
    return ret;
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
/*
 * For all initialization functions, the caller must first initialize
 * a context with kadm5_init_krb5_context which will survive as long
 * as the resulting handle.  The caller should free the context with
 * krb5_free_context.
 */
#[no_mangle]
#[c2rust::src_loc = "2169:1"]
pub unsafe extern "C" fn kadm5_set_string(mut server_handle:
                                              *mut libc::c_void,
                                          mut principal: krb5_principal,
                                          mut key: *const libc::c_char,
                                          mut value: *const libc::c_char)
 -> kadm5_ret_t {
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut ret: kadm5_ret_t = 0;
    let mut kdb: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut adb: osa_princ_ent_rec =
        osa_princ_ent_rec{version: 0,
                          policy: 0 as *mut libc::c_char,
                          aux_attributes: 0,
                          old_key_len: 0,
                          old_key_next: 0,
                          admin_history_kvno: 0,
                          old_keys: 0 as *mut osa_pw_hist_ent,};
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    if principal.is_null() || key.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    ret = kdb_get_entry(handle, principal, &mut kdb, &mut adb) as kadm5_ret_t;
    if ret != 0 { return ret }
    ret =
        krb5_dbe_set_string((*handle).context, kdb, key, value) as
            kadm5_ret_t;
    if !(ret != 0) {
        (*kdb).mask = 0x40000 as libc::c_int as krb5_ui_4;
        ret = kdb_put_entry(handle, kdb, &mut adb) as kadm5_ret_t
    }
    kdb_free_entry(handle, kdb, &mut adb);
    return ret;
}
