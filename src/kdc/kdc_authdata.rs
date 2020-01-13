use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
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
    #[c2rust::src_loc = "175:1"]
    pub type krb5_msgtype = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "180:1"]
    pub type krb5_cksumtype = krb5_int32;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "323:16"]
    pub struct _krb5_address {
        pub magic: krb5_magic,
        pub addrtype: krb5_addrtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< Anonymous realm */
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
    #[c2rust::src_loc = "391:16"]
    pub struct _krb5_checksum {
        pub magic: krb5_magic,
        pub checksum_type: krb5_cksumtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "391:1"]
    pub type krb5_checksum = _krb5_checksum;
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
    /* checksum type */
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
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
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
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
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
 * Compute a checksum (operates on keyblock).
 *
 * @param [in]  context         Library context
 * @param [in]  cksumtype       Checksum type (0 for mandatory type)
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]  input           Input data
 * @param [out] cksum           Generated checksum
 *
 * This function computes a checksum of type @a cksumtype over @a input, using
 * @a key if the checksum type is a keyed checksum.  If @a cksumtype is 0 and
 * @a key is non-null, the checksum type will be the mandatory-to-implement
 * checksum type for the key's encryption type.  The actual checksum key will
 * be derived from @a key and @a usage if key derivation is specified for the
 * checksum type.  The newly created @a cksum must be released by calling
 * krb5_free_checksum_contents() when it is no longer needed.
 *
 * @note This function is similar to krb5_k_make_checksum(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_verify_checksum()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "911:1"]
        pub fn krb5_c_make_checksum(context: krb5_context,
                                    cksumtype: krb5_cksumtype,
                                    key: *const krb5_keyblock,
                                    usage: krb5_keyusage,
                                    input: *const krb5_data,
                                    cksum: *mut krb5_checksum)
         -> krb5_error_code;
        /* *
 * Verify a checksum (operates on keyblock).
 *
 * @param [in]  context         Library context
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           @a key usage
 * @param [in]  data            Data to be used to compute a new checksum
 *                              using @a key to compare @a cksum against
 * @param [in]  cksum           Checksum to be verified
 * @param [out] valid           Non-zero for success, zero for failure
 *
 * This function verifies that @a cksum is a valid checksum for @a data.  If
 * the checksum type of @a cksum is a keyed checksum, @a key is used to verify
 * the checksum.  If the checksum type in @a cksum is 0 and @a key is not NULL,
 * the mandatory checksum type for @a key will be used.  The actual checksum
 * key will be derived from @a key and @a usage if key derivation is specified
 * for the checksum type.
 *
 * @note This function is similar to krb5_k_verify_checksum(), but operates
 * on keyblock @a key.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "939:1"]
        pub fn krb5_c_verify_checksum(context: krb5_context,
                                      key: *const krb5_keyblock,
                                      usage: krb5_keyusage,
                                      data: *const krb5_data,
                                      cksum: *const krb5_checksum,
                                      valid: *mut krb5_boolean)
         -> krb5_error_code;
        /* *
 * Test whether a checksum type is keyed.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is a keyed checksum type, @c FALSE otherwise.
 */
        #[no_mangle]
        #[c2rust::src_loc = "1080:1"]
        pub fn krb5_c_is_keyed_cksum(ctype: krb5_cksumtype) -> krb5_boolean;
        /* *
 * Copy an authorization data list.
 *
 * @param [in]  context         Library context
 * @param [in]  in_authdat      List of @a krb5_authdata structures
 * @param [out] out             New array of @a krb5_authdata structures
 *
 * This function creates a new authorization data list containing a copy of @a
 * in_authdat, which must be null-terminated.  Use krb5_free_authdata() to free
 * @a out when it is no longer needed.
 *
 * @note The last array entry in @a in_authdat must be a NULL pointer.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3863:1"]
        pub fn krb5_copy_authdata(context: krb5_context,
                                  in_authdat: *const *mut krb5_authdata,
                                  out: *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        /* *
 * Find authorization data elements.
 *
 * @param [in]  context         Library context
 * @param [in]  ticket_authdata Authorization data list from ticket
 * @param [in]  ap_req_authdata Authorization data list from AP request
 * @param [in]  ad_type         Authorization data type to find
 * @param [out] results         List of matching entries
 *
 * This function searches @a ticket_authdata and @a ap_req_authdata for
 * elements of type @a ad_type.  Either input list may be NULL, in which case
 * it will not be searched; otherwise, the input lists must be terminated by
 * NULL entries.  This function will search inside AD-IF-RELEVANT containers if
 * found in either list.  Use krb5_free_authdata() to free @a results when it
 * is no longer needed.
 *
 * @version New in 1.10
 */
        #[no_mangle]
        #[c2rust::src_loc = "3885:1"]
        pub fn krb5_find_authdata(context: krb5_context,
                                  ticket_authdata: *const *mut krb5_authdata,
                                  ap_req_authdata: *const *mut krb5_authdata,
                                  ad_type: krb5_authdatatype,
                                  results: *mut *mut *mut krb5_authdata)
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
 * Free the storage assigned to array of authentication data.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of authentication data to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4633:1"]
        pub fn krb5_free_authdata(context: krb5_context,
                                  val: *mut *mut krb5_authdata);
        /* *
 * Free the contents of a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4699:1"]
        pub fn krb5_free_checksum_contents(context: krb5_context,
                                           val: *mut krb5_checksum);
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
        #[no_mangle]
        #[c2rust::src_loc = "8077:1"]
        pub fn krb5_encode_authdata_container(context: krb5_context,
                                              type_0: krb5_authdatatype,
                                              authdata:
                                                  *const *mut krb5_authdata,
                                              container:
                                                  *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
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
    #[c2rust::src_loc = "856:1"]
    pub type krb5_cammac = _krb5_cammac;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "856:16"]
    pub struct _krb5_cammac {
        pub elements: *mut *mut krb5_authdata,
        pub kdc_verifier: *mut krb5_verifier_mac,
        pub svc_verifier: *mut krb5_verifier_mac,
        pub other_verifiers: *mut *mut krb5_verifier_mac,
    }
    #[c2rust::src_loc = "843:1"]
    pub type krb5_verifier_mac = _krb5_verifier_mac;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "843:16"]
    pub struct _krb5_verifier_mac {
        pub princ: krb5_principal,
        pub kvno: krb5_kvno,
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "827:1"]
    pub type krb5_ad_signedpath = _krb5_ad_signedpath;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "827:16"]
    pub struct _krb5_ad_signedpath {
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
        pub delegated: *mut krb5_principal,
        pub method_data: *mut *mut krb5_pa_data,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "819:16"]
    pub struct _krb5_ad_signedpath_data {
        pub client: krb5_principal,
        pub authtime: krb5_timestamp,
        pub delegated: *mut krb5_principal,
        pub method_data: *mut *mut krb5_pa_data,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "819:1"]
    pub type krb5_ad_signedpath_data = _krb5_ad_signedpath_data;
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
    #[c2rust::src_loc = "2274:1"]
    pub unsafe extern "C" fn alloc_data(mut data: *mut krb5_data,
                                        mut len: libc::c_uint)
     -> krb5_error_code {
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
        let mut ptr: *mut libc::c_char =
            calloc(if len > 0 as libc::c_int as libc::c_uint {
                       len
                   } else { 1 as libc::c_int as libc::c_uint } as
                       libc::c_ulong, 1 as libc::c_int as libc::c_ulong) as
                *mut libc::c_char;
        if ptr.is_null() { return 12 as libc::c_int }
        (*data).magic = -(1760647422 as libc::c_long) as krb5_magic;
        (*data).data = ptr;
        (*data).length = len;
        return 0 as libc::c_int;
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_authdata, krb5_principal, krb5_kvno,
                        krb5_checksum, krb5_pa_data, krb5_timestamp,
                        krb5_context, krb5_data, krb5_error_code,
                        krb5_authdatatype};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::plugin_h::krb5_plugin_initvt_fn;
    use super::stdlib_h::calloc;
    use super::stddef_h::size_t;
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
        #[c2rust::src_loc = "952:1"]
        pub fn krb5_free_ad_signedpath(_: krb5_context,
                                       _: *mut krb5_ad_signedpath);
        #[no_mangle]
        #[c2rust::src_loc = "963:1"]
        pub fn k5_free_cammac(context: krb5_context, val: *mut krb5_cammac);
        #[no_mangle]
        #[c2rust::src_loc = "998:1"]
        pub fn k5_free_data_ptr_list(list: *mut *mut krb5_data);
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
        #[no_mangle]
        #[c2rust::src_loc = "1517:1"]
        pub fn encode_krb5_ad_signedpath(_: *const krb5_ad_signedpath,
                                         _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1520:1"]
        pub fn encode_krb5_ad_signedpath_data(_:
                                                  *const krb5_ad_signedpath_data,
                                              _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1541:1"]
        pub fn encode_utf8_strings(ut8fstrings: *const *mut krb5_data,
                                   _: *mut *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1648:1"]
        pub fn decode_krb5_authdata(output: *const krb5_data,
                                    rep: *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1699:1"]
        pub fn decode_krb5_ad_signedpath(_: *const krb5_data,
                                         _: *mut *mut krb5_ad_signedpath)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1723:1"]
        pub fn decode_krb5_cammac(_: *const krb5_data,
                                  _: *mut *mut krb5_cammac)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "2389:1"]
        pub fn krb5int_get_authdata_containee_types(context: krb5_context,
                                                    container:
                                                        *const krb5_authdata,
                                                    nad_types:
                                                        *mut libc::c_uint,
                                                    ad_types:
                                                        *mut *mut krb5_authdatatype)
         -> krb5_error_code;
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
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:27"]
pub mod plugin_h {
    #[c2rust::src_loc = "34:1"]
    pub type krb5_plugin_vtable = *mut krb5_plugin_vtable_st;
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
    use super::krb5_h::{krb5_error_code, krb5_context};
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/kdcauthdata_plugin.h:36"]
pub mod kdcauthdata_plugin_h {
    /* Optional: module initialization function.  If this function returns an
 * error, the KDC will log the failure and ignore the module. */
    #[c2rust::src_loc = "86:1"]
    pub type krb5_kdcauthdata_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut krb5_kdcauthdata_moddata)
                   -> krb5_error_code>;
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2014 by the Massachusetts Institute of Technology.
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
 * This interface is not yet public and may change without honoring the
 * pluggable interface version system.  The header file is deliberately not
 * installed by "make install".
 *
 * This interface references kdb.h, which is not stable.  A module which
 * references fields of krb5_db_entry, invokes libkdb5 functions, or uses the
 * flags argument may not be future-proof even if this interface becomes
 * public.
 *
 * Expecting modules to hand-modify enc_tkt_reply->authorization_data is
 * cumbersome for the module, and doesn't work if the module uses a different
 * allocator from the krb5 tree.  A libkrb5 API to add an authdata value to a
 * list would improve this situation.
 */
    /*
 * Declarations for kdcauthdata plugin module implementors.
 *
 * The kdcauthdata interface has a single supported major version, which is 1.
 * Major version 1 has a current minor version of 1.  kdcauthdata modules
 * should define a function named kdcauthdata_<modulename>_initvt, matching the
 * signature:
 *
 *   krb5_error_code
 *   kdcauthdata_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                              krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for the interface and maj_ver:
 *     maj_ver == 1: Cast to krb5_kdcauthdata_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* Abstract type for module data. */
    #[c2rust::src_loc = "82:1"]
    pub type krb5_kdcauthdata_moddata = *mut krb5_kdcauthdata_moddata_st;
    /* Optional: module cleanup function. */
    #[c2rust::src_loc = "91:1"]
    pub type krb5_kdcauthdata_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_kdcauthdata_moddata) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "119:16"]
    pub struct krb5_kdcauthdata_vtable_st {
        pub name: *const libc::c_char,
        pub init: krb5_kdcauthdata_init_fn,
        pub fini: krb5_kdcauthdata_fini_fn,
        pub handle: krb5_kdcauthdata_handle_fn,
    }
    /*
 * Mandatory: authorization data handling function.
 *
 * All values should be considered input-only, except that the method can
 * modify enc_tkt_reply->authorization_data to add authdata values.  This field
 * is a null-terminated list of allocated pointers; the plugin method must
 * reallocate it to make room for any added authdata values.
 *
 * If this function returns an error, the AS or TGS request will be rejected.
 */
    #[c2rust::src_loc = "105:1"]
    pub type krb5_kdcauthdata_handle_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_kdcauthdata_moddata,
                                    _: libc::c_uint, _: *mut krb5_db_entry,
                                    _: *mut krb5_db_entry,
                                    _: *mut krb5_db_entry,
                                    _: *mut krb5_keyblock,
                                    _: *mut krb5_keyblock,
                                    _: *mut krb5_keyblock, _: *mut krb5_data,
                                    _: *mut krb5_kdc_req,
                                    _: krb5_const_principal,
                                    _: *mut krb5_enc_tkt_part,
                                    _: *mut krb5_enc_tkt_part)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_keyblock,
                        krb5_data, krb5_kdc_req, krb5_const_principal,
                        krb5_enc_tkt_part};
    use super::kdb_h::krb5_db_entry;
    extern "C" {
        #[c2rust::src_loc = "82:16"]
        pub type krb5_kdcauthdata_moddata_st;
    }
    /* KRB5_KDCAUTHDATA_PLUGIN_H */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:28"]
