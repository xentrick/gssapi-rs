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
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
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
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
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
    #[c2rust::src_loc = "354:1"]
    pub type krb5_auth_context = *mut _krb5_auth_context;
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
    /* *< Response to TGS request */
    /* *< Auth req to application server */
    /* *< Response to mutual AP request */
    /* *< Safe application message */
    /* *< Private application message */
    /* *< Cred forwarding message */
    /* *< Error response */
    /* LastReq types */
    /* PADATA types */
    /* *< RFC 4120 */
    /* *< RFC 4120 */
    /* Not used */
    /* *< timestamp encrypted in key. RFC 4120 */
    /* *< SecurId passcode. RFC 4120 */
    /* *< Sesame project. RFC 4120 */
    /* *< OSF DCE. RFC 4120 */
    /* *< Cybersafe. RFC 4120 */
    /* *< Cygnus. RFC 4120, 3961 */
    /* *< Etype info for preauth. RFC 4120 */
    /* *< SAM/OTP */
    /* *< SAM/OTP */
    /* *< PKINIT */
    /* *< PKINIT */
    /* *< PKINIT. RFC 4556 */
    /* *< PKINIT. RFC 4556 */
    /* *< RFC 4120 */
    /* *< RFC 4120 */
    /* *< Windows 2000 referrals. RFC 6820 */
    /* *< SAM/OTP. RFC 4120 */
    /* *< Embedded in typed data. RFC 4120 */
    /* *< draft referral system */
    /* *< draft challenge system, updated */
    /* *< draft challenge system, updated */
    /* MS-KILE */
    /* *< include Windows PAC */
    /* *< username protocol transition request */
    /* *< certificate protocol transition request */
    /* *< AS checksum */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6560 section 4.1 */
    /* *< RFC 6560 section 4.2 */
    /* *< RFC 6560 section 4.3 */
    /* *< RFC 6112 */
    /* *< RFC 6806 */
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
    #[c2rust::src_loc = "1993:16"]
    pub struct _krb5_authenticator {
        pub magic: krb5_magic,
        pub client: krb5_principal,
        pub checksum: *mut krb5_checksum,
        pub cusec: krb5_int32,
        pub ctime: krb5_timestamp,
        pub subkey: *mut krb5_keyblock,
        pub seq_number: krb5_ui_4,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* the unencrypted version */
/* *
 * Ticket authenticator.
 *
 * The C representation of an unencrypted authenticator.
 */
    #[c2rust::src_loc = "1993:1"]
    pub type krb5_authenticator = _krb5_authenticator;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2031:16"]
    pub struct _krb5_last_req_entry {
        pub magic: krb5_magic,
        pub lr_type: krb5_int32,
        pub value: krb5_timestamp,
    }
    /* *< client name/realm */
    /* *< checksum, includes type, optional */
    /* *< client usec portion */
    /* *< client sec portion */
    /* *< true session key, optional */
    /* *< sequence #, optional */
    /* *< authoriazation data */
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
    #[c2rust::src_loc = "2122:16"]
    pub struct _krb5_ap_req {
        pub magic: krb5_magic,
        pub ap_options: krb5_flags,
        pub ticket: *mut krb5_ticket,
        pub authenticator: krb5_enc_data,
    }
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
    /* * Authentication header. */
    #[c2rust::src_loc = "2122:1"]
    pub type krb5_ap_req = _krb5_ap_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2134:16"]
    pub struct _krb5_ap_rep {
        pub magic: krb5_magic,
        pub enc_part: krb5_enc_data,
    }
    /* *< Requested options */
    /* *< Ticket */
    /* *< Encrypted authenticator */
    /* *
 * C representaton of AP-REP message.
 *
 * The server's response to a client's request for mutual authentication.
 */
    #[c2rust::src_loc = "2134:1"]
    pub type krb5_ap_rep = _krb5_ap_rep;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2140:16"]
    pub struct _krb5_ap_rep_enc_part {
        pub magic: krb5_magic,
        pub ctime: krb5_timestamp,
        pub cusec: krb5_int32,
        pub subkey: *mut krb5_keyblock,
        pub seq_number: krb5_ui_4,
    }
    /* *< Ciphertext of ApRepEncPart */
    /* * Cleartext that is encrypted and put into @c _krb5_ap_rep.  */
    #[c2rust::src_loc = "2140:1"]
    pub type krb5_ap_rep_enc_part = _krb5_ap_rep_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2158:16"]
    pub struct _krb5_cred_info {
        pub magic: krb5_magic,
        pub session: *mut krb5_keyblock,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub flags: krb5_flags,
        pub times: krb5_ticket_times,
        pub caddrs: *mut *mut krb5_address,
    }
    /* *< Client time, seconds portion */
    /* *< Client time, microseconds portion */
    /* *< Subkey (optional) */
    /* *< Sequence number */
    /* * Credentials information inserted into @c EncKrbCredPart. */
    #[c2rust::src_loc = "2158:1"]
    pub type krb5_cred_info = _krb5_cred_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2169:16"]
    pub struct _krb5_cred_enc_part {
        pub magic: krb5_magic,
        pub nonce: krb5_int32,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
        pub ticket_info: *mut *mut krb5_cred_info,
    }
    /* *< Session key used to encrypt ticket */
    /* *< Client principal and realm */
    /* *< Server principal and realm */
    /* *< Ticket flags */
    /* *< Auth, start, end, renew_till */
    /* *< Array of pointers to addrs (optional) */
    /* * Cleartext credentials information.  */
    #[c2rust::src_loc = "2169:1"]
    pub type krb5_cred_enc_part = _krb5_cred_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2180:16"]
    pub struct _krb5_cred {
        pub magic: krb5_magic,
        pub tickets: *mut *mut krb5_ticket,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_cred_enc_part,
    }
    /* *< Nonce (optional) */
    /* *< Generation time, seconds portion */
    /* *< Generation time, microseconds portion */
    /* *< Sender address (optional) */
    /* *< Recipient address (optional) */
    /* * Credentials data structure.*/
    #[c2rust::src_loc = "2180:1"]
    pub type krb5_cred = _krb5_cred;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2216:16"]
    pub struct _krb5_pa_pac_req {
        pub include_pac: krb5_boolean,
    }
    #[c2rust::src_loc = "2216:1"]
    pub type krb5_pa_pac_req = _krb5_pa_pac_req;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* *< Tickets */
        /* *< Encrypted part */
        /* *< Unencrypted version, if available */
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
        #[c2rust::src_loc = "353:8"]
        pub type _krb5_auth_context;
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
 * Free a krb5 library context.
 *
 * @param [in] context          Library context
 *
 * This function frees a @a context that was created by krb5_init_context()
 * or krb5_init_secure_context().
 */
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
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
 * Free a krb5_authenticator structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Authenticator structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4607:1"]
        pub fn krb5_free_authenticator(context: krb5_context,
                                       val: *mut krb5_authenticator);
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
 * Free a ticket.
 *
 * @param [in] context          Library context
 * @param [in] val              Ticket to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4644:1"]
        pub fn krb5_free_ticket(context: krb5_context, val: *mut krb5_ticket);
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
 * Free a krb5_ap_rep_enc_part structure.
 *
 * @param [in] context          Library context
 * @param [in] val              AP-REP enc part to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4732:1"]
        pub fn krb5_free_ap_rep_enc_part(context: krb5_context,
                                         val: *mut krb5_ap_rep_enc_part);
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "420:16"]
    pub struct _krb5_etype_info_entry {
        pub magic: krb5_magic,
        pub etype: krb5_enctype,
        pub length: libc::c_uint,
        pub salt: *mut krb5_octet,
        pub s2kparams: krb5_data,
    }
    #[c2rust::src_loc = "420:1"]
    pub type krb5_etype_info_entry = _krb5_etype_info_entry;
    #[c2rust::src_loc = "436:1"]
    pub type krb5_etype_info = *mut *mut krb5_etype_info_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "460:16"]
    pub struct _krb5_sam_challenge_2 {
        pub sam_challenge_2_body: krb5_data,
        pub sam_cksum: *mut *mut krb5_checksum,
    }
    #[c2rust::src_loc = "460:1"]
    pub type krb5_sam_challenge_2 = _krb5_sam_challenge_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "465:16"]
    pub struct _krb5_sam_challenge_2_body {
        pub magic: krb5_magic,
        pub sam_type: krb5_int32,
        pub sam_flags: krb5_flags,
        pub sam_type_name: krb5_data,
        pub sam_track_id: krb5_data,
        pub sam_challenge_label: krb5_data,
        pub sam_challenge: krb5_data,
        pub sam_response_prompt: krb5_data,
        pub sam_pk_for_sad: krb5_data,
        pub sam_nonce: krb5_int32,
        pub sam_etype: krb5_enctype,
    }
    #[c2rust::src_loc = "465:1"]
    pub type krb5_sam_challenge_2_body = _krb5_sam_challenge_2_body;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "515:16"]
    pub struct _krb5_otp_tokeninfo {
        pub flags: krb5_flags,
        pub vendor: krb5_data,
        pub challenge: krb5_data,
        pub length: krb5_int32,
        pub format: krb5_int32,
        pub token_id: krb5_data,
        pub alg_id: krb5_data,
        pub supported_hash_alg: *mut *mut krb5_algorithm_identifier,
        pub iteration_count: krb5_int32,
    }
    #[c2rust::src_loc = "515:1"]
    pub type krb5_otp_tokeninfo = _krb5_otp_tokeninfo;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "527:16"]
    pub struct _krb5_pa_otp_challenge {
        pub nonce: krb5_data,
        pub service: krb5_data,
        pub tokeninfo: *mut *mut krb5_otp_tokeninfo,
        pub salt: krb5_data,
        pub s2kparams: krb5_data,
    }
    #[c2rust::src_loc = "527:1"]
    pub type krb5_pa_otp_challenge = _krb5_pa_otp_challenge;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "535:16"]
    pub struct _krb5_pa_otp_req {
        pub flags: krb5_int32,
        pub nonce: krb5_data,
        pub enc_data: krb5_enc_data,
        pub hash_alg: *mut krb5_algorithm_identifier,
        pub iteration_count: krb5_int32,
        pub otp_value: krb5_data,
        pub pin: krb5_data,
        pub challenge: krb5_data,
        pub time: krb5_timestamp,
        pub counter: krb5_data,
        pub format: krb5_int32,
        pub token_id: krb5_data,
        pub alg_id: krb5_data,
        pub vendor: krb5_data,
    }
    #[c2rust::src_loc = "535:1"]
    pub type krb5_pa_otp_req = _krb5_pa_otp_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "552:16"]
    pub struct _krb5_kkdcp_message {
        pub kerb_message: krb5_data,
        pub target_domain: krb5_data,
        pub dclocator_hint: krb5_int32,
    }
    #[c2rust::src_loc = "552:1"]
    pub type krb5_kkdcp_message = _krb5_kkdcp_message;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "559:16"]
    pub struct _krb5_secure_cookie {
        pub time: time_t,
        pub data: *mut *mut krb5_pa_data,
    }
    #[c2rust::src_loc = "559:1"]
    pub type krb5_secure_cookie = _krb5_secure_cookie;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "746:16"]
    pub struct _krb5_pa_enc_ts {
        pub patimestamp: krb5_timestamp,
        pub pausec: krb5_int32,
    }
    #[c2rust::src_loc = "746:1"]
    pub type krb5_pa_enc_ts = _krb5_pa_enc_ts;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "751:16"]
    pub struct _krb5_pa_for_user {
        pub user: krb5_principal,
        pub cksum: krb5_checksum,
        pub auth_package: krb5_data,
    }
    #[c2rust::src_loc = "751:1"]
    pub type krb5_pa_for_user = _krb5_pa_for_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "757:16"]
    pub struct _krb5_s4u_userid {
        pub nonce: krb5_int32,
        pub user: krb5_principal,
        pub subject_cert: krb5_data,
        pub options: krb5_flags,
    }
    #[c2rust::src_loc = "757:1"]
    pub type krb5_s4u_userid = _krb5_s4u_userid;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "767:16"]
    pub struct _krb5_pa_s4u_x509_user {
        pub user_id: krb5_s4u_userid,
        pub cksum: krb5_checksum,
    }
    #[c2rust::src_loc = "767:1"]
    pub type krb5_pa_s4u_x509_user = _krb5_pa_s4u_x509_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "798:16"]
    pub struct _krb5_fast_finished {
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub client: krb5_principal,
        pub ticket_checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "798:1"]
    pub type krb5_fast_finished = _krb5_fast_finished;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "805:16"]
    pub struct _krb5_fast_response {
        pub magic: krb5_magic,
        pub padata: *mut *mut krb5_pa_data,
        pub strengthen_key: *mut krb5_keyblock,
        pub finished: *mut krb5_fast_finished,
        pub nonce: krb5_int32,
    }
    #[c2rust::src_loc = "805:1"]
    pub type krb5_fast_response = _krb5_fast_response;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "813:16"]
    pub struct _krb5_ad_kdcissued {
        pub ad_checksum: krb5_checksum,
        pub i_principal: krb5_principal,
        pub elements: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "813:1"]
    pub type krb5_ad_kdcissued = _krb5_ad_kdcissued;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "827:16"]
    pub struct _krb5_ad_signedpath {
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
        pub delegated: *mut krb5_principal,
        pub method_data: *mut *mut krb5_pa_data,
    }
    #[c2rust::src_loc = "827:1"]
    pub type krb5_ad_signedpath = _krb5_ad_signedpath;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "834:16"]
    pub struct _krb5_iakerb_header {
        pub target_realm: krb5_data,
        pub cookie: *mut krb5_data,
    }
    #[c2rust::src_loc = "834:1"]
    pub type krb5_iakerb_header = _krb5_iakerb_header;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "839:16"]
    pub struct _krb5_iakerb_finished {
        pub checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "839:1"]
    pub type krb5_iakerb_finished = _krb5_iakerb_finished;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "843:16"]
    pub struct _krb5_verifier_mac {
        pub princ: krb5_principal,
        pub kvno: krb5_kvno,
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "843:1"]
    pub type krb5_verifier_mac = _krb5_verifier_mac;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "856:16"]
    pub struct _krb5_cammac {
        pub elements: *mut *mut krb5_authdata,
        pub kdc_verifier: *mut krb5_verifier_mac,
        pub svc_verifier: *mut krb5_verifier_mac,
        pub other_verifiers: *mut *mut krb5_verifier_mac,
    }
    #[c2rust::src_loc = "856:1"]
    pub type krb5_cammac = _krb5_cammac;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1281:16"]
    pub struct _krb5_safe {
        pub magic: krb5_magic,
        pub user_data: krb5_data,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq_number: krb5_ui_4,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
        pub checksum: *mut krb5_checksum,
    }
    #[c2rust::src_loc = "1281:1"]
    pub type krb5_safe = _krb5_safe;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1293:16"]
    pub struct _krb5_priv {
        pub magic: krb5_magic,
        pub enc_part: krb5_enc_data,
    }
    #[c2rust::src_loc = "1293:1"]
    pub type krb5_priv = _krb5_priv;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1298:16"]
    pub struct _krb5_priv_enc_part {
        pub magic: krb5_magic,
        pub user_data: krb5_data,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq_number: krb5_ui_4,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
    }
    #[c2rust::src_loc = "1298:1"]
    pub type krb5_priv_enc_part = _krb5_priv_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1737:8"]
    pub struct ldap_seqof_key_data {
        pub mkvno: krb5_int32,
        pub kvno: krb5_ui_2,
        pub key_data: *mut _krb5_key_data,
        pub n_key_data: krb5_int16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1925:16"]
    pub struct _krb5int_access {
        pub auth_con_get_subkey_enctype: Option<unsafe extern "C" fn(_:
                                                                         krb5_context,
                                                                     _:
                                                                         krb5_auth_context,
                                                                     _:
                                                                         *mut krb5_enctype)
                                                    -> krb5_error_code>,
        pub mandatory_cksumtype: Option<unsafe extern "C" fn(_: krb5_context,
                                                             _: krb5_enctype,
                                                             _:
                                                                 *mut krb5_cksumtype)
                                            -> krb5_error_code>,
        pub ser_pack_int64: Option<unsafe extern "C" fn(_: int64_t,
                                                        _:
                                                            *mut *mut krb5_octet,
                                                        _: *mut size_t)
                                       -> krb5_error_code>,
        pub ser_unpack_int64: Option<unsafe extern "C" fn(_: *mut int64_t,
                                                          _:
                                                              *mut *mut krb5_octet,
                                                          _: *mut size_t)
                                         -> krb5_error_code>,
        pub asn1_ldap_encode_sequence_of_keys: Option<unsafe extern "C" fn(_:
                                                                               *const ldap_seqof_key_data,
                                                                           _:
                                                                               *mut *mut krb5_data)
                                                          -> krb5_error_code>,
        pub asn1_ldap_decode_sequence_of_keys: Option<unsafe extern "C" fn(_:
                                                                               *const krb5_data,
                                                                           _:
                                                                               *mut *mut ldap_seqof_key_data)
                                                          -> krb5_error_code>,
        pub encode_krb5_auth_pack: Option<unsafe extern "C" fn(_:
                                                                   *const krb5_auth_pack,
                                                               _:
                                                                   *mut *mut krb5_data)
                                              -> krb5_error_code>,
        pub encode_krb5_kdc_dh_key_info: Option<unsafe extern "C" fn(_:
                                                                         *const krb5_kdc_dh_key_info,
                                                                     _:
                                                                         *mut *mut krb5_data)
                                                    -> krb5_error_code>,
        pub encode_krb5_pa_pk_as_rep: Option<unsafe extern "C" fn(_:
                                                                      *const krb5_pa_pk_as_rep,
                                                                  _:
                                                                      *mut *mut krb5_data)
                                                 -> krb5_error_code>,
        pub encode_krb5_pa_pk_as_req: Option<unsafe extern "C" fn(_:
                                                                      *const krb5_pa_pk_as_req,
                                                                  _:
                                                                      *mut *mut krb5_data)
                                                 -> krb5_error_code>,
        pub encode_krb5_reply_key_pack: Option<unsafe extern "C" fn(_:
                                                                        *const krb5_reply_key_pack,
                                                                    _:
                                                                        *mut *mut krb5_data)
                                                   -> krb5_error_code>,
        pub encode_krb5_td_dh_parameters: Option<unsafe extern "C" fn(_:
                                                                          *const *mut krb5_algorithm_identifier,
                                                                      _:
                                                                          *mut *mut krb5_data)
                                                     -> krb5_error_code>,
        pub encode_krb5_td_trusted_certifiers: Option<unsafe extern "C" fn(_:
                                                                               *const *mut krb5_external_principal_identifier,
                                                                           _:
                                                                               *mut *mut krb5_data)
                                                          -> krb5_error_code>,
        pub decode_krb5_auth_pack: Option<unsafe extern "C" fn(_:
                                                                   *const krb5_data,
                                                               _:
                                                                   *mut *mut krb5_auth_pack)
                                              -> krb5_error_code>,
        pub decode_krb5_pa_pk_as_req: Option<unsafe extern "C" fn(_:
                                                                      *const krb5_data,
                                                                  _:
                                                                      *mut *mut krb5_pa_pk_as_req)
                                                 -> krb5_error_code>,
        pub decode_krb5_pa_pk_as_rep: Option<unsafe extern "C" fn(_:
                                                                      *const krb5_data,
                                                                  _:
                                                                      *mut *mut krb5_pa_pk_as_rep)
                                                 -> krb5_error_code>,
        pub decode_krb5_kdc_dh_key_info: Option<unsafe extern "C" fn(_:
                                                                         *const krb5_data,
                                                                     _:
                                                                         *mut *mut krb5_kdc_dh_key_info)
                                                    -> krb5_error_code>,
        pub decode_krb5_principal_name: Option<unsafe extern "C" fn(_:
                                                                        *const krb5_data,
                                                                    _:
                                                                        *mut *mut krb5_principal_data)
                                                   -> krb5_error_code>,
        pub decode_krb5_reply_key_pack: Option<unsafe extern "C" fn(_:
                                                                        *const krb5_data,
                                                                    _:
                                                                        *mut *mut krb5_reply_key_pack)
                                                   -> krb5_error_code>,
        pub decode_krb5_td_dh_parameters: Option<unsafe extern "C" fn(_:
                                                                          *const krb5_data,
                                                                      _:
                                                                          *mut *mut *mut krb5_algorithm_identifier)
                                                     -> krb5_error_code>,
        pub decode_krb5_td_trusted_certifiers: Option<unsafe extern "C" fn(_:
                                                                               *const krb5_data,
                                                                           _:
                                                                               *mut *mut *mut krb5_external_principal_identifier)
                                                          -> krb5_error_code>,
        pub encode_krb5_kdc_req_body: Option<unsafe extern "C" fn(_:
                                                                      *const krb5_kdc_req,
                                                                  _:
                                                                      *mut *mut krb5_data)
                                                 -> krb5_error_code>,
        pub free_kdc_req: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: *mut krb5_kdc_req)
                                     -> ()>,
        pub set_prompt_types: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *mut krb5_prompt_type)
                                         -> ()>,
    }
    #[c2rust::src_loc = "1925:1"]
    pub type krb5int_access = _krb5int_access;
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_octet, krb5_data, krb5_checksum, krb5_enc_data,
                        krb5_timestamp, krb5_pa_data, krb5_principal,
                        krb5_keyblock, krb5_authdata, krb5_kvno, krb5_ui_4,
                        krb5_address, krb5_ui_2, krb5_int16, krb5_error_code,
                        krb5_context, krb5_auth_context, krb5_cksumtype,
                        krb5_principal_data, krb5_kdc_req, krb5_authenticator,
                        krb5_ticket, krb5_enc_tkt_part, krb5_enc_kdc_rep_part,
                        krb5_kdc_rep, krb5_ap_req, krb5_ap_rep,
                        krb5_ap_rep_enc_part, krb5_cred, krb5_cred_enc_part,
                        krb5_error, krb5_pa_pac_req, krb5_authdatatype};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::k5_int_pkinit_h::{krb5_algorithm_identifier, krb5_auth_pack,
                                 krb5_kdc_dh_key_info, krb5_pa_pk_as_rep,
                                 krb5_pa_pk_as_req, krb5_reply_key_pack,
                                 krb5_external_principal_identifier};
    use super::time_t_h::time_t;
    use super::kdb_h::_krb5_key_data;
    use super::stdint_intn_h::int64_t;
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
        #[c2rust::src_loc = "904:1"]
        pub fn krb5_free_sam_challenge_2(_: krb5_context,
                                         _: *mut krb5_sam_challenge_2);
        #[no_mangle]
        #[c2rust::src_loc = "907:1"]
        pub fn krb5_free_sam_challenge_2_body(_: krb5_context,
                                              _:
                                                  *mut krb5_sam_challenge_2_body);
        #[no_mangle]
        #[c2rust::src_loc = "930:1"]
        pub fn krb5_free_pa_enc_ts(_: krb5_context, _: *mut krb5_pa_enc_ts);
        #[no_mangle]
        #[c2rust::src_loc = "933:1"]
        pub fn krb5_free_pa_for_user(_: krb5_context,
                                     _: *mut krb5_pa_for_user);
        #[no_mangle]
        #[c2rust::src_loc = "939:1"]
        pub fn krb5_free_pa_s4u_x509_user(_: krb5_context,
                                          _: *mut krb5_pa_s4u_x509_user);
        #[no_mangle]
        #[c2rust::src_loc = "950:1"]
        pub fn krb5_free_fast_response(_: krb5_context,
                                       _: *mut krb5_fast_response);
        #[no_mangle]
        #[c2rust::src_loc = "951:1"]
        pub fn krb5_free_ad_kdcissued(_: krb5_context,
                                      _: *mut krb5_ad_kdcissued);
        #[no_mangle]
        #[c2rust::src_loc = "952:1"]
        pub fn krb5_free_ad_signedpath(_: krb5_context,
                                       _: *mut krb5_ad_signedpath);
        #[no_mangle]
        #[c2rust::src_loc = "953:1"]
        pub fn krb5_free_iakerb_header(_: krb5_context,
                                       _: *mut krb5_iakerb_header);
        #[no_mangle]
        #[c2rust::src_loc = "954:1"]
        pub fn krb5_free_iakerb_finished(_: krb5_context,
                                         _: *mut krb5_iakerb_finished);
        #[no_mangle]
        #[c2rust::src_loc = "958:1"]
        pub fn k5_free_otp_tokeninfo(context: krb5_context,
                                     val: *mut krb5_otp_tokeninfo);
        #[no_mangle]
        #[c2rust::src_loc = "959:1"]
        pub fn k5_free_pa_otp_challenge(context: krb5_context,
                                        val: *mut krb5_pa_otp_challenge);
        #[no_mangle]
        #[c2rust::src_loc = "961:1"]
        pub fn k5_free_pa_otp_req(context: krb5_context,
                                  val: *mut krb5_pa_otp_req);
        #[no_mangle]
        #[c2rust::src_loc = "963:1"]
        pub fn k5_free_cammac(context: krb5_context, val: *mut krb5_cammac);
        #[no_mangle]
        #[c2rust::src_loc = "964:1"]
        pub fn k5_free_secure_cookie(context: krb5_context,
                                     val: *mut krb5_secure_cookie);
        #[no_mangle]
        #[c2rust::src_loc = "1308:1"]
        pub fn krb5_free_safe(_: krb5_context, _: *mut krb5_safe);
        #[no_mangle]
        #[c2rust::src_loc = "1309:1"]
        pub fn krb5_free_priv(_: krb5_context, _: *mut krb5_priv);
        #[no_mangle]
        #[c2rust::src_loc = "1310:1"]
        pub fn krb5_free_priv_enc_part(_: krb5_context,
                                       _: *mut krb5_priv_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "1554:1"]
        pub fn decode_krb5_sam_challenge_2(_: *const krb5_data,
                                           _: *mut *mut krb5_sam_challenge_2)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1557:1"]
        pub fn decode_krb5_sam_challenge_2_body(_: *const krb5_data,
                                                _:
                                                    *mut *mut krb5_sam_challenge_2_body)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1583:1"]
        pub fn decode_krb5_authenticator(code: *const krb5_data,
                                         rep: *mut *mut krb5_authenticator)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1586:1"]
        pub fn decode_krb5_ticket(code: *const krb5_data,
                                  rep: *mut *mut krb5_ticket)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1589:1"]
        pub fn decode_krb5_encryption_key(output: *const krb5_data,
                                          rep: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1592:1"]
        pub fn decode_krb5_enc_tkt_part(output: *const krb5_data,
                                        rep: *mut *mut krb5_enc_tkt_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1595:1"]
        pub fn decode_krb5_enc_kdc_rep_part(output: *const krb5_data,
                                            rep:
                                                *mut *mut krb5_enc_kdc_rep_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1599:1"]
        pub fn decode_krb5_as_rep(output: *const krb5_data,
                                  rep: *mut *mut krb5_kdc_rep)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1602:1"]
        pub fn decode_krb5_tgs_rep(output: *const krb5_data,
                                   rep: *mut *mut krb5_kdc_rep)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1605:1"]
        pub fn decode_krb5_ap_req(output: *const krb5_data,
                                  rep: *mut *mut krb5_ap_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1608:1"]
        pub fn decode_krb5_ap_rep(output: *const krb5_data,
                                  rep: *mut *mut krb5_ap_rep)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1611:1"]
        pub fn decode_krb5_ap_rep_enc_part(output: *const krb5_data,
                                           rep:
                                               *mut *mut krb5_ap_rep_enc_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1615:1"]
        pub fn decode_krb5_as_req(output: *const krb5_data,
                                  rep: *mut *mut krb5_kdc_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1618:1"]
        pub fn decode_krb5_tgs_req(output: *const krb5_data,
                                   rep: *mut *mut krb5_kdc_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1621:1"]
        pub fn decode_krb5_kdc_req_body(output: *const krb5_data,
                                        rep: *mut *mut krb5_kdc_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1624:1"]
        pub fn decode_krb5_safe(output: *const krb5_data,
                                rep: *mut *mut krb5_safe) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1631:1"]
        pub fn decode_krb5_priv(output: *const krb5_data,
                                rep: *mut *mut krb5_priv) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1634:1"]
        pub fn decode_krb5_enc_priv_part(output: *const krb5_data,
                                         rep: *mut *mut krb5_priv_enc_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1639:1"]
        pub fn decode_krb5_cred(output: *const krb5_data,
                                rep: *mut *mut krb5_cred) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1642:1"]
        pub fn decode_krb5_enc_cred_part(output: *const krb5_data,
                                         rep: *mut *mut krb5_cred_enc_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1645:1"]
        pub fn decode_krb5_error(output: *const krb5_data,
                                 rep: *mut *mut krb5_error)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1648:1"]
        pub fn decode_krb5_authdata(output: *const krb5_data,
                                    rep: *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1651:1"]
        pub fn decode_krb5_padata_sequence(output: *const krb5_data,
                                           rep: *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1654:1"]
        pub fn decode_krb5_typed_data(_: *const krb5_data,
                                      _: *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1657:1"]
        pub fn decode_krb5_etype_info(output: *const krb5_data,
                                      rep:
                                          *mut *mut *mut krb5_etype_info_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1660:1"]
        pub fn decode_krb5_etype_info2(output: *const krb5_data,
                                       rep:
                                           *mut *mut *mut krb5_etype_info_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1663:1"]
        pub fn decode_krb5_enc_data(output: *const krb5_data,
                                    rep: *mut *mut krb5_enc_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1666:1"]
        pub fn decode_krb5_pa_enc_ts(output: *const krb5_data,
                                     rep: *mut *mut krb5_pa_enc_ts)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1672:1"]
        pub fn decode_krb5_pa_for_user(_: *const krb5_data,
                                       _: *mut *mut krb5_pa_for_user)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1675:1"]
        pub fn decode_krb5_pa_s4u_x509_user(_: *const krb5_data,
                                            _:
                                                *mut *mut krb5_pa_s4u_x509_user)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1678:1"]
        pub fn decode_krb5_pa_pac_req(_: *const krb5_data,
                                      _: *mut *mut krb5_pa_pac_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1690:1"]
        pub fn decode_krb5_pa_fx_fast_reply(_: *const krb5_data,
                                            _: *mut *mut krb5_enc_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1693:1"]
        pub fn decode_krb5_fast_response(_: *const krb5_data,
                                         _: *mut *mut krb5_fast_response)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1696:1"]
        pub fn decode_krb5_ad_kdcissued(_: *const krb5_data,
                                        _: *mut *mut krb5_ad_kdcissued)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1699:1"]
        pub fn decode_krb5_ad_signedpath(_: *const krb5_data,
                                         _: *mut *mut krb5_ad_signedpath)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1702:1"]
        pub fn decode_krb5_iakerb_header(_: *const krb5_data,
                                         _: *mut *mut krb5_iakerb_header)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1705:1"]
        pub fn decode_krb5_iakerb_finished(_: *const krb5_data,
                                           _: *mut *mut krb5_iakerb_finished)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1708:1"]
        pub fn decode_krb5_otp_tokeninfo(_: *const krb5_data,
                                         _: *mut *mut krb5_otp_tokeninfo)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1711:1"]
        pub fn decode_krb5_pa_otp_challenge(_: *const krb5_data,
                                            _:
                                                *mut *mut krb5_pa_otp_challenge)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1714:1"]
        pub fn decode_krb5_pa_otp_req(_: *const krb5_data,
                                      _: *mut *mut krb5_pa_otp_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1717:1"]
        pub fn decode_krb5_pa_otp_enc_req(_: *const krb5_data,
                                          _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1720:1"]
        pub fn decode_krb5_kkdcp_message(_: *const krb5_data,
                                         _: *mut *mut krb5_kkdcp_message)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1723:1"]
        pub fn decode_krb5_cammac(_: *const krb5_data,
                                  _: *mut *mut krb5_cammac)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1729:1"]
        pub fn decode_krb5_secure_cookie(_: *const krb5_data,
                                         _: *mut *mut krb5_secure_cookie)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2147:1"]
        pub fn krb5_free_enc_tkt_part(_: krb5_context,
                                      _: *mut krb5_enc_tkt_part);
        #[no_mangle]
        #[c2rust::src_loc = "2150:1"]
        pub fn krb5_free_kdc_req(_: krb5_context, _: *mut krb5_kdc_req);
        #[no_mangle]
        #[c2rust::src_loc = "2151:1"]
        pub fn krb5_free_kdc_rep(_: krb5_context, _: *mut krb5_kdc_rep);
        #[no_mangle]
        #[c2rust::src_loc = "2153:1"]
        pub fn krb5_free_enc_kdc_rep_part(_: krb5_context,
                                          _: *mut krb5_enc_kdc_rep_part);
        #[no_mangle]
        #[c2rust::src_loc = "2155:1"]
        pub fn krb5_free_ap_req(_: krb5_context, _: *mut krb5_ap_req);
        #[no_mangle]
        #[c2rust::src_loc = "2156:1"]
        pub fn krb5_free_ap_rep(_: krb5_context, _: *mut krb5_ap_rep);
        #[no_mangle]
        #[c2rust::src_loc = "2157:1"]
        pub fn krb5_free_cred(_: krb5_context, _: *mut krb5_cred);
        #[no_mangle]
        #[c2rust::src_loc = "2158:1"]
        pub fn krb5_free_cred_enc_part(_: krb5_context,
                                       _: *mut krb5_cred_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "2161:1"]
        pub fn krb5_free_enc_data(_: krb5_context, _: *mut krb5_enc_data);
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:27"]
pub mod k5_int_pkinit_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * COPYRIGHT (C) 2006
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
    /*
 * pkinit structures
 */
    /* PKAuthenticator */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:16"]
    pub struct _krb5_pk_authenticator {
        pub cusec: krb5_int32,
        pub ctime: krb5_timestamp,
        pub nonce: krb5_int32,
        pub paChecksum: krb5_checksum,
        pub freshnessToken: *mut krb5_data,
    }
    #[c2rust::src_loc = "40:1"]
    pub type krb5_pk_authenticator = _krb5_pk_authenticator;
    /* AlgorithmIdentifier */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct _krb5_algorithm_identifier {
        pub algorithm: krb5_data,
        pub parameters: krb5_data,
    }
    #[c2rust::src_loc = "49:1"]
    pub type krb5_algorithm_identifier = _krb5_algorithm_identifier;
    /* Optional */
    /* SubjectPublicKeyInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct _krb5_subject_pk_info {
        pub algorithm: krb5_algorithm_identifier,
        pub subjectPublicKey: krb5_data,
    }
    #[c2rust::src_loc = "55:1"]
    pub type krb5_subject_pk_info = _krb5_subject_pk_info;
    /* BIT STRING */
    /* * AuthPack from RFC 4556*/
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:16"]
    pub struct _krb5_auth_pack {
        pub pkAuthenticator: krb5_pk_authenticator,
        pub clientPublicValue: *mut krb5_subject_pk_info,
        pub supportedCMSTypes: *mut *mut krb5_algorithm_identifier,
        pub clientDHNonce: krb5_data,
        pub supportedKDFs: *mut *mut krb5_data,
    }
    #[c2rust::src_loc = "61:1"]
    pub type krb5_auth_pack = _krb5_auth_pack;
    /* OIDs of KDFs; OPTIONAL */
    /* ExternalPrincipalIdentifier */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct _krb5_external_principal_identifier {
        pub subjectName: krb5_data,
        pub issuerAndSerialNumber: krb5_data,
        pub subjectKeyIdentifier: krb5_data,
    }
    #[c2rust::src_loc = "70:1"]
    pub type krb5_external_principal_identifier
        =
        _krb5_external_principal_identifier;
    /* Optional */
    /* PA-PK-AS-REQ (rfc4556 -- PA TYPE 16) */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:16"]
    pub struct _krb5_pa_pk_as_req {
        pub signedAuthPack: krb5_data,
        pub trustedCertifiers: *mut *mut krb5_external_principal_identifier,
        pub kdcPkId: krb5_data,
    }
    #[c2rust::src_loc = "77:1"]
    pub type krb5_pa_pk_as_req = _krb5_pa_pk_as_req;
    /* Optional */
    /* * Pkinit DHRepInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:16"]
    pub struct _krb5_dh_rep_info {
        pub dhSignedData: krb5_data,
        pub serverDHNonce: krb5_data,
        pub kdfID: *mut krb5_data,
    }
    #[c2rust::src_loc = "84:1"]
    pub type krb5_dh_rep_info = _krb5_dh_rep_info;
    /* OID of selected KDF OPTIONAL */
    /* KDCDHKeyInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:16"]
    pub struct _krb5_kdc_dh_key_info {
        pub subjectPublicKey: krb5_data,
        pub nonce: krb5_int32,
        pub dhKeyExpiration: krb5_timestamp,
    }
    #[c2rust::src_loc = "91:1"]
    pub type krb5_kdc_dh_key_info = _krb5_kdc_dh_key_info;
    /* Optional */
    /* ReplyKeyPack */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:16"]
    pub struct _krb5_reply_key_pack {
        pub replyKey: krb5_keyblock,
        pub asChecksum: krb5_checksum,
    }
    #[c2rust::src_loc = "98:1"]
    pub type krb5_reply_key_pack = _krb5_reply_key_pack;
    /* PA-PK-AS-REP (rfc4556 -- PA TYPE 17) */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "104:16"]
    pub struct _krb5_pa_pk_as_rep {
        pub choice: krb5_pa_pk_as_rep_selection,
        pub u: krb5_pa_pk_as_rep_choices,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:11"]
    pub union krb5_pa_pk_as_rep_choices {
        pub dh_Info: krb5_dh_rep_info,
        pub encKeyPack: krb5_data,
    }
    #[c2rust::src_loc = "105:5"]
    pub type krb5_pa_pk_as_rep_selection = libc::c_int;
    #[c2rust::src_loc = "108:9"]
    pub const choice_pa_pk_as_rep_encKeyPack: krb5_pa_pk_as_rep_selection = 1;
    #[c2rust::src_loc = "107:9"]
    pub const choice_pa_pk_as_rep_dhInfo: krb5_pa_pk_as_rep_selection = 0;
    #[c2rust::src_loc = "106:9"]
    pub const choice_pa_pk_as_rep_UNKNOWN: krb5_pa_pk_as_rep_selection = -1;
    #[c2rust::src_loc = "104:1"]
    pub type krb5_pa_pk_as_rep = _krb5_pa_pk_as_rep;
    use super::krb5_h::{krb5_int32, krb5_timestamp, krb5_checksum, krb5_data,
                        krb5_keyblock};
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:29"]
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
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-spake.h:28"]
pub mod k5_spake_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:16"]
    pub struct krb5_spake_factor_st {
        pub type_0: int32_t,
        pub data: *mut krb5_data,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-spake.h - SPAKE preauth mech declarations */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
 * The SPAKE preauth mechanism allows long-term client keys to be used for
 * preauthentication without exposing them to offline dictionary attacks.  The
 * negotiated key can also be used for second-factor authentication.  This
 * header file declares structures and encoder/decoder functions for the
 * mechanism's padata messages.
 */
    /* SPAKESecondFactor is contained within a SPAKEChallenge, SPAKEResponse, or
 * EncryptedData message and contains a second-factor challenge or response. */
    #[c2rust::src_loc = "48:1"]
    pub type krb5_spake_factor = krb5_spake_factor_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct krb5_spake_support_st {
        pub ngroups: int32_t,
        pub groups: *mut int32_t,
    }
    /* SPAKESupport is sent from the client to the KDC to indicate which group the
 * client supports. */
    #[c2rust::src_loc = "55:1"]
    pub type krb5_spake_support = krb5_spake_support_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:16"]
    pub struct krb5_spake_challenge_st {
        pub group: int32_t,
        pub pubkey: krb5_data,
        pub factors: *mut *mut krb5_spake_factor,
    }
    /* SPAKEChallenge is sent from the KDC to the client to communicate its group
 * selection, public value, and second-factor challenge options. */
    #[c2rust::src_loc = "62:1"]
    pub type krb5_spake_challenge = krb5_spake_challenge_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct krb5_spake_response_st {
        pub pubkey: krb5_data,
        pub factor: krb5_enc_data,
    }
    /* SPAKEResponse is sent from the client to the KDC to communicate its public
 * value and encrypted second-factor response. */
    #[c2rust::src_loc = "70:1"]
    pub type krb5_spake_response = krb5_spake_response_st;
    #[c2rust::src_loc = "75:1"]
    pub type krb5_spake_msgtype = libc::c_int;
    #[c2rust::src_loc = "80:5"]
    pub const SPAKE_MSGTYPE_ENCDATA: krb5_spake_msgtype = 3;
    #[c2rust::src_loc = "79:5"]
    pub const SPAKE_MSGTYPE_RESPONSE: krb5_spake_msgtype = 2;
    #[c2rust::src_loc = "78:5"]
    pub const SPAKE_MSGTYPE_CHALLENGE: krb5_spake_msgtype = 1;
    #[c2rust::src_loc = "77:5"]
    pub const SPAKE_MSGTYPE_SUPPORT: krb5_spake_msgtype = 0;
    #[c2rust::src_loc = "76:5"]
    pub const SPAKE_MSGTYPE_UNKNOWN: krb5_spake_msgtype = -1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:16"]
    pub struct krb5_pa_spake_st {
        pub choice: krb5_spake_msgtype,
        pub u: krb5_spake_message_choices,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "87:11"]
    pub union krb5_spake_message_choices {
        pub support: krb5_spake_support,
        pub challenge: krb5_spake_challenge,
        pub response: krb5_spake_response,
        pub encdata: krb5_enc_data,
    }
    /* PA-SPAKE is a choice among the message types which can appear in a PA-SPAKE
 * padata element. */
    #[c2rust::src_loc = "85:1"]
    pub type krb5_pa_spake = krb5_pa_spake_st;
    use super::stdint_intn_h::int32_t;
    use super::krb5_h::{krb5_data, krb5_enc_data, krb5_error_code,
                        krb5_context};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn decode_krb5_spake_factor(code: *const krb5_data,
                                        val_out: *mut *mut krb5_spake_factor)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "99:1"]
        pub fn k5_free_spake_factor(context: krb5_context,
                                    val: *mut krb5_spake_factor);
        #[no_mangle]
        #[c2rust::src_loc = "103:1"]
        pub fn decode_krb5_pa_spake(code: *const krb5_data,
                                    val_out: *mut *mut krb5_pa_spake)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "105:1"]
        pub fn k5_free_pa_spake(context: krb5_context,
                                val: *mut krb5_pa_spake);
    }
    /* K5_SPAKE_H */
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/asn.1/ktest.h:29"]
pub mod ktest_h {
    use super::krb5_h::{krb5_data, krb5_authenticator, krb5_principal,
                        krb5_keyblock, krb5_ticket, krb5_enc_data,
                        krb5_enc_tkt_part, krb5_authdata,
                        krb5_enc_kdc_rep_part, krb5_kdc_req, krb5_kdc_rep,
                        krb5_pa_data, krb5_ap_req, krb5_ap_rep,
                        krb5_ap_rep_enc_part, krb5_cred, krb5_cred_enc_part,
                        krb5_error, krb5_address, krb5_checksum};
    use super::k5_int_h::{krb5_safe, krb5_priv, krb5_priv_enc_part,
                          krb5_etype_info_entry, krb5_pa_enc_ts,
                          krb5_sam_challenge_2, krb5_sam_challenge_2_body,
                          krb5_pa_for_user, krb5_pa_s4u_x509_user,
                          krb5_ad_kdcissued, krb5_ad_signedpath,
                          krb5_iakerb_header, krb5_iakerb_finished,
                          krb5_fast_response, krb5_otp_tokeninfo,
                          krb5_pa_otp_challenge, krb5_pa_otp_req,
                          krb5_kkdcp_message, krb5_cammac,
                          krb5_secure_cookie};
    use super::k5_int_pkinit_h::{krb5_pa_pk_as_req, krb5_pa_pk_as_rep,
                                 krb5_auth_pack, krb5_kdc_dh_key_info,
                                 krb5_reply_key_pack};
    use super::k5_spake_h::{krb5_spake_factor, krb5_pa_spake};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/asn.1/ktest.h */
/*
 * Copyright (C) 1994 by the Massachusetts Institute of Technology.
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
        /* Fri Jun 10  6:03:17 GMT 1994 */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn ktest_make_sample_data(d: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "42:1"]
        pub fn ktest_make_sample_authenticator(a: *mut krb5_authenticator);
        #[no_mangle]
        #[c2rust::src_loc = "43:1"]
        pub fn ktest_make_sample_principal(p: *mut krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "45:1"]
        pub fn ktest_make_sample_keyblock(kb: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "46:1"]
        pub fn ktest_make_sample_ticket(tkt: *mut krb5_ticket);
        #[no_mangle]
        #[c2rust::src_loc = "47:1"]
        pub fn ktest_make_sample_enc_data(ed: *mut krb5_enc_data);
        #[no_mangle]
        #[c2rust::src_loc = "48:1"]
        pub fn ktest_make_sample_enc_tkt_part(etp: *mut krb5_enc_tkt_part);
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn ktest_make_sample_authorization_data(ad:
                                                        *mut *mut *mut krb5_authdata);
        #[no_mangle]
        #[c2rust::src_loc = "55:1"]
        pub fn ktest_make_sample_enc_kdc_rep_part(ekr:
                                                      *mut krb5_enc_kdc_rep_part);
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn ktest_make_sample_kdc_req(kr: *mut krb5_kdc_req);
        #[no_mangle]
        #[c2rust::src_loc = "60:1"]
        pub fn ktest_make_sample_kdc_rep(kdcr: *mut krb5_kdc_rep);
        #[no_mangle]
        #[c2rust::src_loc = "61:1"]
        pub fn ktest_make_sample_pa_data_array(pad:
                                                   *mut *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "62:1"]
        pub fn ktest_make_sample_empty_pa_data_array(pad:
                                                         *mut *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn ktest_make_sample_ap_req(ar: *mut krb5_ap_req);
        #[no_mangle]
        #[c2rust::src_loc = "65:1"]
        pub fn ktest_make_sample_ap_rep(ar: *mut krb5_ap_rep);
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn ktest_make_sample_ap_rep_enc_part(arep:
                                                     *mut krb5_ap_rep_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn ktest_make_sample_kdc_req_body(krb: *mut krb5_kdc_req);
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn ktest_make_sample_safe(s: *mut krb5_safe);
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn ktest_make_sample_priv(p: *mut krb5_priv);
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn ktest_make_sample_priv_enc_part(pep: *mut krb5_priv_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn ktest_make_sample_cred(c: *mut krb5_cred);
        #[no_mangle]
        #[c2rust::src_loc = "72:1"]
        pub fn ktest_make_sample_cred_enc_part(cep: *mut krb5_cred_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn ktest_make_sample_error(kerr: *mut krb5_error);
        #[no_mangle]
        #[c2rust::src_loc = "78:1"]
        pub fn ktest_make_sample_etype_info(p:
                                                *mut *mut *mut krb5_etype_info_entry);
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn ktest_make_sample_etype_info2(p:
                                                 *mut *mut *mut krb5_etype_info_entry);
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn ktest_make_sample_pa_enc_ts(am: *mut krb5_pa_enc_ts);
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn ktest_make_sample_sam_challenge_2(p:
                                                     *mut krb5_sam_challenge_2);
        #[no_mangle]
        #[c2rust::src_loc = "82:1"]
        pub fn ktest_make_sample_sam_challenge_2_body(p:
                                                          *mut krb5_sam_challenge_2_body);
        #[no_mangle]
        #[c2rust::src_loc = "85:1"]
        pub fn ktest_make_sample_pa_for_user(p: *mut krb5_pa_for_user);
        #[no_mangle]
        #[c2rust::src_loc = "86:1"]
        pub fn ktest_make_sample_pa_s4u_x509_user(p:
                                                      *mut krb5_pa_s4u_x509_user);
        #[no_mangle]
        #[c2rust::src_loc = "87:1"]
        pub fn ktest_make_sample_ad_kdcissued(p: *mut krb5_ad_kdcissued);
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn ktest_make_sample_ad_signedpath(p: *mut krb5_ad_signedpath);
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn ktest_make_sample_iakerb_header(p: *mut krb5_iakerb_header);
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn ktest_make_sample_iakerb_finished(p:
                                                     *mut krb5_iakerb_finished);
        #[no_mangle]
        #[c2rust::src_loc = "92:1"]
        pub fn ktest_make_sample_fast_response(p: *mut krb5_fast_response);
        #[no_mangle]
        #[c2rust::src_loc = "95:1"]
        pub fn ktest_make_minimal_otp_tokeninfo(p: *mut krb5_otp_tokeninfo);
        #[no_mangle]
        #[c2rust::src_loc = "96:1"]
        pub fn ktest_make_maximal_otp_tokeninfo(p: *mut krb5_otp_tokeninfo);
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn ktest_make_minimal_pa_otp_challenge(p:
                                                       *mut krb5_pa_otp_challenge);
        #[no_mangle]
        #[c2rust::src_loc = "98:1"]
        pub fn ktest_make_maximal_pa_otp_challenge(p:
                                                       *mut krb5_pa_otp_challenge);
        #[no_mangle]
        #[c2rust::src_loc = "99:1"]
        pub fn ktest_make_minimal_pa_otp_req(p: *mut krb5_pa_otp_req);
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn ktest_make_maximal_pa_otp_req(p: *mut krb5_pa_otp_req);
        #[no_mangle]
        #[c2rust::src_loc = "103:1"]
        pub fn ktest_make_sample_pa_pk_as_req(p: *mut krb5_pa_pk_as_req);
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn ktest_make_sample_pa_pk_as_rep_dhInfo(p:
                                                         *mut krb5_pa_pk_as_rep);
        #[no_mangle]
        #[c2rust::src_loc = "105:1"]
        pub fn ktest_make_sample_pa_pk_as_rep_encKeyPack(p:
                                                             *mut krb5_pa_pk_as_rep);
        #[no_mangle]
        #[c2rust::src_loc = "106:1"]
        pub fn ktest_make_sample_auth_pack(p: *mut krb5_auth_pack);
        #[no_mangle]
        #[c2rust::src_loc = "107:1"]
        pub fn ktest_make_sample_kdc_dh_key_info(p:
                                                     *mut krb5_kdc_dh_key_info);
        #[no_mangle]
        #[c2rust::src_loc = "108:1"]
        pub fn ktest_make_sample_reply_key_pack(p: *mut krb5_reply_key_pack);
        #[no_mangle]
        #[c2rust::src_loc = "117:1"]
        pub fn ktest_make_sample_kkdcp_message(p: *mut krb5_kkdcp_message);
        #[no_mangle]
        #[c2rust::src_loc = "118:1"]
        pub fn ktest_make_minimal_cammac(p: *mut krb5_cammac);
        #[no_mangle]
        #[c2rust::src_loc = "119:1"]
        pub fn ktest_make_maximal_cammac(p: *mut krb5_cammac);
        #[no_mangle]
        #[c2rust::src_loc = "120:1"]
        pub fn ktest_make_sample_secure_cookie(p: *mut krb5_secure_cookie);
        #[no_mangle]
        #[c2rust::src_loc = "121:1"]
        pub fn ktest_make_minimal_spake_factor(p: *mut krb5_spake_factor);
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn ktest_make_maximal_spake_factor(p: *mut krb5_spake_factor);
        #[no_mangle]
        #[c2rust::src_loc = "123:1"]
        pub fn ktest_make_support_pa_spake(p: *mut krb5_pa_spake);
        #[no_mangle]
        #[c2rust::src_loc = "124:1"]
        pub fn ktest_make_challenge_pa_spake(p: *mut krb5_pa_spake);
        #[no_mangle]
        #[c2rust::src_loc = "125:1"]
        pub fn ktest_make_response_pa_spake(p: *mut krb5_pa_spake);
        #[no_mangle]
        #[c2rust::src_loc = "126:1"]
        pub fn ktest_make_encdata_pa_spake(p: *mut krb5_pa_spake);
        /*----------------------------------------------------------------------*/
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn ktest_empty_authorization_data(ad: *mut *mut krb5_authdata);
        #[no_mangle]
        #[c2rust::src_loc = "131:1"]
        pub fn ktest_destroy_authorization_data(ad:
                                                    *mut *mut *mut krb5_authdata);
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn ktest_destroy_addresses(a: *mut *mut *mut krb5_address);
        #[no_mangle]
        #[c2rust::src_loc = "135:1"]
        pub fn ktest_destroy_address(a: *mut *mut krb5_address);
        #[no_mangle]
        #[c2rust::src_loc = "137:1"]
        pub fn ktest_destroy_pa_data_array(pad: *mut *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "141:1"]
        pub fn ktest_empty_data(d: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "142:1"]
        pub fn ktest_destroy_principal(p: *mut krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "143:1"]
        pub fn ktest_destroy_checksum(cs: *mut *mut krb5_checksum);
        #[no_mangle]
        #[c2rust::src_loc = "144:1"]
        pub fn ktest_empty_keyblock(kb: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn ktest_destroy_keyblock(kb: *mut *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "146:1"]
        pub fn ktest_destroy_authdata(ad: *mut *mut krb5_authdata);
        #[no_mangle]
        #[c2rust::src_loc = "148:1"]
        pub fn ktest_destroy_sequence_of_ticket(sot:
                                                    *mut *mut *mut krb5_ticket);
        #[no_mangle]
        #[c2rust::src_loc = "209:1"]
        pub fn ktest_empty_pa_spake(p: *mut krb5_pa_spake);
        #[no_mangle]
        #[c2rust::src_loc = "208:1"]
        pub fn ktest_empty_spake_factor(p: *mut krb5_spake_factor);
        #[no_mangle]
        #[c2rust::src_loc = "207:1"]
        pub fn ktest_empty_secure_cookie(p: *mut krb5_secure_cookie);
        #[no_mangle]
        #[c2rust::src_loc = "206:1"]
        pub fn ktest_empty_cammac(p: *mut krb5_cammac);
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn ktest_empty_kkdcp_message(p: *mut krb5_kkdcp_message);
        #[no_mangle]
        #[c2rust::src_loc = "170:1"]
        pub fn ktest_empty_priv(p: *mut krb5_priv);
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn ktest_empty_reply_key_pack(p: *mut krb5_reply_key_pack);
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn ktest_empty_kdc_dh_key_info(p: *mut krb5_kdc_dh_key_info);
        #[no_mangle]
        #[c2rust::src_loc = "194:1"]
        pub fn ktest_empty_auth_pack(p: *mut krb5_auth_pack);
        #[no_mangle]
        #[c2rust::src_loc = "193:1"]
        pub fn ktest_empty_pa_pk_as_rep(p: *mut krb5_pa_pk_as_rep);
        #[no_mangle]
        #[c2rust::src_loc = "192:1"]
        pub fn ktest_empty_pa_pk_as_req(p: *mut krb5_pa_pk_as_req);
        #[no_mangle]
        #[c2rust::src_loc = "189:1"]
        pub fn ktest_empty_pa_otp_req(p: *mut krb5_pa_otp_req);
        #[no_mangle]
        #[c2rust::src_loc = "188:1"]
        pub fn ktest_empty_pa_otp_challenge(p: *mut krb5_pa_otp_challenge);
        #[no_mangle]
        #[c2rust::src_loc = "187:1"]
        pub fn ktest_empty_otp_tokeninfo(p: *mut krb5_otp_tokeninfo);
        #[no_mangle]
        #[c2rust::src_loc = "186:1"]
        pub fn ktest_empty_fast_response(p: *mut krb5_fast_response);
        #[no_mangle]
        #[c2rust::src_loc = "185:1"]
        pub fn ktest_empty_iakerb_finished(p: *mut krb5_iakerb_finished);
        #[no_mangle]
        #[c2rust::src_loc = "184:1"]
        pub fn ktest_empty_iakerb_header(p: *mut krb5_iakerb_header);
        #[no_mangle]
        #[c2rust::src_loc = "183:1"]
        pub fn ktest_empty_ad_signedpath(p: *mut krb5_ad_signedpath);
        #[no_mangle]
        #[c2rust::src_loc = "181:1"]
        pub fn ktest_empty_ad_kdcissued(p: *mut krb5_ad_kdcissued);
        #[no_mangle]
        #[c2rust::src_loc = "180:1"]
        pub fn ktest_empty_pa_s4u_x509_user(p: *mut krb5_pa_s4u_x509_user);
        #[no_mangle]
        #[c2rust::src_loc = "179:1"]
        pub fn ktest_empty_pa_for_user(p: *mut krb5_pa_for_user);
        #[no_mangle]
        #[c2rust::src_loc = "176:1"]
        pub fn ktest_empty_sam_challenge_2_body(p:
                                                    *mut krb5_sam_challenge_2_body);
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn ktest_empty_sam_challenge_2(p: *mut krb5_sam_challenge_2);
        #[no_mangle]
        #[c2rust::src_loc = "174:1"]
        pub fn ktest_empty_ap_rep_enc_part(arep: *mut krb5_ap_rep_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "172:1"]
        pub fn ktest_empty_cred(c: *mut krb5_cred);
        #[no_mangle]
        #[c2rust::src_loc = "166:1"]
        pub fn ktest_empty_cred_enc_part(cep: *mut krb5_cred_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "165:1"]
        pub fn ktest_empty_ap_req(ar: *mut krb5_ap_req);
        #[no_mangle]
        #[c2rust::src_loc = "169:1"]
        pub fn ktest_empty_safe(s: *mut krb5_safe);
        #[no_mangle]
        #[c2rust::src_loc = "171:1"]
        pub fn ktest_empty_priv_enc_part(pep: *mut krb5_priv_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "164:1"]
        pub fn ktest_empty_ap_rep(ar: *mut krb5_ap_rep);
        #[no_mangle]
        #[c2rust::src_loc = "162:1"]
        pub fn ktest_empty_enc_kdc_rep_part(ekr: *mut krb5_enc_kdc_rep_part);
        #[no_mangle]
        #[c2rust::src_loc = "160:1"]
        pub fn ktest_empty_enc_tkt_part(etp: *mut krb5_enc_tkt_part);
        #[no_mangle]
        #[c2rust::src_loc = "159:1"]
        pub fn ktest_empty_authenticator(a: *mut krb5_authenticator);
        #[no_mangle]
        #[c2rust::src_loc = "157:1"]
        pub fn ktest_empty_kdc_rep(kr: *mut krb5_kdc_rep);
        #[no_mangle]
        #[c2rust::src_loc = "156:1"]
        pub fn ktest_empty_kdc_req(kr: *mut krb5_kdc_req);
        #[no_mangle]
        #[c2rust::src_loc = "154:1"]
        pub fn ktest_destroy_etype_info(info:
                                            *mut *mut krb5_etype_info_entry);
        #[no_mangle]
        #[c2rust::src_loc = "153:1"]
        pub fn ktest_destroy_etype_info_entry(i: *mut krb5_etype_info_entry);
        #[no_mangle]
        #[c2rust::src_loc = "152:1"]
        pub fn ktest_empty_error(kerr: *mut krb5_error);
        #[no_mangle]
        #[c2rust::src_loc = "151:1"]
        pub fn ktest_destroy_enc_data(ed: *mut krb5_enc_data);
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn ktest_empty_ticket(tkt: *mut krb5_ticket);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/asn.1/utility.h:31"]
pub mod utility_h {
    use super::krb5_h::{krb5_data, krb5_error_code, krb5_context,
                        krb5_auth_context, krb5_enctype, krb5_cksumtype,
                        krb5_octet, krb5_principal_data, krb5_kdc_req,
                        krb5_prompt_type};
    use super::k5_int_h::{krb5int_access, ldap_seqof_key_data};
    use super::stdint_intn_h::int64_t;
    use super::stddef_h::size_t;
    use super::k5_int_pkinit_h::{krb5_auth_pack, krb5_kdc_dh_key_info,
                                 krb5_pa_pk_as_rep, krb5_pa_pk_as_req,
                                 krb5_reply_key_pack,
                                 krb5_algorithm_identifier,
                                 krb5_external_principal_identifier};
    extern "C" {
        /* effects  Parses character string *s into krb5_data *d. */
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn krb5_data_hex_parse(d: *mut krb5_data, s: *const libc::c_char)
         -> krb5_error_code;
        /* requires  *s is the string representation of a sequence of
              hexadecimal octets.  (e.g. "02 01 00")
   effects  Parses *s into krb5_data *d. */
        #[no_mangle]
        #[c2rust::src_loc = "54:23"]
        pub static mut acc: krb5int_access;
        #[no_mangle]
        #[c2rust::src_loc = "55:1"]
        pub fn init_access(progname: *const libc::c_char);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/asn.1/ktest_equal.h:32"]
pub mod ktest_equal_h {
    use super::krb5_h::{krb5_authenticator, krb5_principal_data, krb5_data,
                        krb5_ticket, krb5_enc_tkt_part, krb5_enc_data,
                        krb5_enc_kdc_rep_part, krb5_cred, krb5_error,
                        krb5_ap_req, krb5_ap_rep, krb5_ap_rep_enc_part,
                        krb5_cred_enc_part, krb5_kdc_rep, krb5_kdc_req,
                        krb5_keyblock, krb5_pa_data, krb5_authdata};
    use super::k5_int_h::{krb5_priv, krb5_safe, krb5_priv_enc_part,
                          krb5_pa_enc_ts, krb5_sam_challenge_2,
                          krb5_sam_challenge_2_body, krb5_etype_info_entry,
                          krb5_pa_for_user, krb5_pa_s4u_x509_user,
                          krb5_ad_kdcissued, krb5_ad_signedpath,
                          krb5_iakerb_header, krb5_iakerb_finished,
                          krb5_fast_response, krb5_otp_tokeninfo,
                          krb5_pa_otp_challenge, krb5_pa_otp_req,
                          krb5_kkdcp_message, krb5_cammac,
                          krb5_secure_cookie};
    use super::k5_int_pkinit_h::{krb5_pa_pk_as_req, krb5_pa_pk_as_rep,
                                 krb5_auth_pack, krb5_kdc_dh_key_info,
                                 krb5_reply_key_pack};
    use super::k5_spake_h::{krb5_spake_factor, krb5_pa_spake};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/asn.1/ktest_equal.h */
/*
 * Copyright (C) 1994 by the Massachusetts Institute of Technology.
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
        /* int ktest_equal_structure(krb5_structure *ref, *var) */
/* effects  Returns true (non-zero) if ref and var are
             semantically equivalent (i.e. have the same values,
             but aren't necessarily the same object).
            Returns false (zero) if ref and var differ. */
        #[no_mangle]
        #[c2rust::src_loc = "48:1"]
        pub fn ktest_equal_authenticator(ref_0: *mut krb5_authenticator,
                                         var: *mut krb5_authenticator)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn ktest_equal_principal_data(ref_0: *mut krb5_principal_data,
                                          var: *mut krb5_principal_data)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "52:1"]
        pub fn ktest_equal_data(ref_0: *mut krb5_data, var: *mut krb5_data)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn ktest_equal_ticket(ref_0: *mut krb5_ticket,
                                  var: *mut krb5_ticket) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "55:1"]
        pub fn ktest_equal_enc_tkt_part(ref_0: *mut krb5_enc_tkt_part,
                                        var: *mut krb5_enc_tkt_part)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "59:1"]
        pub fn ktest_equal_enc_data(ref_0: *mut krb5_enc_data,
                                    var: *mut krb5_enc_data) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "61:1"]
        pub fn ktest_equal_enc_kdc_rep_part(ref_0: *mut krb5_enc_kdc_rep_part,
                                            var: *mut krb5_enc_kdc_rep_part)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "62:1"]
        pub fn ktest_equal_priv(ref_0: *mut krb5_priv, var: *mut krb5_priv)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn ktest_equal_cred(ref_0: *mut krb5_cred, var: *mut krb5_cred)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn ktest_equal_error(ref_0: *mut krb5_error, var: *mut krb5_error)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "65:1"]
        pub fn ktest_equal_ap_req(ref_0: *mut krb5_ap_req,
                                  var: *mut krb5_ap_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn ktest_equal_ap_rep(ref_0: *mut krb5_ap_rep,
                                  var: *mut krb5_ap_rep) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn ktest_equal_ap_rep_enc_part(ref_0: *mut krb5_ap_rep_enc_part,
                                           var: *mut krb5_ap_rep_enc_part)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn ktest_equal_safe(ref_0: *mut krb5_safe, var: *mut krb5_safe)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn ktest_equal_enc_cred_part(ref_0: *mut krb5_cred_enc_part,
                                         var: *mut krb5_cred_enc_part)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn ktest_equal_enc_priv_part(ref_0: *mut krb5_priv_enc_part,
                                         var: *mut krb5_priv_enc_part)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn ktest_equal_as_rep(ref_0: *mut krb5_kdc_rep,
                                  var: *mut krb5_kdc_rep) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn ktest_equal_tgs_rep(ref_0: *mut krb5_kdc_rep,
                                   var: *mut krb5_kdc_rep) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "78:1"]
        pub fn ktest_equal_as_req(ref_0: *mut krb5_kdc_req,
                                  var: *mut krb5_kdc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn ktest_equal_tgs_req(ref_0: *mut krb5_kdc_req,
                                   var: *mut krb5_kdc_req) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn ktest_equal_kdc_req_body(ref_0: *mut krb5_kdc_req,
                                        var: *mut krb5_kdc_req)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn ktest_equal_encryption_key(ref_0: *mut krb5_keyblock,
                                          var: *mut krb5_keyblock)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn ktest_equal_krb5_pa_enc_ts(ref_0: *mut krb5_pa_enc_ts,
                                          var: *mut krb5_pa_enc_ts)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "85:1"]
        pub fn ktest_equal_sam_challenge_2(ref_0: *mut krb5_sam_challenge_2,
                                           var: *mut krb5_sam_challenge_2)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "86:1"]
        pub fn ktest_equal_sam_challenge_2_body(ref_0:
                                                    *mut krb5_sam_challenge_2_body,
                                                var:
                                                    *mut krb5_sam_challenge_2_body)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn ktest_equal_sequence_of_pa_data(ref_0: *mut *mut krb5_pa_data,
                                               var: *mut *mut krb5_pa_data)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "108:1"]
        pub fn ktest_equal_authorization_data(ref_0: *mut *mut krb5_authdata,
                                              var: *mut *mut krb5_authdata)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn ktest_equal_etype_info(ref_0: *mut *mut krb5_etype_info_entry,
                                      var: *mut *mut krb5_etype_info_entry)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "117:1"]
        pub fn ktest_equal_pa_for_user(ref_0: *mut krb5_pa_for_user,
                                       var: *mut krb5_pa_for_user)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "118:1"]
        pub fn ktest_equal_pa_s4u_x509_user(ref_0: *mut krb5_pa_s4u_x509_user,
                                            var: *mut krb5_pa_s4u_x509_user)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "120:1"]
        pub fn ktest_equal_ad_kdcissued(ref_0: *mut krb5_ad_kdcissued,
                                        var: *mut krb5_ad_kdcissued)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "123:1"]
        pub fn ktest_equal_ad_signedpath(ref_0: *mut krb5_ad_signedpath,
                                         var: *mut krb5_ad_signedpath)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "125:1"]
        pub fn ktest_equal_iakerb_header(ref_0: *mut krb5_iakerb_header,
                                         var: *mut krb5_iakerb_header)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "127:1"]
        pub fn ktest_equal_iakerb_finished(ref_0: *mut krb5_iakerb_finished,
                                           var: *mut krb5_iakerb_finished)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "129:1"]
        pub fn ktest_equal_fast_response(ref_0: *mut krb5_fast_response,
                                         var: *mut krb5_fast_response)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "131:1"]
        pub fn ktest_equal_otp_tokeninfo(ref_0: *mut krb5_otp_tokeninfo,
                                         var: *mut krb5_otp_tokeninfo)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "133:1"]
        pub fn ktest_equal_pa_otp_challenge(ref_0: *mut krb5_pa_otp_challenge,
                                            var: *mut krb5_pa_otp_challenge)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "135:1"]
        pub fn ktest_equal_pa_otp_req(ref_0: *mut krb5_pa_otp_req,
                                      var: *mut krb5_pa_otp_req)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "141:1"]
        pub fn ktest_equal_pa_pk_as_req(ref_0: *mut krb5_pa_pk_as_req,
                                        var: *mut krb5_pa_pk_as_req)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "142:1"]
        pub fn ktest_equal_pa_pk_as_rep(ref_0: *mut krb5_pa_pk_as_rep,
                                        var: *mut krb5_pa_pk_as_rep)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "143:1"]
        pub fn ktest_equal_auth_pack(ref_0: *mut krb5_auth_pack,
                                     var: *mut krb5_auth_pack) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "144:1"]
        pub fn ktest_equal_kdc_dh_key_info(ref_0: *mut krb5_kdc_dh_key_info,
                                           var: *mut krb5_kdc_dh_key_info)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn ktest_equal_reply_key_pack(ref_0: *mut krb5_reply_key_pack,
                                          var: *mut krb5_reply_key_pack)
         -> libc::c_int;
        /* not DISABLE_PKINIT */
        #[no_mangle]
        #[c2rust::src_loc = "148:1"]
        pub fn ktest_equal_kkdcp_message(ref_0: *mut krb5_kkdcp_message,
                                         var: *mut krb5_kkdcp_message)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn ktest_equal_cammac(ref_0: *mut krb5_cammac,
                                  var: *mut krb5_cammac) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "152:1"]
        pub fn ktest_equal_secure_cookie(ref_0: *mut krb5_secure_cookie,
                                         var: *mut krb5_secure_cookie)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "155:1"]
        pub fn ktest_equal_spake_factor(ref_0: *mut krb5_spake_factor,
                                        var: *mut krb5_spake_factor)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "156:1"]
        pub fn ktest_equal_pa_spake(ref_0: *mut krb5_pa_spake,
                                    var: *mut krb5_pa_spake) -> libc::c_int;
    }
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __int64_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_preauthtype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, krb5_auth_context, _krb5_keyblock,
                       krb5_keyblock, _krb5_checksum, krb5_checksum,
                       _krb5_enc_data, krb5_enc_data, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_transited, krb5_transited, _krb5_enc_tkt_part,
                       krb5_enc_tkt_part, _krb5_ticket, krb5_ticket,
                       _krb5_authenticator, krb5_authenticator,
                       _krb5_last_req_entry, krb5_last_req_entry,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _krb5_error, krb5_error, _krb5_ap_req, krb5_ap_req,
                       _krb5_ap_rep, krb5_ap_rep, _krb5_ap_rep_enc_part,
                       krb5_ap_rep_enc_part, _krb5_cred_info, krb5_cred_info,
                       _krb5_cred_enc_part, krb5_cred_enc_part, _krb5_cred,
                       krb5_cred, _krb5_pa_pac_req, krb5_pa_pac_req,
                       _profile_t, _krb5_auth_context, krb5_init_context,
                       krb5_free_context, krb5_free_principal,
                       krb5_free_authenticator, krb5_free_authdata,
                       krb5_free_ticket, krb5_free_error, krb5_free_keyblock,
                       krb5_free_ap_rep_enc_part, krb5_free_data,
                       krb5_free_data_contents};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_etype_info_entry,
                         krb5_etype_info_entry, krb5_etype_info,
                         _krb5_sam_challenge_2, krb5_sam_challenge_2,
                         _krb5_sam_challenge_2_body,
                         krb5_sam_challenge_2_body, _krb5_otp_tokeninfo,
                         krb5_otp_tokeninfo, _krb5_pa_otp_challenge,
                         krb5_pa_otp_challenge, _krb5_pa_otp_req,
                         krb5_pa_otp_req, _krb5_kkdcp_message,
                         krb5_kkdcp_message, _krb5_secure_cookie,
                         krb5_secure_cookie, _krb5_pa_enc_ts, krb5_pa_enc_ts,
                         _krb5_pa_for_user, krb5_pa_for_user,
                         _krb5_s4u_userid, krb5_s4u_userid,
                         _krb5_pa_s4u_x509_user, krb5_pa_s4u_x509_user,
                         _krb5_fast_finished, krb5_fast_finished,
                         _krb5_fast_response, krb5_fast_response,
                         _krb5_ad_kdcissued, krb5_ad_kdcissued,
                         _krb5_ad_signedpath, krb5_ad_signedpath,
                         _krb5_iakerb_header, krb5_iakerb_header,
                         _krb5_iakerb_finished, krb5_iakerb_finished,
                         _krb5_verifier_mac, krb5_verifier_mac, _krb5_cammac,
                         krb5_cammac, _krb5_safe, krb5_safe, _krb5_priv,
                         krb5_priv, _krb5_priv_enc_part, krb5_priv_enc_part,
                         ldap_seqof_key_data, _krb5int_access, krb5int_access,
                         make_data, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5_free_sam_challenge_2,
                         krb5_free_sam_challenge_2_body, krb5_free_pa_enc_ts,
                         krb5_free_pa_for_user, krb5_free_pa_s4u_x509_user,
                         krb5_free_fast_response, krb5_free_ad_kdcissued,
                         krb5_free_ad_signedpath, krb5_free_iakerb_header,
                         krb5_free_iakerb_finished, k5_free_otp_tokeninfo,
                         k5_free_pa_otp_challenge, k5_free_pa_otp_req,
                         k5_free_cammac, k5_free_secure_cookie,
                         krb5_free_safe, krb5_free_priv,
                         krb5_free_priv_enc_part, decode_krb5_sam_challenge_2,
                         decode_krb5_sam_challenge_2_body,
                         decode_krb5_authenticator, decode_krb5_ticket,
                         decode_krb5_encryption_key, decode_krb5_enc_tkt_part,
                         decode_krb5_enc_kdc_rep_part, decode_krb5_as_rep,
                         decode_krb5_tgs_rep, decode_krb5_ap_req,
                         decode_krb5_ap_rep, decode_krb5_ap_rep_enc_part,
                         decode_krb5_as_req, decode_krb5_tgs_req,
                         decode_krb5_kdc_req_body, decode_krb5_safe,
                         decode_krb5_priv, decode_krb5_enc_priv_part,
                         decode_krb5_cred, decode_krb5_enc_cred_part,
                         decode_krb5_error, decode_krb5_authdata,
                         decode_krb5_padata_sequence, decode_krb5_typed_data,
                         decode_krb5_etype_info, decode_krb5_etype_info2,
                         decode_krb5_enc_data, decode_krb5_pa_enc_ts,
                         decode_krb5_pa_for_user,
                         decode_krb5_pa_s4u_x509_user, decode_krb5_pa_pac_req,
                         decode_krb5_pa_fx_fast_reply,
                         decode_krb5_fast_response, decode_krb5_ad_kdcissued,
                         decode_krb5_ad_signedpath, decode_krb5_iakerb_header,
                         decode_krb5_iakerb_finished,
                         decode_krb5_otp_tokeninfo,
                         decode_krb5_pa_otp_challenge, decode_krb5_pa_otp_req,
                         decode_krb5_pa_otp_enc_req,
                         decode_krb5_kkdcp_message, decode_krb5_cammac,
                         decode_krb5_secure_cookie, krb5_free_enc_tkt_part,
                         krb5_free_kdc_req, krb5_free_kdc_rep,
                         krb5_free_enc_kdc_rep_part, krb5_free_ap_req,
                         krb5_free_ap_rep, krb5_free_cred,
                         krb5_free_cred_enc_part, krb5_free_pa_data,
                         krb5_free_enc_data,
                         krb5int_get_authdata_containee_types};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, com_err};
