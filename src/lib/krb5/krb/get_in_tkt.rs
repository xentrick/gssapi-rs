use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
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
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
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
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "175:1"]
    pub type krb5_msgtype = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
    #[c2rust::src_loc = "182:1"]
    pub type krb5_keyusage = krb5_int32;
    #[c2rust::src_loc = "185:1"]
    pub type krb5_preauthtype = krb5_int32;
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
    #[c2rust::src_loc = "226:1"]
    pub type krb5_const_pointer = *const libc::c_void;
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
    /* *< First component of
                                                NT_WELLKNOWN principals */
    /* *< Windows 2000 UPN and SID */
    /* *< NT 4 style name */
    /* *< NT 4 style name and SID */
    /* * Constant version of krb5_principal_data */
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "323:16"]
    pub struct _krb5_address {
        pub magic: krb5_magic,
        pub addrtype: krb5_addrtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< Anonymous principal name */
    /*
 * end "base-defs.h"
 */
    /*
 * begin "hostaddr.h"
 */
    /* * Structure for address */
    #[c2rust::src_loc = "323:1"]
    pub type krb5_address = _krb5_address;
    /* *
 * Set the send subkey in an auth context with a keyblock.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] keyblock         Send subkey
 *
 * This function sets the send subkey in @a ac to a copy of @a keyblock.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
    /* *
 * Set the send subkey in an auth context.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Send subkey
 *
 * This function sets the send subkey in @a ac to @a key, incrementing its
 * reference count.
 *
 * @version New in 1.9
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the receiving subkey in an auth context with a keyblock.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] keyblock         Receiving subkey
 *
 * This function sets the receiving subkey in @a ac to a copy of @a keyblock.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the receiving subkey in an auth context.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] key              Receiving subkey
 *
 * This function sets the receiving subkey in @a ac to @a key, incrementing its
 * reference count.
 *
 * @version New in 1.9
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* * @deprecated Replaced by krb5_auth_con_getsendsubkey(). */
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
    /* *
 * Generate auth context addresses from a connected socket.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] infd             Connected socket descriptor
 * @param [in] flags            Flags
 *
 * This function sets the local and/or remote addresses in @a auth_context
 * based on the local and remote endpoints of the socket @a infd.  The
 * following flags determine the operations performed:
 *
 * @li #KRB5_AUTH_CONTEXT_GENERATE_LOCAL_ADDR   Generate local address.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_REMOTE_ADDR  Generate remote address.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_LOCAL_FULL_ADDR  Generate local address and port.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_REMOTE_FULL_ADDR Generate remote address and port.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set time offset field in a krb5_context structure.
 *
 * @param [in] context          Library context
 * @param [in] seconds          Real time, seconds portion
 * @param [in] microseconds     Real time, microseconds portion
 *
 * This function sets the time offset in @a context to the difference between
 * the system time and the real time as determined by @a seconds and @a
 * microseconds.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return the time offsets from the os context.
 *
 * @param [in]  context         Library context
 * @param [out] seconds         Time offset, seconds portion
 * @param [out] microseconds    Time offset, microseconds portion
 *
 * This function returns the time offsets in @a context.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* str_conv.c */
/* *
 * Convert a string to an encryption type.
 *
 * @param [in]  string          String to convert to an encryption type
 * @param [out] enctypep        Encryption type
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a salt type.
 *
 * @param [in]  string          String to convert to an encryption type
 * @param [out] salttypep       Salt type to be filled in
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a checksum type.
 *
 * @param [in]  string          String to be converted
 * @param [out] cksumtypep      Checksum type to be filled in
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a timestamp.
 *
 * @param [in]  string          String to be converted
 * @param [out] timestampp      Pointer to timestamp
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a delta time value.
 *
 * @param [in]  string          String to be converted
 * @param [out] deltatp         Delta time to be filled in
 *
 * @retval 0  Success; otherwise - KRB5_DELTAT_BADFORMAT
 */
    /* *
 * Convert an encryption type to a string.
 *
 * @param [in]  enctype         Encryption type
 * @param [out] buffer          Buffer to hold encryption type string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert an encryption type to a name or alias.
 *
 * @param [in]  enctype         Encryption type
 * @param [in]  shortest        Flag
 * @param [out] buffer          Buffer to hold encryption type string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * If @a shortest is FALSE, this function returns the enctype's canonical name
 * (like "aes128-cts-hmac-sha1-96").  If @a shortest is TRUE, it return the
 * enctype's shortest alias (like "aes128-cts").
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a salt type to a string.
 *
 * @param [in]  salttype        Salttype to convert
 * @param [out] buffer          Buffer to receive the converted string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a checksum type to a string.
 *
 * @param [in]  cksumtype       Checksum type
 * @param [out] buffer          Buffer to hold converted checksum type
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a timestamp to a string.
 *
 * @param [in]  timestamp       Timestamp to convert
 * @param [out] buffer          Buffer to hold converted timestamp
 * @param [in]  buflen          Storage available in @a buffer
 *
 * The string is returned in the locale's appropriate date and time
 * representation.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a timestamp to a string, with optional output padding
 *
 * @param [in]  timestamp       Timestamp to convert
 * @param [out] buffer          Buffer to hold the converted timestamp
 * @param [in]  buflen          Length of buffer
 * @param [in]  pad             Optional value to pad @a buffer if converted
 *                              timestamp does not fill it
 *
 * If @a pad is not NULL, @a buffer is padded out to @a buflen - 1 characters
 * with the value of *@a pad.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a relative time value to a string.
 *
 * @param [in]  deltat          Relative time value to convert
 * @param [out] buffer          Buffer to hold time string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* The name of the Kerberos ticket granting service... and its size */
    /* flags for recvauth */
    /* initial ticket api functions */
    /* * Text for prompt used in prompter callback function.  */
    /* *< The prompt to show to the user */
    /* *< Boolean; informative prompt or hidden (e.g. PIN) */
    /* *< Must be allocated before call to  prompt routine */
    /* * Pointer to a prompter callback function. */
    /* *
 * Prompt user for password.
 *
 * @param [in] context          Library context
 * @param      data             Unused (callback argument)
 * @param [in] name             Name to output during prompt
 * @param [in] banner           Banner to output during prompt
 * @param [in] num_prompts      Number of prompts in @a prompts
 * @param [in] prompts          Array of prompts and replies
 *
 * This function is intended to be used as a prompter callback for
 * krb5_get_init_creds_password() or krb5_init_creds_init().
 *
 * Writes @a name and @a banner to stdout, each followed by a newline, then
 * writes each prompt field in the @a prompts array, followed by ": ", and sets
 * the reply field of the entry to a line of input read from stdin.  If the
 * hidden flag is set for a prompt, then terminal echoing is turned off when
 * input is read.
 *
 * @retval
 *  0   Success
 * @return
 * Kerberos error codes
 *
 */
    /* *
 * Long-term password responder question
 *
 * This question is asked when the long-term password is needed. It has no
 * challenge and the response is simply the password string.
 *
 * @version New in 1.11
 */
    /* *
 * OTP responder question
 *
 * The OTP responder question is asked when the KDC indicates that an OTP
 * value is required in order to complete the authentication.  The JSON format
 * of the challenge is:
 *
 *  @n {
 *  @n   "service": <string (optional)>,
 *  @n   "tokenInfo": [
 *  @n      {
 *  @n        "flags":     <number>,
 *  @n        "vendor":    <string (optional)>,
 *  @n        "challenge": <string (optional)>,
 *  @n        "length":    <number (optional)>,
 *  @n        "format":    <number (optional)>,
 *  @n        "tokenID":   <string (optional)>,
 *  @n        "algID":     <string (optional)>,
 *  @n      },
 *  @n      ...
 *  @n    ]
 *  @n  }
 *
 * The answer to the question MUST be JSON formatted:
 *
 * @n  {
 * @n    "tokeninfo": <number>,
 * @n    "value":     <string (optional)>,
 * @n    "pin":       <string (optional)>,
 * @n  }
 *
 * For more detail, please see RFC 6560.
 *
 * @version New in 1.11
 */
    /* *
 * These format constants identify the format of the token value.
 */
    /* *
 * This flag indicates that the token value MUST be collected.
 */
    /* *
 * This flag indicates that the PIN value MUST be collected.
 */
    /* *
 * This flag indicates that the token is now in re-synchronization mode with
 * the server.  The user is expected to reply with the next code displayed on
 * the token.
 */
    /* *
 * This flag indicates that the PIN MUST be returned as a separate item. This
 * flag only takes effect if KRB5_RESPONDER_OTP_FLAGS_COLLECT_PIN is set. If
 * this flag is not set, the responder may either concatenate PIN + token value
 * and store it as "value" in the answer or it may return them separately. If
 * they are returned separately, they will be concatenated internally.
 */
    /* *
 * PKINIT responder question
 *
 * The PKINIT responder question is asked when the client needs a password
 * that's being used to protect key information, and is formatted as a JSON
 * object.  A specific identity's flags value, if not zero, is the bitwise-OR
 * of one or more of the KRB5_RESPONDER_PKINIT_FLAGS_TOKEN_* flags defined
 * below, and possibly other flags to be added later.  Any resemblance to
 * similarly-named CKF_* values in the PKCS#11 API should not be depended on.
 *
 *  @n {
 *  @n     identity <string> : flags <number>,
 *  @n     ...
 *  @n }
 *
 * The answer to the question MUST be JSON formatted:
 *
 *  @n {
 *  @n     identity <string> : password <string>,
 *  @n     ...
 *  @n }
 *
 * @version New in 1.12
 */
    /* *
 * This flag indicates that an incorrect PIN was supplied at least once since
 * the last time the correct PIN was supplied.
 */
    /* *
 * This flag indicates that supplying an incorrect PIN will cause the token to
 * lock itself.
 */
    /* *
 * This flag indicates that the user PIN is locked, and you can't log in to the
 * token with it.
 */
    /* *
 * A container for a set of preauthentication questions and answers
 *
 * A responder context is supplied by the krb5 authentication system to a @ref
 * krb5_responder_fn callback.  It contains a list of questions and can receive
 * answers.  Questions contained in a responder context can be listed using
 * krb5_responder_list_questions(), retrieved using
 * krb5_responder_get_challenge(), or answered using
 * krb5_responder_set_answer().  The form of a question's challenge and
 * answer depend on the question name.
 *
 * @version New in 1.11
 */
    /* *
 * List the question names contained in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 *
 * Return a pointer to a null-terminated list of question names which are
 * present in @a rctx.  The pointer is an alias, valid only as long as the
 * lifetime of @a rctx, and should not be modified or freed by the caller.  A
 * question's challenge can be retrieved using krb5_responder_get_challenge()
 * and answered using krb5_responder_set_answer().
 *
 * @version New in 1.11
 */
    /* *
 * Retrieve the challenge data for a given question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 *
 * Return a pointer to a C string containing the challenge for @a question
 * within @a rctx, or NULL if the question is not present in @a rctx.  The
 * structure of the question depends on the question name, but will always be
 * printable UTF-8 text.  The returned pointer is an alias, valid only as long
 * as the lifetime of @a rctx, and should not be modified or freed by the
 * caller.
 *
 * @version New in 1.11
 */
    /* *
 * Answer a named question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 * @param [in] answer           The string to set (MUST be printable UTF-8)
 *
 * This function supplies an answer to @a question within @a rctx.  The
 * appropriate form of the answer depends on the question name.
 *
 * @retval EINVAL @a question is not present within @a rctx
 *
 * @version New in 1.11
 */
    /* *
 * Responder function for an initial credential exchange.
 *
 * @param [in] ctx              Library context
 * @param [in] data             Callback data
 * @param [in] rctx             Responder context
 *
 * A responder function is like a prompter function, but is used for handling
 * questions and answers as potentially complex data types.  Client
 * preauthentication modules will insert a set of named "questions" into
 * the responder context.  Each question may optionally contain a challenge.
 * This challenge is printable UTF-8, but may be an encoded value.  The
 * precise encoding and contents of the challenge are specific to the question
 * asked.  When the responder is called, it should answer all the questions it
 * understands.  Like the challenge, the answer MUST be printable UTF-8, but
 * may contain structured/encoded data formatted to the expected answer format
 * of the question.
 *
 * If a required question is unanswered, the prompter may be called.
 */
    /* -1 when not specified. */
    /* -1 when not specified. */
    /* *
 * Decode the KRB5_RESPONDER_QUESTION_OTP to a C struct.
 *
 * A convenience function which parses the KRB5_RESPONDER_QUESTION_OTP
 * question challenge data, making it available in native C.  The main feature
 * of this function is the ability to interact with OTP tokens without parsing
 * the JSON.
 *
 * The returned value must be passed to krb5_responder_otp_challenge_free() to
 * be freed.
 *
 * @param [in]  ctx             Library context
 * @param [in]  rctx            Responder context
 * @param [out] chl             Challenge structure
 *
 * @version New in 1.11
 */
    /* *
 * Answer the KRB5_RESPONDER_QUESTION_OTP question.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] ti               The index of the tokeninfo selected
 * @param [in] value            The value to set, or NULL for none
 * @param [in] pin              The pin to set, or NULL for none
 *
 * @version New in 1.11
 */
    /* *
 * Free the value returned by krb5_responder_otp_get_challenge().
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] chl              The challenge to free
 *
 * @version New in 1.11
 */
    /* 0 when not specified or not applicable. */
    /* *
 * Decode the KRB5_RESPONDER_QUESTION_PKINIT to a C struct.
 *
 * A convenience function which parses the KRB5_RESPONDER_QUESTION_PKINIT
 * question challenge data, making it available in native C.  The main feature
 * of this function is the ability to read the challenge without parsing
 * the JSON.
 *
 * The returned value must be passed to krb5_responder_pkinit_challenge_free()
 * to be freed.
 *
 * @param [in]  ctx             Library context
 * @param [in]  rctx            Responder context
 * @param [out] chl_out         Challenge structure
 *
 * @version New in 1.12
 */
    /* *
 * Answer the KRB5_RESPONDER_QUESTION_PKINIT question for one identity.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] identity         The identity for which a PIN is being supplied
 * @param [in] pin              The provided PIN, or NULL for none
 *
 * @version New in 1.12
 */
    /* *
 * Free the value returned by krb5_responder_pkinit_get_challenge().
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] chl              The challenge to free
 *
 * @version New in 1.12
 */
    /* * Store options for @c _krb5_get_init_creds */
    /* *
 * Allocate a new initial credential options structure.
 *
 * @param [in]  context         Library context
 * @param [out] opt             New options structure
 *
 * This function is the preferred way to create an options structure for
 * getting initial credentials, and is required to make use of certain options.
 * Use krb5_get_init_creds_opt_free() to free @a opt when it is no longer
 * needed.
 *
 * @retval 0 - Success; Kerberos errors otherwise.
 */
    /* *
 * Free initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure to free
 *
 * @sa krb5_get_init_creds_opt_alloc()
 */
    /* * @deprecated Use krb5_get_init_creds_opt_alloc() instead. */
    /* *
 * Set the ticket lifetime in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] tkt_life         Ticket lifetime
 */
    /* *
 * Set the ticket renewal lifetime in initial credential options.
 *
 * @param [in] opt              Pointer to @a options field
 * @param [in] renew_life       Ticket renewal lifetime
 */
    /* *
 * Set or unset the forwardable flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] forwardable      Whether credentials should be forwardable
 */
    /* *
 * Set or unset the proxiable flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] proxiable        Whether credentials should be proxiable
 */
    /* *
 * Set or unset the canonicalize flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] canonicalize     Whether to canonicalize client principal
 */
    /* *
 * Set or unset the anonymous flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] anonymous        Whether to make an anonymous request
 *
 * This function may be used to request anonymous credentials from the KDC by
 * setting @a anonymous to non-zero.  Note that anonymous credentials are only
 * a request; clients must verify that credentials are anonymous if that is a
 * requirement.
 */
    /* *
 * Set allowable encryption types in initial credential options.
 *
 * @param [in] opt               Options structure
 * @param [in] etype_list        Array of encryption types
 * @param [in] etype_list_length Length of @a etype_list
 */
    /* *
 * Set address restrictions in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] addresses        Null-terminated array of addresses
 */
    /* *
 * Set preauthentication types in initial credential options.
 *
 * @param [in] opt                 Options structure
 * @param [in] preauth_list        Array of preauthentication types
 * @param [in] preauth_list_length Length of @a preauth_list
 *
 * This function can be used to perform optimistic preauthentication when
 * getting initial credentials, in combination with
 * krb5_get_init_creds_opt_set_salt() and krb5_get_init_creds_opt_set_pa().
 */
    /* *
 * Set salt for optimistic preauthentication in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] salt             Salt data
 *
 * When getting initial credentials with a password, a salt string it used to
 * convert the password to a key.  Normally this salt is obtained from the
 * first KDC reply, but when performing optimistic preauthentication, the
 * client may need to supply the salt string with this function.
 */
    /* *
 * Set or unset change-password-prompt flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] prompt           Whether to prompt to change password
 *
 * This flag is on by default.  It controls whether
 * krb5_get_init_creds_password() will react to an expired-password error by
 * prompting for a new password and attempting to change the old one.
 */
    /* * Generic preauth option attribute/value pairs */
    /* *
 * Supply options for preauthentication in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] attr             Preauthentication option name
 * @param [in] value            Preauthentication option value
 *
 * This function allows the caller to supply options for preauthentication.
 * The values of @a attr and @a value are supplied to each preauthentication
 * module available within @a context.
 */
    /* *
 * Set location of FAST armor ccache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] fast_ccache_name Credential cache name
 *
 * Sets the location of a credential cache containing an armor ticket to
 * protect an initial credential exchange using the FAST protocol extension.
 *
 * In version 1.7, setting an armor ccache requires that FAST be used for the
 * exchange.  In version 1.8 or later, setting the armor ccache causes FAST to
 * be used if the KDC supports it; krb5_get_init_creds_opt_set_fast_flags()
 * must be used to require that FAST be used.
 */
    /* *
 * Set FAST armor cache in initial credential options.
 *
 * @param [in] context           Library context
 * @param [in] opt               Options
 * @param [in] ccache            Credential cache handle
 *
 * This function is similar to krb5_get_init_creds_opt_set_fast_ccache_name(),
 * but uses a credential cache handle instead of a name.
 *
 * @version New in 1.9
 */
    /* *
 * Set an input credential cache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] ccache           Credential cache handle
 *
 * If an input credential cache is set, then the krb5_get_init_creds family of
 * APIs will read settings from it.  Setting an input ccache is desirable when
 * the application wishes to perform authentication in the same way (using the
 * same preauthentication mechanisms, and making the same non-security-
 * sensitive choices) as the previous authentication attempt, which stored
 * information in the passed-in ccache.
 *
 * @version New in 1.11
 */
    /* *
 * Set an output credential cache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] ccache           Credential cache handle
 *
 * If an output credential cache is set, then the krb5_get_init_creds family of
 * APIs will write credentials to it.  Setting an output ccache is desirable
 * both because it simplifies calling code and because it permits the
 * krb5_get_init_creds APIs to write out configuration information about the
 * realm to the ccache.
 */
    /* *
 * @brief Ask the KDC to include or not include a PAC in the ticket
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] req_pac          Whether to request a PAC or not
 *
 * If this option is set, the AS request will include a PAC-REQUEST pa-data
 * item explicitly asking the KDC to either include or not include a privilege
 * attribute certificate in the ticket authorization data.  By default, no
 * request is made; typically the KDC will default to including a PAC if it
 * supports them.
 *
 * @version New in 1.15
 */
    /* *
 * Set FAST flags in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] flags            FAST flags
 *
 * The following flag values are valid:
 * @li #KRB5_FAST_REQUIRED - Require FAST to be used
 *
 * @retval
 * 0 - Success; Kerberos errors otherwise.
 */
    /* *
 * Retrieve FAST flags from initial credential options.
 *
 * @param [in]  context         Library context
 * @param [in]  opt             Options
 * @param [out] out_flags       FAST flags
 *
 * @retval
 * 0 - Success; Kerberos errors otherwise.
 */
    /* Fast flags*/
    /* *< Require KDC to support FAST*/
    /* *
 * Set an expiration callback in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] cb               Callback function
 * @param [in] data             Callback argument
 *
 * Set a callback to receive password and account expiration times.
 *
 * This option only applies to krb5_get_init_creds_password().  @a cb will be
 * invoked if and only if credentials are successfully acquired.  The callback
 * will receive the @a context from the krb5_get_init_creds_password() call and
 * the @a data argument supplied with this API.  The remaining arguments should
 * be interpreted as follows:
 *
 * If @a is_last_req is true, then the KDC reply contained last-req entries
 * which unambiguously indicated the password expiration, account expiration,
 * or both.  (If either value was not present, the corresponding argument will
 * be 0.)  Furthermore, a non-zero @a password_expiration should be taken as a
 * suggestion from the KDC that a warning be displayed.
 *
 * If @a is_last_req is false, then @a account_expiration will be 0 and @a
 * password_expiration will contain the expiration time of either the password
 * or account, or 0 if no expiration time was indicated in the KDC reply.  The
 * callback should independently decide whether to display a password
 * expiration warning.
 *
 * Note that @a cb may be invoked even if credentials are being acquired for
 * the kadmin/changepw service in order to change the password.  It is the
 * caller's responsibility to avoid displaying a password expiry warning in
 * this case.
 *
 * @warning Setting an expire callback with this API will cause
 * krb5_get_init_creds_password() not to send password expiry warnings to the
 * prompter, as it ordinarily may.
 *
 * @version New in 1.9
 */
    /* *
 * Set the responder function in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] responder        Responder function
 * @param [in] data             Responder data argument
 *
 * @version New in 1.11
 */
    /* *
 * Get initial credentials using a password.
 *
 * @param [in]  context         Library context
 * @param [out] creds           New credentials
 * @param [in]  client          Client principal
 * @param [in]  password        Password (or NULL)
 * @param [in]  prompter        Prompter function
 * @param [in]  data            Prompter callback data
 * @param [in]  start_time      Time when ticket becomes valid (0 for now)
 * @param [in]  in_tkt_service  Service name of initial credentials (or NULL)
 * @param [in]  k5_gic_options  Initial credential options
 *
 * This function requests KDC for an initial credentials for @a client using @a
 * password.  If @a password is NULL, a password will be prompted for using @a
 * prompter if necessary.  If @a in_tkt_service is specified, it is parsed as a
 * principal name (with the realm ignored) and used as the service principal
 * for the request; otherwise the ticket-granting service is used.
 *
 * @sa krb5_verify_init_creds()
 *
 * @retval
 *  0    Success
 * @retval
 *  EINVAL Invalid argument
 * @retval
 *  KRB5_KDC_UNREACH Cannot contact any KDC for requested realm
 * @retval
 *  KRB5_PREAUTH_FAILED Generic Pre-athentication failure
 * @retval
 *  KRB5_LIBOS_PWDINTR Password read interrupted
 * @retval
 *  KRB5_REALM_CANT_RESOLVE Cannot resolve network address for KDC in requested realm
 * @retval
 *  KRB5KDC_ERR_KEY_EXP Password has expired
 * @retval
 *  KRB5_LIBOS_BADPWDMATCH Password mismatch
 * @retval
 *  KRB5_CHPW_PWDNULL New password cannot be zero length
 * @retval
 *  KRB5_CHPW_FAIL Password change failed
 * @return
 * Kerberos error codes
 */
    /* *
 * Retrieve enctype, salt and s2kparams from KDC
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal whose information is requested
 * @param [in]  opt             Initial credential options
 * @param [out] enctype_out     The enctype chosen by KDC
 * @param [out] salt_out        Salt returned from KDC
 * @param [out] s2kparams_out   String-to-key parameters returned from KDC
 *
 * Send an initial ticket request for @a principal and extract the encryption
 * type, salt type, and string-to-key parameters from the KDC response.  If the
 * KDC provides no etype-info, set @a enctype_out to @c ENCTYPE_NULL and set @a
 * salt_out and @a s2kparams_out to empty.  If the KDC etype-info provides no
 * salt, compute the default salt and place it in @a salt_out.  If the KDC
 * etype-info provides no string-to-key parameters, set @a s2kparams_out to
 * empty.
 *
 * @a opt may be used to specify options which affect the initial request, such
 * as request encryption types or a FAST armor cache (see
 * krb5_get_init_creds_opt_set_etype_list() and
 * krb5_get_init_creds_opt_set_fast_ccache_name()).
 *
 * Use krb5_free_data_contents() to free @a salt_out and @a s2kparams_out when
 * they are no longer needed.
 *
 * @version New in 1.17
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    /* *< More responses needed */
    /* *
 * Free an initial credentials context.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 */
    /* *
 * Acquire credentials using an initial credentials context.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 *
 * This function synchronously obtains credentials using a context created by
 * krb5_init_creds_init().  On successful return, the credentials can be
 * retrieved with krb5_init_creds_get_creds().
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve acquired credentials from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] creds           Acquired credentials
 *
 * This function copies the acquired initial credentials from @a ctx into @a
 * creds, after the successful completion of krb5_init_creds_get() or
 * krb5_init_creds_step().  Use krb5_free_cred_contents() to free @a creds when
 * it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Get the last error from KDC from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] error           Error from KDC, or NULL if none was received
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Create a context for acquiring initial credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal to get initial creds for
 * @param [in]  prompter        Prompter callback
 * @param [in]  data            Prompter callback argument
 * @param [in]  start_time      Time when credentials become valid (0 for now)
 * @param [in]  options         Options structure (NULL for default)
 * @param [out] ctx             New initial credentials context
 *
 * This function creates a new context for acquiring initial credentials.  Use
 * krb5_init_creds_free() to free @a ctx when it is no longer needed.
 *
 * Any subsequent calls to krb5_init_creds_step(), krb5_init_creds_get(), or
 * krb5_init_creds_free() for this initial credentials context must use the
 * same @a context argument as the one passed to this function.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Specify a keytab to use for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] keytab           Key table handle
 *
 * This function supplies a keytab containing the client key for an initial
 * credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Get the next KDC request for acquiring initial credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [in]  in              KDC response (empty on the first call)
 * @param [out] out             Next KDC request
 * @param [out] realm           Realm for next KDC request
 * @param [out] flags           Output flags
 *
 * This function constructs the next KDC request in an initial credential
 * exchange, allowing the caller to control the transport of KDC requests and
 * replies.  On the first call, @a in should be set to an empty buffer; on
 * subsequent calls, it should be set to the KDC's reply to the previous
 * request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in
 * @a out.  If no more requests are needed, @a flags will not contain
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the initial credential exchange has failed.
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set a password for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] password         Password
 *
 * This function supplies a password to be used to construct the client key for
 * an initial credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Specify a service principal for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] service          Service principal string
 *
 * This function supplies a service principal string to acquire initial
 * credentials for instead of the default krbtgt service.  @a service is parsed
 * as a principal name; any realm part is ignored.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve ticket times from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] times           Ticket times for acquired credentials
 *
 * The initial credentials context must have completed obtaining credentials
 * via either krb5_init_creds_get() or krb5_init_creds_step().
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Create a context to get credentials from a KDC's Ticket Granting Service.
 *
 * @param[in]  context          Library context
 * @param[in]  ccache           Credential cache handle
 * @param[in]  creds            Input credentials
 * @param[in]  options          @ref KRB5_GC options for this request.
 * @param[out] ctx              New TGS request context
 *
 * This function prepares to obtain credentials matching @a creds, either by
 * retrieving them from @a ccache or by making requests to ticket-granting
 * services beginning with a ticket-granting ticket for the client principal's
 * realm.
 *
 * The resulting TGS acquisition context can be used asynchronously with
 * krb5_tkt_creds_step() or synchronously with krb5_tkt_creds_get().  See also
 * krb5_get_credentials() for synchronous use.
 *
 * Use krb5_tkt_creds_free() to free @a ctx when it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Synchronously obtain credentials using a TGS request context.
 *
 * @param[in] context           Library context
 * @param[in] ctx               TGS request context
 *
 * This function synchronously obtains credentials using a context created by
 * krb5_tkt_creds_init().  On successful return, the credentials can be
 * retrieved with krb5_tkt_creds_get_creds().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve acquired credentials from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] creds            Acquired credentials
 *
 * This function copies the acquired initial credentials from @a ctx into @a
 * creds, after the successful completion of krb5_tkt_creds_get() or
 * krb5_tkt_creds_step().  Use krb5_free_cred_contents() to free @a creds when
 * it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a TGS request context.
 *
 * @param[in]  context  Library context
 * @param[in]  ctx      TGS request context
 *
 * @version New in 1.9
 */
    /* *< More responses needed */
    /* *
 * Get the next KDC request in a TGS exchange.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[in]  in               KDC response (empty on the first call)
 * @param[out] out              Next KDC request
 * @param[out] realm            Realm for next KDC request
 * @param[out] flags            Output flags
 *
 * This function constructs the next KDC request for a TGS exchange, allowing
 * the caller to control the transport of KDC requests and replies.  On the
 * first call, @a in should be set to an empty buffer; on subsequent calls, it
 * should be set to the KDC's reply to the previous request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in @a
 * out.  If no more requests are needed, @a flags will not contain
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the TGS exchange has failed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve ticket times from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] times            Ticket times for acquired credentials
 *
 * The TGS request context must have completed obtaining credentials via either
 * krb5_tkt_creds_get() or krb5_tkt_creds_step().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Get initial credentials using a key table.
 *
 * @param [in]  context         Library context
 * @param [out] creds           New credentials
 * @param [in]  client          Client principal
 * @param [in]  arg_keytab      Key table handle
 * @param [in]  start_time      Time when ticket becomes valid (0 for now)
 * @param [in]  in_tkt_service  Service name of initial credentials (or NULL)
 * @param [in]  k5_gic_options  Initial credential options
 *
 * This function requests KDC for an initial credentials for @a client using a
 * client key stored in @a arg_keytab.  If @a in_tkt_service is specified, it
 * is parsed as a principal name (with the realm ignored) and used as the
 * service principal for the request; otherwise the ticket-granting service is
 * used.
 *
 * @sa krb5_verify_init_creds()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
    /* *< boolean */
    /* *
 * Initialize a credential verification options structure.
 *
 * @param [in] k5_vic_options   Verification options structure
 */
    /* *
 * Set whether credential verification is required.
 *
 * @param [in] k5_vic_options   Verification options structure
 * @param [in] ap_req_nofail    Whether to require successful verification
 *
 * This function determines how krb5_verify_init_creds() behaves if no keytab
 * information is available.  If @a ap_req_nofail is @c FALSE, verification
 * will be skipped in this case and krb5_verify_init_creds() will return
 * successfully.  If @a ap_req_nofail is @c TRUE, krb5_verify_init_creds() will
 * not return successfully unless verification can be performed.
 *
 * If this function is not used, the behavior of krb5_verify_init_creds() is
 * determined through configuration.
 */
    /* *
 * Verify initial credentials against a keytab.
 *
 * @param [in] context          Library context
 * @param [in] creds            Initial credentials to be verified
 * @param [in] server           Server principal (or NULL)
 * @param [in] keytab           Key table (NULL to use default keytab)
 * @param [in] ccache           Credential cache for fetched creds (or NULL)
 * @param [in] options          Verification options (NULL for default options)
 *
 * This function attempts to verify that @a creds were obtained from a KDC with
 * knowledge of a key in @a keytab, or the default keytab if @a keytab is NULL.
 * If @a server is provided, the highest-kvno key entry for that principal name
 * is used to verify the credentials; otherwise, all unique "host" service
 * principals in the keytab are tried.
 *
 * If the specified keytab does not exist, or is empty, or cannot be read, or
 * does not contain an entry for @a server, then credential verification may be
 * skipped unless configuration demands that it succeed.  The caller can
 * control this behavior by providing a verification options structure; see
 * krb5_verify_init_creds_opt_init() and
 * krb5_verify_init_creds_opt_set_ap_req_nofail().
 *
 * If @a ccache is NULL, any additional credentials fetched during the
 * verification process will be destroyed.  If @a ccache points to NULL, a
 * memory ccache will be created for the additional credentials and returned in
 * @a ccache.  If @a ccache points to a valid credential cache handle, the
 * additional credentials will be stored in that cache.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Get validated credentials from the KDC.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Validated credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a validated credential using a postdated credential from
 * @a ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential;
 * otherwise, the ticket-granting service is used.
 *
 * If successful, the validated credential is placed in @a creds.
 *
 * @sa krb5_get_renewed_creds()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_NO_2ND_TKT Request missing second ticket
 * @retval
 * KRB5_NO_TKT_SUPPLIED Request did not supply a ticket
 * @retval
 * KRB5_PRINC_NOMATCH Requested principal and ticket do not match
 * @retval
 * KRB5_KDCREP_MODIFIED KDC reply did not match expectations
 * @retval
 * KRB5_KDCREP_SKEW Clock skew too great in KDC reply
 * @return
 * Kerberos error codes
 */
    /* *
 * Get renewed credential from KDC using an existing credential.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Renewed credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a renewed credential using an existing one from @a
 * ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential; otherwise,
 * the ticket-granting service is used.
 *
 * If successful, the renewed credential is placed in @a creds.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Decode an ASN.1-formatted ticket.
 *
 * @param [in]  code            ASN.1-formatted ticket
 * @param [out] rep             Decoded ticket information
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve a string value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       String value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_boolean()
 */
    /* *
 * Retrieve a boolean value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       Boolean value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_string()
 */
    /*
 * Prompter enhancements
 */
/* * Prompt for password */
    /* * Prompt for new password (during password change) */
    /* * Prompt for new password again */
    /* * Prompt for preauthentication data (such as an OTP value) */
    /* *
 * Get prompt types array from a context.
 *
 * @param [in] context          Library context
 *
 * @return
 * Pointer to an array of prompt types corresponding to the prompter's @a
 * prompts arguments.  Each type has one of the following values:
 *  @li #KRB5_PROMPT_TYPE_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD_AGAIN
 *  @li #KRB5_PROMPT_TYPE_PREAUTH
 */
    /* Error reporting */
/* *
 * Set an extended error message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] ...              printf(3) style parameters
 */
    /* *
 * Set an extended error message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] args             List of vprintf(3) style arguments
 */
    /* *
 * Add a prefix to the message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the current message for @a code.  The
 * prefix will be separated from the old message with a colon and space.
 */
    /* *
 * Add a prefix to the message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_prepend_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
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
    #[c2rust::src_loc = "398:16"]
    pub struct _krb5_enc_data {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub kvno: krb5_kvno,
        pub ciphertext: krb5_data,
    }
    #[c2rust::src_loc = "398:1"]
    pub type krb5_enc_data = _krb5_enc_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    /* *< RFC 8070 */
    /* *< MS-KILE and MS-SFU */
    /* *< currently must be zero */
    /* * Transited encoding types */
    /* * alternate authentication types */
    /* authorization data types. See RFC 4120 section 5.2.6 */
    /* * @defgroup KRB5_AUTHDATA KRB5_AUTHDATA
 * @{
 */
    /* *< RFC 4537 */
    /* *< formerly 142 in krb5 1.8 */
    /* * @} */
    /* end of KRB5_AUTHDATA group */
    /* password change constants */
    /* *< Success */
    /* *< Malformed request */
    /* *< Server error */
    /* *< Authentication error */
    /* *< Password change rejected */
    /* These are Microsoft's extensions in RFC 3244, and it looks like
   they'll become standardized, possibly with other additions.  */
    /* *< Not authorized */
    /* *< Unknown RPC version */
    /* * The presented credentials were not obtained using a password directly */
    /*
 * end "proto.h"
 */
    /* Time set */
/* * Ticket start time, end time, and renewal duration. */
    #[c2rust::src_loc = "1936:1"]
    pub type krb5_ticket_times = _krb5_ticket_times;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1946:16"]
    pub struct _krb5_authdata {
        pub magic: krb5_magic,
        pub ad_type: krb5_authdatatype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< Time at which KDC issued the initial ticket that corresponds to this ticket */
    /* XXX ? should ktime in KDC_REP == authtime
       in ticket? otherwise client can't get this */
    /* *< optional in ticket, if not present, use @a authtime */
    /* *< Ticket expiration time */
    /* *< Latest time at which renewal of ticket can be valid */
    /* * Structure for auth data */
    #[c2rust::src_loc = "1946:1"]
    pub type krb5_authdata = _krb5_authdata;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1954:16"]
    pub struct _krb5_transited {
        pub magic: krb5_magic,
        pub tr_type: krb5_octet,
        pub tr_contents: krb5_data,
    }
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
    /* * Structure for transited encoding */
    #[c2rust::src_loc = "1954:1"]
    pub type krb5_transited = _krb5_transited;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1961:16"]
    pub struct _krb5_enc_tkt_part {
        pub magic: krb5_magic,
        pub flags: krb5_flags,
        pub session: *mut krb5_keyblock,
        pub client: krb5_principal,
        pub transited: krb5_transited,
        pub times: krb5_ticket_times,
        pub caddrs: *mut *mut krb5_address,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    /* *< Transited encoding type */
    /* *< Contents */
    /* * Encrypted part of ticket. */
    #[c2rust::src_loc = "1961:1"]
    pub type krb5_enc_tkt_part = _krb5_enc_tkt_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1979:16"]
    pub struct _krb5_ticket {
        pub magic: krb5_magic,
        pub server: krb5_principal,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_tkt_part,
    }
    /* to-be-encrypted portion */
    /* *< flags */
    /* *< session key: includes enctype */
    /* *< client name/realm */
    /* *< list of transited realms */
    /* *< auth, start, end, renew_till */
    /* *< array of ptrs to addresses */
    /* *< auth data */
    /* *
 * Ticket structure.
 *
 * The C representation of the ticket message, with a pointer to the
 * C representation of the encrypted part.
 */
    #[c2rust::src_loc = "1979:1"]
    pub type krb5_ticket = _krb5_ticket;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2013:16"]
    pub struct _krb5_creds {
        pub magic: krb5_magic,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub keyblock: krb5_keyblock,
        pub times: krb5_ticket_times,
        pub is_skey: krb5_boolean,
        pub ticket_flags: krb5_flags,
        pub addresses: *mut *mut krb5_address,
        pub ticket: krb5_data,
        pub second_ticket: krb5_data,
        pub authdata: *mut *mut krb5_authdata,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* * Credentials structure including ticket, session key, and lifetime info. */
    #[c2rust::src_loc = "2013:1"]
    pub type krb5_creds = _krb5_creds;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2031:16"]
    pub struct _krb5_last_req_entry {
        pub magic: krb5_magic,
        pub lr_type: krb5_int32,
        pub value: krb5_timestamp,
    }
    /* *< client's principal identifier */
    /* *< server's principal identifier */
    /* *< session encryption key info */
    /* *< lifetime info */
    /* *< true if ticket is encrypted in
                                           another ticket's skey */
    /* *< flags in ticket */
    /* *< addrs in ticket */
    /* *< ticket string itself */
    /* *< second ticket, if related to
                                           ticket (via DUPLICATE-SKEY or
                                           ENC-TKT-IN-SKEY) */
    /* *< authorization data */
    /* * Last request entry */
    #[c2rust::src_loc = "2031:1"]
    pub type krb5_last_req_entry = _krb5_last_req_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< LR type */
    /* *< Timestamp */
    /* * Pre-authentication data */
    #[c2rust::src_loc = "2038:1"]
    pub type krb5_pa_data = _krb5_pa_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2054:16"]
    pub struct _krb5_kdc_req {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub padata: *mut *mut krb5_pa_data,
        pub kdc_options: krb5_flags,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub from: krb5_timestamp,
        pub till: krb5_timestamp,
        pub rtime: krb5_timestamp,
        pub nonce: krb5_int32,
        pub nktypes: libc::c_int,
        pub ktype: *mut krb5_enctype,
        pub addresses: *mut *mut krb5_address,
        pub authorization_data: krb5_enc_data,
        pub unenc_authdata: *mut *mut krb5_authdata,
        pub second_ticket: *mut *mut krb5_ticket,
    }
    /* *< Preauthentication data type */
    /* *< Length of data */
    /* *< Data */
    /* * C representation of KDC-REQ protocol message, including KDC-REQ-BODY */
    #[c2rust::src_loc = "2054:1"]
    pub type krb5_kdc_req = _krb5_kdc_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2079:16"]
    pub struct _krb5_enc_kdc_rep_part {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub session: *mut krb5_keyblock,
        pub last_req: *mut *mut krb5_last_req_entry,
        pub nonce: krb5_int32,
        pub key_exp: krb5_timestamp,
        pub flags: krb5_flags,
        pub times: krb5_ticket_times,
        pub server: krb5_principal,
        pub caddrs: *mut *mut krb5_address,
        pub enc_padata: *mut *mut krb5_pa_data,
    }
    /* *< KRB5_AS_REQ or KRB5_TGS_REQ */
    /* *< Preauthentication data */
    /* real body */
    /* *< Requested options */
    /* *< Client principal and realm */
    /* *< Server principal and realm */
    /* *< Requested start time */
    /* *< Requested end time */
    /* *< Requested renewable end time */
    /* *< Nonce to match request and response */
    /* *< Number of enctypes */
    /* *< Requested enctypes */
    /* *< Requested addresses (optional) */
    /* *< Encrypted authz data (optional) */
    /* *< Unencrypted authz data */
    /* *< Second ticket array (optional) */
    /* *
 * C representation of @c EncKDCRepPart protocol message.
 *
 * This is the cleartext message that is encrypted and inserted in @c KDC-REP.
 */
    #[c2rust::src_loc = "2079:1"]
    pub type krb5_enc_kdc_rep_part = _krb5_enc_kdc_rep_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2095:16"]
    pub struct _krb5_kdc_rep {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub padata: *mut *mut krb5_pa_data,
        pub client: krb5_principal,
        pub ticket: *mut krb5_ticket,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_kdc_rep_part,
    }
    /* encrypted part: */
    /* *< krb5 message type */
    /* *< Session key */
    /* *< Array of pointers to entries */
    /* *< Nonce from request */
    /* *< Expiration date */
    /* *< Ticket flags */
    /* *< Lifetime info */
    /* *< Server's principal identifier */
    /* *< Array of ptrs to addrs, optional */
    /* *< Encrypted preauthentication data */
    /* * Representation of the @c KDC-REP protocol message. */
    #[c2rust::src_loc = "2095:1"]
    pub type krb5_kdc_rep = _krb5_kdc_rep;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2107:16"]
    pub struct _krb5_error {
        pub magic: krb5_magic,
        pub ctime: krb5_timestamp,
        pub cusec: krb5_int32,
        pub susec: krb5_int32,
        pub stime: krb5_timestamp,
        pub error: krb5_ui_4,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub text: krb5_data,
        pub e_data: krb5_data,
    }
    /* cleartext part: */
    /* *< KRB5_AS_REP or KRB5_KDC_REP */
    /* *< Preauthentication data from KDC */
    /* *< Client principal and realm */
    /* *< Ticket */
    /* *< Encrypted part of reply */
    /* *< Unencrypted version, if available */
    /* * Error message structure */
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2216:16"]
    pub struct _krb5_pa_pac_req {
        pub include_pac: krb5_boolean,
    }
    #[c2rust::src_loc = "2216:1"]
    pub type krb5_pa_pac_req = _krb5_pa_pac_req;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6424:16"]
    pub struct _krb5_prompt {
        pub prompt: *mut libc::c_char,
        pub hidden: libc::c_int,
        pub reply: *mut krb5_data,
    }
    #[c2rust::src_loc = "6424:1"]
    pub type krb5_prompt = _krb5_prompt;
    #[c2rust::src_loc = "6431:1"]
    pub type krb5_prompter_fct
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char, _: libc::c_int,
                                    _: *mut krb5_prompt) -> krb5_error_code>;
    #[c2rust::src_loc = "6603:1"]
    pub type krb5_responder_context = *mut krb5_responder_context_st;
    #[c2rust::src_loc = "6681:1"]
    pub type krb5_responder_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_responder_context)
                   -> krb5_error_code>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6811:16"]
    pub struct _krb5_get_init_creds_opt {
        pub flags: krb5_flags,
        pub tkt_life: krb5_deltat,
        pub renew_life: krb5_deltat,
        pub forwardable: libc::c_int,
        pub proxiable: libc::c_int,
        pub etype_list: *mut krb5_enctype,
        pub etype_list_length: libc::c_int,
        pub address_list: *mut *mut krb5_address,
        pub preauth_list: *mut krb5_preauthtype,
        pub preauth_list_length: libc::c_int,
        pub salt: *mut krb5_data,
    }
    #[c2rust::src_loc = "6811:1"]
    pub type krb5_get_init_creds_opt = _krb5_get_init_creds_opt;
    #[c2rust::src_loc = "7313:1"]
    pub type krb5_init_creds_context = *mut _krb5_init_creds_context;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    use super::init_creds_ctx_h::{krb5_responder_context_st,
                                  _krb5_init_creds_context};
    extern "C" {
        /* some of these may be meaningless in certain contexts */
        /* *< Client sec portion; optional */
        /* *< Client usec portion; optional */
        /* *< Server usec portion */
        /* *< Server sec portion */
        /* *< Error code (protocol error #'s) */
        /* *< Client principal and realm */
        /* *< Server principal and realm */
        /* *< Descriptive text */
        /* *< Additional error-describing data */
        /* * TRUE if a PAC should be included in TGS-REP */
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
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
        /* *
 * Build an anonymous principal.
 *
 * This function returns constant storage that must not be freed.
 *
 * @sa #KRB5_ANONYMOUS_PRINCSTR
 */
        #[no_mangle]
        #[c2rust::src_loc = "309:1"]
        pub fn krb5_anonymous_principal() -> krb5_const_principal;
        /* *
 * Decrypt data using a key (operates on keyblock).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Encrypted data
 * @param [out]    output       Decrypted data
 *
 * This function decrypts the data block @a input and stores the output into @a
 * output. The actual decryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result.  The usual practice is to allocate an output buffer as
 * long as the ciphertext, and let krb5_c_decrypt() trim @a output->length.
 * For some enctypes, the resulting @a output->length may include padding
 * bytes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "560:1"]
        pub fn krb5_c_decrypt(context: krb5_context,
                              key: *const krb5_keyblock, usage: krb5_keyusage,
                              cipher_state: *const krb5_data,
                              input: *const krb5_enc_data,
                              output: *mut krb5_data) -> krb5_error_code;
        /* *
 * Compute the KRB-FX-CF2 combination of two keys and pepper strings.
 *
 * @param [in]  context         Library context
 * @param [in]  k1              KDC contribution key
 * @param [in]  pepper1         String "PKINIT"
 * @param [in]  k2              Reply key
 * @param [in]  pepper2         String "KeyExchange"
 * @param [out] out             Output key
 *
 * This function computes the KRB-FX-CF2 function over its inputs and places
 * the results in a newly allocated keyblock.  This function is simple in that
 * it assumes that @a pepper1 and @a pepper2 are C strings with no internal
 * nulls and that the enctype of the result will be the same as that of @a k1.
 * @a k1 and @a k2 may be of different enctypes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "727:1"]
        pub fn krb5_c_fx_cf2_simple(context: krb5_context,
                                    k1: *const krb5_keyblock,
                                    pepper1: *const libc::c_char,
                                    k2: *const krb5_keyblock,
                                    pepper2: *const libc::c_char,
                                    out: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        /* KRB5_DEPRECATED */
        /* *
 * Initialize a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] principal        Default principal name
 *
 * Destroy any existing contents of @a cache and initialize it for the default
 * principal @a principal.
 *
 * @retval
 *  0  Success
 * @return
 *  System errors; Permission errors; Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2368:1"]
        pub fn krb5_cc_initialize(context: krb5_context, cache: krb5_ccache,
                                  principal: krb5_principal)
         -> krb5_error_code;
        /* *
 * Store credentials in a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] creds            Credentials to be stored in cache
 *
 * This function stores @a creds into @a cache.  If @a creds->server and the
 * server in the decoded ticket @a creds->ticket differ, the credentials will
 * be stored under both server principal names.
 *
 * @retval
 *  0  Success
 * @return Permission errors; storage failure errors; Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2421:1"]
        pub fn krb5_cc_store_cred(context: krb5_context, cache: krb5_ccache,
                                  creds: *mut krb5_creds) -> krb5_error_code;
        /* *< Ignore realm if present */
        /* *
 * Convert a string principal name to a krb5_principal with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [in]  flags           Flag
 * @param [out] principal_out   New principal
 *
 * Similar to krb5_parse_name(), this function converts a single-string
 * representation of a principal name to a krb5_principal structure.
 *
 * The following flags are valid:
 * @li #KRB5_PRINCIPAL_PARSE_NO_REALM - no realm must be present in @a name
 * @li #KRB5_PRINCIPAL_PARSE_REQUIRE_REALM - realm must be present in @a name
 * @li #KRB5_PRINCIPAL_PARSE_ENTERPRISE - create single-component enterprise
 *                                        principal
 * @li #KRB5_PRINCIPAL_PARSE_IGNORE_REALM - ignore realm if present in @a name
 *
 * If @c KRB5_PRINCIPAL_PARSE_NO_REALM or @c KRB5_PRINCIPAL_PARSE_IGNORE_REALM
 * is specified in @a flags, the realm of the new principal will be empty.
 * Otherwise, the default realm for @a context will be used if @a name does not
 * specify a realm.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3468:1"]
        pub fn krb5_parse_name_flags(context: krb5_context,
                                     name: *const libc::c_char,
                                     flags: libc::c_int,
                                     principal_out: *mut krb5_principal)
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
        /* *
 * Compare the realms of two principals.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * @retval
 * TRUE if the realm names are the same; FALSE otherwise
 */
        #[no_mangle]
        #[c2rust::src_loc = "3650:1"]
        pub fn krb5_realm_compare(context: krb5_context,
                                  princ1: krb5_const_principal,
                                  princ2: krb5_const_principal)
         -> krb5_boolean;
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
 * Compare two principals ignoring realm components.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * Similar to krb5_principal_compare(), but do not compare the realm
 * components of the principals.
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
        #[no_mangle]
        #[c2rust::src_loc = "3682:1"]
        pub fn krb5_principal_compare_any_realm(context: krb5_context,
                                                princ1: krb5_const_principal,
                                                princ2: krb5_const_principal)
         -> krb5_boolean;
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
        #[no_mangle]
        #[c2rust::src_loc = "3766:1"]
        pub fn krb5_copy_keyblock_contents(context: krb5_context,
                                           from: *const krb5_keyblock,
                                           to: *mut krb5_keyblock)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3829:1"]
        pub fn krb5_copy_addresses(context: krb5_context,
                                   inaddr: *const *mut krb5_address,
                                   outaddr: *mut *mut *mut krb5_address)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3995:1"]
        pub fn krb5_build_principal_ext(context: krb5_context,
                                        princ: *mut krb5_principal,
                                        rlen: libc::c_uint,
                                        realm: *const libc::c_char, _: ...)
         -> krb5_error_code;
        /* *
 * Convert a principal name into the default salt for that principal.
 *
 * @param [in]  context         Library context
 * @param [in]  pr              Principal name
 * @param [out] ret             Default salt for @a pr to be filled in
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4313:1"]
        pub fn krb5_principal2salt(context: krb5_context,
                                   pr: krb5_const_principal,
                                   ret: *mut krb5_data) -> krb5_error_code;
        /* *
 * Get a configuration value from a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for this principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [out]    data         Data to be fetched
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4457:1"]
        pub fn krb5_cc_get_config(context: krb5_context, id: krb5_ccache,
                                  principal: krb5_const_principal,
                                  key: *const libc::c_char,
                                  data: *mut krb5_data) -> krb5_error_code;
        /* *
 * Store a configuration value in a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for a specific principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [in]     data         Data to store, or NULL to remove
 *
 * @note Existing configuration under the same key is over-written.
 *
 * @warning Before version 1.10 @a data was assumed to be always non-null.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4481:1"]
        pub fn krb5_cc_set_config(context: krb5_context, id: krb5_ccache,
                                  principal: krb5_const_principal,
                                  key: *const libc::c_char,
                                  data: *mut krb5_data) -> krb5_error_code;
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
 * Free the data stored in array of addresses.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of addresses to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4620:1"]
        pub fn krb5_free_addresses(context: krb5_context,
                                   val: *mut *mut krb5_address);
        /* *
 * Free an error allocated by krb5_read_error() or krb5_sendauth().
 *
 * @param [in] context          Library context
 * @param [in] val              Error data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4655:1"]
        pub fn krb5_free_error(context: krb5_context, val: *mut krb5_error);
        /* *
 * Free the contents of a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4677:1"]
        pub fn krb5_free_cred_contents(context: krb5_context,
                                       val: *mut krb5_creds);
        /* *
 * Free a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Keyblock to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4710:1"]
        pub fn krb5_free_keyblock(context: krb5_context,
                                  val: *mut krb5_keyblock);
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
 * Free a krb5_data structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4743:1"]
        pub fn krb5_free_data(context: krb5_context, val: *mut krb5_data);
        /* *
 * Free the contents of a krb5_data structure and zero the data field.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4758:1"]
        pub fn krb5_free_data_contents(context: krb5_context,
                                       val: *mut krb5_data);
        /* *
 * Free a string representation of a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Name string to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
        /* From krb5/os, but needed by the outside world */
/* *
 * Retrieve the system time of day, in sec and ms, since the epoch.
 *
 * @param [in]  context         Library context
 * @param [out] seconds         System timeofday, seconds portion
 * @param [out] microseconds    System timeofday, microseconds portion
 *
 * This function retrieves the system time of day with the context
 * specific time offset adjustment.
 *
 * @sa krb5_crypto_us_timeofday()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4819:1"]
        pub fn krb5_us_timeofday(context: krb5_context,
                                 seconds: *mut krb5_timestamp,
                                 microseconds: *mut krb5_int32)
         -> krb5_error_code;
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
 * Return all interface addresses for this host.
 *
 * @param [in]  context         Library context
 * @param [out] addr            Array of krb5_address pointers, ending with
 *                              NULL
 *
 * Use krb5_free_addresses() to free @a addr when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4868:1"]
        pub fn krb5_os_localaddr(context: krb5_context,
                                 addr: *mut *mut *mut krb5_address)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6310:1"]
        pub fn krb5_string_to_deltat(string: *mut libc::c_char,
                                     deltatp: *mut krb5_deltat)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6851:1"]
        pub fn krb5_get_init_creds_opt_alloc(context: krb5_context,
                                             opt:
                                                 *mut *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6863:1"]
        pub fn krb5_get_init_creds_opt_free(context: krb5_context,
                                            opt:
                                                *mut krb5_get_init_creds_opt);
        #[no_mangle]
        #[c2rust::src_loc = "6868:1"]
        pub fn krb5_get_init_creds_opt_init(opt:
                                                *mut krb5_get_init_creds_opt);
        #[no_mangle]
        #[c2rust::src_loc = "6877:1"]
        pub fn krb5_get_init_creds_opt_set_tkt_life(opt:
                                                        *mut krb5_get_init_creds_opt,
                                                    tkt_life: krb5_deltat);
        #[no_mangle]
        #[c2rust::src_loc = "6887:1"]
        pub fn krb5_get_init_creds_opt_set_renew_life(opt:
                                                          *mut krb5_get_init_creds_opt,
                                                      renew_life:
                                                          krb5_deltat);
        #[no_mangle]
        #[c2rust::src_loc = "6897:1"]
        pub fn krb5_get_init_creds_opt_set_forwardable(opt:
                                                           *mut krb5_get_init_creds_opt,
                                                       forwardable:
                                                           libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "6907:1"]
        pub fn krb5_get_init_creds_opt_set_proxiable(opt:
                                                         *mut krb5_get_init_creds_opt,
                                                     proxiable: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "6917:1"]
        pub fn krb5_get_init_creds_opt_set_canonicalize(opt:
                                                            *mut krb5_get_init_creds_opt,
                                                        canonicalize:
                                                            libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "6943:1"]
        pub fn krb5_get_init_creds_opt_set_etype_list(opt:
                                                          *mut krb5_get_init_creds_opt,
                                                      etype_list:
                                                          *mut krb5_enctype,
                                                      etype_list_length:
                                                          libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "6954:1"]
        pub fn krb5_get_init_creds_opt_set_address_list(opt:
                                                            *mut krb5_get_init_creds_opt,
                                                        addresses:
                                                            *mut *mut krb5_address);
        #[no_mangle]
        #[c2rust::src_loc = "6969:1"]
        pub fn krb5_get_init_creds_opt_set_preauth_list(opt:
                                                            *mut krb5_get_init_creds_opt,
                                                        preauth_list:
                                                            *mut krb5_preauthtype,
                                                        preauth_list_length:
                                                            libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "8043:1"]
        pub fn krb5_clear_error_message(ctx: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "7926:1"]
        pub fn krb5_prepend_error_message(ctx: krb5_context,
                                          code: krb5_error_code,
                                          fmt: *const libc::c_char, _: ...);
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
    #[c2rust::src_loc = "776:1"]
    pub type krb5_fast_armor = _krb5_fast_armor;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "776:16"]
    pub struct _krb5_fast_armor {
        pub armor_type: krb5_int32,
        pub armor_value: krb5_data,
    }
    #[inline]
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
    }
    #[inline]
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
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
    #[c2rust::src_loc = "2268:1"]
    pub unsafe extern "C" fn string2data(mut str: *mut libc::c_char)
     -> krb5_data {
        return make_data(str as *mut libc::c_void,
                         strlen(str) as libc::c_uint);
    }
    #[inline]
    #[c2rust::src_loc = "2251:1"]
    pub unsafe extern "C" fn make_data(mut data: *mut libc::c_void,
                                       mut len: libc::c_uint) -> krb5_data {
        let mut d: krb5_data =
            krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
        d.magic = -(1760647422 as libc::c_long) as krb5_magic;
        d.data = data as *mut libc::c_char;
        d.length = len;
        return d;
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
    #[c2rust::src_loc = "2368:1"]
    pub unsafe extern "C" fn ts_within(mut a: krb5_timestamp,
                                       mut b: krb5_timestamp,
                                       mut d: krb5_deltat) -> krb5_boolean {
        return (ts_after(a, ts_incr(b, d)) == 0 &&
                    ts_after(b, ts_incr(a, d)) == 0) as libc::c_int as
                   krb5_boolean;
    }
    #[inline]
    #[c2rust::src_loc = "2346:1"]
    pub unsafe extern "C" fn ts_delta(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_deltat {
        return (a as uint32_t).wrapping_sub(b as uint32_t) as krb5_deltat;
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
    #[c2rust::src_loc = "2262:1"]
    pub unsafe extern "C" fn empty_data() -> krb5_data {
        return make_data(0 as *mut libc::c_void,
                         0 as libc::c_int as libc::c_uint);
    }
    #[inline]
    #[c2rust::src_loc = "2315:1"]
    pub unsafe extern "C" fn k5memdup(mut in_0: *const libc::c_void,
                                      mut len: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = k5alloc(len, code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
        return ptr;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data, krb5_context, krb5_pa_data, krb5_kdc_rep,
                        krb5_kdc_req, krb5_error_code, krb5_preauthtype,
                        krb5_pa_pac_req, krb5_timestamp, krb5_ticket,
                        krb5_enc_data, krb5_keyblock, krb5_const_pointer,
                        krb5_error};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stddef_h::size_t;
    use super::string_h::{explicit_bzero, strlen, memcmp, memcpy};
    use super::stdlib_h::{free, calloc};
    use super::stdint_uintn_h::uint32_t;
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
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "2151:1"]
        pub fn krb5_free_kdc_rep(_: krb5_context, _: *mut krb5_kdc_rep);
        #[no_mangle]
        #[c2rust::src_loc = "2150:1"]
        pub fn krb5_free_kdc_req(_: krb5_context, _: *mut krb5_kdc_req);
        #[no_mangle]
        #[c2rust::src_loc = "616:1"]
        pub fn krb5_sendto_kdc(_: krb5_context, _: *const krb5_data,
                               _: *const krb5_data, _: *mut krb5_data,
                               _: *mut libc::c_int, _: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "898:1"]
        pub fn krb5int_copy_data_contents(_: krb5_context,
                                          _: *const krb5_data,
                                          _: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1405:1"]
        pub fn encode_krb5_as_req(rep: *const krb5_kdc_req,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "885:1"]
        pub fn k5_add_pa_data_from_data(list: *mut *mut *mut krb5_pa_data,
                                        pa_type: krb5_preauthtype,
                                        data: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1490:1"]
        pub fn encode_krb5_pa_pac_req(_: *const krb5_pa_pac_req,
                                      _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "890:1"]
        pub fn k5_add_empty_pa_data(list: *mut *mut *mut krb5_pa_data,
                                    pa_type: krb5_preauthtype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1411:1"]
        pub fn encode_krb5_kdc_req_body(rep: *const krb5_kdc_req,
                                        code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1378:1"]
        pub fn encode_krb5_ticket(rep: *const krb5_ticket,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2161:1"]
        pub fn krb5_free_enc_data(_: krb5_context, _: *mut krb5_enc_data);
        #[no_mangle]
        #[c2rust::src_loc = "1589:1"]
        pub fn decode_krb5_encryption_key(output: *const krb5_data,
                                          rep: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1663:1"]
        pub fn decode_krb5_enc_data(output: *const krb5_data,
                                    rep: *mut *mut krb5_enc_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "865:1"]
        pub fn krb5int_find_pa_data(_: krb5_context,
                                    _: *const *mut krb5_pa_data,
                                    _: krb5_preauthtype) -> *mut krb5_pa_data;
        #[no_mangle]
        #[c2rust::src_loc = "2233:1"]
        pub fn krb5_set_time_offsets(_: krb5_context, _: krb5_timestamp,
                                     _: krb5_int32) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2099:1"]
        pub fn krb5_kdc_rep_decrypt_proc(_: krb5_context,
                                         _: *const krb5_keyblock,
                                         _: krb5_const_pointer,
                                         _: *mut krb5_kdc_rep)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1599:1"]
        pub fn decode_krb5_as_rep(output: *const krb5_data,
                                  rep: *mut *mut krb5_kdc_rep)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1645:1"]
        pub fn decode_krb5_error(output: *const krb5_data,
                                 rep: *mut *mut krb5_error)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2085:1"]
        pub fn krb5_get_default_in_tkt_ktypes(_: krb5_context,
                                              _: *mut *mut krb5_enctype)
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn k5_clear_error(ep: *mut errinfo);
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
        #[c2rust::src_loc = "89:1"]
        pub fn profile_get_boolean(profile: profile_t,
                                   name: *const libc::c_char,
                                   subname: *const libc::c_char,
                                   subsubname: *const libc::c_char,
                                   def_val: libc::c_int,
                                   ret_default: *mut libc::c_int)
         -> libc::c_long;
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/init_creds_ctx.h:31"]
pub mod init_creds_ctx_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:8"]
    pub struct krb5_responder_context_st {
        pub items: *mut k5_response_items,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "20:8"]
    pub struct _krb5_init_creds_context {
        pub opt: *mut krb5_get_init_creds_opt,
        pub opt_storage: krb5_get_init_creds_opt,
        pub identify_realm: krb5_boolean,
        pub subject_cert: *const krb5_data,
        pub in_tkt_service: *mut libc::c_char,
        pub prompter: krb5_prompter_fct,
        pub prompter_data: *mut libc::c_void,
        pub gak_fct: get_as_key_fn,
        pub gak_data: *mut libc::c_void,
        pub request_time: krb5_timestamp,
        pub start_time: krb5_deltat,
        pub tkt_life: krb5_deltat,
        pub renew_life: krb5_deltat,
        pub complete: krb5_boolean,
        pub loopcount: libc::c_uint,
        pub gakpw: gak_password,
        pub err_reply: *mut krb5_error,
        pub err_padata: *mut *mut krb5_pa_data,
        pub cred: krb5_creds,
        pub request: *mut krb5_kdc_req,
        pub reply: *mut krb5_kdc_rep,
        pub outer_request_body: *mut krb5_data,
        pub inner_request_body: *mut krb5_data,
        pub encoded_previous_request: *mut krb5_data,
        pub fast_state: *mut krb5int_fast_request_state,
        pub optimistic_padata: *mut *mut krb5_pa_data,
        pub method_padata: *mut *mut krb5_pa_data,
        pub more_padata: *mut *mut krb5_pa_data,
        pub default_salt: krb5_boolean,
        pub salt: krb5_data,
        pub s2kparams: krb5_data,
        pub as_key: krb5_keyblock,
        pub etype: krb5_enctype,
        pub info_pa_permitted: krb5_boolean,
        pub restarted: krb5_boolean,
        pub fallback_disabled: krb5_boolean,
        pub encts_disabled: krb5_boolean,
        pub rctx: krb5_responder_context_st,
        pub selected_preauth_type: krb5_preauthtype,
        pub allowed_preauth_type: krb5_preauthtype,
        pub cc_config_in: k5_json_object,
        pub cc_config_out: k5_json_object,
        pub pa_offset: krb5_timestamp,
        pub pa_offset_usec: krb5_int32,
        pub pa_offset_state: C2RustUnnamed,
        pub preauth_reqctx: krb5_preauth_req_context,
    }
    #[c2rust::src_loc = "9:1"]
    pub type krb5_preauth_req_context = *mut krb5_preauth_req_context_st;
    #[c2rust::src_loc = "75:5"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "75:42"]
    pub const AUTH_OFFSET: C2RustUnnamed = 2;
    #[c2rust::src_loc = "75:27"]
    pub const UNAUTH_OFFSET: C2RustUnnamed = 1;
    #[c2rust::src_loc = "75:12"]
    pub const NO_OFFSET: C2RustUnnamed = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:8"]
    pub struct gak_password {
        pub storage: krb5_data,
        pub password: *const krb5_data,
    }
    use super::int_proto_h::{k5_response_items, get_as_key_fn};
    use super::krb5_h::{krb5_get_init_creds_opt, krb5_boolean, krb5_data,
                        krb5_prompter_fct, krb5_timestamp, krb5_deltat,
                        krb5_error, krb5_pa_data, krb5_creds, krb5_kdc_req,
                        krb5_kdc_rep, krb5_keyblock, krb5_enctype,
                        krb5_preauthtype, krb5_int32, krb5_context,
                        krb5_principal_data, krb5_principal, krb5_error_code};
    use super::fast_h::krb5int_fast_request_state;
    use super::k5_json_h::k5_json_object;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[c2rust::src_loc = "9:16"]
        pub type krb5_preauth_req_context_st;
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn krb5_get_as_key_password(context: krb5_context,
                                        client: krb5_principal,
                                        etype: krb5_enctype,
                                        prompter: krb5_prompter_fct,
                                        prompter_data: *mut libc::c_void,
                                        salt: *mut krb5_data,
                                        params: *mut krb5_data,
                                        as_key: *mut krb5_keyblock,
                                        gak_data: *mut libc::c_void,
                                        ritems: *mut k5_response_items)
         -> krb5_error_code;
    }
    /* !KRB5_INIT_CREDS_CONTEXT */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:28"]
pub mod int_proto_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/int-proto.h - Prototypes for libkrb5 internal functions */
/*
 * Copyright 1990,1991 the Massachusetts Institute of Technology.
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
    #[c2rust::src_loc = "32:1"]
    pub type k5_response_items = k5_response_items_st;
    #[c2rust::src_loc = "34:1"]
    pub type get_as_key_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_principal,
                                    _: krb5_enctype, _: krb5_prompter_fct,
                                    _: *mut libc::c_void, _: *mut krb5_data,
                                    _: *mut krb5_data, _: *mut krb5_keyblock,
                                    _: *mut libc::c_void,
                                    _: *mut k5_response_items)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_principal,
                        krb5_enctype, krb5_prompter_fct, krb5_data,
                        krb5_keyblock, krb5_init_creds_context, krb5_pa_data,
                        krb5_boolean, krb5_preauthtype, krb5_error,
                        krb5_get_init_creds_opt, krb5_kdc_req, krb5_creds,
                        krb5_ccache, krb5_responder_fn};
    use super::k5_int_h::_krb5_context;
    use super::stdint_intn_h::int32_t;
    use super::init_creds_ctx_h::_krb5_init_creds_context;
    use super::k5_err_h::errinfo;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "32:16"]
        pub type k5_response_items_st;
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn krb5int_libdefault_boolean(_: krb5_context,
                                          _: *const krb5_data,
                                          _: *const libc::c_char,
                                          _: *mut libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "47:1"]
        pub fn krb5int_libdefault_string(context: krb5_context,
                                         realm: *const krb5_data,
                                         option: *const libc::c_char,
                                         ret_value: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn k5_generate_nonce(context: krb5_context, out: *mut int32_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "211:1"]
        pub fn k5_preauth(context: krb5_context, ctx: krb5_init_creds_context,
                          in_padata: *mut *mut krb5_pa_data,
                          must_preauth: krb5_boolean,
                          padata_out: *mut *mut *mut krb5_pa_data,
                          pa_type_out: *mut krb5_preauthtype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "216:1"]
        pub fn k5_preauth_tryagain(context: krb5_context,
                                   ctx: krb5_init_creds_context,
                                   pa_type: krb5_preauthtype,
                                   err: *mut krb5_error,
                                   err_padata: *mut *mut krb5_pa_data,
                                   padata_out: *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "227:1"]
        pub fn k5_preauth_note_failed(ctx: krb5_init_creds_context,
                                      pa_type: krb5_preauthtype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "230:1"]
        pub fn k5_preauth_prepare_request(context: krb5_context,
                                          opt: *mut krb5_get_init_creds_opt,
                                          request: *mut krb5_kdc_req);
        #[no_mangle]
        #[c2rust::src_loc = "234:1"]
        pub fn k5_preauth_request_context_init(context: krb5_context,
                                               ctx: krb5_init_creds_context);
        #[no_mangle]
        #[c2rust::src_loc = "238:1"]
        pub fn k5_preauth_request_context_fini(context: krb5_context,
                                               ctx: krb5_init_creds_context);
        #[no_mangle]
        #[c2rust::src_loc = "242:1"]
        pub fn k5_preauth_check_context(context: krb5_context,
                                        ctx: krb5_init_creds_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "245:1"]
        pub fn k5_response_items_new(ri_out: *mut *mut k5_response_items)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "248:1"]
        pub fn k5_response_items_free(ri: *mut k5_response_items);
        #[no_mangle]
        #[c2rust::src_loc = "254:1"]
        pub fn k5_response_items_empty(ri: *const k5_response_items)
         -> krb5_boolean;
        /* Save code and its extended message (if any) in out. */
        #[no_mangle]
        #[c2rust::src_loc = "277:1"]
        pub fn k5_save_ctx_error(ctx: krb5_context, code: krb5_error_code,
                                 out: *mut errinfo);
        /* Return the code from in and restore its extended message (if any). */
        #[no_mangle]
        #[c2rust::src_loc = "281:1"]
        pub fn k5_restore_ctx_error(ctx: krb5_context, in_0: *mut errinfo)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "314:1"]
        pub fn k5_copy_creds_contents(_: krb5_context, _: *const krb5_creds,
                                      _: *mut krb5_creds) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "331:1"]
        pub fn k5_count_etypes(list: *const krb5_enctype) -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "337:1"]
        pub fn k5_gic_opt_get_in_ccache(opt: *mut krb5_get_init_creds_opt)
         -> krb5_ccache;
        #[no_mangle]
        #[c2rust::src_loc = "340:1"]
        pub fn k5_gic_opt_get_out_ccache(opt: *mut krb5_get_init_creds_opt)
         -> krb5_ccache;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn k5_gic_opt_get_responder(opt: *mut krb5_get_init_creds_opt,
                                        responder_out: *mut krb5_responder_fn,
                                        data_out: *mut *mut libc::c_void);
        /* Return -1 if no PAC request option was specified, or the option value as a
 * boolean (0 or 1). */
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn k5_gic_opt_pac_request(opt: *mut krb5_get_init_creds_opt)
         -> libc::c_int;
    }
    /* KRB5_INT_FUNC_PROTO__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-json.h:31"]
pub mod k5_json_h {
    /*
 * Object
 */
    #[c2rust::src_loc = "160:1"]
    pub type k5_json_object = *mut k5_json_object_st;
    /*
 * The k5_json_value C type can represent any kind of JSON value.  It has no
 * static type safety since it is represented using a void pointer, so be
 * careful with it.  Its type can be checked dynamically with k5_json_get_tid()
 * and the above constants.
 */
    #[c2rust::src_loc = "86:1"]
    pub type k5_json_value = *mut libc::c_void;
    #[c2rust::src_loc = "87:1"]
    pub type k5_json_tid = libc::c_uint;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "160:16"]
        pub type k5_json_object_st;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn k5_json_get_tid(val: k5_json_value) -> k5_json_tid;
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn k5_json_release(val: k5_json_value);
        #[no_mangle]
        #[c2rust::src_loc = "164:1"]
        pub fn k5_json_object_create(val_out: *mut k5_json_object)
         -> libc::c_int;
        /* Return the number of mappings in an object. */
        #[no_mangle]
        #[c2rust::src_loc = "169:1"]
        pub fn k5_json_object_count(obj: k5_json_object) -> size_t;
        /*
 * JSON encoding and decoding
 */
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn k5_json_encode(val: k5_json_value,
                              json_out: *mut *mut libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "216:1"]
        pub fn k5_json_decode(str: *const libc::c_char,
                              val_out: *mut k5_json_value) -> libc::c_int;
    }
    /* K5_JSON_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/fast.h:30"]
pub mod fast_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/fast.h */
/*
 * Copyright (C) 2009 by the Massachusetts Institute of Technology.
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:8"]
    pub struct krb5int_fast_request_state {
        pub fast_outer_request: krb5_kdc_req,
        pub armor_key: *mut krb5_keyblock,
        pub armor: *mut krb5_fast_armor,
        pub fast_state_flags: krb5_ui_4,
        pub fast_options: krb5_ui_4,
        pub nonce: krb5_int32,
    }
    #[c2rust::src_loc = "51:1"]
    pub type kdc_req_encoder_proc
        =
        Option<unsafe extern "C" fn(_: *const krb5_kdc_req,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_kdc_req, krb5_keyblock, krb5_ui_4, krb5_int32,
                        krb5_error_code, krb5_data, krb5_context, krb5_error,
                        krb5_pa_data, krb5_boolean, krb5_kdc_rep,
                        krb5_get_init_creds_opt};
    use super::k5_int_h::{krb5_fast_armor, _krb5_context};
    extern "C" {
        /* Perform FAST */
        #[no_mangle]
        #[c2rust::src_loc = "45:1"]
        pub fn krb5int_fast_prep_req_body(context: krb5_context,
                                          state:
                                              *mut krb5int_fast_request_state,
                                          request: *mut krb5_kdc_req,
                                          encoded_req_body:
                                              *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn krb5int_fast_prep_req(context: krb5_context,
                                     state: *mut krb5int_fast_request_state,
                                     request: *mut krb5_kdc_req,
                                     to_be_checksummed: *const krb5_data,
                                     encoder: kdc_req_encoder_proc,
                                     encoded_request: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "62:1"]
        pub fn krb5int_fast_process_error(context: krb5_context,
                                          state:
                                              *mut krb5int_fast_request_state,
                                          err_replyptr: *mut *mut krb5_error,
                                          out_padata:
                                              *mut *mut *mut krb5_pa_data,
                                          retry: *mut krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn krb5int_fast_process_response(context: krb5_context,
                                             state:
                                                 *mut krb5int_fast_request_state,
                                             resp: *mut krb5_kdc_rep,
                                             strengthen_key:
                                                 *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn krb5int_fast_make_state(context: krb5_context,
                                       state:
                                           *mut *mut krb5int_fast_request_state)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn krb5int_fast_free_state(context: krb5_context,
                                       state:
                                           *mut krb5int_fast_request_state);
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn krb5int_fast_as_armor(context: krb5_context,
                                     state: *mut krb5int_fast_request_state,
                                     opt: *mut krb5_get_init_creds_opt,
                                     request: *mut krb5_kdc_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn krb5int_fast_reply_key(context: krb5_context,
                                      strengthen_key: *const krb5_keyblock,
                                      existing_key: *const krb5_keyblock,
                                      output_key: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn krb5int_fast_verify_nego(context: krb5_context,
                                        state:
                                            *mut krb5int_fast_request_state,
                                        rep: *mut krb5_kdc_rep,
                                        request: *mut krb5_data,
                                        decrypting_key: *mut krb5_keyblock,
                                        fast_avail: *mut krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "101:1"]
        pub fn k5_upgrade_to_fast_p(context: krb5_context,
                                    state: *mut krb5int_fast_request_state,
                                    padata: *mut *mut krb5_pa_data)
         -> krb5_boolean;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/time.h:27"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
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
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "176:17"]
        pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
                      _: libc::c_int) -> libc::c_long;
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "276:15"]
        pub fn strspn(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_ulong;
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
#[c2rust::header_src = "/usr/include/assert.h:27"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:29"]
pub mod os_proto_h {
    use super::krb5_h::{krb5_timestamp, krb5_int32, krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn k5_time_with_offset(offset: krb5_timestamp,
                                   offset_usec: krb5_int32,
                                   time_out: *mut krb5_timestamp,
                                   usec_out: *mut krb5_int32)
         -> krb5_error_code;
    }
    /* KRB5_LIBOS_INT_PROTO__ */
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_msgtype, krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_authdatatype, krb5_keyusage, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_const_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _krb5_enc_data, krb5_enc_data,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_transited, krb5_transited,
                       _krb5_enc_tkt_part, krb5_enc_tkt_part, _krb5_ticket,
                       krb5_ticket, _krb5_creds, krb5_creds,
                       _krb5_last_req_entry, krb5_last_req_entry,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _krb5_error, krb5_error, _krb5_pa_pac_req,
                       krb5_pa_pac_req, krb5_ccache, _krb5_prompt,
                       krb5_prompt, krb5_prompter_fct, krb5_responder_context,
                       krb5_responder_fn, _krb5_get_init_creds_opt,
                       krb5_get_init_creds_opt, krb5_init_creds_context,
                       _profile_t, _krb5_ccache, krb5_anonymous_principal,
                       krb5_c_decrypt, krb5_c_fx_cf2_simple,
                       krb5_cc_initialize, krb5_cc_store_cred,
                       krb5_parse_name_flags, krb5_unparse_name,
                       krb5_realm_compare, krb5_principal_compare,
                       krb5_principal_compare_any_realm,
                       krb5_copy_keyblock_contents, krb5_copy_principal,
                       krb5_copy_addresses, krb5_build_principal_ext,
                       krb5_principal2salt, krb5_cc_get_config,
                       krb5_cc_set_config, krb5_free_principal,
                       krb5_free_addresses, krb5_free_error,
                       krb5_free_cred_contents, krb5_free_keyblock,
                       krb5_free_keyblock_contents, krb5_free_data,
                       krb5_free_data_contents, krb5_free_unparsed_name,
                       krb5_us_timeofday, krb5_timeofday, krb5_os_localaddr,
                       krb5_string_to_deltat, krb5_get_init_creds_opt_alloc,
                       krb5_get_init_creds_opt_free,
                       krb5_get_init_creds_opt_init,
                       krb5_get_init_creds_opt_set_tkt_life,
                       krb5_get_init_creds_opt_set_renew_life,
                       krb5_get_init_creds_opt_set_forwardable,
                       krb5_get_init_creds_opt_set_proxiable,
                       krb5_get_init_creds_opt_set_canonicalize,
                       krb5_get_init_creds_opt_set_etype_list,
                       krb5_get_init_creds_opt_set_address_list,
                       krb5_get_init_creds_opt_set_preauth_list,
                       krb5_set_error_message, krb5_clear_error_message,
                       krb5_prepend_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_fast_armor, _krb5_fast_armor,
                         zapfree, ts_after, ts_incr, string2data, make_data,
                         k5alloc, k5calloc, ts_within, ts_delta,
                         data_eq_string, k5memdup0, empty_data, k5memdup,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_free_pa_data,
                         krb5_free_kdc_rep, krb5_free_kdc_req,
                         krb5_sendto_kdc, krb5int_copy_data_contents,
                         encode_krb5_as_req, k5_add_pa_data_from_data,
                         encode_krb5_pa_pac_req, k5_add_empty_pa_data,
                         encode_krb5_kdc_req_body, encode_krb5_ticket,
                         krb5_free_enc_data, decode_krb5_encryption_key,
                         decode_krb5_enc_data, krb5int_find_pa_data,
                         krb5_set_time_offsets, krb5_kdc_rep_decrypt_proc,
                         decode_krb5_as_rep, decode_krb5_error,
                         krb5_get_default_in_tkt_ktypes};
pub use self::k5_err_h::{errinfo, k5_clear_error};
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_get_boolean};
pub use self::init_creds_ctx_h::{krb5_responder_context_st,
                                 _krb5_init_creds_context,
                                 krb5_preauth_req_context, C2RustUnnamed,
                                 AUTH_OFFSET, UNAUTH_OFFSET, NO_OFFSET,
                                 gak_password, krb5_preauth_req_context_st,
                                 krb5_get_as_key_password};
pub use self::int_proto_h::{k5_response_items, get_as_key_fn,
                            k5_response_items_st, krb5int_libdefault_boolean,
                            krb5int_libdefault_string, k5_generate_nonce,
                            k5_preauth, k5_preauth_tryagain,
                            k5_preauth_note_failed,
                            k5_preauth_prepare_request,
                            k5_preauth_request_context_init,
                            k5_preauth_request_context_fini,
                            k5_preauth_check_context, k5_response_items_new,
                            k5_response_items_free, k5_response_items_empty,
                            k5_save_ctx_error, k5_restore_ctx_error,
                            k5_copy_creds_contents, k5_count_etypes,
                            k5_gic_opt_get_in_ccache,
                            k5_gic_opt_get_out_ccache,
                            k5_gic_opt_get_responder, k5_gic_opt_pac_request};
pub use self::k5_json_h::{k5_json_object, k5_json_value, k5_json_tid,
                          k5_json_object_st, k5_json_get_tid, k5_json_release,
                          k5_json_object_create, k5_json_object_count,
                          k5_json_encode, k5_json_decode};
pub use self::fast_h::{krb5int_fast_request_state, kdc_req_encoder_proc,
                       krb5int_fast_prep_req_body, krb5int_fast_prep_req,
                       krb5int_fast_process_error,
                       krb5int_fast_process_response, krb5int_fast_make_state,
                       krb5int_fast_free_state, krb5int_fast_as_armor,
                       krb5int_fast_reply_key, krb5int_fast_verify_nego,
                       k5_upgrade_to_fast_p};
use self::stdio_h::asprintf;
use self::time_h::time;
use self::libintl_h::dgettext;
use self::stdlib_h::{free, calloc, malloc, strtol};
use self::string_h::{explicit_bzero, strlen, strspn, strdup, memcmp, memset,
                     memcpy};
use self::assert_h::__assert_fail;
use self::k5_trace_h::krb5int_trace;
use self::os_proto_h::k5_time_with_offset;
/*
 * Decrypt the AS reply in ctx, populating ctx->reply->enc_part2.  If
 * strengthen_key is not null, combine it with the reply key as specified in
 * RFC 6113 section 5.4.3.  Place the key used in *key_out.
 */
#[c2rust::src_loc = "47:1"]
unsafe extern "C" fn decrypt_as_reply(mut context: krb5_context,
                                      mut ctx: krb5_init_creds_context,
                                      mut strengthen_key:
                                          *const krb5_keyblock,
                                      mut key_out: *mut krb5_keyblock)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut responder: krb5_responder_fn = None;
    let mut responder_data: *mut libc::c_void = 0 as *mut libc::c_void;
    memset(key_out as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    memset(&mut key as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    if (*ctx).as_key.length != 0 {
        /* The reply key was computed or replaced during preauth processing;
         * try it. */
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"AS key determined by preauth: {keyblock}\x00" as
                              *const u8 as *const libc::c_char,
                          &mut (*ctx).as_key as *mut krb5_keyblock);
        }
        ret =
            krb5int_fast_reply_key(context, strengthen_key,
                                   &mut (*ctx).as_key, &mut key);
        if ret != 0 { return ret }
        ret =
            krb5_kdc_rep_decrypt_proc(context, &mut key,
                                      0 as krb5_const_pointer, (*ctx).reply);
        if ret == 0 { *key_out = key; return 0 as libc::c_int }
        krb5_free_keyblock_contents(context, &mut key);
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Decrypt with preauth AS key failed: {kerr}\x00" as
                              *const u8 as *const libc::c_char, ret);
        }
        /*
         * For two reasons, we fall back to trying or retrying the gak_fct if
         * this fails:
         *
         * 1. The KDC might encrypt the reply using a different enctype than
         *    the AS key we computed during preauth.
         *
         * 2. For 1.1.1 and prior KDC's, when SAM is used with USE_SAD_AS_KEY,
         *    the AS-REP is encrypted in the client long-term key instead of
         *    the SAD.
         *
         * The gak_fct for krb5_get_init_creds_with_password() caches the
         * password, so this fallback does not result in a second password
         * prompt.
         */
    } else {
        /*
         * No AS key was computed during preauth processing, perhaps because
         * preauth was not used.  If the caller supplied a responder callback,
         * possibly invoke it before calling the gak_fct for real.
         */
        k5_gic_opt_get_responder((*ctx).opt, &mut responder,
                                 &mut responder_data);
        if responder.is_some() {
            /* Indicate a need for the AS key by calling the gak_fct with a
             * NULL as_key. */
            ret =
                (*ctx).gak_fct.expect("non-null function pointer")(context,
                                                                   (*(*ctx).request).client,
                                                                   (*ctx).etype,
                                                                   None,
                                                                   0 as
                                                                       *mut libc::c_void,
                                                                   0 as
                                                                       *mut krb5_data,
                                                                   0 as
                                                                       *mut krb5_data,
                                                                   0 as
                                                                       *mut krb5_keyblock,
                                                                   (*ctx).gak_data,
                                                                   (*ctx).rctx.items);
            if ret != 0 { return ret }
            /* If that produced a responder question, invoke the responder. */
            if k5_response_items_empty((*ctx).rctx.items) == 0 {
                ret =
                    Some(responder.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                                            responder_data,
                                                                                                            &mut (*ctx).rctx);
                if ret != 0 { return ret }
            }
        }
    }
    /* Compute or re-compute the AS key, prompting for the password if
     * necessary. */
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Getting AS key, salt \"{data}\", params \"{data}\"\x00"
                          as *const u8 as *const libc::c_char,
                      &mut (*ctx).salt as *mut krb5_data,
                      &mut (*ctx).s2kparams as *mut krb5_data);
    }
    ret =
        (*ctx).gak_fct.expect("non-null function pointer")(context,
                                                           (*(*ctx).request).client,
                                                           (*(*ctx).reply).enc_part.enctype,
                                                           (*ctx).prompter,
                                                           (*ctx).prompter_data,
                                                           &mut (*ctx).salt,
                                                           &mut (*ctx).s2kparams,
                                                           &mut (*ctx).as_key,
                                                           (*ctx).gak_data,
                                                           (*ctx).rctx.items);
    if ret != 0 { return ret }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"AS key obtained from gak_fct: {keyblock}\x00" as
                          *const u8 as *const libc::c_char,
                      &mut (*ctx).as_key as *mut krb5_keyblock);
    }
    ret =
        krb5int_fast_reply_key(context, strengthen_key, &mut (*ctx).as_key,
                               &mut key);
    if ret != 0 { return ret }
    ret =
        krb5_kdc_rep_decrypt_proc(context, &mut key, 0 as krb5_const_pointer,
                                  (*ctx).reply);
    if ret != 0 { krb5_free_keyblock_contents(context, &mut key); return ret }
    *key_out = key;
    return 0 as libc::c_int;
}
/* *
 * Fully anonymous replies include a pa_pkinit_kx padata type including the KDC
 * contribution key.  This routine confirms that the session key is of the
 * right form for fully anonymous requests.  It is here rather than in the
 * preauth code because the session key cannot be verified until the AS reply
 * is decrypted and the preauth code all runs before the AS reply is decrypted.
 */
