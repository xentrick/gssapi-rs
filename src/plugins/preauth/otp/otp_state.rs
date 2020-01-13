use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:30"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:30"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:30"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:30"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:30"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:30"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:30"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:30"]
pub mod krb5_h {
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
    #[c2rust::src_loc = "398:16"]
    pub struct _krb5_enc_data {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub kvno: krb5_kvno,
        pub ciphertext: krb5_data,
    }
    #[c2rust::src_loc = "398:1"]
    pub type krb5_enc_data = _krb5_enc_data;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint32_t;
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
        /* *< Omit realm always */
        /* *< Don't escape special characters */
        /* *
 * Convert krb5_principal structure to a string with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [in]  flags           Flags
 * @param [out] name            String representation of principal name
 *
 * Similar to krb5_unparse_name(), this function converts a krb5_principal
 * structure to a string representation.
 *
 * The following flags are valid:
 * @li #KRB5_PRINCIPAL_UNPARSE_SHORT - omit realm if it is the local realm
 * @li #KRB5_PRINCIPAL_UNPARSE_NO_REALM - omit realm
 * @li #KRB5_PRINCIPAL_UNPARSE_DISPLAY - do not quote special characters
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
        #[no_mangle]
        #[c2rust::src_loc = "3547:1"]
        pub fn krb5_unparse_name_flags(context: krb5_context,
                                       principal: krb5_const_principal,
                                       flags: libc::c_int,
                                       name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "5131:1"]
        pub fn krb5_get_profile(context: krb5_context,
                                profile: *mut *mut _profile_t)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:30"]
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
    #[c2rust::src_loc = "2268:1"]
    pub unsafe extern "C" fn string2data(mut str: *mut libc::c_char)
     -> krb5_data {
        return make_data(str as *mut libc::c_void,
                         strlen(str) as libc::c_uint);
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
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    /* Like k5memdup, but add a final null byte. */
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data, krb5_enc_data, krb5_timestamp,
                        krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::k5_int_pkinit_h::krb5_algorithm_identifier;
    use super::string_h::{strlen, memcpy};
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:30"]
pub mod profile_h {
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn profile_abandon(profile: profile_t);
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn profile_get_values(profile: profile_t,
                                  names: *const *const libc::c_char,
                                  ret_values: *mut *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn profile_free_list(list: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn profile_get_string(profile: profile_t,
                                  name: *const libc::c_char,
                                  subname: *const libc::c_char,
                                  subsubname: *const libc::c_char,
                                  def_val: *const libc::c_char,
                                  ret_string: *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn profile_get_integer(profile: profile_t,
                                   name: *const libc::c_char,
                                   subname: *const libc::c_char,
                                   subsubname: *const libc::c_char,
                                   def_val: libc::c_int,
                                   ret_default: *mut libc::c_int)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn profile_get_boolean(profile: profile_t,
                                   name: *const libc::c_char,
                                   subname: *const libc::c_char,
                                   subsubname: *const libc::c_char,
                                   def_val: libc::c_int,
                                   ret_default: *mut libc::c_int)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn profile_get_subsection_names(profile: profile_t,
                                            names: *mut *const libc::c_char,
                                            ret_names:
                                                *mut *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn profile_release_string(str: *mut libc::c_char);
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:30"]
pub mod k5_int_pkinit_h {
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
    use super::krb5_h::krb5_data;
    /* Optional */
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/verto.h:30"]
pub mod verto_h {
    extern "C" {
        #[c2rust::src_loc = "48:16"]
        pub type verto_ctx;
    }
    /* VERTO_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/otp/otp_state.h:30"]
pub mod otp_state_h {
    #[c2rust::src_loc = "38:9"]
    pub type otp_response = libc::c_uint;
    #[c2rust::src_loc = "40:5"]
    pub const otp_response_success: otp_response = 1;
    #[c2rust::src_loc = "39:5"]
    pub const otp_response_fail: otp_response = 0;
    #[c2rust::src_loc = "44:1"]
    pub type otp_state = otp_state_st;
    #[c2rust::src_loc = "45:1"]
    pub type otp_cb
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_error_code,
                                    _: otp_response,
                                    _: *const *mut libc::c_char) -> ()>;
    use super::otp_state_st;
    use super::krb5_h::krb5_error_code;
    /* OTP_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krad.h:32"]
pub mod krad_h {
    #[c2rust::src_loc = "60:1"]
    pub type krad_attrset = krad_attrset_st;
    #[c2rust::src_loc = "62:1"]
    pub type krad_client = krad_client_st;
    #[c2rust::src_loc = "64:1"]
    pub type krad_attr = libc::c_uchar;
    #[c2rust::src_loc = "61:1"]
    pub type krad_packet = krad_packet_st;
    #[c2rust::src_loc = "63:1"]
    pub type krad_code = libc::c_uchar;
    /* Called when a response is received or the request times out. */
    #[c2rust::src_loc = "67:1"]
    pub type krad_cb
        =
        Option<unsafe extern "C" fn(_: krb5_error_code, _: *const krad_packet,
                                    _: *const krad_packet,
                                    _: *mut libc::c_void) -> ()>;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_data, krb5_ui_4};
    use super::k5_int_h::_krb5_context;
    use super::stddef_h::size_t;
    use super::verto_h::verto_ctx;
    extern "C" {
        #[c2rust::src_loc = "60:16"]
        pub type krad_attrset_st;
        #[c2rust::src_loc = "62:16"]
        pub type krad_client_st;
        #[c2rust::src_loc = "61:16"]
        pub type krad_packet_st;
        /*
 * Code
 */
        /* Convert a code name to its number. Only works for codes defined
 * by RFC 2875 or 2882. Returns 0 if the name was not found. */
        #[no_mangle]
        #[c2rust::src_loc = "85:1"]
        pub fn krad_code_name2num(name: *const libc::c_char) -> krad_code;
        /*
 * Attribute
 */
        /* Convert an attribute name to its number. Only works for attributes defined
 * by RFC 2865. Returns 0 if the name was not found. */
        #[no_mangle]
        #[c2rust::src_loc = "99:1"]
        pub fn krad_attr_name2num(name: *const libc::c_char) -> krad_attr;
        /*
 * Attribute set
 */
        /* Create a new attribute set. */
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn krad_attrset_new(ctx: krb5_context,
                                set: *mut *mut krad_attrset)
         -> krb5_error_code;
        /* Create a deep copy of an attribute set. */
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn krad_attrset_copy(set: *const krad_attrset,
                                 copy: *mut *mut krad_attrset)
         -> krb5_error_code;
        /* Free an attribute set. */
        #[no_mangle]
        #[c2rust::src_loc = "120:1"]
        pub fn krad_attrset_free(set: *mut krad_attrset);
        /* Add an attribute to a set. */
        #[no_mangle]
        #[c2rust::src_loc = "124:1"]
        pub fn krad_attrset_add(set: *mut krad_attrset, type_0: krad_attr,
                                data: *const krb5_data) -> krb5_error_code;
        /* Add a four-octet unsigned number attribute to the given set. */
        #[no_mangle]
        #[c2rust::src_loc = "128:1"]
        pub fn krad_attrset_add_number(set: *mut krad_attrset,
                                       type_0: krad_attr, num: krb5_ui_4)
         -> krb5_error_code;
        /* Delete the specified attribute. */
        #[no_mangle]
        #[c2rust::src_loc = "132:1"]
        pub fn krad_attrset_del(set: *mut krad_attrset, type_0: krad_attr,
                                indx: size_t);
        /* Get the code for the given packet. */
        #[no_mangle]
        #[c2rust::src_loc = "219:1"]
        pub fn krad_packet_get_code(pkt: *const krad_packet) -> krad_code;
        /*
 * Client
 */
        /* Create a new client. */
        /* Free the client. */
        /*
 * Send a request to a radius server.
 *
 * The remote host may be specified by one of the following formats:
 *  - /path/to/unix.socket
 *  - IPv4
 *  - IPv4:port
 *  - IPv4:service
 *  - [IPv6]
 *  - [IPv6]:port
 *  - [IPv6]:service
 *  - hostname
 *  - hostname:port
 *  - hostname:service
 *
 * The timeout parameter (milliseconds) is the total timeout across all remote
 * hosts (when DNS returns multiple entries) and all retries.  For stream
 * sockets, the retries parameter is ignored and no retries are performed.
 *
 * The cb function will be called with the data argument when either a response
 * is received or the request times out on all possible remote hosts.
 */
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn krad_client_send(rc: *mut krad_client, code: krad_code,
                                attrs: *const krad_attrset,
                                remote: *const libc::c_char,
                                secret: *const libc::c_char,
                                timeout: libc::c_int, retries: size_t,
                                cb: krad_cb, data: *mut libc::c_void)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "231:1"]
        pub fn krad_client_new(kctx: krb5_context, vctx: *mut verto_ctx,
                               client: *mut *mut krad_client)
         -> krb5_error_code;
    }
    /* KRAD_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-json.h:33"]
pub mod k5_json_h {
    /*
 * Array
 */
    #[c2rust::src_loc = "128:1"]
    pub type k5_json_array = *mut k5_json_array_st;
    /*
 * The k5_json_value C type can represent any kind of JSON value.  It has no
 * static type safety since it is represented using a void pointer, so be
 * careful with it.  Its type can be checked dynamically with k5_json_get_tid()
 * and the above constants.
 */
    #[c2rust::src_loc = "86:1"]
    pub type k5_json_value = *mut libc::c_void;
    /*
 * Object
 */
    #[c2rust::src_loc = "160:1"]
    pub type k5_json_object = *mut k5_json_object_st;
    /*
 * String
 */
    #[c2rust::src_loc = "186:1"]
    pub type k5_json_string = *mut k5_json_string_st;
    #[c2rust::src_loc = "87:1"]
    pub type k5_json_tid = libc::c_uint;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "128:16"]
        pub type k5_json_array_st;
        #[c2rust::src_loc = "160:16"]
        pub type k5_json_object_st;
        #[c2rust::src_loc = "186:16"]
        pub type k5_json_string_st;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn k5_json_get_tid(val: k5_json_value) -> k5_json_tid;
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn k5_json_release(val: k5_json_value);
        #[no_mangle]
        #[c2rust::src_loc = "131:1"]
        pub fn k5_json_array_length(array: k5_json_array) -> size_t;
        /* Both of these functions increment the reference count on val. */
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn k5_json_array_add(array: k5_json_array, val: k5_json_value)
         -> libc::c_int;
        /* Get an alias to the idx-th element of array, without incrementing the
 * reference count.  The caller must check idx against the array length. */
        #[no_mangle]
        #[c2rust::src_loc = "139:1"]
        pub fn k5_json_array_get(array: k5_json_array, idx: size_t)
         -> k5_json_value;
        #[no_mangle]
        #[c2rust::src_loc = "164:1"]
        pub fn k5_json_object_create(val_out: *mut k5_json_object)
         -> libc::c_int;
        /* Get an alias to the object's value for key, without incrementing the
 * reference count.  Returns NULL if there is no value for key. */
        #[no_mangle]
        #[c2rust::src_loc = "180:1"]
        pub fn k5_json_object_get(obj: k5_json_object,
                                  key: *const libc::c_char) -> k5_json_value;
        #[no_mangle]
        #[c2rust::src_loc = "191:1"]
        pub fn k5_json_string_utf8(string: k5_json_string)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "216:1"]
        pub fn k5_json_decode(str: *const libc::c_char,
                              val_out: *mut k5_json_value) -> libc::c_int;
    }
    /* K5_JSON_H */
}
#[c2rust::header_src = "/usr/include/ctype.h:35"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:30"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:30"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:30"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:30"]
pub mod unistd_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "877:1"]
        pub fn gethostname(__name: *mut libc::c_char, __len: size_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:30"]
pub mod k5_platform_h {
    extern "C" {
        /*
 * Compose two path components, inserting the platform-appropriate path
 * separator if needed.  If path2 is an absolute path, path1 will be discarded
 * and path_out will be a copy of path2.  Returns 0 on success or ENOMEM on
 * allocation failure.
 */
        #[no_mangle]
        #[c2rust::src_loc = "1074:1"]
        pub fn k5_path_join(path1: *const libc::c_char,
                            path2: *const libc::c_char,
                            path_out: *mut *mut libc::c_char) -> libc::c_long;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/string.h:30"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
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
pub use self::types_h::{__int32_t, __uint32_t, __off_t, __off64_t, __ssize_t};
pub use self::sys_types_h::ssize_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_int32, krb5_ui_4, krb5_boolean, krb5_kvno,
                       krb5_enctype, krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_const_principal,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_enc_data, krb5_enc_data,
                       _profile_t, krb5_unparse_name, krb5_unparse_name_flags,
                       krb5_free_unparsed_name, krb5_get_profile};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_pa_otp_req, krb5_pa_otp_req,
                         make_data, string2data, k5calloc, k5alloc, k5memdup0,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_abandon, profile_get_values,
                          profile_free_list, profile_get_string,
                          profile_get_integer, profile_get_boolean,
                          profile_get_subsection_names,
                          profile_release_string};
pub use self::com_err_h::{errcode_t, com_err};
pub use self::k5_int_pkinit_h::{_krb5_algorithm_identifier,
                                krb5_algorithm_identifier};
use self::verto_h::verto_ctx;
pub use self::otp_state_h::{otp_response, otp_response_success,
                            otp_response_fail, otp_state, otp_cb};
pub use self::krad_h::{krad_attrset, krad_client, krad_attr, krad_packet,
                       krad_code, krad_cb, krad_attrset_st, krad_client_st,
                       krad_packet_st, krad_code_name2num, krad_attr_name2num,
                       krad_attrset_new, krad_attrset_copy, krad_attrset_free,
                       krad_attrset_add, krad_attrset_add_number,
                       krad_attrset_del, krad_packet_get_code,
                       krad_client_send, krad_client_new};
pub use self::k5_json_h::{k5_json_array, k5_json_value, k5_json_object,
                          k5_json_string, k5_json_tid, k5_json_array_st,
                          k5_json_object_st, k5_json_string_st,
                          k5_json_get_tid, k5_json_release,
                          k5_json_array_length, k5_json_array_add,
                          k5_json_array_get, k5_json_object_create,
                          k5_json_object_get, k5_json_string_utf8,
                          k5_json_decode};
pub use self::ctype_h::{_ISspace, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
use self::stdlib_h::{calloc, free};
use self::stdio_h::{fclose, fopen, asprintf, fgets};
use self::errno_h::__errno_location;
use self::unistd_h::gethostname;
use self::k5_platform_h::k5_path_join;
use self::string_h::{strlen, strdup, strcmp, memset, memcpy};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "73:8"]
pub struct otp_state_st {
    pub ctx: krb5_context,
    pub types: *mut token_type,
    pub radius: *mut krad_client,
    pub attrs: *mut krad_attrset,
}
#[c2rust::src_loc = "48:1"]
pub type token_type = token_type_st;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "48:16"]
pub struct token_type_st {
    pub name: *mut libc::c_char,
    pub server: *mut libc::c_char,
    pub secret: *mut libc::c_char,
    pub timeout: libc::c_int,
    pub retries: size_t,
    pub strip_realm: krb5_boolean,
    pub indicators: *mut *mut libc::c_char,
}
#[c2rust::src_loc = "58:1"]
pub type token = token_st;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "58:16"]
pub struct token_st {
    pub type_0: *const token_type,
    pub username: krb5_data,
    pub indicators: *mut *mut libc::c_char,
}
#[c2rust::src_loc = "64:1"]
pub type request = request_st;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "64:16"]
pub struct request_st {
    pub state: *mut otp_state,
    pub tokens: *mut token,
    pub index: ssize_t,
    pub cb: otp_cb,
    pub data: *mut libc::c_void,
    pub attrs: *mut krad_attrset,
}
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn read_secret_file(mut secret_file: *const libc::c_char,
                                      mut secret: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut retval: krb5_error_code = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    *secret = 0 as *mut libc::c_char;
    retval =
        k5_path_join(b"/usr/local/var/krb5kdc\x00" as *const u8 as
                         *const libc::c_char, secret_file, &mut filename) as
            krb5_error_code;
    if retval != 0 as libc::c_int {
        com_err(b"otp\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"Unable to resolve secret file \'%s\'\x00" as *const u8 as
                    *const libc::c_char, filename);
    } else {
        file = fopen(filename, b"r\x00" as *const u8 as *const libc::c_char);
        if file.is_null() {
            retval = *__errno_location();
            com_err(b"otp\x00" as *const u8 as *const libc::c_char,
                    retval as errcode_t,
                    b"Unable to open secret file \'%s\'\x00" as *const u8 as
                        *const libc::c_char, filename);
        } else {
            if fgets(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong as libc::c_int, file).is_null() {
                retval = 5 as libc::c_int
            }
            fclose(file);
            if retval != 0 as libc::c_int {
                com_err(b"otp\x00" as *const u8 as *const libc::c_char,
                        retval as errcode_t,
                        b"Unable to read secret file \'%s\'\x00" as *const u8
                            as *const libc::c_char, filename);
            } else {
                /* Strip whitespace. */
                i = 0 as libc::c_int as size_t;
                while buf[i as usize] as libc::c_int != '\u{0}' as i32 {
                    if *(*__ctype_b_loc()).offset(buf[i as usize] as
                                                      libc::c_int as isize) as
                           libc::c_int &
                           _ISspace as libc::c_int as libc::c_ushort as
                               libc::c_int == 0 {
                        break ;
                    }
                    i = i.wrapping_add(1)
                }
                j = strlen(buf.as_mut_ptr());
                while j > i {
                    if *(*__ctype_b_loc()).offset(buf[j.wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                                                          as usize] as
                                                      libc::c_int as isize) as
                           libc::c_int &
                           _ISspace as libc::c_int as libc::c_ushort as
                               libc::c_int == 0 {
                        break ;
                    }
                    j = j.wrapping_sub(1)
                }
                *secret =
                    k5memdup0(&mut *buf.as_mut_ptr().offset(i as isize) as
                                  *mut libc::c_char as *const libc::c_void,
                              j.wrapping_sub(i), &mut retval) as
                        *mut libc::c_char
            }
        }
    }
    free(filename as *mut libc::c_void);
    return retval;
}
/* Free the contents of a single token type. */
#[c2rust::src_loc = "132:1"]
unsafe extern "C" fn token_type_free(mut type_0: *mut token_type) {
    if type_0.is_null() { return }
    free((*type_0).name as *mut libc::c_void);
    free((*type_0).server as *mut libc::c_void);
    free((*type_0).secret as *mut libc::c_void);
    profile_free_list((*type_0).indicators);
}
/* Construct the internal default token type. */
#[c2rust::src_loc = "145:1"]
unsafe extern "C" fn token_type_default(mut out: *mut token_type)
 -> krb5_error_code {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut server: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut secret: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(out as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<token_type>() as libc::c_ulong);
    name = strdup(b"DEFAULT\x00" as *const u8 as *const libc::c_char);
    if !name.is_null() {
        if !(asprintf(&mut server as *mut *mut libc::c_char,
                      b"/usr/local/var/run/krb5kdc/%s.socket\x00" as *const u8
                          as *const libc::c_char, name) < 0 as libc::c_int) {
            secret = strdup(b"\x00" as *const u8 as *const libc::c_char);
            if !secret.is_null() {
                (*out).name = name;
                (*out).server = server;
                (*out).secret = secret;
                (*out).timeout = 5 as libc::c_int * 1000 as libc::c_int;
                (*out).retries = 3 as libc::c_int as size_t;
                (*out).strip_realm = 0 as libc::c_int as krb5_boolean;
                return 0 as libc::c_int
            }
        }
    }
    free(name as *mut libc::c_void);
    free(server as *mut libc::c_void);
    free(secret as *mut libc::c_void);
    return 12 as libc::c_int;
}
/* Decode a single token type from the profile. */
#[c2rust::src_loc = "177:1"]
unsafe extern "C" fn token_type_decode(mut profile: profile_t,
                                       mut name: *const libc::c_char,
                                       mut out: *mut token_type)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut server: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut secret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut indicators: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut keys: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    let mut strip_realm: libc::c_int = 0;
    let mut timeout: libc::c_int = 0;
    let mut retries: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    memset(out as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<token_type>() as libc::c_ulong);
    /* Set the name. */
    name_copy = strdup(name);
    if name_copy.is_null() { return 12 as libc::c_int }
    /* Set strip_realm. */
    retval =
        profile_get_boolean(profile,
                            b"otp\x00" as *const u8 as *const libc::c_char,
                            name,
                            b"strip_realm\x00" as *const u8 as
                                *const libc::c_char, 1 as libc::c_int,
                            &mut strip_realm) as krb5_error_code;
    if !(retval != 0 as libc::c_int) {
        /* Set the server. */
        retval =
            profile_get_string(profile,
                               b"otp\x00" as *const u8 as *const libc::c_char,
                               name,
                               b"server\x00" as *const u8 as
                                   *const libc::c_char,
                               0 as *const libc::c_char, &mut pstr) as
                krb5_error_code;
        if !(retval != 0 as libc::c_int) {
            if !pstr.is_null() {
                server = strdup(pstr);
                profile_release_string(pstr);
                if server.is_null() {
                    retval = 12 as libc::c_int;
                    current_block = 8996832148142699521;
                } else { current_block = 5948590327928692120; }
            } else if asprintf(&mut server as *mut *mut libc::c_char,
                               b"/usr/local/var/run/krb5kdc/%s.socket\x00" as
                                   *const u8 as *const libc::c_char, name) <
                          0 as libc::c_int {
                retval = 12 as libc::c_int;
                current_block = 8996832148142699521;
            } else { current_block = 5948590327928692120; }
            match current_block {
                8996832148142699521 => { }
                _ => {
                    /* Get the secret (optional for Unix-domain sockets). */
                    retval =
                        profile_get_string(profile,
                                           b"otp\x00" as *const u8 as
                                               *const libc::c_char, name,
                                           b"secret\x00" as *const u8 as
                                               *const libc::c_char,
                                           0 as *const libc::c_char,
                                           &mut pstr) as krb5_error_code;
                    if !(retval != 0 as libc::c_int) {
                        if !pstr.is_null() {
                            retval = read_secret_file(pstr, &mut secret);
                            profile_release_string(pstr);
                            if retval != 0 as libc::c_int {
                                current_block = 8996832148142699521;
                            } else { current_block = 3275366147856559585; }
                        } else if *server.offset(0 as libc::c_int as isize) as
                                      libc::c_int != '/' as i32 {
                            com_err(b"otp\x00" as *const u8 as
                                        *const libc::c_char,
                                    22 as libc::c_int as errcode_t,
                                    b"Secret missing (token type \'%s\')\x00"
                                        as *const u8 as *const libc::c_char,
                                    name);
                            retval = 22 as libc::c_int;
                            current_block = 8996832148142699521;
                        } else {
                            /* Use the default empty secret for UNIX domain stream sockets. */
                            secret =
                                strdup(b"\x00" as *const u8 as
                                           *const libc::c_char);
                            if secret.is_null() {
                                retval = 12 as libc::c_int;
                                current_block = 8996832148142699521;
                            } else { current_block = 3275366147856559585; }
                        }
                        match current_block {
                            8996832148142699521 => { }
                            _ => {
                                /* Get the timeout (profile value in seconds, result in milliseconds). */
                                retval =
                                    profile_get_integer(profile,
                                                        b"otp\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        name,
                                                        b"timeout\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        5 as libc::c_int,
                                                        &mut timeout) as
                                        krb5_error_code;
                                if !(retval != 0 as libc::c_int) {
                                    timeout *= 1000 as libc::c_int;
                                    /* Get the number of retries. */
                                    retval =
                                        profile_get_integer(profile,
                                                            b"otp\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            name,
                                                            b"retries\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            3 as libc::c_int,
                                                            &mut retries) as
                                            krb5_error_code;
                                    if !(retval != 0 as libc::c_int) {
                                        /* Get the authentication indicators to assert if this token is used. */
                                        keys[0 as libc::c_int as usize] =
                                            b"otp\x00" as *const u8 as
                                                *const libc::c_char;
                                        keys[1 as libc::c_int as usize] =
                                            name;
                                        keys[2 as libc::c_int as usize] =
                                            b"indicator\x00" as *const u8 as
                                                *const libc::c_char;
                                        keys[3 as libc::c_int as usize] =
                                            0 as *const libc::c_char;
                                        retval =
                                            profile_get_values(profile,
                                                               keys.as_mut_ptr(),
                                                               &mut indicators)
                                                as krb5_error_code;
                                        if retval as libc::c_long ==
                                               -(1429577725 as libc::c_long) {
                                            retval = 0 as libc::c_int
                                        }
                                        if !(retval != 0 as libc::c_int) {
                                            (*out).name = name_copy;
                                            (*out).server = server;
                                            (*out).secret = secret;
                                            (*out).timeout = timeout;
                                            (*out).retries =
                                                retries as size_t;
                                            (*out).strip_realm =
                                                strip_realm as krb5_boolean;
                                            (*out).indicators = indicators;
                                            secret = 0 as *mut libc::c_char;
                                            server = secret;
                                            name_copy = server;
                                            indicators =
                                                0 as *mut *mut libc::c_char
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
    free(name_copy as *mut libc::c_void);
    free(server as *mut libc::c_void);
    free(secret as *mut libc::c_void);
    profile_free_list(indicators);
    return retval;
}
/* Free an array of token types. */
#[c2rust::src_loc = "282:1"]
unsafe extern "C" fn token_types_free(mut types: *mut token_type) {
    let mut i: size_t = 0;
    if types.is_null() { return }
    i = 0 as libc::c_int as size_t;
    while !(*types.offset(i as isize)).server.is_null() {
        token_type_free(&mut *types.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free(types as *mut libc::c_void);
}
/* Decode an array of token types from the profile. */
#[c2rust::src_loc = "297:1"]
unsafe extern "C" fn token_types_decode(mut profile: profile_t,
                                        mut out: *mut *mut token_type)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut hier: [*const libc::c_char; 2] =
        [b"otp\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char];
    let mut types: *mut token_type = 0 as *mut token_type;
    let mut names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut retval: krb5_error_code = 0;
    let mut i: size_t = 0;
    let mut pos: size_t = 0;
    let mut have_default: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    retval =
        profile_get_subsection_names(profile, hier.as_mut_ptr(), &mut names)
            as krb5_error_code;
    if retval != 0 as libc::c_int { return retval }
    /* Check if any of the profile subsections overrides the default. */
    i = 0 as libc::c_int as size_t;
    while !(*names.offset(i as isize)).is_null() {
        if strcmp(*names.offset(i as isize),
                  b"DEFAULT\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            have_default = 1 as libc::c_int as krb5_boolean
        }
        i = i.wrapping_add(1)
    }
    /* Leave space for the default (possibly) and the terminator. */
    types =
        k5calloc(i.wrapping_add(2 as libc::c_int as libc::c_ulong),
                 ::std::mem::size_of::<token_type>() as libc::c_ulong,
                 &mut retval) as *mut token_type;
    if !types.is_null() {
        /* If no default has been specified, use our internal default. */
        pos = 0 as libc::c_int as size_t;
        if have_default == 0 {
            let fresh0 = pos;
            pos = pos.wrapping_add(1);
            retval = token_type_default(&mut *types.offset(fresh0 as isize));
            if retval != 0 as libc::c_int {
                current_block = 8315345885692132479;
            } else { current_block = 17833034027772472439; }
        } else { current_block = 17833034027772472439; }
        match current_block {
            8315345885692132479 => { }
            _ =>
            /* Decode each profile section into a token type element. */
            {
                i = 0 as libc::c_int as size_t;
                loop  {
                    if (*names.offset(i as isize)).is_null() {
                        current_block = 2668756484064249700;
                        break ;
                    }
                    let fresh1 = pos;
                    pos = pos.wrapping_add(1);
                    retval =
                        token_type_decode(profile, *names.offset(i as isize),
                                          &mut *types.offset(fresh1 as
                                                                 isize));
                    if retval != 0 as libc::c_int {
                        current_block = 8315345885692132479;
                        break ;
                    }
                    i = i.wrapping_add(1)
                }
                match current_block {
                    8315345885692132479 => { }
                    _ => { *out = types; types = 0 as *mut token_type }
                }
            }
        }
    }
    profile_free_list(names);
    token_types_free(types);
    return retval;
}
/* Free a null-terminated array of strings. */
#[c2rust::src_loc = "347:1"]
unsafe extern "C" fn free_strings(mut list: *mut *mut libc::c_char) {
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    p = list;
    while !p.is_null() && !(*p).is_null() {
        free(*p as *mut libc::c_void);
        p = p.offset(1)
    }
    free(list as *mut libc::c_void);
}
/* Free the contents of a single token. */
#[c2rust::src_loc = "358:1"]
unsafe extern "C" fn token_free_contents(mut t: *mut token) {
    if t.is_null() { return }
    free((*t).username.data as *mut libc::c_void);
    free_strings((*t).indicators);
}
/* Decode a JSON array of strings into a null-terminated list of C strings. */
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn indicators_decode(mut ctx: krb5_context,
                                       mut val: k5_json_value,
                                       mut indicators_out:
                                           *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut arr: k5_json_array = 0 as *mut k5_json_array_st;
    let mut obj: k5_json_value = 0 as *mut libc::c_void;
    let mut indicators: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    *indicators_out = 0 as *mut *mut libc::c_char;
    if k5_json_get_tid(val) != 129 as libc::c_int as libc::c_uint {
        return 22 as libc::c_int
    }
    arr = val as k5_json_array;
    len = k5_json_array_length(arr);
    indicators =
        calloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) as
            *mut *mut libc::c_char;
    if indicators.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int as size_t;
    while i < len {
        obj = k5_json_array_get(arr, i);
        if k5_json_get_tid(obj) != 131 as libc::c_int as libc::c_uint {
            free_strings(indicators);
            return 22 as libc::c_int
        }
        let ref mut fresh2 = *indicators.offset(i as isize);
        *fresh2 = strdup(k5_json_string_utf8(obj as k5_json_string));
        if (*indicators.offset(i as isize)).is_null() {
            free_strings(indicators);
            return 12 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    *indicators_out = indicators;
    return 0 as libc::c_int;
}
/* Decode a single token from a JSON token object. */
#[c2rust::src_loc = "404:1"]
unsafe extern "C" fn token_decode(mut ctx: krb5_context,
                                  mut princ: krb5_const_principal,
                                  mut types: *const token_type,
                                  mut obj: k5_json_object,
                                  mut out: *mut token) -> krb5_error_code {
    let mut typename: *const libc::c_char =
        b"DEFAULT\x00" as *const u8 as *const libc::c_char;
    let mut type_0: *const token_type = 0 as *const token_type;
    let mut username: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut indicators: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut retval: krb5_error_code = 0;
    let mut val: k5_json_value = 0 as *mut libc::c_void;
    let mut i: size_t = 0;
    let mut flags: libc::c_int = 0;
    memset(out as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<token>() as libc::c_ulong);
    /* Find the token type. */
    val =
        k5_json_object_get(obj,
                           b"type\x00" as *const u8 as *const libc::c_char);
    if !val.is_null() &&
           k5_json_get_tid(val) == 131 as libc::c_int as libc::c_uint {
        typename = k5_json_string_utf8(val as k5_json_string)
    }
    i = 0 as libc::c_int as size_t;
    while !(*types.offset(i as isize)).server.is_null() {
        if strcmp(typename, (*types.offset(i as isize)).name) ==
               0 as libc::c_int {
            type_0 = &*types.offset(i as isize) as *const token_type
        }
        i = i.wrapping_add(1)
    }
    if type_0.is_null() { return 22 as libc::c_int }
    /* Get the username, either from obj or from unparsing the principal. */
    val =
        k5_json_object_get(obj,
                           b"username\x00" as *const u8 as
                               *const libc::c_char);
    if !val.is_null() &&
           k5_json_get_tid(val) == 131 as libc::c_int as libc::c_uint {
        username = strdup(k5_json_string_utf8(val as k5_json_string));
        if username.is_null() { return 12 as libc::c_int }
    } else {
        flags =
            if (*type_0).strip_realm != 0 {
                0x2 as libc::c_int
            } else { 0 as libc::c_int };
        retval = krb5_unparse_name_flags(ctx, princ, flags, &mut username);
        if retval != 0 as libc::c_int { return retval }
    }
    /* Get the authentication indicators if specified. */
    val =
        k5_json_object_get(obj,
                           b"indicators\x00" as *const u8 as
                               *const libc::c_char);
    if !val.is_null() {
        retval = indicators_decode(ctx, val, &mut indicators);
        if retval != 0 as libc::c_int {
            free(username as *mut libc::c_void);
            return retval
        }
    }
    (*out).type_0 = type_0;
    (*out).username = string2data(username);
    (*out).indicators = indicators;
    return 0 as libc::c_int;
}
/* Free an array of tokens. */
#[c2rust::src_loc = "459:1"]
unsafe extern "C" fn tokens_free(mut tokens: *mut token) {
    let mut i: size_t = 0;
    if tokens.is_null() { return }
    i = 0 as libc::c_int as size_t;
    while !(*tokens.offset(i as isize)).type_0.is_null() {
        token_free_contents(&mut *tokens.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free(tokens as *mut libc::c_void);
}
/* Decode a principal config string into a JSON array.  Treat an empty string
 * or array as if it were "[{}]" which uses the default token type. */
#[c2rust::src_loc = "475:1"]
unsafe extern "C" fn decode_config_json(mut config: *const libc::c_char,
                                        mut out: *mut k5_json_array)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut val: k5_json_value = 0 as *mut libc::c_void;
    let mut obj: k5_json_object = 0 as *mut k5_json_object_st;
    *out = 0 as k5_json_array;
    /* Decode the config string and make sure it's an array. */
    retval =
        k5_json_decode(if !config.is_null() {
                           config
                       } else {
                           b"[{}]\x00" as *const u8 as *const libc::c_char
                       }, &mut val);
    if !(retval != 0 as libc::c_int) {
        if k5_json_get_tid(val) != 129 as libc::c_int as libc::c_uint {
            retval = 22 as libc::c_int
        } else {
            /* If the array is empty, add in an empty object. */
            if k5_json_array_length(val as k5_json_array) ==
                   0 as libc::c_int as libc::c_ulong {
                retval = k5_json_object_create(&mut obj);
                if retval != 0 as libc::c_int {
                    current_block = 10485851455608404399;
                } else {
                    retval =
                        k5_json_array_add(val as k5_json_array,
                                          obj as k5_json_value);
                    k5_json_release(obj as k5_json_value);
                    if retval != 0 as libc::c_int {
                        current_block = 10485851455608404399;
                    } else { current_block = 12800627514080957624; }
                }
            } else { current_block = 12800627514080957624; }
            match current_block {
                10485851455608404399 => { }
                _ => { *out = val as k5_json_array; return 0 as libc::c_int }
            }
        }
    }
    k5_json_release(val);
    return retval;
}
/* Decode an array of tokens from the configuration string. */
#[c2rust::src_loc = "513:1"]
unsafe extern "C" fn tokens_decode(mut ctx: krb5_context,
                                   mut princ: krb5_const_principal,
                                   mut types: *const token_type,
                                   mut config: *const libc::c_char,
                                   mut out: *mut *mut token)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut arr: k5_json_array = 0 as k5_json_array;
    let mut obj: k5_json_value = 0 as *mut libc::c_void;
    let mut tokens: *mut token = 0 as *mut token;
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    retval = decode_config_json(config, &mut arr);
    if retval != 0 as libc::c_int { return retval }
    len = k5_json_array_length(arr);
    tokens =
        k5calloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                 ::std::mem::size_of::<token>() as libc::c_ulong, &mut retval)
            as *mut token;
    if !tokens.is_null() {
        i = 0 as libc::c_int as size_t;
        loop  {
            if !(i < len) { current_block = 8831408221741692167; break ; }
            obj = k5_json_array_get(arr, i);
            if k5_json_get_tid(obj) != 130 as libc::c_int as libc::c_uint {
                retval = 22 as libc::c_int;
                current_block = 7573220398060824864;
                break ;
            } else {
                retval =
                    token_decode(ctx, princ, types, obj as k5_json_object,
                                 &mut *tokens.offset(i as isize));
                if retval != 0 as libc::c_int {
                    current_block = 7573220398060824864;
                    break ;
                }
                i = i.wrapping_add(1)
            }
        }
        match current_block {
            7573220398060824864 => { }
            _ => { *out = tokens; tokens = 0 as *mut token }
        }
    }
    k5_json_release(arr as k5_json_value);
    tokens_free(tokens);
    return retval;
}
#[c2rust::src_loc = "552:1"]
unsafe extern "C" fn request_free(mut req: *mut request) {
    if req.is_null() { return }
    krad_attrset_free((*req).attrs);
    tokens_free((*req).tokens);
    free(req as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "563:1"]
pub unsafe extern "C" fn otp_state_new(mut ctx: krb5_context,
                                       mut out: *mut *mut otp_state)
 -> krb5_error_code {
    let mut hostname: [libc::c_char; 65] = [0; 65];
    let mut retval: krb5_error_code = 0;
    let mut profile: profile_t = 0 as *mut _profile_t;
    let mut hndata: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut self_0: *mut otp_state = 0 as *mut otp_state;
    retval =
        gethostname(hostname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 65]>() as
                        libc::c_ulong);
    if retval != 0 as libc::c_int { return retval }
    self_0 =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<otp_state>() as libc::c_ulong) as
            *mut otp_state;
    if self_0.is_null() { return 12 as libc::c_int }
    retval = krb5_get_profile(ctx, &mut profile);
    if !(retval != 0 as libc::c_int) {
        retval = token_types_decode(profile, &mut (*self_0).types);
        profile_abandon(profile);
        if !(retval != 0 as libc::c_int) {
            retval = krad_attrset_new(ctx, &mut (*self_0).attrs);
            if !(retval != 0 as libc::c_int) {
                hndata =
                    make_data(hostname.as_mut_ptr() as *mut libc::c_void,
                              strlen(hostname.as_mut_ptr()) as libc::c_uint);
                retval =
                    krad_attrset_add((*self_0).attrs,
                                     krad_attr_name2num(b"NAS-Identifier\x00"
                                                            as *const u8 as
                                                            *const libc::c_char),
                                     &mut hndata);
                if !(retval != 0 as libc::c_int) {
                    retval =
                        krad_attrset_add_number((*self_0).attrs,
                                                krad_attr_name2num(b"Service-Type\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char),
                                                8 as libc::c_int as
                                                    krb5_ui_4);
                    if !(retval != 0 as libc::c_int) {
                        (*self_0).ctx = ctx;
                        *out = self_0;
                        return 0 as libc::c_int
                    }
                }
            }
        }
    }
    otp_state_free(self_0);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "614:1"]
pub unsafe extern "C" fn otp_state_free(mut self_0: *mut otp_state) {
    if self_0.is_null() { return }
    krad_attrset_free((*self_0).attrs);
    token_types_free((*self_0).types);
    free(self_0 as *mut libc::c_void);
}
#[c2rust::src_loc = "625:1"]
unsafe extern "C" fn callback(mut retval: krb5_error_code,
                              mut rqst: *const krad_packet,
                              mut resp: *const krad_packet,
                              mut data: *mut libc::c_void) {
    let mut req: *mut request = data as *mut request;
    let mut tok: *mut token =
        &mut *(*req).tokens.offset((*req).index as isize) as *mut token;
    let mut indicators: *const *mut libc::c_char =
        0 as *const *mut libc::c_char;
    (*req).index += 1;
    if !(retval != 0 as libc::c_int) {
        /* If we received an accept packet, success! */
        if krad_packet_get_code(resp) as libc::c_int ==
               krad_code_name2num(b"Access-Accept\x00" as *const u8 as
                                      *const libc::c_char) as libc::c_int {
            indicators = (*tok).indicators;
            if indicators.is_null() {
                indicators = (*(*tok).type_0).indicators
            }
            (*req).cb.expect("non-null function pointer")((*req).data, retval,
                                                          otp_response_success,
                                                          indicators);
            request_free(req);
            return
        }
        /* If we have no more tokens to try, failure! */
        if !(*(*req).tokens.offset((*req).index as isize)).type_0.is_null() {
            /* Try the next token. */
            request_send(req);
            return
        }
    }
    (*req).cb.expect("non-null function pointer")((*req).data, retval,
                                                  otp_response_fail,
                                                  0 as
                                                      *const *mut libc::c_char);
    request_free(req);
}
#[c2rust::src_loc = "662:1"]
unsafe extern "C" fn request_send(mut req: *mut request) {
    let mut retval: krb5_error_code = 0;
    let mut tok: *mut token =
        &mut *(*req).tokens.offset((*req).index as isize) as *mut token;
    let mut t: *const token_type = (*tok).type_0;
    retval =
        krad_attrset_add((*req).attrs,
                         krad_attr_name2num(b"User-Name\x00" as *const u8 as
                                                *const libc::c_char),
                         &mut (*tok).username);
    if !(retval != 0 as libc::c_int) {
        retval =
            krad_client_send((*(*req).state).radius,
                             krad_code_name2num(b"Access-Request\x00" as
                                                    *const u8 as
                                                    *const libc::c_char),
                             (*req).attrs, (*t).server, (*t).secret,
                             (*t).timeout, (*t).retries,
                             Some(callback as
                                      unsafe extern "C" fn(_: krb5_error_code,
                                                           _:
                                                               *const krad_packet,
                                                           _:
                                                               *const krad_packet,
                                                           _:
                                                               *mut libc::c_void)
                                          -> ()), req as *mut libc::c_void);
        krad_attrset_del((*req).attrs,
                         krad_attr_name2num(b"User-Name\x00" as *const u8 as
                                                *const libc::c_char),
                         0 as libc::c_int as size_t);
        if !(retval != 0 as libc::c_int) { return }
    }
    (*req).cb.expect("non-null function pointer")((*req).data, retval,
                                                  otp_response_fail,
                                                  0 as
                                                      *const *mut libc::c_char);
    request_free(req);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/preauth/otp/otp_state.h - Internal declarations for OTP module */
/*
 * Copyright 2013 Red Hat, Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *    1. Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *    2. Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
 * OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/* Other values reserved for responses like next token or new pin. */
#[no_mangle]
#[c2rust::src_loc = "689:1"]
pub unsafe extern "C" fn otp_state_verify(mut state: *mut otp_state,
                                          mut ctx: *mut verto_ctx,
                                          mut princ: krb5_const_principal,
                                          mut config: *const libc::c_char,
                                          mut req: *const krb5_pa_otp_req,
                                          mut cb: otp_cb,
                                          mut data: *mut libc::c_void) {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut rqst: *mut request = 0 as *mut request;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*state).radius.is_null() {
        retval = krad_client_new((*state).ctx, ctx, &mut (*state).radius);
        if retval != 0 as libc::c_int {
            current_block = 6838892553957440395;
        } else { current_block = 7095457783677275021; }
    } else { current_block = 7095457783677275021; }
    match current_block {
        7095457783677275021 => {
            rqst =
                calloc(1 as libc::c_int as libc::c_ulong,
                       ::std::mem::size_of::<request>() as libc::c_ulong) as
                    *mut request;
            if rqst.is_null() {
                Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                                 12
                                                                                                     as
                                                                                                     libc::c_int,
                                                                                                 otp_response_fail,
                                                                                                 0
                                                                                                     as
                                                                                                     *const *mut libc::c_char);
                return
            }
            (*rqst).state = state;
            (*rqst).data = data;
            (*rqst).cb = cb;
            retval = krad_attrset_copy((*state).attrs, &mut (*rqst).attrs);
            if !(retval != 0 as libc::c_int) {
                retval =
                    krad_attrset_add((*rqst).attrs,
                                     krad_attr_name2num(b"User-Password\x00"
                                                            as *const u8 as
                                                            *const libc::c_char),
                                     &(*req).otp_value);
                if !(retval != 0 as libc::c_int) {
                    retval =
                        tokens_decode((*state).ctx, princ, (*state).types,
                                      config, &mut (*rqst).tokens);
                    if retval != 0 as libc::c_int {
                        if krb5_unparse_name((*state).ctx, princ, &mut name)
                               == 0 as libc::c_int {
                            com_err(b"otp\x00" as *const u8 as
                                        *const libc::c_char,
                                    retval as errcode_t,
                                    b"Can\'t decode otp config string for principal \'%s\'\x00"
                                        as *const u8 as *const libc::c_char,
                                    name);
                            krb5_free_unparsed_name((*state).ctx, name);
                        }
                    } else { request_send(rqst); return }
                }
            }
        }
        _ => { }
    }
    Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(data,
                                                                                     retval,
                                                                                     otp_response_fail,
                                                                                     0
                                                                                         as
                                                                                         *const *mut libc::c_char);
    request_free(rqst);
}
