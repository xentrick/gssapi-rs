use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:56"]
pub mod types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __u_short = libc::c_ushort;
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
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:56"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    use super::types_h::__u_short;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:56"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:56"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:56"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:56"]
pub mod pthreadtypes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    use super::thread_shared_types_h::__pthread_mutex_s;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:56"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:56"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:56"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:56"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[inline]
    #[c2rust::src_loc = "356:1"]
    pub unsafe extern "C" fn k5_mutex_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return k5_os_mutex_init(m);
    }
    #[inline]
    #[c2rust::src_loc = "367:1"]
    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_lock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_lock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          376 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"void k5_mutex_lock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_unlock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_unlock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          388 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void k5_mutex_unlock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    use super::pthreadtypes_h::pthread_mutex_t;
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::strerror;
    use super::assert_h::__assert_fail;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "290:1"]
        pub fn k5_os_mutex_init(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn k5_os_mutex_destroy(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "292:1"]
        pub fn k5_os_mutex_lock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn k5_os_mutex_unlock(m: *mut k5_os_mutex) -> libc::c_int;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:56"]
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
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::{_krb5_context, _krb5_kt};
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
        #[no_mangle]
        #[c2rust::src_loc = "4261:1"]
        pub fn krb5_kt_free_entry(context: krb5_context,
                                  entry: *mut krb5_keytab_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:56"]
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
    #[c2rust::src_loc = "2040:1"]
    pub type krb5_kt_ops = _krb5_kt_ops;
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
                        krb5_keytab_entry, krb5_kt_cursor, krb5_timestamp};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
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
        #[c2rust::src_loc = "614:1"]
        pub fn krb5_lock_file(_: krb5_context, _: libc::c_int, _: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "615:1"]
        pub fn krb5_unlock_file(_: krb5_context, _: libc::c_int)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:56"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:56"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:56"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdlib.h:56"]
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
#[c2rust::header_src = "/usr/include/stdio.h:56"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "304:1"]
        pub fn setbuf(__stream: *mut FILE, __buf: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "646:15"]
        pub fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
                     _: *mut FILE) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "652:15"]
        pub fn fwrite(_: *const libc::c_void, _: libc::c_ulong,
                      _: libc::c_ulong, _: *mut FILE) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "684:1"]
        pub fn fseek(__stream: *mut FILE, __off: libc::c_long,
                     __whence: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "689:1"]
        pub fn ftell(__stream: *mut FILE) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "759:1"]
        pub fn feof(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "786:1"]
        pub fn fileno(__stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:56"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:56"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:56"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:56"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:56"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:56"]
pub mod in_h {
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn ntohl(__netlong: uint32_t) -> uint32_t;
        #[no_mangle]
        #[c2rust::src_loc = "376:1"]
        pub fn ntohs(__netshort: uint16_t) -> uint16_t;
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:57"]
pub mod os_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "169:1"]
        pub fn k5_create_secure_file(_: krb5_context,
                                     pathname: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "170:1"]
        pub fn k5_sync_disk_file(_: krb5_context, fp: *mut FILE)
         -> krb5_error_code;
    }
    /* KRB5_LIBOS_INT_PROTO__ */
}
pub use self::types_h::{__u_short, __uint8_t, __int16_t, __uint16_t,
                        __int32_t, __uint32_t, __off_t, __off64_t};
pub use self::sys_types_h::u_short;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_mutex_init,
                            k5_mutex_lock, k5_mutex_unlock, k5_os_mutex_init,
                            k5_os_mutex_destroy, k5_os_mutex_lock,
                            k5_os_mutex_unlock};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_int32, krb5_boolean,
                       krb5_kvno, krb5_enctype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _profile_t,
                       krb5_unparse_name, krb5_principal_compare,
                       krb5_kt_free_entry, krb5_timeofday,
                       krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops,
                         krb5_kt_ops, ts_after, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_lock_file, krb5_unlock_file};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
use self::stdlib_h::{malloc, calloc, free};
use self::stdio_h::{stderr, fclose, fflush, fopen, setbuf, fprintf, snprintf,
                    fread, fwrite, fseek, ftell, feof, fileno};
use self::fcntl_h::fcntl;
use self::errno_h::__errno_location;
use self::libintl_h::dgettext;
use self::string_h::{explicit_bzero, strerror, strdup, memset};
use self::assert_h::__assert_fail;
use self::in_h::{ntohl, ntohs, htonl, htons};
use self::os_proto_h::{k5_create_secure_file, k5_sync_disk_file};
/*
 * Types
 */
#[c2rust::src_loc = "78:1"]
pub type krb5_ktfile_data = _krb5_ktfile_data;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "78:16"]
pub struct _krb5_ktfile_data {
    pub name: *mut libc::c_char,
    pub openf: *mut FILE,
    pub iobuf: [libc::c_char; 8192],
    pub version: libc::c_int,
    pub iter_count: libc::c_uint,
    pub start_offset: libc::c_long,
    pub lock: k5_mutex_t,
}
/* Name of the file */
/* open file, if any. */
/* so we can zap it later */
/* Version number of keytab */
/* Number of active iterators */
/* Starting offset after version */
/* Protect openf, version */
/* Formerly lib/krb5/keytab/file/ktf_util.c */
/*
 * This function contains utilities for the file based implementation of
 * the keytab.  There are no public functions in this file.
 *
 * This file is the only one that has knowledge of the format of a
 * keytab file.
 *
 * The format is as follows:
 *
 * <file format vno>
 * <record length>
 * principal timestamp vno key
 * <record length>
 * principal timestamp vno key
 * ....
 *
 * A length field (sizeof(krb5_int32)) exists between entries.  When this
 * length is positive it indicates an active entry, when negative a hole.
 * The length indicates the size of the block in the file (this may be
 * larger than the size of the next record, since we are using a first
 * fit algorithm for re-using holes and the first fit may be larger than
 * the entry we are writing).  Another (compatible) implementation could
 * break up holes when allocating them to smaller entries to minimize
 * wasted space.  (Such an implementation should also coalesce adjacent
 * holes to reduce fragmentation).  This implementation does neither.
 *
 * There are no separators between fields of an entry.
 * A principal is a length-encoded array of length-encoded strings.  The
 * length is a krb5_int16 in each case.  The specific format, then, is
 * multiple entries concatinated with no separators.  An entry has this
 * exact format:
 *
 * sizeof(krb5_int16) bytes for number of components in the principal;
 * then, each component listed in ordser.
 * For each component, sizeof(krb5_int16) bytes for the number of bytes
 * in the component, followed by the component.
 * sizeof(krb5_int32) for the principal type (for KEYTAB V2 and higher)
 * sizeof(krb5_int32) bytes for the timestamp
 * sizeof(krb5_octet) bytes for the key version number
 * sizeof(krb5_int16) bytes for the enctype
 * sizeof(krb5_int16) bytes for the key length, followed by the key
 */
#[c2rust::src_loc = "725:1"]
pub type krb5_kt_vno = krb5_int16;
/*
 * Some limitations:
 *
 * If the file OPENF is left open between calls, we have an iterator
 * active, and OPENF is opened in read-only mode.  So, no changes
 * can be made via that handle.
 *
 * An advisory file lock is used while the file is open.  Thus,
 * multiple handles on the same underlying file cannot be used without
 * disrupting the locking in effect.
 *
 * The start_offset field is only valid if the file is open.  It will
 * almost certainly always be the same constant.  It's used so that
 * if an iterator is active, and we start another one, we don't have
 * to seek back to the start and re-read the version number to set
 * the position for the iterator.
 */
/*
 * Macros
 */
/*
 * This is an implementation specific resolver.  It returns a keytab id
 * initialized with file keytab routines.
 */
#[c2rust::src_loc = "188:1"]
unsafe extern "C" fn krb5_ktfile_resolve(mut context: krb5_context,
                                         mut name: *const libc::c_char,
                                         mut id_out: *mut krb5_keytab)
 -> krb5_error_code {
    let mut data: *mut krb5_ktfile_data = 0 as *mut krb5_ktfile_data;
    let mut err: krb5_error_code = 12 as libc::c_int;
    let mut id: krb5_keytab = 0 as *mut _krb5_kt;
    *id_out = 0 as krb5_keytab;
    id =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<_krb5_kt>() as libc::c_ulong) as
            krb5_keytab;
    if id.is_null() { return 12 as libc::c_int }
    (*id).ops = &krb5_ktf_ops;
    data =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_ktfile_data>() as libc::c_ulong) as
            *mut krb5_ktfile_data;
    if !data.is_null() {
        (*data).name = strdup(name);
        if !(*data).name.is_null() {
            err = k5_mutex_init(&mut (*data).lock);
            if !(err != 0) {
                (*data).openf = 0 as *mut FILE;
                (*data).version = 0 as libc::c_int;
                (*data).iter_count = 0 as libc::c_int as libc::c_uint;
                (*id).data = data as krb5_pointer;
                (*id).magic = -(1760647382 as libc::c_long) as krb5_magic;
                *id_out = id;
                return 0 as libc::c_int
            }
        }
    }
    if !data.is_null() { free((*data).name as *mut libc::c_void); }
    free(data as *mut libc::c_void);
    free(id as *mut libc::c_void);
    return err;
}
/*
 * "Close" a file-based keytab and invalidate the id.  This means
 * free memory hidden in the structures.
 */
