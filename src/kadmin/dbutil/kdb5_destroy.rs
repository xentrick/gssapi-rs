use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
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
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
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
    /* *
 * Add a prefix to a different error code's message.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the message for @a old_code.  The prefix
 * will be separated from the old message with a colon and space.  Set the
 * resulting message as the extended error message for @a code.
 */
    /* *
 * Add a prefix to a different error code's message using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_wrap_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
    /* *
 * Copy the most recent extended error message from one context to another.
 *
 * @param [in] dest_ctx         Library context to copy message to
 * @param [in] src_ctx          Library context with current message
 */
    /* *
 * Get the (possibly extended) error message for a code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 *
 * The behavior of krb5_get_error_message() is only defined the first time it
 * is called after a failed call to a krb5 function using the same context, and
 * only when the error code passed in is the same as that returned by the krb5
 * function.
 *
 * This function never returns NULL, so its result may be used unconditionally
 * as a C string.
 *
 * The string returned by this function must be freed using
 * krb5_free_error_message()
 *
 * @note Future versions may return the same string for the second
 * and following calls.
 */
    /* *
 * Free an error message generated by krb5_get_error_message().
 *
 * @param [in] ctx              Library context
 * @param [in] msg              Pointer to error message
 */
    /* *
 * Clear the extended error message in a context.
 *
 * @param [in] ctx              Library context
 *
 * This function unsets the extended error message in a context, to ensure that
 * it is not mistakenly applied to another occurrence of the same error code.
 */
    /* *
 * Unwrap authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  container       Authorization data to be decoded
 * @param [out] authdata        List of decoded authorization data
 *
 * @sa krb5_encode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Wrap authorization data in a container.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  authdata        List of authorization data to be encoded
 * @param [out] container       List of encoded authorization data
 *
 * The result is returned in @a container as a single-element list.
 *
 * @sa krb5_decode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /*
 * AD-KDCIssued
 */
/* *
 * Encode and sign AD-KDCIssued authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  key             Session key
 * @param [in]  issuer          The name of the issuing principal
 * @param [in]  authdata        List of authorization data to be signed
 * @param [out] ad_kdcissued    List containing AD-KDCIssued authdata
 *
 * This function wraps a list of authorization data entries @a authdata in an
 * AD-KDCIssued container (see RFC 4120 section 5.2.6.2) signed with @a key.
 * The result is returned in @a ad_kdcissued as a single-element list.
 */
    /* *
 * Unwrap and verify AD-KDCIssued authorization data.
 *
 * @param [in] context          Library context
 * @param [in] key              Session key
 * @param [in] ad_kdcissued     AD-KDCIssued authorization data to be unwrapped
 * @param [out] issuer          Name of issuing principal (or NULL)
 * @param [out] authdata        Unwrapped list of authorization data
 *
 * This function unwraps an AD-KDCIssued authdatum (see RFC 4120 section
 * 5.2.6.2) and verifies its signature against @a key.  The issuer field of the
 * authdatum element is returned in @a issuer, and the unwrapped list of
 * authdata is returned in @a authdata.
 */
    /*
 * Windows PAC
 */
    /* Microsoft defined types of data */
    /* *< Logon information */
    /* *< Credentials information */
    /* *< Server checksum */
    /* *< KDC checksum */
    /* *< Client name and ticket info */
    /* *< Constrained delegation info */
    /* *< User principal name and DNS info */
    /* * PAC data structure to convey authorization information */
    /* *
 * Add a buffer to a PAC handle.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] type             Buffer type
 * @param [in] data             contents
 *
 * This function adds a buffer of type @a type and contents @a data to @a pac
 * if there isn't already a buffer of this type present.
 *
 * The valid values of @a type is one of the following:
 * @li #KRB5_PAC_LOGON_INFO         -  Logon information
 * @li #KRB5_PAC_CREDENTIALS_INFO   -  Credentials information
 * @li #KRB5_PAC_SERVER_CHECKSUM    -  Server checksum
 * @li #KRB5_PAC_PRIVSVR_CHECKSUM   -  KDC checksum
 * @li #KRB5_PAC_CLIENT_INFO        -  Client name and ticket information
 * @li #KRB5_PAC_DELEGATION_INFO    -  Constrained delegation information
 * @li #KRB5_PAC_UPN_DNS_INFO       -  User principal name and DNS information
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a PAC handle.
 *
 * @param [in] context         Library context
 * @param [in] pac             PAC to be freed
 *
 * This function frees the contents of @a pac and the structure itself.
 */
    /* *
 * Retrieve a buffer value from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  type            Type of buffer to retrieve
 * @param [out] data            Buffer value
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return an array of buffer types in a PAC handle.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] len             Number of entries in @a types
 * @param [out] types           Array of buffer types
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Create an empty Privilege Attribute Certificate (PAC) handle.
 *
 * @param [in]  context         Library context
 * @param [out] pac             New PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:32"]
pub mod kdb_log_h {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:32"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop_hdr.h:32"]
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
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:30"]
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
    use super::krb5_h::{krb5_enctype, krb5_int32, krb5_context,
                        krb5_error_code};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "365:1"]
        pub fn krb5_db_destroy(kcontext: krb5_context,
                               db_args: *mut *mut libc::c_char)
         -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:30"]
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
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "137:14"]
        pub static mut stdin: *mut FILE;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:27"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "825:1"]
        pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:27"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "50:12"]
        pub static mut optind: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
                      __shortopts: *const libc::c_char) -> libc::c_int;
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
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/dbutil/kdb5_util.h:32"]
pub mod kdb5_util_h {
    use super::admin_h::kadm5_config_params;
    use super::krb5_h::{krb5_enctype, krb5_deltat, krb5_timestamp, krb5_flags,
                        krb5_int32, krb5_kvno, krb5_context, krb5_boolean};
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "32:14"]
        pub static mut progname: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "40:28"]
        pub static mut global_params: kadm5_config_params;
        #[no_mangle]
        #[c2rust::src_loc = "43:15"]
        pub static mut db5util_db_args: *mut *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn usage();
        #[no_mangle]
        #[c2rust::src_loc = "39:21"]
        pub static mut util_context: krb5_context;
        #[no_mangle]
        #[c2rust::src_loc = "37:21"]
        pub static mut dbactive: krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "38:12"]
        pub static mut exit_status: libc::c_int;
    }
}
pub use self::types_h::{__uint16_t, __int32_t, __uint32_t, __off_t,
                        __off64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _profile_t};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::kdb_log_h::{_kdb_log_context, kdb_hlog_t, kdb_hlog};
pub use self::iprop_h::{kdb_sno_t, kdbe_time_t};
pub use self::iprop_hdr_h::{iprop_role, IPROP_REPLICA, IPROP_MASTER,
                            IPROP_NULL};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, com_err};
pub use self::kdb_h::{__krb5_key_salt_tuple, krb5_key_salt_tuple,
                      krb5_db_destroy};
pub use self::admin_h::{_kadm5_config_params, kadm5_config_params};
use self::stdio_h::{printf, fgets, stdin};
use self::unistd_h::unlink;
use self::getopt_core_h::{optind, getopt};
use self::libintl_h::dgettext;
use self::string_h::strcmp;
use self::kdb5_util_h::{progname, global_params, db5util_db_args, usage,
                        util_context, dbactive, exit_status};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/dbutil/kdb5_destroy.c - Destroy a KDC database */