pub mod kdb_h {
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
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
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
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
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    use super::krb5_h::{krb5_magic, krb5_ui_2, krb5_ui_4, krb5_flags,
                        krb5_deltat, krb5_timestamp, krb5_kvno, krb5_int16,
                        krb5_octet, krb5_principal, krb5_data, krb5_context,
                        krb5_int32, krb5_error_code, krb5_keyblock,
                        krb5_principal_data, krb5_const_principal,
                        krb5_authdata};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn krb5_dbe_find_enctype(kcontext: krb5_context,
                                     dbentp: *mut krb5_db_entry,
                                     ktype: krb5_int32, stype: krb5_int32,
                                     kvno: krb5_int32,
                                     kdatap: *mut *mut krb5_key_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "446:1"]
        pub fn krb5_dbe_decrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         key_data: *const krb5_key_data,
                                         dbkey: *mut krb5_keyblock,
                                         keysalt: *mut krb5_keysalt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "659:1"]
        pub fn krb5_db_sign_authdata(kcontext: krb5_context,
                                     flags: libc::c_uint,
                                     client_princ: krb5_const_principal,
                                     server_princ: krb5_const_principal,
                                     client: *mut krb5_db_entry,
                                     server: *mut krb5_db_entry,
                                     header_server: *mut krb5_db_entry,
                                     local_tgt: *mut krb5_db_entry,
                                     client_key: *mut krb5_keyblock,
                                     server_key: *mut krb5_keyblock,
                                     header_key: *mut krb5_keyblock,
                                     local_tgt_key: *mut krb5_keyblock,
                                     session_key: *mut krb5_keyblock,
                                     authtime: krb5_timestamp,
                                     tgt_auth_data: *mut *mut krb5_authdata,
                                     ad_info: *mut libc::c_void,
                                     auth_indicators:
                                         *mut *mut *mut krb5_data,
                                     signed_auth_data:
                                         *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
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
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
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
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/kdc_util.h:28"]
pub mod kdc_util_h {
    use super::krb5_h::{krb5_principal_data, krb5_const_principal,
                        krb5_boolean, krb5_context, krb5_authdata, krb5_data,
                        krb5_error_code, krb5_enc_tkt_part, krb5_keyblock};
    use super::k5_int_h::{_krb5_context, krb5_cammac};
    use super::kdb_h::krb5_db_entry;
    use super::com_err_h::errcode_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "43:1"]
        pub fn is_cross_tgs_principal(_: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "123:1"]
        pub fn authind_extract(context: krb5_context,
                               authdata: *mut *mut krb5_authdata,
                               indicators: *mut *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "128:1"]
        pub fn cammac_create(context: krb5_context,
                             enc_tkt_reply: *mut krb5_enc_tkt_part,
                             server_key: *mut krb5_keyblock,
                             tgt: *mut krb5_db_entry,
                             tgt_key: *mut krb5_keyblock,
                             contents: *mut *mut krb5_authdata,
                             cammac_out: *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn cammac_check_kdcver(context: krb5_context,
                                   cammac: *mut krb5_cammac,
                                   enc_tkt: *mut krb5_enc_tkt_part,
                                   tgt: *mut krb5_db_entry,
                                   tgt_key: *mut krb5_keyblock)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "160:1"]
        pub fn kdc_err(call_context: krb5_context, code: errcode_t,
                       fmt: *const libc::c_char, _: ...);
    }
    /* __KRB5_KDC_UTIL__ */
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_keyusage, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_checksum, krb5_checksum, _krb5_enc_data,
                       krb5_enc_data, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_transited,
                       krb5_transited, _krb5_enc_tkt_part, krb5_enc_tkt_part,
                       _krb5_ticket, krb5_ticket, _krb5_pa_data, krb5_pa_data,
                       _krb5_kdc_req, krb5_kdc_req, _profile_t,
                       krb5_c_decrypt, krb5_c_make_checksum,
                       krb5_c_verify_checksum, krb5_c_is_keyed_cksum,
                       krb5_copy_authdata, krb5_find_authdata,
                       krb5_free_principal, krb5_free_authdata,
                       krb5_free_checksum_contents,
                       krb5_free_keyblock_contents, krb5_free_data,
                       krb5_encode_authdata_container};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_cammac, _krb5_cammac,
                         krb5_verifier_mac, _krb5_verifier_mac,
                         krb5_ad_signedpath, _krb5_ad_signedpath,
                         _krb5_ad_signedpath_data, krb5_ad_signedpath_data,
                         make_data, alloc_data, k5calloc, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_free_ad_signedpath,
                         k5_free_cammac, k5_free_data_ptr_list,
                         k5_plugin_load_all, k5_plugin_free_modules,
                         encode_krb5_ad_signedpath,
                         encode_krb5_ad_signedpath_data, encode_utf8_strings,
                         decode_krb5_authdata, decode_krb5_ad_signedpath,
                         decode_krb5_cammac, krb5_free_pa_data,
                         krb5int_get_authdata_containee_types};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::errcode_t;
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_initvt_fn,
                         krb5_plugin_vtable_st};