#[c2rust::src_loc = "237:1"]
unsafe extern "C" fn krb5_ktfile_close(mut context: krb5_context,
                                       mut id: krb5_keytab)
 -> krb5_error_code 
 /*
 * This routine is responsible for freeing all memory allocated
 * for this keytab.  There are no system resources that need
 * to be freed nor are there any open files.
 *
 * This routine should undo anything done by krb5_ktfile_resolve().
 */
 {
    free((*((*id).data as *mut krb5_ktfile_data)).name as *mut libc::c_void);
    explicit_bzero((*((*id).data as *mut krb5_ktfile_data)).iobuf.as_mut_ptr()
                       as *mut libc::c_void, 8192 as libc::c_int as size_t);
    k5_os_mutex_destroy(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    free((*id).data);
    (*id).ops = 0 as *const _krb5_kt_ops;
    free(id as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* Return true if k1 is more recent than k2, applying wraparound heuristics. */
#[c2rust::src_loc = "257:1"]
unsafe extern "C" fn more_recent(mut k1: *const krb5_keytab_entry,
                                 mut k2: *const krb5_keytab_entry)
 -> krb5_boolean {
    /*
     * If a small kvno was written at the same time or later than a large kvno,
     * the kvno probably wrapped at some boundary, so consider the small kvno
     * more recent.  Wraparound can happen due to pre-1.14 keytab file format
     * limitations (8-bit kvno storage), pre-1.14 kadmin protocol limitations
     * (8-bit kvno marshalling), or KDB limitations (16-bit kvno storage).
     */
    if ts_after((*k2).timestamp, (*k1).timestamp) == 0 &&
           (*k1).vno < 128 as libc::c_int as libc::c_uint &&
           (*k2).vno > 240 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as krb5_boolean
    }
    if ts_after((*k1).timestamp, (*k2).timestamp) == 0 &&
           (*k1).vno > 240 as libc::c_int as libc::c_uint &&
           (*k2).vno < 128 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as krb5_boolean
    }
    /* Otherwise do a simple version comparison. */
    return ((*k1).vno > (*k2).vno) as libc::c_int as krb5_boolean;
}
/*
 * This is the get_entry routine for the file based keytab implementation.
 * It opens the keytab file, and either retrieves the entry or returns
 * an error.
 */
#[c2rust::src_loc = "284:1"]
unsafe extern "C" fn krb5_ktfile_get_entry(mut context: krb5_context,
                                           mut id: krb5_keytab,
                                           mut principal:
                                               krb5_const_principal,
                                           mut kvno: krb5_kvno,
                                           mut enctype: krb5_enctype,
                                           mut entry: *mut krb5_keytab_entry)
 -> krb5_error_code {
    let mut cur_entry: krb5_keytab_entry =
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
    let mut new_entry: krb5_keytab_entry =
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
    let mut kerror: krb5_error_code = 0 as libc::c_int;
    let mut found_wrong_kvno: libc::c_int = 0 as libc::c_int;
    let mut was_open: libc::c_int = 0;
    let mut princname: *mut libc::c_char = 0 as *mut libc::c_char;
    k5_mutex_lock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    if !(*((*id).data as *mut krb5_ktfile_data)).openf.is_null() {
        was_open = 1 as libc::c_int;
        if fseek((*((*id).data as *mut krb5_ktfile_data)).openf,
                 (*((*id).data as *mut krb5_ktfile_data)).start_offset,
                 0 as libc::c_int) == -(1 as libc::c_int) {
            k5_mutex_unlock(&mut (*((*id).data as
                                        *mut krb5_ktfile_data)).lock);
            return *__errno_location()
        }
    } else {
        was_open = 0 as libc::c_int;
        /* Open the keyfile for reading */
        kerror = krb5_ktfileint_openr(context, id);
        if kerror != 0 {
            k5_mutex_unlock(&mut (*((*id).data as
                                        *mut krb5_ktfile_data)).lock);
            return kerror
        }
    }
    /*
     * For efficiency and simplicity, we'll use a while true that
     * is exited with a break statement.
     */
    cur_entry.principal = 0 as krb5_principal;
    cur_entry.vno = 0 as libc::c_int as krb5_kvno;
    cur_entry.key.contents = 0 as *mut krb5_octet;
    loop  {
        kerror = krb5_ktfileint_read_entry(context, id, &mut new_entry);
        if kerror != 0 { break ; }
        /* by the time this loop exits, it must either free cur_entry,
           and copy new_entry there, or free new_entry.  Otherwise, it
           leaks. */
        /* if the principal isn't the one requested, free new_entry
           and continue to the next. */
        if krb5_principal_compare(context, principal,
                                  new_entry.principal as krb5_const_principal)
               == 0 {
            krb5_kt_free_entry(context, &mut new_entry);
        } else if enctype != 0 as libc::c_int &&
                      enctype != new_entry.key.enctype {
            krb5_kt_free_entry(context, &mut new_entry);
        } else if kvno == 0 as libc::c_int as libc::c_uint ||
                      new_entry.vno == 0 as libc::c_int as libc::c_uint {
            /* If the enctype is not ignored and doesn't match, free new_entry and
           continue to the next. */
            /* If this entry is more recent (or the first match), free the
             * current and keep the new.  Otherwise, free the new. */
            if cur_entry.principal.is_null() ||
                   more_recent(&mut new_entry, &mut cur_entry) != 0 {
                krb5_kt_free_entry(context, &mut cur_entry);
                cur_entry = new_entry
            } else { krb5_kt_free_entry(context, &mut new_entry); }
        } else if new_entry.vno == kvno {
            krb5_kt_free_entry(context, &mut cur_entry);
            cur_entry = new_entry;
            if new_entry.vno == kvno { break ; }
        } else if new_entry.vno == kvno & 0xff as libc::c_int as libc::c_uint
                      && cur_entry.principal.is_null() {
            cur_entry = new_entry
        } else {
            found_wrong_kvno += 1;
            krb5_kt_free_entry(context, &mut new_entry);
        }
    }
    if kerror as libc::c_long == -(1765328202 as libc::c_long) {
        if !cur_entry.principal.is_null() {
            kerror = 0 as libc::c_int
        } else if found_wrong_kvno != 0 {
            kerror = -(1765328154 as libc::c_long) as krb5_error_code
        } else {
            kerror = -(1765328203 as libc::c_long) as krb5_error_code;
            if krb5_unparse_name(context, principal, &mut princname) ==
                   0 as libc::c_int {
                krb5_set_error_message(context, kerror,
                                       dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"No key table entry found for %s\x00"
                                                    as *const u8 as
                                                    *const libc::c_char),
                                       princname);
                free(princname as *mut libc::c_void);
            }
        }
    }
    if kerror != 0 {
        if was_open == 0 as libc::c_int { krb5_ktfileint_close(context, id); }
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        krb5_kt_free_entry(context, &mut cur_entry);
        return kerror
    }
    if was_open == 0 as libc::c_int &&
           {
               kerror = krb5_ktfileint_close(context, id);
               (kerror) != 0 as libc::c_int
           } {
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        krb5_kt_free_entry(context, &mut cur_entry);
        return kerror
    }
    k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    *entry = cur_entry;
    return 0 as libc::c_int;
}
/*
             * If this kvno matches exactly, free the current, keep the new,
             * and break out.  If it matches the low 8 bits of the desired
             * kvno, remember the first match (because the recorded kvno may
             * have been truncated due to pre-1.14 keytab format or kadmin
             * protocol limitations) but keep looking for an exact match.
             * Otherwise, remember that we were here so we can return the right
             * error, and free the new.
             */
/*
 * Get the name of the file containing a file-based keytab.
 */