pub use self::k5_int_pkinit_h::{_krb5_pk_authenticator, krb5_pk_authenticator,
                                _krb5_algorithm_identifier,
                                krb5_algorithm_identifier,
                                _krb5_subject_pk_info, krb5_subject_pk_info,
                                _krb5_auth_pack, krb5_auth_pack,
                                _krb5_external_principal_identifier,
                                krb5_external_principal_identifier,
                                _krb5_pa_pk_as_req, krb5_pa_pk_as_req,
                                _krb5_dh_rep_info, krb5_dh_rep_info,
                                _krb5_kdc_dh_key_info, krb5_kdc_dh_key_info,
                                _krb5_reply_key_pack, krb5_reply_key_pack,
                                _krb5_pa_pk_as_rep, krb5_pa_pk_as_rep_choices,
                                krb5_pa_pk_as_rep_selection,
                                choice_pa_pk_as_rep_encKeyPack,
                                choice_pa_pk_as_rep_dhInfo,
                                choice_pa_pk_as_rep_UNKNOWN,
                                krb5_pa_pk_as_rep};
pub use self::kdb_h::_krb5_key_data;
pub use self::k5_spake_h::{krb5_spake_factor_st, krb5_spake_factor,
                           krb5_spake_support_st, krb5_spake_support,
                           krb5_spake_challenge_st, krb5_spake_challenge,
                           krb5_spake_response_st, krb5_spake_response,
                           krb5_spake_msgtype, SPAKE_MSGTYPE_ENCDATA,
                           SPAKE_MSGTYPE_RESPONSE, SPAKE_MSGTYPE_CHALLENGE,
                           SPAKE_MSGTYPE_SUPPORT, SPAKE_MSGTYPE_UNKNOWN,
                           krb5_pa_spake_st, krb5_spake_message_choices,
                           krb5_pa_spake, decode_krb5_spake_factor,
                           k5_free_spake_factor, decode_krb5_pa_spake,
                           k5_free_pa_spake};