#[c2rust::src_loc = "146:1"]
unsafe extern "C" fn verify_anonymous(mut context: krb5_context,
                                      mut request: *mut krb5_kdc_req,
                                      mut reply: *mut krb5_kdc_rep,
                                      mut as_key: *mut krb5_keyblock)
 -> krb5_error_code {
    let mut current_block: u64; /* Only applies to fully anonymous */
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut scratch: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut kdc_key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut expected: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut enc: *mut krb5_enc_data = 0 as *mut krb5_enc_data;
    let mut session: *mut krb5_keyblock = (*(*reply).enc_part2).session;
    if krb5_principal_compare_any_realm(context,
                                        (*request).client as
                                            krb5_const_principal,
                                        krb5_anonymous_principal()) == 0 {
        return 0 as libc::c_int
    }
    pa = krb5int_find_pa_data(context, (*reply).padata, 147 as libc::c_int);
    if pa.is_null() {
        current_block = 464771064215715192;
    } else {
        scratch.length = (*pa).length;
        scratch.data = (*pa).contents as *mut libc::c_char;
        ret = decode_krb5_enc_data(&mut scratch, &mut enc);
        if ret != 0 {
            current_block = 13638914137047553509;
        } else {
            scratch.data =
                k5alloc((*enc).ciphertext.length as size_t, &mut ret) as
                    *mut libc::c_char;
            if ret != 0 {
                current_block = 13638914137047553509;
            } else {
                scratch.length = (*enc).ciphertext.length;
                ret =
                    krb5_c_decrypt(context, as_key, 44 as libc::c_int,
                                   0 as *const krb5_data, enc, &mut scratch);
                if ret != 0 {
                    free(scratch.data as *mut libc::c_void);
                    current_block = 13638914137047553509;
                } else {
                    ret =
                        decode_krb5_encryption_key(&mut scratch,
                                                   &mut kdc_key);
                    explicit_bzero(scratch.data as *mut libc::c_void,
                                   scratch.length as size_t);
                    free(scratch.data as *mut libc::c_void);
                    if ret != 0 {
                        current_block = 13638914137047553509;
                    } else {
                        ret =
                            krb5_c_fx_cf2_simple(context, kdc_key,
                                                 b"PKINIT\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 as_key,
                                                 b"KEYEXCHANGE\x00" as
                                                     *const u8 as
                                                     *const libc::c_char,
                                                 &mut expected);
                        if ret != 0 {
                            current_block = 13638914137047553509;
                        } else if (*expected).enctype != (*session).enctype ||
                                      (*expected).length != (*session).length
                                      ||
                                      memcmp((*expected).contents as
                                                 *const libc::c_void,
                                             (*session).contents as
                                                 *const libc::c_void,
                                             (*expected).length as
                                                 libc::c_ulong) !=
                                          0 as libc::c_int {
                            current_block = 464771064215715192;
                        } else { current_block = 13638914137047553509; }
                    }
                }
            }
        }
    }
    match current_block {
        464771064215715192 => {
            ret = -(1765328237 as libc::c_long) as krb5_error_code;
            krb5_set_error_message(context, ret,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Reply has wrong form of session key for anonymous request\x00"
                                                as *const u8 as
                                                *const libc::c_char));
        }
        _ => { }
    }
    if !kdc_key.is_null() { krb5_free_keyblock(context, kdc_key); }
    if !expected.is_null() { krb5_free_keyblock(context, expected); }
    if !enc.is_null() { krb5_free_enc_data(context, enc); }
    return ret;
}
#[c2rust::src_loc = "206:1"]
unsafe extern "C" fn verify_as_reply(mut context: krb5_context,
                                     mut time_now: krb5_timestamp,
                                     mut request: *mut krb5_kdc_req,
                                     mut as_reply: *mut krb5_kdc_rep)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut canon_req: libc::c_int = 0;
    let mut canon_ok: libc::c_int = 0;
    let mut time_offset: krb5_timestamp = 0;
    /* check the contents for sanity: */
    if (*(*as_reply).enc_part2).times.starttime == 0 {
        (*(*as_reply).enc_part2).times.starttime =
            (*(*as_reply).enc_part2).times.authtime
    }
    /*
     * We only allow the AS-REP server name to be changed if the
     * caller set the canonicalize flag (or requested an enterprise
     * principal) and we requested (and received) a TGT.
     */
    canon_req =
        ((*request).kdc_options & 0x10000 as libc::c_int != 0 as libc::c_int
             || (*(*request).client).type_0 == 10 as libc::c_int ||
             (*request).kdc_options & 0x8000 as libc::c_int != 0) as
            libc::c_int;
    if canon_req != 0 {
        canon_ok =
            ((*(*request).server).length == 2 as libc::c_int &&
                 data_eq_string(*(*(*request).server).data.offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                                b"krbtgt\x00" as *const u8 as
                                    *const libc::c_char) != 0 &&
                 ((*(*(*as_reply).enc_part2).server).length ==
                      2 as libc::c_int &&
                      data_eq_string(*(*(*(*as_reply).enc_part2).server).data.offset(0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize),
                                     b"krbtgt\x00" as *const u8 as
                                         *const libc::c_char) != 0)) as
                libc::c_int
    } else { canon_ok = 0 as libc::c_int }
    if canon_ok == 0 &&
           krb5_principal_compare(context,
                                  (*(*as_reply).enc_part2).server as
                                      krb5_const_principal,
                                  (*request).server as krb5_const_principal)
               == 0 ||
           canon_req == 0 &&
               krb5_principal_compare(context,
                                      (*as_reply).client as
                                          krb5_const_principal,
                                      (*request).client as
                                          krb5_const_principal) == 0 ||
           krb5_principal_compare(context,
                                  (*(*as_reply).enc_part2).server as
                                      krb5_const_principal,
                                  (*(*as_reply).ticket).server as
                                      krb5_const_principal) == 0 ||
           (*request).nonce != (*(*as_reply).enc_part2).nonce ||
           (*request).kdc_options & 0x2000000 as libc::c_int != 0 &&
               (*request).from != 0 as libc::c_int &&
               (*request).from != (*(*as_reply).enc_part2).times.starttime ||
           (*request).till != 0 as libc::c_int &&
               ts_after((*(*as_reply).enc_part2).times.endtime,
                        (*request).till) != 0 ||
           (*request).kdc_options & 0x800000 as libc::c_int != 0 &&
               (*request).rtime != 0 as libc::c_int &&
               ts_after((*(*as_reply).enc_part2).times.renew_till,
                        (*request).rtime) != 0 ||
           (*request).kdc_options & 0x10 as libc::c_int != 0 &&
               (*request).kdc_options & 0x800000 as libc::c_int == 0 &&
               (*(*as_reply).enc_part2).flags & 0x800000 as libc::c_int != 0
               && (*request).till != 0 as libc::c_int &&
               ts_after((*(*as_reply).enc_part2).times.renew_till,
                        (*request).till) != 0 {
        return -(1765328237 as libc::c_long) as krb5_error_code
    }
    if (*context).library_options & 0x1 as libc::c_int != 0 {
        time_offset =
            ts_delta((*(*as_reply).enc_part2).times.authtime, time_now);
        retval =
            krb5_set_time_offsets(context, time_offset, 0 as libc::c_int);
        if retval != 0 { return retval }
    } else if (*request).from == 0 as libc::c_int &&
                  ts_within((*(*as_reply).enc_part2).times.starttime,
                            time_now, (*context).clockskew) == 0 {
        return -(1765328236 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "274:1"]
unsafe extern "C" fn stash_as_reply(mut context: krb5_context,
                                    mut as_reply: *mut krb5_kdc_rep,
                                    mut creds: *mut krb5_creds,
                                    mut ccache: krb5_ccache)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut packet: *mut krb5_data = 0 as *mut krb5_data;
    let mut client: krb5_principal = 0 as *mut krb5_principal_data;
    let mut server: krb5_principal = 0 as *mut krb5_principal_data;
    client = 0 as krb5_principal;
    server = 0 as krb5_principal;
    if (*creds).client.is_null() {
        retval =
            krb5_copy_principal(context,
                                (*as_reply).client as krb5_const_principal,
                                &mut client);
        if retval != 0 {
            current_block = 12998750015275230484;
        } else { current_block = 6873731126896040597; }
    } else { current_block = 6873731126896040597; }
    match current_block {
        6873731126896040597 => {
            if (*creds).server.is_null() {
                retval =
                    krb5_copy_principal(context,
                                        (*(*as_reply).enc_part2).server as
                                            krb5_const_principal,
                                        &mut server);
                if retval != 0 {
                    current_block = 12998750015275230484;
                } else { current_block = 6937071982253665452; }
            } else { current_block = 6937071982253665452; }
            match current_block {
                12998750015275230484 => { }
                _ =>
                /* fill in the credentials */
                {
                    retval =
                        krb5_copy_keyblock_contents(context,
                                                    (*(*as_reply).enc_part2).session,
                                                    &mut (*creds).keyblock); /* this is an AS_REQ, so cannot
                                           be encrypted in skey */
                    if !(retval != 0) {
                        (*creds).times = (*(*as_reply).enc_part2).times;
                        (*creds).is_skey = 0 as libc::c_int as krb5_boolean;
                        (*creds).ticket_flags =
                            (*(*as_reply).enc_part2).flags;
                        retval =
                            krb5_copy_addresses(context,
                                                (*(*as_reply).enc_part2).caddrs,
                                                &mut (*creds).addresses);
                        if !(retval != 0) {
                            (*creds).second_ticket.length =
                                0 as libc::c_int as libc::c_uint;
                            (*creds).second_ticket.data =
                                0 as *mut libc::c_char;
                            retval =
                                encode_krb5_ticket((*as_reply).ticket,
                                                   &mut packet);
                            if !(retval != 0) {
                                (*creds).ticket = *packet;
                                free(packet as *mut libc::c_void);
                                /* store it in the ccache! */
                                if !ccache.is_null() {
                                    retval =
                                        krb5_cc_store_cred(context, ccache,
                                                           creds);
                                    if retval != 0 {
                                        current_block = 12998750015275230484;
                                    } else {
                                        current_block = 2370887241019905314;
                                    }
                                } else {
                                    current_block = 2370887241019905314;
                                }
                                match current_block {
                                    12998750015275230484 => { }
                                    _ => {
                                        if (*creds).client.is_null() {
                                            (*creds).client = client
                                        }
                                        if (*creds).server.is_null() {
                                            (*creds).server = server
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
    if retval != 0 {
        if !client.is_null() { krb5_free_principal(context, client); }
        if !server.is_null() { krb5_free_principal(context, server); }
        if !(*creds).keyblock.contents.is_null() {
            memset((*creds).keyblock.contents as *mut libc::c_void,
                   0 as libc::c_int,
                   (*creds).keyblock.length as libc::c_ulong);
            free((*creds).keyblock.contents as *mut libc::c_void);
            (*creds).keyblock.contents = 0 as *mut krb5_octet;
            (*creds).keyblock.length = 0 as libc::c_int as libc::c_uint
        }
        if !(*creds).ticket.data.is_null() {
            free((*creds).ticket.data as *mut libc::c_void);
            (*creds).ticket.data = 0 as *mut libc::c_char
        }
        if !(*creds).addresses.is_null() {
            krb5_free_addresses(context, (*creds).addresses);
            (*creds).addresses = 0 as *mut *mut krb5_address
        }
    }
    return retval;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/get_in_tkt.c */
/*
 * Copyright 1990,1991, 2003, 2008 by the Massachusetts Institute of Technology.
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
/* some typedef's for the function args to make things look a bit cleaner */
#[c2rust::src_loc = "355:1"]
unsafe extern "C" fn make_preauth_list(mut context: krb5_context,
                                       mut ptypes: *mut krb5_preauthtype,
                                       mut nptypes: libc::c_int,
                                       mut ret_list:
                                           *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ptypep: *mut krb5_preauthtype = 0 as *mut krb5_preauthtype;
    let mut preauthp: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut i: libc::c_int = 0;
    if nptypes < 0 as libc::c_int {
        nptypes = 0 as libc::c_int;
        ptypep = ptypes;
        while *ptypep != 0 { ptypep = ptypep.offset(1); nptypes += 1 }
    }
    /* allocate space for a NULL to terminate the list */
    preauthp =
        malloc(((nptypes + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_pa_data>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_pa_data;
    if preauthp.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int;
    while i < nptypes {
        let ref mut fresh0 = *preauthp.offset(i as isize);
        *fresh0 =
            malloc(::std::mem::size_of::<krb5_pa_data>() as libc::c_ulong) as
                *mut krb5_pa_data;
        if (*fresh0).is_null() {
            while i >= 0 as libc::c_int {
                free(*preauthp.offset(i as isize) as *mut libc::c_void);
                i -= 1
            }
            free(preauthp as *mut libc::c_void);
            return 12 as libc::c_int
        }
        (**preauthp.offset(i as isize)).magic =
            -(1760647406 as libc::c_long) as krb5_magic;
        (**preauthp.offset(i as isize)).pa_type = *ptypes.offset(i as isize);
        (**preauthp.offset(i as isize)).length =
            0 as libc::c_int as libc::c_uint;
        let ref mut fresh1 = (**preauthp.offset(i as isize)).contents;
        *fresh1 = 0 as *mut krb5_octet;
        i += 1
    }
    /* fill in the terminating NULL */
    let ref mut fresh2 = *preauthp.offset(nptypes as isize);
    *fresh2 = 0 as *mut krb5_pa_data;
    *ret_list = preauthp;
    return 0 as libc::c_int;
}
/* Sort a pa_data sequence so that types named in the "preferred_preauth_types"
 * libdefaults entry are listed before any others. */
#[c2rust::src_loc = "402:1"]
unsafe extern "C" fn sort_krb5_padata_sequence(mut context: krb5_context,
                                               mut realm: *mut krb5_data,
                                               mut padata:
                                                   *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    let mut ret: krb5_error_code = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: libc::c_long = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut preauth_types: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut need_free_string: libc::c_int = 1 as libc::c_int;
    if padata.is_null() ||
           (*padata.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    ret =
        krb5int_libdefault_string(context, realm,
                                  b"preferred_preauth_types\x00" as *const u8
                                      as *const libc::c_char,
                                  &mut preauth_types);
    if ret != 0 as libc::c_int || preauth_types.is_null() {
        /* Try to use PKINIT first. */
        preauth_types =
            b"17, 16, 15, 14\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        need_free_string = 0 as libc::c_int
    }
    base = 0 as libc::c_int;
    p = preauth_types;
    while *p as libc::c_int != '\u{0}' as i32 {
        /* skip whitespace to find an entry */
        p =
            p.offset(strspn(p, b", \x00" as *const u8 as *const libc::c_char)
                         as isize);
        if !(*p as libc::c_int != '\u{0}' as i32) { continue ; }
        /* see if we can extract a number */
        l = strtol(p, &mut q, 10 as libc::c_int);
        if !(!q.is_null() && q > p as *mut libc::c_char) { break ; }
        /* got a valid number; search for a matchin entry */
        i = base;
        while !(*padata.offset(i as isize)).is_null() {
            /* bubble the matching entry to the front of the list */
            if (**padata.offset(i as isize)).pa_type as libc::c_long == l {
                tmp = *padata.offset(i as isize);
                j = i;
                while j > base {
                    let ref mut fresh3 = *padata.offset(j as isize);
                    *fresh3 = *padata.offset((j - 1 as libc::c_int) as isize);
                    j -= 1
                }
                let ref mut fresh4 = *padata.offset(base as isize);
                *fresh4 = tmp;
                base += 1;
                break ;
            } else { i += 1 }
        }
        p = q
    }
    if need_free_string != 0 { free(preauth_types as *mut libc::c_void); }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "473:1"]
unsafe extern "C" fn build_in_tkt_name(mut context: krb5_context,
                                       mut in_tkt_service:
                                           *const libc::c_char,
                                       mut client: krb5_const_principal,
                                       mut server_out: *mut krb5_principal)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut server: krb5_principal = 0 as krb5_principal;
    *server_out = 0 as krb5_principal;
    if !in_tkt_service.is_null() {
        ret =
            krb5_parse_name_flags(context, in_tkt_service, 0x8 as libc::c_int,
                                  &mut server);
        if ret != 0 { return ret }
        krb5_free_data_contents(context, &mut (*server).realm);
        ret =
            krb5int_copy_data_contents(context, &(*client).realm,
                                       &mut (*server).realm);
        if ret != 0 { krb5_free_principal(context, server); return ret }
    } else {
        ret =
            krb5_build_principal_ext(context,
                                     &mut server as *mut krb5_principal,
                                     (*client).realm.length,
                                     (*client).realm.data, 6 as libc::c_int,
                                     b"krbtgt\x00" as *const u8 as
                                         *const libc::c_char,
                                     (*client).realm.length,
                                     (*client).realm.data, 0 as libc::c_int);
        if ret != 0 { return ret }
    }
    /*
     * Windows Server 2008 R2 RODC insists on TGS principal names having the
     * right name type.
     */
    if (*server).length == 2 as libc::c_int &&
           data_eq_string(*(*server).data.offset(0 as libc::c_int as isize),
                          b"krbtgt\x00" as *const u8 as *const libc::c_char)
               != 0 {
        (*server).type_0 = 2 as libc::c_int
    }
    *server_out = server;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "519:1"]
pub unsafe extern "C" fn krb5_init_creds_free(mut context: krb5_context,
                                              mut ctx:
                                                  krb5_init_creds_context) {
    if ctx.is_null() { return }
    k5_response_items_free((*ctx).rctx.items);
    free((*ctx).in_tkt_service as *mut libc::c_void);
    zapfree((*ctx).gakpw.storage.data as *mut libc::c_void,
            (*ctx).gakpw.storage.length as size_t);
    k5_preauth_request_context_fini(context, ctx);
    krb5_free_error(context, (*ctx).err_reply);
    krb5_free_pa_data(context, (*ctx).err_padata);
    krb5_free_cred_contents(context, &mut (*ctx).cred);
    krb5_free_kdc_req(context, (*ctx).request);
    krb5_free_kdc_rep(context, (*ctx).reply);
    krb5_free_data(context, (*ctx).outer_request_body);
    krb5_free_data(context, (*ctx).inner_request_body);
    krb5_free_data(context, (*ctx).encoded_previous_request);
    krb5int_fast_free_state(context, (*ctx).fast_state);
    krb5_free_pa_data(context, (*ctx).optimistic_padata);
    krb5_free_pa_data(context, (*ctx).method_padata);
    krb5_free_pa_data(context, (*ctx).more_padata);
    krb5_free_data_contents(context, &mut (*ctx).salt);
    krb5_free_data_contents(context, &mut (*ctx).s2kparams);
    krb5_free_keyblock_contents(context, &mut (*ctx).as_key);
    k5_json_release((*ctx).cc_config_in as k5_json_value);
    k5_json_release((*ctx).cc_config_out as k5_json_value);
    free(ctx as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "550:1"]
pub unsafe extern "C" fn k5_init_creds_get(mut context: krb5_context,
                                           mut ctx: krb5_init_creds_context,
                                           mut use_master: *mut libc::c_int)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut request: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut reply: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut realm: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut tcp_only: libc::c_int = 0 as libc::c_int;
    let mut master: libc::c_int = *use_master;
    request.length = 0 as libc::c_int as libc::c_uint;
    request.data = 0 as *mut libc::c_char;
    reply.length = 0 as libc::c_int as libc::c_uint;
    reply.data = 0 as *mut libc::c_char;
    realm.length = 0 as libc::c_int as libc::c_uint;
    realm.data = 0 as *mut libc::c_char;
    loop  {
        code =
            krb5_init_creds_step(context, ctx, &mut reply, &mut request,
                                 &mut realm, &mut flags);
        if code as libc::c_long == -(1765328332 as libc::c_long) &&
               tcp_only == 0 {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"Request or response is too big for UDP; retrying with TCP\x00"
                                  as *const u8 as *const libc::c_char);
            }
            tcp_only = 1 as libc::c_int
        } else if code != 0 as libc::c_int ||
                      flags & 0x1 as libc::c_int as libc::c_uint == 0 {
            break ;
        }
        krb5_free_data_contents(context, &mut reply);
        master = *use_master;
        code =
            krb5_sendto_kdc(context, &mut request, &mut realm, &mut reply,
                            &mut master, tcp_only);
        if code != 0 as libc::c_int { break ; }
        krb5_free_data_contents(context, &mut request);
        krb5_free_data_contents(context, &mut realm);
    }
    krb5_free_data_contents(context, &mut request);
    krb5_free_data_contents(context, &mut reply);
    krb5_free_data_contents(context, &mut realm);
    *use_master = master;
    return code;
}
/* Heimdal API */
#[no_mangle]
#[c2rust::src_loc = "602:1"]
pub unsafe extern "C" fn krb5_init_creds_get(mut context: krb5_context,
                                             mut ctx: krb5_init_creds_context)
 -> krb5_error_code {
    let mut use_master: libc::c_int = 0 as libc::c_int;
    return k5_init_creds_get(context, ctx, &mut use_master);
}
#[no_mangle]
#[c2rust::src_loc = "611:1"]
pub unsafe extern "C" fn krb5_init_creds_get_creds(mut context: krb5_context,
                                                   mut ctx:
                                                       krb5_init_creds_context,
                                                   mut creds: *mut krb5_creds)
 -> krb5_error_code {
    if (*ctx).complete == 0 {
        return -(1765328241 as libc::c_long) as krb5_error_code
    }
    return k5_copy_creds_contents(context, &mut (*ctx).cred, creds);
}
#[no_mangle]
#[c2rust::src_loc = "622:1"]
pub unsafe extern "C" fn krb5_init_creds_get_times(mut context: krb5_context,
                                                   mut ctx:
                                                       krb5_init_creds_context,
                                                   mut times:
                                                       *mut krb5_ticket_times)
 -> krb5_error_code {
    if (*ctx).complete == 0 {
        return -(1765328241 as libc::c_long) as krb5_error_code
    }
    *times = (*ctx).cred.times;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "635:1"]
pub unsafe extern "C" fn krb5_init_creds_get_error(mut context: krb5_context,
                                                   mut ctx:
                                                       krb5_init_creds_context,
                                                   mut error:
                                                       *mut *mut krb5_error)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut ret: *mut krb5_error = 0 as *mut krb5_error;
    *error = 0 as *mut krb5_error;
    if (*ctx).err_reply.is_null() { return 0 as libc::c_int }
    ret =
        k5alloc(::std::mem::size_of::<krb5_error>() as libc::c_ulong,
                &mut code) as *mut krb5_error;
    if !(code != 0 as libc::c_int) {
        (*ret).magic = -(1760647402 as libc::c_long) as krb5_magic;
        (*ret).ctime = (*(*ctx).err_reply).ctime;
        (*ret).cusec = (*(*ctx).err_reply).cusec;
        (*ret).susec = (*(*ctx).err_reply).susec;
        (*ret).stime = (*(*ctx).err_reply).stime;
        (*ret).error = (*(*ctx).err_reply).error;
        if !(*(*ctx).err_reply).client.is_null() {
            code =
                krb5_copy_principal(context,
                                    (*(*ctx).err_reply).client as
                                        krb5_const_principal,
                                    &mut (*ret).client);
            if code != 0 as libc::c_int {
                current_block = 14737279513370991710;
            } else { current_block = 4166486009154926805; }
        } else { current_block = 4166486009154926805; }
        match current_block {
            14737279513370991710 => { }
            _ => {
                code =
                    krb5_copy_principal(context,
                                        (*(*ctx).err_reply).server as
                                            krb5_const_principal,
                                        &mut (*ret).server);
                if !(code != 0 as libc::c_int) {
                    code =
                        krb5int_copy_data_contents(context,
                                                   &mut (*(*ctx).err_reply).text,
                                                   &mut (*ret).text);
                    if !(code != 0 as libc::c_int) {
                        code =
                            krb5int_copy_data_contents(context,
                                                       &mut (*(*ctx).err_reply).e_data,
                                                       &mut (*ret).e_data);
                        if !(code != 0 as libc::c_int) { *error = ret }
                    }
                }
            }
        }
    }
    if code != 0 as libc::c_int { krb5_free_error(context, ret); }
    return code;
}
/* Return the current time, possibly using the offset from a previously
 * received preauth-required error. */
#[no_mangle]
#[c2rust::src_loc = "691:1"]
pub unsafe extern "C" fn k5_init_creds_current_time(mut context: krb5_context,
                                                    mut ctx:
                                                        krb5_init_creds_context,
                                                    mut allow_unauth:
                                                        krb5_boolean,
                                                    mut time_out:
                                                        *mut krb5_timestamp,
                                                    mut usec_out:
                                                        *mut krb5_int32)
 -> krb5_error_code {
    if (*ctx).pa_offset_state as libc::c_uint !=
           NO_OFFSET as libc::c_int as libc::c_uint &&
           (allow_unauth != 0 ||
                (*ctx).pa_offset_state as libc::c_uint ==
                    AUTH_OFFSET as libc::c_int as libc::c_uint) &&
           (*context).library_options & 0x1 as libc::c_int != 0 {
        /* Use the offset we got from a preauth-required error. */
        return k5_time_with_offset((*ctx).pa_offset, (*ctx).pa_offset_usec,
                                   time_out, usec_out)
    } else {
        /* Use the time offset from the context, or no offset. */
        return krb5_us_timeofday(context, time_out, usec_out)
    };
}
/* Set the timestamps for ctx->request based on the desired lifetimes. */
#[c2rust::src_loc = "709:1"]
unsafe extern "C" fn set_request_times(mut context: krb5_context,
                                       mut ctx: krb5_init_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut from: krb5_timestamp = 0;
    let mut now: krb5_timestamp = 0;
    let mut now_ms: krb5_int32 = 0;
    code =
        k5_init_creds_current_time(context, ctx,
                                   1 as libc::c_int as krb5_boolean, &mut now,
                                   &mut now_ms);
    if code != 0 as libc::c_int { return code }
    /* Omit request start time unless the caller explicitly asked for one. */
    from = ts_incr(now, (*ctx).start_time);
    if (*ctx).start_time != 0 as libc::c_int { (*(*ctx).request).from = from }
    (*(*ctx).request).till = ts_incr(from, (*ctx).tkt_life);
    if (*ctx).renew_life > 0 as libc::c_int {
        /* Don't ask for a smaller renewable time than the lifetime. */
        (*(*ctx).request).rtime = ts_incr(from, (*ctx).renew_life);
        if ts_after((*(*ctx).request).till, (*(*ctx).request).rtime) != 0 {
            (*(*ctx).request).rtime = (*(*ctx).request).till
        }
        (*(*ctx).request).kdc_options &= !(0x10 as libc::c_int)
    } else { (*(*ctx).request).rtime = 0 as libc::c_int }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "740:1"]
unsafe extern "C" fn read_allowed_preauth_type(mut context: krb5_context,
                                               mut ctx:
                                                   krb5_init_creds_context) {
    let mut ret: krb5_error_code = 0;
    let mut config: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut in_ccache: krb5_ccache = k5_gic_opt_get_in_ccache((*ctx).opt);
    (*ctx).allowed_preauth_type = 0 as libc::c_int;
    if in_ccache.is_null() { return }
    memset(&mut config as *mut krb5_data as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_data>() as libc::c_ulong);
    if krb5_cc_get_config(context, in_ccache,
                          (*(*ctx).request).server as krb5_const_principal,
                          b"pa_type\x00" as *const u8 as *const libc::c_char,
                          &mut config) != 0 as libc::c_int {
        return
    }
    tmp =
        k5memdup0(config.data as *const libc::c_void, config.length as size_t,
                  &mut ret) as *mut libc::c_char;
    krb5_free_data_contents(context, &mut config);
    if tmp.is_null() { return }
    (*ctx).allowed_preauth_type =
        strtol(tmp, &mut p, 10 as libc::c_int) as krb5_preauthtype;
    if p.is_null() || *p as libc::c_int != '\u{0}' as i32 {
        (*ctx).allowed_preauth_type = 0 as libc::c_int
    }
    free(tmp as *mut libc::c_void);
}
/* Return true if encrypted timestamp is disabled for realm. */
#[c2rust::src_loc = "766:1"]
unsafe extern "C" fn encts_disabled(mut profile: profile_t,
                                    mut realm: *const krb5_data)
 -> krb5_boolean {
    let mut ret: krb5_error_code = 0;
    let mut realmstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bval: libc::c_int = 0;
    realmstr =
        k5memdup0((*realm).data as *const libc::c_void,
                  (*realm).length as size_t, &mut ret) as *mut libc::c_char;
    if realmstr.is_null() { return 0 as libc::c_int as krb5_boolean }
    ret =
        profile_get_boolean(profile,
                            b"realms\x00" as *const u8 as *const libc::c_char,
                            realmstr,
                            b"disable_encrypted_timestamp\x00" as *const u8 as
                                *const libc::c_char, 0 as libc::c_int,
                            &mut bval) as krb5_error_code;
    free(realmstr as *mut libc::c_void);
    return if ret == 0 as libc::c_int { bval } else { 0 as libc::c_int } as
               krb5_boolean;
}
/* *
 * Throw away any pre-authentication realm state and begin with a
 * unauthenticated or optimistically authenticated request.  If fast_upgrade is
 * set, use FAST for this request.
 */
#[c2rust::src_loc = "788:1"]
unsafe extern "C" fn restart_init_creds_loop(mut context: krb5_context,
                                             mut ctx: krb5_init_creds_context,
                                             mut fast_upgrade: krb5_boolean)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0 as libc::c_int;
    krb5_free_pa_data(context, (*ctx).optimistic_padata);
    krb5_free_pa_data(context, (*ctx).method_padata);
    krb5_free_pa_data(context, (*ctx).more_padata);
    krb5_free_pa_data(context, (*ctx).err_padata);
    krb5_free_error(context, (*ctx).err_reply);
    (*ctx).more_padata = 0 as *mut *mut krb5_pa_data;
    (*ctx).method_padata = (*ctx).more_padata;
    (*ctx).optimistic_padata = (*ctx).method_padata;
    (*ctx).err_padata = 0 as *mut *mut krb5_pa_data;
    (*ctx).err_reply = 0 as *mut krb5_error;
    (*ctx).selected_preauth_type = 0 as libc::c_int;
    krb5int_fast_free_state(context, (*ctx).fast_state);
    (*ctx).fast_state = 0 as *mut krb5int_fast_request_state;
    code = krb5int_fast_make_state(context, &mut (*ctx).fast_state);
    if !(code != 0 as libc::c_int) {
        if fast_upgrade != 0 {
            (*(*ctx).fast_state).fast_state_flags =
                ((*(*ctx).fast_state).fast_state_flags as libc::c_long |
                     (1 as libc::c_long) << 0 as libc::c_int) as krb5_ui_4
        }
        k5_preauth_request_context_fini(context, ctx);
        k5_preauth_request_context_init(context, ctx);
        krb5_free_data(context, (*ctx).outer_request_body);
        (*ctx).outer_request_body = 0 as *mut krb5_data;
        if (*(*ctx).opt).flags & 0x40 as libc::c_int != 0 {
            code =
                make_preauth_list(context, (*(*ctx).opt).preauth_list,
                                  (*(*ctx).opt).preauth_list_length,
                                  &mut (*ctx).optimistic_padata);
            if code != 0 {
                current_block = 5104248820744160078;
            } else { current_block = 2838571290723028321; }
        } else { current_block = 2838571290723028321; }
        match current_block {
            5104248820744160078 => { }
            _ => {
                /* Never set encts_disabled back to false, so it can't be circumvented with
     * client realm referrals. */
                if encts_disabled((*context).profile,
                                  &mut (*(*(*ctx).request).client).realm) != 0
                   {
                    (*ctx).encts_disabled = 1 as libc::c_int as krb5_boolean
                }
                krb5_free_principal(context, (*(*ctx).request).server);
                (*(*ctx).request).server = 0 as krb5_principal;
                code =
                    build_in_tkt_name(context, (*ctx).in_tkt_service,
                                      (*(*ctx).request).client as
                                          krb5_const_principal,
                                      &mut (*(*ctx).request).server);
                if !(code != 0 as libc::c_int) {
                    code =
                        krb5int_fast_as_armor(context, (*ctx).fast_state,
                                              (*ctx).opt, (*ctx).request);
                    if !(code != 0 as libc::c_int) {
                        /* give the preauth plugins a chance to prep the request body */
                        k5_preauth_prepare_request(context, (*ctx).opt,
                                                   (*ctx).request);
                        code =
                            krb5int_fast_prep_req_body(context,
                                                       (*ctx).fast_state,
                                                       (*ctx).request,
                                                       &mut (*ctx).outer_request_body);
                        if !(code != 0 as libc::c_int) {
                            /* Read the allowed preauth type for this server principal from the input
     * ccache, if the application supplied one. */
                            read_allowed_preauth_type(context, ctx);
                        }
                    }
                }
            }
        }
    }
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "859:1"]
pub unsafe extern "C" fn krb5_init_creds_init(mut context: krb5_context,
                                              mut client: krb5_principal,
                                              mut prompter: krb5_prompter_fct,
                                              mut data: *mut libc::c_void,
                                              mut start_time: krb5_deltat,
                                              mut opt:
                                                  *mut krb5_get_init_creds_opt,
                                              mut pctx:
                                                  *mut krb5_init_creds_context)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut ctx: krb5_init_creds_context = 0 as *mut _krb5_init_creds_context;
    let mut tmp: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Getting initial credentials for {princ}\x00" as
                          *const u8 as *const libc::c_char, client);
    }
    ctx =
        k5alloc(::std::mem::size_of::<_krb5_init_creds_context>() as
                    libc::c_ulong, &mut code) as krb5_init_creds_context;
    if !(code != 0 as libc::c_int) {
        (*ctx).request =
            k5alloc(::std::mem::size_of::<krb5_kdc_req>() as libc::c_ulong,
                    &mut code) as *mut krb5_kdc_req;
        if !(code != 0 as libc::c_int) {
            (*ctx).info_pa_permitted = 1 as libc::c_int as krb5_boolean;
            code =
                krb5_copy_principal(context, client as krb5_const_principal,
                                    &mut (*(*ctx).request).client);
            if !(code != 0 as libc::c_int) {
                (*ctx).prompter = prompter;
                (*ctx).prompter_data = data;
                (*ctx).gak_fct =
                    Some(krb5_get_as_key_password as
                             unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_principal,
                                                  _: krb5_enctype,
                                                  _: krb5_prompter_fct,
                                                  _: *mut libc::c_void,
                                                  _: *mut krb5_data,
                                                  _: *mut krb5_data,
                                                  _: *mut krb5_keyblock,
                                                  _: *mut libc::c_void,
                                                  _: *mut k5_response_items)
                                 -> krb5_error_code);
                (*ctx).gak_data =
                    &mut (*ctx).gakpw as *mut gak_password as
                        *mut libc::c_void;
                (*ctx).start_time = start_time;
                if opt.is_null() {
                    (*ctx).opt = &mut (*ctx).opt_storage;
                    krb5_get_init_creds_opt_init((*ctx).opt);
                } else { (*ctx).opt = opt }
                code = k5_response_items_new(&mut (*ctx).rctx.items);
                if !(code != 0 as libc::c_int) {
                    /* Initialise request parameters as per krb5_get_init_creds() */
                    (*(*ctx).request).kdc_options =
                        (*context).kdc_default_options;
                    /* forwardable */
                    if (*(*ctx).opt).flags & 0x4 as libc::c_int != 0 {
                        tmp = (*(*ctx).opt).forwardable
                    } else if !(krb5int_libdefault_boolean(context,
                                                           &mut (*(*(*ctx).request).client).realm,
                                                           b"forwardable\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           &mut tmp) ==
                                    0 as libc::c_int) {
                        tmp = 0 as libc::c_int
                    }
                    if tmp != 0 {
                        (*(*ctx).request).kdc_options |=
                            0x40000000 as libc::c_int
                    }
                    /* proxiable */
                    if (*(*ctx).opt).flags & 0x8 as libc::c_int != 0 {
                        tmp = (*(*ctx).opt).proxiable
                    } else if !(krb5int_libdefault_boolean(context,
                                                           &mut (*(*(*ctx).request).client).realm,
                                                           b"proxiable\x00" as
                                                               *const u8 as
                                                               *const libc::c_char,
                                                           &mut tmp) ==
                                    0 as libc::c_int) {
                        tmp = 0 as libc::c_int
                    }
                    if tmp != 0 {
                        (*(*ctx).request).kdc_options |=
                            0x10000000 as libc::c_int
                    }
                    /* canonicalize */
                    if (*(*ctx).opt).flags & 0x200 as libc::c_int != 0 {
                        tmp = 1 as libc::c_int
                    } else if !(krb5int_libdefault_boolean(context,
                                                           &mut (*(*(*ctx).request).client).realm,
                                                           b"canonicalize\x00"
                                                               as *const u8 as
                                                               *const libc::c_char,
                                                           &mut tmp) ==
                                    0 as libc::c_int) {
                        tmp = 0 as libc::c_int
                    }
                    if tmp != 0 {
                        (*(*ctx).request).kdc_options |=
                            0x10000 as libc::c_int
                    }
                    /* allow_postdate */
                    if (*ctx).start_time > 0 as libc::c_int {
                        (*(*ctx).request).kdc_options |=
                            0x4000000 as libc::c_int |
                                0x2000000 as libc::c_int
                    }
                    /* ticket lifetime */
                    if (*(*ctx).opt).flags & 0x1 as libc::c_int != 0 {
                        (*ctx).tkt_life =
                            (*(*ctx).opt).tkt_life; /* previously hardcoded in kinit */
                        current_block = 17747245473264231573;
                    } else if krb5int_libdefault_string(context,
                                                        &mut (*(*(*ctx).request).client).realm,
                                                        b"ticket_lifetime\x00"
                                                            as *const u8 as
                                                            *const libc::c_char,
                                                        &mut str) ==
                                  0 as libc::c_int {
                        code =
                            krb5_string_to_deltat(str, &mut (*ctx).tkt_life);
                        if code != 0 as libc::c_int {
                            current_block = 2347778622597730573;
                        } else {
                            free(str as *mut libc::c_void);
                            str = 0 as *mut libc::c_char;
                            current_block = 17747245473264231573;
                        }
                    } else {
                        (*ctx).tkt_life =
                            24 as libc::c_int * 60 as libc::c_int *
                                60 as libc::c_int;
                        current_block = 17747245473264231573;
                    }
                    match current_block {
                        2347778622597730573 => { }
                        _ =>
                        /* renewable lifetime */
                        {
                            if (*(*ctx).opt).flags & 0x2 as libc::c_int != 0 {
                                (*ctx).renew_life = (*(*ctx).opt).renew_life;
                                current_block = 8835654301469918283;
                            } else if krb5int_libdefault_string(context,
                                                                &mut (*(*(*ctx).request).client).realm,
                                                                b"renew_lifetime\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                &mut str) ==
                                          0 as libc::c_int {
                                code =
                                    krb5_string_to_deltat(str,
                                                          &mut (*ctx).renew_life);
                                if code != 0 as libc::c_int {
                                    current_block = 2347778622597730573;
                                } else {
                                    free(str as *mut libc::c_void);
                                    str = 0 as *mut libc::c_char;
                                    current_block = 8835654301469918283;
                                }
                            } else {
                                (*ctx).renew_life = 0 as libc::c_int;
                                current_block = 8835654301469918283;
                            }
                            match current_block {
                                2347778622597730573 => { }
                                _ => {
                                    if (*ctx).renew_life > 0 as libc::c_int {
                                        (*(*ctx).request).kdc_options |=
                                            0x800000 as libc::c_int
                                    }
                                    /* enctypes */
                                    if (*(*ctx).opt).flags &
                                           0x10 as libc::c_int != 0 {
                                        (*(*ctx).request).ktype =
                                            k5memdup((*(*ctx).opt).etype_list
                                                         as
                                                         *const libc::c_void,
                                                     ((*(*ctx).opt).etype_list_length
                                                          as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_enctype>()
                                                                                          as
                                                                                          libc::c_ulong),
                                                     &mut code) as
                                                *mut krb5_enctype;
                                        if code != 0 as libc::c_int {
                                            current_block =
                                                2347778622597730573;
                                        } else {
                                            (*(*ctx).request).nktypes =
                                                (*(*ctx).opt).etype_list_length;
                                            current_block =
                                                5684854171168229155;
                                        }
                                    } else if krb5_get_default_in_tkt_ktypes(context,
                                                                             &mut (*(*ctx).request).ktype)
                                                  == 0 as libc::c_int {
                                        (*(*ctx).request).nktypes =
                                            k5_count_etypes((*(*ctx).request).ktype)
                                                as libc::c_int;
                                        current_block = 5684854171168229155;
                                    } else {
                                        /* there isn't any useful default here. */
                                        code =
                                            -(1765328147 as libc::c_long) as
                                                krb5_error_code;
                                        current_block = 2347778622597730573;
                                    }
                                    match current_block {
                                        2347778622597730573 => { }
                                        _ => {
                                            /*
     * Set a default enctype for optimistic preauth.  If we're not doing
     * optimistic preauth, this should ordinarily get overwritten when we
     * process the etype-info2 of the preauth-required error.
     */
                                            if (*(*ctx).request).nktypes >
                                                   0 as libc::c_int {
                                                (*ctx).etype =
                                                    *(*(*ctx).request).ktype.offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                            }
                                            /* addresses */
                                            if (*(*ctx).opt).flags &
                                                   0x20 as libc::c_int != 0 {
                                                code =
                                                    krb5_copy_addresses(context,
                                                                        (*(*ctx).opt).address_list,
                                                                        &mut (*(*ctx).request).addresses);
                                                if code != 0 as libc::c_int {
                                                    current_block =
                                                        2347778622597730573;
                                                } else {
                                                    current_block =
                                                        17167606947040001567;
                                                }
                                            } else if krb5int_libdefault_boolean(context,
                                                                                 &mut (*(*(*ctx).request).client).realm,
                                                                                 b"noaddresses\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 &mut tmp)
                                                          != 0 as libc::c_int
                                                          || tmp != 0 {
                                                (*(*ctx).request).addresses =
                                                    0 as
                                                        *mut *mut krb5_address;
                                                current_block =
                                                    17167606947040001567;
                                            } else {
                                                code =
                                                    krb5_os_localaddr(context,
                                                                      &mut (*(*ctx).request).addresses);
                                                if code != 0 as libc::c_int {
                                                    current_block =
                                                        2347778622597730573;
                                                } else {
                                                    current_block =
                                                        17167606947040001567;
                                                }
                                            }
                                            match current_block {
                                                2347778622597730573 => { }
                                                _ => {
                                                    if (*(*ctx).opt).flags &
                                                           0x80 as libc::c_int
                                                           != 0 {
                                                        code =
                                                            krb5int_copy_data_contents(context,
                                                                                       (*(*ctx).opt).salt,
                                                                                       &mut (*ctx).salt);
                                                        if code !=
                                                               0 as
                                                                   libc::c_int
                                                           {
                                                            current_block =
                                                                2347778622597730573;
                                                        } else {
                                                            (*ctx).default_salt
                                                                =
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    krb5_boolean;
                                                            current_block =
                                                                2750570471926810434;
                                                        }
                                                    } else {
                                                        (*ctx).salt =
                                                            empty_data();
                                                        (*ctx).default_salt =
                                                            1 as libc::c_int
                                                                as
                                                                krb5_boolean;
                                                        current_block =
                                                            2750570471926810434;
                                                    }
                                                    match current_block {
                                                        2347778622597730573 =>
                                                        {
                                                        }
                                                        _ =>
                                                        /* Anonymous. */
                                                        {
                                                            if (*(*ctx).opt).flags
                                                                   &
                                                                   0x400 as
                                                                       libc::c_int
                                                                   != 0 {
                                                                (*(*ctx).request).kdc_options
                                                                    |=
                                                                    0x8000 as
                                                                        libc::c_int;
                                                                /* Remap @REALM to WELLKNOWN/ANONYMOUS@REALM. */
                                                                if (*client).length
                                                                       ==
                                                                       1 as
                                                                           libc::c_int
                                                                       &&
                                                                       (*(*client).data.offset(0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   isize)).length
                                                                           ==
                                                                           0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                   {
                                                                    let mut new_client:
                                                                            krb5_principal =
                                                                        0 as
                                                                            *mut krb5_principal_data;
                                                                    code =
                                                                        krb5_build_principal_ext(context,
                                                                                                 &mut new_client
                                                                                                     as
                                                                                                     *mut krb5_principal,
                                                                                                 (*client).realm.length,
                                                                                                 (*client).realm.data,
                                                                                                 strlen(b"WELLKNOWN\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char),
                                                                                                 b"WELLKNOWN\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 strlen(b"ANONYMOUS\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char),
                                                                                                 b"ANONYMOUS\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 0
                                                                                                     as
                                                                                                     libc::c_int);
                                                                    if code !=
                                                                           0 {
                                                                        current_block
                                                                            =
                                                                            2347778622597730573;
                                                                    } else {
                                                                        krb5_free_principal(context,
                                                                                            (*(*ctx).request).client);
                                                                        (*(*ctx).request).client
                                                                            =
                                                                            new_client;
                                                                        (*(*(*ctx).request).client).type_0
                                                                            =
                                                                            11
                                                                                as
                                                                                libc::c_int;
                                                                        current_block
                                                                            =
                                                                            14865402277128115059;
                                                                    }
                                                                } else {
                                                                    current_block
                                                                        =
                                                                        14865402277128115059;
                                                                }
                                                            } else {
                                                                current_block
                                                                    =
                                                                    14865402277128115059;
                                                            }
                                                            match current_block
                                                                {
                                                                2347778622597730573
                                                                => {
                                                                }
                                                                _ => {
                                                                    /* We will also handle anonymous if the input principal is the anonymous
     * principal. */
                                                                    if krb5_principal_compare_any_realm(context,
                                                                                                        (*(*ctx).request).client
                                                                                                            as
                                                                                                            krb5_const_principal,
                                                                                                        krb5_anonymous_principal())
                                                                           !=
                                                                           0 {
                                                                        (*(*ctx).request).kdc_options
                                                                            |=
                                                                            0x8000
                                                                                as
                                                                                libc::c_int;
                                                                        (*(*(*ctx).request).client).type_0
                                                                            =
                                                                            11
                                                                                as
                                                                                libc::c_int
                                                                    }
                                                                    code =
                                                                        restart_init_creds_loop(context,
                                                                                                ctx,
                                                                                                0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    krb5_boolean);
                                                                    if !(code
                                                                             !=
                                                                             0)
                                                                       {
                                                                        *pctx
                                                                            =
                                                                            ctx;
                                                                        ctx =
                                                                            0
                                                                                as
                                                                                krb5_init_creds_context
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
        }
    }
    krb5_init_creds_free(context, ctx);
    free(str as *mut libc::c_void);
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "1068:1"]
pub unsafe extern "C" fn krb5_init_creds_set_service(mut context:
                                                         krb5_context,
                                                     mut ctx:
                                                         krb5_init_creds_context,
                                                     mut service:
                                                         *const libc::c_char)
 -> krb5_error_code {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Setting initial creds service to {str}\x00" as
                          *const u8 as *const libc::c_char, service);
    }
    s = strdup(service);
    if s.is_null() { return 12 as libc::c_int }
    free((*ctx).in_tkt_service as *mut libc::c_void);
    (*ctx).in_tkt_service = s;
    return restart_init_creds_loop(context, ctx,
                                   0 as libc::c_int as krb5_boolean);
}
#[c2rust::src_loc = "1087:1"]
unsafe extern "C" fn init_creds_validate_reply(mut context: krb5_context,
                                               mut ctx:
                                                   krb5_init_creds_context,
                                               mut reply: *mut krb5_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut error: *mut krb5_error = 0 as *mut krb5_error;
    let mut as_reply: *mut krb5_kdc_rep = 0 as *mut krb5_kdc_rep;
    krb5_free_error(context, (*ctx).err_reply);
    (*ctx).err_reply = 0 as *mut krb5_error;
    krb5_free_kdc_rep(context, (*ctx).reply);
    (*ctx).reply = 0 as *mut krb5_kdc_rep;
    if !reply.is_null() && (*reply).length != 0 &&
           *(*reply).data.offset(0 as libc::c_int as isize) as libc::c_int &
               !(0x20 as libc::c_int) ==
               30 as libc::c_int | 0x40 as libc::c_int {
        code = decode_krb5_error(reply, &mut error);
        if code != 0 as libc::c_int { return code }
        if !error.is_null() {
        } else {
            __assert_fail(b"error != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"get_in_tkt.c\x00" as *const u8 as
                              *const libc::c_char,
                          1107 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 94],
                                                    &[libc::c_char; 94]>(b"krb5_error_code init_creds_validate_reply(krb5_context, krb5_init_creds_context, krb5_data *)\x00")).as_ptr());
        }
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Received error from KDC: {kerr}\x00" as *const u8
                              as *const libc::c_char,
                          (*error).error as libc::c_long +
                              -(1765328384 as libc::c_long));
        }
        if (*error).error == 52 as libc::c_int as libc::c_uint {
            krb5_free_error(context, error);
            return -(1765328332 as libc::c_long) as krb5_error_code
        } else { (*ctx).err_reply = error; return 0 as libc::c_int }
    }
    /*
     * Check to make sure it isn't a V4 reply.
     */
    if (*reply).length != 0 as libc::c_int as libc::c_uint &&
           !(!reply.is_null() && (*reply).length != 0 &&
                 *(*reply).data.offset(0 as libc::c_int as isize) as
                     libc::c_int & !(0x20 as libc::c_int) ==
                     11 as libc::c_int | 0x40 as libc::c_int) {
        /* these are in <kerberosIV/prot.h> as well but it isn't worth including. */
        /* check here for V4 reply */
        let mut t_switch: libc::c_uint = 0;
        /* From v4 g_in_tkt.c: This used to be
           switch (pkt_msg_type(rpkt) & ~1) {
           but SCO 3.2v4 cc compiled that incorrectly.  */
        t_switch =
            *(*reply).data.offset(1 as libc::c_int as isize) as libc::c_uint;
        t_switch &= !(1 as libc::c_int) as libc::c_uint;
        if t_switch ==
               ((5 as libc::c_int) << 1 as libc::c_int) as libc::c_uint &&
               *(*reply).data.offset(0 as libc::c_int as isize) as libc::c_int
                   == 4 as libc::c_int {
            code = -(1765328165 as libc::c_long) as krb5_error_code
        } else { code = -(1765328344 as libc::c_long) as krb5_error_code }
        return code
    }
    /* It must be a KRB_AS_REP message, or an bad returned packet */
    code = decode_krb5_as_rep(reply, &mut as_reply);
    if code != 0 as libc::c_int { return code }
    if (*as_reply).msg_type != 11 as libc::c_int as krb5_msgtype {
        krb5_free_kdc_rep(context, as_reply);
        return -(1765328344 as libc::c_long) as krb5_error_code
    }
    (*ctx).reply = as_reply;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1160:1"]
unsafe extern "C" fn save_selected_preauth_type(mut context: krb5_context,
                                                mut ccache: krb5_ccache,
                                                mut ctx:
                                                    krb5_init_creds_context)
 -> krb5_error_code {
    let mut config_data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut code: krb5_error_code = 0;
    if (*ctx).selected_preauth_type == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    if asprintf(&mut tmp as *mut *mut libc::c_char,
                b"%ld\x00" as *const u8 as *const libc::c_char,
                (*ctx).selected_preauth_type as libc::c_long) <
           0 as libc::c_int {
        return 12 as libc::c_int
    }
    config_data = string2data(tmp);
    code =
        krb5_cc_set_config(context, ccache,
                           (*ctx).cred.server as krb5_const_principal,
                           b"pa_type\x00" as *const u8 as *const libc::c_char,
                           &mut config_data);
    free(tmp as *mut libc::c_void);
    return code;
}
#[c2rust::src_loc = "1179:1"]
unsafe extern "C" fn clear_cc_config_out_data(mut context: krb5_context,
                                              mut ctx:
                                                  krb5_init_creds_context)
 -> krb5_error_code {
    k5_json_release((*ctx).cc_config_out as k5_json_value);
    (*ctx).cc_config_out = 0 as k5_json_object;
    return k5_json_object_create(&mut (*ctx).cc_config_out);
}
#[c2rust::src_loc = "1187:1"]
unsafe extern "C" fn read_cc_config_in_data(mut context: krb5_context,
                                            mut ctx: krb5_init_creds_context)
 -> krb5_error_code {
    let mut val: k5_json_value = 0 as *mut libc::c_void;
    let mut config: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut code: krb5_error_code = 0;
    let mut i: libc::c_int = 0;
    let mut in_ccache: krb5_ccache = k5_gic_opt_get_in_ccache((*ctx).opt);
    k5_json_release((*ctx).cc_config_in as k5_json_value);
    (*ctx).cc_config_in = 0 as k5_json_object;
    if in_ccache.is_null() { return 0 as libc::c_int }
    memset(&mut config as *mut krb5_data as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_data>() as libc::c_ulong);
    code =
        krb5_cc_get_config(context, in_ccache,
                           (*(*ctx).request).server as krb5_const_principal,
                           b"pa_config_data\x00" as *const u8 as
                               *const libc::c_char, &mut config);
    if code != 0 { return code }
    i =
        asprintf(&mut encoded as *mut *mut libc::c_char,
                 b"%.*s\x00" as *const u8 as *const libc::c_char,
                 config.length as libc::c_int, config.data);
    krb5_free_data_contents(context, &mut config);
    if i < 0 as libc::c_int { return 12 as libc::c_int }
    code = k5_json_decode(encoded, &mut val);
    free(encoded as *mut libc::c_void);
    if code != 0 { return code }
    if k5_json_get_tid(val) != 130 as libc::c_int as libc::c_uint {
        k5_json_release(val);
        return 22 as libc::c_int
    }
    (*ctx).cc_config_in = val as k5_json_object;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1226:1"]
unsafe extern "C" fn save_cc_config_out_data(mut context: krb5_context,
                                             mut ccache: krb5_ccache,
                                             mut ctx: krb5_init_creds_context)
 -> krb5_error_code {
    let mut config: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut code: krb5_error_code = 0;
    if (*ctx).cc_config_out.is_null() ||
           k5_json_object_count((*ctx).cc_config_out) ==
               0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    code =
        k5_json_encode((*ctx).cc_config_out as k5_json_value, &mut encoded);
    if code != 0 { return code }
    config = string2data(encoded);
    code =
        krb5_cc_set_config(context, ccache,
                           (*ctx).cred.server as krb5_const_principal,
                           b"pa_config_data\x00" as *const u8 as
                               *const libc::c_char, &mut config);
    free(encoded as *mut libc::c_void);
    return code;
}
/* Add a KERB-PA-PAC-REQUEST pa-data item if the gic options require one. */
#[c2rust::src_loc = "1248:1"]
unsafe extern "C" fn maybe_add_pac_request(mut context: krb5_context,
                                           mut ctx: krb5_init_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut pac_req: krb5_pa_pac_req = krb5_pa_pac_req{include_pac: 0,};
    let mut encoded: *mut krb5_data = 0 as *mut krb5_data;
    let mut val: libc::c_int = 0;
    val = k5_gic_opt_pac_request((*ctx).opt);
    if val == -(1 as libc::c_int) { return 0 as libc::c_int }
    pac_req.include_pac = val as krb5_boolean;
    code = encode_krb5_pa_pac_req(&mut pac_req, &mut encoded);
    if code != 0 { return code }
    code =
        k5_add_pa_data_from_data(&mut (*(*ctx).request).padata,
                                 128 as libc::c_int, encoded);
    krb5_free_data(context, encoded);
    return code;
}
#[c2rust::src_loc = "1270:1"]
unsafe extern "C" fn init_creds_step_request(mut context: krb5_context,
                                             mut ctx: krb5_init_creds_context,
                                             mut out: *mut krb5_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut pa_type: krb5_preauthtype = 0;
    let mut copy: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut save: errinfo =
        {
            let mut init =
                errinfo{code: 0 as libc::c_int as libc::c_long,
                        msg: 0 as *mut libc::c_char,};
            init
        };
    let mut rcode: uint32_t =
        if (*ctx).err_reply.is_null() {
            0 as libc::c_int as libc::c_uint
        } else { (*(*ctx).err_reply).error };
    if (*ctx).loopcount >= 16 as libc::c_int as libc::c_uint {
        code = -(1765328161 as libc::c_long) as krb5_error_code
    } else {
        /* RFC 6113 requires a new nonce for the inner request on each try. */
        code = k5_generate_nonce(context, &mut (*(*ctx).request).nonce);
        if !(code != 0 as libc::c_int) {
            /* Reset the request timestamps, possibly adjusting to the KDC time. */
            code = set_request_times(context, ctx);
            if !(code != 0 as libc::c_int) {
                krb5_free_data(context, (*ctx).inner_request_body);
                (*ctx).inner_request_body = 0 as *mut krb5_data;
                code =
                    encode_krb5_kdc_req_body((*ctx).request,
                                             &mut (*ctx).inner_request_body);
                if !(code != 0) {
                    /*
     * Read cached preauth configuration data for this server principal from
     * the in_ccache, if the application supplied one, and delete any that was
     * stored by a previous (clearly failed) module.
     */
                    read_cc_config_in_data(context, ctx);
                    clear_cc_config_out_data(context, ctx);
                    (*(*ctx).request).padata = 0 as *mut *mut krb5_pa_data;
                    if !(*ctx).optimistic_padata.is_null() {
                        /* Our first attempt, using an optimistic padata list. */
                        if (*context).trace_callback.is_some() {
                            krb5int_trace(context,
                                          b"Attempting optimistic preauth\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                        }
                        code =
                            k5_preauth(context, ctx, (*ctx).optimistic_padata,
                                       1 as libc::c_int as krb5_boolean,
                                       &mut (*(*ctx).request).padata,
                                       &mut (*ctx).selected_preauth_type);
                        krb5_free_pa_data(context, (*ctx).optimistic_padata);
                        (*ctx).optimistic_padata =
                            0 as *mut *mut krb5_pa_data;
                        if code != 0 {
                            /* Make an unauthenticated request. */
                            krb5_clear_error_message(context);
                            code = 0 as libc::c_int
                        }
                    }
                    if !(*ctx).more_padata.is_null() {
                        /* Continuing after KDC_ERR_MORE_PREAUTH_DATA_REQUIRED. */
                        if (*context).trace_callback.is_some() {
                            krb5int_trace(context,
                                          b"Continuing preauth mech {patype}\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*ctx).selected_preauth_type);
                        }
                        code =
                            k5_preauth(context, ctx, (*ctx).more_padata,
                                       1 as libc::c_int as krb5_boolean,
                                       &mut (*(*ctx).request).padata,
                                       &mut pa_type)
                    } else if rcode == 24 as libc::c_int as libc::c_uint {
                        /* Report the KDC-side failure code if we can't try another mech. */
                        code =
                            -(1765328360 as libc::c_long) as krb5_error_code
                    } else if rcode != 0 &&
                                  rcode != 25 as libc::c_int as libc::c_uint {
                        /* Retrying after an error (possibly mechanism-specific), using error
         * padata to figure out what to change. */
                        if (*context).trace_callback.is_some() {
                            krb5int_trace(context,
                                          b"Recovering from KDC error {int} using preauth mech {patype}\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*(*ctx).err_reply).error,
                                          (*ctx).selected_preauth_type);
                        }
                        code =
                            k5_preauth_tryagain(context, ctx,
                                                (*ctx).selected_preauth_type,
                                                (*ctx).err_reply,
                                                (*ctx).err_padata,
                                                &mut (*(*ctx).request).padata);
                        if code != 0 {
                            krb5_clear_error_message(context);
                            code =
                                ((*(*ctx).err_reply).error as libc::c_long +
                                     -(1765328384 as libc::c_long)) as
                                    krb5_error_code
                        }
                    }
                    /* Don't continue after a keyboard interrupt. */
                    if !(code as libc::c_long ==
                             -(1765328252 as libc::c_long)) {
                        /* Don't continue if fallback is disabled. */
                        if !(code != 0 && (*ctx).fallback_disabled != 0) {
                            if code != 0 {
                                /* See if we can try a different preauth mech before giving up. */
                                k5_save_ctx_error(context, code, &mut save);
                                (*ctx).selected_preauth_type =
                                    0 as libc::c_int
                            }
                            if (*(*ctx).request).padata.is_null() &&
                                   !(*ctx).method_padata.is_null() {
                                /* Retrying after KDC_ERR_PREAUTH_REQUIRED, or trying again with a
         * different mechanism after a failure. */
                                if (*context).trace_callback.is_some() {
                                    krb5int_trace(context,
                                                  b"Preauthenticating using KDC method data\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                }
                                code =
                                    k5_preauth(context, ctx,
                                               (*ctx).method_padata,
                                               1 as libc::c_int as
                                                   krb5_boolean,
                                               &mut (*(*ctx).request).padata,
                                               &mut (*ctx).selected_preauth_type);
                                if code != 0 {
                                    if save.code !=
                                           0 as libc::c_int as libc::c_long {
                                        code =
                                            k5_restore_ctx_error(context,
                                                                 &mut save)
                                    }
                                    current_block = 15580250230409149699;
                                } else {
                                    current_block = 14001958660280927786;
                                }
                            } else { current_block = 14001958660280927786; }
                            match current_block {
                                15580250230409149699 => { }
                                _ => {
                                    if (*(*ctx).request).padata.is_null() {
                                        if (*context).trace_callback.is_some()
                                           {
                                            krb5int_trace(context,
                                                          b"Sending unauthenticated request\x00"
                                                              as *const u8 as
                                                              *const libc::c_char);
                                        }
                                    }
                                    /* Remember when we sent this request (after any preauth delay). */
                                    (*ctx).request_time =
                                        time(0 as *mut time_t) as
                                            krb5_timestamp;
                                    if !(*ctx).encoded_previous_request.is_null()
                                       {
                                        krb5_free_data(context,
                                                       (*ctx).encoded_previous_request);
                                        (*ctx).encoded_previous_request =
                                            0 as *mut krb5_data
                                    }
                                    if (*ctx).info_pa_permitted != 0 {
                                        code =
                                            k5_add_empty_pa_data(&mut (*(*ctx).request).padata,
                                                                 150 as
                                                                     libc::c_int);
                                        if code != 0 {
                                            current_block =
                                                15580250230409149699;
                                        } else {
                                            code =
                                                k5_add_empty_pa_data(&mut (*(*ctx).request).padata,
                                                                     149 as
                                                                         libc::c_int);
                                            current_block =
                                                5658374378798827547;
                                        }
                                    } else {
                                        current_block = 5658374378798827547;
                                    }
                                    match current_block {
                                        15580250230409149699 => { }
                                        _ => {
                                            if !(code != 0) {
                                                if !(*ctx).subject_cert.is_null()
                                                   {
                                                    code =
                                                        krb5int_copy_data_contents(context,
                                                                                   (*ctx).subject_cert,
                                                                                   &mut copy);
                                                    if code != 0 {
                                                        current_block =
                                                            15580250230409149699;
                                                    } else {
                                                        code =
                                                            k5_add_pa_data_from_data(&mut (*(*ctx).request).padata,
                                                                                     130
                                                                                         as
                                                                                         libc::c_int,
                                                                                     &mut copy);
                                                        krb5_free_data_contents(context,
                                                                                &mut copy);
                                                        if code != 0 {
                                                            current_block =
                                                                15580250230409149699;
                                                        } else {
                                                            current_block =
                                                                15855550149339537395;
                                                        }
                                                    }
                                                } else {
                                                    current_block =
                                                        15855550149339537395;
                                                }
                                                match current_block {
                                                    15580250230409149699 => {
                                                    }
                                                    _ => {
                                                        code =
                                                            maybe_add_pac_request(context,
                                                                                  ctx);
                                                        if !(code != 0) {
                                                            code =
                                                                krb5int_fast_prep_req(context,
                                                                                      (*ctx).fast_state,
                                                                                      (*ctx).request,
                                                                                      (*ctx).outer_request_body,
                                                                                      Some(encode_krb5_as_req
                                                                                               as
                                                                                               unsafe extern "C" fn(_:
                                                                                                                        *const krb5_kdc_req,
                                                                                                                    _:
                                                                                                                        *mut *mut krb5_data)
                                                                                                   ->
                                                                                                       krb5_error_code),
                                                                                      &mut (*ctx).encoded_previous_request);
                                                            if !(code !=
                                                                     0 as
                                                                         libc::c_int)
                                                               {
                                                                code =
                                                                    krb5int_copy_data_contents(context,
                                                                                               (*ctx).encoded_previous_request,
                                                                                               out);
                                                                (code) !=
                                                                    0 as
                                                                        libc::c_int;
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
    krb5_free_pa_data(context, (*(*ctx).request).padata);
    (*(*ctx).request).padata = 0 as *mut *mut krb5_pa_data;
    k5_clear_error(&mut save);
    return code;
}
/* Ensure that the reply enctype was among the requested enctypes. */
#[c2rust::src_loc = "1425:1"]
unsafe extern "C" fn check_reply_enctype(mut ctx: krb5_init_creds_context)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*(*ctx).request).nktypes {
        if *(*(*ctx).request).ktype.offset(i as isize) ==
               (*(*ctx).reply).enc_part.enctype {
            return 0 as libc::c_int
        }
        i += 1
    }
    return -(1765328147 as libc::c_long) as krb5_error_code;
}
/* Note the difference between the KDC's time, as reported to us in a
 * preauth-required error, and the current time. */
#[c2rust::src_loc = "1439:1"]
unsafe extern "C" fn note_req_timestamp(mut context: krb5_context,
                                        mut ctx: krb5_init_creds_context,
                                        mut kdc_time: krb5_timestamp,
                                        mut kdc_usec: krb5_int32) {
    let mut now: krb5_timestamp = 0;
    let mut usec: krb5_int32 = 0;
    if k5_time_with_offset(0 as libc::c_int, 0 as libc::c_int, &mut now,
                           &mut usec) != 0 as libc::c_int {
        return
    }
    (*ctx).pa_offset = ts_delta(kdc_time, now);
    (*ctx).pa_offset_usec = kdc_usec - usec;
    (*ctx).pa_offset_state =
        if !(*(*ctx).fast_state).armor_key.is_null() {
            AUTH_OFFSET as libc::c_int
        } else { UNAUTH_OFFSET as libc::c_int } as C2RustUnnamed;
}
/*
 * Determine whether err is a client referral to another realm, given the
 * previously requested client principal name.
 *
 * RFC 6806 Section 7 requires that KDCs return the referral realm in an error
 * type WRONG_REALM, but Microsoft Windows Server 2003 (and possibly others)
 * return the realm in a PRINCIPAL_UNKNOWN message.
 */
#[c2rust::src_loc = "1462:1"]
unsafe extern "C" fn is_referral(mut context: krb5_context,
                                 mut err: *mut krb5_error,
                                 mut client: krb5_principal) -> krb5_boolean {
    if (*err).error != 68 as libc::c_int as libc::c_uint &&
           (*err).error != 6 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as krb5_boolean
    }
    if (*err).client.is_null() { return 0 as libc::c_int as krb5_boolean }
    return (krb5_realm_compare(context, (*err).client as krb5_const_principal,
                               client as krb5_const_principal) == 0) as
               libc::c_int as krb5_boolean;
}
/* Transfer error padata to method data in ctx and sort it according to
 * configuration. */
#[c2rust::src_loc = "1475:1"]
unsafe extern "C" fn accept_method_data(mut context: krb5_context,
                                        mut ctx: krb5_init_creds_context)
 -> krb5_error_code {
    krb5_free_pa_data(context, (*ctx).method_padata);
    (*ctx).method_padata = (*ctx).err_padata;
    (*ctx).err_padata = 0 as *mut *mut krb5_pa_data;
    return sort_krb5_padata_sequence(context,
                                     &mut (*(*(*ctx).request).client).realm,
                                     (*ctx).method_padata);
}
#[c2rust::src_loc = "1485:1"]
unsafe extern "C" fn init_creds_step_reply(mut context: krb5_context,
                                           mut ctx: krb5_init_creds_context,
                                           mut in_0: *mut krb5_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut kdc_padata: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut kdc_pa_type: krb5_preauthtype = 0;
    let mut retry: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut canon_flag: libc::c_int = 0 as libc::c_int;
    let mut reply_code: uint32_t = 0;
    let mut strengthen_key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut encrypting_key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut fast_avail: krb5_boolean = 0;
    let mut out_ccache: krb5_ccache = k5_gic_opt_get_out_ccache((*ctx).opt);
    encrypting_key.length = 0 as libc::c_int as libc::c_uint;
    encrypting_key.contents = 0 as *mut krb5_octet;
    /* process previous KDC response */
    code = init_creds_validate_reply(context, ctx, in_0);
    if !(code != 0 as libc::c_int) {
        /* per referrals draft, enterprise principals imply canonicalization */
        canon_flag =
            ((*(*ctx).request).kdc_options & 0x10000 as libc::c_int !=
                 0 as libc::c_int ||
                 (*(*(*ctx).request).client).type_0 == 10 as libc::c_int) as
                libc::c_int;
        if !(*ctx).err_reply.is_null() {
            krb5_free_pa_data(context, (*ctx).more_padata);
            krb5_free_pa_data(context, (*ctx).err_padata);
            (*ctx).err_padata = 0 as *mut *mut krb5_pa_data;
            (*ctx).more_padata = (*ctx).err_padata;
            code =
                krb5int_fast_process_error(context, (*ctx).fast_state,
                                           &mut (*ctx).err_reply,
                                           &mut (*ctx).err_padata,
                                           &mut retry);
            if !(code != 0 as libc::c_int) {
                reply_code = (*(*ctx).err_reply).error;
                if (*ctx).restarted == 0 &&
                       k5_upgrade_to_fast_p(context, (*ctx).fast_state,
                                            (*ctx).err_padata) != 0 {
                    /* Retry with FAST after discovering that the KDC supports
             * it.  (FAST negotiation usually avoids this restart.) */
                    if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"Upgrading to FAST due to presence of PA_FX_FAST in reply\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                    }
                    (*ctx).restarted = 1 as libc::c_int as krb5_boolean;
                    code =
                        restart_init_creds_loop(context, ctx,
                                                1 as libc::c_int as
                                                    krb5_boolean)
                } else if (*ctx).restarted == 0 &&
                              reply_code == 24 as libc::c_int as libc::c_uint
                              &&
                              (*ctx).selected_preauth_type == 0 as libc::c_int
                 {
                    /* The KDC didn't like our informational padata (probably a pre-1.7
             * MIT krb5 KDC).  Retry without it. */
                    (*ctx).info_pa_permitted =
                        0 as libc::c_int as krb5_boolean;
                    (*ctx).restarted = 1 as libc::c_int as krb5_boolean;
                    code =
                        restart_init_creds_loop(context, ctx,
                                                0 as libc::c_int as
                                                    krb5_boolean)
                } else if reply_code == 90 as libc::c_int as libc::c_uint {
                    /* We sent an expired KDC cookie.  Start over, allowing another
             * FAST upgrade. */
                    (*ctx).restarted = 0 as libc::c_int as krb5_boolean;
                    code =
                        restart_init_creds_loop(context, ctx,
                                                0 as libc::c_int as
                                                    krb5_boolean)
                } else if (*ctx).identify_realm != 0 &&
                              (reply_code == 25 as libc::c_int as libc::c_uint
                                   ||
                                   reply_code ==
                                       23 as libc::c_int as libc::c_uint) {
                    /* The client exists in this realm; we can stop. */
                    (*ctx).complete = 1 as libc::c_int as krb5_boolean
                } else if reply_code == 25 as libc::c_int as libc::c_uint &&
                              retry != 0 {
                    note_req_timestamp(context, ctx,
                                       (*(*ctx).err_reply).stime,
                                       (*(*ctx).err_reply).susec);
                    code = accept_method_data(context, ctx)
                } else if reply_code == 24 as libc::c_int as libc::c_uint &&
                              retry != 0 {
                    note_req_timestamp(context, ctx,
                                       (*(*ctx).err_reply).stime,
                                       (*(*ctx).err_reply).susec);
                    /* Don't try again with the mechanism that failed. */
                    code =
                        k5_preauth_note_failed(ctx,
                                               (*ctx).selected_preauth_type);
                    if !(code != 0) {
                        (*ctx).selected_preauth_type = 0 as libc::c_int;
                        /* Accept or update method data if the KDC sent it. */
                        if !(*ctx).err_padata.is_null() {
                            code = accept_method_data(context, ctx)
                        }
                    }
                } else if reply_code == 91 as libc::c_int as libc::c_uint &&
                              retry != 0 {
                    (*ctx).more_padata = (*ctx).err_padata;
                    (*ctx).err_padata = 0 as *mut *mut krb5_pa_data
                } else if canon_flag != 0 &&
                              is_referral(context, (*ctx).err_reply,
                                          (*(*ctx).request).client) != 0 {
                    if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"Following referral to realm {data}\x00"
                                          as *const u8 as *const libc::c_char,
                                      &mut (*(*(*ctx).err_reply).client).realm
                                          as *mut krb5_data);
                    }
                    /* Rewrite request.client with realm from error reply */
                    krb5_free_data_contents(context,
                                            &mut (*(*(*ctx).request).client).realm);
                    code =
                        krb5int_copy_data_contents(context,
                                                   &mut (*(*(*ctx).err_reply).client).realm,
                                                   &mut (*(*(*ctx).request).client).realm);
                    if !(code != 0 as libc::c_int) {
                        /* Reset per-realm negotiation state. */
                        (*ctx).restarted = 0 as libc::c_int as krb5_boolean;
                        (*ctx).info_pa_permitted =
                            1 as libc::c_int as krb5_boolean;
                        code =
                            restart_init_creds_loop(context, ctx,
                                                    0 as libc::c_int as
                                                        krb5_boolean)
                    }
                } else if retry != 0 &&
                              (*ctx).selected_preauth_type != 0 as libc::c_int
                 {
                    code = 0 as libc::c_int
                } else {
                    /* error + no hints (or no preauth mech) = give up */
                    code =
                        (reply_code as krb5_error_code as libc::c_long +
                             -(1765328384 as libc::c_long)) as krb5_error_code
                }
            }
        } else {
            /* We have a response. Process it. */
            if !(*ctx).reply.is_null() {
            } else {
                __assert_fail(b"ctx->reply != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"get_in_tkt.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1594 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 90],
                                                        &[libc::c_char; 90]>(b"krb5_error_code init_creds_step_reply(krb5_context, krb5_init_creds_context, krb5_data *)\x00")).as_ptr());
            }
            /* Check for replies (likely forged) with unasked-for enctypes. */
            code = check_reply_enctype(ctx);
            if !(code != 0 as libc::c_int) {
                /* process any preauth data in the as_reply */
                code =
                    krb5int_fast_process_response(context, (*ctx).fast_state,
                                                  (*ctx).reply,
                                                  &mut strengthen_key);
                if !(code != 0 as libc::c_int) {
                    if (*ctx).identify_realm != 0 {
                        /* Just getting a reply means the client exists in this realm. */
                        (*ctx).complete = 1 as libc::c_int as krb5_boolean
                    } else {
                        code =
                            sort_krb5_padata_sequence(context,
                                                      &mut (*(*(*ctx).request).client).realm,
                                                      (*(*ctx).reply).padata);
                        if !(code != 0 as libc::c_int) {
                            (*ctx).etype = (*(*ctx).reply).enc_part.enctype;
                            /* Process the final reply padata.  Don't restrict the preauth types or
     * record a selected preauth type. */
                            (*ctx).allowed_preauth_type = 0 as libc::c_int;
                            code =
                                k5_preauth(context, ctx,
                                           (*(*ctx).reply).padata,
                                           0 as libc::c_int as krb5_boolean,
                                           &mut kdc_padata, &mut kdc_pa_type);
                            if !(code != 0 as libc::c_int) {
                                /*
     * If we haven't gotten a salt from another source yet, set up one
     * corresponding to the client principal returned by the KDC.  We
     * could get the same effect by passing local_as_reply->client to
     * gak_fct below, but that would put the canonicalized client name
     * in the prompt, which raises issues of needing to sanitize
     * unprintable characters.  So for now we just let it affect the
     * salt.  local_as_reply->client will be checked later on in
     * verify_as_reply.
     */
                                if (*ctx).default_salt != 0 {
                                    code =
                                        krb5_principal2salt(context,
                                                            (*(*ctx).reply).client
                                                                as
                                                                krb5_const_principal,
                                                            &mut (*ctx).salt);
                                    if (*context).trace_callback.is_some() {
                                        krb5int_trace(context,
                                                      b"Salt derived from principal: {data}\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      &mut (*ctx).salt as
                                                          *mut krb5_data);
                                    }
                                    if code != 0 as libc::c_int {
                                        current_block = 12374903851600119525;
                                    } else {
                                        current_block = 15587532755333643506;
                                    }
                                } else {
                                    current_block = 15587532755333643506;
                                }
                                match current_block {
                                    12374903851600119525 => { }
                                    _ => {
                                        code =
                                            decrypt_as_reply(context, ctx,
                                                             strengthen_key,
                                                             &mut encrypting_key);
                                        if !(code != 0) {
                                            if (*context).trace_callback.is_some()
                                               {
                                                krb5int_trace(context,
                                                              b"Decrypted AS reply; session key is: {keyblock}\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              (*(*(*ctx).reply).enc_part2).session);
                                            }
                                            code =
                                                krb5int_fast_verify_nego(context,
                                                                         (*ctx).fast_state,
                                                                         (*ctx).reply,
                                                                         (*ctx).encoded_previous_request,
                                                                         &mut encrypting_key,
                                                                         &mut fast_avail);
                                            if !(code != 0) {
                                                code =
                                                    verify_as_reply(context,
                                                                    (*ctx).request_time,
                                                                    (*ctx).request,
                                                                    (*ctx).reply);
                                                if !(code != 0 as libc::c_int)
                                                   {
                                                    code =
                                                        verify_anonymous(context,
                                                                         (*ctx).request,
                                                                         (*ctx).reply,
                                                                         &mut (*ctx).as_key);
                                                    if !(code != 0) {
                                                        code =
                                                            stash_as_reply(context,
                                                                           (*ctx).reply,
                                                                           &mut (*ctx).cred,
                                                                           0
                                                                               as
                                                                               krb5_ccache);
                                                        if !(code !=
                                                                 0 as
                                                                     libc::c_int)
                                                           {
                                                            if !out_ccache.is_null()
                                                               {
                                                                let mut config_data:
                                                                        krb5_data =
                                                                    krb5_data{magic:
                                                                                  0,
                                                                              length:
                                                                                  0,
                                                                              data:
                                                                                  0
                                                                                      as
                                                                                      *mut libc::c_char,};
                                                                code =
                                                                    krb5_cc_initialize(context,
                                                                                       out_ccache,
                                                                                       (*ctx).cred.client);
                                                                if !(code !=
                                                                         0 as
                                                                             libc::c_int)
                                                                   {
                                                                    code =
                                                                        krb5_cc_store_cred(context,
                                                                                           out_ccache,
                                                                                           &mut (*ctx).cred);
                                                                    if !(code
                                                                             !=
                                                                             0
                                                                                 as
                                                                                 libc::c_int)
                                                                       {
                                                                        if fast_avail
                                                                               !=
                                                                               0
                                                                           {
                                                                            config_data.data
                                                                                =
                                                                                b"yes\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char
                                                                                    as
                                                                                    *mut libc::c_char;
                                                                            config_data.length
                                                                                =
                                                                                strlen(config_data.data)
                                                                                    as
                                                                                    libc::c_uint;
                                                                            code
                                                                                =
                                                                                krb5_cc_set_config(context,
                                                                                                   out_ccache,
                                                                                                   (*ctx).cred.server
                                                                                                       as
                                                                                                       krb5_const_principal,
                                                                                                   b"fast_avail\x00"
                                                                                                       as
                                                                                                       *const u8
                                                                                                       as
                                                                                                       *const libc::c_char,
                                                                                                   &mut config_data);
                                                                            if code
                                                                                   !=
                                                                                   0
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                current_block
                                                                                    =
                                                                                    7737674294725511839;
                                                                            } else {
                                                                                current_block
                                                                                    =
                                                                                    2463987395154258233;
                                                                            }
                                                                        } else {
                                                                            current_block
                                                                                =
                                                                                2463987395154258233;
                                                                        }
                                                                        match current_block
                                                                            {
                                                                            7737674294725511839
                                                                            =>
                                                                            {
                                                                            }
                                                                            _
                                                                            =>
                                                                            {
                                                                                code
                                                                                    =
                                                                                    save_selected_preauth_type(context,
                                                                                                               out_ccache,
                                                                                                               ctx);
                                                                                if !(code
                                                                                         !=
                                                                                         0
                                                                                             as
                                                                                             libc::c_int)
                                                                                   {
                                                                                    code
                                                                                        =
                                                                                        save_cc_config_out_data(context,
                                                                                                                out_ccache,
                                                                                                                ctx)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                if code !=
                                                                       0 as
                                                                           libc::c_int
                                                                   {
                                                                    krb5_prepend_error_message(context,
                                                                                               code,
                                                                                               dgettext(b"mit-krb5\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char,
                                                                                                        b"Failed to store credentials\x00"
                                                                                                            as
                                                                                                            *const u8
                                                                                                            as
                                                                                                            *const libc::c_char));
                                                                }
                                                            }
                                                            k5_preauth_request_context_fini(context,
                                                                                            ctx);
                                                            /* success */
                                                            (*ctx).complete =
                                                                1 as
                                                                    libc::c_int
                                                                    as
                                                                    krb5_boolean
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
    /* Return error code, or continue with next iteration */
    krb5_free_pa_data(context, kdc_padata);
    krb5_free_keyblock(context, strengthen_key);
    krb5_free_keyblock_contents(context, &mut encrypting_key);
    return code;
}
/*
 * Do next step of credentials acquisition.
 *
 * On success returns 0 or KRB5KRB_ERR_RESPONSE_TOO_BIG if the request
 * should be sent with TCP.
 */
#[no_mangle]
#[c2rust::src_loc = "1711:1"]
pub unsafe extern "C" fn krb5_init_creds_step(mut context: krb5_context,
                                              mut ctx:
                                                  krb5_init_creds_context,
                                              mut in_0: *mut krb5_data,
                                              mut out: *mut krb5_data,
                                              mut realm: *mut krb5_data,
                                              mut flags: *mut libc::c_uint)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut code2: krb5_error_code = 0;
    *flags = 0 as libc::c_int as libc::c_uint;
    (*out).data = 0 as *mut libc::c_char;
    (*out).length = 0 as libc::c_int as libc::c_uint;
    (*realm).data = 0 as *mut libc::c_char;
    (*realm).length = 0 as libc::c_int as libc::c_uint;
    if (*ctx).complete != 0 { return 22 as libc::c_int }
    code = k5_preauth_check_context(context, ctx);
    if code != 0 { return code }
    if (*in_0).length != 0 as libc::c_int as libc::c_uint {
        code = init_creds_step_reply(context, ctx, in_0);
        if code as libc::c_long == -(1765328332 as libc::c_long) {
            code2 =
                krb5int_copy_data_contents(context,
                                           (*ctx).encoded_previous_request,
                                           out);
            if code2 != 0 as libc::c_int {
                code = code2;
                current_block = 1846501805419483333;
            } else { current_block = 5842629396148213946; }
        } else if code != 0 as libc::c_int || (*ctx).complete != 0 {
            current_block = 1846501805419483333;
        } else { current_block = 4956146061682418353; }
    } else { current_block = 4956146061682418353; }
    match current_block {
        4956146061682418353 => {
            code = init_creds_step_request(context, ctx, out);
            if code != 0 as libc::c_int {
                current_block = 1846501805419483333;
            } else {
                /* Only a new request increments the loop count, not a TCP retry */
                (*ctx).loopcount = (*ctx).loopcount.wrapping_add(1);
                current_block = 5842629396148213946;
            }
        }
        _ => { }
    }
    match current_block {
        5842629396148213946 => {
            if !(*(*ctx).request).server.is_null() {
            } else {
                __assert_fail(b"ctx->request->server != NULL\x00" as *const u8
                                  as *const libc::c_char,
                              b"get_in_tkt.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1760 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 131],
                                                        &[libc::c_char; 131]>(b"krb5_error_code krb5_init_creds_step(krb5_context, krb5_init_creds_context, krb5_data *, krb5_data *, krb5_data *, unsigned int *)\x00")).as_ptr());
            }
            code2 =
                krb5int_copy_data_contents(context,
                                           &mut (*(*(*ctx).request).server).realm,
                                           realm);
            if code2 != 0 as libc::c_int { code = code2 }
        }
        _ => { }
    }
    if code as libc::c_long == -(1765328378 as libc::c_long) {
        let mut client_name: *mut libc::c_char = 0 as *mut libc::c_char;
        /* See if we can produce a more detailed error message */
        code2 =
            krb5_unparse_name(context,
                              (*(*ctx).request).client as
                                  krb5_const_principal, &mut client_name);
        if code2 == 0 as libc::c_int {
            krb5_set_error_message(context, code,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Client \'%s\' not found in Kerberos database\x00"
                                                as *const u8 as
                                                *const libc::c_char),
                                   client_name);
            krb5_free_unparsed_name(context, client_name);
        }
    }
    *flags =
        if (*ctx).complete != 0 {
            0 as libc::c_int
        } else { 0x1 as libc::c_int } as libc::c_uint;
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "1788:1"]
pub unsafe extern "C" fn k5_get_init_creds(mut context: krb5_context,
                                           mut creds: *mut krb5_creds,
                                           mut client: krb5_principal,
                                           mut prompter: krb5_prompter_fct,
                                           mut prompter_data:
                                               *mut libc::c_void,
                                           mut start_time: krb5_deltat,
                                           mut in_tkt_service:
                                               *const libc::c_char,
                                           mut options:
                                               *mut krb5_get_init_creds_opt,
                                           mut gak_fct: get_as_key_fn,
                                           mut gak_data: *mut libc::c_void,
                                           mut use_master: *mut libc::c_int,
                                           mut as_reply:
                                               *mut *mut krb5_kdc_rep)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut ctx: krb5_init_creds_context = 0 as krb5_init_creds_context;
    code =
        krb5_init_creds_init(context, client, prompter, prompter_data,
                             start_time, options, &mut ctx);
    if !(code != 0 as libc::c_int) {
        (*ctx).gak_fct = gak_fct;
        (*ctx).gak_data = gak_data;
        if !in_tkt_service.is_null() {
            code = krb5_init_creds_set_service(context, ctx, in_tkt_service);
            if code != 0 as libc::c_int {
                current_block = 11061450337792111624;
            } else { current_block = 14523784380283086299; }
        } else { current_block = 14523784380283086299; }
        match current_block {
            11061450337792111624 => { }
            _ => {
                code = k5_init_creds_get(context, ctx, use_master);
                if !(code != 0 as libc::c_int) {
                    code = krb5_init_creds_get_creds(context, ctx, creds);
                    if !(code != 0 as libc::c_int) {
                        if !as_reply.is_null() {
                            *as_reply = (*ctx).reply;
                            (*ctx).reply = 0 as *mut krb5_kdc_rep
                        }
                    }
                }
            }
        }
    }
    krb5_init_creds_free(context, ctx);
    return code;
}
/*
 * Make AS requests with the canonicalize flag set, stopping when we get a
 * message indicating which realm the client principal is in.  Set *client_out
 * to a copy of client with the canonical realm.  If subject_cert is non-null,
 * include PA_S4U_X509_USER pa-data with the subject certificate each request.
 * (See [MS-SFU] 3.1.5.1.1.1 and 3.1.5.1.1.2.)
 */
#[no_mangle]
#[c2rust::src_loc = "1837:1"]
pub unsafe extern "C" fn k5_identify_realm(mut context: krb5_context,
                                           mut client: krb5_principal,
                                           mut subject_cert: *const krb5_data,
                                           mut client_out:
                                               *mut krb5_principal)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut opts: *mut krb5_get_init_creds_opt =
        0 as *mut krb5_get_init_creds_opt;
    let mut ctx: krb5_init_creds_context = 0 as krb5_init_creds_context;
    let mut use_master: libc::c_int = 0 as libc::c_int;
    *client_out = 0 as krb5_principal;
    ret = krb5_get_init_creds_opt_alloc(context, &mut opts);
    if !(ret != 0) {
        krb5_get_init_creds_opt_set_tkt_life(opts, 15 as libc::c_int);
        krb5_get_init_creds_opt_set_renew_life(opts, 0 as libc::c_int);
        krb5_get_init_creds_opt_set_forwardable(opts, 0 as libc::c_int);
        krb5_get_init_creds_opt_set_proxiable(opts, 0 as libc::c_int);
        krb5_get_init_creds_opt_set_canonicalize(opts, 1 as libc::c_int);
        ret =
            krb5_init_creds_init(context, client, None,
                                 0 as *mut libc::c_void, 0 as libc::c_int,
                                 opts, &mut ctx);
        if !(ret != 0) {
            (*ctx).identify_realm = 1 as libc::c_int as krb5_boolean;
            (*ctx).subject_cert = subject_cert;
            ret = k5_init_creds_get(context, ctx, &mut use_master);
            if !(ret != 0) {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"Identified realm of client principal as {data}\x00"
                                      as *const u8 as *const libc::c_char,
                                  &mut (*(*(*ctx).request).client).realm as
                                      *mut krb5_data);
                }
                ret =
                    krb5_copy_principal(context,
                                        (*(*ctx).request).client as
                                            krb5_const_principal, client_out)
            }
        }
    }
    krb5_get_init_creds_opt_free(context, opts);
    krb5_init_creds_free(context, ctx);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1877:1"]
pub unsafe extern "C" fn k5_populate_gic_opt(mut context: krb5_context,
                                             mut out:
                                                 *mut *mut krb5_get_init_creds_opt,
                                             mut options: krb5_flags,
                                             mut addrs:
                                                 *const *mut krb5_address,
                                             mut ktypes: *mut krb5_enctype,
                                             mut pre_auth_types:
                                                 *mut krb5_preauthtype,
                                             mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut starttime: krb5_timestamp = 0;
    let mut lifetime: krb5_deltat = 0;
    let mut opt: *mut krb5_get_init_creds_opt =
        0 as *mut krb5_get_init_creds_opt;
    let mut retval: krb5_error_code = 0;
    *out = 0 as *mut krb5_get_init_creds_opt;
    retval = krb5_get_init_creds_opt_alloc(context, &mut opt);
    if retval != 0 { return retval }
    if !addrs.is_null() {
        krb5_get_init_creds_opt_set_address_list(opt,
                                                 addrs as
                                                     *mut *mut krb5_address);
    }
    if !ktypes.is_null() {
        i = k5_count_etypes(ktypes) as libc::c_int;
        if i != 0 { krb5_get_init_creds_opt_set_etype_list(opt, ktypes, i); }
    }
    if !pre_auth_types.is_null() {
        i = 0 as libc::c_int;
        while *pre_auth_types.offset(i as isize) != 0 { i += 1 }
        if i != 0 {
            krb5_get_init_creds_opt_set_preauth_list(opt, pre_auth_types, i);
        }
    }
    if options & 0x40000000 as libc::c_int != 0 {
        krb5_get_init_creds_opt_set_forwardable(opt, 1 as libc::c_int);
    } else { krb5_get_init_creds_opt_set_forwardable(opt, 0 as libc::c_int); }
    if options & 0x10000000 as libc::c_int != 0 {
        krb5_get_init_creds_opt_set_proxiable(opt, 1 as libc::c_int);
    } else { krb5_get_init_creds_opt_set_proxiable(opt, 0 as libc::c_int); }
    if !creds.is_null() && (*creds).times.endtime != 0 {
        retval = krb5_timeofday(context, &mut starttime);
        if retval != 0 {
            krb5_get_init_creds_opt_free(context, opt);
            return retval
        } else {
            if (*creds).times.starttime != 0 {
                starttime = (*creds).times.starttime
            }
            lifetime = ts_delta((*creds).times.endtime, starttime);
            krb5_get_init_creds_opt_set_tkt_life(opt, lifetime);
        }
    }
    *out = opt;
    return 0 as libc::c_int;
}