#[c2rust::src_loc = "415:1"]
unsafe extern "C" fn krb5_ktfile_get_name(mut context: krb5_context,
                                          mut id: krb5_keytab,
                                          mut name: *mut libc::c_char,
                                          mut len: libc::c_uint)
 -> krb5_error_code 
 /*
 * This routine returns the name of the name of the file associated with
 * this file-based keytab.  name is zeroed and the filename is truncated
 * to fit in name if necessary.  The name is prefixed with PREFIX:, so that
 * trt will happen if the name is passed back to resolve.
 */
 {
    let mut result: libc::c_int = 0;
    memset(name as *mut libc::c_void, 0 as libc::c_int, len as libc::c_ulong);
    result =
        snprintf(name, len as libc::c_ulong,
                 b"%s:%s\x00" as *const u8 as *const libc::c_char,
                 (*(*id).ops).prefix,
                 (*((*id).data as *mut krb5_ktfile_data)).name);
    if result as libc::c_uint as libc::c_ulong >= len as size_t {
        return -(1765328155 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
/*
 * krb5_ktfile_start_seq_get()
 */
#[c2rust::src_loc = "437:1"]
unsafe extern "C" fn krb5_ktfile_start_seq_get(mut context: krb5_context,
                                               mut id: krb5_keytab,
                                               mut cursorp:
                                                   *mut krb5_kt_cursor)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut fileoff: *mut libc::c_long = 0 as *mut libc::c_long;
    k5_mutex_lock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    if (*((*id).data as *mut krb5_ktfile_data)).iter_count ==
           0 as libc::c_int as libc::c_uint {
        retval = krb5_ktfileint_openr(context, id);
        if retval != 0 {
            k5_mutex_unlock(&mut (*((*id).data as
                                        *mut krb5_ktfile_data)).lock);
            return retval
        }
    }
    fileoff =
        malloc(::std::mem::size_of::<libc::c_long>() as libc::c_ulong) as
            *mut libc::c_long;
    if fileoff.is_null() {
        if (*((*id).data as *mut krb5_ktfile_data)).iter_count ==
               0 as libc::c_int as libc::c_uint {
            krb5_ktfileint_close(context, id);
        }
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        return 12 as libc::c_int
    }
    *fileoff = (*((*id).data as *mut krb5_ktfile_data)).start_offset;
    *cursorp = fileoff as krb5_kt_cursor;
    let ref mut fresh0 = (*((*id).data as *mut krb5_ktfile_data)).iter_count;
    *fresh0 = (*fresh0).wrapping_add(1);
    if (*((*id).data as *mut krb5_ktfile_data)).iter_count ==
           0 as libc::c_int as libc::c_uint {
        /* Wrapped?!  */
        let ref mut fresh1 =
            (*((*id).data as *mut krb5_ktfile_data)).iter_count;
        *fresh1 = (*fresh1).wrapping_sub(1);
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        krb5_set_error_message(context,
                               -(1765328200 as libc::c_long) as
                                   krb5_error_code,
                               b"Too many keytab iterators active\x00" as
                                   *const u8 as *const libc::c_char);
        return -(1765328200 as libc::c_long) as krb5_error_code
        /* XXX */
    }
    k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    return 0 as libc::c_int;
}
/*
 * krb5_ktfile_get_next()
 */
#[c2rust::src_loc = "477:1"]
unsafe extern "C" fn krb5_ktfile_get_next(mut context: krb5_context,
                                          mut id: krb5_keytab,
                                          mut entry: *mut krb5_keytab_entry,
                                          mut cursor: *mut krb5_kt_cursor)
 -> krb5_error_code {
    let mut fileoff: *mut libc::c_long = *cursor as *mut libc::c_long;
    let mut cur_entry: krb5_keytab_entry =
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
    let mut kerror: krb5_error_code = 0;
    k5_mutex_lock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    if (*((*id).data as *mut krb5_ktfile_data)).openf.is_null() {
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        return -(1765328200 as libc::c_long) as krb5_error_code
    }
    if fseek((*((*id).data as *mut krb5_ktfile_data)).openf, *fileoff,
             0 as libc::c_int) == -(1 as libc::c_int) {
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        return -(1765328202 as libc::c_long) as krb5_error_code
    }
    kerror = krb5_ktfileint_read_entry(context, id, &mut cur_entry);
    if kerror != 0 {
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        return kerror
    }
    *fileoff = ftell((*((*id).data as *mut krb5_ktfile_data)).openf);
    *entry = cur_entry;
    k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    return 0 as libc::c_int;
}
/*
 * krb5_ktfile_end_get()
 */
#[c2rust::src_loc = "507:1"]
unsafe extern "C" fn krb5_ktfile_end_get(mut context: krb5_context,
                                         mut id: krb5_keytab,
                                         mut cursor: *mut krb5_kt_cursor)
 -> krb5_error_code {
    let mut kerror: krb5_error_code = 0;
    free(*cursor);
    k5_mutex_lock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    let ref mut fresh2 = (*((*id).data as *mut krb5_ktfile_data)).iter_count;
    *fresh2 = (*fresh2).wrapping_sub(1);
    if !(*((*id).data as *mut krb5_ktfile_data)).openf.is_null() &&
           (*((*id).data as *mut krb5_ktfile_data)).iter_count ==
               0 as libc::c_int as libc::c_uint {
        kerror = krb5_ktfileint_close(context, id)
    } else { kerror = 0 as libc::c_int }
    k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    return kerror;
}
/* routines to be included on extended version (write routines) */
/*
 * krb5_ktfile_add()
 */
#[c2rust::src_loc = "527:1"]
unsafe extern "C" fn krb5_ktfile_add(mut context: krb5_context,
                                     mut id: krb5_keytab,
                                     mut entry: *mut krb5_keytab_entry)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    k5_mutex_lock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    if !(*((*id).data as *mut krb5_ktfile_data)).openf.is_null() {
        /* Iterator(s) active -- no changes.  */
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        krb5_set_error_message(context,
                               -(1765328200 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Cannot change keytab with keytab iterators active\x00"
                                            as *const u8 as
                                            *const libc::c_char));
        return -(1765328200 as libc::c_long) as krb5_error_code
        /* XXX */
    }
    retval = krb5_ktfileint_openw(context, id);
    if retval != 0 {
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        return retval
    }
    if fseek((*((*id).data as *mut krb5_ktfile_data)).openf,
             0 as libc::c_int as libc::c_long, 2 as libc::c_int) ==
           -(1 as libc::c_int) {
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        return -(1765328202 as libc::c_long) as krb5_error_code
    }
    retval = krb5_ktfileint_write_entry(context, id, entry);
    krb5_ktfileint_close(context, id);
    k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    return retval;
}
/*
 * krb5_ktfile_remove()
 */
#[c2rust::src_loc = "558:1"]
unsafe extern "C" fn krb5_ktfile_remove(mut context: krb5_context,
                                        mut id: krb5_keytab,
                                        mut entry: *mut krb5_keytab_entry)
 -> krb5_error_code {
    let mut cur_entry: krb5_keytab_entry =
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
    let mut kerror: krb5_error_code = 0;
    let mut delete_point: krb5_int32 = 0;
    k5_mutex_lock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    if !(*((*id).data as *mut krb5_ktfile_data)).openf.is_null() {
        /* Iterator(s) active -- no changes.  */
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        krb5_set_error_message(context,
                               -(1765328200 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Cannot change keytab with keytab iterators active\x00"
                                            as *const u8 as
                                            *const libc::c_char));
        return -(1765328200 as libc::c_long) as krb5_error_code
        /* XXX */
    }
    kerror = krb5_ktfileint_openw(context, id);
    if kerror != 0 {
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        return kerror
    }
    loop 
         /*
     * For efficiency and simplicity, we'll use a while true that
     * is exited with a break statement.
     */
         {
        kerror =
            krb5_ktfileint_internal_read_entry(context, id, &mut cur_entry,
                                               &mut delete_point);
        if kerror != 0 { break ; }
        if (*entry).vno == cur_entry.vno &&
               (*entry).key.enctype == cur_entry.key.enctype &&
               krb5_principal_compare(context,
                                      (*entry).principal as
                                          krb5_const_principal,
                                      cur_entry.principal as
                                          krb5_const_principal) != 0 {
            /* found a match */
            krb5_kt_free_entry(context, &mut cur_entry);
            break ;
        } else { krb5_kt_free_entry(context, &mut cur_entry); }
    }
    if kerror as libc::c_long == -(1765328202 as libc::c_long) {
        kerror = -(1765328203 as libc::c_long) as krb5_error_code
    }
    if kerror != 0 {
        krb5_ktfileint_close(context, id);
        k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
        return kerror
    }
    kerror = krb5_ktfileint_delete_entry(context, id, delete_point);
    if kerror != 0 {
        krb5_ktfileint_close(context, id);
    } else { kerror = krb5_ktfileint_close(context, id) }
    k5_mutex_unlock(&mut (*((*id).data as *mut krb5_ktfile_data)).lock);
    return kerror;
}
/*
 * krb5_ktf_ops
 */
#[no_mangle]
#[c2rust::src_loc = "623:27"]
pub static mut krb5_ktf_ops: _krb5_kt_ops =
    unsafe {
        {
            let mut init =
                _krb5_kt_ops{magic: 0 as libc::c_int,
                             prefix:
                                 b"FILE\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             resolve:
                                 Some(krb5_ktfile_resolve as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *const libc::c_char,
                                                               _:
                                                                   *mut krb5_keytab)
                                              -> krb5_error_code),
                             get_name:
                                 Some(krb5_ktfile_get_name as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   libc::c_uint)
                                              -> krb5_error_code),
                             close:
                                 Some(krb5_ktfile_close as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab)
                                              -> krb5_error_code),
                             get:
                                 Some(krb5_ktfile_get_entry as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   krb5_const_principal,
                                                               _: krb5_kvno,
                                                               _:
                                                                   krb5_enctype,
                                                               _:
                                                                   *mut krb5_keytab_entry)
                                              -> krb5_error_code),
                             start_seq_get:
                                 Some(krb5_ktfile_start_seq_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             get_next:
                                 Some(krb5_ktfile_get_next as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_keytab_entry,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             end_get:
                                 Some(krb5_ktfile_end_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             add:
                                 Some(krb5_ktfile_add as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_keytab_entry)
                                              -> krb5_error_code),
                             remove:
                                 Some(krb5_ktfile_remove as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_keytab_entry)
                                              -> krb5_error_code),};
            init
        }
    };
/*
 * krb5_ktf_writable_ops -- this is the same as krb5_ktf_ops except for the
 * prefix.  WRFILE should no longer be needed, but is effectively aliased to
 * FILE for compatibility.
 */