use self::stdio_h::printf;
use self::stdlib_h::{exit, free};
use self::string_h::memset;
use self::ktest_h::{ktest_make_sample_data, ktest_make_sample_authenticator,
                    ktest_make_sample_principal, ktest_make_sample_keyblock,
                    ktest_make_sample_ticket, ktest_make_sample_enc_data,
                    ktest_make_sample_enc_tkt_part,
                    ktest_make_sample_authorization_data,
                    ktest_make_sample_enc_kdc_rep_part,
                    ktest_make_sample_kdc_req, ktest_make_sample_kdc_rep,
                    ktest_make_sample_pa_data_array,
                    ktest_make_sample_empty_pa_data_array,
                    ktest_make_sample_ap_req, ktest_make_sample_ap_rep,
                    ktest_make_sample_ap_rep_enc_part,
                    ktest_make_sample_kdc_req_body, ktest_make_sample_safe,
                    ktest_make_sample_priv, ktest_make_sample_priv_enc_part,
                    ktest_make_sample_cred, ktest_make_sample_cred_enc_part,
                    ktest_make_sample_error, ktest_make_sample_etype_info,
                    ktest_make_sample_etype_info2,
                    ktest_make_sample_pa_enc_ts,
                    ktest_make_sample_sam_challenge_2,
                    ktest_make_sample_sam_challenge_2_body,
                    ktest_make_sample_pa_for_user,
                    ktest_make_sample_pa_s4u_x509_user,
                    ktest_make_sample_ad_kdcissued,
                    ktest_make_sample_ad_signedpath,
                    ktest_make_sample_iakerb_header,
                    ktest_make_sample_iakerb_finished,
                    ktest_make_sample_fast_response,
                    ktest_make_minimal_otp_tokeninfo,
                    ktest_make_maximal_otp_tokeninfo,
                    ktest_make_minimal_pa_otp_challenge,
                    ktest_make_maximal_pa_otp_challenge,
                    ktest_make_minimal_pa_otp_req,
                    ktest_make_maximal_pa_otp_req,
                    ktest_make_sample_pa_pk_as_req,
                    ktest_make_sample_pa_pk_as_rep_dhInfo,
                    ktest_make_sample_pa_pk_as_rep_encKeyPack,
                    ktest_make_sample_auth_pack,
                    ktest_make_sample_kdc_dh_key_info,
                    ktest_make_sample_reply_key_pack,
                    ktest_make_sample_kkdcp_message,
                    ktest_make_minimal_cammac, ktest_make_maximal_cammac,
                    ktest_make_sample_secure_cookie,
                    ktest_make_minimal_spake_factor,
                    ktest_make_maximal_spake_factor,
                    ktest_make_support_pa_spake,
                    ktest_make_challenge_pa_spake,
                    ktest_make_response_pa_spake, ktest_make_encdata_pa_spake,
                    ktest_empty_authorization_data,
                    ktest_destroy_authorization_data, ktest_destroy_addresses,
                    ktest_destroy_address, ktest_destroy_pa_data_array,
                    ktest_empty_data, ktest_destroy_principal,
                    ktest_destroy_checksum, ktest_empty_keyblock,
                    ktest_destroy_keyblock, ktest_destroy_authdata,
                    ktest_destroy_sequence_of_ticket, ktest_empty_pa_spake,
                    ktest_empty_spake_factor, ktest_empty_secure_cookie,
                    ktest_empty_cammac, ktest_empty_kkdcp_message,
                    ktest_empty_priv, ktest_empty_reply_key_pack,
                    ktest_empty_kdc_dh_key_info, ktest_empty_auth_pack,
                    ktest_empty_pa_pk_as_rep, ktest_empty_pa_pk_as_req,
                    ktest_empty_pa_otp_req, ktest_empty_pa_otp_challenge,
                    ktest_empty_otp_tokeninfo, ktest_empty_fast_response,
                    ktest_empty_iakerb_finished, ktest_empty_iakerb_header,
                    ktest_empty_ad_signedpath, ktest_empty_ad_kdcissued,
                    ktest_empty_pa_s4u_x509_user, ktest_empty_pa_for_user,
                    ktest_empty_sam_challenge_2_body,
                    ktest_empty_sam_challenge_2, ktest_empty_ap_rep_enc_part,
                    ktest_empty_cred, ktest_empty_cred_enc_part,
                    ktest_empty_ap_req, ktest_empty_safe,
                    ktest_empty_priv_enc_part, ktest_empty_ap_rep,
                    ktest_empty_enc_kdc_rep_part, ktest_empty_enc_tkt_part,
                    ktest_empty_authenticator, ktest_empty_kdc_rep,
                    ktest_empty_kdc_req, ktest_destroy_etype_info,
                    ktest_destroy_etype_info_entry, ktest_empty_error,
                    ktest_destroy_enc_data, ktest_empty_ticket};
