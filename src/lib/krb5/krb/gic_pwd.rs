use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:2"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:2"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:2"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:2"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:2"]
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
 * Free a string allocated by a krb5 function.
 *
 * @param [in] context          Library context
 * @param [in] val              String to be freed
 *
 * @version New in 1.10
 */
    /* *
 * Free an array of encryption types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of enctypes to be freed
 *
 * @version New in 1.12
 */
    /* *
 * Free an array of checksum types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of checksum types to be freed
 */
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
    /* *
 * Check if a timestamp is within the allowed clock skew of the current time.
 *
 * @param [in]     context      Library context
 * @param [in]     date         Timestamp to check
 *
 * This function checks if @a date is close enough to the current time
 * according to the configured allowable clock skew.
 *
 * @version New in 1.10
 *
 * @retval 0 Success
 * @retval KRB5KRB_AP_ERR_SKEW @a date is not within allowable clock skew
 */
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
    /* *
 * Override the default realm for the specified context.
 *
 * @param [in]     context      Library context
 * @param [in]     lrealm       Realm name for the default realm
 *
 * If @a lrealm is NULL, clear the default realm setting.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Free a default realm string returned by krb5_get_default_realm().
 *
 * @param [in] context          Library context
 * @param [in] lrealm           Realm to be freed
 */
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
    /* *
 * Generate a full principal name from a service name.
 *
 * @param [in]  context         Library context
 * @param [in]  hostname        Host name, or NULL to use local host
 * @param [in]  sname           Service name, or NULL to use @c "host"
 * @param [in]  type            Principal type
 * @param [out] ret_princ       Generated principal
 *
 * This function converts a @a hostname and @a sname into @a krb5_principal
 * structure @a ret_princ.  The returned principal will be of the form @a
 * sname\/hostname\@REALM where REALM is determined by krb5_get_host_realm().
 * In some cases this may be the referral (empty) realm.
 *
 * The @a type can be one of the following:
 *
 * @li #KRB5_NT_SRV_HST canonicalizes the host name before looking up the
 * realm and generating the principal.
 *
 * @li #KRB5_NT_UNKNOWN accepts the hostname as given, and does not
 * canonicalize it.
 *
 * Use krb5_free_principal to free @a ret_princ when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Test whether a principal matches a matching principal.
 *
 * @param [in]  context         Library context
 * @param [in]  matching        Matching principal
 * @param [in]  princ           Principal to test
 *
 * @note A matching principal is a host-based principal with an empty realm
 * and/or second data component (hostname).  Profile configuration may cause
 * the hostname to be ignored even if it is present.  A principal matches a
 * matching principal if the former has the same non-empty (and non-ignored)
 * components of the latter.
 *
 * If @a matching is NULL, return TRUE.  If @a matching is not a matching
 * principal, return the value of krb5_principal_compare(context, matching,
 * princ).
 *
 * @return
 * TRUE if @a princ matches @a matching, FALSE otherwise.
 */
    /* *
 * Change a password for an existing Kerberos account.
 *
 * @param [in]  context             Library context
 * @param [in]  creds               Credentials for kadmin/changepw service
 * @param [in]  newpw               New password
 * @param [out] result_code         Numeric error code from server
 * @param [out] result_code_string  String equivalent to @a result_code
 * @param [out] result_string       Change password response from the KDC
 *
 * Change the password for the existing principal identified by @a creds.
 *
 * The possible values of the output @a result_code are:
 *
 * @li #KRB5_KPASSWD_SUCCESS   (0) - success
 * @li #KRB5_KPASSWD_MALFORMED (1) - Malformed request error
 * @li #KRB5_KPASSWD_HARDERROR (2) - Server error
 * @li #KRB5_KPASSWD_AUTHERROR (3) - Authentication error
 * @li #KRB5_KPASSWD_SOFTERROR (4) - Password change rejected
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set a password for a principal using specified credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  creds                Credentials for kadmin/changepw service
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the credentials @a creds to set the password @a newpw for
 * the principal @a change_password_for.  It implements the set password
 * operation of RFC 3244, for interoperability with Microsoft Windows
 * implementations.
 *
 * @note If @a change_password_for is NULL, the change is performed on the
 * current principal. If @a change_password_for is non-null, the change is
 * performed on the principal name passed in @a change_password_for.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @sa krb5_set_password_using_ccache()
 *
 * @retval
 * 0  Success and result_code is set to #KRB5_KPASSWD_SUCCESS.
 * @return
 * Kerberos error codes.
 */
    /* *
 * Set a password for a principal using cached credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  ccache               Credential cache
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the cached credentials from @a ccache to set the password
 * @a newpw for the principal @a change_password_for.  It implements RFC 3244
 * set password operation (interoperable with MS Windows implementations) using
 * the credential cache.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @note If @a change_password_for is set to NULL, the change is performed on
 * the default principal in @a ccache. If @a change_password_for is non null,
 * the change is performed on the specified principal.
 *
 * @sa krb5_set_password()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Get a result message for changing or setting a password.
 *
 * @param [in]  context            Library context
 * @param [in]  server_string      Data returned from the remote system
 * @param [out] message_out        A message displayable to the user
 *
 * This function processes the @a server_string returned in the @a
 * result_string parameter of krb5_change_password(), krb5_set_password(), and
 * related functions, and returns a displayable string.  If @a server_string
 * contains Active Directory structured policy information, it will be
 * converted into human-readable text.
 *
 * Use krb5_free_string() to free @a message_out when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 *
 * @version New in 1.11
 */
    /* *
 * Retrieve configuration profile from the context.
 *
 * @param [in]  context         Library context
 * @param [out] profile         Pointer to data read from a configuration file
 *
 * This function creates a new @a profile object that reflects profile
 * in the supplied @a context.
 *
 * The @a profile object may be freed with profile_release() function.
 * See profile.h and profile API for more details.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
    /* * @deprecated Replaced by krb5_get_init_creds_password().*/
    /* * @deprecated Replaced by krb5_get_init_creds(). */
    /* * @deprecated Replaced by krb5_get_init_creds_keytab(). */
    /* KRB5_DEPRECATED */
    /* *
 * Parse and decrypt a @c KRB_AP_REQ message.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     inbuf          AP-REQ message to be parsed
 * @param [in]     server         Matching principal for server, or NULL to
 *                                allow any principal in keytab
 * @param [in]     keytab         Key table, or NULL to use the default
 * @param [out]    ap_req_options If non-null, the AP-REQ flags on output
 * @param [out]    ticket         If non-null, ticket from the AP-REQ message
 *
 * This function parses, decrypts and verifies a AP-REQ message from @a inbuf
 * and stores the authenticator in @a auth_context.
 *
 * If a keyblock was specified in @a auth_context using
 * krb5_auth_con_setuseruserkey(), that key is used to decrypt the ticket in
 * AP-REQ message and @a keytab is ignored.  In this case, @a server should be
 * specified as a complete principal name to allow for proper transited-path
 * checking and replay cache selection.
 *
 * Otherwise, the decryption key is obtained from @a keytab, or from the
 * default keytab if it is NULL.  In this case, @a server may be a complete
 * principal name, a matching principal (see krb5_sname_match()), or NULL to
 * match any principal name.  The keys tried against the encrypted part of the
 * ticket are determined as follows:
 *
 * - If @a server is a complete principal name, then its entry in @a keytab is
 *   tried.
 * - Otherwise, if @a keytab is iterable, then all entries in @a keytab which
 *   match @a server are tried.
 * - Otherwise, the server principal in the ticket must match @a server, and
 *   its entry in @a keytab is tried.
 *
 * The client specified in the decrypted authenticator must match the client
 * specified in the decrypted ticket.
 *
 * If the @a remote_addr field of @a auth_context is set, the request must come
 * from that address.
 *
 * If a replay cache handle is provided in the @a auth_context, the
 * authenticator and ticket are verified against it.  If no conflict is found,
 * the new authenticator is then stored in the replay cache of @a auth_context.
 *
 * Various other checks are performed on the decoded data, including
 * cross-realm policy, clockskew, and ticket validation times.
 *
 * On success the authenticator, subkey, and remote sequence number of the
 * request are stored in @a auth_context. If the #AP_OPTS_MUTUAL_REQUIRED
 * bit is set, the local sequence number is XORed with the remote sequence
 * number in the request.
 *
 * Use krb5_free_ticket() to free @a ticket when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve a service key from a key table.
 *
 * @param [in]  context     Library context
 * @param [in]  keyprocarg  Name of a key table (NULL to use default name)
 * @param [in]  principal   Service principal
 * @param [in]  vno         Key version number (0 for highest available)
 * @param [in]  enctype     Encryption type (0 for any type)
 * @param [out] key         Service key from key table
 *
 * Open and search the specified key table for the entry identified by @a
 * principal, @a enctype, and @a vno. If no key is found, return an error code.
 *
 * The default key table is used, unless @a keyprocarg is non-null.
 * @a keyprocarg designates a specific key table.
 *
 * Use krb5_free_keyblock() to free @a key when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return Kerberos error code if not found or @a keyprocarg is invalid.
 */
    /* *
 * Format a @c KRB-SAFE message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  userdata        User data in the message
 * @param [out] der_out         Formatted @c KRB-SAFE buffer
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function creates an integrity protected @c KRB-SAFE message
 * using data supplied by the application.
 *
 * Fields in @a auth_context specify the checksum type, the keyblock that
 * can be used to seed the checksum, full addresses (host and port) for
 * the sender and receiver, and @ref KRB5_AUTH_CONTEXT flags.
 *
 * The local address in @a auth_context must be set, and is used to form the
 * sender address used in the KRB-SAFE message.  The remote address is
 * optional; if specified, it will be used to form the receiver address used in
 * the message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, a
 * timestamp is included in the KRB-SAFE message, and an entry for the message
 * is entered in an in-memory replay cache to detect if the message is
 * reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not set, no
 * replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, a timestamp is included in the KRB-SAFE message and is stored
 * in @a rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-SAFE message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Format a @c KRB-PRIV message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  userdata        User data for @c KRB-PRIV message
 * @param [out] der_out         Formatted @c KRB-PRIV message
 * @param [out] rdata_out       Replay data (NULL if not needed)
 *
 * This function is similar to krb5_mk_safe(), but the message is encrypted and
 * integrity-protected, not just integrity-protected.
 *
 * The local address in @a auth_context must be set, and is used to form the
 * sender address used in the KRB-PRIV message.  The remote address is
 * optional; if specified, it will be used to form the receiver address used in
 * the message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, a
 * timestamp is included in the KRB-PRIV message, and an entry for the message
 * is entered in an in-memory replay cache to detect if the message is
 * reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not set, no
 * replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, a timestamp is included in the KRB-PRIV message and is stored
 * in @a rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-PRIV message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Client function for @c sendauth protocol.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     fd             File descriptor that describes network socket
 * @param [in]     appl_version   Application protocol version to be matched
 *                                with the receiver's application version
 * @param [in]     client         Client principal
 * @param [in]     server         Server principal
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     in_data        Data to be sent to the server
 * @param [in]     in_creds       Input credentials, or NULL to use @a ccache
 * @param [in]     ccache         Credential cache
 * @param [out]    error          If non-null, contains KRB_ERROR message
 *                                returned from server
 * @param [out]    rep_result     If non-null and @a ap_req_options is
 *                                #AP_OPTS_MUTUAL_REQUIRED, contains the result
 *                                of mutual authentication exchange
 * @param [out]    out_creds      If non-null, the retrieved credentials
 *
 * This function performs the client side of a sendauth/recvauth exchange by
 * sending and receiving messages over @a fd.
 *
 * Credentials may be specified in three ways:
 *
 * @li If @a in_creds is NULL, credentials are obtained with
 * krb5_get_credentials() using the principals @a client and @a server.  @a
 * server must be non-null; @a client may NULL to use the default principal of
 * @a ccache.
 *
 * @li If @a in_creds is non-null, but does not contain a ticket, credentials
 * for the exchange are obtained with krb5_get_credentials() using @a in_creds.
 * In this case, the values of @a client and @a server are unused.
 *
 * @li If @a in_creds is a complete credentials structure, it used directly.
 * In this case, the values of @a client, @a server, and @a ccache are unused.
 *
 * If the server is using a different application protocol than that specified
 * in @a appl_version, an error will be returned.
 *
 * Use krb5_free_creds() to free @a out_creds, krb5_free_ap_rep_enc_part() to
 * free @a rep_result, and krb5_free_error() to free @a error when they are no
 * longer needed.
 *
 * @sa krb5_recvauth()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Server function for @a sendauth protocol.
 *
 * @param [in]     context      Library context
 * @param [in,out] auth_context Pre-existing or newly created auth context
 * @param [in]     fd           File descriptor
 * @param [in]     appl_version Application protocol version to be matched
 *                              against the client's application version
 * @param [in]     server       Server principal (NULL for any in @a keytab)
 * @param [in]     flags        Additional specifications
 * @param [in]     keytab       Key table containing service keys
 * @param [out]    ticket       Ticket (NULL if not needed)
 *
 * This function performs the server side of a sendauth/recvauth exchange by
 * sending and receiving messages over @a fd.
 *
 * Use krb5_free_ticket() to free @a ticket when it is no longer needed.
 *
 * @sa krb5_sendauth()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Server function for @a sendauth protocol with version parameter.
 *
 * @param [in]     context      Library context
 * @param [in,out] auth_context Pre-existing or newly created auth context
 * @param [in]     fd           File descriptor
 * @param [in]     server       Server principal (NULL for any in @a keytab)
 * @param [in]     flags        Additional specifications
 * @param [in]     keytab       Decryption key
 * @param [out]    ticket       Ticket (NULL if not needed)
 * @param [out]    version      sendauth protocol version (NULL if not needed)
 *
 * This function is similar to krb5_recvauth() with the additional output
 * information place into @a version.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Format a @c KRB-CRED message for an array of credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creds           Null-terminated array of credentials
 * @param [out] der_out         Encoded credentials
 * @param [out] rdata_out       Replay cache information (NULL if not needed)
 *
 * This function takes an array of credentials @a creds and formats
 * a @c KRB-CRED message @a der_out to pass to krb5_rd_cred().
 *
 * The local and remote addresses in @a auth_context are optional; if either is
 * specified, they are used to form the sender and receiver addresses in the
 * KRB-CRED message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, an entry
 * for the message is entered in an in-memory replay cache to detect if the
 * message is reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not
 * set, no replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, the timestamp used for the KRB-CRED message is stored in @a
 * rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-CRED message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * The message will be encrypted using the send subkey of @a auth_context if it
 * is present, or the session key otherwise.  If neither key is present, the
 * credentials will not be encrypted, and the message should only be sent over
 * a secure channel.  No replay cache entry is used in this case.
 *
 * @retval
 *  0 Success
 * @retval
 *  ENOMEM Insufficient memory
 * @retval
 *   KRB5_RC_REQUIRED Message replay detection requires @a rcache parameter
 * @return
 * Kerberos error codes
 */
    /* *
 * Format a @c KRB-CRED message for a single set of credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creds           Pointer to credentials
 * @param [out] der_out         Encoded credentials
 * @param [out] rdata_out       Replay cache data (NULL if not needed)
 *
 * This is a convenience function that calls krb5_mk_ncred() with a single set
 * of credentials.
 *
 * @retval
 * 0 Success
 * @retval
 *  ENOMEM Insufficient memory
 * @retval
 *  KRB5_RC_REQUIRED   Message replay detection requires @a rcache parameter
 * @return
 * Kerberos error codes
 */
    /* *
 * Read and validate a @c KRB-CRED message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creddata        @c KRB-CRED message
 * @param [out] creds_out       Null-terminated array of forwarded credentials
 * @param [out] rdata_out       Replay data (NULL if not needed)
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.`
 *
 * @a creddata will be decrypted using the receiving subkey if it is present in
 * @a auth_context, or the session key if the receiving subkey is not present
 * or fails to decrypt the message.
 *
 * Use krb5_free_tgt_creds() to free @a creds_out when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Get a forwarded TGT and format a @c KRB-CRED message.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] rhost            Remote host
 * @param [in] client           Client principal of TGT
 * @param [in] server           Principal of server to receive TGT
 * @param [in] cc               Credential cache handle (NULL to use default)
 * @param [in] forwardable      Whether TGT should be forwardable
 * @param [out] outbuf          KRB-CRED message
 *
 * Get a TGT for use at the remote host @a rhost and format it into a KRB-CRED
 * message.  If @a rhost is NULL and @a server is of type #KRB5_NT_SRV_HST,
 * the second component of @a server will be used.
 *
 * @retval
 *  0 Success
 * @retval
 *   ENOMEM Insufficient memory
 * @retval
 *   KRB5_PRINC_NOMATCH Requested principal and ticket do not match
 * @retval
 *   KRB5_NO_TKT_SUPPLIED Request did not supply a ticket
 * @retval
 *   KRB5_CC_BADNAME Credential cache name or principal name malformed
 * @return
 * Kerberos error codes
 */
    /* *
 * Create and initialize an authentication context.
 *
 * @param [in]  context         Library context
 * @param [out] auth_context    Authentication context
 *
 * This function creates an authentication context to hold configuration and
 * state relevant to krb5 functions for authenticating principals and
 * protecting messages once authentication has occurred.
 *
 * By default, flags for the context are set to enable the use of the replay
 * cache (#KRB5_AUTH_CONTEXT_DO_TIME), but not sequence numbers.  Use
 * krb5_auth_con_setflags() to change the flags.
 *
 * The allocated @a auth_context must be freed with krb5_auth_con_free() when
 * it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a krb5_auth_context structure.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context to be freed
 *
 * This function frees an auth context allocated by krb5_auth_con_init().
 *
 * @retval 0  (always)
 */
    /* *
 * Set a flags field in a krb5_auth_context structure.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] flags            Flags bit mask
 *
 * Valid values for @a flags are:
 * @li #KRB5_AUTH_CONTEXT_DO_TIME Use timestamps
 * @li #KRB5_AUTH_CONTEXT_RET_TIME Save timestamps
 * @li #KRB5_AUTH_CONTEXT_DO_SEQUENCE Use sequence numbers
 * @li #KRB5_AUTH_CONTEXT_RET_SEQUENCE Save sequence numbers
 *
 * @retval 0 (always)
 */
    /* *
 * Retrieve flags from a krb5_auth_context structure.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] flags           Flags bit mask
 *
 * Valid values for @a flags are:
 * @li #KRB5_AUTH_CONTEXT_DO_TIME Use timestamps
 * @li #KRB5_AUTH_CONTEXT_RET_TIME Save timestamps
 * @li #KRB5_AUTH_CONTEXT_DO_SEQUENCE Use sequence numbers
 * @li #KRB5_AUTH_CONTEXT_RET_SEQUENCE Save sequence numbers
 *
 * @retval 0 (always)
 */
    /* *
 * Set a checksum callback in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] func             Checksum callback
 * @param [in] data             Callback argument
 *
 * Set a callback to obtain checksum data in krb5_mk_req().  The callback will
 * be invoked after the subkey and local sequence number are stored in @a
 * auth_context.
 *
 * @retval 0 (always)
 */
    /* *
 * Get the checksum callback from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] func            Checksum callback
 * @param [out] data            Callback argument
 *
 * @retval 0 (always)
 */
    /* *
 * Set the local and remote addresses in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_addr       Local address
 * @param [in] remote_addr      Remote address
 *
 * This function releases the storage assigned to the contents of the local and
 * remote addresses of @a auth_context and then sets them to @a local_addr and
 * @a remote_addr respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve address fields from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] local_addr      Local address (NULL if not needed)
 * @param [out] remote_addr     Remote address (NULL if not needed)
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set local and remote port fields in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_port       Local port
 * @param [in] remote_port      Remote port
 *
 * This function releases the storage assigned to the contents of the local and
 * remote ports of @a auth_context and then sets them to @a local_port and @a
 * remote_port respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the session key in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] keyblock         User key
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the session key from an auth context as a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] keyblock        Session key
 *
 * This function creates a keyblock containing the session key from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the session key from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] key             Session key
 *
 * This function sets @a key to the session key from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 (always)
 */
    /* *
 * Retrieve the send subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] keyblock        Send subkey
 *
 * This function creates a keyblock containing the send subkey from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the send subkey from an auth context.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Send subkey
 *
 * This function sets @a key to the send subkey from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the receiving subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] keyblock        Receiving subkey
 *
 * This function creates a keyblock containing the receiving subkey from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the receiving subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Receiving subkey
 *
 * This function sets @a key to the receiving subkey from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[c2rust::src_loc = "6811:1"]
    pub type krb5_get_init_creds_opt = _krb5_get_init_creds_opt;
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
    #[c2rust::src_loc = "7158:1"]
    pub type krb5_expire_callback_func
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_timestamp, _: krb5_timestamp,
                                    _: krb5_boolean) -> ()>;
    #[c2rust::src_loc = "6431:1"]
    pub type krb5_prompter_fct
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char, _: libc::c_int,
                                    _: *mut krb5_prompt) -> krb5_error_code>;
    #[c2rust::src_loc = "6424:1"]
    pub type krb5_prompt = _krb5_prompt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6424:16"]
    pub struct _krb5_prompt {
        pub prompt: *mut libc::c_char,
        pub hidden: libc::c_int,
        pub reply: *mut krb5_data,
    }
    #[c2rust::src_loc = "7313:1"]
    pub type krb5_init_creds_context = *mut _krb5_init_creds_context;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    use super::init_creds_ctx_h::_krb5_init_creds_context;
    use super::stddef_h::size_t;
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
 * Convert a string (such as a password) to a key with additional parameters.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  string          String to be converted
 * @param [in]  salt            Salt value
 * @param [in]  params          Parameters
 * @param [out] key             Generated key
 *
 * This function is similar to krb5_c_string_to_key(), but also takes
 * parameters which may affect the algorithm in an enctype-dependent way.  The
 * newly created @a key must be released by calling
 * krb5_free_keyblock_contents() when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "861:1"]
        pub fn krb5_c_string_to_key_with_params(context: krb5_context,
                                                enctype: krb5_enctype,
                                                string: *const krb5_data,
                                                salt: *const krb5_data,
                                                params: *const krb5_data,
                                                key: *mut krb5_keyblock)
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
 * Free a string representation of a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Name string to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5011:1"]
        pub fn krb5_change_password(context: krb5_context,
                                    creds: *mut krb5_creds,
                                    newpw: *const libc::c_char,
                                    result_code: *mut libc::c_int,
                                    result_code_string: *mut krb5_data,
                                    result_string: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5110:1"]
        pub fn krb5_chpw_message(context: krb5_context,
                                 server_string: *const krb5_data,
                                 message_out: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6863:1"]
        pub fn krb5_get_init_creds_opt_free(context: krb5_context,
                                            opt:
                                                *mut krb5_get_init_creds_opt);
        #[no_mangle]
        #[c2rust::src_loc = "6461:1"]
        pub fn krb5_prompter_posix(context: krb5_context,
                                   data: *mut libc::c_void,
                                   name: *const libc::c_char,
                                   banner: *const libc::c_char,
                                   num_prompts: libc::c_int,
                                   prompts: *mut krb5_prompt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6381:1"]
        pub fn krb5_timestamp_to_string(timestamp: krb5_timestamp,
                                        buffer: *mut libc::c_char,
                                        buflen: size_t) -> krb5_error_code;
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
        #[c2rust::src_loc = "7097:1"]
        pub fn krb5_get_init_creds_opt_set_out_ccache(context: krb5_context,
                                                      opt:
                                                          *mut krb5_get_init_creds_opt,
                                                      ccache: krb5_ccache)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:2"]
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
    /* allowable clock skew */
    /* Message size above which we'll try TCP first in send-to-kdc
       type code.  Aside from the 2**16 size limit, we put no
       absolute limit on the UDP packet size.  */
    /* Use the config-file ktypes instead of app-specified?  */
    /* locate_kdc module stuff */
    /* preauth module stuff */
    /* cache module stuff */
    /* localauth module stuff */
    /* hostrealm module stuff */
    /* TLS module vtable (if loaded) */
    /* error detail info */
    /* For Sun iprop code; does this really have to be here?  */
    /* could be used in a table to find an etype and initialize a block */
    /* internal message representations */
    /* user data */
    /* client time, optional */
    /* microsecond portion of time,
                                           optional */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /* data integrity checksum */
    /* encrypted part */
    /* user data */
    /* client time, optional */
    /* microsecond portion of time, opt. */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /*
 * Begin "asn1.h"
 */
    /* ASN.1 encoding knowledge; KEEP IN SYNC WITH ASN.1 defs! */
