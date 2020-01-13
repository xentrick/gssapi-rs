use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
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
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
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
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
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
#[c2rust::header_src = "/usr/include/unistd.h:27"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
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
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
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
    /* #define      TKT_FLG_RESERVED        0x00004000 */
/* #define      TKT_FLG_RESERVED        0x00002000 */
/* #define      TKT_FLG_RESERVED        0x00001000 */
/* #define      TKT_FLG_RESERVED        0x00000800 */
/* #define      TKT_FLG_RESERVED        0x00000400 */
/* #define      TKT_FLG_RESERVED        0x00000200 */
/* #define      TKT_FLG_RESERVED        0x00000100 */
/* #define      TKT_FLG_RESERVED        0x00000080 */
/* #define      TKT_FLG_RESERVED        0x00000040 */
/* #define      TKT_FLG_RESERVED        0x00000020 */
/* #define      TKT_FLG_RESERVED        0x00000010 */
/* #define      TKT_FLG_RESERVED        0x00000008 */
/* #define      TKT_FLG_RESERVED        0x00000004 */
/* #define      TKT_FLG_RESERVED        0x00000002 */
/* #define      TKT_FLG_RESERVED        0x00000001 */
    /* definitions for lr_type fields. */
    /* definitions for msec direction bit for KRB_SAFE, KRB_PRIV */
    /*
 * end "fieldbits.h"
 */
    /*
 * begin "proto.h"
 */
    /* * Protocol version number */
    /* Message types */
    /* *< Initial authentication request */
    /* *< Response to AS request */
    /* *< Ticket granting server request */
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
    /*
 * end "safepriv.h"
 */
    /*
 * begin "ccache.h"
 */
    /* * Cursor for sequential lookup */
    #[c2rust::src_loc = "2278:1"]
    pub type krb5_cc_cursor = krb5_pointer;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    /* * Cursor for iterating over all ccaches */
    #[c2rust::src_loc = "2287:1"]
    pub type krb5_cccol_cursor = *mut _krb5_cccol_cursor;
    /*
 * end "rcache.h"
 */
    /*
 * begin "keytab.h"
 */
    /* XXX */
    /* *< Long enough for MAXPATHLEN + some extra */
    #[c2rust::src_loc = "2724:1"]
    pub type krb5_kt_cursor = krb5_pointer;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2727:16"]
    pub struct krb5_keytab_entry_st {
        pub magic: krb5_magic,
        pub principal: krb5_principal,
        pub timestamp: krb5_timestamp,
        pub vno: krb5_kvno,
        pub key: krb5_keyblock,
    }
    /* * A key table entry. */
    #[c2rust::src_loc = "2727:1"]
    pub type krb5_keytab_entry = krb5_keytab_entry_st;
    #[c2rust::src_loc = "2736:1"]
    pub type krb5_keytab = *mut _krb5_kt;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, _krb5_kt};
    use super::stddef_h::size_t;
    extern "C" {
        /* *< Principal of this key */
        /* *< Time entry written to keytable */
        /* *< Key version number */
        /* *< The secret key */
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
        #[c2rust::src_loc = "2285:8"]
        pub type _krb5_cccol_cursor;
        /* Flags for krb5_cc_retrieve_cred. */
/* * The requested lifetime must be at least as great as the time specified. */
        /* * The is_skey field must match exactly. */
        /* * All the flags set in the match credentials must be set. */
        /* * All the time fields must match exactly. */
        /* * All the flags must match exactly. */
        /* * The authorization data must match. */
        /* * Only the name portion of the principal name must match. */
        /* * The second ticket must match. */
        /* * The encryption key type must match. */
        /* * The supported key types must match. */
        /* Flags for krb5_cc_set_flags and similar. */
/* * Open and close the file for each cache operation. */
        /* *< @deprecated has no effect */
        /* *
 * Retrieve the name, but not type of a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * @warning Returns the name of the credential cache.  The result is an alias
 * into @a cache and should not be freed or modified by the caller.  This name
 * does not include the cache type, so should not be used as input to
 * krb5_cc_resolve().
 *
 * @return
 * On success - the name of the credential cache.
 */
        #[no_mangle]
        #[c2rust::src_loc = "2330:1"]
        pub fn krb5_cc_get_name(context_0: krb5_context, cache: krb5_ccache)
         -> *const libc::c_char;
        /* *
 * Retrieve the full name of a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] fullname_out    Full name of cache
 *
 * Use krb5_free_string() to free @a fullname_out when it is no longer needed.
 *
 * @version New in 1.10
 */
        #[no_mangle]
        #[c2rust::src_loc = "2344:1"]
        pub fn krb5_cc_get_full_name(context_0: krb5_context,
                                     cache: krb5_ccache,
                                     fullname_out: *mut *mut libc::c_char)
         -> krb5_error_code;
        /* *
 * Close a credential cache handle.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * This function closes a credential cache handle @a cache without affecting
 * the contents of the cache.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2403:1"]
        pub fn krb5_cc_close(context_0: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        /* *
 * Get the default principal of a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] principal       Primary principal
 *
 * Returns the default client principal of a credential cache as set by
 * krb5_cc_initialize().
 *
 * Use krb5_free_principal() to free @a principal when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2479:1"]
        pub fn krb5_cc_get_principal(context_0: krb5_context,
                                     cache: krb5_ccache,
                                     principal: *mut krb5_principal)
         -> krb5_error_code;
        /* *
 * Prepare to sequentially read every credential in a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] cursor          Cursor
 *
 * krb5_cc_end_seq_get() must be called to complete the retrieve operation.
 *
 * @note If the cache represented by @a cache is modified between the time of
 * the call to this function and the time of the final krb5_cc_end_seq_get(),
 * these changes may not be reflected in the results of krb5_cc_next_cred()
 * calls.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2499:1"]
        pub fn krb5_cc_start_seq_get(context_0: krb5_context,
                                     cache: krb5_ccache,
                                     cursor: *mut krb5_cc_cursor)
         -> krb5_error_code;
        /* *
 * Retrieve the next entry from the credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [in]  cursor          Cursor
 * @param [out] creds           Next credential cache entry
 *
 * This function fills in @a creds with the next entry in @a cache and advances
 * @a cursor.
 *
 * Use krb5_free_cred_contents() to free @a creds when it is no longer needed.
 *
 * @sa krb5_cc_start_seq_get(), krb5_end_seq_get()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2520:1"]
        pub fn krb5_cc_next_cred(context_0: krb5_context, cache: krb5_ccache,
                                 cursor: *mut krb5_cc_cursor,
                                 creds: *mut krb5_creds) -> krb5_error_code;
        /* *
 * Finish a series of sequential processing credential cache entries.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] cursor           Cursor
 *
 * This function finishes processing credential cache entries and invalidates
 * @a cursor.
 *
 * @sa krb5_cc_start_seq_get(), krb5_cc_next_cred()
 *
 * @retval 0 (always)
 */
        #[no_mangle]
        #[c2rust::src_loc = "2538:1"]
        pub fn krb5_cc_end_seq_get(context_0: krb5_context,
                                   cache: krb5_ccache,
                                   cursor: *mut krb5_cc_cursor)
         -> krb5_error_code;
        /* *
 * Retrieve the type of a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * @return The type of a credential cache as an alias that must not be modified
 * or freed by the caller.
 */
        #[no_mangle]
        #[c2rust::src_loc = "2598:1"]
        pub fn krb5_cc_get_type(context_0: krb5_context, cache: krb5_ccache)
         -> *const libc::c_char;
        /* *
 * Prepare to iterate over the collection of known credential caches.
 *
 * @param [in]  context         Library context
 * @param [out] cursor          Cursor
 *
 * Get a new cache iteration @a cursor that will iterate over all known
 * credential caches independent of type.
 *
 * Use krb5_cccol_cursor_free() to release @a cursor when it is no longer
 * needed.
 *
 * @sa krb5_cccol_cursor_next()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2635:1"]
        pub fn krb5_cccol_cursor_new(context_0: krb5_context,
                                     cursor: *mut krb5_cccol_cursor)
         -> krb5_error_code;
        /* *
 * Get the next credential cache in the collection.
 *
 * @param [in]  context         Library context
 * @param [in]  cursor          Cursor
 * @param [out] ccache          Credential cache handle
 *
 * @note When all caches are iterated over and the end of the list is reached,
 * @a ccache is set to NULL.
 *
 * Use krb5_cc_close() to close @a ccache when it is no longer needed.
 *
 * @sa krb5_cccol_cursor_new(), krb5_cccol_cursor_free()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2654:1"]
        pub fn krb5_cccol_cursor_next(context_0: krb5_context,
                                      cursor: krb5_cccol_cursor,
                                      ccache: *mut krb5_ccache)
         -> krb5_error_code;
        /* *
 * Free a credential cache collection cursor.
 *
 * @param [in] context          Library context
 * @param [in] cursor           Cursor
 *
 * @sa krb5_cccol_cursor_new(), krb5_cccol_cursor_next()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2668:1"]
        pub fn krb5_cccol_cursor_free(context_0: krb5_context,
                                      cursor: *mut krb5_cccol_cursor)
         -> krb5_error_code;
        /* *
 * Get a key table name.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] name            Key table name
 * @param [in]  namelen         Maximum length to fill in name
 *
 * Fill @a name with the name of @a keytab including the type and delimiter.
 *
 * @sa MAX_KEYTAB_NAME_LEN
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_KT_NAME_TOOLONG  Key table name does not fit in @a namelen bytes
 *
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2770:1"]
        pub fn krb5_kt_get_name(context_0: krb5_context, keytab: krb5_keytab,
                                name: *mut libc::c_char,
                                namelen: libc::c_uint) -> krb5_error_code;
        /* *
 * Start a sequential retrieval of key table entries.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * Prepare to read sequentially every key in the specified key table.  Use
 * krb5_kt_end_seq_get() to release the cursor when it is no longer needed.
 *
 * @sa krb5_kt_next_entry(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2833:1"]
        pub fn krb5_kt_start_seq_get(context_0: krb5_context,
                                     keytab: krb5_keytab,
                                     cursor: *mut krb5_kt_cursor)
         -> krb5_error_code;
        /* *
 * Retrieve the next entry from the key table.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] entry           Returned key table entry
 * @param [in]  cursor          Key table cursor
 *
 * Return the next sequential entry in @a keytab and advance @a cursor.
 * Callers must release the returned entry with krb5_kt_free_entry().
 *
 * @sa krb5_kt_start_seq_get(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_KT_END - if the last entry was reached
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2857:1"]
        pub fn krb5_kt_next_entry(context_0: krb5_context,
                                  keytab: krb5_keytab,
                                  entry: *mut krb5_keytab_entry,
                                  cursor: *mut krb5_kt_cursor)
         -> krb5_error_code;
        /* *
 * Release a keytab cursor.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * This function should be called to release the cursor created by
 * krb5_kt_start_seq_get().
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2876:1"]
        pub fn krb5_kt_end_seq_get(context_0: krb5_context,
                                   keytab: krb5_keytab,
                                   cursor: *mut krb5_kt_cursor)
         -> krb5_error_code;
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
        pub fn krb5_init_context(context_0: *mut krb5_context)
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
        pub fn krb5_unparse_name(context_0: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3664:1"]
        pub fn krb5_principal_compare(context_0: krb5_context,
                                      princ1: krb5_const_principal,
                                      princ2: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "4173:1"]
        pub fn krb5_kt_resolve(context_0: krb5_context,
                               name: *const libc::c_char,
                               ktid: *mut krb5_keytab) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4226:1"]
        pub fn krb5_kt_default(context_0: krb5_context, id: *mut krb5_keytab)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4244:1"]
        pub fn krb5_kt_client_default(context_0: krb5_context,
                                      keytab_out: *mut krb5_keytab)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4257:1"]
        pub fn krb5_free_keytab_entry_contents(context_0: krb5_context,
                                               entry: *mut krb5_keytab_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4403:1"]
        pub fn krb5_cc_set_default_name(context_0: krb5_context,
                                        name: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4425:1"]
        pub fn krb5_cc_default(context_0: krb5_context,
                               ccache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4496:1"]
        pub fn krb5_is_config_principal(context_0: krb5_context,
                                        principal: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context_0: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4644:1"]
        pub fn krb5_free_ticket(context_0: krb5_context,
                                val: *mut krb5_ticket);
        #[no_mangle]
        #[c2rust::src_loc = "4677:1"]
        pub fn krb5_free_cred_contents(context_0: krb5_context,
                                       val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context_0: krb5_context,
                                       val: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "4778:1"]
        pub fn krb5_free_string(context_0: krb5_context,
                                val: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "6341:1"]
        pub fn krb5_enctype_to_name(enctype: krb5_enctype,
                                    shortest: krb5_boolean,
                                    buffer: *mut libc::c_char, buflen: size_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6398:1"]
        pub fn krb5_timestamp_to_sfstring(timestamp: krb5_timestamp,
                                          buffer: *mut libc::c_char,
                                          buflen: size_t,
                                          pad: *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7810:1"]
        pub fn krb5_decode_ticket(code: *const krb5_data,
                                  rep: *mut *mut krb5_ticket)
         -> krb5_error_code;
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
    #[c2rust::src_loc = "2076:8"]
    pub struct _krb5_kt {
        pub magic: krb5_magic,
        pub ops: *const _krb5_kt_ops,
        pub data: krb5_pointer,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2040:16"]
    pub struct _krb5_kt_ops {
        pub magic: krb5_magic,
        pub prefix: *mut libc::c_char,
        pub resolve: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *const libc::c_char,
                                                 _: *mut krb5_keytab)
                                -> krb5_error_code>,
        pub get_name: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_keytab,
                                                  _: *mut libc::c_char,
                                                  _: libc::c_uint)
                                 -> krb5_error_code>,
        pub close: Option<unsafe extern "C" fn(_: krb5_context,
                                               _: krb5_keytab)
                              -> krb5_error_code>,
        pub get: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_keytab,
                                             _: krb5_const_principal,
                                             _: krb5_kvno, _: krb5_enctype,
                                             _: *mut krb5_keytab_entry)
                            -> krb5_error_code>,
        pub start_seq_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: krb5_keytab,
                                                       _: *mut krb5_kt_cursor)
                                      -> krb5_error_code>,
        pub get_next: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_keytab,
                                                  _: *mut krb5_keytab_entry,
                                                  _: *mut krb5_kt_cursor)
                                 -> krb5_error_code>,
        pub end_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: krb5_keytab,
                                                 _: *mut krb5_kt_cursor)
                                -> krb5_error_code>,
        pub add: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_keytab,
                                             _: *mut krb5_keytab_entry)
                            -> krb5_error_code>,
        pub remove: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: krb5_keytab,
                                                _: *mut krb5_keytab_entry)
                               -> krb5_error_code>,
    }
    #[inline]
    #[c2rust::src_loc = "2237:1"]
    pub unsafe extern "C" fn data_eq(mut d1: krb5_data, mut d2: krb5_data)
     -> libc::c_int {
        return (d1.length == d2.length &&
                    (d1.length == 0 as libc::c_int as libc::c_uint ||
                         memcmp(d1.data as *const libc::c_void,
                                d2.data as *const libc::c_void,
                                d1.length as libc::c_ulong) == 0)) as
                   libc::c_int;
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
    /* Return true if a comes after b. */
    #[inline]
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor, krb5_data,
                        krb5_timestamp};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::string_h::{memcmp, strlen};
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
        #[c2rust::src_loc = "2096:1"]
        pub fn krb5int_c_deprecated_enctype(_: krb5_enctype) -> krb5_boolean;
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
    #[c2rust::src_loc = "27:1"]
    pub type et_old_error_hook_func
        =
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: errcode_t,
                                    _: *const libc::c_char,
                                    _: *mut __va_list_tag) -> ()>;
    use super::internal::__va_list_tag;
    extern "C" {
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
        /*@modifies internalState@*/
        /*
 * The display routine should be application specific.  A global hook,
 * may cause inappropriate display procedures to be called between
 * applications under non-Unix environments.
 */
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn set_com_err_hook(_: et_old_error_hook_func)
         -> et_old_error_hook_func;
    }
    /* ! defined(__COM_ERR_H) */
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
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "341:12"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: ::std::ffi::VaList) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "521:1"]
        pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "528:1"]
        pub fn putchar(__c: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "626:1"]
        pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
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
#[c2rust::header_src = "/usr/include/time.h:27"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:27"]
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
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "252:14"]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/socket-utils.h:27"]
pub mod socket_utils_h {
    #[inline]
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn ss2sa(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr {
        return ss as *mut sockaddr;
    }
    #[inline]
    #[c2rust::src_loc = "83:1"]
    pub unsafe extern "C" fn ss2sin(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr_in {
        return ss as *mut sockaddr_in;
    }
    #[inline]
    #[c2rust::src_loc = "87:1"]
    pub unsafe extern "C" fn ss2sin6(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr_in6 {
        return ss as *mut sockaddr_in6;
    }
    /* Return the length of an IPv4 or IPv6 socket structure; abort if it is
 * neither. */
    #[inline]
    #[c2rust::src_loc = "135:1"]
    pub unsafe extern "C" fn sa_socklen(mut sa: *mut sockaddr) -> socklen_t {
        if (*sa).sa_family as libc::c_int == 10 as libc::c_int {
            return ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as
                       socklen_t
        } else if (*sa).sa_family as libc::c_int == 2 as libc::c_int {
            return ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                       socklen_t
        } else { abort(); };
    }
    use super::socket_h::{sockaddr_storage, sockaddr};
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::unistd_h::socklen_t;
    use super::sockaddr_h::sa_family_t;
    use super::stdlib_h::abort;
    /* SOCKET_UTILS_H */
}
#[c2rust::header_src = "/usr/include/locale.h:30"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/fake-addrinfo.h:810"]
pub mod fake_addrinfo_h {
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn krb5int_gai_strerror(err: libc::c_int) -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "219:1"]
        pub fn krb5int_getnameinfo(sa: *const sockaddr, salen: socklen_t,
                                   hbuf: *mut libc::c_char, hbuflen: size_t,
                                   sbuf: *mut libc::c_char, sbuflen: size_t,
                                   flags: libc::c_int) -> libc::c_int;
    }
    /* FAI_DEFINED */
}
pub use self::internal::__va_list_tag;
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t, __off_t,
                        __off64_t, __time_t, __socklen_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::unistd_h::socklen_t;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_authdatatype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_enc_data, krb5_enc_data, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_transited, krb5_transited, _krb5_enc_tkt_part,
                       krb5_enc_tkt_part, _krb5_ticket, krb5_ticket,
                       _krb5_creds, krb5_creds, krb5_cc_cursor, krb5_ccache,
                       krb5_cccol_cursor, krb5_kt_cursor,
                       krb5_keytab_entry_st, krb5_keytab_entry, krb5_keytab,
                       _profile_t, _krb5_ccache, _krb5_cccol_cursor,
                       krb5_cc_get_name, krb5_cc_get_full_name, krb5_cc_close,
                       krb5_cc_get_principal, krb5_cc_start_seq_get,
                       krb5_cc_next_cred, krb5_cc_end_seq_get,
                       krb5_cc_get_type, krb5_cccol_cursor_new,
                       krb5_cccol_cursor_next, krb5_cccol_cursor_free,
                       krb5_kt_get_name, krb5_kt_start_seq_get,
                       krb5_kt_next_entry, krb5_kt_end_seq_get,
                       krb5_init_context, krb5_unparse_name,
                       krb5_principal_compare, krb5_kt_resolve,
                       krb5_kt_default, krb5_kt_client_default,
                       krb5_free_keytab_entry_contents,
                       krb5_cc_set_default_name, krb5_cc_default,
                       krb5_is_config_principal, krb5_free_principal,
                       krb5_free_ticket, krb5_free_cred_contents,
                       krb5_free_unparsed_name, krb5_free_string,
                       krb5_enctype_to_name, krb5_timestamp_to_sfstring,
                       krb5_decode_ticket, krb5_get_error_message,
                       krb5_free_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops, data_eq,
                         data_eq_string, ts_after, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5int_c_deprecated_enctype};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, et_old_error_hook_func, com_err,
                          set_com_err_hook};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t};