use self::utility_h::{krb5_data_hex_parse, acc, init_access};
use self::ktest_equal_h::{ktest_equal_authenticator,
                          ktest_equal_principal_data, ktest_equal_data,
                          ktest_equal_ticket, ktest_equal_enc_tkt_part,
                          ktest_equal_enc_data, ktest_equal_enc_kdc_rep_part,
                          ktest_equal_priv, ktest_equal_cred,
                          ktest_equal_error, ktest_equal_ap_req,
                          ktest_equal_ap_rep, ktest_equal_ap_rep_enc_part,
                          ktest_equal_safe, ktest_equal_enc_cred_part,
                          ktest_equal_enc_priv_part, ktest_equal_as_rep,
                          ktest_equal_tgs_rep, ktest_equal_as_req,
                          ktest_equal_tgs_req, ktest_equal_kdc_req_body,
                          ktest_equal_encryption_key,
                          ktest_equal_krb5_pa_enc_ts,
                          ktest_equal_sam_challenge_2,
                          ktest_equal_sam_challenge_2_body,
                          ktest_equal_sequence_of_pa_data,
                          ktest_equal_authorization_data,
                          ktest_equal_etype_info, ktest_equal_pa_for_user,
                          ktest_equal_pa_s4u_x509_user,
                          ktest_equal_ad_kdcissued, ktest_equal_ad_signedpath,
                          ktest_equal_iakerb_header,
                          ktest_equal_iakerb_finished,
                          ktest_equal_fast_response,
                          ktest_equal_otp_tokeninfo,
                          ktest_equal_pa_otp_challenge,
                          ktest_equal_pa_otp_req, ktest_equal_pa_pk_as_req,
                          ktest_equal_pa_pk_as_rep, ktest_equal_auth_pack,
                          ktest_equal_kdc_dh_key_info,
                          ktest_equal_reply_key_pack,
                          ktest_equal_kkdcp_message, ktest_equal_cammac,
                          ktest_equal_secure_cookie, ktest_equal_spake_factor,
                          ktest_equal_pa_spake};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/asn.1/krb5_decode_test.c */