/* here we use some knowledge of ASN.1 encodings */
/*
  Ticket is APPLICATION 1.
  Authenticator is APPLICATION 2.
  AS_REQ is APPLICATION 10.
  AS_REP is APPLICATION 11.
  TGS_REQ is APPLICATION 12.
  TGS_REP is APPLICATION 13.
  AP_REQ is APPLICATION 14.
  AP_REP is APPLICATION 15.
  KRB_SAFE is APPLICATION 20.
  KRB_PRIV is APPLICATION 21.
  KRB_CRED is APPLICATION 22.
  EncASRepPart is APPLICATION 25.
  EncTGSRepPart is APPLICATION 26.
  EncAPRepPart is APPLICATION 27.
  EncKrbPrivPart is APPLICATION 28.
  EncKrbCredPart is APPLICATION 29.
  KRB_ERROR is APPLICATION 30.
*/
/* allow either constructed or primitive encoding, so check for bit 6
   set or reset */
    /* ************************************************************************
 * Prototypes for krb5_encode.c
 *************************************************************************/
    /*
  krb5_error_code encode_krb5_structure(const krb5_structure *rep,
  krb5_data **code);
  modifies  *code
  effects   Returns the ASN.1 encoding of *rep in **code.
  Returns ASN1_MISSING_FIELD if a required field is emtpy in *rep.
  Returns ENOMEM if memory runs out.
*/
    /* yes, the translation is identical to that used for KDC__REP */
    /* yes, the translation is identical to that used for KDC__REP */
    /* ************************************************************************
 * End of prototypes for krb5_encode.c
 *************************************************************************/
    /* ************************************************************************
 * Prototypes for krb5_decode.c
 *************************************************************************/