use self::stdlib_h::{abort, exit};
use self::stdio_h::{stdout, fprintf, printf, vfprintf, snprintf, fputc,
                    putchar, fputs, stderr};
use self::getopt_core_h::{optind, getopt};
use self::time_h::time;
use self::k5_platform_h::krb5int_strlcpy;
use self::libintl_h::dgettext;
use self::string_h::{strlen, strrchr, strcmp, memcmp, memset, memcpy};
pub use self::socket_utils_h::{ss2sa, ss2sin, ss2sin6, sa_socklen};
use self::locale_h::setlocale;
use self::fake_addrinfo_h::{krb5int_gai_strerror, krb5int_getnameinfo};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* clients/klist/klist.c - List contents of credential cache or keytab */
/*
 * Copyright 1990 by the Massachusetts Institute of Technology.
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
/* Need definition of INET6 before network headers, for IRIX.  */
#[no_mangle]
#[c2rust::src_loc = "52:5"]
pub static mut show_flags: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "52:21"]
pub static mut show_time: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "52:36"]
pub static mut status_only: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "52:53"]
pub static mut show_keys: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "53:5"]
pub static mut show_etype: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "53:21"]
pub static mut show_addresses: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "53:41"]
pub static mut no_resolve: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "53:57"]
pub static mut print_version: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "54:5"]
pub static mut show_adtype: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "54:22"]
pub static mut show_all: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "54:36"]
pub static mut list_all: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "54:50"]
pub static mut use_client_keytab: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "55:5"]
pub static mut show_config: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "56:7"]
pub static mut defname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "57:7"]
pub static mut progname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "58:16"]
pub static mut now: krb5_timestamp = 0;
#[no_mangle]
#[c2rust::src_loc = "59:14"]
pub static mut timestamp_width: libc::c_uint = 0;
#[no_mangle]
#[c2rust::src_loc = "61:14"]
pub static mut context: krb5_context =
    0 as *const _krb5_context as *mut _krb5_context;
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"Usage: %s [-e] [-V] [[-c] [-l] [-A] [-d] [-f] [-s] [-a [-n]]] [-k [-t] [-K]] [name]\n\x00"
                         as *const u8 as *const libc::c_char), progname);
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-c specifies credentials cache\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-k specifies keytab\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t   (Default is credentials cache)\n\x00" as *const u8
                         as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-i uses default client keytab if no name given\n\x00"
                         as *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-l lists credential caches in collection\n\x00" as
                         *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-A shows content of all credential caches\n\x00" as
                         *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-e shows the encryption type\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-V shows the Kerberos version and exits\n\x00" as
                         *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\toptions for credential caches:\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t\t-d shows the submitted authorization data types\n\x00"
                         as *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t\t-f shows credentials flags\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t\t-s sets exit status based on valid tgt existence\n\x00"
                         as *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t\t-a displays the address list\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t\t\t-n do not reverse-resolve\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\toptions for keytabs:\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t\t-t shows keytab entry timestamps\n\x00" as
                         *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t\t-K shows keytab entry keys\n\x00" as *const u8 as
                         *const libc::c_char));
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "109:1"]
unsafe extern "C" fn extended_com_err_fn(mut prog: *const libc::c_char,
                                         mut code: errcode_t,
                                         mut fmt: *const libc::c_char,
                                         mut args: ::std::ffi::VaList) {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    msg = krb5_get_error_message(context, code as krb5_error_code);
    fprintf(stderr, b"%s: %s%s\x00" as *const u8 as *const libc::c_char, prog,
            msg,
            if *fmt as libc::c_int == '\u{0}' as i32 {
                b"\x00" as *const u8 as *const libc::c_char
            } else { b" \x00" as *const u8 as *const libc::c_char });
    krb5_free_error_message(context, msg);
    vfprintf(stderr, fmt, args.as_va_list());
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
}
#[c2rust::src_loc = "122:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut ret: krb5_error_code = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 8192] = [0; 8192];
    let mut c: libc::c_int = 0;
    let mut mode: libc::c_int = 0;
    setlocale(6 as libc::c_int, b"\x00" as *const u8 as *const libc::c_char);
    progname =
        if !strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).is_null() {
            strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).offset(1 as libc::c_int as isize)
        } else { *argv.offset(0 as libc::c_int as isize) };
    set_com_err_hook(Some(extended_com_err_fn as
                              unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: errcode_t,
                                                   _: *const libc::c_char,
                                                   _: ::std::ffi::VaList)
                                  -> ()));
    name = 0 as *mut libc::c_char;
    mode = 0 as libc::c_int;
    loop 
         /* V = version so v can be used for verbose later if desired. */
         {
        c =
            getopt(argc, argv as *const *mut libc::c_char,
                   b"dfetKsnacki45lAVC\x00" as *const u8 as
                       *const libc::c_char);
        if !(c != -(1 as libc::c_int)) { break ; }
        match c {
            100 => { show_adtype = 1 as libc::c_int }
            102 => { show_flags = 1 as libc::c_int }
            101 => { show_etype = 1 as libc::c_int }
            116 => { show_time = 1 as libc::c_int }
            75 => { show_keys = 1 as libc::c_int }
            115 => { status_only = 1 as libc::c_int }
            110 => { no_resolve = 1 as libc::c_int }
            97 => { show_addresses = 1 as libc::c_int }
            99 => {
                if mode != 0 as libc::c_int { usage(); }
                mode = 1 as libc::c_int
            }
            107 => {
                if mode != 0 as libc::c_int { usage(); }
                mode = 2 as libc::c_int
            }
            105 => { use_client_keytab = 1 as libc::c_int }
            52 => {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Kerberos 4 is no longer supported\n\x00" as
                                     *const u8 as *const libc::c_char));
                exit(3 as libc::c_int);
            }
            53 => { }
            108 => { list_all = 1 as libc::c_int }
            65 => { show_all = 1 as libc::c_int }
            67 => { show_config = 1 as libc::c_int }
            86 => { print_version = 1 as libc::c_int }
            _ => { usage(); }
        }
    }
    if no_resolve != 0 && show_addresses == 0 { usage(); }
    if mode == 0 as libc::c_int || mode == 1 as libc::c_int {
        if show_time != 0 || show_keys != 0 { usage(); }
        if show_all != 0 && list_all != 0 || status_only != 0 && list_all != 0
           {
            usage();
        }
    } else if show_flags != 0 || status_only != 0 || show_addresses != 0 ||
                  show_all != 0 || list_all != 0 {
        usage();
    }
    if argc - optind > 1 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Extra arguments (starting with \"%s\").\n\x00" as
                             *const u8 as *const libc::c_char),
                *argv.offset((optind + 1 as libc::c_int) as isize));
        usage();
    }
    if print_version != 0 {
        /* No access to autoconf vars; fix somehow. */
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"%s version %s\n\x00" as *const u8 as
                            *const libc::c_char),
               b"Kerberos 5\x00" as *const u8 as *const libc::c_char,
               b"1.18-prerelease\x00" as *const u8 as
                   *const libc::c_char); /* Hopefully large enough for any type */
        exit(0 as libc::c_int);
    }
    name =
        if optind == argc - 1 as libc::c_int {
            *argv.offset(optind as isize)
        } else { 0 as *mut libc::c_char };
    now = time(0 as *mut time_t) as krb5_timestamp;
    if krb5_timestamp_to_sfstring(now, tmp.as_mut_ptr(),
                                  20 as libc::c_int as size_t,
                                  0 as *mut libc::c_char) == 0 ||
           krb5_timestamp_to_sfstring(now, tmp.as_mut_ptr(),
                                      ::std::mem::size_of::<[libc::c_char; 8192]>()
                                          as libc::c_ulong,
                                      0 as *mut libc::c_char) == 0 {
        timestamp_width =
            strlen(tmp.as_mut_ptr()) as libc::c_int as libc::c_uint
    } else { timestamp_width = 15 as libc::c_int as libc::c_uint }
    ret = krb5_init_context(&mut context);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing krb5\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if !name.is_null() && mode != 2 as libc::c_int {
        ret = krb5_cc_set_default_name(context, name);
        if ret != 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while setting default cache name\x00" as
                                 *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
    }
    if list_all != 0 {
        list_all_ccaches();
    } else if show_all != 0 {
        show_all_ccaches();
    } else if mode == 0 as libc::c_int || mode == 1 as libc::c_int {
        do_ccache();
    } else { do_keytab(name); }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "262:1"]