pub use self::kdcauthdata_plugin_h::{krb5_kdcauthdata_init_fn,
                                     krb5_kdcauthdata_moddata,
                                     krb5_kdcauthdata_fini_fn,
                                     krb5_kdcauthdata_vtable_st,
                                     krb5_kdcauthdata_handle_fn,
                                     krb5_kdcauthdata_moddata_st};
pub use self::kdb_h::{krb5_db_entry, _krb5_db_entry_new, krb5_key_data,
                      _krb5_key_data, krb5_tl_data, _krb5_tl_data,
                      krb5_keysalt, _krb5_keysalt, krb5_dbe_find_enctype,
                      krb5_dbe_decrypt_key_data, krb5_db_sign_authdata};
use self::libintl_h::dgettext;
use self::stdlib_h::{free, realloc, calloc};
use self::string_h::{memset, memcpy};
use self::assert_h::__assert_fail;
use self::kdc_util_h::{is_cross_tgs_principal, authind_extract, cammac_create,
                       cammac_check_kdcver, kdc_err};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/kdc_authdata.c - Authorization data routines for the KDC */
/*
 * Copyright (C) 2007 Apple Inc.  All Rights Reserved.
 * Copyright (C) 2008, 2009 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "38:1"]
pub type kdcauthdata_handle = kdcauthdata_handle_st;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "38:16"]
pub struct kdcauthdata_handle_st {
    pub vt: krb5_kdcauthdata_vtable_st,
    pub data: krb5_kdcauthdata_moddata,
}
#[c2rust::src_loc = "43:28"]
static mut authdata_modules: *mut kdcauthdata_handle =
    0 as *const kdcauthdata_handle as *mut kdcauthdata_handle;
#[c2rust::src_loc = "44:15"]
static mut n_authdata_modules: size_t = 0;
/* Load authdata plugin modules. */
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn load_authdata_plugins(mut context: krb5_context)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut modules: *mut krb5_plugin_initvt_fn =
        0 as *mut krb5_plugin_initvt_fn;
    let mut mod_0: *mut krb5_plugin_initvt_fn =
        0 as *mut krb5_plugin_initvt_fn;
    let mut list: *mut kdcauthdata_handle = 0 as *mut kdcauthdata_handle;
    let mut h: *mut kdcauthdata_handle = 0 as *mut kdcauthdata_handle;
    let mut count: size_t = 0;
    ret = k5_plugin_load_all(context, 9 as libc::c_int, &mut modules);
    if ret != 0 { return ret }
    /* Allocate a large enough list of handles. */
    count = 0 as libc::c_int as size_t;
    while (*modules.offset(count as isize)).is_some() {
        count = count.wrapping_add(1)
    }
    list =
        calloc(count.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<kdcauthdata_handle>() as libc::c_ulong)
            as *mut kdcauthdata_handle;
    if list.is_null() {
        k5_plugin_free_modules(context, modules);
        return 12 as libc::c_int
    }
    /* Initialize each module's vtable and module data. */
    count = 0 as libc::c_int as size_t;
    let mut current_block_18: u64;
    mod_0 = modules;
    while (*mod_0).is_some() {
        h = &mut *list.offset(count as isize) as *mut kdcauthdata_handle;
        memset(h as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<kdcauthdata_handle>() as libc::c_ulong);
        ret =
            (*mod_0).expect("non-null function pointer")(context,
                                                         1 as libc::c_int,
                                                         1 as libc::c_int,
                                                         &mut (*h).vt as
                                                             *mut krb5_kdcauthdata_vtable_st
                                                             as
                                                             krb5_plugin_vtable);
        if !(ret != 0) {
            if (*h).vt.init.is_some() {
                ret =
                    (*h).vt.init.expect("non-null function pointer")(context,
                                                                     &mut (*h).data);
                if ret != 0 {
                    kdc_err(context, ret as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while loading authdata module %s\x00"
                                         as *const u8 as *const libc::c_char),
                            (*h).vt.name);
                    current_block_18 = 12800627514080957624;
                } else { current_block_18 = 14401909646449704462; }
            } else { current_block_18 = 14401909646449704462; }
            match current_block_18 {
                12800627514080957624 => { }
                _ => { count = count.wrapping_add(1) }
            }
        }
        /* Version mismatch, keep going. */
        mod_0 = mod_0.offset(1)
    }
    authdata_modules = list;
    n_authdata_modules = count;
    k5_plugin_free_modules(context, modules);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn unload_authdata_plugins(mut context: krb5_context)
 -> krb5_error_code {
    let mut h: *mut kdcauthdata_handle = 0 as *mut kdcauthdata_handle;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n_authdata_modules {
        h =
            &mut *authdata_modules.offset(i as isize) as
                *mut kdcauthdata_handle;
        if (*h).vt.fini.is_some() {
            (*h).vt.fini.expect("non-null function pointer")(context,
                                                             (*h).data);
        }
        i = i.wrapping_add(1)
    }
    free(authdata_modules as *mut libc::c_void);
    authdata_modules = 0 as *mut kdcauthdata_handle;
    return 0 as libc::c_int;
}
/* Return true if authdata should be filtered when copying from untrusted
 * authdata.  If desired_type is non-zero, look only for that type. */
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn is_kdc_issued_authdatum(mut context: krb5_context,
                                             mut authdata: *mut krb5_authdata,
                                             mut desired_type:
                                                 krb5_authdatatype)
 -> krb5_boolean {
    let mut current_block: u64;
    let mut result: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut ad_type: krb5_authdatatype = 0;
    let mut i: libc::c_uint = 0;
    let mut count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ad_types: *mut krb5_authdatatype = 0 as *mut krb5_authdatatype;
    let mut containee_types: *mut krb5_authdatatype =
        0 as *mut krb5_authdatatype;
    if (*authdata).ad_type == 1 as libc::c_int {
        if krb5int_get_authdata_containee_types(context, authdata, &mut count,
                                                &mut containee_types) !=
               0 as libc::c_int {
            current_block = 872638551271727672;
        } else {
            ad_types = containee_types;
            current_block = 3276175668257526147;
        }
    } else {
        ad_type = (*authdata).ad_type;
        count = 1 as libc::c_int as libc::c_uint;
        ad_types = &mut ad_type;
        current_block = 3276175668257526147;
    }
    match current_block {
        3276175668257526147 => {
            i = 0 as libc::c_int as libc::c_uint;
            while i < count {
                match *ad_types.offset(i as isize) {
                    512 | 4 | 128 | 96 | 97 => {
                        result =
                            if desired_type != 0 {
                                (desired_type == *ad_types.offset(i as isize))
                                    as libc::c_int
                            } else { 1 as libc::c_int } as krb5_boolean
                    }
                    _ => { result = 0 as libc::c_int as krb5_boolean }
                }
                if result != 0 { break ; }
                i = i.wrapping_add(1)
            }
        }
        _ => { }
    }
    free(containee_types as *mut libc::c_void);
    return result;
}
/* Return true if authdata contains any elements which should only come from
 * the KDC.  If desired_type is non-zero, look only for that type. */