/*
  krb5_error_code decode_krb5_structure(const krb5_data *code,
  krb5_structure **rep);

  requires  Expects **rep to not have been allocated;
  a new *rep is allocated regardless of the old value.
  effects   Decodes *code into **rep.
  Returns ENOMEM if memory is exhausted.
  Returns asn1 and krb5 errors.
*/
    /* kdb.h */
    /* Master key version number */
    /* kvno of key_data elements (all the same) */
    /* ************************************************************************
 * End of prototypes for krb5_decode.c
 *************************************************************************/
    /* KRB5_ASN1__ */
    /*
 * End "asn1.h"
 */
    /*
 * Internal krb5 library routines
 */
    /* Return true if s is non-empty and composed solely of digits. */
    /*
 * Initialization routines.
 */
    /* [De]serialize 4-byte integer */
    /* [De]serialize 8-byte integer */
    /* [De]serialize byte string */
    /* Fill in the buffer with random alpha-numeric data. */
    /* value to use when requesting a keytab entry and KVNO doesn't matter */
    /* value to use when requesting a keytab entry and enctype doesn't matter */
    /* To keep happy libraries which are (for now) accessing internal stuff */
    /* Make sure to increment by one when changing the struct */
    /* Used for KDB LDAP back end.  */
    /*
     * pkinit asn.1 encode/decode functions
     */
    /* Set *tag_out to the integrity tag of *enc.  (Does not allocate memory;
 * returned buffer is a subrange of *ctext.) */
    /*
 * This structure was exposed and used in macros in krb5 1.2, so do not
 * change its ABI.
 */
    /* routines always present */
    /* routines to be included on extended version (write routines) */
    /* Not sure it's ready for exposure just yet.  */
    /*
 * Referral definitions and subfunctions.
 */
    /* should move into k5-int.h */
    /* chk_trans.c */
    /* free_rtree.c */
    /* Some data comparison and conversion functions.  */
    /* Allocate at least one byte since zero-byte allocs may return NULL. */
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    /* Allocate at least one byte since zero-byte allocs may return NULL. */
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    /* Return a copy of the len bytes of memory at in; set *code to 0 or ENOMEM. */
    /* Like k5memdup, but add a final null byte. */
    /* Convert a krb5_timestamp to a time_t value, treating the negative range of
 * krb5_timestamp as times between 2038 and 2106 (if time_t is 64-bit). */
    /* Return the delta between two timestamps (a - b) as a signed 32-bit value,
 * without relying on undefined behavior. */
    #[inline]
    #[c2rust::src_loc = "2346:1"]
    pub unsafe extern "C" fn ts_delta(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_deltat {
        return (a as uint32_t).wrapping_sub(b as uint32_t) as krb5_deltat;
    }
    #[inline]
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
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
    #[c2rust::src_loc = "2274:1"]
    pub unsafe extern "C" fn alloc_data(mut data: *mut krb5_data,
                                        mut len: libc::c_uint)
     -> krb5_error_code {
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
    #[inline]
    #[c2rust::src_loc = "2268:1"]
    pub unsafe extern "C" fn string2data(mut str: *mut libc::c_char)
     -> krb5_data {
        return make_data(str as *mut libc::c_void,
                         strlen(str) as libc::c_uint);
    }
    #[inline]
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_timestamp, krb5_context, krb5_kdc_rep, krb5_data,
                        krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    use super::string_h::{explicit_bzero, strlen};
    use super::stdlib_h::{free, calloc};
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
        #[c2rust::src_loc = "2151:1"]
        pub fn krb5_free_kdc_rep(_: krb5_context, _: *mut krb5_kdc_rep);
        #[no_mangle]
        #[c2rust::src_loc = "898:1"]
        pub fn krb5int_copy_data_contents(_: krb5_context,
                                          _: *const krb5_data,
                                          _: *mut krb5_data)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/init_creds_ctx.h:4"]
pub mod init_creds_ctx_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:8"]
    pub struct gak_password {
        pub storage: krb5_data,
        pub password: *const krb5_data,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:8"]
    pub struct krb5_responder_context_st {
        pub items: *mut k5_response_items,
    }
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
    use super::krb5_h::{krb5_data, krb5_get_init_creds_opt, krb5_boolean,
                        krb5_prompter_fct, krb5_timestamp, krb5_deltat,
                        krb5_error, krb5_pa_data, krb5_creds, krb5_kdc_req,
                        krb5_kdc_rep, krb5_keyblock, krb5_enctype,
                        krb5_preauthtype, krb5_int32};
    use super::int_proto_h::{k5_response_items, get_as_key_fn,
                             krb5int_fast_request_state};
    use super::k5_json_h::k5_json_object;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
        #[c2rust::src_loc = "9:16"]
        pub type krb5_preauth_req_context_st;
    }
    /* !KRB5_INIT_CREDS_CONTEXT */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:4"]
pub mod int_proto_h {
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
                        krb5_keyblock, krb5_get_init_creds_opt,
                        krb5_expire_callback_func, krb5_creds,
                        krb5_principal_data, krb5_deltat, krb5_kdc_rep,
                        krb5_flags, krb5_address, krb5_preauthtype};
    use super::k5_int_h::_krb5_context;
    use super::k5_err_h::errinfo;
    extern "C" {
        #[c2rust::src_loc = "32:16"]
        pub type k5_response_items_st;
        #[c2rust::src_loc = "30:8"]
        pub type krb5int_fast_request_state;
        #[no_mangle]
        #[c2rust::src_loc = "277:1"]
        pub fn k5_save_ctx_error(ctx: krb5_context, code: krb5_error_code,
                                 out: *mut errinfo);
        #[no_mangle]
        #[c2rust::src_loc = "281:1"]
        pub fn k5_restore_ctx_error(ctx: krb5_context, in_0: *mut errinfo)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "349:1"]
        pub fn k5_gic_opt_get_expire_cb(opt: *mut krb5_get_init_creds_opt,
                                        cb_out:
                                            *mut krb5_expire_callback_func,
                                        data_out: *mut *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "272:1"]
        pub fn k5_response_items_get_answer(ri: *const k5_response_items,
                                            question: *const libc::c_char)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn k5_response_items_ask_question(ri: *mut k5_response_items,
                                              question: *const libc::c_char,
                                              challenge: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "289:1"]
        pub fn k5_get_init_creds(context: krb5_context,
                                 creds: *mut krb5_creds,
                                 client: krb5_principal,
                                 prompter: krb5_prompter_fct,
                                 prompter_data: *mut libc::c_void,
                                 start_time: krb5_deltat,
                                 in_tkt_service: *const libc::c_char,
                                 options: *mut krb5_get_init_creds_opt,
                                 gak: get_as_key_fn,
                                 gak_data: *mut libc::c_void,
                                 master: *mut libc::c_int,
                                 as_reply: *mut *mut krb5_kdc_rep)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "308:1"]
        pub fn k5_populate_gic_opt(context: krb5_context,
                                   opt: *mut *mut krb5_get_init_creds_opt,
                                   options: krb5_flags,
                                   addrs: *const *mut krb5_address,
                                   ktypes: *mut krb5_enctype,
                                   pre_auth_types: *mut krb5_preauthtype,
                                   creds: *mut krb5_creds) -> krb5_error_code;
        /*
 * Make a shallow copy of opt, with all pointer fields aliased, or NULL on an
 * out-of-memory failure.  The caller must free the result with free, and must
 * not use it with the following functions:
 *
 *     krb5_get_init_creds_opt_free
 *     krb5_get_init_creds_opt_set_pa
 *     krb5_get_init_creds_opt_set_fast_ccache
 *     krb5_get_init_creds_opt_set_fast_ccache_name
 */
        #[no_mangle]
        #[c2rust::src_loc = "367:1"]
        pub fn k5_gic_opt_shallow_copy(opt: *mut krb5_get_init_creds_opt)
         -> *mut krb5_get_init_creds_opt;
    }
    /* KRB5_INT_FUNC_PROTO__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:2"]
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
        /*@shared@*/
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-json.h:4"]
pub mod k5_json_h {
    /*
 * Object
 */
    #[c2rust::src_loc = "160:1"]
    pub type k5_json_object = *mut k5_json_object_st;
    extern "C" {
        #[c2rust::src_loc = "160:16"]
        pub type k5_json_object_st;
    }
    /* K5_JSON_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:2"]
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
#[c2rust::header_src = "/usr/include/stdio.h:2"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:2"]
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
#[c2rust::header_src = "/usr/include/libintl.h:2"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:2"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:2"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:6"]
pub mod os_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_prompt_type};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "179:1"]
        pub fn k5_set_prompt_types(_: krb5_context, _: *mut krb5_prompt_type);
    }
    /* KRB5_LIBOS_INT_PROTO__ */
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_msgtype, krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_authdatatype, krb5_preauthtype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_principal_data,
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
                       _krb5_error, krb5_error, krb5_ccache,
                       krb5_get_init_creds_opt, _krb5_get_init_creds_opt,
                       krb5_expire_callback_func, krb5_prompter_fct,
                       krb5_prompt, _krb5_prompt, krb5_init_creds_context,
                       _profile_t, _krb5_ccache,
                       krb5_c_string_to_key_with_params, krb5_cc_store_cred,
                       krb5_unparse_name, krb5_principal2salt,
                       krb5_free_principal, krb5_free_cred_contents,
                       krb5_free_keyblock_contents, krb5_free_unparsed_name,
                       krb5_timeofday, krb5_change_password,
                       krb5_chpw_message, krb5_get_init_creds_opt_free,
                       krb5_prompter_posix, krb5_timestamp_to_string,
                       krb5_get_init_creds_opt_set_tkt_life,
                       krb5_get_init_creds_opt_set_renew_life,
                       krb5_get_init_creds_opt_set_forwardable,
                       krb5_get_init_creds_opt_set_proxiable,
                       krb5_get_init_creds_opt_set_out_ccache};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, ts_delta, zapfree, make_data,
                         alloc_data, string2data, ts_after, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_free_kdc_rep,
                         krb5int_copy_data_contents};