/*
 * Copyright (C) 1994 by the Massachusetts Institute of Technology.
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
#[no_mangle]
#[c2rust::src_loc = "37:14"]
pub static mut test_context: krb5_context =
    0 as *const _krb5_context as *mut _krb5_context;
#[no_mangle]
#[c2rust::src_loc = "38:5"]
pub static mut error_count: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "57:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut code: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut retval: krb5_error_code = 0;
    retval = krb5_init_context(&mut test_context);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while initializing krb5\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    init_access(*argv.offset(0 as libc::c_int as isize));
    /* ***************************************************************/
    /* decode_krb5_authenticator */
    let mut ref_0: krb5_authenticator =
        krb5_authenticator{magic: 0,
                           client: 0 as *mut krb5_principal_data,
                           checksum: 0 as *mut krb5_checksum,
                           cusec: 0,
                           ctime: 0,
                           subkey: 0 as *mut krb5_keyblock,
                           seq_number: 0,
                           authorization_data: 0 as *mut *mut krb5_authdata,};
    let mut var: *mut krb5_authenticator = 0 as *mut krb5_authenticator;
    ktest_make_sample_authenticator(&mut ref_0);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"62 81 A1 30 81 9E A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 0F 30 0D A0 03 02 01 01 A1 06 04 04 31 32 33 34 A4 05 02 03 01 E2 40 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A6 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A7 03 02 01 11 A8 24 30 22 30 0F A0 03 02 01 01 A1 08 04 06 66 6F 6F 62 61 72 30 0F A0 03 02 01 01 A1 08 04 06 66 6F 6F 62 61 72\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_authenticator(&mut code, &mut var);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_authenticator(&mut ref_0, var) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"authenticator\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_authenticator(test_context, var);
    ref_0.seq_number = 0xffffff80 as libc::c_uint;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"62 81 A1 30 81 9E   A0 03 02 01 05   A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55   A2 1A 30 18      A0 03 02 01 01      A1 11 30 0F         1B 06 68 66 74 73 61 69         1B 05 65 78 74 72 61   A3 0F 30 0D      A0 03 02 01 01      A1 06 04 04 31 32 33 34   A4 05 02 03 01 E2 40   A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A   A6 13 30 11      A0 03 02 01 01      A1 0A 04 08 31 32 33 34 35 36 37 38   A7 03 02 01 80   A8 24 30 22      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_authenticator(&mut code, &mut var);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_authenticator(&mut ref_0, var) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"authenticator\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(80 -> seq-number 0xffffff80)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_authenticator(test_context, var);
    ref_0.seq_number = 0xffffffff as libc::c_uint;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"62 81 A1 30 81 9E   A0 03 02 01 05   A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55   A2 1A 30 18      A0 03 02 01 01      A1 11 30 0F         1B 06 68 66 74 73 61 69         1B 05 65 78 74 72 61   A3 0F 30 0D      A0 03 02 01 01      A1 06 04 04 31 32 33 34   A4 05 02 03 01 E2 40   A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A   A6 13 30 11      A0 03 02 01 01      A1 0A 04 08 31 32 33 34 35 36 37 38   A7 03 02 01 FF   A8 24 30 22      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_authenticator(&mut code, &mut var);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_authenticator(&mut ref_0, var) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"authenticator\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(FF -> seq-number 0xffffffff)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_authenticator(test_context, var);
    ref_0.seq_number = 0xff as libc::c_int as krb5_ui_4;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"62 81 A2 30 81 9F   A0 03 02 01 05   A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55   A2 1A 30 18      A0 03 02 01 01      A1 11 30 0F         1B 06 68 66 74 73 61 69         1B 05 65 78 74 72 61   A3 0F 30 0D      A0 03 02 01 01      A1 06 04 04 31 32 33 34   A4 05 02 03 01 E2 40   A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A   A6 13 30 11      A0 03 02 01 01      A1 0A 04 08 31 32 33 34 35 36 37 38   A7 04 02 02 00 FF   A8 24 30 22      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_authenticator(&mut code, &mut var);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_authenticator(&mut ref_0, var) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"authenticator\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(00FF -> seq-number 0xff)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_authenticator(test_context, var);
    ref_0.seq_number = 0xffffffff as libc::c_uint;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"62 81 A5 30 81 A2   A0 03 02 01 05   A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55   A2 1A 30 18      A0 03 02 01 01      A1 11 30 0F         1B 06 68 66 74 73 61 69         1B 05 65 78 74 72 61   A3 0F 30 0D      A0 03 02 01 01      A1 06 04 04 31 32 33 34   A4 05 02 03 01 E2 40   A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A   A6 13 30 11      A0 03 02 01 01      A1 0A 04 08 31 32 33 34 35 36 37 38   A7 07 02 05 00 FF FF FF FF   A8 24 30 22      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_authenticator(&mut code, &mut var);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_authenticator(&mut ref_0, var) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"authenticator\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(00FFFFFFFF -> seq-number 0xffffffff)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_authenticator(test_context, var);
    ref_0.seq_number = 0x7fffffff as libc::c_int as krb5_ui_4;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"62 81 A4 30 81 A1   A0 03 02 01 05   A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55   A2 1A 30 18      A0 03 02 01 01      A1 11 30 0F         1B 06 68 66 74 73 61 69         1B 05 65 78 74 72 61   A3 0F 30 0D      A0 03 02 01 01      A1 06 04 04 31 32 33 34   A4 05 02 03 01 E2 40   A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A   A6 13 30 11      A0 03 02 01 01      A1 0A 04 08 31 32 33 34 35 36 37 38   A7 06 02 04 7F FF FF FF   A8 24 30 22      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_authenticator(&mut code, &mut var);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_authenticator(&mut ref_0, var) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"authenticator\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(7FFFFFFF -> seq-number 0x7fffffff)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_authenticator(test_context, var);
    ref_0.seq_number = 0xffffffff as libc::c_uint;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"62 81 A4 30 81 A1   A0 03 02 01 05   A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55   A2 1A 30 18      A0 03 02 01 01      A1 11 30 0F         1B 06 68 66 74 73 61 69         1B 05 65 78 74 72 61   A3 0F 30 0D      A0 03 02 01 01      A1 06 04 04 31 32 33 34   A4 05 02 03 01 E2 40   A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A   A6 13 30 11      A0 03 02 01 01      A1 0A 04 08 31 32 33 34 35 36 37 38   A7 06 02 04 FF FF FF FF   A8 24 30 22      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72      30 0F         A0 03 02 01 01         A1 08 04 06 66 6F 6F 62 61 72\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_authenticator(&mut code, &mut var);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_authenticator(&mut ref_0, var) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"authenticator\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(FFFFFFFF -> seq-number 0xffffffff)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_authenticator(test_context, var);
    ktest_destroy_checksum(&mut ref_0.checksum);
    ktest_destroy_keyblock(&mut ref_0.subkey);
    ref_0.seq_number = 0 as libc::c_int as krb5_ui_4;
    ktest_empty_authorization_data(ref_0.authorization_data);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"62 4F 30 4D A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 05 02 03 01 E2 40 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_authenticator(&mut code, &mut var);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_authenticator(&mut ref_0, var) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"authenticator\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals empty)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_authenticator(test_context, var);
    ktest_destroy_authorization_data(&mut ref_0.authorization_data);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"62 4F 30 4D A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 05 02 03 01 E2 40 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_authenticator(&mut code, &mut var);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"authenticator\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_authenticator(&mut ref_0, var) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"authenticator\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_authenticator(test_context, var);
    ktest_empty_authenticator(&mut ref_0);
    /* ***************************************************************/
    /* decode_krb5_ticket */
    let mut ref_1: krb5_ticket =
        krb5_ticket{magic: 0,
                    server: 0 as *mut krb5_principal_data,
                    enc_part:
                        krb5_enc_data{magic: 0,
                                      enctype: 0,
                                      kvno: 0,
                                      ciphertext:
                                          krb5_data{magic: 0,
                                                    length: 0,
                                                    data:
                                                        0 as
                                                            *mut libc::c_char,},},
                    enc_part2: 0 as *mut krb5_enc_tkt_part,};
    let mut var_0: *mut krb5_ticket = 0 as *mut krb5_ticket;
    ktest_make_sample_ticket(&mut ref_1);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"ticket\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_ticket(&mut code, &mut var_0);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"ticket\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_ticket(&mut ref_1, var_0) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"ticket\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_ticket(test_context, var_0);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"61 61 30 5F A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 A4 03 02 01 01\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"ticket\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_ticket(&mut code, &mut var_0);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"ticket\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_ticket(&mut ref_1, var_0) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"ticket\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(+ trailing [4] INTEGER\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_ticket(test_context, var_0);
    /*
  "61 80 30 80 "
  "  A0 03 02 01 05 "
  "  A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 "
  "  A2 80 30 80 "
  "    A0 03 02 01 01 "
  "    A1 80 30 80 "
  "      1B 06 68 66 74 73 61 69 "
  "      1B 05 65 78 74 72 61 "
  "    00 00 00 00 "
  "  00 00 00 00 "
  "  A3 80 30 80 "
  "    A0 03 02 01 00 "
  "    A1 03 02 01 05 "
  "    A2 17 04 15 6B 72 62 41 53 4E 2E 31 "
  "      20 74 65 73 74 20 6D 65 73 73 61 67 65 "
  "  00 00 00 00"
  "00 00 00 00"
*/
    retval =
        krb5_data_hex_parse(&mut code,
                            b"61 80 30 80 A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 80 30 80 A0 03 02 01 01 A1 80 30 80 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 00 00 00 00 00 00 00 00 A3 80 30 80 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 00 00 00 00 00 00 00 00\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"ticket\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_ticket(&mut code, &mut var_0);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"ticket\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_ticket(&mut ref_1, var_0) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"ticket\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(indefinite lengths)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_ticket(test_context, var_0);
    /*
  "61 80 30 80 "
  "  A0 03 02 01 05 "
  "  A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 "
  "  A2 80 30 80 "
  "    A0 03 02 01 01 "
  "    A1 80 30 80 "
  "      1B 06 68 66 74 73 61 69 "
  "      1B 05 65 78 74 72 61 "
  "    00 00 00 00 "
  "  00 00 00 00 "
  "  A3 80 30 80 "
  "    A0 03 02 01 00 "
  "    A1 03 02 01 05 "
  "    A2 17 04 15 6B 72 62 41 53 4E 2E 31 "
  "      20 74 65 73 74 20 6D 65 73 73 61 67 65 "
  "  00 00 00 00"
  "  A4 03 02 01 01 "
  "00 00 00 00"
*/
    retval =
        krb5_data_hex_parse(&mut code,
                            b"61 80 30 80 A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 80 30 80 A0 03 02 01 01 A1 80 30 80 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 00 00 00 00 00 00 00 00 A3 80 30 80 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 00 00 00 00 A4 03 02 01 01 00 00 00 00\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"ticket\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_ticket(&mut code, &mut var_0);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"ticket\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_ticket(&mut ref_1, var_0) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"ticket\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(indefinite lengths + trailing [4] INTEGER)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_ticket(test_context, var_0);
    ktest_empty_ticket(&mut ref_1);
    /* ***************************************************************/
    /* decode_krb5_encryption_key */
    let mut ref_2: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut var_1: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    ktest_make_sample_keyblock(&mut ref_2);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 16 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A2 03 02 01 01\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(+ trailing [2] INTEGER)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 1A A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A2 07 30 05 A0 03 02 01 01\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(+ trailing [2] SEQUENCE {[0] INTEGER})\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 80 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 00 00\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(indefinite lengths)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 80 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A2 03 02 01 01 00 00\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(indefinite lengths + trailing [2] INTEGER)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 80 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A2 80 30 80 A0 03 02 01 01 00 00 00 00 00 00\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(indefinite lengths + trailing [2] SEQUENCE {[0] INTEGER})\x00"
               as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 80 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 30 80 A0 03 02 01 01 00 00 00 00\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(indefinite lengths + trailing SEQUENCE {[0] INTEGER})\x00" as
               *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    ref_2.enctype = -(1 as libc::c_int);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 11 A0 03 02 01 FF A1 0A 04 08 31 32 33 34 35 36 37 38\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(enctype = -1)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    ref_2.enctype = -(255 as libc::c_int);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 12 A0 04 02 02 FF 01 A1 0A 04 08 31 32 33 34 35 36 37 38\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(enctype = -255)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    ref_2.enctype = 255 as libc::c_int;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 12 A0 04 02 02 00 FF A1 0A 04 08 31 32 33 34 35 36 37 38\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(enctype = 255)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    ref_2.enctype =
        (2147483648 as libc::c_uint).wrapping_neg() as krb5_enctype;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 14 A0 06 02 04 80 00 00 00 A1 0A 04 08 31 32 33 34 35 36 37 38\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(enctype = -2147483648)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    ref_2.enctype = 2147483647 as libc::c_int;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 14 A0 06 02 04 7F FF FF FF A1 0A 04 08 31 32 33 34 35 36 37 38\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_encryption_key(&mut code, &mut var_1);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"encryption_key\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_encryption_key(&mut ref_2, var_1) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"encryption_key\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(enctype = 2147483647)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_keyblock(test_context, var_1);
    ktest_empty_keyblock(&mut ref_2);
    /* ***************************************************************/
    /* decode_krb5_enc_tkt_part */
    let mut ref_3: krb5_enc_tkt_part =
        krb5_enc_tkt_part{magic: 0,
                          flags: 0,
                          session: 0 as *mut krb5_keyblock,
                          client: 0 as *mut krb5_principal_data,
                          transited:
                              krb5_transited{magic: 0,
                                             tr_type: 0,
                                             tr_contents:
                                                 krb5_data{magic: 0,
                                                           length: 0,
                                                           data:
                                                               0 as
                                                                   *mut libc::c_char,},},
                          times:
                              krb5_ticket_times{authtime: 0,
                                                starttime: 0,
                                                endtime: 0,
                                                renew_till: 0,},
                          caddrs: 0 as *mut *mut krb5_address,
                          authorization_data: 0 as *mut *mut krb5_authdata,};
    let mut var_2: *mut krb5_enc_tkt_part = 0 as *mut krb5_enc_tkt_part;
    ktest_make_sample_enc_tkt_part(&mut ref_3);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"63 82 01 14 30 82 01 10 A0 07 03 05 00 FE DC BA 98 A1 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 2E 30 2C A0 03 02 01 01 A1 25 04 23 45 44 55 2C 4D 49 54 2E 2C 41 54 48 45 4E 41 2E 2C 57 41 53 48 49 4E 47 54 4F 4E 2E 45 44 55 2C 43 53 2E A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A6 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A8 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A9 20 30 1E 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 AA 24 30 22 30 0F A0 03 02 01 01 A1 08 04 06 66 6F 6F 62 61 72 30 0F A0 03 02 01 01 A1 08 04 06 66 6F 6F 62 61 72\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_tkt_part(&mut code, &mut var_2);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_tkt_part(&mut ref_3, var_2) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_enc_tkt_part(test_context, var_2);
    ref_3.times.starttime = 0 as libc::c_int;
    ref_3.times.renew_till = 0 as libc::c_int;
    ktest_destroy_address(&mut *ref_3.caddrs.offset(1 as libc::c_int as
                                                        isize));
    ktest_destroy_address(&mut *ref_3.caddrs.offset(0 as libc::c_int as
                                                        isize));
    ktest_destroy_authdata(&mut *ref_3.authorization_data.offset(1 as
                                                                     libc::c_int
                                                                     as
                                                                     isize));
    ktest_destroy_authdata(&mut *ref_3.authorization_data.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize));
    /* ISODE version fails on the empty caddrs field */
    ktest_destroy_addresses(&mut ref_3.caddrs);
    ktest_destroy_authorization_data(&mut ref_3.authorization_data);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"63 81 A5 30 81 A2 A0 07 03 05 00 FE DC BA 98 A1 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 2E 30 2C A0 03 02 01 01 A1 25 04 23 45 44 55 2C 4D 49 54 2E 2C 41 54 48 45 4E 41 2E 2C 57 41 53 48 49 4E 47 54 4F 4E 2E 45 44 55 2C 43 53 2E A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_tkt_part(&mut code, &mut var_2);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_tkt_part(&mut ref_3, var_2) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_enc_tkt_part(test_context, var_2);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"63 81 A6 30 81 A3 A0 08 03 06 02 FE DC BA 98 DC A1 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 2E 30 2C A0 03 02 01 01 A1 25 04 23 45 44 55 2C 4D 49 54 2E 2C 41 54 48 45 4E 41 2E 2C 57 41 53 48 49 4E 47 54 4F 4E 2E 45 44 55 2C 43 53 2E A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_tkt_part(&mut code, &mut var_2);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_tkt_part(&mut ref_3, var_2) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL + bitstring enlarged to 38 bits)\x00" as
               *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_enc_tkt_part(test_context, var_2);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"63 81 A6 30 81 A3 A0 08 03 06 00 FE DC BA 98 DE A1 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 2E 30 2C A0 03 02 01 01 A1 25 04 23 45 44 55 2C 4D 49 54 2E 2C 41 54 48 45 4E 41 2E 2C 57 41 53 48 49 4E 47 54 4F 4E 2E 45 44 55 2C 43 53 2E A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_tkt_part(&mut code, &mut var_2);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_tkt_part(&mut ref_3, var_2) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL + bitstring enlarged to 40 bits)\x00" as
               *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_enc_tkt_part(test_context, var_2);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"63 81 A5 30 81 A2 A0 07 03 05 03 FE DC BA 98 A1 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 2E 30 2C A0 03 02 01 01 A1 25 04 23 45 44 55 2C 4D 49 54 2E 2C 41 54 48 45 4E 41 2E 2C 57 41 53 48 49 4E 47 54 4F 4E 2E 45 44 55 2C 43 53 2E A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_tkt_part(&mut code, &mut var_2);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_tkt_part(&mut ref_3, var_2) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL + bitstring reduced to 29 bits)\x00" as *const u8
               as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_enc_tkt_part(test_context, var_2);
    ref_3.flags =
        (ref_3.flags as libc::c_uint & 0xffffff00 as libc::c_uint) as
            krb5_flags;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"63 81 A4 30 81 A1 A0 06 03 04 00 FE DC BA A1 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 2E 30 2C A0 03 02 01 01 A1 25 04 23 45 44 55 2C 4D 49 54 2E 2C 41 54 48 45 4E 41 2E 2C 57 41 53 48 49 4E 47 54 4F 4E 2E 45 44 55 2C 43 53 2E A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_tkt_part(&mut code, &mut var_2);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_tkt_part(&mut ref_3, var_2) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_tkt_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL + bitstring reduced to 24 bits)\x00" as *const u8
               as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_enc_tkt_part(test_context, var_2);
    ktest_empty_enc_tkt_part(&mut ref_3);
    /* ***************************************************************/
    /* decode_krb5_enc_kdc_rep_part */
    let mut ref_4: krb5_enc_kdc_rep_part =
        krb5_enc_kdc_rep_part{magic: 0,
                              msg_type: 0,
                              session: 0 as *mut krb5_keyblock,
                              last_req: 0 as *mut *mut krb5_last_req_entry,
                              nonce: 0,
                              key_exp: 0,
                              flags: 0,
                              times:
                                  krb5_ticket_times{authtime: 0,
                                                    starttime: 0,
                                                    endtime: 0,
                                                    renew_till: 0,},
                              server: 0 as *mut krb5_principal_data,
                              caddrs: 0 as *mut *mut krb5_address,
                              enc_padata: 0 as *mut *mut krb5_pa_data,};
    let mut var_3: *mut krb5_enc_kdc_rep_part =
        0 as *mut krb5_enc_kdc_rep_part;
    ktest_make_sample_enc_kdc_rep_part(&mut ref_4);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"7A 82 01 0E 30 82 01 0A A0 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A1 36 30 34 30 18 A0 03 02 01 FB A1 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A 30 18 A0 03 02 01 FB A1 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A2 03 02 01 2A A3 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A4 07 03 05 00 FE DC BA 98 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A6 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A8 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A9 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 AA 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 AB 20 30 1E 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_kdc_rep_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_kdc_rep_part(&mut code, &mut var_3);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_kdc_rep_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_kdc_rep_part(&mut ref_4, var_3) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_kdc_rep_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_enc_kdc_rep_part(test_context, var_3);
    ref_4.key_exp = 0 as libc::c_int;
    /* ref.times.starttime = 0;*/
    ref_4.times.starttime = ref_4.times.authtime;
    ref_4.times.renew_till = 0 as libc::c_int;
    ref_4.flags &= !(0x800000 as libc::c_int);
    ktest_destroy_addresses(&mut ref_4.caddrs);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"7A 81 B2 30 81 AF A0 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A1 36 30 34 30 18 A0 03 02 01 FB A1 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A 30 18 A0 03 02 01 FB A1 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A2 03 02 01 2A A4 07 03 05 00 FE 5C BA 98 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A9 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 AA 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_kdc_rep_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_kdc_rep_part(&mut code, &mut var_3);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_kdc_rep_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_kdc_rep_part(&mut ref_4, var_3) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_kdc_rep_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_enc_kdc_rep_part(test_context, var_3);
    ktest_empty_enc_kdc_rep_part(&mut ref_4);
    /* ***************************************************************/
    /* decode_krb5_as_rep */
    let mut ref_5: krb5_kdc_rep =
        krb5_kdc_rep{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     client: 0 as *mut krb5_principal_data,
                     ticket: 0 as *mut krb5_ticket,
                     enc_part:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     enc_part2: 0 as *mut krb5_enc_kdc_rep_part,};
    let mut var_4: *mut krb5_kdc_rep = 0 as *mut krb5_kdc_rep;
    ktest_make_sample_kdc_rep(&mut ref_5);
    ref_5.msg_type = 11 as libc::c_int as krb5_msgtype;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6B 81 EA 30 81 E7 A0 03 02 01 05 A1 03 02 01 0B A2 26 30 24 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 A3 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A4 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A5 5E 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 A6 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"as_rep\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_as_rep(&mut code, &mut var_4);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"as_rep\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_as_rep(&mut ref_5, var_4) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"as_rep\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_rep(test_context, var_4);
    /*
  6B 80 30 80
  A0 03 02 01 05
  A1 03 02 01 0B
  A2 80 30 80
  30 80
  A1 03 02 01 0D
  A2 09 04 07 70 61 2D 64 61 74 61
  00 00
  30 80
  A1 03 02 01 0D
  A2 09 04 07 70 61 2D 64 61 74 61
  00 00
  00 00 00 00
  A3 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55
  A4 80 30 80
  A0 03 02 01 01
  A1 80 30 80
  1B 06 68 66 74 73 61 69
  1B 05 65 78 74 72 61
  00 00 00 00
  00 00 00 00
  A5 80 61 80 30 80
  A0 03 02 01 05
  A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55
  A2 80 30 80
  A0 03 02 01 01
  A1 80 30 80
  1B 06 68 66 74 73 61 69
  1B 05 65 78 74 72 61
  00 00 00 00
  00 00 00 00
  A3 80 30 80
  A0 03 02 01 00
  A1 03 02 01 05
  A2 17 04 15 6B 72 62 41 53 4E 2E 31
  20 74 65 73 74 20 6D 65
  73 73 61 67 65
  00 00 00 00
  00 00 00 00 00 00
  A6 80 30 80
  A0 03 02 01 00
  A1 03 02 01 05
  A2 17 04 15 6B 72 62 41 53 4E 2E 31
  20 74 65 73 74 20 6D 65
  73 73 61 67 65
  00 00 00 00
  00 00 00 00
*/
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6B 80 30 80 A0 03 02 01 05 A1 03 02 01 0B A2 80 30 80 30 80 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 00 00 30 80 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 00 00 00 00 00 00 A3 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A4 80 30 80 A0 03 02 01 01 A1 80 30 80 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 00 00 00 00 00 00 00 00 A5 80 61 80 30 80 A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 80 30 80 A0 03 02 01 01 A1 80 30 80 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 00 00 00 00 00 00 00 00 A3 80 30 80 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 00 00 00 00 00 00 00 00 00 00 A6 80 30 80 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 00 00 00 00 00 00 00 00\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"as_rep\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_as_rep(&mut code, &mut var_4);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"as_rep\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_as_rep(&mut ref_5, var_4) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"as_rep\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(indefinite lengths)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_rep(test_context, var_4);
    ktest_destroy_pa_data_array(&mut ref_5.padata);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6B 81 C2 30 81 BF A0 03 02 01 05 A1 03 02 01 0B A3 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A4 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A5 5E 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 A6 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"as_rep\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_as_rep(&mut code, &mut var_4);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"as_rep\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_as_rep(&mut ref_5, var_4) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"as_rep\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_rep(test_context, var_4);
    ktest_empty_kdc_rep(&mut ref_5);
    /* ***************************************************************/
    /* decode_krb5_tgs_rep */
    let mut ref_6: krb5_kdc_rep =
        krb5_kdc_rep{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     client: 0 as *mut krb5_principal_data,
                     ticket: 0 as *mut krb5_ticket,
                     enc_part:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     enc_part2: 0 as *mut krb5_enc_kdc_rep_part,};
    let mut var_5: *mut krb5_kdc_rep = 0 as *mut krb5_kdc_rep;
    ktest_make_sample_kdc_rep(&mut ref_6);
    ref_6.msg_type = 13 as libc::c_int as krb5_msgtype;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6D 81 EA 30 81 E7 A0 03 02 01 05 A1 03 02 01 0D A2 26 30 24 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 A3 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A4 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A5 5E 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 A6 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"tgs_rep\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_tgs_rep(&mut code, &mut var_5);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"tgs_rep\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_tgs_rep(&mut ref_6, var_5) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"tgs_rep\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_rep(test_context, var_5);
    ktest_destroy_pa_data_array(&mut ref_6.padata);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6D 81 C2 30 81 BF A0 03 02 01 05 A1 03 02 01 0D A3 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A4 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A5 5E 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 A6 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"tgs_rep\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_tgs_rep(&mut code, &mut var_5);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"tgs_rep\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_tgs_rep(&mut ref_6, var_5) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"tgs_rep\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_rep(test_context, var_5);
    ktest_empty_kdc_rep(&mut ref_6);
    /* ***************************************************************/
    /* decode_krb5_ap_req */
    let mut ref_7: krb5_ap_req =
        krb5_ap_req{magic: 0,
                    ap_options: 0,
                    ticket: 0 as *mut krb5_ticket,
                    authenticator:
                        krb5_enc_data{magic: 0,
                                      enctype: 0,
                                      kvno: 0,
                                      ciphertext:
                                          krb5_data{magic: 0,
                                                    length: 0,
                                                    data:
                                                        0 as
                                                            *mut libc::c_char,},},};
    let mut var_6: *mut krb5_ap_req = 0 as *mut krb5_ap_req;
    ktest_make_sample_ap_req(&mut ref_7);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6E 81 9D 30 81 9A A0 03 02 01 05 A1 03 02 01 0E A2 07 03 05 00 FE DC BA 98 A3 5E 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 A4 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"ap_req\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_ap_req(&mut code, &mut var_6);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"ap_req\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_ap_req(&mut ref_7, var_6) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"ap_req\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_ap_req(test_context, var_6);
    ktest_empty_ap_req(&mut ref_7);
    /* ***************************************************************/
    /* decode_krb5_ap_rep */
    let mut ref_8: krb5_ap_rep =
        krb5_ap_rep{magic: 0,
                    enc_part:
                        krb5_enc_data{magic: 0,
                                      enctype: 0,
                                      kvno: 0,
                                      ciphertext:
                                          krb5_data{magic: 0,
                                                    length: 0,
                                                    data:
                                                        0 as
                                                            *mut libc::c_char,},},};
    let mut var_7: *mut krb5_ap_rep = 0 as *mut krb5_ap_rep;
    ktest_make_sample_ap_rep(&mut ref_8);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6F 33 30 31 A0 03 02 01 05 A1 03 02 01 0F A2 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"ap_rep\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_ap_rep(&mut code, &mut var_7);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"ap_rep\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_ap_rep(&mut ref_8, var_7) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"ap_rep\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_ap_rep(test_context, var_7);
    ktest_empty_ap_rep(&mut ref_8);
    /* ***************************************************************/
    /* decode_krb5_ap_rep_enc_part */
    let mut ref_9: krb5_ap_rep_enc_part =
        krb5_ap_rep_enc_part{magic: 0,
                             ctime: 0,
                             cusec: 0,
                             subkey: 0 as *mut krb5_keyblock,
                             seq_number: 0,};
    let mut var_8: *mut krb5_ap_rep_enc_part = 0 as *mut krb5_ap_rep_enc_part;
    ktest_make_sample_ap_rep_enc_part(&mut ref_9);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"7B 36 30 34 A0 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A1 05 02 03 01 E2 40 A2 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A3 03 02 01 11\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"ap_rep_enc_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_ap_rep_enc_part(&mut code, &mut var_8);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"ap_rep_enc_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_ap_rep_enc_part(&mut ref_9, var_8) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"ap_rep_enc_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_ap_rep_enc_part(test_context, var_8);
    ktest_destroy_keyblock(&mut ref_9.subkey);
    ref_9.seq_number = 0 as libc::c_int as krb5_ui_4;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"7B 1C 30 1A A0 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A1 05 02 03 01 E2 40\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"ap_rep_enc_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_ap_rep_enc_part(&mut code, &mut var_8);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"ap_rep_enc_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_ap_rep_enc_part(&mut ref_9, var_8) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"ap_rep_enc_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_ap_rep_enc_part(test_context, var_8);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"7B 06 30 04 A0 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A1 05 02 03 01 E2 40\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_ap_rep_enc_part(&mut code, &mut var_8);
    if retval as libc::c_long != 1859794437 as libc::c_long {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    } else { printf(b"OK: \x00" as *const u8 as *const libc::c_char); }
    printf(b"ap_rep_enc_part(optionals NULL + expect ASN1_OVERRUN for inconsistent length of timestamp)\n\x00"
               as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_ap_rep_enc_part(test_context, var_8);
    ktest_empty_ap_rep_enc_part(&mut ref_9);
    /* ***************************************************************/
    /* decode_krb5_as_req */
    let mut ref_10: krb5_kdc_req =
        krb5_kdc_req{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     kdc_options: 0,
                     client: 0 as *mut krb5_principal_data,
                     server: 0 as *mut krb5_principal_data,
                     from: 0,
                     till: 0,
                     rtime: 0,
                     nonce: 0,
                     nktypes: 0,
                     ktype: 0 as *mut krb5_enctype,
                     addresses: 0 as *mut *mut krb5_address,
                     authorization_data:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     unenc_authdata: 0 as *mut *mut krb5_authdata,
                     second_ticket: 0 as *mut *mut krb5_ticket,};
    let mut var_9: *mut krb5_kdc_req = 0 as *mut krb5_kdc_req;
    ktest_make_sample_kdc_req(&mut ref_10);
    ref_10.msg_type = 10 as libc::c_int as krb5_msgtype;
    ref_10.kdc_options &= !(0x8 as libc::c_int);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6A 82 01 E4 30 82 01 E0 A1 03 02 01 05 A2 03 02 01 0A A3 26 30 24 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 A4 82 01 AA 30 82 01 A6 A0 07 03 05 00 FE DC BA 90 A1 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A6 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 03 02 01 2A A8 08 30 06 02 01 00 02 01 01 A9 20 30 1E 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 AA 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 AB 81 BF 30 81 BC 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"as_req\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_as_req(&mut code, &mut var_9);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"as_req\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_as_req(&mut ref_10, var_9) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"as_req\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_req(test_context, var_9);
    ktest_destroy_pa_data_array(&mut ref_10.padata);
    ktest_destroy_principal(&mut ref_10.client);
    ktest_destroy_principal(&mut ref_10.server);
    ref_10.kdc_options |= 0x8 as libc::c_int;
    ref_10.from = 0 as libc::c_int;
    ref_10.rtime = 0 as libc::c_int;
    ktest_destroy_addresses(&mut ref_10.addresses);
    ktest_destroy_enc_data(&mut ref_10.authorization_data);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6A 82 01 14 30 82 01 10 A1 03 02 01 05 A2 03 02 01 0A A4 82 01 02 30 81 FF A0 07 03 05 00 FE DC BA 98 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 03 02 01 2A A8 08 30 06 02 01 00 02 01 01 AB 81 BF 30 81 BC 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"as_req\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_as_req(&mut code, &mut var_9);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"as_req\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_as_req(&mut ref_10, var_9) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"as_req\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL except second_ticket)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_req(test_context, var_9);
    ktest_destroy_sequence_of_ticket(&mut ref_10.second_ticket);
    ktest_make_sample_principal(&mut ref_10.server);
    ref_10.kdc_options &= !(0x8 as libc::c_int);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6A 69 30 67 A1 03 02 01 05 A2 03 02 01 0A A4 5B 30 59 A0 07 03 05 00 FE DC BA 90 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 03 02 01 2A A8 08 30 06 02 01 00 02 01 01\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"as_req\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_as_req(&mut code, &mut var_9);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"as_req\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_as_req(&mut ref_10, var_9) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"as_req\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL except server)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_req(test_context, var_9);
    ktest_empty_kdc_req(&mut ref_10);
    /* ***************************************************************/
    /* decode_krb5_tgs_req */
    let mut ref_11: krb5_kdc_req =
        krb5_kdc_req{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     kdc_options: 0,
                     client: 0 as *mut krb5_principal_data,
                     server: 0 as *mut krb5_principal_data,
                     from: 0,
                     till: 0,
                     rtime: 0,
                     nonce: 0,
                     nktypes: 0,
                     ktype: 0 as *mut krb5_enctype,
                     addresses: 0 as *mut *mut krb5_address,
                     authorization_data:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     unenc_authdata: 0 as *mut *mut krb5_authdata,
                     second_ticket: 0 as *mut *mut krb5_ticket,};
    let mut var_10: *mut krb5_kdc_req = 0 as *mut krb5_kdc_req;
    ktest_make_sample_kdc_req(&mut ref_11);
    ref_11.msg_type = 12 as libc::c_int as krb5_msgtype;
    ref_11.kdc_options &= !(0x8 as libc::c_int);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6C 82 01 E4 30 82 01 E0 A1 03 02 01 05 A2 03 02 01 0C A3 26 30 24 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 A4 82 01 AA 30 82 01 A6 A0 07 03 05 00 FE DC BA 90 A1 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A6 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 03 02 01 2A A8 08 30 06 02 01 00 02 01 01 A9 20 30 1E 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 AA 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 AB 81 BF 30 81 BC 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"tgs_req\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_tgs_req(&mut code, &mut var_10);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"tgs_req\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_tgs_req(&mut ref_11, var_10) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"tgs_req\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_req(test_context, var_10);
    ktest_destroy_pa_data_array(&mut ref_11.padata);
    ktest_destroy_principal(&mut ref_11.client);
    ktest_destroy_principal(&mut ref_11.server);
    ref_11.kdc_options |= 0x8 as libc::c_int;
    ref_11.from = 0 as libc::c_int;
    ref_11.rtime = 0 as libc::c_int;
    ktest_destroy_addresses(&mut ref_11.addresses);
    ktest_destroy_enc_data(&mut ref_11.authorization_data);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6C 82 01 14 30 82 01 10 A1 03 02 01 05 A2 03 02 01 0C A4 82 01 02 30 81 FF A0 07 03 05 00 FE DC BA 98 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 03 02 01 2A A8 08 30 06 02 01 00 02 01 01 AB 81 BF 30 81 BC 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"tgs_req\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_tgs_req(&mut code, &mut var_10);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"tgs_req\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_tgs_req(&mut ref_11, var_10) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"tgs_req\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL except second_ticket)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_req(test_context, var_10);
    ktest_destroy_sequence_of_ticket(&mut ref_11.second_ticket);
    ktest_make_sample_principal(&mut ref_11.server);
    ref_11.kdc_options &= !(0x8 as libc::c_int);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"6C 69 30 67 A1 03 02 01 05 A2 03 02 01 0C A4 5B 30 59 A0 07 03 05 00 FE DC BA 90 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 03 02 01 2A A8 08 30 06 02 01 00 02 01 01\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"tgs_req\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_tgs_req(&mut code, &mut var_10);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"tgs_req\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_tgs_req(&mut ref_11, var_10) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"tgs_req\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL except server)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_req(test_context, var_10);
    ktest_empty_kdc_req(&mut ref_11);
    /* ***************************************************************/
    /* decode_krb5_kdc_req_body */
    let mut ref_12: krb5_kdc_req =
        krb5_kdc_req{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     kdc_options: 0,
                     client: 0 as *mut krb5_principal_data,
                     server: 0 as *mut krb5_principal_data,
                     from: 0,
                     till: 0,
                     rtime: 0,
                     nonce: 0,
                     nktypes: 0,
                     ktype: 0 as *mut krb5_enctype,
                     addresses: 0 as *mut *mut krb5_address,
                     authorization_data:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     unenc_authdata: 0 as *mut *mut krb5_authdata,
                     second_ticket: 0 as *mut *mut krb5_ticket,};
    let mut var_11: *mut krb5_kdc_req = 0 as *mut krb5_kdc_req;
    memset(&mut ref_12 as *mut krb5_kdc_req as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_kdc_req>() as libc::c_ulong);
    ktest_make_sample_kdc_req_body(&mut ref_12);
    ref_12.kdc_options &= !(0x8 as libc::c_int);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 82 01 A6 A0 07 03 05 00 FE DC BA 90 A1 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A6 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 03 02 01 2A A8 08 30 06 02 01 00 02 01 01 A9 20 30 1E 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 AA 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 AB 81 BF 30 81 BC 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_kdc_req_body(&mut code, &mut var_11);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_kdc_req_body(&mut ref_12, var_11) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_req(test_context, var_11);
    ktest_destroy_principal(&mut ref_12.client);
    ktest_destroy_principal(&mut ref_12.server);
    ref_12.kdc_options |= 0x8 as libc::c_int;
    ref_12.from = 0 as libc::c_int;
    ref_12.rtime = 0 as libc::c_int;
    ktest_destroy_addresses(&mut ref_12.addresses);
    ktest_destroy_enc_data(&mut ref_12.authorization_data);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 81 FF A0 07 03 05 00 FE DC BA 98 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 03 02 01 2A A8 08 30 06 02 01 00 02 01 01 AB 81 BF 30 81 BC 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_kdc_req_body(&mut code, &mut var_11);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_kdc_req_body(&mut ref_12, var_11) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL except second_ticket)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_req(test_context, var_11);
    ktest_destroy_sequence_of_ticket(&mut ref_12.second_ticket);
    ktest_make_sample_principal(&mut ref_12.server);
    ref_12.kdc_options &= !(0x8 as libc::c_int);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 59 A0 07 03 05 00 FE DC BA 90 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 03 02 01 2A A8 08 30 06 02 01 00 02 01 01\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_kdc_req_body(&mut code, &mut var_11);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_kdc_req_body(&mut ref_12, var_11) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL except server)\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_req(test_context, var_11);
    ref_12.nktypes = 0 as libc::c_int;
    free(ref_12.ktype as *mut libc::c_void);
    ref_12.ktype = 0 as *mut krb5_enctype;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 53 A0 07 03 05 00 FE DC BA 90 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 03 02 01 2A A8 02 30 00\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_kdc_req_body(&mut code, &mut var_11);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_kdc_req_body(&mut ref_12, var_11) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"kdc_req_body\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL except server; zero-length etypes)\x00" as
               *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_kdc_req(test_context, var_11);
    ktest_empty_kdc_req(&mut ref_12);
    /* ***************************************************************/
    /* decode_krb5_safe */
    let mut ref_13: krb5_safe =
        krb5_safe{magic: 0,
                  user_data:
                      krb5_data{magic: 0,
                                length: 0,
                                data: 0 as *mut libc::c_char,},
                  timestamp: 0,
                  usec: 0,
                  seq_number: 0,
                  s_address: 0 as *mut krb5_address,
                  r_address: 0 as *mut krb5_address,
                  checksum: 0 as *mut krb5_checksum,};
    let mut var_12: *mut krb5_safe = 0 as *mut krb5_safe;
    ktest_make_sample_safe(&mut ref_13);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"74 6E 30 6C A0 03 02 01 05 A1 03 02 01 14 A2 4F 30 4D A0 0A 04 08 6B 72 62 35 64 61 74 61 A1 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A2 05 02 03 01 E2 40 A3 03 02 01 11 A4 0F 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 A5 0F 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 A3 0F 30 0D A0 03 02 01 01 A1 06 04 04 31 32 33 34\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"safe\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_safe(&mut code, &mut var_12);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"safe\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_safe(&mut ref_13, var_12) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"safe\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_safe(test_context, var_12);
    ref_13.timestamp = 0 as libc::c_int;
    ref_13.usec = 0 as libc::c_int;
    ref_13.seq_number = 0 as libc::c_int as krb5_ui_4;
    ktest_destroy_address(&mut ref_13.r_address);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"74 3E 30 3C A0 03 02 01 05 A1 03 02 01 14 A2 1F 30 1D A0 0A 04 08 6B 72 62 35 64 61 74 61 A4 0F 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 A3 0F 30 0D A0 03 02 01 01 A1 06 04 04 31 32 33 34\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"safe\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_safe(&mut code, &mut var_12);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"safe\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_safe(&mut ref_13, var_12) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"safe\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_safe(test_context, var_12);
    ktest_empty_safe(&mut ref_13);
    /* ***************************************************************/
    /* decode_krb5_priv */
    let mut ref_14: krb5_priv =
        krb5_priv{magic: 0,
                  enc_part:
                      krb5_enc_data{magic: 0,
                                    enctype: 0,
                                    kvno: 0,
                                    ciphertext:
                                        krb5_data{magic: 0,
                                                  length: 0,
                                                  data:
                                                      0 as
                                                          *mut libc::c_char,},},};
    let mut var_13: *mut krb5_priv = 0 as *mut krb5_priv;
    ktest_make_sample_priv(&mut ref_14);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"75 33 30 31 A0 03 02 01 05 A1 03 02 01 15 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"priv\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_priv(&mut code, &mut var_13);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"priv\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_priv(&mut ref_14, var_13) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"priv\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_priv(test_context, var_13);
    ktest_empty_priv(&mut ref_14);
    /* ***************************************************************/
    /* decode_krb5_enc_priv_part */
    let mut ref_15: krb5_priv_enc_part =
        krb5_priv_enc_part{magic: 0,
                           user_data:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           timestamp: 0,
                           usec: 0,
                           seq_number: 0,
                           s_address: 0 as *mut krb5_address,
                           r_address: 0 as *mut krb5_address,};
    let mut var_14: *mut krb5_priv_enc_part = 0 as *mut krb5_priv_enc_part;
    ktest_make_sample_priv_enc_part(&mut ref_15);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"7C 4F 30 4D A0 0A 04 08 6B 72 62 35 64 61 74 61 A1 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A2 05 02 03 01 E2 40 A3 03 02 01 11 A4 0F 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 A5 0F 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_priv_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_priv_part(&mut code, &mut var_14);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_priv_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_priv_part(&mut ref_15, var_14) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_priv_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_priv_enc_part(test_context, var_14);
    ref_15.timestamp = 0 as libc::c_int;
    ref_15.usec = 0 as libc::c_int;
    ref_15.seq_number = 0 as libc::c_int as krb5_ui_4;
    ktest_destroy_address(&mut ref_15.r_address);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"7C 1F 30 1D A0 0A 04 08 6B 72 62 35 64 61 74 61 A4 0F 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_priv_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_priv_part(&mut code, &mut var_14);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_priv_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_priv_part(&mut ref_15, var_14) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_priv_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_priv_enc_part(test_context, var_14);
    ktest_empty_priv_enc_part(&mut ref_15);
    /* ***************************************************************/
    /* decode_krb5_cred */
    let mut ref_16: krb5_cred =
        krb5_cred{magic: 0,
                  tickets: 0 as *mut *mut krb5_ticket,
                  enc_part:
                      krb5_enc_data{magic: 0,
                                    enctype: 0,
                                    kvno: 0,
                                    ciphertext:
                                        krb5_data{magic: 0,
                                                  length: 0,
                                                  data:
                                                      0 as
                                                          *mut libc::c_char,},},
                  enc_part2: 0 as *mut krb5_cred_enc_part,};
    let mut var_15: *mut krb5_cred = 0 as *mut krb5_cred;
    ktest_make_sample_cred(&mut ref_16);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"76 81 F6 30 81 F3 A0 03 02 01 05 A1 03 02 01 16 A2 81 BF 30 81 BC 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"cred\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_cred(&mut code, &mut var_15);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"cred\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_cred(&mut ref_16, var_15) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"cred\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_cred(test_context, var_15);
    ktest_empty_cred(&mut ref_16);
    /* ***************************************************************/
    /* decode_krb5_enc_cred_part */
    let mut ref_17: krb5_cred_enc_part =
        krb5_cred_enc_part{magic: 0,
                           nonce: 0,
                           timestamp: 0,
                           usec: 0,
                           s_address: 0 as *mut krb5_address,
                           r_address: 0 as *mut krb5_address,
                           ticket_info: 0 as *mut *mut krb5_cred_info,};
    let mut var_16: *mut krb5_cred_enc_part = 0 as *mut krb5_cred_enc_part;
    ktest_make_sample_cred_enc_part(&mut ref_17);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"7D 82 02 23 30 82 02 1F A0 82 01 DA 30 82 01 D6 30 81 E8 A0 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 07 03 05 00 FE DC BA 98 A4 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A6 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A8 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A9 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 AA 20 30 1E 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 30 81 E8 A0 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 07 03 05 00 FE DC BA 98 A4 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A6 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A8 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A9 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 AA 20 30 1E 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 A1 03 02 01 2A A2 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A3 05 02 03 01 E2 40 A4 0F 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 A5 0F 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_cred_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_cred_part(&mut code, &mut var_16);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_cred_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_cred_part(&mut ref_17, var_16) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_cred_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_cred_enc_part(test_context, var_16);
    /* free_cred_enc_part does not free the pointer */
    free(var_16 as *mut libc::c_void);
    ktest_destroy_principal(&mut (**ref_17.ticket_info.offset(0 as libc::c_int
                                                                  as
                                                                  isize)).client);
    ktest_destroy_principal(&mut (**ref_17.ticket_info.offset(0 as libc::c_int
                                                                  as
                                                                  isize)).server);
    (**ref_17.ticket_info.offset(0 as libc::c_int as isize)).flags =
        0 as libc::c_int;
    (**ref_17.ticket_info.offset(0 as libc::c_int as isize)).times.authtime =
        0 as libc::c_int;
    (**ref_17.ticket_info.offset(0 as libc::c_int as isize)).times.starttime =
        0 as libc::c_int;
    (**ref_17.ticket_info.offset(0 as libc::c_int as isize)).times.endtime =
        0 as libc::c_int;
    (**ref_17.ticket_info.offset(0 as libc::c_int as isize)).times.renew_till
        = 0 as libc::c_int;
    ktest_destroy_addresses(&mut (**ref_17.ticket_info.offset(0 as libc::c_int
                                                                  as
                                                                  isize)).caddrs);
    ref_17.nonce = 0 as libc::c_int;
    ref_17.timestamp = 0 as libc::c_int;
    ref_17.usec = 0 as libc::c_int;
    ktest_destroy_address(&mut ref_17.s_address);
    ktest_destroy_address(&mut ref_17.r_address);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"7D 82 01 0E 30 82 01 0A A0 82 01 06 30 82 01 02 30 15 A0 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 30 81 E8 A0 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 07 03 05 00 FE DC BA 98 A4 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A6 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A8 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A9 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 AA 20 30 1E 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_cred_part\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_cred_part(&mut code, &mut var_16);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_cred_part\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_cred_part(&mut ref_17, var_16) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_cred_part\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_cred_enc_part(test_context, var_16);
    /* free_cred_enc_part does not free the pointer */
    free(var_16 as *mut libc::c_void);
    ktest_empty_cred_enc_part(&mut ref_17);
    /* ***************************************************************/
    /* decode_krb5_error */
    let mut ref_18: krb5_error =
        krb5_error{magic: 0,
                   ctime: 0,
                   cusec: 0,
                   susec: 0,
                   stime: 0,
                   error: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   text:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   e_data:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},};
    let mut var_17: *mut krb5_error = 0 as *mut krb5_error;
    ktest_make_sample_error(&mut ref_18);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"7E 81 BA 30 81 B7 A0 03 02 01 05 A1 03 02 01 1E A2 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A3 05 02 03 01 E2 40 A4 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A5 05 02 03 01 E2 40 A6 03 02 01 3C A7 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A8 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A9 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 AA 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 AB 0A 1B 08 6B 72 62 35 64 61 74 61 AC 0A 04 08 6B 72 62 35 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"error\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_error(&mut code, &mut var_17);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"error\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_error(&mut ref_18, var_17) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"error\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_error(test_context, var_17);
    ref_18.ctime = 0 as libc::c_int;
    ktest_destroy_principal(&mut ref_18.client);
    ktest_empty_data(&mut ref_18.text);
    ktest_empty_data(&mut ref_18.e_data);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"7E 60 30 5E A0 03 02 01 05 A1 03 02 01 1E A3 05 02 03 01 E2 40 A4 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A5 05 02 03 01 E2 40 A6 03 02 01 3C A9 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 AA 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"error\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_error(&mut code, &mut var_17);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"error\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_error(&mut ref_18, var_17) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"error\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_error(test_context, var_17);
    ktest_empty_error(&mut ref_18);
    /* ***************************************************************/
    /* decode_krb5_authdata and krb5int_get_authdata_containee_types */
    let mut ref_19: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut var_18: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut tmp: krb5_authdata =
        krb5_authdata{magic: 0,
                      ad_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut count: libc::c_uint = 0;
    let mut types: *mut krb5_authdatatype = 0 as *mut krb5_authdatatype;
    ktest_make_sample_authorization_data(&mut ref_19);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 22 30 0F A0 03 02 01 01 A1 08 04 06 66 6F 6F 62 61 72 30 0F A0 03 02 01 01 A1 08 04 06 66 6F 6F 62 61 72\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"parsing authorization_data\x00" as *const u8 as
                    *const libc::c_char, retval as errcode_t,
                b"\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_authdata(&mut code, &mut var_18);
    if retval != 0 {
        com_err(b"decoding authorization_data\x00" as *const u8 as
                    *const libc::c_char, retval as errcode_t,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    if ktest_equal_authorization_data(ref_19, var_18) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"authorization_data\n\x00" as *const u8 as *const libc::c_char);
    tmp.length = code.length;
    tmp.contents = code.data as *mut krb5_octet;
    retval =
        krb5int_get_authdata_containee_types(test_context, &mut tmp,
                                             &mut count, &mut types);
    if retval != 0 {
        com_err(b"reading authdata types\x00" as *const u8 as
                    *const libc::c_char, retval as errcode_t,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    if count == 2 as libc::c_int as libc::c_uint &&
           *types.offset(0 as libc::c_int as isize) == 1 as libc::c_int &&
           *types.offset(1 as libc::c_int as isize) == 1 as libc::c_int {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"authorization_data(types only)\n\x00" as *const u8 as
               *const libc::c_char);
    free(types as *mut libc::c_void);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_authdata(test_context, var_18);
    ktest_destroy_authorization_data(&mut ref_19);
    /* ***************************************************************/
    /* decode_krb5_padata_sequence and decode_krb5_typed_data */
    let mut ref_20: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut var_19: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    ktest_make_sample_pa_data_array(&mut ref_20);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 24 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"parsing padata_sequence\x00" as *const u8 as
                    *const libc::c_char, retval as errcode_t,
                b"\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_padata_sequence(&mut code, &mut var_19);
    if retval != 0 {
        com_err(b"decoding padata_sequence\x00" as *const u8 as
                    *const libc::c_char, retval as errcode_t,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    if ktest_equal_sequence_of_pa_data(ref_20, var_19) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_data\n\x00" as *const u8 as *const libc::c_char);
    krb5_free_pa_data(test_context, var_19);
    krb5_free_data_contents(test_context, &mut code);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 24 30 10 A0 03 02 01 0D A1 09 04 07 70 61 2D 64 61 74 61 30 10 A0 03 02 01 0D A1 09 04 07 70 61 2D 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"parsing padata_sequence\x00" as *const u8 as
                    *const libc::c_char, retval as errcode_t,
                b"\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_typed_data(&mut code, &mut var_19);
    if retval != 0 {
        com_err(b"decoding typed_data\x00" as *const u8 as
                    *const libc::c_char, retval as errcode_t,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    if ktest_equal_sequence_of_pa_data(ref_20, var_19) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"typed_data\n\x00" as *const u8 as *const libc::c_char);
    krb5_free_pa_data(test_context, var_19);
    krb5_free_data_contents(test_context, &mut code);
    ktest_destroy_pa_data_array(&mut ref_20);
    /* ***************************************************************/
    /* decode_krb5_padata_sequence (empty) */
    let mut ref_21: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut var_20: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    ktest_make_sample_empty_pa_data_array(&mut ref_21);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 00\x00" as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"parsing padata_sequence (empty)\x00" as *const u8 as
                    *const libc::c_char, retval as errcode_t,
                b"\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_padata_sequence(&mut code, &mut var_20);
    if retval != 0 {
        com_err(b"decoding padata_sequence (empty)\x00" as *const u8 as
                    *const libc::c_char, retval as errcode_t,
                b"\x00" as *const u8 as *const libc::c_char);
    }
    if ktest_equal_sequence_of_pa_data(ref_21, var_20) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_data (empty)\n\x00" as *const u8 as *const libc::c_char);
    krb5_free_pa_data(test_context, var_20);
    krb5_free_data_contents(test_context, &mut code);
    ktest_destroy_pa_data_array(&mut ref_21);
    /* ***************************************************************/
    /* decode_etype_info */
    let mut ref_22: krb5_etype_info = 0 as *mut *mut krb5_etype_info_entry;
    let mut var_21: krb5_etype_info = 0 as *mut *mut krb5_etype_info_entry;
    ktest_make_sample_etype_info(&mut ref_22);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 33 30 14 A0 03 02 01 00 A1 0D 04 0B 4D 6F 72 74 6F 6E 27 73 20 23 30 30 05 A0 03 02 01 01 30 14 A0 03 02 01 02 A1 0D 04 0B 4D 6F 72 74 6F 6E 27 73 20 23 32\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing etype_info\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_etype_info(&mut code, &mut var_21);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding etype_info\x00" as *const u8 as
                    *const libc::c_char);
    }
    if ktest_equal_etype_info(ref_22, var_21) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"etype_info\n\x00" as *const u8 as *const libc::c_char);
    ktest_destroy_etype_info(var_21);
    ktest_destroy_etype_info_entry(*ref_22.offset(2 as libc::c_int as isize));
    let ref mut fresh0 = *ref_22.offset(2 as libc::c_int as isize);
    *fresh0 = 0 as *mut krb5_etype_info_entry;
    ktest_destroy_etype_info_entry(*ref_22.offset(1 as libc::c_int as isize));
    let ref mut fresh1 = *ref_22.offset(1 as libc::c_int as isize);
    *fresh1 = 0 as *mut krb5_etype_info_entry;
    krb5_free_data_contents(test_context, &mut code);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 16 30 14 A0 03 02 01 00 A1 0D 04 0B 4D 6F 72 74 6F 6E 27 73 20 23 30\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing etype_info (only one)\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_etype_info(&mut code, &mut var_21);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding etype_info (only one)\x00" as *const u8 as
                    *const libc::c_char);
    }
    if ktest_equal_etype_info(ref_22, var_21) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"etype_info (only one)\n\x00" as *const u8 as
               *const libc::c_char);
    ktest_destroy_etype_info(var_21);
    ktest_destroy_etype_info_entry(*ref_22.offset(0 as libc::c_int as isize));
    let ref mut fresh2 = *ref_22.offset(0 as libc::c_int as isize);
    *fresh2 = 0 as *mut krb5_etype_info_entry;
    krb5_free_data_contents(test_context, &mut code);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 00\x00" as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing etype_info (no info)\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_etype_info(&mut code, &mut var_21);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding etype_info (no info)\x00" as *const u8 as
                    *const libc::c_char);
    }
    if ktest_equal_etype_info(ref_22, var_21) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"etype_info (no info)\n\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    ktest_destroy_etype_info(var_21);
    ktest_destroy_etype_info(ref_22);
    /* ***************************************************************/
    /* decode_etype_info2 */
    let mut ref_23: krb5_etype_info = 0 as *mut *mut krb5_etype_info_entry;
    let mut var_22: krb5_etype_info = 0 as *mut *mut krb5_etype_info_entry;
    ktest_make_sample_etype_info2(&mut ref_23);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 51 30 1E A0 03 02 01 00 A1 0D 1B 0B 4D 6F 72 74 6F 6E 27 73 20 23 30 A2 08 04 06 73 32 6B 3A 20 30 30 0F A0 03 02 01 01 A2 08 04 06 73 32 6B 3A 20 31 30 1E A0 03 02 01 02 A1 0D 1B 0B 4D 6F 72 74 6F 6E 27 73 20 23 32 A2 08 04 06 73 32 6B 3A 20 32\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing etype_info2\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_etype_info2(&mut code, &mut var_22);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding etype_info2\x00" as *const u8 as
                    *const libc::c_char);
    }
    if ktest_equal_etype_info(ref_23, var_22) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"etype_info2\n\x00" as *const u8 as *const libc::c_char);
    ktest_destroy_etype_info(var_22);
    ktest_destroy_etype_info_entry(*ref_23.offset(2 as libc::c_int as isize));
    let ref mut fresh3 = *ref_23.offset(2 as libc::c_int as isize);
    *fresh3 = 0 as *mut krb5_etype_info_entry;
    ktest_destroy_etype_info_entry(*ref_23.offset(1 as libc::c_int as isize));
    let ref mut fresh4 = *ref_23.offset(1 as libc::c_int as isize);
    *fresh4 = 0 as *mut krb5_etype_info_entry;
    krb5_free_data_contents(test_context, &mut code);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 20 30 1E A0 03 02 01 00 A1 0D 1B 0B 4D 6F 72 74 6F 6E 27 73 20 23 30 A2 08 04 06 73 32 6B 3A 20 30\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing etype_info2 (only one)\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_etype_info2(&mut code, &mut var_22);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding etype_info2 (only one)\x00" as *const u8 as
                    *const libc::c_char);
    }
    if ktest_equal_etype_info(ref_23, var_22) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"etype_info2 (only one)\n\x00" as *const u8 as
               *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    ktest_destroy_etype_info(var_22);
    ktest_destroy_etype_info(ref_23);
    /* ***************************************************************/
    /* decode_pa_enc_ts */
    let mut ref_24: krb5_pa_enc_ts =
        krb5_pa_enc_ts{patimestamp: 0, pausec: 0,};
    let mut var_23: *mut krb5_pa_enc_ts = 0 as *mut krb5_pa_enc_ts;
    ktest_make_sample_pa_enc_ts(&mut ref_24);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 1A A0 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A1 05 02 03 01 E2 40\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_enc_ts\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_enc_ts(&mut code, &mut var_23);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_enc_ts\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_krb5_pa_enc_ts(&mut ref_24, var_23) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_enc_ts\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_pa_enc_ts(test_context, var_23);
    ref_24.pausec = 0 as libc::c_int;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 13 A0 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_enc_ts (no usec)\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_enc_ts(&mut code, &mut var_23);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_enc_ts (no usec)\x00" as *const u8 as
                    *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_krb5_pa_enc_ts(&mut ref_24, var_23) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_enc_ts (no usec)\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_pa_enc_ts(test_context, var_23);
    /* ***************************************************************/
    /* decode_enc_data */
    let mut ref_25: krb5_enc_data =
        krb5_enc_data{magic: 0,
                      enctype: 0,
                      kvno: 0,
                      ciphertext:
                          krb5_data{magic: 0,
                                    length: 0,
                                    data: 0 as *mut libc::c_char,},};
    let mut var_24: *mut krb5_enc_data = 0 as *mut krb5_enc_data;
    ktest_make_sample_enc_data(&mut ref_25);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_data\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_data(&mut code, &mut var_24);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_data\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_data(&mut ref_25, var_24) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_data\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_ktest_free_enc_data(test_context, var_24);
    ref_25.kvno = 0xff000000 as libc::c_uint;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 26 A0 03 02 01 00 A1 06 02 04 FF 00 00 00 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_data\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_data(&mut code, &mut var_24);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_data\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_data(&mut ref_25, var_24) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_data\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(MSB-set kvno)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_ktest_free_enc_data(test_context, var_24);
    ref_25.kvno = 0xffffffff as libc::c_uint;
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 23 A0 03 02 01 00 A1 03 02 01 FF A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"enc_data\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_enc_data(&mut code, &mut var_24);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"enc_data\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_data(&mut ref_25, var_24) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"enc_data\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(kvno=-1)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_ktest_free_enc_data(test_context, var_24);
    ktest_destroy_enc_data(&mut ref_25);
    /* ***************************************************************/
    /* decode_sam_challenge_2 */
    let mut ref_26: krb5_sam_challenge_2 =
        krb5_sam_challenge_2{sam_challenge_2_body:
                                 krb5_data{magic: 0,
                                           length: 0,
                                           data: 0 as *mut libc::c_char,},
                             sam_cksum: 0 as *mut *mut krb5_checksum,};
    let mut var_25: *mut krb5_sam_challenge_2 =
        0 as *mut krb5_sam_challenge_2;
    ktest_make_sample_sam_challenge_2(&mut ref_26);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 22 A0 0D 30 0B 04 09 63 68 61 6C 6C 65 6E 67 65 A1 11 30 0F 30 0D A0 03 02 01 01 A1 06 04 04 31 32 33 34\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"sam_challenge_2\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_sam_challenge_2(&mut code, &mut var_25);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"sam_challenge_2\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_sam_challenge_2(&mut ref_26, var_25) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"sam_challenge_2\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_sam_challenge_2(test_context, var_25);
    ktest_empty_sam_challenge_2(&mut ref_26);
    /* ***************************************************************/
    /* decode_sam_challenge_2_body */
    let mut ref_27: krb5_sam_challenge_2_body =
        krb5_sam_challenge_2_body{magic: 0,
                                  sam_type: 0,
                                  sam_flags: 0,
                                  sam_type_name:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_track_id:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_challenge_label:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_challenge:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_response_prompt:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_pk_for_sad:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_nonce: 0,
                                  sam_etype: 0,};
    let mut var_26: *mut krb5_sam_challenge_2_body =
        0 as *mut krb5_sam_challenge_2_body;
    ktest_make_sample_sam_challenge_2_body(&mut ref_27);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 64 A0 03 02 01 2A A1 07 03 05 00 80 00 00 00 A2 0B 04 09 74 79 70 65 20 6E 61 6D 65 A4 11 04 0F 63 68 61 6C 6C 65 6E 67 65 20 6C 61 62 65 6C A5 10 04 0E 63 68 61 6C 6C 65 6E 67 65 20 69 70 73 65 A6 16 04 14 72 65 73 70 6F 6E 73 65 5F 70 72 6F 6D 70 74 20 69 70 73 65 A8 05 02 03 54 32 10 A9 03 02 01 14\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"sam_challenge_2_body\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_sam_challenge_2_body(&mut code, &mut var_26);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"sam_challenge_2_body\x00" as *const u8 as
                    *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_sam_challenge_2_body(&mut ref_27, var_26) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"sam_challenge_2_body\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_sam_challenge_2_body(test_context, var_26);
    ktest_empty_sam_challenge_2_body(&mut ref_27);
    /* ***************************************************************/
    /* decode_pa_for_user */
    let mut ref_28: krb5_pa_for_user =
        krb5_pa_for_user{user: 0 as *mut krb5_principal_data,
                         cksum:
                             krb5_checksum{magic: 0,
                                           checksum_type: 0,
                                           length: 0,
                                           contents: 0 as *mut krb5_octet,},
                         auth_package:
                             krb5_data{magic: 0,
                                       length: 0,
                                       data: 0 as *mut libc::c_char,},};
    let mut var_27: *mut krb5_pa_for_user = 0 as *mut krb5_pa_for_user;
    ktest_make_sample_pa_for_user(&mut ref_28);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 4B A0 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 0F 30 0D A0 03 02 01 01 A1 06 04 04 31 32 33 34 A3 0A 1B 08 6B 72 62 35 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_for_user\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_for_user(&mut code, &mut var_27);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_for_user\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_for_user(&mut ref_28, var_27) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_for_user\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_pa_for_user(test_context, var_27);
    ktest_empty_pa_for_user(&mut ref_28);
    /* ***************************************************************/
    /* decode_pa_s4u_x509_user */
    let mut ref_29: krb5_pa_s4u_x509_user =
        krb5_pa_s4u_x509_user{user_id:
                                  krb5_s4u_userid{nonce: 0,
                                                  user:
                                                      0 as
                                                          *mut krb5_principal_data,
                                                  subject_cert:
                                                      krb5_data{magic: 0,
                                                                length: 0,
                                                                data:
                                                                    0 as
                                                                        *mut libc::c_char,},
                                                  options: 0,},
                              cksum:
                                  krb5_checksum{magic: 0,
                                                checksum_type: 0,
                                                length: 0,
                                                contents:
                                                    0 as *mut krb5_octet,},};
    let mut var_28: *mut krb5_pa_s4u_x509_user =
        0 as *mut krb5_pa_s4u_x509_user;
    ktest_make_sample_pa_s4u_x509_user(&mut ref_29);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 68 A0 55 30 53 A0 06 02 04 00 CA 14 9A A1 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 12 04 10 70 61 5F 73 34 75 5F 78 35 30 39 5F 75 73 65 72 A4 07 03 05 00 80 00 00 00 A1 0F 30 0D A0 03 02 01 01 A1 06 04 04 31 32 33 34\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_s4u_x509_user\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_s4u_x509_user(&mut code, &mut var_28);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_s4u_x509_user\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_s4u_x509_user(&mut ref_29, var_28) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_s4u_x509_user\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_pa_s4u_x509_user(test_context, var_28);
    ktest_empty_pa_s4u_x509_user(&mut ref_29);
    /* ***************************************************************/
    /* decode_pa_pac_req */
    /* This type has no encoder and is very simple.  Test two
         * hand-generated encodings. */
    let mut req1: *mut krb5_pa_pac_req = 0 as *mut krb5_pa_pac_req;
    let mut req2: *mut krb5_pa_pac_req = 0 as *mut krb5_pa_pac_req;
    code =
        make_data(b"0\x05\xa0\x03\x01\x01\x00\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_void,
                  7 as libc::c_int as libc::c_uint);
    retval = decode_krb5_pa_pac_req(&mut code, &mut req1);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while decoding PA-PAC-REQ\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    code =
        make_data(b"0\x05\xa0\x03\x01\x01\xff\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_void,
                  7 as libc::c_int as libc::c_uint);
    retval = decode_krb5_pa_pac_req(&mut code, &mut req2);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while decoding PA-PAC-REQ\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if (*req1).include_pac != 0 as libc::c_int as libc::c_uint ||
           (*req2).include_pac != 1 as libc::c_int as libc::c_uint {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    } else { printf(b"OK: \x00" as *const u8 as *const libc::c_char); }
    printf(b"pa_pac_rec\n\x00" as *const u8 as *const libc::c_char);
    free(req1 as *mut libc::c_void);
    free(req2 as *mut libc::c_void);
    /* ***************************************************************/
    /* decode_ad_kdcissued */
    let mut ref_30: krb5_ad_kdcissued =
        krb5_ad_kdcissued{ad_checksum:
                              krb5_checksum{magic: 0,
                                            checksum_type: 0,
                                            length: 0,
                                            contents: 0 as *mut krb5_octet,},
                          i_principal: 0 as *mut krb5_principal_data,
                          elements: 0 as *mut *mut krb5_authdata,};
    let mut var_29: *mut krb5_ad_kdcissued = 0 as *mut krb5_ad_kdcissued;
    ktest_make_sample_ad_kdcissued(&mut ref_30);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 65 A0 0F 30 0D A0 03 02 01 01 A1 06 04 04 31 32 33 34 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 24 30 22 30 0F A0 03 02 01 01 A1 08 04 06 66 6F 6F 62 61 72 30 0F A0 03 02 01 01 A1 08 04 06 66 6F 6F 62 61 72\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"ad_kdcissued\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_ad_kdcissued(&mut code, &mut var_29);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"ad_kdcissued\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_ad_kdcissued(&mut ref_30, var_29) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"ad_kdcissued\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_ad_kdcissued(test_context, var_29);
    ktest_empty_ad_kdcissued(&mut ref_30);
    /* ***************************************************************/
    /* decode_ad_signedpath */
    let mut ref_31: krb5_ad_signedpath =
        krb5_ad_signedpath{enctype: 0,
                           checksum:
                               krb5_checksum{magic: 0,
                                             checksum_type: 0,
                                             length: 0,
                                             contents: 0 as *mut krb5_octet,},
                           delegated: 0 as *mut krb5_principal,
                           method_data: 0 as *mut *mut krb5_pa_data,};
    let mut var_30: *mut krb5_ad_signedpath = 0 as *mut krb5_ad_signedpath;
    ktest_make_sample_ad_signedpath(&mut ref_31);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 3E A0 03 02 01 01 A1 0F 30 0D A0 03 02 01 01 A1 06 04 04 31 32 33 34 A3 26 30 24 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"ad_signedpath\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_ad_signedpath(&mut code, &mut var_30);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"ad_signedpath\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_ad_signedpath(&mut ref_31, var_30) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"ad_signedpath\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_ad_signedpath(test_context, var_30);
    ktest_empty_ad_signedpath(&mut ref_31);
    /* ***************************************************************/
    /* decode_iakerb_header */
    let mut ref_32: krb5_iakerb_header =
        krb5_iakerb_header{target_realm:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           cookie: 0 as *mut krb5_data,};
    let mut var_31: *mut krb5_iakerb_header = 0 as *mut krb5_iakerb_header;
    ktest_make_sample_iakerb_header(&mut ref_32);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 18 A1 0A 04 08 6B 72 62 35 64 61 74 61 A2 0A 04 08 6B 72 62 35 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"iakerb_header\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_iakerb_header(&mut code, &mut var_31);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"iakerb_header\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_iakerb_header(&mut ref_32, var_31) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"iakerb_header\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_iakerb_header(test_context, var_31);
    ktest_empty_iakerb_header(&mut ref_32);
    /* ***************************************************************/
    /* decode_iakerb_finished */
    let mut ref_33: krb5_iakerb_finished =
        krb5_iakerb_finished{checksum:
                                 krb5_checksum{magic: 0,
                                               checksum_type: 0,
                                               length: 0,
                                               contents:
                                                   0 as *mut krb5_octet,},};
    let mut var_32: *mut krb5_iakerb_finished =
        0 as *mut krb5_iakerb_finished;
    ktest_make_sample_iakerb_finished(&mut ref_33);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 11 A1 0F 30 0D A0 03 02 01 01 A1 06 04 04 31 32 33 34\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"iakerb_finished\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_iakerb_finished(&mut code, &mut var_32);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"iakerb_finished\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_iakerb_finished(&mut ref_33, var_32) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"iakerb_finished\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_iakerb_finished(test_context, var_32);
    ktest_empty_iakerb_finished(&mut ref_33);
    /* ***************************************************************/
    /* decode_fast_response */
    let mut ref_34: krb5_fast_response =
        krb5_fast_response{magic: 0,
                           padata: 0 as *mut *mut krb5_pa_data,
                           strengthen_key: 0 as *mut krb5_keyblock,
                           finished: 0 as *mut krb5_fast_finished,
                           nonce: 0,};
    let mut var_33: *mut krb5_fast_response = 0 as *mut krb5_fast_response;
    ktest_make_sample_fast_response(&mut ref_34);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 81 9F A0 26 30 24 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 A1 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A2 5B 30 59 A0 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A1 05 02 03 01 E2 40 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 0F 30 0D A0 03 02 01 01 A1 06 04 04 31 32 33 34 A3 03 02 01 2A\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"fast_response\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_fast_response(&mut code, &mut var_33);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"fast_response\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_fast_response(&mut ref_34, var_33) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"fast_response\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_fast_response(test_context, var_33);
    ktest_empty_fast_response(&mut ref_34);
    /* ***************************************************************/
    /* decode_pa_fx_fast_reply */
    let mut ref_35: krb5_enc_data =
        krb5_enc_data{magic: 0,
                      enctype: 0,
                      kvno: 0,
                      ciphertext:
                          krb5_data{magic: 0,
                                    length: 0,
                                    data: 0 as *mut libc::c_char,},};
    let mut var_34: *mut krb5_enc_data = 0 as *mut krb5_enc_data;
    ktest_make_sample_enc_data(&mut ref_35);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"A0 29 30 27 A0 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_fx_fast_reply\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_fx_fast_reply(&mut code, &mut var_34);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_fx_fast_reply\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_enc_data(&mut ref_35, var_34) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_fx_fast_reply\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_enc_data(test_context, var_34);
    ktest_destroy_enc_data(&mut ref_35);
    /* ***************************************************************/
    /* decode_krb5_otp_tokeninfo */
    let mut ref_36: krb5_otp_tokeninfo =
        krb5_otp_tokeninfo{flags: 0,
                           vendor:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           challenge:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           length: 0,
                           format: 0,
                           token_id:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           alg_id:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           supported_hash_alg:
                               0 as *mut *mut krb5_algorithm_identifier,
                           iteration_count: 0,};
    let mut var_35: *mut krb5_otp_tokeninfo = 0 as *mut krb5_otp_tokeninfo;
    ktest_make_minimal_otp_tokeninfo(&mut ref_36);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 07 80 05 00 00 00 00 00\x00" as *const u8 as
                                *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"otp_tokeninfo\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_otp_tokeninfo(&mut code, &mut var_35);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"otp_tokeninfo\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_otp_tokeninfo(&mut ref_36, var_35) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"otp_tokeninfo\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_otp_tokeninfo(test_context, var_35);
    ktest_empty_otp_tokeninfo(&mut ref_36);
    let mut ref_37: krb5_otp_tokeninfo =
        krb5_otp_tokeninfo{flags: 0,
                           vendor:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           challenge:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           length: 0,
                           format: 0,
                           token_id:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           alg_id:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           supported_hash_alg:
                               0 as *mut *mut krb5_algorithm_identifier,
                           iteration_count: 0,};
    let mut var_36: *mut krb5_otp_tokeninfo = 0 as *mut krb5_otp_tokeninfo;
    ktest_make_maximal_otp_tokeninfo(&mut ref_37);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 72 80 05 00 77 00 00 00 81 0B 45 78 61 6D 70 6C 65 63 6F 72 70 82 05 68 61 72 6B 21 83 01 0A 84 01 02 85 09 79 6F 75 72 74 6F 6B 65 6E 86 28 75 72 6E 3A 69 65 74 66 3A 70 61 72 61 6D 73 3A 78 6D 6C 3A 6E 73 3A 6B 65 79 70 72 6F 76 3A 70 73 6B 63 3A 68 6F 74 70 A7 16 30 0B 06 09 60 86 48 01 65 03 04 02 01 30 07 06 05 2B 0E 03 02 1A 88 02 03 E8\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"otp_tokeninfo\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_otp_tokeninfo(&mut code, &mut var_36);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"otp_tokeninfo\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_otp_tokeninfo(&mut ref_37, var_36) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"otp_tokeninfo\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_otp_tokeninfo(test_context, var_36);
    ktest_empty_otp_tokeninfo(&mut ref_37);
    /* ***************************************************************/
    /* decode_krb5_pa_otp_challenge */
    let mut ref_38: krb5_pa_otp_challenge =
        krb5_pa_otp_challenge{nonce:
                                  krb5_data{magic: 0,
                                            length: 0,
                                            data: 0 as *mut libc::c_char,},
                              service:
                                  krb5_data{magic: 0,
                                            length: 0,
                                            data: 0 as *mut libc::c_char,},
                              tokeninfo: 0 as *mut *mut krb5_otp_tokeninfo,
                              salt:
                                  krb5_data{magic: 0,
                                            length: 0,
                                            data: 0 as *mut libc::c_char,},
                              s2kparams:
                                  krb5_data{magic: 0,
                                            length: 0,
                                            data: 0 as *mut libc::c_char,},};
    let mut var_37: *mut krb5_pa_otp_challenge =
        0 as *mut krb5_pa_otp_challenge;
    ktest_make_minimal_pa_otp_challenge(&mut ref_38);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 15 80 08 6D 69 6E 6E 6F 6E 63 65 A2 09 30 07 80 05 00 00 00 00 00\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_otp_challenge\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_otp_challenge(&mut code, &mut var_37);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_otp_challenge\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_otp_challenge(&mut ref_38, var_37) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_otp_challenge\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_pa_otp_challenge(test_context, var_37);
    ktest_empty_pa_otp_challenge(&mut ref_38);
    let mut ref_39: krb5_pa_otp_challenge =
        krb5_pa_otp_challenge{nonce:
                                  krb5_data{magic: 0,
                                            length: 0,
                                            data: 0 as *mut libc::c_char,},
                              service:
                                  krb5_data{magic: 0,
                                            length: 0,
                                            data: 0 as *mut libc::c_char,},
                              tokeninfo: 0 as *mut *mut krb5_otp_tokeninfo,
                              salt:
                                  krb5_data{magic: 0,
                                            length: 0,
                                            data: 0 as *mut libc::c_char,},
                              s2kparams:
                                  krb5_data{magic: 0,
                                            length: 0,
                                            data: 0 as *mut libc::c_char,},};
    let mut var_38: *mut krb5_pa_otp_challenge =
        0 as *mut krb5_pa_otp_challenge;
    ktest_make_maximal_pa_otp_challenge(&mut ref_39);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 81 A5 80 08 6D 61 78 6E 6F 6E 63 65 81 0B 74 65 73 74 73 65 72 76 69 63 65 A2 7D 30 07 80 05 00 00 00 00 00 30 72 80 05 00 77 00 00 00 81 0B 45 78 61 6D 70 6C 65 63 6F 72 70 82 05 68 61 72 6B 21 83 01 0A 84 01 02 85 09 79 6F 75 72 74 6F 6B 65 6E 86 28 75 72 6E 3A 69 65 74 66 3A 70 61 72 61 6D 73 3A 78 6D 6C 3A 6E 73 3A 6B 65 79 70 72 6F 76 3A 70 73 6B 63 3A 68 6F 74 70 A7 16 30 0B 06 09 60 86 48 01 65 03 04 02 01 30 07 06 05 2B 0E 03 02 1A 88 02 03 E8 83 07 6B 65 79 73 61 6C 74 84 04 31 32 33 34\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_otp_challenge\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_otp_challenge(&mut code, &mut var_38);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_otp_challenge\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_otp_challenge(&mut ref_39, var_38) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_otp_challenge\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_pa_otp_challenge(test_context, var_38);
    ktest_empty_pa_otp_challenge(&mut ref_39);
    /* ***************************************************************/
    /* decode_krb5_pa_otp_req */
    let mut ref_40: krb5_pa_otp_req =
        krb5_pa_otp_req{flags: 0,
                        nonce:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        enc_data:
                            krb5_enc_data{magic: 0,
                                          enctype: 0,
                                          kvno: 0,
                                          ciphertext:
                                              krb5_data{magic: 0,
                                                        length: 0,
                                                        data:
                                                            0 as
                                                                *mut libc::c_char,},},
                        hash_alg: 0 as *mut krb5_algorithm_identifier,
                        iteration_count: 0,
                        otp_value:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        pin:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        challenge:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        time: 0,
                        counter:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        format: 0,
                        token_id:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        alg_id:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        vendor:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},};
    let mut var_39: *mut krb5_pa_otp_req = 0 as *mut krb5_pa_otp_req;
    ktest_make_minimal_pa_otp_req(&mut ref_40);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 2C 80 05 00 00 00 00 00 A2 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_otp_req\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_otp_req(&mut code, &mut var_39);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_otp_req\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_otp_req(&mut ref_40, var_39) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_otp_req\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_pa_otp_req(test_context, var_39);
    ktest_empty_pa_otp_req(&mut ref_40);
    let mut ref_41: krb5_pa_otp_req =
        krb5_pa_otp_req{flags: 0,
                        nonce:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        enc_data:
                            krb5_enc_data{magic: 0,
                                          enctype: 0,
                                          kvno: 0,
                                          ciphertext:
                                              krb5_data{magic: 0,
                                                        length: 0,
                                                        data:
                                                            0 as
                                                                *mut libc::c_char,},},
                        hash_alg: 0 as *mut krb5_algorithm_identifier,
                        iteration_count: 0,
                        otp_value:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        pin:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        challenge:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        time: 0,
                        counter:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        format: 0,
                        token_id:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        alg_id:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},
                        vendor:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},};
    let mut var_40: *mut krb5_pa_otp_req = 0 as *mut krb5_pa_otp_req;
    ktest_make_maximal_pa_otp_req(&mut ref_41);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 81 B9 80 05 00 60 00 00 00 81 05 6E 6F 6E 63 65 A2 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 A3 0B 06 09 60 86 48 01 65 03 04 02 01 84 02 03 E8 85 05 66 72 6F 67 73 86 0A 6D 79 66 69 72 73 74 70 69 6E 87 05 68 61 72 6B 21 88 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A 89 03 33 34 36 8A 01 02 8B 09 79 6F 75 72 74 6F 6B 65 6E 8C 28 75 72 6E 3A 69 65 74 66 3A 70 61 72 61 6D 73 3A 78 6D 6C 3A 6E 73 3A 6B 65 79 70 72 6F 76 3A 70 73 6B 63 3A 68 6F 74 70 8D 0B 45 78 61 6D 70 6C 65 63 6F 72 70\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_otp_req\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_otp_req(&mut code, &mut var_40);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_otp_req\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_otp_req(&mut ref_41, var_40) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_otp_req\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_pa_otp_req(test_context, var_40);
    ktest_empty_pa_otp_req(&mut ref_41);
    /* ***************************************************************/
    /* decode_krb5_pa_otp_enc_req */
    let mut ref_42: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut var_41: *mut krb5_data = 0 as *mut krb5_data;
    ktest_make_sample_data(&mut ref_42);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 0A 80 08 6B 72 62 35 64 61 74 61\x00" as
                                *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_otp_enc_req\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_otp_enc_req(&mut code, &mut var_41);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_otp_enc_req\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_data(&mut ref_42, var_41) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_otp_enc_req\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_data(test_context, var_41);
    ktest_empty_data(&mut ref_42);
    /* ***************************************************************/
    /* decode_krb5_kkdcp_message */
    let mut ref_43: krb5_kkdcp_message =
        krb5_kkdcp_message{kerb_message:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           target_domain:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           dclocator_hint: 0,};
    let mut var_42: *mut krb5_kkdcp_message = 0 as *mut krb5_kkdcp_message;
    ktest_make_sample_kkdcp_message(&mut ref_43);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 82 01 FC A0 82 01 EC 04 82 01 E8 6A 82 01 E4 30 82 01 E0 A1 03 02 01 05 A2 03 02 01 0A A3 26 30 24 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 A4 82 01 AA 30 82 01 A6 A0 07 03 05 00 FE DC BA 98 A1 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A2 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A3 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A4 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A5 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A6 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A7 03 02 01 2A A8 08 30 06 02 01 00 02 01 01 A9 20 30 1E 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 30 0D A0 03 02 01 02 A1 06 04 04 12 D0 00 23 AA 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 AB 81 BF 30 81 BC 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 61 5C 30 5A A0 03 02 01 05 A1 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A2 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65 A1 0A 1B 08 6B 72 62 35 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"kkdcp_message\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_kkdcp_message(&mut code, &mut var_42);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"kkdcp_message\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_kkdcp_message(&mut ref_43, var_42) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"kkdcp_message\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    ktest_free_kkdcp_message(test_context, var_42);
    ktest_empty_kkdcp_message(&mut ref_43);
    /* ***************************************************************/
    /* decode_krb5_cammac */
    let mut ref_44: krb5_cammac =
        krb5_cammac{elements: 0 as *mut *mut krb5_authdata,
                    kdc_verifier: 0 as *mut krb5_verifier_mac,
                    svc_verifier: 0 as *mut krb5_verifier_mac,
                    other_verifiers: 0 as *mut *mut krb5_verifier_mac,};
    let mut var_43: *mut krb5_cammac = 0 as *mut krb5_cammac;
    ktest_make_minimal_cammac(&mut ref_44);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 12 A0 10 30 0E 30 0C A0 03 02 01 01 A1 05 04 03 61 64 31\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"cammac\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_cammac(&mut code, &mut var_43);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"cammac\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_cammac(&mut ref_44, var_43) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"cammac\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_cammac(test_context, var_43);
    ktest_empty_cammac(&mut ref_44);
    let mut ref_45: krb5_cammac =
        krb5_cammac{elements: 0 as *mut *mut krb5_authdata,
                    kdc_verifier: 0 as *mut krb5_verifier_mac,
                    svc_verifier: 0 as *mut krb5_verifier_mac,
                    other_verifiers: 0 as *mut *mut krb5_verifier_mac,};
    let mut var_44: *mut krb5_cammac = 0 as *mut krb5_cammac;
    ktest_make_maximal_cammac(&mut ref_45);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 81 F2 A0 1E 30 1C 30 0C A0 03 02 01 01 A1 05 04 03 61 64 31 30 0C A0 03 02 01 02 A1 05 04 03 61 64 32 A1 3D 30 3B A0 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A1 03 02 01 05 A2 03 02 01 10 A3 13 30 11 A0 03 02 01 01 A1 0A 04 08 63 6B 73 75 6D 6B 64 63 A2 3D 30 3B A0 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A1 03 02 01 05 A2 03 02 01 10 A3 13 30 11 A0 03 02 01 01 A1 0A 04 08 63 6B 73 75 6D 73 76 63 A3 52 30 50 30 13 A3 11 30 0F A0 03 02 01 01 A1 08 04 06 63 6B 73 75 6D 31 30 39 A0 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61 A1 03 02 01 05 A2 03 02 01 10 A3 11 30 0F A0 03 02 01 01 A1 08 04 06 63 6B 73 75 6D 32\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"cammac\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_cammac(&mut code, &mut var_44);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"cammac\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_cammac(&mut ref_45, var_44) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"cammac\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_cammac(test_context, var_44);
    ktest_empty_cammac(&mut ref_45);
    /* ***************************************************************/
    /* decode_krb5_secure_cookie */
    let mut ref_46: krb5_secure_cookie =
        krb5_secure_cookie{time: 0, data: 0 as *mut *mut krb5_pa_data,};
    let mut var_45: *mut krb5_secure_cookie = 0 as *mut krb5_secure_cookie;
    ktest_make_sample_secure_cookie(&mut ref_46);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 2C 02 04 2D F8 02 25 30 24 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61 30 10 A1 03 02 01 0D A2 09 04 07 70 61 2D 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"secure_cookie\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_secure_cookie(&mut code, &mut var_45);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"secure_cookie\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_secure_cookie(&mut ref_46, var_45) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"secure_cookie\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_secure_cookie(test_context, var_45);
    ktest_empty_secure_cookie(&mut ref_46);
    /* ***************************************************************/
    /* decode_krb5_spake_factor */
    let mut ref_47: krb5_spake_factor =
        krb5_spake_factor{type_0: 0, data: 0 as *mut krb5_data,};
    let mut var_46: *mut krb5_spake_factor = 0 as *mut krb5_spake_factor;
    ktest_make_minimal_spake_factor(&mut ref_47);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 05 A0 03 02 01 01\x00" as *const u8 as
                                *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"spake_factor\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_spake_factor(&mut code, &mut var_46);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"spake_factor\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_spake_factor(&mut ref_47, var_46) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"spake_factor\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(optionals NULL)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_spake_factor(test_context, var_46);
    ktest_empty_spake_factor(&mut ref_47);
    let mut ref_48: krb5_spake_factor =
        krb5_spake_factor{type_0: 0, data: 0 as *mut krb5_data,};
    let mut var_47: *mut krb5_spake_factor = 0 as *mut krb5_spake_factor;
    ktest_make_maximal_spake_factor(&mut ref_48);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 0E A0 03 02 01 02 A1 07 04 05 66 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"spake_factor\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_spake_factor(&mut code, &mut var_47);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"spake_factor\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_spake_factor(&mut ref_48, var_47) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"spake_factor\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_spake_factor(test_context, var_47);
    ktest_empty_spake_factor(&mut ref_48);
    /* ***************************************************************/
    /* decode_krb5_pa_spake */
    let mut ref_49: krb5_pa_spake =
        krb5_pa_spake{choice: SPAKE_MSGTYPE_SUPPORT,
                      u:
                          krb5_spake_message_choices{support:
                                                         krb5_spake_support{ngroups:
                                                                                0,
                                                                            groups:
                                                                                0
                                                                                    as
                                                                                    *mut int32_t,},},};
    let mut var_48: *mut krb5_pa_spake = 0 as *mut krb5_pa_spake;
    ktest_make_support_pa_spake(&mut ref_49);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"A0 0C 30 0A A0 08 30 06 02 01 01 02 01 02\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_spake\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_spake(&mut code, &mut var_48);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_spake\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_spake(&mut ref_49, var_48) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_spake\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(support)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_pa_spake(test_context, var_48);
    ktest_empty_pa_spake(&mut ref_49);
    let mut ref_50: krb5_pa_spake =
        krb5_pa_spake{choice: SPAKE_MSGTYPE_SUPPORT,
                      u:
                          krb5_spake_message_choices{support:
                                                         krb5_spake_support{ngroups:
                                                                                0,
                                                                            groups:
                                                                                0
                                                                                    as
                                                                                    *mut int32_t,},},};
    let mut var_49: *mut krb5_pa_spake = 0 as *mut krb5_pa_spake;
    ktest_make_challenge_pa_spake(&mut ref_50);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"A1 2D 30 2B A0 03 02 01 01 A1 09 04 07 54 20 76 61 6C 75 65 A2 19 30 17 30 05 A0 03 02 01 01 30 0E A0 03 02 01 02 A1 07 04 05 66 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_spake\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_spake(&mut code, &mut var_49);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_spake\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_spake(&mut ref_50, var_49) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_spake\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(challenge)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_pa_spake(test_context, var_49);
    ktest_empty_pa_spake(&mut ref_50);
    let mut ref_51: krb5_pa_spake =
        krb5_pa_spake{choice: SPAKE_MSGTYPE_SUPPORT,
                      u:
                          krb5_spake_message_choices{support:
                                                         krb5_spake_support{ngroups:
                                                                                0,
                                                                            groups:
                                                                                0
                                                                                    as
                                                                                    *mut int32_t,},},};
    let mut var_50: *mut krb5_pa_spake = 0 as *mut krb5_pa_spake;
    ktest_make_response_pa_spake(&mut ref_51);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"A2 34 30 32 A0 09 04 07 53 20 76 61 6C 75 65 A1 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_spake\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_spake(&mut code, &mut var_50);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_spake\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_spake(&mut ref_51, var_50) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_spake\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(response)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_pa_spake(test_context, var_50);
    ktest_empty_pa_spake(&mut ref_51);
    let mut ref_52: krb5_pa_spake =
        krb5_pa_spake{choice: SPAKE_MSGTYPE_SUPPORT,
                      u:
                          krb5_spake_message_choices{support:
                                                         krb5_spake_support{ngroups:
                                                                                0,
                                                                            groups:
                                                                                0
                                                                                    as
                                                                                    *mut int32_t,},},};
    let mut var_51: *mut krb5_pa_spake = 0 as *mut krb5_pa_spake;
    ktest_make_encdata_pa_spake(&mut ref_52);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"A3 25 30 23 A0 03 02 01 00 A1 03 02 01 05 A2 17 04 15 6B 72 62 41 53 4E 2E 31 20 74 65 73 74 20 6D 65 73 73 61 67 65\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"pa_spake\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = decode_krb5_pa_spake(&mut code, &mut var_51);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"pa_spake\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_spake(&mut ref_52, var_51) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"pa_spake\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(encdata)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    k5_free_pa_spake(test_context, var_51);
    ktest_empty_pa_spake(&mut ref_52);
    /* ***************************************************************/
    /* decode_krb5_pa_pk_as_req */
    let mut ref_53: krb5_pa_pk_as_req =
        krb5_pa_pk_as_req{signedAuthPack:
                              krb5_data{magic: 0,
                                        length: 0,
                                        data: 0 as *mut libc::c_char,},
                          trustedCertifiers:
                              0 as
                                  *mut *mut krb5_external_principal_identifier,
                          kdcPkId:
                              krb5_data{magic: 0,
                                        length: 0,
                                        data: 0 as *mut libc::c_char,},};
    let mut var_52: *mut krb5_pa_pk_as_req = 0 as *mut krb5_pa_pk_as_req;
    ktest_make_sample_pa_pk_as_req(&mut ref_53);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 38 80 08 6B 72 62 35 64 61 74 61 A1 22 30 20 30 1E 80 08 6B 72 62 35 64 61 74 61 81 08 6B 72 62 35 64 61 74 61 82 08 6B 72 62 35 64 61 74 61 82 08 6B 72 62 35 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_pa_pk_as_req\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval =
        acc.decode_krb5_pa_pk_as_req.expect("non-null function pointer")(&mut code,
                                                                         &mut var_52);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_pa_pk_as_req\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_pk_as_req(&mut ref_53, var_52) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"krb5_pa_pk_as_req\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    ktest_free_pa_pk_as_req(test_context, var_52);
    ktest_empty_pa_pk_as_req(&mut ref_53);
    /* ***************************************************************/
    /* decode_krb5_pa_pk_as_rep */
    let mut ref_54: krb5_pa_pk_as_rep =
        krb5_pa_pk_as_rep{choice: choice_pa_pk_as_rep_dhInfo,
                          u:
                              krb5_pa_pk_as_rep_choices{dh_Info:
                                                            krb5_dh_rep_info{dhSignedData:
                                                                                 krb5_data{magic:
                                                                                               0,
                                                                                           length:
                                                                                               0,
                                                                                           data:
                                                                                               0
                                                                                                   as
                                                                                                   *mut libc::c_char,},
                                                                             serverDHNonce:
                                                                                 krb5_data{magic:
                                                                                               0,
                                                                                           length:
                                                                                               0,
                                                                                           data:
                                                                                               0
                                                                                                   as
                                                                                                   *mut libc::c_char,},
                                                                             kdfID:
                                                                                 0
                                                                                     as
                                                                                     *mut krb5_data,},},};
    let mut var_53: *mut krb5_pa_pk_as_rep = 0 as *mut krb5_pa_pk_as_rep;
    ktest_make_sample_pa_pk_as_rep_dhInfo(&mut ref_54);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"A0 28 30 26 80 08 6B 72 62 35 64 61 74 61 A1 0A 04 08 6B 72 62 35 64 61 74 61 A2 0E 30 0C A0 0A 06 08 6B 72 62 35 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_pa_pk_as_rep\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval =
        acc.decode_krb5_pa_pk_as_rep.expect("non-null function pointer")(&mut code,
                                                                         &mut var_53);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_pa_pk_as_rep\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_pk_as_rep(&mut ref_54, var_53) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"krb5_pa_pk_as_rep\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(dhInfo)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    ktest_free_pa_pk_as_rep(test_context, var_53);
    ktest_empty_pa_pk_as_rep(&mut ref_54);
    let mut ref_55: krb5_pa_pk_as_rep =
        krb5_pa_pk_as_rep{choice: choice_pa_pk_as_rep_dhInfo,
                          u:
                              krb5_pa_pk_as_rep_choices{dh_Info:
                                                            krb5_dh_rep_info{dhSignedData:
                                                                                 krb5_data{magic:
                                                                                               0,
                                                                                           length:
                                                                                               0,
                                                                                           data:
                                                                                               0
                                                                                                   as
                                                                                                   *mut libc::c_char,},
                                                                             serverDHNonce:
                                                                                 krb5_data{magic:
                                                                                               0,
                                                                                           length:
                                                                                               0,
                                                                                           data:
                                                                                               0
                                                                                                   as
                                                                                                   *mut libc::c_char,},
                                                                             kdfID:
                                                                                 0
                                                                                     as
                                                                                     *mut krb5_data,},},};
    let mut var_54: *mut krb5_pa_pk_as_rep = 0 as *mut krb5_pa_pk_as_rep;
    ktest_make_sample_pa_pk_as_rep_encKeyPack(&mut ref_55);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"81 08 6B 72 62 35 64 61 74 61\x00" as *const u8
                                as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_pa_pk_as_rep\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval =
        acc.decode_krb5_pa_pk_as_rep.expect("non-null function pointer")(&mut code,
                                                                         &mut var_54);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_pa_pk_as_rep\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_pa_pk_as_rep(&mut ref_55, var_54) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"krb5_pa_pk_as_rep\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"(encKeyPack)\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    ktest_free_pa_pk_as_rep(test_context, var_54);
    ktest_empty_pa_pk_as_rep(&mut ref_55);
    /* ***************************************************************/
    /* decode_krb5_auth_pack */
    let mut ref_56: krb5_auth_pack =
        krb5_auth_pack{pkAuthenticator:
                           krb5_pk_authenticator{cusec: 0,
                                                 ctime: 0,
                                                 nonce: 0,
                                                 paChecksum:
                                                     krb5_checksum{magic: 0,
                                                                   checksum_type:
                                                                       0,
                                                                   length: 0,
                                                                   contents:
                                                                       0 as
                                                                           *mut krb5_octet,},
                                                 freshnessToken:
                                                     0 as *mut krb5_data,},
                       clientPublicValue: 0 as *mut krb5_subject_pk_info,
                       supportedCMSTypes:
                           0 as *mut *mut krb5_algorithm_identifier,
                       clientDHNonce:
                           krb5_data{magic: 0,
                                     length: 0,
                                     data: 0 as *mut libc::c_char,},
                       supportedKDFs: 0 as *mut *mut krb5_data,};
    let mut var_55: *mut krb5_auth_pack = 0 as *mut krb5_auth_pack;
    ktest_make_sample_auth_pack(&mut ref_56);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 81 93 A0 29 30 27 A0 05 02 03 01 E2 40 A1 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A A2 03 02 01 2A A3 06 04 04 31 32 33 34 A1 22 30 20 30 13 06 09 2A 86 48 86 F7 12 01 02 02 04 06 70 61 72 61 6D 73 03 09 00 6B 72 62 35 64 61 74 61 A2 24 30 22 30 13 06 09 2A 86 48 86 F7 12 01 02 02 04 06 70 61 72 61 6D 73 30 0B 06 09 2A 86 48 86 F7 12 01 02 02 A3 0A 04 08 6B 72 62 35 64 61 74 61 A4 10 30 0E 30 0C A0 0A 06 08 6B 72 62 35 64 61 74 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_auth_pack\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval =
        acc.decode_krb5_auth_pack.expect("non-null function pointer")(&mut code,
                                                                      &mut var_55);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_auth_pack\x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_auth_pack(&mut ref_56, var_55) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"krb5_auth_pack\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    ktest_free_auth_pack(test_context, var_55);
    ktest_empty_auth_pack(&mut ref_56);
    /* ***************************************************************/
    /* decode_krb5_kdc_dh_key_info */
    let mut ref_57: krb5_kdc_dh_key_info =
        krb5_kdc_dh_key_info{subjectPublicKey:
                                 krb5_data{magic: 0,
                                           length: 0,
                                           data: 0 as *mut libc::c_char,},
                             nonce: 0,
                             dhKeyExpiration: 0,};
    let mut var_56: *mut krb5_kdc_dh_key_info =
        0 as *mut krb5_kdc_dh_key_info;
    ktest_make_sample_kdc_dh_key_info(&mut ref_57);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 25 A0 0B 03 09 00 6B 72 62 35 64 61 74 61 A1 03 02 01 2A A2 11 18 0F 31 39 39 34 30 36 31 30 30 36 30 33 31 37 5A\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_kdc_dh_key_info\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval =
        acc.decode_krb5_kdc_dh_key_info.expect("non-null function pointer")(&mut code,
                                                                            &mut var_56);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_kdc_dh_key_info\x00" as *const u8 as
                    *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_kdc_dh_key_info(&mut ref_57, var_56) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"krb5_kdc_dh_key_info\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    ktest_free_kdc_dh_key_info(test_context, var_56);
    ktest_empty_kdc_dh_key_info(&mut ref_57);
    /* ***************************************************************/
    /* decode_krb5_reply_key_pack */
    let mut ref_58: krb5_reply_key_pack =
        krb5_reply_key_pack{replyKey:
                                krb5_keyblock{magic: 0,
                                              enctype: 0,
                                              length: 0,
                                              contents:
                                                  0 as *mut krb5_octet,},
                            asChecksum:
                                krb5_checksum{magic: 0,
                                              checksum_type: 0,
                                              length: 0,
                                              contents:
                                                  0 as *mut krb5_octet,},};
    let mut var_57: *mut krb5_reply_key_pack = 0 as *mut krb5_reply_key_pack;
    ktest_make_sample_reply_key_pack(&mut ref_58);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 26 A0 13 30 11 A0 03 02 01 01 A1 0A 04 08 31 32 33 34 35 36 37 38 A1 0F 30 0D A0 03 02 01 01 A1 06 04 04 31 32 33 34\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_reply_key_pack\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval =
        acc.decode_krb5_reply_key_pack.expect("non-null function pointer")(&mut code,
                                                                           &mut var_57);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_reply_key_pack\x00" as *const u8 as
                    *const libc::c_char);
        error_count += 1
    }
    if ktest_equal_reply_key_pack(&mut ref_58, var_57) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"krb5_reply_key_pack\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    ktest_free_reply_key_pack(test_context, var_57);
    ktest_empty_reply_key_pack(&mut ref_58);
    /* ***************************************************************/
    /* decode_krb5_principal_name */
    /* We have no encoder for this type (KerberosName from RFC 4556); the
     * encoding is hand-generated. */
    let mut ref_59: krb5_principal = 0 as *mut krb5_principal_data;
    let mut var_58: krb5_principal = 0 as *mut krb5_principal_data;
    ktest_make_sample_principal(&mut ref_59);
    retval =
        krb5_data_hex_parse(&mut code,
                            b"30 2E A0 10 1B 0E 41 54 48 45 4E 41 2E 4D 49 54 2E 45 44 55 A1 1A 30 18 A0 03 02 01 01 A1 11 30 0F 1B 06 68 66 74 73 61 69 1B 05 65 78 74 72 61\x00"
                                as *const u8 as *const libc::c_char);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while parsing %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_principal_name\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval =
        acc.decode_krb5_principal_name.expect("non-null function pointer")(&mut code,
                                                                           &mut var_58);
    if retval != 0 {
        com_err(b"krb5_decode_test\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while decoding %s\x00" as *const u8 as *const libc::c_char,
                b"krb5_principal_name\x00" as *const u8 as
                    *const libc::c_char);
        error_count += 1
    }
    if equal_principal(&mut ref_59, var_58) != 0 {
        printf(b"OK: \x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"ERROR: \x00" as *const u8 as *const libc::c_char);
        error_count += 1
    }
    printf(b"krb5_principal_name\x00" as *const u8 as *const libc::c_char);
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           b"\x00" as *const u8 as *const libc::c_char);
    krb5_free_data_contents(test_context, &mut code);
    krb5_free_principal(test_context, var_58);
    /* not DISABLE_PKINIT */
    krb5_free_context(test_context);
    exit(error_count);
}
#[no_mangle]
#[c2rust::src_loc = "1232:1"]
pub unsafe extern "C" fn krb5_ktest_free_enc_data(mut context: krb5_context,
                                                  mut val:
                                                      *mut krb5_enc_data) {
    if !val.is_null() {
        krb5_free_data_contents(context, &mut (*val).ciphertext);
        free(val as *mut libc::c_void);
    };
}
/* Glue function to make ktest_equal_principal_data look like what decode_run
 * expects. */