#[no_mangle]
#[c2rust::src_loc = "643:27"]
pub static mut krb5_ktf_writable_ops: _krb5_kt_ops =
    unsafe {
        {
            let mut init =
                _krb5_kt_ops{magic: 0 as libc::c_int,
                             prefix:
                                 b"WRFILE\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             resolve:
                                 Some(krb5_ktfile_resolve as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *const libc::c_char,
                                                               _:
                                                                   *mut krb5_keytab)
                                              -> krb5_error_code),
                             get_name:
                                 Some(krb5_ktfile_get_name as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   libc::c_uint)
                                              -> krb5_error_code),
                             close:
                                 Some(krb5_ktfile_close as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab)
                                              -> krb5_error_code),
                             get:
                                 Some(krb5_ktfile_get_entry as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   krb5_const_principal,
                                                               _: krb5_kvno,
                                                               _:
                                                                   krb5_enctype,
                                                               _:
                                                                   *mut krb5_keytab_entry)
                                              -> krb5_error_code),
                             start_seq_get:
                                 Some(krb5_ktfile_start_seq_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             get_next:
                                 Some(krb5_ktfile_get_next as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_keytab_entry,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             end_get:
                                 Some(krb5_ktfile_end_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             add:
                                 Some(krb5_ktfile_add as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_keytab_entry)
                                              -> krb5_error_code),
                             remove:
                                 Some(krb5_ktfile_remove as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_keytab_entry)
                                              -> krb5_error_code),};
            init
        }
    };
/*
 * krb5_kt_dfl_ops
 */
#[no_mangle]
#[c2rust::src_loc = "661:19"]
pub static mut krb5_kt_dfl_ops: krb5_kt_ops =
    unsafe {
        {
            let mut init =
                _krb5_kt_ops{magic: 0 as libc::c_int,
                             prefix:
                                 b"FILE\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             resolve:
                                 Some(krb5_ktfile_resolve as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *const libc::c_char,
                                                               _:
                                                                   *mut krb5_keytab)
                                              -> krb5_error_code),
                             get_name:
                                 Some(krb5_ktfile_get_name as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   libc::c_uint)
                                              -> krb5_error_code),
                             close:
                                 Some(krb5_ktfile_close as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab)
                                              -> krb5_error_code),
                             get:
                                 Some(krb5_ktfile_get_entry as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   krb5_const_principal,
                                                               _: krb5_kvno,
                                                               _:
                                                                   krb5_enctype,
                                                               _:
                                                                   *mut krb5_keytab_entry)
                                              -> krb5_error_code),
                             start_seq_get:
                                 Some(krb5_ktfile_start_seq_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             get_next:
                                 Some(krb5_ktfile_get_next as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_keytab_entry,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             end_get:
                                 Some(krb5_ktfile_end_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_keytab,
                                                               _:
                                                                   *mut krb5_kt_cursor)
                                              -> krb5_error_code),
                             add: None,
                             remove: None,};
            init
        }
    };
#[c2rust::src_loc = "729:1"]
unsafe extern "C" fn krb5_ktfileint_open(mut context: krb5_context,
                                         mut id: krb5_keytab,
                                         mut mode: libc::c_int)
 -> krb5_error_code {
    let mut kerror: krb5_error_code = 0;
    let mut kt_vno: krb5_kt_vno = 0;
    let mut writevno: libc::c_int = 0 as libc::c_int;
    *__errno_location() = 0 as libc::c_int;
    let ref mut fresh3 = (*((*id).data as *mut krb5_ktfile_data)).openf;
    *fresh3 =
        fopen((*((*id).data as *mut krb5_ktfile_data)).name,
              if mode == 0x2 as libc::c_int {
                  b"rb+\x00" as *const u8 as *const libc::c_char
              } else { b"rb\x00" as *const u8 as *const libc::c_char });
    if (*((*id).data as *mut krb5_ktfile_data)).openf.is_null() {
        's_83:
            {
                if mode == 0x2 as libc::c_int &&
                       *__errno_location() == 2 as libc::c_int {
                    /* try making it first time around */
                    k5_create_secure_file(context,
                                          (*((*id).data as
                                                 *mut krb5_ktfile_data)).name);
                    *__errno_location() = 0 as libc::c_int;
                    let ref mut fresh4 =
                        (*((*id).data as *mut krb5_ktfile_data)).openf;
                    *fresh4 =
                        fopen((*((*id).data as *mut krb5_ktfile_data)).name,
                              b"rb+\x00" as *const u8 as *const libc::c_char);
                    if !(*((*id).data as
                               *mut krb5_ktfile_data)).openf.is_null() {
                        writevno = 1 as libc::c_int;
                        break 's_83 ;
                    }
                }
                match *__errno_location() {
                    0 => {
                        /* XXX */
                        return 24 as libc::c_int
                    }
                    2 => {
                        krb5_set_error_message(context, 2 as libc::c_int,
                                               dgettext(b"mit-krb5\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        b"Key table file \'%s\' not found\x00"
                                                            as *const u8 as
                                                            *const libc::c_char),
                                               (*((*id).data as
                                                      *mut krb5_ktfile_data)).name);
                        return 2 as libc::c_int
                    }
                    _ => { return *__errno_location() }
                }
            }
    }
    fcntl(fileno((*((*id).data as *mut krb5_ktfile_data)).openf),
          2 as libc::c_int, 1 as libc::c_int);
    kerror =
        krb5_lock_file(context,
                       fileno((*((*id).data as *mut krb5_ktfile_data)).openf),
                       mode);
    if kerror != 0 {
        fclose((*((*id).data as *mut krb5_ktfile_data)).openf);
        let ref mut fresh5 = (*((*id).data as *mut krb5_ktfile_data)).openf;
        *fresh5 = 0 as *mut FILE;
        return kerror
    }
    /* assume ANSI or BSD-style stdio */
    setbuf((*((*id).data as *mut krb5_ktfile_data)).openf,
           (*((*id).data as *mut krb5_ktfile_data)).iobuf.as_mut_ptr());
    /* get the vno and verify it */
    if writevno != 0 {
        kt_vno =
            htons(0x502 as libc::c_int as krb5_kt_vno as uint16_t) as
                krb5_kt_vno;
        (*((*id).data as *mut krb5_ktfile_data)).version =
            0x502 as libc::c_int as krb5_kt_vno as libc::c_int;
        if fwrite(&mut kt_vno as *mut krb5_kt_vno as *const libc::c_void,
                  ::std::mem::size_of::<krb5_kt_vno>() as libc::c_ulong,
                  1 as libc::c_int as libc::c_ulong,
                  (*((*id).data as *mut krb5_ktfile_data)).openf) == 0 {
            kerror = *__errno_location();
            krb5_unlock_file(context,
                             fileno((*((*id).data as
                                           *mut krb5_ktfile_data)).openf));
            fclose((*((*id).data as *mut krb5_ktfile_data)).openf);
            let ref mut fresh6 =
                (*((*id).data as *mut krb5_ktfile_data)).openf;
            *fresh6 = 0 as *mut FILE;
            return kerror
        }
    } else {
        /* gotta verify it instead... */
        if fread(&mut kt_vno as *mut krb5_kt_vno as *mut libc::c_void,
                 ::std::mem::size_of::<krb5_kt_vno>() as libc::c_ulong,
                 1 as libc::c_int as libc::c_ulong,
                 (*((*id).data as *mut krb5_ktfile_data)).openf) == 0 {
            if feof((*((*id).data as *mut krb5_ktfile_data)).openf) != 0 {
                kerror = -(1765328171 as libc::c_long) as krb5_error_code
            } else { kerror = *__errno_location() }
            krb5_unlock_file(context,
                             fileno((*((*id).data as
                                           *mut krb5_ktfile_data)).openf));
            fclose((*((*id).data as *mut krb5_ktfile_data)).openf);
            let ref mut fresh7 =
                (*((*id).data as *mut krb5_ktfile_data)).openf;
            *fresh7 = 0 as *mut FILE;
            return kerror
        }
        let ref mut fresh8 = (*((*id).data as *mut krb5_ktfile_data)).version;
        *fresh8 = ntohs(kt_vno as uint16_t) as libc::c_int;
        kt_vno = *fresh8 as krb5_kt_vno;
        if kt_vno as libc::c_int != 0x502 as libc::c_int &&
               kt_vno as libc::c_int != 0x501 as libc::c_int {
            krb5_unlock_file(context,
                             fileno((*((*id).data as
                                           *mut krb5_ktfile_data)).openf));
            fclose((*((*id).data as *mut krb5_ktfile_data)).openf);
            let ref mut fresh9 =
                (*((*id).data as *mut krb5_ktfile_data)).openf;
            *fresh9 = 0 as *mut FILE;
            return -(1765328171 as libc::c_long) as krb5_error_code
        }
    }
    (*((*id).data as *mut krb5_ktfile_data)).start_offset =
        ftell((*((*id).data as *mut krb5_ktfile_data)).openf);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "809:1"]