pub use self::k5_err_h::{errinfo, k5_clear_error};
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::init_creds_ctx_h::{gak_password, krb5_responder_context_st,
                                 _krb5_init_creds_context,
                                 krb5_preauth_req_context, C2RustUnnamed,
                                 AUTH_OFFSET, UNAUTH_OFFSET, NO_OFFSET,
                                 krb5_preauth_req_context_st};
pub use self::int_proto_h::{k5_response_items, get_as_key_fn,
                            k5_response_items_st, krb5int_fast_request_state,
                            k5_save_ctx_error, k5_restore_ctx_error,
                            k5_gic_opt_get_expire_cb,
                            k5_response_items_get_answer,
                            k5_response_items_ask_question, k5_get_init_creds,
                            k5_populate_gic_opt, k5_gic_opt_shallow_copy};
pub use self::com_err_h::{errcode_t, error_message};
pub use self::k5_json_h::{k5_json_object, k5_json_object_st};
use self::stdlib_h::{calloc, free};
use self::stdio_h::snprintf;
use self::k5_platform_h::krb5int_strlcpy;
use self::libintl_h::dgettext;
use self::string_h::{explicit_bzero, strlen, strdup, strcmp, memset, memcpy};
use self::k5_trace_h::krb5int_trace;
use self::os_proto_h::k5_set_prompt_types;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
#[no_mangle]
#[c2rust::src_loc = "8:1"]
pub unsafe extern "C" fn krb5_get_as_key_password(mut context: krb5_context,
                                                  mut client: krb5_principal,
                                                  mut etype: krb5_enctype,
                                                  mut prompter:
                                                      krb5_prompter_fct,
                                                  mut prompter_data:
                                                      *mut libc::c_void,
                                                  mut salt: *mut krb5_data,
                                                  mut params: *mut krb5_data,
                                                  mut as_key:
                                                      *mut krb5_keyblock,
                                                  mut gak_data:
                                                      *mut libc::c_void,
                                                  mut ritems:
                                                      *mut k5_response_items)
 -> krb5_error_code {
    let mut gp: *mut gak_password = gak_data as *mut gak_password;
    let mut ret: krb5_error_code = 0;
    let mut defsalt: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut clientstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut promptstr: [libc::c_char; 1024] = [0; 1024];
    let mut pwbuf: [libc::c_char; 1024] = [0; 1024];
    let mut pw: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut prompt: krb5_prompt =
        krb5_prompt{prompt: 0 as *mut libc::c_char,
                    hidden: 0,
                    reply: 0 as *mut krb5_data,};
    let mut prompt_type: krb5_prompt_type = 0;
    let mut rpass: *const libc::c_char = 0 as *const libc::c_char;
    /* If we need to get the AS key via the responder, ask for it. */
    if as_key.is_null() {
        if !(*gp).password.is_null() { return 0 as libc::c_int }
        return k5_response_items_ask_question(ritems,
                                              b"password\x00" as *const u8 as
                                                  *const libc::c_char,
                                              b"\x00" as *const u8 as
                                                  *const libc::c_char)
    }
    /* If there's already a key of the correct etype, we're done.
       If the etype is wrong, free the existing key, and make
       a new one.

       XXX This was the old behavior, and was wrong in hw preauth
       cases.  Is this new behavior -- always asking -- correct in all
       cases?  */
    if (*as_key).length != 0 {
        if (*as_key).enctype != etype {
            krb5_free_keyblock_contents(context, as_key);
            (*as_key).length = 0 as libc::c_int as libc::c_uint
        }
    }
    if (*gp).password.is_null() {
        /* Check the responder for the password. */
        rpass =
            k5_response_items_get_answer(ritems,
                                         b"password\x00" as *const u8 as
                                             *const libc::c_char);
        if !rpass.is_null() {
            ret =
                alloc_data(&mut (*gp).storage, strlen(rpass) as libc::c_uint);
            if ret != 0 { return ret }
            memcpy((*gp).storage.data as *mut libc::c_void,
                   rpass as *const libc::c_void, strlen(rpass));
            (*gp).password = &mut (*gp).storage
        }
    }
    if (*gp).password.is_null() {
        if prompter.is_none() { return 5 as libc::c_int }
        ret =
            krb5_unparse_name(context, client as krb5_const_principal,
                              &mut clientstr);
        if ret != 0 { return ret }
        snprintf(promptstr.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong,
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Password for %s\x00" as *const u8 as
                              *const libc::c_char), clientstr);
        free(clientstr as *mut libc::c_void);
        pw =
            make_data(pwbuf.as_mut_ptr() as *mut libc::c_void,
                      ::std::mem::size_of::<[libc::c_char; 1024]>() as
                          libc::c_ulong as libc::c_uint);
        prompt.prompt = promptstr.as_mut_ptr();
        prompt.hidden = 1 as libc::c_int;
        prompt.reply = &mut pw;
        prompt_type = 0x1 as libc::c_int;
        /* PROMPTER_INVOCATION */
        k5_set_prompt_types(context, &mut prompt_type);
        ret =
            Some(prompter.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                                   prompter_data,
                                                                                                   0
                                                                                                       as
                                                                                                       *const libc::c_char,
                                                                                                   0
                                                                                                       as
                                                                                                       *const libc::c_char,
                                                                                                   1
                                                                                                       as
                                                                                                       libc::c_int,
                                                                                                   &mut prompt);
        k5_set_prompt_types(context, 0 as *mut krb5_prompt_type);
        if ret != 0 { return ret }
        ret =
            krb5int_copy_data_contents(context, &mut pw, &mut (*gp).storage);
        explicit_bzero(pw.data as *mut libc::c_void, pw.length as size_t);
        if ret != 0 { return ret }
        (*gp).password = &mut (*gp).storage
    }
    if salt.is_null() {
        ret =
            krb5_principal2salt(context, client as krb5_const_principal,
                                &mut defsalt);
        if ret != 0 { return ret }
        salt = &mut defsalt
    } else { defsalt.length = 0 as libc::c_int as libc::c_uint }
    ret =
        krb5_c_string_to_key_with_params(context, etype, (*gp).password, salt,
                                         if !(*params).data.is_null() {
                                             params
                                         } else { 0 as *mut krb5_data },
                                         as_key);
    if defsalt.length != 0 { free(defsalt.data as *mut libc::c_void); }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "117:1"]