#[c2rust::src_loc = "154:1"]
unsafe extern "C" fn has_kdc_issued_authdata(mut context: krb5_context,
                                             mut authdata:
                                                 *mut *mut krb5_authdata,
                                             mut desired_type:
                                                 krb5_authdatatype)
 -> krb5_boolean {
    let mut i: libc::c_int = 0;
    if authdata.is_null() { return 0 as libc::c_int as krb5_boolean }
    i = 0 as libc::c_int;
    while !(*authdata.offset(i as isize)).is_null() {
        if is_kdc_issued_authdatum(context, *authdata.offset(i as isize),
                                   desired_type) != 0 {
            return 1 as libc::c_int as krb5_boolean
        }
        i += 1
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* Return true if authdata contains any mandatory-for-KDC elements. */
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn has_mandatory_for_kdc_authdata(mut context: krb5_context,
                                                    mut authdata:
                                                        *mut *mut krb5_authdata)
 -> krb5_boolean {
    let mut i: libc::c_int = 0;
    if authdata.is_null() { return 0 as libc::c_int as krb5_boolean }
    i = 0 as libc::c_int;
    while !(*authdata.offset(i as isize)).is_null() {
        if (**authdata.offset(i as isize)).ad_type == 8 as libc::c_int {
            return 1 as libc::c_int as krb5_boolean
        }
        i += 1
    }
    return 0 as libc::c_int as krb5_boolean;
}
/*
 * Add the elements of in_authdata to out_authdata.  If copy is false,
 * in_authdata is invalid on successful return.  If ignore_kdc_issued is true,
 * KDC-issued authdata is not copied.
 */
#[c2rust::src_loc = "189:1"]
unsafe extern "C" fn merge_authdata(mut context: krb5_context,
                                    mut in_authdata: *mut *mut krb5_authdata,
                                    mut out_authdata:
                                        *mut *mut *mut krb5_authdata,
                                    mut copy: krb5_boolean,
                                    mut ignore_kdc_issued: krb5_boolean)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut nadata: size_t = 0 as libc::c_int as size_t;
    let mut in_copy: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut authdata: *mut *mut krb5_authdata = *out_authdata;
    if in_authdata.is_null() ||
           (*in_authdata.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    if !authdata.is_null() {
        nadata = 0 as libc::c_int as size_t;
        while !(*authdata.offset(nadata as isize)).is_null() {
            nadata = nadata.wrapping_add(1)
        }
    }
    i = 0 as libc::c_int as size_t;
    while !(*in_authdata.offset(i as isize)).is_null() {
        i = i.wrapping_add(1)
    }
    if copy != 0 {
        ret = krb5_copy_authdata(context, in_authdata, &mut in_copy);
        if ret != 0 { return ret }
        in_authdata = in_copy
    }
    authdata =
        realloc(authdata as *mut libc::c_void,
                nadata.wrapping_add(i).wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_authdata>()
                                                                                        as
                                                                                        libc::c_ulong))
            as *mut *mut krb5_authdata;
    if authdata.is_null() {
        krb5_free_authdata(context, in_copy);
        return 12 as libc::c_int
    }
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    while !(*in_authdata.offset(i as isize)).is_null() {
        if ignore_kdc_issued != 0 &&
               is_kdc_issued_authdatum(context,
                                       *in_authdata.offset(i as isize),
                                       0 as libc::c_int) != 0 {
            free((**in_authdata.offset(i as isize)).contents as
                     *mut libc::c_void);
            free(*in_authdata.offset(i as isize) as *mut libc::c_void);
        } else {
            let fresh0 = j;
            j = j.wrapping_add(1);
            let ref mut fresh1 =
                *authdata.offset(nadata.wrapping_add(fresh0) as isize);
            *fresh1 = *in_authdata.offset(i as isize)
        }
        i = i.wrapping_add(1)
    }
    let ref mut fresh2 = *authdata.offset(nadata.wrapping_add(j) as isize);
    *fresh2 = 0 as *mut krb5_authdata;
    free(in_authdata as *mut libc::c_void);
    if (*authdata.offset(0 as libc::c_int as isize)).is_null() {
        free(authdata as *mut libc::c_void);
        authdata = 0 as *mut *mut krb5_authdata
    }
    *out_authdata = authdata;
    return 0 as libc::c_int;
}
/* Copy TGS-REQ authorization data into the ticket authdata. */
#[c2rust::src_loc = "247:1"]
unsafe extern "C" fn copy_request_authdata(mut context: krb5_context,
                                           mut client_key: *mut krb5_keyblock,
                                           mut req: *mut krb5_kdc_req,
                                           mut enc_tkt_req:
                                               *mut krb5_enc_tkt_part,
                                           mut tkt_authdata:
                                               *mut *mut *mut krb5_authdata)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut plaintext: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    if !enc_tkt_req.is_null() {
    } else {
        __assert_fail(b"enc_tkt_req != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdc_authdata.c\x00" as *const u8 as
                          *const libc::c_char,
                      255 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 125],
                                                &[libc::c_char; 125]>(b"krb5_error_code copy_request_authdata(krb5_context, krb5_keyblock *, krb5_kdc_req *, krb5_enc_tkt_part *, krb5_authdata ***)\x00")).as_ptr());
    }
    ret =
        alloc_data(&mut plaintext,
                   (*req).authorization_data.ciphertext.length);
    if ret != 0 { return ret }
    /*
     * RFC 4120 requires authdata in the TGS body to be encrypted in the subkey
     * with usage 5 if a subkey is present, and in the TGS session key with key
     * usage 4 if it is not.  Prior to krb5 1.7, we got this wrong, always
     * decrypting the authorization data with the TGS session key and usage 4.
     * For the sake of conservatism, try the decryption the old way (wrong if
     * client_key is a subkey) first, and then try again the right way (in the
     * case where client_key is a subkey) if the first way fails.
     */
    ret =
        krb5_c_decrypt(context, (*enc_tkt_req).session, 4 as libc::c_int,
                       0 as *const krb5_data, &mut (*req).authorization_data,
                       &mut plaintext);
    if ret != 0 {
        ret =
            krb5_c_decrypt(context, client_key, 5 as libc::c_int,
                           0 as *const krb5_data,
                           &mut (*req).authorization_data, &mut plaintext)
    }
    if !(ret != 0) {
        /* Decode the decrypted authdata and make it available to modules in the
     * request. */
        ret =
            decode_krb5_authdata(&mut plaintext, &mut (*req).unenc_authdata);
        if !(ret != 0) {
            if has_mandatory_for_kdc_authdata(context, (*req).unenc_authdata)
                   != 0 {
                ret = -(1765328372 as libc::c_long) as krb5_error_code
            } else {
                /* Add a copy of the requested authdata to the ticket, ignoring KDC-issued
     * types. */
                ret =
                    merge_authdata(context, (*req).unenc_authdata,
                                   tkt_authdata,
                                   1 as libc::c_int as krb5_boolean,
                                   1 as libc::c_int as krb5_boolean)
            }
        }
    }
    free(plaintext.data as *mut libc::c_void);
    return ret;
}
/* Copy TGT authorization data into the ticket authdata. */
#[c2rust::src_loc = "303:1"]
unsafe extern "C" fn copy_tgt_authdata(mut context: krb5_context,
                                       mut request: *mut krb5_kdc_req,
                                       mut tgt_authdata:
                                           *mut *mut krb5_authdata,
                                       mut tkt_authdata:
                                           *mut *mut *mut krb5_authdata)
 -> krb5_error_code {
    if has_mandatory_for_kdc_authdata(context, tgt_authdata) != 0 {
        return -(1765328372 as libc::c_long) as krb5_error_code
    }
    /* Add a copy of the TGT authdata to the ticket, ignoring KDC-issued
     * types. */
    return merge_authdata(context, tgt_authdata, tkt_authdata,
                          1 as libc::c_int as krb5_boolean,
                          1 as libc::c_int as krb5_boolean);
}
/* Fetch authorization data from KDB module. */
#[c2rust::src_loc = "316:1"]
unsafe extern "C" fn fetch_kdb_authdata(mut context: krb5_context,
                                        mut flags: libc::c_uint,
                                        mut client: *mut krb5_db_entry,
                                        mut server: *mut krb5_db_entry,
                                        mut header_server: *mut krb5_db_entry,
                                        mut local_tgt: *mut krb5_db_entry,
                                        mut client_key: *mut krb5_keyblock,
                                        mut server_key: *mut krb5_keyblock,
                                        mut header_key: *mut krb5_keyblock,
                                        mut local_tgt_key: *mut krb5_keyblock,
                                        mut req: *mut krb5_kdc_req,
                                        mut altcprinc: krb5_const_principal,
                                        mut ad_info: *mut libc::c_void,
                                        mut enc_tkt_req:
                                            *mut krb5_enc_tkt_part,
                                        mut enc_tkt_reply:
                                            *mut krb5_enc_tkt_part,
                                        mut auth_indicators:
                                            *mut *mut *mut krb5_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut tgt_authdata: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    let mut db_authdata: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    let mut tgs_req: krb5_boolean =
        ((*req).msg_type == 12 as libc::c_int as krb5_msgtype) as libc::c_int
            as krb5_boolean;
    let mut actual_client: krb5_const_principal =
        0 as *const krb5_principal_data;
    /*
     * Check whether KDC issued authorization data should be included.
     * A server can explicitly disable the inclusion of authorization
     * data by setting the KRB5_KDB_NO_AUTH_DATA_REQUIRED flag on its
     * principal entry. Otherwise authorization data will be included
     * if it was present in the TGT, the client is from another realm
     * or protocol transition/constrained delegation was used, or, in
     * the AS-REQ case, if the pre-auth data indicated the PAC should
     * be present.
     */
    if tgs_req != 0 {
        if !enc_tkt_req.is_null() {
        } else {
            __assert_fail(b"enc_tkt_req != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"kdc_authdata.c\x00" as *const u8 as
                              *const libc::c_char,
                          343 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 302],
                                                    &[libc::c_char; 302]>(b"krb5_error_code fetch_kdb_authdata(krb5_context, unsigned int, krb5_db_entry *, krb5_db_entry *, krb5_db_entry *, krb5_db_entry *, krb5_keyblock *, krb5_keyblock *, krb5_keyblock *, krb5_keyblock *, krb5_kdc_req *, krb5_const_principal, void *, krb5_enc_tkt_part *, krb5_enc_tkt_part *, krb5_data ***)\x00")).as_ptr());
        }
        if (*server).attributes & 0x400000 as libc::c_int != 0 {
            return 0 as libc::c_int
        }
        if (*enc_tkt_req).authorization_data.is_null() &&
               flags &
                   (0x1000 as libc::c_int |
                        (0x100 as libc::c_int | 0x200 as libc::c_int)) as
                       libc::c_uint == 0 {
            return 0 as libc::c_int
        }
        if (*enc_tkt_reply).times.authtime == (*enc_tkt_req).times.authtime {
        } else {
            __assert_fail(b"enc_tkt_reply->times.authtime == enc_tkt_req->times.authtime\x00"
                              as *const u8 as *const libc::c_char,
                          b"kdc_authdata.c\x00" as *const u8 as
                              *const libc::c_char,
                          352 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 302],
                                                    &[libc::c_char; 302]>(b"krb5_error_code fetch_kdb_authdata(krb5_context, unsigned int, krb5_db_entry *, krb5_db_entry *, krb5_db_entry *, krb5_db_entry *, krb5_keyblock *, krb5_keyblock *, krb5_keyblock *, krb5_keyblock *, krb5_kdc_req *, krb5_const_principal, void *, krb5_enc_tkt_part *, krb5_enc_tkt_part *, krb5_data ***)\x00")).as_ptr());
        }
    } else if flags & 0x20 as libc::c_int as libc::c_uint == 0 {
        return 0 as libc::c_int
    }
    /* S4U referral replies should contain authdata for the requested client,
     * even though they use the requesting service as the ticket client. */
    if flags & (0x100 as libc::c_int | 0x200 as libc::c_int) as libc::c_uint
           != 0 {
        actual_client = altcprinc
    } else { actual_client = (*enc_tkt_reply).client as krb5_const_principal }
    tgt_authdata =
        if tgs_req != 0 {
            (*enc_tkt_req).authorization_data
        } else { 0 as *mut *mut krb5_authdata };
    ret =
        krb5_db_sign_authdata(context, flags, actual_client,
                              (*req).server as krb5_const_principal, client,
                              server, header_server, local_tgt, client_key,
                              server_key, header_key, local_tgt_key,
                              (*enc_tkt_reply).session,
                              (*enc_tkt_reply).times.authtime, tgt_authdata,
                              ad_info, auth_indicators, &mut db_authdata);
    if ret != 0 {
        return if ret as libc::c_long == -(1765328134 as libc::c_long) {
                   0 as libc::c_int
               } else { ret }
    }
    /* Add the KDB authdata to the ticket, without copying or filtering. */
    ret =
        merge_authdata(context, db_authdata,
                       &mut (*enc_tkt_reply).authorization_data,
                       0 as libc::c_int as krb5_boolean,
                       0 as libc::c_int as krb5_boolean);
    if ret != 0 { krb5_free_authdata(context, db_authdata); }
    return ret;
}
#[c2rust::src_loc = "383:1"]
unsafe extern "C" fn make_signedpath_data(mut context: krb5_context,
                                          mut client: krb5_const_principal,
                                          mut authtime: krb5_timestamp,
                                          mut deleg_path: *mut krb5_principal,
                                          mut method_data:
                                              *mut *mut krb5_pa_data,
                                          mut authdata:
                                              *mut *mut krb5_authdata,
                                          mut data: *mut *mut krb5_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut sp_data: krb5_ad_signedpath_data =
        krb5_ad_signedpath_data{client: 0 as *mut krb5_principal_data,
                                authtime: 0,
                                delegated: 0 as *mut krb5_principal,
                                method_data: 0 as *mut *mut krb5_pa_data,
                                authorization_data:
                                    0 as *mut *mut krb5_authdata,};
    let mut sign_authdata: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut count: size_t = 0;
    memset(&mut sp_data as *mut krb5_ad_signedpath_data as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_ad_signedpath_data>() as libc::c_ulong);
    count = 0 as libc::c_int as size_t;
    while !authdata.is_null() && !(*authdata.offset(count as isize)).is_null()
          {
        count = count.wrapping_add(1)
    }
    if count != 0 as libc::c_int as libc::c_ulong {
        /* Make a shallow copy with AD-SIGNTICKET filtered out. */
        sign_authdata =
            k5calloc(count.wrapping_add(1 as libc::c_int as libc::c_ulong),
                     ::std::mem::size_of::<*mut krb5_authdata>() as
                         libc::c_ulong, &mut ret) as *mut *mut krb5_authdata;
        if sign_authdata.is_null() { return ret }
        i = 0 as libc::c_int as size_t;
        j = 0 as libc::c_int as size_t;
        while !(*authdata.offset(i as isize)).is_null() {
            if !(is_kdc_issued_authdatum(context,
                                         *authdata.offset(i as isize),
                                         512 as libc::c_int) != 0) {
                let fresh3 = j;
                j = j.wrapping_add(1);
                let ref mut fresh4 = *sign_authdata.offset(fresh3 as isize);
                *fresh4 = *authdata.offset(i as isize)
            }
            i = i.wrapping_add(1)
        }
        let ref mut fresh5 = *sign_authdata.offset(j as isize);
        *fresh5 = 0 as *mut krb5_authdata
    }
    sp_data.client = client as krb5_principal;
    sp_data.authtime = authtime;
    sp_data.delegated = deleg_path;
    sp_data.method_data = method_data;
    sp_data.authorization_data = sign_authdata;
    ret = encode_krb5_ad_signedpath_data(&mut sp_data, data);
    if !sign_authdata.is_null() { free(sign_authdata as *mut libc::c_void); }
    return ret;
}
#[c2rust::src_loc = "428:1"]
unsafe extern "C" fn verify_signedpath_checksum(mut context: krb5_context,
                                                mut local_tgt:
                                                    *mut krb5_db_entry,
                                                mut local_tgt_key:
                                                    *mut krb5_keyblock,
                                                mut enc_tkt_part:
                                                    *mut krb5_enc_tkt_part,
                                                mut deleg_path:
                                                    *mut krb5_principal,
                                                mut method_data:
                                                    *mut *mut krb5_pa_data,
                                                mut cksum: *mut krb5_checksum,
                                                mut valid_out:
                                                    *mut krb5_boolean)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut data: *mut krb5_data = 0 as *mut krb5_data;
    let mut kd: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut tgtkey: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut kvno: krb5_kvno = 0;
    let mut valid: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut tries: libc::c_int = 0;
    *valid_out = 0 as libc::c_int as krb5_boolean;
    memset(&mut tgtkey as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    if krb5_c_is_keyed_cksum((*cksum).checksum_type) == 0 {
        return -(1765328334 as libc::c_long) as krb5_error_code
    }
    ret =
        make_signedpath_data(context,
                             (*enc_tkt_part).client as krb5_const_principal,
                             (*enc_tkt_part).times.authtime, deleg_path,
                             method_data, (*enc_tkt_part).authorization_data,
                             &mut data);
    if ret != 0 { return ret }
    ret =
        krb5_c_verify_checksum(context, local_tgt_key, -(21 as libc::c_int),
                               data, cksum, &mut valid);
    if ret != 0 || valid == 0 {
        /* There is no kvno in AD-SIGNTICKET, so try two previous versions. */
        kvno =
            ((*(*local_tgt).key_data.offset(0 as libc::c_int as
                                                isize)).key_data_kvno as
                 libc::c_int - 1 as libc::c_int) as krb5_kvno;
        tries = 2 as libc::c_int;
        while tries > 0 as libc::c_int &&
                  kvno > 0 as libc::c_int as libc::c_uint {
            /* Get the first local tgt key of this kvno. */
            ret =
                krb5_dbe_find_enctype(context, local_tgt, -(1 as libc::c_int),
                                      -(1 as libc::c_int), kvno as krb5_int32,
                                      &mut kd);
            if ret != 0 {
                ret = 0 as libc::c_int;
                break ;
            } else {
                ret =
                    krb5_dbe_decrypt_key_data(context,
                                              0 as *const krb5_keyblock, kd,
                                              &mut tgtkey,
                                              0 as *mut krb5_keysalt);
                if ret != 0 { break ; }
                ret =
                    krb5_c_verify_checksum(context, &mut tgtkey,
                                           -(21 as libc::c_int), data, cksum,
                                           &mut valid);
                krb5_free_keyblock_contents(context, &mut tgtkey);
                if ret == 0 && valid != 0 { break ; }
                tries -= 1;
                kvno = kvno.wrapping_sub(1)
            }
        }
    }
    *valid_out = valid;
    krb5_free_data(context, data);
    return ret;
}
#[c2rust::src_loc = "489:1"]
unsafe extern "C" fn verify_signedpath(mut context: krb5_context,
                                       mut local_tgt: *mut krb5_db_entry,
                                       mut local_tgt_key: *mut krb5_keyblock,
                                       mut enc_tkt_part:
                                           *mut krb5_enc_tkt_part,
                                       mut delegated_out:
                                           *mut *mut krb5_principal,
                                       mut pathsigned_out: *mut krb5_boolean)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut sp: *mut krb5_ad_signedpath = 0 as *mut krb5_ad_signedpath;
    let mut sp_authdata: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    let mut enc_sp: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    *delegated_out = 0 as *mut krb5_principal;
    *pathsigned_out = 0 as libc::c_int as krb5_boolean;
    ret =
        krb5_find_authdata(context, (*enc_tkt_part).authorization_data,
                           0 as *const *mut krb5_authdata, 512 as libc::c_int,
                           &mut sp_authdata);
    if !(ret != 0) {
        if !(sp_authdata.is_null() ||
                 (**sp_authdata.offset(0 as libc::c_int as isize)).ad_type !=
                     512 as libc::c_int ||
                 !(*sp_authdata.offset(1 as libc::c_int as isize)).is_null())
           {
            enc_sp.data =
                (**sp_authdata.offset(0 as libc::c_int as isize)).contents as
                    *mut libc::c_char;
            enc_sp.length =
                (**sp_authdata.offset(0 as libc::c_int as isize)).length;
            ret = decode_krb5_ad_signedpath(&mut enc_sp, &mut sp);
            if ret != 0 {
                /* Treat an invalid signedpath authdata element as a missing one, since
         * we believe MS is using the same number for something else. */
                ret = 0 as libc::c_int
            } else {
                ret =
                    verify_signedpath_checksum(context, local_tgt,
                                               local_tgt_key, enc_tkt_part,
                                               (*sp).delegated,
                                               (*sp).method_data,
                                               &mut (*sp).checksum,
                                               pathsigned_out);
                if !(ret != 0) {
                    if *pathsigned_out != 0 {
                        *delegated_out = (*sp).delegated;
                        (*sp).delegated = 0 as *mut krb5_principal
                    }
                }
            }
        }
    }
    krb5_free_ad_signedpath(context, sp);
    krb5_free_authdata(context, sp_authdata);
    return ret;
}
#[c2rust::src_loc = "542:1"]
unsafe extern "C" fn make_signedpath_checksum(mut context: krb5_context,
                                              mut for_user_princ:
                                                  krb5_const_principal,
                                              mut local_tgt_key:
                                                  *mut krb5_keyblock,
                                              mut enc_tkt_part:
                                                  *mut krb5_enc_tkt_part,
                                              mut deleg_path:
                                                  *mut krb5_principal,
                                              mut method_data:
                                                  *mut *mut krb5_pa_data,
                                              mut cksum_out:
                                                  *mut krb5_checksum,
                                              mut enctype_out:
                                                  *mut krb5_enctype)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut data: *mut krb5_data = 0 as *mut krb5_data;
    let mut client: krb5_const_principal = 0 as *const krb5_principal_data;
    memset(cksum_out as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_checksum>() as libc::c_ulong);
    *enctype_out = 0 as libc::c_int;
    client =
        if !for_user_princ.is_null() {
            for_user_princ
        } else { (*enc_tkt_part).client as *const krb5_principal_data };
    ret =
        make_signedpath_data(context, client, (*enc_tkt_part).times.authtime,
                             deleg_path, method_data,
                             (*enc_tkt_part).authorization_data, &mut data);
    if ret != 0 { return ret }
    ret =
        krb5_c_make_checksum(context, 0 as libc::c_int, local_tgt_key,
                             -(21 as libc::c_int), data, cksum_out);
    krb5_free_data(context, data);
    if ret != 0 { return ret }
    *enctype_out = (*local_tgt_key).enctype;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "576:1"]