/*
 * Copyright 1990, 2008 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "38:7"]
pub static mut yes: *mut libc::c_char =
    b"yes\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
/* \n to compare against result of
                                           fgets */
#[no_mangle]
#[c2rust::src_loc = "41:1"]
pub unsafe extern "C" fn kdb5_destroy(mut argc: libc::c_int,
                                      mut argv: *mut *mut libc::c_char) {
    extern "C" {
        #[link_name = "optind"]
        pub static mut optind_0: libc::c_int;
    }
    let mut optchar: libc::c_int = 0;
    let mut dbname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 5] = [0; 5];
    let mut retval1: krb5_error_code = 0;
    let mut force: libc::c_int = 0 as libc::c_int;
    dbname = global_params.dbname;
    optind = 1 as libc::c_int;
    loop  {
        optchar =
            getopt(argc, argv as *const *mut libc::c_char,
                   b"f\x00" as *const u8 as *const libc::c_char);
        if !(optchar != -(1 as libc::c_int)) { break ; }
        match optchar {
            102 => {
                force += 1
                /*NOTREACHED*/
            }
            63 | _ => { usage(); return }
        }
    }
    if force == 0 {
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Deleting KDC database stored in \'%s\', are you sure?\n\x00"
                            as *const u8 as *const libc::c_char), dbname);
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"(type \'yes\' to confirm)? \x00" as *const u8 as
                            *const libc::c_char));
        if fgets(buf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                     as libc::c_int, stdin).is_null() {
            exit_status += 1;
            return
        }
        if strcmp(buf.as_mut_ptr(), yes) != 0 { exit_status += 1; return }
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"OK, deleting database \'%s\'...\n\x00" as *const u8
                            as *const libc::c_char), dbname);
    }
    retval1 = krb5_db_destroy(util_context, db5util_db_args);
    if retval1 != 0 {
        com_err(progname, retval1 as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"deleting database \'%s\'\x00" as *const u8 as
                             *const libc::c_char), dbname);
        exit_status += 1;
        return
    }
    if global_params.iprop_enabled != 0 {
        unlink(global_params.iprop_logfile);
    }
    dbactive = 0 as libc::c_int as krb5_boolean;
    printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                    b"** Database \'%s\' destroyed.\n\x00" as *const u8 as
                        *const libc::c_char), dbname);
}