pub unsafe extern "C" fn krb5_init_creds_set_password(mut context:
                                                          krb5_context,
                                                      mut ctx:
                                                          krb5_init_creds_context,
                                                      mut password:
                                                          *const libc::c_char)
 -> krb5_error_code {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = strdup(password);
    if s.is_null() { return 12 as libc::c_int }
    zapfree((*ctx).gakpw.storage.data as *mut libc::c_void,
            (*ctx).gakpw.storage.length as size_t);
    (*ctx).gakpw.storage = string2data(s);
    (*ctx).gakpw.password = &mut (*ctx).gakpw.storage;
    (*ctx).gak_fct =
        Some(krb5_get_as_key_password as
                 unsafe extern "C" fn(_: krb5_context, _: krb5_principal,
                                      _: krb5_enctype, _: krb5_prompter_fct,
                                      _: *mut libc::c_void, _: *mut krb5_data,
                                      _: *mut krb5_data,
                                      _: *mut krb5_keyblock,
                                      _: *mut libc::c_void,
                                      _: *mut k5_response_items)
                     -> krb5_error_code);
    (*ctx).gak_data =
        &mut (*ctx).gakpw as *mut gak_password as *mut libc::c_void;
    return 0 as libc::c_int;
}
/* Return the password expiry time indicated by enc_part2.  Set *is_last_req
 * if the information came from a last_req value. */
#[c2rust::src_loc = "138:1"]
unsafe extern "C" fn get_expiry_times(mut enc_part2:
                                          *mut krb5_enc_kdc_rep_part,
                                      mut pw_exp: *mut krb5_timestamp,
                                      mut acct_exp: *mut krb5_timestamp,
                                      mut is_last_req: *mut krb5_boolean) {
    let mut last_req: *mut *mut krb5_last_req_entry =
        0 as *mut *mut krb5_last_req_entry;
    let mut lr_type: krb5_int32 = 0;
    *pw_exp = 0 as libc::c_int;
    *acct_exp = 0 as libc::c_int;
    *is_last_req = 0 as libc::c_int as krb5_boolean;
    /* Look for last-req entries for password or account expiration. */
    if !(*enc_part2).last_req.is_null() {
        last_req = (*enc_part2).last_req;
        while !(*last_req).is_null() {
            lr_type = (**last_req).lr_type;
            if lr_type == 6 as libc::c_int || lr_type == -(6 as libc::c_int) {
                *is_last_req = 1 as libc::c_int as krb5_boolean;
                *pw_exp = (**last_req).value
            } else if lr_type == 7 as libc::c_int ||
                          lr_type == -(7 as libc::c_int) {
                *is_last_req = 1 as libc::c_int as krb5_boolean;
                *acct_exp = (**last_req).value
            }
            last_req = last_req.offset(1)
        }
    }
    /* If we didn't find any, use the ambiguous key_exp field. */
    if *is_last_req == 0 as libc::c_int as libc::c_uint {
        *pw_exp = (*enc_part2).key_exp
    };
}
/*
 * Send an appropriate warning prompter if as_reply indicates that the password
 * is going to expire soon.  If an expire callback was provided, use that
 * instead.
 */