unsafe extern "C" fn make_signedpath(mut context: krb5_context,
                                     mut for_user_princ: krb5_const_principal,
                                     mut server: krb5_principal,
                                     mut local_tgt_key: *mut krb5_keyblock,
                                     mut deleg_path: *mut krb5_principal,
                                     mut enc_tkt_reply:
                                         *mut krb5_enc_tkt_part)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut sp: krb5_ad_signedpath =
        krb5_ad_signedpath{enctype: 0,
                           checksum:
                               krb5_checksum{magic: 0,
                                             checksum_type: 0,
                                             length: 0,
                                             contents: 0 as *mut krb5_octet,},
                           delegated: 0 as *mut krb5_principal,
                           method_data: 0 as *mut *mut krb5_pa_data,};
    let mut data: *mut krb5_data = 0 as *mut krb5_data;
    let mut ad_datum: krb5_authdata =
        krb5_authdata{magic: 0,
                      ad_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut ad_data: [*mut krb5_authdata; 2] = [0 as *mut krb5_authdata; 2];
    let mut if_relevant: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    let mut count: size_t = 0;
    memset(&mut sp as *mut krb5_ad_signedpath as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_ad_signedpath>() as libc::c_ulong);
    count = 0 as libc::c_int as size_t;
    while !deleg_path.is_null() &&
              !(*deleg_path.offset(count as isize)).is_null() {
        count = count.wrapping_add(1)
    }
    sp.delegated =
        k5calloc(count.wrapping_add(2 as libc::c_int as libc::c_ulong),
                 ::std::mem::size_of::<krb5_principal>() as libc::c_ulong,
                 &mut ret) as *mut krb5_principal;
    if !sp.delegated.is_null() {
        /* Combine existing and new transited services, if any */
        if !deleg_path.is_null() {
            memcpy(sp.delegated as *mut libc::c_void,
                   deleg_path as *const libc::c_void,
                   count.wrapping_mul(::std::mem::size_of::<krb5_principal>()
                                          as libc::c_ulong));
        }
        if !server.is_null() {
            let fresh6 = count;
            count = count.wrapping_add(1);
            let ref mut fresh7 = *sp.delegated.offset(fresh6 as isize);
            *fresh7 = server
        }
        let ref mut fresh8 = *sp.delegated.offset(count as isize);
        *fresh8 = 0 as krb5_principal;
        sp.method_data = 0 as *mut *mut krb5_pa_data;
        ret =
            make_signedpath_checksum(context, for_user_princ, local_tgt_key,
                                     enc_tkt_reply, sp.delegated,
                                     sp.method_data, &mut sp.checksum,
                                     &mut sp.enctype);
        if ret != 0 {
            if ret as libc::c_long == -(1765328334 as libc::c_long) {
                /*
             * In the hopefully unlikely case the TGS key enctype has an
             * unkeyed mandatory checksum type, do not fail so we do not
             * prevent the KDC from servicing requests.
             */
                ret = 0 as libc::c_int
            }
        } else {
            ret = encode_krb5_ad_signedpath(&mut sp, &mut data);
            if !(ret != 0) {
                ad_datum.ad_type = 512 as libc::c_int;
                ad_datum.contents = (*data).data as *mut krb5_octet;
                ad_datum.length = (*data).length;
                ad_data[0 as libc::c_int as usize] = &mut ad_datum;
                ad_data[1 as libc::c_int as usize] = 0 as *mut krb5_authdata;
                ret =
                    krb5_encode_authdata_container(context, 1 as libc::c_int,
                                                   ad_data.as_mut_ptr(),
                                                   &mut if_relevant);
                if !(ret != 0) {
                    /* Add the authdata to the ticket, without copying or filtering. */
                    ret =
                        merge_authdata(context, if_relevant,
                                       &mut (*enc_tkt_reply).authorization_data,
                                       0 as libc::c_int as krb5_boolean,
                                       0 as libc::c_int as
                                           krb5_boolean); /* merge_authdata() freed */
                    if !(ret != 0) {
                        if_relevant = 0 as *mut *mut krb5_authdata
                    }
                }
            }
        }
    }
    free(sp.delegated as *mut libc::c_void);
    krb5_free_authdata(context, if_relevant);
    krb5_free_data(context, data);
    krb5_free_checksum_contents(context, &mut sp.checksum);
    krb5_free_pa_data(context, sp.method_data);
    return ret;
}
#[c2rust::src_loc = "651:1"]
unsafe extern "C" fn free_deleg_path(mut context: krb5_context,
                                     mut deleg_path: *mut krb5_principal) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !deleg_path.is_null() && !(*deleg_path.offset(i as isize)).is_null()
          {
        krb5_free_principal(context, *deleg_path.offset(i as isize));
        i += 1
    }
    free(deleg_path as *mut libc::c_void);
}
/* Return true if the Windows 2000 PAC is the only element in the supplied
 * authorization data. */