#[c2rust::src_loc = "1244:1"]
unsafe extern "C" fn equal_principal(mut ref_0: *mut krb5_principal,
                                     mut var: krb5_principal) -> libc::c_int {
    return ktest_equal_principal_data(*ref_0, var);
}
#[c2rust::src_loc = "1250:1"]
unsafe extern "C" fn ktest_free_auth_pack(mut context: krb5_context,
                                          mut val: *mut krb5_auth_pack) {
    if !val.is_null() { ktest_empty_auth_pack(val); }
    free(val as *mut libc::c_void);
}
#[c2rust::src_loc = "1258:1"]
unsafe extern "C" fn ktest_free_kdc_dh_key_info(mut context: krb5_context,
                                                mut val:
                                                    *mut krb5_kdc_dh_key_info) {
    if !val.is_null() { ktest_empty_kdc_dh_key_info(val); }
    free(val as *mut libc::c_void);
}
#[c2rust::src_loc = "1266:1"]
unsafe extern "C" fn ktest_free_pa_pk_as_req(mut context: krb5_context,
                                             mut val:
                                                 *mut krb5_pa_pk_as_req) {
    if !val.is_null() { ktest_empty_pa_pk_as_req(val); }
    free(val as *mut libc::c_void);
}
#[c2rust::src_loc = "1274:1"]
unsafe extern "C" fn ktest_free_pa_pk_as_rep(mut context: krb5_context,
                                             mut val:
                                                 *mut krb5_pa_pk_as_rep) {
    if !val.is_null() { ktest_empty_pa_pk_as_rep(val); }
    free(val as *mut libc::c_void);
}
#[c2rust::src_loc = "1282:1"]
unsafe extern "C" fn ktest_free_reply_key_pack(mut context: krb5_context,
                                               mut val:
                                                   *mut krb5_reply_key_pack) {
    if !val.is_null() { ktest_empty_reply_key_pack(val); }
    free(val as *mut libc::c_void);
}
/* not DISABLE_PKINIT */
#[c2rust::src_loc = "1292:1"]
unsafe extern "C" fn ktest_free_kkdcp_message(mut context: krb5_context,
                                              mut val:
                                                  *mut krb5_kkdcp_message) {
    if !val.is_null() { ktest_empty_kkdcp_message(val); }
    free(val as *mut libc::c_void);
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