#[c2rust::src_loc = "175:1"]
unsafe extern "C" fn warn_pw_expiry(mut context: krb5_context,
                                    mut options: *mut krb5_get_init_creds_opt,
                                    mut prompter: krb5_prompter_fct,
                                    mut data: *mut libc::c_void,
                                    mut in_tkt_service: *const libc::c_char,
                                    mut as_reply: *mut krb5_kdc_rep) {
    let mut ret: krb5_error_code = 0;
    let mut expire_cb: krb5_expire_callback_func = None;
    let mut expire_data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut pw_exp: krb5_timestamp = 0;
    let mut acct_exp: krb5_timestamp = 0;
    let mut now: krb5_timestamp = 0;
    let mut is_last_req: krb5_boolean = 0;
    let mut delta: krb5_deltat = 0;
    let mut ts: [libc::c_char; 256] = [0; 256];
    let mut banner: [libc::c_char; 1024] = [0; 1024];
    get_expiry_times((*as_reply).enc_part2, &mut pw_exp, &mut acct_exp,
                     &mut is_last_req);
    k5_gic_opt_get_expire_cb(options, &mut expire_cb, &mut expire_data);
    if expire_cb.is_some() {
        /* Invoke the expire callback and don't send prompter warnings. */
        Some(expire_cb.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                                expire_data,
                                                                                                pw_exp,
                                                                                                acct_exp,
                                                                                                is_last_req);
        return
    }
    /* Don't warn if no password expiry value was sent. */
    if pw_exp == 0 as libc::c_int { return }
    /* Don't warn if the password is being changed. */
    if !in_tkt_service.is_null() &&
           strcmp(in_tkt_service,
                  b"kadmin/changepw\x00" as *const u8 as *const libc::c_char)
               == 0 as libc::c_int {
        return
    }
    /*
     * If the expiry time came from a last_req field, assume the KDC wants us
     * to warn.  Otherwise, warn only if the expiry time is less than a week
     * from now.
     */
    ret = krb5_timeofday(context, &mut now);
    if ret != 0 as libc::c_int { return }
    if is_last_req == 0 &&
           (ts_after(now, pw_exp) != 0 ||
                ts_delta(pw_exp, now) >
                    7 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int *
                        60 as libc::c_int) {
        return
    }
    if prompter.is_none() { return }
    ret =
        krb5_timestamp_to_string(pw_exp, ts.as_mut_ptr(),
                                 ::std::mem::size_of::<[libc::c_char; 256]>()
                                     as libc::c_ulong);
    if ret != 0 as libc::c_int { return }
    delta = ts_delta(pw_exp, now);
    if delta < 3600 as libc::c_int {
        snprintf(banner.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong,
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Warning: Your password will expire in less than one hour on %s\x00"
                              as *const u8 as *const libc::c_char),
                 ts.as_mut_ptr());
    } else if delta < 86400 as libc::c_int * 2 as libc::c_int {
        snprintf(banner.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong,
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Warning: Your password will expire in %d hour%s on %s\x00"
                              as *const u8 as *const libc::c_char),
                 delta / 3600 as libc::c_int,
                 if delta < 7200 as libc::c_int {
                     b"\x00" as *const u8 as *const libc::c_char
                 } else { b"s\x00" as *const u8 as *const libc::c_char },
                 ts.as_mut_ptr());
    } else {
        snprintf(banner.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong,
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Warning: Your password will expire in %d days on %s\x00"
                              as *const u8 as *const libc::c_char),
                 delta / 86400 as libc::c_int, ts.as_mut_ptr());
    }
    /* PROMPTER_INVOCATION */
    Some(prompter.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                           data,
                                                                                           0
                                                                                               as
                                                                                               *const libc::c_char,
                                                                                           banner.as_mut_ptr(),
                                                                                           0
                                                                                               as
                                                                                               libc::c_int,
                                                                                           0
                                                                                               as
                                                                                               *mut krb5_prompt);
}
/*
 * Create a temporary options structure for getting a kadmin/changepw ticket,
 * based on the appplication-specified options.  Propagate all application
 * options which affect preauthentication, but not options which affect the
 * resulting ticket or how it is stored.  Set lifetime and flags appropriate
 * for a ticket which we will use immediately and then discard.
 *
 * The caller should free the result with free().
 */
#[c2rust::src_loc = "252:1"]
unsafe extern "C" fn make_chpw_options(mut context: krb5_context,
                                       mut in_0: *mut krb5_get_init_creds_opt,
                                       mut out:
                                           *mut *mut krb5_get_init_creds_opt)
 -> krb5_error_code {
    let mut opt: *mut krb5_get_init_creds_opt =
        0 as *mut krb5_get_init_creds_opt;
    *out = 0 as *mut krb5_get_init_creds_opt;
    opt = k5_gic_opt_shallow_copy(in_0);
    if opt.is_null() { return 12 as libc::c_int }
    /* Get a non-forwardable, non-proxiable, short-lifetime ticket. */
    krb5_get_init_creds_opt_set_tkt_life(opt,
                                         5 as libc::c_int *
                                             60 as libc::c_int);
    krb5_get_init_creds_opt_set_renew_life(opt, 0 as libc::c_int);
    krb5_get_init_creds_opt_set_forwardable(opt, 0 as libc::c_int);
    krb5_get_init_creds_opt_set_proxiable(opt, 0 as libc::c_int);
    /* Unset options which should only apply to the actual ticket. */
    (*opt).flags &= !(0x20 as libc::c_int);
    (*opt).flags &= !(0x400 as libc::c_int);
    /* The output ccache should only be used for the actual ticket. */
    krb5_get_init_creds_opt_set_out_ccache(context, opt, 0 as krb5_ccache);
    *out = opt;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "280:1"]