#[c2rust::src_loc = "663:1"]
unsafe extern "C" fn only_pac_p(mut context: krb5_context,
                                mut authdata: *mut *mut krb5_authdata)
 -> krb5_boolean {
    return (has_kdc_issued_authdata(context, authdata, 128 as libc::c_int) !=
                0 && (*authdata.offset(1 as libc::c_int as isize)).is_null())
               as libc::c_int as krb5_boolean;
}
/* Verify AD-SIGNTICKET authdata if we need to, and insert an AD-SIGNEDPATH
 * element if we should. */
#[c2rust::src_loc = "673:1"]
unsafe extern "C" fn handle_signticket(mut context: krb5_context,
                                       mut flags: libc::c_uint,
                                       mut client: *mut krb5_db_entry,
                                       mut server: *mut krb5_db_entry,
                                       mut local_tgt: *mut krb5_db_entry,
                                       mut local_tgt_key: *mut krb5_keyblock,
                                       mut req: *mut krb5_kdc_req,
                                       mut for_user_princ:
                                           krb5_const_principal,
                                       mut enc_tkt_req:
                                           *mut krb5_enc_tkt_part,
                                       mut enc_tkt_reply:
                                           *mut krb5_enc_tkt_part)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut deleg_path: *mut krb5_principal = 0 as *mut krb5_principal;
    let mut signed_path: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut s4u2proxy: krb5_boolean = 0;
    s4u2proxy = flags & 0x200 as libc::c_int as libc::c_uint;
    /*
     * The Windows PAC fulfils the same role as the signed path
     * if it is the only authorization data element.
     */
    if (*req).msg_type == 12 as libc::c_int as krb5_msgtype &&
           only_pac_p(context, (*enc_tkt_req).authorization_data) == 0 {
        ret =
            verify_signedpath(context, local_tgt, local_tgt_key, enc_tkt_req,
                              &mut deleg_path, &mut signed_path);
        if ret != 0 {
            current_block = 5637848305381526314;
        } else if s4u2proxy != 0 &&
                      signed_path == 0 as libc::c_int as libc::c_uint {
            ret = -(1765328371 as libc::c_long) as krb5_error_code;
            current_block = 5637848305381526314;
        } else { current_block = 3276175668257526147; }
    } else { current_block = 3276175668257526147; }
    match current_block {
        3276175668257526147 =>
        /* No point in including signedpath authdata for a cross-realm TGT, since
     * it will be presented to a different KDC. */
        {
            if (*server).attributes & 0x400000 as libc::c_int == 0 &&
                   is_cross_tgs_principal((*server).princ as
                                              krb5_const_principal) == 0 &&
                   only_pac_p(context, (*enc_tkt_reply).authorization_data) ==
                       0 {
                ret =
                    make_signedpath(context, for_user_princ,
                                    if s4u2proxy != 0 {
                                        (*client).princ
                                    } else { 0 as krb5_principal },
                                    local_tgt_key, deleg_path, enc_tkt_reply);
                (ret) != 0;
            }
        }
        _ => { }
    }
    free_deleg_path(context, deleg_path);
    return ret;
}
/* Add authentication indicator authdata to enc_tkt_reply, wrapped in a CAMMAC
 * and an IF-RELEVANT container. */