unsafe extern "C" fn do_keytab(mut name: *const libc::c_char) {
    let mut ret: krb5_error_code = 0;
    let mut kt: krb5_keytab = 0 as *mut _krb5_kt;
    let mut entry: krb5_keytab_entry =
        krb5_keytab_entry{magic: 0,
                          principal: 0 as *mut krb5_principal_data,
                          timestamp: 0,
                          vno: 0,
                          key:
                              krb5_keyblock{magic: 0,
                                            enctype: 0,
                                            length: 0,
                                            contents:
                                                0 as *mut krb5_octet,},};
    let mut cursor: krb5_kt_cursor = 0 as *mut libc::c_void;
    let mut i: libc::c_uint = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    if name.is_null() && use_client_keytab != 0 {
        ret = krb5_kt_client_default(context, &mut kt);
        if ret != 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while getting default client keytab\x00" as
                                 *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
    } else if name.is_null() {
        ret = krb5_kt_default(context, &mut kt);
        if ret != 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while getting default keytab\x00" as *const u8
                                 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
    } else {
        ret = krb5_kt_resolve(context, name, &mut kt);
        if ret != 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while resolving keytab %s\x00" as *const u8 as
                                 *const libc::c_char), name);
            exit(1 as libc::c_int);
        }
    }
    ret =
        krb5_kt_get_name(context, kt, buf.as_mut_ptr(),
                         8192 as libc::c_int as libc::c_uint);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting keytab name\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    printf(b"Keytab name: %s\n\x00" as *const u8 as *const libc::c_char,
           buf.as_mut_ptr());
    ret = krb5_kt_start_seq_get(context, kt, &mut cursor);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while starting keytab scan\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /* XXX Translating would disturb table alignment; skip for now. */
    if show_time != 0 {
        printf(b"KVNO Timestamp\x00" as *const u8 as *const libc::c_char);
        fillit(stdout,
               (timestamp_width as
                    libc::c_ulong).wrapping_sub(::std::mem::size_of::<[libc::c_char; 10]>()
                                                    as
                                                    libc::c_ulong).wrapping_add(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_ulong)
                   as libc::c_uint, ' ' as i32);
        printf(b"Principal\n\x00" as *const u8 as *const libc::c_char);
        printf(b"---- \x00" as *const u8 as *const libc::c_char);
        fillit(stdout, timestamp_width, '-' as i32);
        printf(b" \x00" as *const u8 as *const libc::c_char);
        fillit(stdout,
               ((78 as libc::c_int as
                     libc::c_uint).wrapping_sub(timestamp_width) as
                    libc::c_ulong).wrapping_sub(::std::mem::size_of::<[libc::c_char; 5]>()
                                                    as libc::c_ulong) as
                   libc::c_uint, '-' as i32);
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"KVNO Principal\n\x00" as *const u8 as *const libc::c_char);
        printf(b"---- --------------------------------------------------------------------------\n\x00"
                   as *const u8 as *const libc::c_char);
    }
    loop  {
        ret = krb5_kt_next_entry(context, kt, &mut entry, &mut cursor);
        if !(ret == 0 as libc::c_int) { break ; }
        ret =
            krb5_unparse_name(context,
                              entry.principal as krb5_const_principal,
                              &mut pname);
        if ret != 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while unparsing principal name\x00" as
                                 *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
        printf(b"%4d \x00" as *const u8 as *const libc::c_char, entry.vno);
        if show_time != 0 {
            printtime(entry.timestamp);
            printf(b" \x00" as *const u8 as *const libc::c_char);
        }
        printf(b"%s\x00" as *const u8 as *const libc::c_char, pname);
        if show_etype != 0 {
            printf(b" (%s) \x00" as *const u8 as *const libc::c_char,
                   etype_string(entry.key.enctype));
        }
        if show_keys != 0 {
            printf(b" (0x\x00" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int as libc::c_uint;
            while i < entry.key.length {
                printf(b"%02x\x00" as *const u8 as *const libc::c_char,
                       *entry.key.contents.offset(i as isize) as libc::c_int);
                i = i.wrapping_add(1)
            }
            printf(b")\x00" as *const u8 as *const libc::c_char);
        }
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        krb5_free_unparsed_name(context, pname);
        krb5_free_keytab_entry_contents(context, &mut entry);
    }
    if ret != 0 && ret as libc::c_long != -(1765328202 as libc::c_long) {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while scanning keytab\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    ret = krb5_kt_end_seq_get(context, kt, &mut cursor);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while ending keytab scan\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    exit(0 as libc::c_int);
}
#[c2rust::src_loc = "359:1"]
unsafe extern "C" fn list_all_ccaches() {
    let mut ret: krb5_error_code = 0;
    let mut cache: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut cursor: krb5_cccol_cursor = 0 as *mut _krb5_cccol_cursor;
    let mut exit_status: libc::c_int = 0;
    ret = krb5_cccol_cursor_new(context, &mut cursor);
    if ret != 0 {
        if status_only == 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while listing ccache collection\x00" as
                                 *const u8 as *const libc::c_char));
        }
        exit(1 as libc::c_int);
    }
    /* XXX Translating would disturb table alignment; skip for now. */
    printf(b"%-30s %s\n\x00" as *const u8 as *const libc::c_char,
           b"Principal name\x00" as *const u8 as *const libc::c_char,
           b"Cache name\x00" as *const u8 as *const libc::c_char);
    printf(b"%-30s %s\n\x00" as *const u8 as *const libc::c_char,
           b"--------------\x00" as *const u8 as *const libc::c_char,
           b"----------\x00" as *const u8 as *const libc::c_char);
    exit_status = 1 as libc::c_int;
    loop  {
        ret = krb5_cccol_cursor_next(context, cursor, &mut cache);
        if !(ret == 0 as libc::c_int && !cache.is_null()) { break ; }
        exit_status =
            (list_ccache(cache) != 0 && exit_status != 0) as libc::c_int;
        krb5_cc_close(context, cache);
    }
    krb5_cccol_cursor_free(context, &mut cursor);
    exit(exit_status);
}
#[c2rust::src_loc = "387:1"]
unsafe extern "C" fn list_ccache(mut cache: krb5_ccache) -> libc::c_int {
    let mut ret: krb5_error_code = 0;
    let mut princ: krb5_principal = 0 as krb5_principal;
    let mut princname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ccname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expired: libc::c_int = 0;
    let mut status: libc::c_int = 1 as libc::c_int;
    ret = krb5_cc_get_principal(context, cache, &mut princ);
    if !(ret != 0) {
        ret =
            krb5_unparse_name(context, princ as krb5_const_principal,
                              &mut princname);
        if !(ret != 0) {
            ret = krb5_cc_get_full_name(context, cache, &mut ccname);
            if !(ret != 0) {
                expired = check_ccache(cache);
                printf(b"%-30.30s %s\x00" as *const u8 as *const libc::c_char,
                       princname, ccname);
                if expired != 0 {
                    printf(b" %s\x00" as *const u8 as *const libc::c_char,
                           dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"(Expired)\x00" as *const u8 as
                                        *const libc::c_char));
                }
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
                status = 0 as libc::c_int
            }
        }
    }
    /* Uninitialized cache file, probably. */
    krb5_free_principal(context, princ);
    krb5_free_unparsed_name(context, princname);
    krb5_free_string(context, ccname);
    return status;
}
#[c2rust::src_loc = "421:1"]
unsafe extern "C" fn show_all_ccaches() {
    let mut ret: krb5_error_code = 0;
    let mut cache: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut cursor: krb5_cccol_cursor = 0 as *mut _krb5_cccol_cursor;
    let mut first: krb5_boolean = 0;
    let mut exit_status: libc::c_int = 0;
    let mut st: libc::c_int = 0;
    ret = krb5_cccol_cursor_new(context, &mut cursor);
    if ret != 0 {
        if status_only == 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while listing ccache collection\x00" as
                                 *const u8 as *const libc::c_char));
        }
        exit(1 as libc::c_int);
    }
    exit_status = 1 as libc::c_int;
    first = 1 as libc::c_int as krb5_boolean;
    loop  {
        ret = krb5_cccol_cursor_next(context, cursor, &mut cache);
        if !(ret == 0 as libc::c_int && !cache.is_null()) { break ; }
        if status_only == 0 && first == 0 {
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        first = 0 as libc::c_int as krb5_boolean;
        st =
            if status_only != 0 {
                check_ccache(cache)
            } else { show_ccache(cache) };
        exit_status = (st != 0 && exit_status != 0) as libc::c_int;
        krb5_cc_close(context, cache);
    }
    krb5_cccol_cursor_free(context, &mut cursor);
    exit(exit_status);
}
#[c2rust::src_loc = "451:1"]
unsafe extern "C" fn do_ccache() {
    let mut ret: krb5_error_code = 0;
    let mut cache: krb5_ccache = 0 as *mut _krb5_ccache;
    ret = krb5_cc_default(context, &mut cache);
    if ret != 0 {
        if status_only == 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while resolving ccache\x00" as *const u8 as
                                 *const libc::c_char));
        }
        exit(1 as libc::c_int);
    }
    exit(if status_only != 0 {
             check_ccache(cache)
         } else { show_ccache(cache) });
}
/* Display the contents of cache. */
#[c2rust::src_loc = "467:1"]
unsafe extern "C" fn show_ccache(mut cache: krb5_ccache) -> libc::c_int {
    let mut cur: krb5_cc_cursor = 0 as *mut libc::c_void;
    let mut creds: krb5_creds =
        krb5_creds{magic: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   keyblock:
                       krb5_keyblock{magic: 0,
                                     enctype: 0,
                                     length: 0,
                                     contents: 0 as *mut krb5_octet,},
                   times:
                       krb5_ticket_times{authtime: 0,
                                         starttime: 0,
                                         endtime: 0,
                                         renew_till: 0,},
                   is_skey: 0,
                   ticket_flags: 0,
                   addresses: 0 as *mut *mut krb5_address,
                   ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   second_ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   authdata: 0 as *mut *mut krb5_authdata,};
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut ret: krb5_error_code = 0;
    ret = krb5_cc_get_principal(context, cache, &mut princ);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                b"\x00" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int
    }
    ret =
        krb5_unparse_name(context, princ as krb5_const_principal,
                          &mut defname);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while unparsing principal name\x00" as *const u8 as
                             *const libc::c_char));
        return 1 as libc::c_int
    }
    printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                    b"Ticket cache: %s:%s\nDefault principal: %s\n\n\x00" as
                        *const u8 as *const libc::c_char),
           krb5_cc_get_type(context, cache), krb5_cc_get_name(context, cache),
           defname);
    /* XXX Translating would disturb table alignment; skip for now. */
    fputs(b"Valid starting\x00" as *const u8 as *const libc::c_char, stdout);
    fillit(stdout,
           (timestamp_width as
                libc::c_ulong).wrapping_sub(::std::mem::size_of::<[libc::c_char; 15]>()
                                                as
                                                libc::c_ulong).wrapping_add(3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
               as libc::c_uint, ' ' as i32);
    fputs(b"Expires\x00" as *const u8 as *const libc::c_char, stdout);
    fillit(stdout,
           (timestamp_width as
                libc::c_ulong).wrapping_sub(::std::mem::size_of::<[libc::c_char; 8]>()
                                                as
                                                libc::c_ulong).wrapping_add(3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
               as libc::c_uint, ' ' as i32);
    fputs(b"Service principal\n\x00" as *const u8 as *const libc::c_char,
          stdout);
    ret = krb5_cc_start_seq_get(context, cache, &mut cur);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while starting to retrieve tickets\x00" as
                             *const u8 as *const libc::c_char));
        return 1 as libc::c_int
    }
    loop  {
        ret = krb5_cc_next_cred(context, cache, &mut cur, &mut creds);
        if !(ret == 0 as libc::c_int) { break ; }
        if show_config != 0 ||
               krb5_is_config_principal(context,
                                        creds.server as krb5_const_principal)
                   == 0 {
            show_credential(&mut creds);
        }
        krb5_free_cred_contents(context, &mut creds);
    }
    krb5_free_principal(context, princ);
    krb5_free_unparsed_name(context, defname);
    defname = 0 as *mut libc::c_char;
    if ret as libc::c_long == -(1765328242 as libc::c_long) {
        ret = krb5_cc_end_seq_get(context, cache, &mut cur);
        if ret != 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while finishing ticket retrieval\x00" as
                                 *const u8 as *const libc::c_char));
            return 1 as libc::c_int
        }
        return 0 as libc::c_int
    } else {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while retrieving a ticket\x00" as *const u8 as
                             *const libc::c_char));
        return 1 as libc::c_int
    };
}
/* Return 0 if cache is accessible, present, and unexpired; return 1 if not. */
#[c2rust::src_loc = "523:1"]
unsafe extern "C" fn check_ccache(mut cache: krb5_ccache) -> libc::c_int {
    let mut ret: krb5_error_code = 0;
    let mut cur: krb5_cc_cursor = 0 as *mut libc::c_void;
    let mut creds: krb5_creds =
        krb5_creds{magic: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   keyblock:
                       krb5_keyblock{magic: 0,
                                     enctype: 0,
                                     length: 0,
                                     contents: 0 as *mut krb5_octet,},
                   times:
                       krb5_ticket_times{authtime: 0,
                                         starttime: 0,
                                         endtime: 0,
                                         renew_till: 0,},
                   is_skey: 0,
                   ticket_flags: 0,
                   addresses: 0 as *mut *mut krb5_address,
                   ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   second_ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   authdata: 0 as *mut *mut krb5_authdata,};
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut found_tgt: krb5_boolean = 0;
    let mut found_current_tgt: krb5_boolean = 0;
    let mut found_current_cred: krb5_boolean = 0;
    if krb5_cc_get_principal(context, cache, &mut princ) != 0 as libc::c_int {
        return 1 as libc::c_int
    }
    if krb5_cc_start_seq_get(context, cache, &mut cur) != 0 as libc::c_int {
        return 1 as libc::c_int
    }
    found_current_cred = 0 as libc::c_int as krb5_boolean;
    found_current_tgt = found_current_cred;
    found_tgt = found_current_tgt;
    loop  {
        ret = krb5_cc_next_cred(context, cache, &mut cur, &mut creds);
        if !(ret == 0 as libc::c_int) { break ; }
        if is_local_tgt(creds.server, &mut (*princ).realm) != 0 {
            found_tgt = 1 as libc::c_int as krb5_boolean;
            if ts_after(creds.times.endtime, now) != 0 {
                found_current_tgt = 1 as libc::c_int as krb5_boolean
            }
        } else if krb5_is_config_principal(context,
                                           creds.server as
                                               krb5_const_principal) == 0 &&
                      ts_after(creds.times.endtime, now) != 0 {
            found_current_cred = 1 as libc::c_int as krb5_boolean
        }
        krb5_free_cred_contents(context, &mut creds);
    }
    krb5_free_principal(context, princ);
    if ret as libc::c_long != -(1765328242 as libc::c_long) {
        return 1 as libc::c_int
    }
    if krb5_cc_end_seq_get(context, cache, &mut cur) != 0 as libc::c_int {
        return 1 as libc::c_int
    }
    /* If the cache contains at least one local TGT, require that it be
     * current.  Otherwise accept any current cred. */
    if found_tgt != 0 {
        return if found_current_tgt != 0 {
                   0 as libc::c_int
               } else { 1 as libc::c_int }
    }
    return if found_current_cred != 0 {
               0 as libc::c_int
           } else { 1 as libc::c_int };
}
/* Return true if princ is the local krbtgt principal for local_realm. */
#[c2rust::src_loc = "562:1"]
unsafe extern "C" fn is_local_tgt(mut princ: krb5_principal,
                                  mut realm: *mut krb5_data) -> krb5_boolean {
    return ((*princ).length == 2 as libc::c_int &&
                data_eq((*princ).realm, *realm) != 0 &&
                data_eq_string(*(*princ).data.offset(0 as libc::c_int as
                                                         isize),
                               b"krbtgt\x00" as *const u8 as
                                   *const libc::c_char) != 0 &&
                data_eq(*(*princ).data.offset(1 as libc::c_int as isize),
                        *realm) != 0) as libc::c_int as
               krb5_boolean; /* D/d are taken.  Use short strings? */
}
#[c2rust::src_loc = "570:1"]
unsafe extern "C" fn etype_string(mut enctype: krb5_enctype)
 -> *mut libc::c_char {
    static mut buf: [libc::c_char; 100] = [0; 100];
    let mut bp: *mut libc::c_char = buf.as_mut_ptr();
    let mut deplen: size_t = 0;
    let mut buflen: size_t =
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong;
    if krb5int_c_deprecated_enctype(enctype) != 0 {
        deplen =
            krb5int_strlcpy(bp,
                            b"DEPRECATED:\x00" as *const u8 as
                                *const libc::c_char, buflen);
        buflen =
            (buflen as libc::c_ulong).wrapping_sub(deplen) as size_t as
                size_t;
        bp = bp.offset(deplen as isize)
    }
    if krb5_enctype_to_name(enctype, 0 as libc::c_int as krb5_boolean, bp,
                            buflen) != 0 {
        snprintf(bp, buflen,
                 b"etype %d\x00" as *const u8 as *const libc::c_char,
                 enctype);
    }
    return buf.as_mut_ptr();
}
#[c2rust::src_loc = "588:1"]
unsafe extern "C" fn flags_string(mut cred: *mut krb5_creds)
 -> *mut libc::c_char {
    static mut buf: [libc::c_char; 32] = [0; 32];
    let mut i: libc::c_int = 0 as libc::c_int;
    if (*cred).ticket_flags & 0x40000000 as libc::c_int != 0 {
        let fresh0 = i;
        i = i + 1;
        buf[fresh0 as usize] = 'F' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x20000000 as libc::c_int != 0 {
        let fresh1 = i;
        i = i + 1;
        buf[fresh1 as usize] = 'f' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x10000000 as libc::c_int != 0 {
        let fresh2 = i;
        i = i + 1;
        buf[fresh2 as usize] = 'P' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x8000000 as libc::c_int != 0 {
        let fresh3 = i;
        i = i + 1;
        buf[fresh3 as usize] = 'p' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x4000000 as libc::c_int != 0 {
        let fresh4 = i;
        i = i + 1;
        buf[fresh4 as usize] = 'D' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x2000000 as libc::c_int != 0 {
        let fresh5 = i;
        i = i + 1;
        buf[fresh5 as usize] = 'd' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x1000000 as libc::c_int != 0 {
        let fresh6 = i;
        i = i + 1;
        buf[fresh6 as usize] = 'i' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x800000 as libc::c_int != 0 {
        let fresh7 = i;
        i = i + 1;
        buf[fresh7 as usize] = 'R' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x400000 as libc::c_int != 0 {
        let fresh8 = i;
        i = i + 1;
        buf[fresh8 as usize] = 'I' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x100000 as libc::c_int != 0 {
        let fresh9 = i;
        i = i + 1;
        buf[fresh9 as usize] = 'H' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x200000 as libc::c_int != 0 {
        let fresh10 = i;
        i = i + 1;
        buf[fresh10 as usize] = 'A' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x80000 as libc::c_int != 0 {
        let fresh11 = i;
        i = i + 1;
        buf[fresh11 as usize] = 'T' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x40000 as libc::c_int != 0 {
        let fresh12 = i;
        i = i + 1;
        buf[fresh12 as usize] = 'O' as i32 as libc::c_char
    }
    if (*cred).ticket_flags & 0x8000 as libc::c_int != 0 {
        let fresh13 = i;
        i = i + 1;
        buf[fresh13 as usize] = 'a' as i32 as libc::c_char
    }
    buf[i as usize] = '\u{0}' as i32 as libc::c_char;
    return buf.as_mut_ptr();
}
#[c2rust::src_loc = "626:1"]
unsafe extern "C" fn printtime(mut ts: krb5_timestamp) {
    let mut timestring: [libc::c_char; 8192] = [0; 8192];
    let mut fill: libc::c_char = ' ' as i32 as libc::c_char;
    if krb5_timestamp_to_sfstring(ts, timestring.as_mut_ptr(),
                                  timestamp_width.wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                                      as size_t, &mut fill) == 0 {
        printf(b"%s\x00" as *const u8 as *const libc::c_char,
               timestring.as_mut_ptr());
    };
}
#[c2rust::src_loc = "636:1"]
unsafe extern "C" fn print_config_data(mut col: libc::c_int,
                                       mut data: *mut krb5_data) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*data).length {
        while col < 8 as libc::c_int { putchar(' ' as i32); col += 1 }
        if *(*data).data.offset(i as isize) as libc::c_int >
               0x20 as libc::c_int &&
               (*(*data).data.offset(i as isize) as libc::c_int) <
                   0x7f as libc::c_int {
            putchar(*(*data).data.offset(i as isize) as libc::c_int);
            col += 1
        } else {
            col +=
                printf(b"\\%03o\x00" as *const u8 as *const libc::c_char,
                       *(*data).data.offset(i as isize) as libc::c_uchar as
                           libc::c_int)
        }
        if col > 72 as libc::c_int {
            putchar('\n' as i32);
            col = 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    if col > 0 as libc::c_int { putchar('\n' as i32); };
}
#[c2rust::src_loc = "661:1"]
unsafe extern "C" fn show_credential(mut cred: *mut krb5_creds) {
    let mut ret: krb5_error_code = 0;
    let mut tkt: *mut krb5_ticket = 0 as *mut krb5_ticket;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tktsname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flags: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut extra_field: libc::c_int = 0 as libc::c_int;
    let mut ccol: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut is_config: krb5_boolean =
        krb5_is_config_principal(context,
                                 (*cred).server as krb5_const_principal);
    ret =
        krb5_unparse_name(context, (*cred).client as krb5_const_principal,
                          &mut name);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while unparsing client name\x00" as *const u8 as
                             *const libc::c_char));
    } else {
        ret =
            krb5_unparse_name(context, (*cred).server as krb5_const_principal,
                              &mut sname);
        if ret != 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while unparsing server name\x00" as *const u8
                                 as *const libc::c_char));
        } else {
            if is_config == 0 {
                krb5_decode_ticket(&mut (*cred).ticket, &mut tkt);
            }
            if (*cred).times.starttime == 0 {
                (*cred).times.starttime = (*cred).times.authtime
            }
            if is_config == 0 {
                printtime((*cred).times.starttime);
                putchar(' ' as i32);
                putchar(' ' as i32);
                printtime((*cred).times.endtime);
                putchar(' ' as i32);
                putchar(' ' as i32);
                printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                       sname);
            } else {
                fputs(b"config: \x00" as *const u8 as *const libc::c_char,
                      stdout);
                ccol = 8 as libc::c_int;
                i = 1 as libc::c_int;
                while i < (*(*cred).server).length {
                    ccol +=
                        printf(b"%s%.*s%s\x00" as *const u8 as
                                   *const libc::c_char,
                               if i > 1 as libc::c_int {
                                   b"(\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"\x00" as *const u8 as *const libc::c_char
                               },
                               (*(*(*cred).server).data.offset(i as
                                                                   isize)).length
                                   as libc::c_int,
                               (*(*(*cred).server).data.offset(i as
                                                                   isize)).data,
                               if i > 1 as libc::c_int {
                                   b")\x00" as *const u8 as
                                       *const libc::c_char
                               } else {
                                   b"\x00" as *const u8 as *const libc::c_char
                               });
                    i += 1
                }
                fputs(b" = \x00" as *const u8 as *const libc::c_char, stdout);
                ccol += 3 as libc::c_int
            }
            if strcmp(name, defname) != 0 {
                printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                    *const libc::c_char,
                                b"\tfor client %s\x00" as *const u8 as
                                    *const libc::c_char), name);
                extra_field += 1
            }
            if is_config != 0 {
                print_config_data(ccol, &mut (*cred).ticket);
            }
            if (*cred).times.renew_till != 0 {
                if extra_field == 0 {
                    fputs(b"\t\x00" as *const u8 as *const libc::c_char,
                          stdout);
                } else {
                    fputs(b", \x00" as *const u8 as *const libc::c_char,
                          stdout);
                }
                fputs(dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"renew until \x00" as *const u8 as
                                   *const libc::c_char), stdout);
                printtime((*cred).times.renew_till);
                extra_field += 2 as libc::c_int
            }
            if show_flags != 0 {
                flags = flags_string(cred);
                if !flags.is_null() && *flags as libc::c_int != 0 {
                    if extra_field == 0 {
                        fputs(b"\t\x00" as *const u8 as *const libc::c_char,
                              stdout);
                    } else {
                        fputs(b", \x00" as *const u8 as *const libc::c_char,
                              stdout);
                    }
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Flags: %s\x00" as *const u8 as
                                        *const libc::c_char), flags);
                    extra_field += 1
                }
            }
            if extra_field > 2 as libc::c_int {
                fputs(b"\n\x00" as *const u8 as *const libc::c_char, stdout);
                extra_field = 0 as libc::c_int
            }
            if show_etype != 0 && !tkt.is_null() {
                if extra_field == 0 {
                    fputs(b"\t\x00" as *const u8 as *const libc::c_char,
                          stdout);
                } else {
                    fputs(b", \x00" as *const u8 as *const libc::c_char,
                          stdout);
                }
                printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                    *const libc::c_char,
                                b"Etype (skey, tkt): %s, \x00" as *const u8 as
                                    *const libc::c_char),
                       etype_string((*cred).keyblock.enctype));
                printf(b"%s \x00" as *const u8 as *const libc::c_char,
                       etype_string((*tkt).enc_part.enctype));
                extra_field += 1
            }
            if show_adtype != 0 {
                if !(*cred).authdata.is_null() {
                    if extra_field == 0 {
                        fputs(b"\t\x00" as *const u8 as *const libc::c_char,
                              stdout);
                    } else {
                        fputs(b", \x00" as *const u8 as *const libc::c_char,
                              stdout);
                    }
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"AD types: \x00" as *const u8 as
                                        *const libc::c_char));
                    i = 0 as libc::c_int;
                    while !(*(*cred).authdata.offset(i as isize)).is_null() {
                        if i != 0 {
                            printf(b", \x00" as *const u8 as
                                       *const libc::c_char);
                        }
                        printf(b"%d\x00" as *const u8 as *const libc::c_char,
                               (**(*cred).authdata.offset(i as
                                                              isize)).ad_type);
                        i += 1
                    }
                    extra_field += 1
                }
            }
            /* If any additional info was printed, extra_field is non-zero. */
            if extra_field != 0 { putchar('\n' as i32); }
            if show_addresses != 0 {
                if (*cred).addresses.is_null() ||
                       (*(*cred).addresses.offset(0 as libc::c_int as
                                                      isize)).is_null() {
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"\tAddresses: (none)\n\x00" as *const u8
                                        as *const libc::c_char));
                } else {
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"\tAddresses: \x00" as *const u8 as
                                        *const libc::c_char));
                    one_addr(*(*cred).addresses.offset(0 as libc::c_int as
                                                           isize));
                    i = 1 as libc::c_int;
                    while !(*(*cred).addresses.offset(i as isize)).is_null() {
                        printf(b", \x00" as *const u8 as *const libc::c_char);
                        one_addr(*(*cred).addresses.offset(i as isize));
                        i += 1
                    }
                    printf(b"\n\x00" as *const u8 as *const libc::c_char);
                }
            }
            /* Display the ticket server if it is different from the server name the
     * entry was cached under (most commonly for referrals). */
            if !tkt.is_null() &&
                   krb5_principal_compare(context,
                                          (*cred).server as
                                              krb5_const_principal,
                                          (*tkt).server as
                                              krb5_const_principal) == 0 {
                ret =
                    krb5_unparse_name(context,
                                      (*tkt).server as krb5_const_principal,
                                      &mut tktsname);
                if ret != 0 {
                    com_err(progname, ret as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while unparsing ticket server name\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                } else {
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"\tTicket server: %s\n\x00" as *const u8
                                        as *const libc::c_char), tktsname);
                    krb5_free_unparsed_name(context, tktsname);
                }
            }
        }
    }
    krb5_free_unparsed_name(context, name);
    krb5_free_unparsed_name(context, sname);
    krb5_free_ticket(context, tkt);
}
/* For ss2sin etc. */
#[c2rust::src_loc = "812:1"]
unsafe extern "C" fn one_addr(mut a: *mut krb5_address) {
    let mut ss: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut sinp: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut sin6p: *mut sockaddr_in6 = 0 as *mut sockaddr_in6;
    let mut err: libc::c_int = 0;
    let mut namebuf: [libc::c_char; 1025] = [0; 1025];
    memset(&mut ss as *mut sockaddr_storage as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong);
    match (*a).addrtype {
        2 => {
            if (*a).length != 4 as libc::c_int as libc::c_uint {
                printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                    *const libc::c_char,
                                b"broken address (type %d length %d)\x00" as
                                    *const u8 as *const libc::c_char),
                       (*a).addrtype, (*a).length);
                return
            }
            sinp = ss2sin(&mut ss);
            (*sinp).sin_family = 2 as libc::c_int as sa_family_t;
            memcpy(&mut (*sinp).sin_addr as *mut in_addr as *mut libc::c_void,
                   (*a).contents as *const libc::c_void,
                   4 as libc::c_int as libc::c_ulong);
        }
        24 => {
            if (*a).length != 16 as libc::c_int as libc::c_uint {
                printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                    *const libc::c_char,
                                b"broken address (type %d length %d)\x00" as
                                    *const u8 as *const libc::c_char),
                       (*a).addrtype, (*a).length);
                return
            }
            sin6p = ss2sin6(&mut ss);
            (*sin6p).sin6_family = 10 as libc::c_int as sa_family_t;
            memcpy(&mut (*sin6p).sin6_addr as *mut in6_addr as
                       *mut libc::c_void,
                   (*a).contents as *const libc::c_void,
                   16 as libc::c_int as libc::c_ulong);
        }
        _ => {
            printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                *const libc::c_char,
                            b"unknown addrtype %d\x00" as *const u8 as
                                *const libc::c_char), (*a).addrtype);
            return
        }
    }
    namebuf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    err =
        krb5int_getnameinfo(ss2sa(&mut ss), sa_socklen(ss2sa(&mut ss)),
                            namebuf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1025]>() as
                                libc::c_ulong, 0 as *mut libc::c_char,
                            0 as libc::c_int as size_t,
                            if no_resolve != 0 {
                                1 as libc::c_int as libc::c_uint
                            } else { 0 as libc::c_uint } as libc::c_int);
    if err != 0 {
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"unprintable address (type %d, error %d %s)\x00" as
                            *const u8 as *const libc::c_char), (*a).addrtype,
               err, krb5int_gai_strerror(err));
        return
    }
    printf(b"%s\x00" as *const u8 as *const libc::c_char,
           namebuf.as_mut_ptr());
}
#[c2rust::src_loc = "861:1"]
unsafe extern "C" fn fillit(mut f: *mut FILE, mut num: libc::c_uint,
                            mut c: libc::c_int) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num { fputc(c, f); i = i.wrapping_add(1) };
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