pub unsafe extern "C" fn krb5_get_init_creds_password(mut context:
                                                          krb5_context,
                                                      mut creds:
                                                          *mut krb5_creds,
                                                      mut client:
                                                          krb5_principal,
                                                      mut password:
                                                          *const libc::c_char,
                                                      mut prompter:
                                                          krb5_prompter_fct,
                                                      mut data:
                                                          *mut libc::c_void,
                                                      mut start_time:
                                                          krb5_deltat,
                                                      mut in_tkt_service:
                                                          *const libc::c_char,
                                                      mut options:
                                                          *mut krb5_get_init_creds_opt)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut use_master: libc::c_int = 0;
    let mut as_reply: *mut krb5_kdc_rep = 0 as *mut krb5_kdc_rep;
    let mut tries: libc::c_int = 0;
    let mut chpw_creds: krb5_creds =
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
    let mut chpw_opts: *mut krb5_get_init_creds_opt =
        0 as *mut krb5_get_init_creds_opt;
    let mut gakpw: gak_password =
        gak_password{storage:
                         krb5_data{magic: 0,
                                   length: 0,
                                   data: 0 as *mut libc::c_char,},
                     password: 0 as *const krb5_data,};
    let mut pw0: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut pw1: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut banner: [libc::c_char; 1024] = [0; 1024];
    let mut pw0array: [libc::c_char; 1024] = [0; 1024];
    let mut pw1array: [libc::c_char; 1024] = [0; 1024];
    let mut prompt: [krb5_prompt; 2] =
        [krb5_prompt{prompt: 0 as *mut libc::c_char,
                     hidden: 0,
                     reply: 0 as *mut krb5_data,}; 2];
    let mut prompt_types: [krb5_prompt_type; 2] = [0; 2];
    let mut errsave: errinfo =
        {
            let mut init =
                errinfo{code: 0 as libc::c_int as libc::c_long,
                        msg: 0 as *mut libc::c_char,};
            init
        };
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    use_master = 0 as libc::c_int;
    as_reply = 0 as *mut krb5_kdc_rep;
    memset(&mut chpw_creds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    memset(&mut gakpw as *mut gak_password as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<gak_password>() as libc::c_ulong);
    if !password.is_null() {
        pw0 = string2data(password as *mut libc::c_char);
        gakpw.password = &mut pw0
    }
    /* first try: get the requested tkt from any kdc */
    ret =
        k5_get_init_creds(context, creds, client, prompter, data, start_time,
                          in_tkt_service, options,
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
                                                        _:
                                                            *mut k5_response_items)
                                       -> krb5_error_code),
                          &mut gakpw as *mut gak_password as
                              *mut libc::c_void, &mut use_master,
                          &mut as_reply);
    /* check for success */
    if !(ret == 0 as libc::c_int) {
        /* If all the kdc's are unavailable, or if the error was due to a
       user interrupt, fail */
        if !(ret as libc::c_long == -(1765328228 as libc::c_long) ||
                 ret as libc::c_long == -(1765328164 as libc::c_long) ||
                 ret as libc::c_long == -(1765328252 as libc::c_long) ||
                 ret as libc::c_long == -(1765328254 as libc::c_long)) {
            /* if the reply did not come from the master kdc, try again with
       the master kdc */
            if use_master == 0 {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"Retrying AS request with master KDC\x00"
                                      as *const u8 as *const libc::c_char);
                }
                use_master = 1 as libc::c_int;
                k5_save_ctx_error(context, ret, &mut errsave);
                if !as_reply.is_null() {
                    krb5_free_kdc_rep(context, as_reply);
                    as_reply = 0 as *mut krb5_kdc_rep
                }
                ret =
                    k5_get_init_creds(context, creds, client, prompter, data,
                                      start_time, in_tkt_service, options,
                                      Some(krb5_get_as_key_password as
                                               unsafe extern "C" fn(_:
                                                                        krb5_context,
                                                                    _:
                                                                        krb5_principal,
                                                                    _:
                                                                        krb5_enctype,
                                                                    _:
                                                                        krb5_prompter_fct,
                                                                    _:
                                                                        *mut libc::c_void,
                                                                    _:
                                                                        *mut krb5_data,
                                                                    _:
                                                                        *mut krb5_data,
                                                                    _:
                                                                        *mut krb5_keyblock,
                                                                    _:
                                                                        *mut libc::c_void,
                                                                    _:
                                                                        *mut k5_response_items)
                                                   -> krb5_error_code),
                                      &mut gakpw as *mut gak_password as
                                          *mut libc::c_void, &mut use_master,
                                      &mut as_reply);
                if ret == 0 as libc::c_int {
                    current_block = 2302882124141812546;
                } else {
                    /* If the master is unreachable, return the error from the replica we
         * were able to contact and reset the use_master flag. */
                    if ret as libc::c_long == -(1765328228 as libc::c_long) ||
                           ret as libc::c_long ==
                               -(1765328164 as libc::c_long) ||
                           ret as libc::c_long ==
                               -(1765328230 as libc::c_long) {
                        ret = k5_restore_ctx_error(context, &mut errsave);
                        use_master = 0 as libc::c_int
                    }
                    current_block = 14434620278749266018;
                }
            } else { current_block = 14434620278749266018; }
            match current_block {
                2302882124141812546 => { }
                _ =>
                /* at this point, we have an error from the master.  if the error
       is not password expired, or if it is but there's no prompter,
       return this error */
                {
                    if !(ret as libc::c_long != -(1765328361 as libc::c_long)
                             || prompter.is_none()) {
                        /* historically the default has been to prompt for password change.
     * if the change password prompt option has not been set, we continue
     * to prompt.  Prompting is only disabled if the option has been set
     * and the value has been set to false.
     */
                        if !(!options.is_null() &&
                                 (*options).flags & 0x100 as libc::c_int == 0)
                           {
                            if (*context).trace_callback.is_some() {
                                krb5int_trace(context,
                                              b"Principal expired; getting changepw ticket\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                            }
                            /* ok, we have an expired password.  Give the user a few chances
       to change it */
                            ret =
                                make_chpw_options(context, options,
                                                  &mut chpw_opts);
                            if !(ret != 0) {
                                ret =
                                    k5_get_init_creds(context,
                                                      &mut chpw_creds, client,
                                                      prompter, data,
                                                      start_time,
                                                      b"kadmin/changepw\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      chpw_opts,
                                                      Some(krb5_get_as_key_password
                                                               as
                                                               unsafe extern "C" fn(_:
                                                                                        krb5_context,
                                                                                    _:
                                                                                        krb5_principal,
                                                                                    _:
                                                                                        krb5_enctype,
                                                                                    _:
                                                                                        krb5_prompter_fct,
                                                                                    _:
                                                                                        *mut libc::c_void,
                                                                                    _:
                                                                                        *mut krb5_data,
                                                                                    _:
                                                                                        *mut krb5_data,
                                                                                    _:
                                                                                        *mut krb5_keyblock,
                                                                                    _:
                                                                                        *mut libc::c_void,
                                                                                    _:
                                                                                        *mut k5_response_items)
                                                                   ->
                                                                       krb5_error_code),
                                                      &mut gakpw as
                                                          *mut gak_password as
                                                          *mut libc::c_void,
                                                      &mut use_master,
                                                      0 as
                                                          *mut *mut krb5_kdc_rep);
                                if !(ret != 0) {
                                    pw0.data = pw0array.as_mut_ptr();
                                    *pw0.data.offset(0 as libc::c_int as
                                                         isize) =
                                        '\u{0}' as i32 as libc::c_char;
                                    pw0.length =
                                        ::std::mem::size_of::<[libc::c_char; 1024]>()
                                            as libc::c_ulong as libc::c_uint;
                                    prompt[0 as libc::c_int as usize].prompt =
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"Enter new password\x00" as
                                                     *const u8 as
                                                     *const libc::c_char);
                                    prompt[0 as libc::c_int as usize].hidden =
                                        1 as libc::c_int;
                                    prompt[0 as libc::c_int as usize].reply =
                                        &mut pw0;
                                    prompt_types[0 as libc::c_int as usize] =
                                        0x2 as libc::c_int;
                                    pw1.data = pw1array.as_mut_ptr();
                                    *pw1.data.offset(0 as libc::c_int as
                                                         isize) =
                                        '\u{0}' as i32 as libc::c_char;
                                    pw1.length =
                                        ::std::mem::size_of::<[libc::c_char; 1024]>()
                                            as libc::c_ulong as libc::c_uint;
                                    prompt[1 as libc::c_int as usize].prompt =
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"Enter it again\x00" as
                                                     *const u8 as
                                                     *const libc::c_char);
                                    prompt[1 as libc::c_int as usize].hidden =
                                        1 as libc::c_int;
                                    prompt[1 as libc::c_int as usize].reply =
                                        &mut pw1;
                                    prompt_types[1 as libc::c_int as usize] =
                                        0x3 as libc::c_int;
                                    krb5int_strlcpy(banner.as_mut_ptr(),
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"Password expired.  You must change it now.\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char),
                                                    ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                        as libc::c_ulong);
                                    tries = 3 as libc::c_int;
                                    loop  {
                                        if !(tries != 0) {
                                            current_block =
                                                8723848109087415604;
                                            break ;
                                        }
                                        if (*context).trace_callback.is_some()
                                           {
                                            krb5int_trace(context,
                                                          b"Attempting password change; {int} tries remaining\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          tries);
                                        }
                                        pw0.length =
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong as
                                                libc::c_uint;
                                        pw1.length =
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong as
                                                libc::c_uint;
                                        /* PROMPTER_INVOCATION */
                                        k5_set_prompt_types(context,
                                                            prompt_types.as_mut_ptr());
                                        ret =
                                            Some(prompter.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                                                                   data,
                                                                                                                                   0
                                                                                                                                       as
                                                                                                                                       *const libc::c_char,
                                                                                                                                   banner.as_mut_ptr(),
                                                                                                                                   (::std::mem::size_of::<[krb5_prompt; 2]>()
                                                                                                                                        as
                                                                                                                                        libc::c_ulong).wrapping_div(::std::mem::size_of::<krb5_prompt>()
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_ulong)
                                                                                                                                       as
                                                                                                                                       libc::c_int,
                                                                                                                                   prompt.as_mut_ptr());
                                        k5_set_prompt_types(context,
                                                            0 as
                                                                *mut krb5_prompt_type);
                                        if ret != 0 {
                                            current_block =
                                                2302882124141812546;
                                            break ;
                                        }
                                        if strcmp(pw0.data, pw1.data) !=
                                               0 as libc::c_int {
                                            ret =
                                                -(1765328253 as libc::c_long)
                                                    as krb5_error_code;
                                            snprintf(banner.as_mut_ptr(),
                                                     ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                         as libc::c_ulong,
                                                     dgettext(b"mit-krb5\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              b"%s.  Please try again.\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char),
                                                     error_message(ret as
                                                                       errcode_t));
                                        } else if pw0.length ==
                                                      0 as libc::c_int as
                                                          libc::c_uint {
                                            ret =
                                                -(1765328151 as libc::c_long)
                                                    as krb5_error_code;
                                            snprintf(banner.as_mut_ptr(),
                                                     ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                         as libc::c_ulong,
                                                     dgettext(b"mit-krb5\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              b"%s.  Please try again.\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char),
                                                     error_message(ret as
                                                                       errcode_t));
                                        } else {
                                            let mut result_code: libc::c_int =
                                                0;
                                            let mut code_string: krb5_data =
                                                krb5_data{magic: 0,
                                                          length: 0,
                                                          data:
                                                              0 as
                                                                  *mut libc::c_char,};
                                            let mut result_string: krb5_data =
                                                krb5_data{magic: 0,
                                                          length: 0,
                                                          data:
                                                              0 as
                                                                  *mut libc::c_char,};
                                            ret =
                                                krb5_change_password(context,
                                                                     &mut chpw_creds,
                                                                     pw0array.as_mut_ptr(),
                                                                     &mut result_code,
                                                                     &mut code_string,
                                                                     &mut result_string);
                                            if ret != 0 {
                                                current_block =
                                                    2302882124141812546;
                                                break ;
                                            }
                                            /* the change succeeded.  go on */
                                            if result_code == 0 as libc::c_int
                                               {
                                                free(code_string.data as
                                                         *mut libc::c_void);
                                                free(result_string.data as
                                                         *mut libc::c_void);
                                                current_block =
                                                    8723848109087415604;
                                                break ;
                                            } else {
                                                /* set this in case the retry loop falls through */
                                                ret =
                                                    -(1765328150 as
                                                          libc::c_long) as
                                                        krb5_error_code;
                                                if result_code !=
                                                       4 as libc::c_int {
                                                    free(code_string.data as
                                                             *mut libc::c_void);
                                                    free(result_string.data as
                                                             *mut libc::c_void);
                                                    current_block =
                                                        2302882124141812546;
                                                    break ;
                                                } else {
                                                    /* the error was soft, so try again */
                                                    if krb5_chpw_message(context,
                                                                         &mut result_string,
                                                                         &mut message)
                                                           != 0 as libc::c_int
                                                       {
                                                        message =
                                                            0 as
                                                                *mut libc::c_char
                                                    }
                                                    /* 100 is I happen to know that no code_string will be longer
               than 100 chars */
                                                    if !message.is_null() &&
                                                           strlen(message) >
                                                               (::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                    as
                                                                    libc::c_ulong).wrapping_sub(100
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong)
                                                       {
                                                        *message.offset((::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                             as
                                                                             libc::c_ulong).wrapping_sub(100
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_ulong)
                                                                            as
                                                                            isize)
                                                            =
                                                            '\u{0}' as i32 as
                                                                libc::c_char
                                                    }
                                                    snprintf(banner.as_mut_ptr(),
                                                             ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                                 as
                                                                 libc::c_ulong,
                                                             dgettext(b"mit-krb5\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      b"%.*s%s%s.  Please try again.\n\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char),
                                                             code_string.length
                                                                 as
                                                                 libc::c_int,
                                                             code_string.data,
                                                             if !message.is_null()
                                                                {
                                                                 b": \x00" as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             } else {
                                                                 b"\x00" as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             },
                                                             if !message.is_null()
                                                                {
                                                                 message as
                                                                     *const libc::c_char
                                                             } else {
                                                                 b"\x00" as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                             });
                                                    free(message as
                                                             *mut libc::c_void);
                                                    free(code_string.data as
                                                             *mut libc::c_void);
                                                    free(result_string.data as
                                                             *mut libc::c_void);
                                                }
                                            }
                                        }
                                        tries -= 1
                                    }
                                    match current_block {
                                        2302882124141812546 => { }
                                        _ => {
                                            if !(ret != 0) {
                                                /* the password change was successful.  Get an initial ticket
       from the master.  this is the last try.  the return from this
       is final.  */
                                                if (*context).trace_callback.is_some()
                                                   {
                                                    krb5int_trace(context,
                                                                  b"Getting initial TGT with changed password\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char);
                                                }
                                                gakpw.password = &mut pw0;
                                                ret =
                                                    k5_get_init_creds(context,
                                                                      creds,
                                                                      client,
                                                                      prompter,
                                                                      data,
                                                                      start_time,
                                                                      in_tkt_service,
                                                                      options,
                                                                      Some(krb5_get_as_key_password
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        krb5_context,
                                                                                                    _:
                                                                                                        krb5_principal,
                                                                                                    _:
                                                                                                        krb5_enctype,
                                                                                                    _:
                                                                                                        krb5_prompter_fct,
                                                                                                    _:
                                                                                                        *mut libc::c_void,
                                                                                                    _:
                                                                                                        *mut krb5_data,
                                                                                                    _:
                                                                                                        *mut krb5_data,
                                                                                                    _:
                                                                                                        *mut krb5_keyblock,
                                                                                                    _:
                                                                                                        *mut libc::c_void,
                                                                                                    _:
                                                                                                        *mut k5_response_items)
                                                                                   ->
                                                                                       krb5_error_code),
                                                                      &mut gakpw
                                                                          as
                                                                          *mut gak_password
                                                                          as
                                                                          *mut libc::c_void,
                                                                      &mut use_master,
                                                                      &mut as_reply);
                                                (ret) != 0;
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
    if ret == 0 as libc::c_int {
        warn_pw_expiry(context, options, prompter, data, in_tkt_service,
                       as_reply);
    }
    free(chpw_opts as *mut libc::c_void);
    zapfree(gakpw.storage.data as *mut libc::c_void,
            gakpw.storage.length as size_t);
    memset(pw0array.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong);
    memset(pw1array.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong);
    krb5_free_cred_contents(context, &mut chpw_creds);
    if !as_reply.is_null() { krb5_free_kdc_rep(context, as_reply); }
    k5_clear_error(&mut errsave);
    return ret;
}
/*
  Rewrites get_in_tkt in terms of newer get_init_creds API.
  Attempts to get an initial ticket for creds->client to use server
  creds->server, (realm is taken from creds->client), with options
  options, and using creds->times.starttime, creds->times.endtime,
  creds->times.renew_till as from, till, and rtime.
  creds->times.renew_till is ignored unless the RENEWABLE option is requested.

  If addrs is non-NULL, it is used for the addresses requested.  If it is
  null, the system standard addresses are used.

  If password is non-NULL, it is converted using the cryptosystem entry
  point for a string conversion routine, seeded with the client's name.
  If password is passed as NULL, the password is read from the terminal,
  and then converted into a key.

  A succesful call will place the ticket in the credentials cache ccache.

  returns system errors, encryption errors
*/
#[no_mangle]
#[c2rust::src_loc = "534:1"]
pub unsafe extern "C" fn krb5_get_in_tkt_with_password(mut context:
                                                           krb5_context,
                                                       mut options:
                                                           krb5_flags,
                                                       mut addrs:
                                                           *const *mut krb5_address,
                                                       mut ktypes:
                                                           *mut krb5_enctype,
                                                       mut pre_auth_types:
                                                           *mut krb5_preauthtype,
                                                       mut password:
                                                           *const libc::c_char,
                                                       mut ccache:
                                                           krb5_ccache,
                                                       mut creds:
                                                           *mut krb5_creds,
                                                       mut ret_as_reply:
                                                           *mut *mut krb5_kdc_rep)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut gakpw: gak_password =
        gak_password{storage:
                         krb5_data{magic: 0,
                                   length: 0,
                                   data: 0 as *mut libc::c_char,},
                     password: 0 as *const krb5_data,};
    let mut pw: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut server: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut server_princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut client_princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut use_master: libc::c_int = 0 as libc::c_int;
    let mut opts: *mut krb5_get_init_creds_opt =
        0 as *mut krb5_get_init_creds_opt;
    memset(&mut gakpw as *mut gak_password as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<gak_password>() as libc::c_ulong);
    if !password.is_null() {
        pw = string2data(password as *mut libc::c_char);
        gakpw.password = &mut pw
    }
    retval =
        k5_populate_gic_opt(context, &mut opts, options, addrs, ktypes,
                            pre_auth_types, creds);
    if retval != 0 { return retval }
    retval =
        krb5_unparse_name(context, (*creds).server as krb5_const_principal,
                          &mut server);
    if retval != 0 {
        krb5_get_init_creds_opt_free(context, opts);
        return retval
    }
    server_princ = (*creds).server;
    client_princ = (*creds).client;
    retval =
        k5_get_init_creds(context, creds, (*creds).client,
                          Some(krb5_prompter_posix as
                                   unsafe extern "C" fn(_: krb5_context,
                                                        _: *mut libc::c_void,
                                                        _:
                                                            *const libc::c_char,
                                                        _:
                                                            *const libc::c_char,
                                                        _: libc::c_int,
                                                        _: *mut krb5_prompt)
                                       -> krb5_error_code),
                          0 as *mut libc::c_void, 0 as libc::c_int, server,
                          opts,
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
                                                        _:
                                                            *mut k5_response_items)
                                       -> krb5_error_code),
                          &mut gakpw as *mut gak_password as
                              *mut libc::c_void, &mut use_master,
                          ret_as_reply);
    krb5_free_unparsed_name(context, server);
    krb5_get_init_creds_opt_free(context, opts);
    zapfree(gakpw.storage.data as *mut libc::c_void,
            gakpw.storage.length as size_t);
    if retval != 0 { return retval }
    krb5_free_principal(context, (*creds).server);
    krb5_free_principal(context, (*creds).client);
    (*creds).client = client_princ;
    (*creds).server = server_princ;
    /* store it in the ccache! */
    if !ccache.is_null() {
        retval = krb5_cc_store_cred(context, ccache, creds);
        if retval != 0 { return retval }
    }
    return retval;
}