#[c2rust::src_loc = "724:1"]
unsafe extern "C" fn add_auth_indicators(mut context: krb5_context,
                                         mut auth_indicators:
                                             *const *mut krb5_data,
                                         mut server_key: *mut krb5_keyblock,
                                         mut krbtgt: *mut krb5_db_entry,
                                         mut krbtgt_key: *mut krb5_keyblock,
                                         mut enc_tkt_reply:
                                             *mut krb5_enc_tkt_part)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut der_indicators: *mut krb5_data = 0 as *mut krb5_data;
    let mut ad: krb5_authdata =
        krb5_authdata{magic: 0,
                      ad_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut list: [*mut krb5_authdata; 2] = [0 as *mut krb5_authdata; 2];
    let mut cammac: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    /* Format the authentication indicators into an authdata list. */
    ret = encode_utf8_strings(auth_indicators, &mut der_indicators);
    if !(ret != 0) {
        ad.ad_type = 97 as libc::c_int;
        ad.length = (*der_indicators).length;
        ad.contents = (*der_indicators).data as *mut uint8_t;
        list[0 as libc::c_int as usize] = &mut ad;
        list[1 as libc::c_int as usize] = 0 as *mut krb5_authdata;
        /* Wrap the list in CAMMAC and IF-RELEVANT containers. */
        ret =
            cammac_create(context, enc_tkt_reply, server_key, krbtgt,
                          krbtgt_key, list.as_mut_ptr(), &mut cammac);
        if !(ret != 0) {
            /* Add the wrapped authdata to the ticket, without copying or filtering. */
            ret =
                merge_authdata(context, cammac,
                               &mut (*enc_tkt_reply).authorization_data,
                               0 as libc::c_int as krb5_boolean,
                               0 as libc::c_int as
                                   krb5_boolean); /* merge_authdata() freed */
            if !(ret != 0) { cammac = 0 as *mut *mut krb5_authdata }
        }
    }
    krb5_free_data(context, der_indicators);
    krb5_free_authdata(context, cammac);
    return ret;
}
/* Extract any properly verified authentication indicators from the authdata in
 * enc_tkt. */