unsafe extern "C" fn krb5_ktfileint_openr(mut context: krb5_context,
                                          mut id: krb5_keytab)
 -> krb5_error_code {
    return krb5_ktfileint_open(context, id, 0x1 as libc::c_int);
}
#[c2rust::src_loc = "815:1"]
unsafe extern "C" fn krb5_ktfileint_openw(mut context: krb5_context,
                                          mut id: krb5_keytab)
 -> krb5_error_code {
    return krb5_ktfileint_open(context, id, 0x2 as libc::c_int);
}
#[c2rust::src_loc = "821:1"]
unsafe extern "C" fn krb5_ktfileint_close(mut context: krb5_context,
                                          mut id: krb5_keytab)
 -> krb5_error_code {
    let mut kerror: krb5_error_code = 0;
    if (*((*id).data as *mut krb5_ktfile_data)).openf.is_null() {
        return 0 as libc::c_int
    }
    kerror =
        krb5_unlock_file(context,
                         fileno((*((*id).data as
                                       *mut krb5_ktfile_data)).openf));
    fclose((*((*id).data as *mut krb5_ktfile_data)).openf);
    let ref mut fresh10 = (*((*id).data as *mut krb5_ktfile_data)).openf;
    *fresh10 = 0 as *mut FILE;
    return kerror;
}
#[c2rust::src_loc = "835:1"]
unsafe extern "C" fn krb5_ktfileint_delete_entry(mut context: krb5_context,
                                                 mut id: krb5_keytab,
                                                 mut delete_point: krb5_int32)
 -> krb5_error_code {
    let mut size: krb5_int32 = 0;
    let mut len: krb5_int32 = 0;
    let mut iobuf: [libc::c_char; 8192] = [0; 8192];
    if fseek((*((*id).data as *mut krb5_ktfile_data)).openf,
             delete_point as libc::c_long, 0 as libc::c_int) != 0 {
        return *__errno_location()
    }
    if fread(&mut size as *mut krb5_int32 as *mut libc::c_void,
             ::std::mem::size_of::<krb5_int32>() as libc::c_ulong,
             1 as libc::c_int as libc::c_ulong,
             (*((*id).data as *mut krb5_ktfile_data)).openf) == 0 {
        return -(1765328202 as libc::c_long) as krb5_error_code
    }
    if (*((*id).data as *mut krb5_ktfile_data)).version !=
           0x501 as libc::c_int {
        size = ntohl(size as uint32_t) as krb5_int32
    }
    if size > 0 as libc::c_int {
        let mut minus_size: krb5_int32 = -size;
        if (*((*id).data as *mut krb5_ktfile_data)).version !=
               0x501 as libc::c_int {
            minus_size = htonl(minus_size as uint32_t) as krb5_int32
        }
        if fseek((*((*id).data as *mut krb5_ktfile_data)).openf,
                 delete_point as libc::c_long, 0 as libc::c_int) != 0 {
            return *__errno_location()
        }
        if fwrite(&mut minus_size as *mut krb5_int32 as *const libc::c_void,
                  ::std::mem::size_of::<krb5_int32>() as libc::c_ulong,
                  1 as libc::c_int as libc::c_ulong,
                  (*((*id).data as *mut krb5_ktfile_data)).openf) == 0 {
            return -(1765328200 as libc::c_long) as krb5_error_code
        }
        if size < 8192 as libc::c_int {
            len = size
        } else { len = 8192 as libc::c_int }
        memset(iobuf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               len as size_t);
        while size > 0 as libc::c_int {
            if fwrite(iobuf.as_mut_ptr() as *const libc::c_void,
                      1 as libc::c_int as libc::c_ulong, len as size_t,
                      (*((*id).data as *mut krb5_ktfile_data)).openf) == 0 {
                return -(1765328200 as libc::c_long) as krb5_error_code
            }
            size -= len;
            if size < len { len = size }
        }
        return k5_sync_disk_file(context,
                                 (*((*id).data as
                                        *mut krb5_ktfile_data)).openf)
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "888:1"]
unsafe extern "C" fn krb5_ktfileint_internal_read_entry(mut context:
                                                            krb5_context,
                                                        mut id: krb5_keytab,
                                                        mut ret_entry:
                                                            *mut krb5_keytab_entry,
                                                        mut delete_point:
                                                            *mut krb5_int32)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut vno: krb5_octet = 0;
    let mut count: krb5_int16 = 0;
    let mut u_count: libc::c_uint = 0;
    let mut u_princ_size: libc::c_uint = 0;
    let mut enctype: krb5_int16 = 0;
    let mut princ_size: krb5_int16 = 0;
    let mut i: libc::c_int = 0;
    let mut size: krb5_int32 = 0;
    let mut start_pos: krb5_int32 = 0;
    let mut pos: krb5_int32 = 0;
    let mut error: krb5_error_code = 0;
    let mut tmpdata: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut princ: *mut krb5_data = 0 as *mut krb5_data;
    let mut vno32: uint32_t = 0;
    memset(ret_entry as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_keytab_entry>() as libc::c_ulong);
    (*ret_entry).magic = -(1760647389 as libc::c_long) as krb5_magic;
    /* fseek to synchronise buffered I/O on the key table. */
    if fseek((*((*id).data as *mut krb5_ktfile_data)).openf,
             0 as libc::c_long, 1 as libc::c_int) < 0 as libc::c_int {
        return *__errno_location()
    }
    loop  {
        *delete_point =
            ftell((*((*id).data as *mut krb5_ktfile_data)).openf) as
                krb5_int32;
        if fread(&mut size as *mut krb5_int32 as *mut libc::c_void,
                 ::std::mem::size_of::<krb5_int32>() as libc::c_ulong,
                 1 as libc::c_int as libc::c_ulong,
                 (*((*id).data as *mut krb5_ktfile_data)).openf) == 0 {
            return -(1765328202 as libc::c_long) as krb5_error_code
        }
        if (*((*id).data as *mut krb5_ktfile_data)).version !=
               0x501 as libc::c_int {
            size = ntohl(size as uint32_t) as krb5_int32
        }
        if size < 0 as libc::c_int {
            if fseek((*((*id).data as *mut krb5_ktfile_data)).openf,
                     -size as libc::c_long, 1 as libc::c_int) != 0 {
                return *__errno_location()
            }
        }
        if !(size < 0 as libc::c_int) { break ; }
    }
    if size == 0 as libc::c_int {
        return -(1765328202 as libc::c_long) as krb5_error_code
    }
    start_pos =
        ftell((*((*id).data as *mut krb5_ktfile_data)).openf) as krb5_int32;
    /* deal with guts of parsing... */
    /* first, int16 with #princ components */
    if fread(&mut count as *mut krb5_int16 as *mut libc::c_void,
             ::std::mem::size_of::<krb5_int16>() as libc::c_ulong,
             1 as libc::c_int as libc::c_ulong,
             (*((*id).data as *mut krb5_ktfile_data)).openf) == 0 {
        return -(1765328202 as libc::c_long) as krb5_error_code
    }
    if (*((*id).data as *mut krb5_ktfile_data)).version ==
           0x501 as libc::c_int {
        count = (count as libc::c_int - 1 as libc::c_int) as krb5_int16
        /* V1 includes the realm in the count */
    } else { count = ntohs(count as uint16_t) as krb5_int16 }
    if count == 0 || (count as libc::c_int) < 0 as libc::c_int {
        return -(1765328202 as libc::c_long) as krb5_error_code
    }
    (*ret_entry).principal =
        malloc(::std::mem::size_of::<krb5_principal_data>() as libc::c_ulong)
            as krb5_principal;
    if (*ret_entry).principal.is_null() { return 12 as libc::c_int }
    u_count = count as libc::c_uint;
    (*(*ret_entry).principal).magic =
        -(1760647423 as libc::c_long) as krb5_magic;
    (*(*ret_entry).principal).length = u_count as krb5_int32;
    (*(*ret_entry).principal).data =
        calloc(u_count as libc::c_ulong,
               ::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    if (*(*ret_entry).principal).data.is_null() {
        free((*ret_entry).principal as *mut libc::c_void);
        (*ret_entry).principal = 0 as krb5_principal;
        return 12 as libc::c_int
    }
    /* Now, get the realm data */
    if fread(&mut princ_size as *mut krb5_int16 as *mut libc::c_void,
             ::std::mem::size_of::<krb5_int16>() as libc::c_ulong,
             1 as libc::c_int as libc::c_ulong,
             (*((*id).data as *mut krb5_ktfile_data)).openf) == 0 {
        error = -(1765328202 as libc::c_long) as krb5_error_code
    } else {
        if (*((*id).data as *mut krb5_ktfile_data)).version !=
               0x501 as libc::c_int {
            princ_size = ntohs(princ_size as uint16_t) as krb5_int16
        } /* Some things might be expecting null */
                                /* termination...  ``Be conservative in */
                                /* what you send out'' */
        if princ_size == 0 || (princ_size as libc::c_int) < 0 as libc::c_int {
            error = -(1765328202 as libc::c_long) as krb5_error_code
        } else {
            u_princ_size = princ_size as libc::c_uint;
            (*(*ret_entry).principal).realm.length = u_princ_size;
            tmpdata =
                malloc(u_princ_size.wrapping_add(1 as libc::c_int as
                                                     libc::c_uint) as
                           libc::c_ulong) as *mut libc::c_char;
            if tmpdata.is_null() {
                error = 12 as libc::c_int
            } else if fread(tmpdata as *mut libc::c_void,
                            1 as libc::c_int as libc::c_ulong,
                            u_princ_size as libc::c_ulong,
                            (*((*id).data as *mut krb5_ktfile_data)).openf) !=
                          princ_size as size_t {
                free(tmpdata as *mut libc::c_void);
                error = -(1765328202 as libc::c_long) as krb5_error_code
            } else {
                *tmpdata.offset(princ_size as isize) =
                    0 as libc::c_int as libc::c_char;
                (*(*ret_entry).principal).realm.data = tmpdata;
                i = 0 as libc::c_int;
                loop  {
                    if !(i < count as libc::c_int) {
                        current_block = 16231175055492490595;
                        break ;
                    }
                    princ =
                        &mut *(*(*ret_entry).principal).data.offset(i as
                                                                        isize)
                            as *mut krb5_data;
                    if fread(&mut princ_size as *mut krb5_int16 as
                                 *mut libc::c_void,
                             ::std::mem::size_of::<krb5_int16>() as
                                 libc::c_ulong,
                             1 as libc::c_int as libc::c_ulong,
                             (*((*id).data as *mut krb5_ktfile_data)).openf)
                           == 0 {
                        error =
                            -(1765328202 as libc::c_long) as krb5_error_code;
                        current_block = 4639666093008142149;
                        break ;
                    } else {
                        if (*((*id).data as *mut krb5_ktfile_data)).version !=
                               0x501 as libc::c_int {
                            princ_size =
                                ntohs(princ_size as uint16_t) as krb5_int16
                        }
                        if princ_size == 0 ||
                               (princ_size as libc::c_int) < 0 as libc::c_int
                           {
                            error =
                                -(1765328202 as libc::c_long) as
                                    krb5_error_code;
                            current_block = 4639666093008142149;
                            break ;
                        } else {
                            u_princ_size = princ_size as libc::c_uint;
                            (*princ).length = u_princ_size;
                            (*princ).data =
                                malloc(u_princ_size.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                           as libc::c_ulong) as
                                    *mut libc::c_char;
                            if (*princ).data.is_null() {
                                error = 12 as libc::c_int;
                                current_block = 4639666093008142149;
                                break ;
                            } else if fread((*princ).data as
                                                *mut libc::c_void,
                                            ::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong,
                                            u_princ_size as libc::c_ulong,
                                            (*((*id).data as
                                                   *mut krb5_ktfile_data)).openf)
                                          == 0 {
                                error =
                                    -(1765328202 as libc::c_long) as
                                        krb5_error_code;
                                current_block = 4639666093008142149;
                                break ;
                            } else {
                                *(*princ).data.offset(princ_size as isize) =
                                    0 as libc::c_int as libc::c_char;
                                i += 1
                            }
                        }
                    }
                    /* Null terminate */
                }
                match current_block {
                    4639666093008142149 => { }
                    _ =>
                    /* read in the principal type, if we can get it */
                    {
                        if (*((*id).data as *mut krb5_ktfile_data)).version !=
                               0x501 as libc::c_int {
                            if fread(&mut (*(*ret_entry).principal).type_0 as
                                         *mut krb5_int32 as *mut libc::c_void,
                                     ::std::mem::size_of::<krb5_int32>() as
                                         libc::c_ulong,
                                     1 as libc::c_int as libc::c_ulong,
                                     (*((*id).data as
                                            *mut krb5_ktfile_data)).openf) ==
                                   0 {
                                error =
                                    -(1765328202 as libc::c_long) as
                                        krb5_error_code;
                                current_block = 4639666093008142149;
                            } else {
                                (*(*ret_entry).principal).type_0 =
                                    ntohl((*(*ret_entry).principal).type_0 as
                                              uint32_t) as krb5_int32;
                                current_block = 2310077433060450808;
                            }
                        } else { current_block = 2310077433060450808; }
                        match current_block {
                            4639666093008142149 => { }
                            _ =>
                            /* read in the timestamp */
                            {
                                if fread(&mut (*ret_entry).timestamp as
                                             *mut krb5_timestamp as
                                             *mut libc::c_void,
                                         ::std::mem::size_of::<krb5_timestamp>()
                                             as libc::c_ulong,
                                         1 as libc::c_int as libc::c_ulong,
                                         (*((*id).data as
                                                *mut krb5_ktfile_data)).openf)
                                       == 0 {
                                    error =
                                        -(1765328202 as libc::c_long) as
                                            krb5_error_code
                                } else {
                                    if (*((*id).data as
                                              *mut krb5_ktfile_data)).version
                                           != 0x501 as libc::c_int {
                                        (*ret_entry).timestamp =
                                            ntohl((*ret_entry).timestamp as
                                                      uint32_t) as
                                                krb5_timestamp
                                    }
                                    /* read in the version number */
                                    if fread(&mut vno as *mut krb5_octet as
                                                 *mut libc::c_void,
                                             ::std::mem::size_of::<krb5_octet>()
                                                 as libc::c_ulong,
                                             1 as libc::c_int as
                                                 libc::c_ulong,
                                             (*((*id).data as
                                                    *mut krb5_ktfile_data)).openf)
                                           == 0 {
                                        error =
                                            -(1765328202 as libc::c_long) as
                                                krb5_error_code
                                    } else {
                                        (*ret_entry).vno = vno as krb5_kvno;
                                        /* key type */
                                        if fread(&mut enctype as
                                                     *mut krb5_int16 as
                                                     *mut libc::c_void,
                                                 ::std::mem::size_of::<krb5_int16>()
                                                     as libc::c_ulong,
                                                 1 as libc::c_int as
                                                     libc::c_ulong,
                                                 (*((*id).data as
                                                        *mut krb5_ktfile_data)).openf)
                                               == 0 {
                                            error =
                                                -(1765328202 as libc::c_long)
                                                    as krb5_error_code
                                        } else {
                                            if (*((*id).data as
                                                      *mut krb5_ktfile_data)).version
                                                   != 0x501 as libc::c_int {
                                                enctype =
                                                    ntohs(enctype as uint16_t)
                                                        as krb5_int16
                                            }
                                            (*ret_entry).key.enctype =
                                                enctype as krb5_enctype;
                                            /* key contents */
                                            (*ret_entry).key.magic =
                                                -(1760647421 as libc::c_long)
                                                    as krb5_magic;
                                            if fread(&mut count as
                                                         *mut krb5_int16 as
                                                         *mut libc::c_void,
                                                     ::std::mem::size_of::<krb5_int16>()
                                                         as libc::c_ulong,
                                                     1 as libc::c_int as
                                                         libc::c_ulong,
                                                     (*((*id).data as
                                                            *mut krb5_ktfile_data)).openf)
                                                   == 0 {
                                                error =
                                                    -(1765328202 as
                                                          libc::c_long) as
                                                        krb5_error_code
                                            } else {
                                                if (*((*id).data as
                                                          *mut krb5_ktfile_data)).version
                                                       != 0x501 as libc::c_int
                                                   {
                                                    count =
                                                        ntohs(count as
                                                                  uint16_t) as
                                                            krb5_int16
                                                }
                                                if count == 0 ||
                                                       (count as libc::c_int)
                                                           < 0 as libc::c_int
                                                   {
                                                    error =
                                                        -(1765328202 as
                                                              libc::c_long) as
                                                            krb5_error_code
                                                } else {
                                                    u_count =
                                                        count as libc::c_uint;
                                                    (*ret_entry).key.length =
                                                        u_count;
                                                    (*ret_entry).key.contents
                                                        =
                                                        malloc(u_count as
                                                                   libc::c_ulong)
                                                            as
                                                            *mut krb5_octet;
                                                    if (*ret_entry).key.contents.is_null()
                                                       {
                                                        error =
                                                            12 as libc::c_int
                                                    } else if fread((*ret_entry).key.contents
                                                                        as
                                                                        *mut libc::c_void,
                                                                    ::std::mem::size_of::<krb5_octet>()
                                                                        as
                                                                        libc::c_ulong,
                                                                    count as
                                                                        libc::c_ulong,
                                                                    (*((*id).data
                                                                           as
                                                                           *mut krb5_ktfile_data)).openf)
                                                                  == 0 {
                                                        error =
                                                            -(1765328202 as
                                                                  libc::c_long)
                                                                as
                                                                krb5_error_code
                                                    } else {
                                                        /* Check for a 32-bit kvno extension if four or more bytes remain. */
                                                        pos =
                                                            ftell((*((*id).data
                                                                         as
                                                                         *mut krb5_ktfile_data)).openf)
                                                                as krb5_int32;
                                                        if pos - start_pos +
                                                               4 as
                                                                   libc::c_int
                                                               <= size {
                                                            if fread(&mut vno32
                                                                         as
                                                                         *mut uint32_t
                                                                         as
                                                                         *mut libc::c_void,
                                                                     ::std::mem::size_of::<uint32_t>()
                                                                         as
                                                                         libc::c_ulong,
                                                                     1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong,
                                                                     (*((*id).data
                                                                            as
                                                                            *mut krb5_ktfile_data)).openf)
                                                                   == 0 {
                                                                error =
                                                                    -(1765328202
                                                                          as
                                                                          libc::c_long)
                                                                        as
                                                                        krb5_error_code;
                                                                current_block
                                                                    =
                                                                    4639666093008142149;
                                                            } else {
                                                                if (*((*id).data
                                                                          as
                                                                          *mut krb5_ktfile_data)).version
                                                                       !=
                                                                       0x501
                                                                           as
                                                                           libc::c_int
                                                                   {
                                                                    vno32 =
                                                                        ntohl(vno32)
                                                                }
                                                                /* If the value is 0, the bytes are just zero-fill. */
                                                                if vno32 != 0
                                                                   {
                                                                    (*ret_entry).vno
                                                                        =
                                                                        vno32
                                                                }
                                                                current_block
                                                                    =
                                                                    16314074004867283505;
                                                            }
                                                        } else {
                                                            current_block =
                                                                16314074004867283505;
                                                        }
                                                        match current_block {
                                                            4639666093008142149
                                                            => {
                                                            }
                                                            _ =>
                                                            /*
     * Reposition file pointer to the next inter-record length field.
     */
                                                            {
                                                                if fseek((*((*id).data
                                                                                as
                                                                                *mut krb5_ktfile_data)).openf,
                                                                         (start_pos
                                                                              +
                                                                              size)
                                                                             as
                                                                             libc::c_long,
                                                                         0 as
                                                                             libc::c_int)
                                                                       ==
                                                                       -(1 as
                                                                             libc::c_int)
                                                                   {
                                                                    error =
                                                                        *__errno_location()
                                                                } else {
                                                                    return 0
                                                                               as
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
    }
    i = 0 as libc::c_int;
    while i < (*(*ret_entry).principal).length {
        free((*(*(*ret_entry).principal).data.offset(i as isize)).data as
                 *mut libc::c_void);
        i += 1
    }
    free((*(*ret_entry).principal).data as *mut libc::c_void);
    (*(*ret_entry).principal).data = 0 as *mut krb5_data;
    free((*ret_entry).principal as *mut libc::c_void);
    (*ret_entry).principal = 0 as krb5_principal;
    return error;
}
#[c2rust::src_loc = "1115:1"]
unsafe extern "C" fn krb5_ktfileint_read_entry(mut context: krb5_context,
                                               mut id: krb5_keytab,
                                               mut entryp:
                                                   *mut krb5_keytab_entry)
 -> krb5_error_code {
    let mut delete_point: krb5_int32 = 0;
    return krb5_ktfileint_internal_read_entry(context, id, entryp,
                                              &mut delete_point);
}
#[c2rust::src_loc = "1123:1"]
unsafe extern "C" fn krb5_ktfileint_write_entry(mut context: krb5_context,
                                                mut id: krb5_keytab,
                                                mut entry:
                                                    *mut krb5_keytab_entry)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut vno: krb5_octet = 0;
    let mut princ: *mut krb5_data = 0 as *mut krb5_data;
    let mut count: krb5_int16 = 0;
    let mut size: krb5_int16 = 0;
    let mut enctype: krb5_int16 = 0;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut timestamp: krb5_timestamp = 0;
    let mut princ_type: krb5_int32 = 0;
    let mut size_needed: krb5_int32 = 0;
    let mut commit_point: krb5_int32 = -(1 as libc::c_int);
    let mut vno32: uint32_t = 0;
    let mut i: libc::c_int = 0;
    retval = krb5_ktfileint_size_entry(context, entry, &mut size_needed);
    if retval != 0 { return retval }
    retval =
        krb5_ktfileint_find_slot(context, id, &mut size_needed,
                                 &mut commit_point);
    if retval != 0 { return retval }
    /* fseek to synchronise buffered I/O on the key table. */
    /* XXX Without the weird setbuf crock, can we get rid of this now?  */
    if fseek((*((*id).data as *mut krb5_ktfile_data)).openf,
             0 as libc::c_long, 1 as libc::c_int) < 0 as libc::c_int {
        return *__errno_location()
    }
    if (*((*id).data as *mut krb5_ktfile_data)).version ==
           0x501 as libc::c_int {
        count =
            ((*(*entry).principal).length as krb5_int16 as libc::c_int +
                 1 as libc::c_int) as krb5_int16
    } else {
        count = htons((*(*entry).principal).length as u_short) as krb5_int16
    }
    if !(fwrite(&mut count as *mut krb5_int16 as *const libc::c_void,
                ::std::mem::size_of::<krb5_int16>() as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                (*((*id).data as *mut krb5_ktfile_data)).openf) == 0) {
        size = (*(*entry).principal).realm.length as krb5_int16;
        if (*((*id).data as *mut krb5_ktfile_data)).version !=
               0x501 as libc::c_int {
            size = htons(size as uint16_t) as krb5_int16
        }
        if !(fwrite(&mut size as *mut krb5_int16 as *const libc::c_void,
                    ::std::mem::size_of::<krb5_int16>() as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    (*((*id).data as *mut krb5_ktfile_data)).openf) == 0) {
            if !(fwrite((*(*entry).principal).realm.data as
                            *const libc::c_void,
                        ::std::mem::size_of::<libc::c_char>() as
                            libc::c_ulong,
                        (*(*entry).principal).realm.length as libc::c_ulong,
                        (*((*id).data as *mut krb5_ktfile_data)).openf) == 0)
               {
                count = (*(*entry).principal).length as krb5_int16;
                i = 0 as libc::c_int;
                loop  {
                    if !(i < count as libc::c_int) {
                        current_block = 8845338526596852646;
                        break ;
                    }
                    princ =
                        &mut *(*(*entry).principal).data.offset(i as isize) as
                            *mut krb5_data;
                    size = (*princ).length as krb5_int16;
                    if (*((*id).data as *mut krb5_ktfile_data)).version !=
                           0x501 as libc::c_int {
                        size = htons(size as uint16_t) as krb5_int16
                    }
                    if fwrite(&mut size as *mut krb5_int16 as
                                  *const libc::c_void,
                              ::std::mem::size_of::<krb5_int16>() as
                                  libc::c_ulong,
                              1 as libc::c_int as libc::c_ulong,
                              (*((*id).data as *mut krb5_ktfile_data)).openf)
                           == 0 {
                        current_block = 16910009627297427231;
                        break ;
                    }
                    if fwrite((*princ).data as *const libc::c_void,
                              ::std::mem::size_of::<libc::c_char>() as
                                  libc::c_ulong,
                              (*princ).length as libc::c_ulong,
                              (*((*id).data as *mut krb5_ktfile_data)).openf)
                           == 0 {
                        current_block = 16910009627297427231;
                        break ;
                    }
                    i += 1
                }
                match current_block {
                    16910009627297427231 => { }
                    _ =>
                    /*
     * Write out the principal type
     */
                    {
                        if (*((*id).data as *mut krb5_ktfile_data)).version !=
                               0x501 as libc::c_int {
                            princ_type =
                                htonl((*(*entry).principal).type_0 as
                                          uint32_t) as krb5_int32;
                            if fwrite(&mut princ_type as *mut krb5_int32 as
                                          *const libc::c_void,
                                      ::std::mem::size_of::<krb5_int32>() as
                                          libc::c_ulong,
                                      1 as libc::c_int as libc::c_ulong,
                                      (*((*id).data as
                                             *mut krb5_ktfile_data)).openf) ==
                                   0 {
                                current_block = 16910009627297427231;
                            } else { current_block = 18435049525520518667; }
                        } else { current_block = 18435049525520518667; }
                        match current_block {
                            16910009627297427231 => { }
                            _ => {
                                /*
     * Fill in the time of day the entry was written to the keytab.
     */
                                if krb5_timeofday(context,
                                                  &mut (*entry).timestamp) !=
                                       0 {
                                    (*entry).timestamp = 0 as libc::c_int
                                }
                                if (*((*id).data as
                                          *mut krb5_ktfile_data)).version ==
                                       0x501 as libc::c_int {
                                    timestamp = (*entry).timestamp
                                } else {
                                    timestamp =
                                        htonl((*entry).timestamp as uint32_t)
                                            as krb5_timestamp
                                }
                                if !(fwrite(&mut timestamp as
                                                *mut krb5_timestamp as
                                                *const libc::c_void,
                                            ::std::mem::size_of::<krb5_timestamp>()
                                                as libc::c_ulong,
                                            1 as libc::c_int as libc::c_ulong,
                                            (*((*id).data as
                                                   *mut krb5_ktfile_data)).openf)
                                         == 0) {
                                    /* key version number */
                                    vno = (*entry).vno as krb5_octet;
                                    if !(fwrite(&mut vno as *mut krb5_octet as
                                                    *const libc::c_void,
                                                ::std::mem::size_of::<krb5_octet>()
                                                    as libc::c_ulong,
                                                1 as libc::c_int as
                                                    libc::c_ulong,
                                                (*((*id).data as
                                                       *mut krb5_ktfile_data)).openf)
                                             == 0) {
                                        /* key type */
                                        if (*((*id).data as
                                                  *mut krb5_ktfile_data)).version
                                               == 0x501 as libc::c_int {
                                            enctype =
                                                (*entry).key.enctype as
                                                    krb5_int16
                                        } else {
                                            enctype =
                                                htons((*entry).key.enctype as
                                                          uint16_t) as
                                                    krb5_int16
                                        }
                                        if !(fwrite(&mut enctype as
                                                        *mut krb5_int16 as
                                                        *const libc::c_void,
                                                    ::std::mem::size_of::<krb5_int16>()
                                                        as libc::c_ulong,
                                                    1 as libc::c_int as
                                                        libc::c_ulong,
                                                    (*((*id).data as
                                                           *mut krb5_ktfile_data)).openf)
                                                 == 0) {
                                            /* key length */
                                            if (*((*id).data as
                                                      *mut krb5_ktfile_data)).version
                                                   == 0x501 as libc::c_int {
                                                size =
                                                    (*entry).key.length as
                                                        krb5_int16
                                            } else {
                                                size =
                                                    htons((*entry).key.length
                                                              as uint16_t) as
                                                        krb5_int16
                                            }
                                            if !(fwrite(&mut size as
                                                            *mut krb5_int16 as
                                                            *const libc::c_void,
                                                        ::std::mem::size_of::<krb5_int16>()
                                                            as libc::c_ulong,
                                                        1 as libc::c_int as
                                                            libc::c_ulong,
                                                        (*((*id).data as
                                                               *mut krb5_ktfile_data)).openf)
                                                     == 0) {
                                                if !(fwrite((*entry).key.contents
                                                                as
                                                                *const libc::c_void,
                                                            ::std::mem::size_of::<krb5_octet>()
                                                                as
                                                                libc::c_ulong,
                                                            (*entry).key.length
                                                                as
                                                                libc::c_ulong,
                                                            (*((*id).data as
                                                                   *mut krb5_ktfile_data)).openf)
                                                         == 0) {
                                                    /* 32-bit key version number */
                                                    vno32 = (*entry).vno;
                                                    if (*((*id).data as
                                                              *mut krb5_ktfile_data)).version
                                                           !=
                                                           0x501 as
                                                               libc::c_int {
                                                        vno32 = htonl(vno32)
                                                    }
                                                    if !(fwrite(&mut vno32 as
                                                                    *mut uint32_t
                                                                    as
                                                                    *const libc::c_void,
                                                                ::std::mem::size_of::<uint32_t>()
                                                                    as
                                                                    libc::c_ulong,
                                                                1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong,
                                                                (*((*id).data
                                                                       as
                                                                       *mut krb5_ktfile_data)).openf)
                                                             == 0) {
                                                        if !(fflush((*((*id).data
                                                                           as
                                                                           *mut krb5_ktfile_data)).openf)
                                                                 != 0) {
                                                            retval =
                                                                k5_sync_disk_file(context,
                                                                                  (*((*id).data
                                                                                         as
                                                                                         *mut krb5_ktfile_data)).openf);
                                                            if retval != 0 {
                                                                return retval
                                                            }
                                                            if fseek((*((*id).data
                                                                            as
                                                                            *mut krb5_ktfile_data)).openf,
                                                                     commit_point
                                                                         as
                                                                         libc::c_long,
                                                                     0 as
                                                                         libc::c_int)
                                                                   != 0 {
                                                                return *__errno_location()
                                                            }
                                                            if (*((*id).data
                                                                      as
                                                                      *mut krb5_ktfile_data)).version
                                                                   !=
                                                                   0x501 as
                                                                       libc::c_int
                                                               {
                                                                size_needed =
                                                                    htonl(size_needed
                                                                              as
                                                                              uint32_t)
                                                                        as
                                                                        krb5_int32
                                                            }
                                                            if !(fwrite(&mut size_needed
                                                                            as
                                                                            *mut krb5_int32
                                                                            as
                                                                            *const libc::c_void,
                                                                        ::std::mem::size_of::<krb5_int32>()
                                                                            as
                                                                            libc::c_ulong,
                                                                        1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong,
                                                                        (*((*id).data
                                                                               as
                                                                               *mut krb5_ktfile_data)).openf)
                                                                     == 0) {
                                                                if !(fflush((*((*id).data
                                                                                   as
                                                                                   *mut krb5_ktfile_data)).openf)
                                                                         != 0)
                                                                   {
                                                                    retval =
                                                                        k5_sync_disk_file(context,
                                                                                          (*((*id).data
                                                                                                 as
                                                                                                 *mut krb5_ktfile_data)).openf);
                                                                    return retval
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
    return -(1765328200 as libc::c_long) as krb5_error_code;
}
/*
 * Determine the size needed for a file entry for the given
 * keytab entry.
 */
#[c2rust::src_loc = "1272:1"]
unsafe extern "C" fn krb5_ktfileint_size_entry(mut context: krb5_context,
                                               mut entry:
                                                   *mut krb5_keytab_entry,
                                               mut size_needed:
                                                   *mut krb5_int32)
 -> krb5_error_code {
    let mut count: krb5_int16 = 0;
    let mut total_size: krb5_int32 = 0;
    let mut i: krb5_int32 = 0;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    count = (*(*entry).principal).length as krb5_int16;
    total_size =
        ::std::mem::size_of::<krb5_int16>() as libc::c_ulong as krb5_int32;
    total_size =
        (total_size as
             libc::c_ulong).wrapping_add(((*(*entry).principal).realm.length
                                              as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_int16>()
                                                                              as
                                                                              libc::c_ulong))
            as krb5_int32 as krb5_int32;
    i = 0 as libc::c_int;
    while i < count as libc::c_int {
        total_size =
            (total_size as
                 libc::c_ulong).wrapping_add(((*(*(*entry).principal).data.offset(i
                                                                                      as
                                                                                      isize)).length
                                                  as
                                                  libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_int16>()
                                                                                  as
                                                                                  libc::c_ulong))
                as krb5_int32 as krb5_int32;
        i += 1
    }
    total_size =
        (total_size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_int32>()
                                             as libc::c_ulong) as krb5_int32
            as krb5_int32;
    total_size =
        (total_size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_timestamp>()
                                             as libc::c_ulong) as krb5_int32
            as krb5_int32;
    total_size =
        (total_size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_octet>()
                                             as libc::c_ulong) as krb5_int32
            as krb5_int32;
    total_size =
        (total_size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_int16>()
                                             as libc::c_ulong) as krb5_int32
            as krb5_int32;
    total_size =
        (total_size as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<krb5_int16>()
                                              as
                                              libc::c_ulong).wrapping_add((*entry).key.length
                                                                              as
                                                                              libc::c_ulong))
            as krb5_int32 as krb5_int32;
    total_size =
        (total_size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<uint32_t>() as
                                             libc::c_ulong) as krb5_int32 as
            krb5_int32;
    *size_needed = total_size;
    return retval;
}
/*
 * Find and reserve a slot in the file for an entry of the needed size.
 * The commit point will be set to the position in the file where the
 * the length (sizeof(krb5_int32) bytes) of this node should be written
 * when commiting the write.  The file position left as a result of this
 * call is the position where the actual data should be written.
 *
 * The size_needed argument may be adjusted if we find a hole that is
 * larger than the size needed.  (Recall that size_needed will be used
 * to commit the write, but that this field must indicate the size of the
 * block in the file rather than the size of the actual entry)
 */
#[c2rust::src_loc = "1310:1"]
unsafe extern "C" fn krb5_ktfileint_find_slot(mut context: krb5_context,
                                              mut id: krb5_keytab,
                                              mut size_needed:
                                                  *mut krb5_int32,
                                              mut commit_point_ptr:
                                                  *mut krb5_int32)
 -> krb5_error_code {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut size: krb5_int32 = 0;
    let mut zero_point: krb5_int32 = 0;
    let mut commit_point: krb5_int32 = 0;
    let mut kt_vno: krb5_kt_vno = 0;
    fp = (*((*id).data as *mut krb5_ktfile_data)).openf;
    /* Skip over file version number. */
    if fseek(fp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int) != 0 {
        return *__errno_location()
    }
    if fread(&mut kt_vno as *mut krb5_kt_vno as *mut libc::c_void,
             ::std::mem::size_of::<krb5_kt_vno>() as libc::c_ulong,
             1 as libc::c_int as libc::c_ulong, fp) == 0 {
        return *__errno_location()
    }
    loop  {
        commit_point = ftell(fp) as krb5_int32;
        if commit_point == -(1 as libc::c_int) { return *__errno_location() }
        if fread(&mut size as *mut krb5_int32 as *mut libc::c_void,
                 ::std::mem::size_of::<krb5_int32>() as libc::c_ulong,
                 1 as libc::c_int as libc::c_ulong, fp) == 0 {
            /* Hit the end of file, reserve this slot. */
            /* Necessary to avoid a later fseek failing on Solaris 10. */
            if fseek(fp, 0 as libc::c_int as libc::c_long, 1 as libc::c_int)
                   != 0 {
                return *__errno_location()
            }
            /* htonl(0) is 0, so no need to worry about byte order */
            size = 0 as libc::c_int;
            if fwrite(&mut size as *mut krb5_int32 as *const libc::c_void,
                      ::std::mem::size_of::<krb5_int32>() as libc::c_ulong,
                      1 as libc::c_int as libc::c_ulong, fp) == 0 {
                return *__errno_location()
            }
            break ;
        } else {
            if (*((*id).data as *mut krb5_ktfile_data)).version !=
                   0x501 as libc::c_int {
                size = ntohl(size as uint32_t) as krb5_int32
            }
            if size > 0 as libc::c_int {
                /* Non-empty record; seek past it. */
                if fseek(fp, size as libc::c_long, 1 as libc::c_int) != 0 {
                    return *__errno_location()
                }
            } else if size < 0 as libc::c_int {
                /* Empty record; use if it's big enough, seek past otherwise. */
                size = -size;
                if size >= *size_needed {
                    *size_needed = size;
                    break ;
                } else if fseek(fp, size as libc::c_long, 1 as libc::c_int) !=
                              0 {
                    return *__errno_location()
                }
            } else {
                /* Empty record at end of file; use it. */
            /* Ensure the new record will be followed by another 0. */
                zero_point = ftell(fp) as krb5_int32;
                if zero_point == -(1 as libc::c_int) {
                    return *__errno_location()
                }
                if fseek(fp, *size_needed as libc::c_long, 1 as libc::c_int)
                       != 0 {
                    return *__errno_location()
                }
                /* htonl(0) is 0, so no need to worry about byte order */
                if fwrite(&mut size as *mut krb5_int32 as *const libc::c_void,
                          ::std::mem::size_of::<krb5_int32>() as
                              libc::c_ulong,
                          1 as libc::c_int as libc::c_ulong, fp) == 0 {
                    return *__errno_location()
                }
                if fseek(fp, zero_point as libc::c_long, 0 as libc::c_int) !=
                       0 {
                    return *__errno_location()
                }
                break ;
            }
        }
    }
    *commit_point_ptr = commit_point;
    return 0 as libc::c_int;
}
/* LEAN_CLIENT */