#[no_mangle]
#[c2rust::src_loc = "765:1"]
pub unsafe extern "C" fn get_auth_indicators(mut context: krb5_context,
                                             mut enc_tkt:
                                                 *mut krb5_enc_tkt_part,
                                             mut local_tgt:
                                                 *mut krb5_db_entry,
                                             mut local_tgt_key:
                                                 *mut krb5_keyblock,
                                             mut indicators_out:
                                                 *mut *mut *mut krb5_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut cammacs: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut adp: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut cammac: *mut krb5_cammac = 0 as *mut krb5_cammac;
    let mut indicators: *mut *mut krb5_data = 0 as *mut *mut krb5_data;
    let mut der_cammac: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    *indicators_out = 0 as *mut *mut krb5_data;
    ret =
        krb5_find_authdata(context, (*enc_tkt).authorization_data,
                           0 as *const *mut krb5_authdata, 96 as libc::c_int,
                           &mut cammacs);
    if !(ret != 0) {
        adp = cammacs;
        loop  {
            if !(!adp.is_null() && !(*adp).is_null()) {
                current_block = 13056961889198038528;
                break ;
            }
            der_cammac =
                make_data((**adp).contents as *mut libc::c_void,
                          (**adp).length);
            ret = decode_krb5_cammac(&mut der_cammac, &mut cammac);
            if ret != 0 { current_block = 120937800574074383; break ; }
            if cammac_check_kdcver(context, cammac, enc_tkt, local_tgt,
                                   local_tgt_key) != 0 {
                ret =
                    authind_extract(context, (*cammac).elements,
                                    &mut indicators);
                if ret != 0 { current_block = 120937800574074383; break ; }
            }
            k5_free_cammac(context, cammac);
            cammac = 0 as *mut krb5_cammac;
            adp = adp.offset(1)
        }
        match current_block {
            120937800574074383 => { }
            _ => {
                *indicators_out = indicators;
                indicators = 0 as *mut *mut krb5_data
            }
        }
    }
    krb5_free_authdata(context, cammacs);
    k5_free_cammac(context, cammac);
    k5_free_data_ptr_list(indicators);
    return ret;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/kdc_util.h */
/*
 * Portions Copyright (C) 2007 Apple Inc.
 * Copyright 1990, 2007, 2014 by the Massachusetts Institute of Technology.
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
 *
 *
 * Declarations for policy.c
 */
/* authind.c */
/* cammac.c */
/* do_as_req.c */
/* do_tgs_req.c */
/* dispatch.c */
/* kdc_preauth.c */
/* kdc_preauth_ec.c */
/* kdc_preauth_enctsc.c */
/* kdc_authdata.c */
#[no_mangle]
#[c2rust::src_loc = "807:1"]
pub unsafe extern "C" fn handle_authdata(mut context: krb5_context,
                                         mut flags: libc::c_uint,
                                         mut client: *mut krb5_db_entry,
                                         mut server: *mut krb5_db_entry,
                                         mut header_server:
                                             *mut krb5_db_entry,
                                         mut local_tgt: *mut krb5_db_entry,
                                         mut local_tgt_key:
                                             *mut krb5_keyblock,
                                         mut client_key: *mut krb5_keyblock,
                                         mut server_key: *mut krb5_keyblock,
                                         mut header_key: *mut krb5_keyblock,
                                         mut req_pkt: *mut krb5_data,
                                         mut req: *mut krb5_kdc_req,
                                         mut altcprinc: krb5_const_principal,
                                         mut ad_info: *mut libc::c_void,
                                         mut enc_tkt_req:
                                             *mut krb5_enc_tkt_part,
                                         mut auth_indicators:
                                             *mut *mut *mut krb5_data,
                                         mut enc_tkt_reply:
                                             *mut krb5_enc_tkt_part)
 -> krb5_error_code {
    let mut h: *mut kdcauthdata_handle = 0 as *mut kdcauthdata_handle;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut i: size_t = 0;
    if (*req).msg_type == 12 as libc::c_int as krb5_msgtype &&
           !(*req).authorization_data.ciphertext.data.is_null() {
        /* Copy TGS request authdata.  This must be done first so that modules
         * have access to the unencrypted request authdata. */
        ret =
            copy_request_authdata(context, client_key, req, enc_tkt_req,
                                  &mut (*enc_tkt_reply).authorization_data);
        if ret != 0 { return ret }
    }
    /* Invoke loaded module handlers. */
    if (*enc_tkt_reply).flags & 0x8000 as libc::c_int == 0 {
        i = 0 as libc::c_int as size_t;
        while i < n_authdata_modules {
            h =
                &mut *authdata_modules.offset(i as isize) as
                    *mut kdcauthdata_handle;
            ret =
                (*h).vt.handle.expect("non-null function pointer")(context,
                                                                   (*h).data,
                                                                   flags,
                                                                   client,
                                                                   server,
                                                                   header_server,
                                                                   client_key,
                                                                   server_key,
                                                                   header_key,
                                                                   req_pkt,
                                                                   req,
                                                                   altcprinc,
                                                                   enc_tkt_req,
                                                                   enc_tkt_reply);
            if ret != 0 {
                kdc_err(context, ret as errcode_t,
                        b"from authdata module %s\x00" as *const u8 as
                            *const libc::c_char, (*h).vt.name);
            }
            i = i.wrapping_add(1)
        }
    }
    if (*req).msg_type == 12 as libc::c_int as krb5_msgtype {
        /* Copy authdata from the TGT to the issued ticket. */
        ret =
            copy_tgt_authdata(context, req, (*enc_tkt_req).authorization_data,
                              &mut (*enc_tkt_reply).authorization_data);
        if ret != 0 { return ret }
    }
    if (*enc_tkt_reply).flags & 0x8000 as libc::c_int == 0 {
        /* Fetch authdata from the KDB if appropriate. */
        ret =
            fetch_kdb_authdata(context, flags, client, server, header_server,
                               local_tgt, client_key, server_key, header_key,
                               local_tgt_key, req, altcprinc, ad_info,
                               enc_tkt_req, enc_tkt_reply, auth_indicators);
        if ret != 0 { return ret }
    }
    /* Add auth indicators if any were given. */
    if !auth_indicators.is_null() && !(*auth_indicators).is_null() &&
           (*server).attributes & 0x400000 as libc::c_int == 0 {
        ret =
            add_auth_indicators(context, *auth_indicators, server_key,
                                local_tgt, local_tgt_key, enc_tkt_reply);
        if ret != 0 { return ret }
    }
    if (*enc_tkt_reply).flags & 0x8000 as libc::c_int == 0 {
        /* Validate and insert AD-SIGNTICKET authdata.  This must happen last
         * since it contains a signature over the other authdata. */
        ret =
            handle_signticket(context, flags, client, server, local_tgt,
                              local_tgt_key, req, altcprinc, enc_tkt_req,
                              enc_tkt_reply);
        if ret != 0 { return ret }
    }
    return 0 as libc::c_int;
}
