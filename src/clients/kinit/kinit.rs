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
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:28"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:28"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:28"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:28"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:28"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:28"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/getopt_ext.h:28"]
pub mod getopt_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct option {
        pub name: *const libc::c_char,
        pub has_arg: libc::c_int,
        pub flag: *mut libc::c_int,
        pub val: libc::c_int,
    }
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn getopt_long(___argc: libc::c_int,
                           ___argv: *const *mut libc::c_char,
                           __shortopts: *const libc::c_char,
                           __longopts: *const option,
                           __longind: *mut libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:28"]
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
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    /* KRB5_OLD_CRYPTO */
    /*
 * end "encryption.h"
 */
    /*
 * begin "fieldbits.h"
 */
    /* kdc_options for kdc_request */
/* options is 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      KDC_OPT_RESERVED        0x80000000 */
    /* #define      KDC_OPT_UNUSED          0x01000000 */
    /* #define      KDC_OPT_UNUSED          0x00400000 */
/* #define      KDC_OPT_RESERVED        0x00200000 */
/* #define      KDC_OPT_RESERVED        0x00100000 */
/* #define      KDC_OPT_RESERVED        0x00080000 */
/* #define      KDC_OPT_RESERVED        0x00040000 */
    /* #define      KDC_OPT_RESERVED        0x00004000 */
/* #define      KDC_OPT_RESERVED        0x00002000 */
/* #define      KDC_OPT_RESERVED        0x00001000 */
/* #define      KDC_OPT_RESERVED        0x00000800 */
/* #define      KDC_OPT_RESERVED        0x00000400 */
/* #define      KDC_OPT_RESERVED        0x00000200 */
/* #define      KDC_OPT_RESERVED        0x00000100 */
/* #define      KDC_OPT_RESERVED        0x00000080 */
/* #define      KDC_OPT_RESERVED        0x00000040 */
    /* #define      KDC_OPT_UNUSED          0x00000004 */
    /*
 * Mask of ticket flags in the TGT which should be converted into KDC
 * options when using the TGT to get derivitive tickets.
 *
 *  New mask = KDC_OPT_FORWARDABLE | KDC_OPT_PROXIABLE |
 *             KDC_OPT_ALLOW_POSTDATE | KDC_OPT_RENEWABLE
 */
    /* definitions for ap_options fields */
    /* * @defgroup AP_OPTS AP_OPTS
 *
 * ap_options are 32 bits; each host is responsible to put the 4 bytes
 * representing these bits into net order before transmission
 * @{
 */
    /* *< Use session key */
    /* *< Perform a mutual
                                                 authentication exchange */
    /* *< Generate a subsession key
                                                 from the current session key
                                                 obtained from the
                                                 credentials */
    /* #define      AP_OPTS_RESERVED        0x10000000 */
/* #define      AP_OPTS_RESERVED        0x08000000 */
/* #define      AP_OPTS_RESERVED        0x04000000 */
/* #define      AP_OPTS_RESERVED        0x02000000 */
/* #define      AP_OPTS_RESERVED        0x01000000 */
/* #define      AP_OPTS_RESERVED        0x00800000 */
/* #define      AP_OPTS_RESERVED        0x00400000 */
/* #define      AP_OPTS_RESERVED        0x00200000 */
/* #define      AP_OPTS_RESERVED        0x00100000 */
/* #define      AP_OPTS_RESERVED        0x00080000 */
/* #define      AP_OPTS_RESERVED        0x00040000 */
/* #define      AP_OPTS_RESERVED        0x00020000 */
/* #define      AP_OPTS_RESERVED        0x00010000 */
/* #define      AP_OPTS_RESERVED        0x00008000 */
/* #define      AP_OPTS_RESERVED        0x00004000 */
/* #define      AP_OPTS_RESERVED        0x00002000 */
/* #define      AP_OPTS_RESERVED        0x00001000 */
/* #define      AP_OPTS_RESERVED        0x00000800 */
/* #define      AP_OPTS_RESERVED        0x00000400 */
/* #define      AP_OPTS_RESERVED        0x00000200 */
/* #define      AP_OPTS_RESERVED        0x00000100 */
/* #define      AP_OPTS_RESERVED        0x00000080 */
/* #define      AP_OPTS_RESERVED        0x00000040 */
/* #define      AP_OPTS_RESERVED        0x00000020 */
/* #define      AP_OPTS_RESERVED        0x00000010 */
/* #define      AP_OPTS_RESERVED        0x00000008 */
/* #define      AP_OPTS_RESERVED        0x00000004 */
    /* * @} */
    /* end of AP_OPTS group */
    /* definitions for ad_type fields. */
    /* Ticket flags */
/* flags are 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      TKT_FLG_RESERVED        0x80000000 */
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
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
    /* * Credentials structure including ticket, session key, and lifetime info. */
    #[c2rust::src_loc = "2013:1"]
    pub type krb5_creds = _krb5_creds;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "7004:16"]
    pub struct _krb5_gic_opt_pa_data {
        pub attr: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    #[c2rust::src_loc = "7004:1"]
    pub type krb5_gic_opt_pa_data = _krb5_gic_opt_pa_data;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
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
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
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
        pub fn krb5_cc_get_name(context: krb5_context, cache: krb5_ccache)
         -> *const libc::c_char;
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
        pub fn krb5_cc_close(context: krb5_context, cache: krb5_ccache)
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
        pub fn krb5_cc_get_principal(context: krb5_context,
                                     cache: krb5_ccache,
                                     principal: *mut krb5_principal)
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
        pub fn krb5_cc_get_type(context: krb5_context, cache: krb5_ccache)
         -> *const libc::c_char;
        /* *
 * Create a new credential cache of the specified type with a unique name.
 *
 * @param [in]  context         Library context
 * @param [in]  type            Credential cache type name
 * @param [in]  hint            Unused
 * @param [out] id              Credential cache handle
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2697:1"]
        pub fn krb5_cc_new_unique(context: krb5_context,
                                  type_0: *const libc::c_char,
                                  hint: *const libc::c_char,
                                  id: *mut krb5_ccache) -> krb5_error_code;
        /* *
 * Close a key table handle.
 *
 * @param [in] context          Library context
 * @param [in] keytab           Key table handle
 *
 * @retval 0
 */
        #[no_mangle]
        #[c2rust::src_loc = "2782:1"]
        pub fn krb5_kt_close(context: krb5_context, keytab: krb5_keytab)
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
        #[no_mangle]
        #[c2rust::src_loc = "3468:1"]
        pub fn krb5_parse_name_flags(context: krb5_context,
                                     name: *const libc::c_char,
                                     flags: libc::c_int,
                                     principal_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3995:1"]
        pub fn krb5_build_principal_ext(context: krb5_context,
                                        princ: *mut krb5_principal,
                                        rlen: libc::c_uint,
                                        realm: *const libc::c_char, _: ...)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4173:1"]
        pub fn krb5_kt_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               ktid: *mut krb5_keytab) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4244:1"]
        pub fn krb5_kt_client_default(context: krb5_context,
                                      keytab_out: *mut krb5_keytab)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4341:1"]
        pub fn krb5_cc_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               cache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4425:1"]
        pub fn krb5_cc_default(context: krb5_context,
                               ccache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4513:1"]
        pub fn krb5_cc_switch(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4527:1"]
        pub fn krb5_cc_support_switch(context: krb5_context,
                                      type_0: *const libc::c_char)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "4547:1"]
        pub fn krb5_cc_cache_match(context: krb5_context,
                                   client: krb5_principal,
                                   cache_out: *mut krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4677:1"]
        pub fn krb5_free_cred_contents(context: krb5_context,
                                       val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "4868:1"]
        pub fn krb5_os_localaddr(context: krb5_context,
                                 addr: *mut *mut *mut krb5_address)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4887:1"]
        pub fn krb5_get_default_realm(context: krb5_context,
                                      lrealm: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4912:1"]
        pub fn krb5_free_default_realm(context: krb5_context,
                                       lrealm: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "4961:1"]
        pub fn krb5_sname_to_principal(context: krb5_context,
                                       hostname: *const libc::c_char,
                                       sname: *const libc::c_char,
                                       type_0: krb5_int32,
                                       ret_princ: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6299:1"]
        pub fn krb5_string_to_timestamp(string: *mut libc::c_char,
                                        timestampp: *mut krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6310:1"]
        pub fn krb5_string_to_deltat(string: *mut libc::c_char,
                                     deltatp: *mut krb5_deltat)
         -> krb5_error_code;
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
        #[c2rust::src_loc = "6932:1"]
        pub fn krb5_get_init_creds_opt_set_anonymous(opt:
                                                         *mut krb5_get_init_creds_opt,
                                                     anonymous: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "6954:1"]
        pub fn krb5_get_init_creds_opt_set_address_list(opt:
                                                            *mut krb5_get_init_creds_opt,
                                                        addresses:
                                                            *mut *mut krb5_address);
        #[no_mangle]
        #[c2rust::src_loc = "7021:1"]
        pub fn krb5_get_init_creds_opt_set_pa(context: krb5_context,
                                              opt:
                                                  *mut krb5_get_init_creds_opt,
                                              attr: *const libc::c_char,
                                              value: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7041:1"]
        pub fn krb5_get_init_creds_opt_set_fast_ccache_name(context:
                                                                krb5_context,
                                                            opt:
                                                                *mut krb5_get_init_creds_opt,
                                                            fast_ccache_name:
                                                                *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7079:1"]
        pub fn krb5_get_init_creds_opt_set_in_ccache(context: krb5_context,
                                                     opt:
                                                         *mut krb5_get_init_creds_opt,
                                                     ccache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7097:1"]
        pub fn krb5_get_init_creds_opt_set_out_ccache(context: krb5_context,
                                                      opt:
                                                          *mut krb5_get_init_creds_opt,
                                                      ccache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7117:1"]
        pub fn krb5_get_init_creds_opt_set_pac_request(context: krb5_context,
                                                       opt:
                                                           *mut krb5_get_init_creds_opt,
                                                       req_pac: krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7268:1"]
        pub fn krb5_get_init_creds_password(context: krb5_context,
                                            creds: *mut krb5_creds,
                                            client: krb5_principal,
                                            password: *const libc::c_char,
                                            prompter: krb5_prompter_fct,
                                            data: *mut libc::c_void,
                                            start_time: krb5_deltat,
                                            in_tkt_service:
                                                *const libc::c_char,
                                            k5_gic_options:
                                                *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7661:1"]
        pub fn krb5_get_init_creds_keytab(context: krb5_context,
                                          creds: *mut krb5_creds,
                                          client: krb5_principal,
                                          arg_keytab: krb5_keytab,
                                          start_time: krb5_deltat,
                                          in_tkt_service: *const libc::c_char,
                                          k5_gic_options:
                                              *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7771:1"]
        pub fn krb5_get_validated_creds(context: krb5_context,
                                        creds: *mut krb5_creds,
                                        client: krb5_principal,
                                        ccache: krb5_ccache,
                                        in_tkt_service: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7797:1"]
        pub fn krb5_get_renewed_creds(context: krb5_context,
                                      creds: *mut krb5_creds,
                                      client: krb5_principal,
                                      ccache: krb5_ccache,
                                      in_tkt_service: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7880:1"]
        pub fn krb5_get_prompt_types(context: krb5_context)
         -> *mut krb5_prompt_type;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:28"]
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
    #[c2rust::src_loc = "2346:1"]
    pub unsafe extern "C" fn ts_delta(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_deltat {
        return (a as uint32_t).wrapping_sub(b as uint32_t) as krb5_deltat;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor, krb5_principal,
                        krb5_timestamp};
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
        #[c2rust::src_loc = "2126:1"]
        pub fn k5_kt_get_principal(context: krb5_context, keytab: krb5_keytab,
                                   princ_out: *mut krb5_principal)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:28"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:28"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:28"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:28"]
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
#[c2rust::header_src = "/usr/include/pwd.h:46"]
pub mod pwd_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct passwd {
        pub pw_name: *mut libc::c_char,
        pub pw_passwd: *mut libc::c_char,
        pub pw_uid: __uid_t,
        pub pw_gid: __gid_t,
        pub pw_gecos: *mut libc::c_char,
        pub pw_dir: *mut libc::c_char,
        pub pw_shell: *mut libc::c_char,
    }
    use super::types_h::{__uid_t, __gid_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn getpwuid(__uid: __uid_t) -> *mut passwd;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:28"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::stddef_h::size_t;
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "137:14"]
        pub static mut stdin: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "308:1"]
        pub fn setvbuf(__stream: *mut FILE, __buf: *mut libc::c_char,
                       __modes: libc::c_int, __n: size_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "341:12"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: ::std::ffi::VaList) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "786:1"]
        pub fn fileno(__stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:28"]
pub mod unistd_h {
    use super::types_h::__uid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getuid() -> __uid_t;
        #[no_mangle]
        #[c2rust::src_loc = "779:1"]
        pub fn isatty(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:28"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "36:14"]
        pub static mut optarg: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "50:12"]
        pub static mut optind: libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/time.h:28"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:28"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
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
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/clients/kinit/extern.h:31"]
pub mod extern_h {
    use super::krb5_h::{krb5_context, krb5_error_code};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* clients/kinit/extern.h - Global declarations for kinit */
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
        #[no_mangle]
        #[c2rust::src_loc = "30:1"]
        pub fn kinit_kdb_init(pcontext: *mut krb5_context,
                              realm: *mut libc::c_char) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "31:1"]
        pub fn kinit_kdb_fini();
    }
    /* KINIT_EXTERN_H */
}
#[c2rust::header_src = "/usr/include/locale.h:32"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
pub use self::internal::__va_list_tag;
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __uid_t, __gid_t,
                        __off_t, __off64_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::getopt_ext_h::{option, getopt_long};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_authdatatype,
                       krb5_preauthtype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_creds, krb5_creds,
                       krb5_ccache, krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _krb5_prompt,
                       krb5_prompt, krb5_prompter_fct,
                       _krb5_get_init_creds_opt, krb5_get_init_creds_opt,
                       _krb5_gic_opt_pa_data, krb5_gic_opt_pa_data,
                       _profile_t, _krb5_ccache, krb5_cc_get_name,
                       krb5_cc_initialize, krb5_cc_close, krb5_cc_store_cred,
                       krb5_cc_get_principal, krb5_cc_get_type,
                       krb5_cc_new_unique, krb5_kt_close, krb5_init_context,
                       krb5_free_context, krb5_parse_name_flags,
                       krb5_unparse_name, krb5_build_principal_ext,
                       krb5_kt_resolve, krb5_kt_client_default,
                       krb5_cc_resolve, krb5_cc_default, krb5_cc_switch,
                       krb5_cc_support_switch, krb5_cc_cache_match,
                       krb5_free_principal, krb5_free_cred_contents,
                       krb5_free_unparsed_name, krb5_os_localaddr,
                       krb5_get_default_realm, krb5_free_default_realm,
                       krb5_sname_to_principal, krb5_string_to_timestamp,
                       krb5_string_to_deltat, krb5_prompter_posix,
                       krb5_get_init_creds_opt_alloc,
                       krb5_get_init_creds_opt_free,
                       krb5_get_init_creds_opt_set_tkt_life,
                       krb5_get_init_creds_opt_set_renew_life,
                       krb5_get_init_creds_opt_set_forwardable,
                       krb5_get_init_creds_opt_set_proxiable,
                       krb5_get_init_creds_opt_set_canonicalize,
                       krb5_get_init_creds_opt_set_anonymous,
                       krb5_get_init_creds_opt_set_address_list,
                       krb5_get_init_creds_opt_set_pa,
                       krb5_get_init_creds_opt_set_fast_ccache_name,
                       krb5_get_init_creds_opt_set_in_ccache,
                       krb5_get_init_creds_opt_set_out_ccache,
                       krb5_get_init_creds_opt_set_pac_request,
                       krb5_get_init_creds_password,
                       krb5_get_init_creds_keytab, krb5_get_validated_creds,
                       krb5_get_renewed_creds, krb5_get_prompt_types,
                       krb5_get_error_message, krb5_free_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops, ts_delta,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, k5_kt_get_principal};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, et_old_error_hook_func, com_err,
                          set_com_err_hook};
pub use self::pwd_h::{passwd, getpwuid};
use self::stdlib_h::{realloc, free, exit};
use self::stdio_h::{stdin, stdout, stderr, setvbuf, fprintf, vfprintf,
                    fileno};
use self::unistd_h::{getuid, isatty};
use self::getopt_core_h::{optarg, optind};
use self::time_h::time;
use self::libintl_h::dgettext;
use self::string_h::{strlen, strrchr, strchr, strncmp, memset};
use self::extern_h::{kinit_kdb_init, kinit_kdb_fini};
use self::locale_h::setlocale;
#[c2rust::src_loc = "81:9"]
pub type action_type = libc::c_uint;
#[c2rust::src_loc = "81:41"]
pub const VALIDATE: action_type = 3;
#[c2rust::src_loc = "81:34"]
pub const RENEW: action_type = 2;
#[c2rust::src_loc = "81:25"]
pub const INIT_KT: action_type = 1;
#[c2rust::src_loc = "81:16"]
pub const INIT_PW: action_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "83:8"]
pub struct k_opts {
    pub starttime: krb5_deltat,
    pub lifetime: krb5_deltat,
    pub rlife: krb5_deltat,
    pub forwardable: libc::c_int,
    pub proxiable: libc::c_int,
    pub request_pac: libc::c_int,
    pub anonymous: libc::c_int,
    pub addresses: libc::c_int,
    pub not_forwardable: libc::c_int,
    pub not_proxiable: libc::c_int,
    pub not_request_pac: libc::c_int,
    pub no_addresses: libc::c_int,
    pub verbose: libc::c_int,
    pub principal_name: *mut libc::c_char,
    pub service_name: *mut libc::c_char,
    pub keytab_name: *mut libc::c_char,
    pub k5_in_cache_name: *mut libc::c_char,
    pub k5_out_cache_name: *mut libc::c_char,
    pub armor_ccache: *mut libc::c_char,
    pub action: action_type,
    pub use_client_keytab: libc::c_int,
    pub num_pa_opts: libc::c_int,
    pub pa_opts: *mut krb5_gic_opt_pa_data,
    pub canonicalize: libc::c_int,
    pub enterprise: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "120:8"]
pub struct k5_data {
    pub ctx: krb5_context,
    pub in_cc: krb5_ccache,
    pub out_cc: krb5_ccache,
    pub me: krb5_principal,
    pub name: *mut libc::c_char,
    pub switch_to_cache: krb5_boolean,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* clients/kinit/kinit.c - Initialize a credential cache */
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
/* For asprintf and getopt */
#[c2rust::src_loc = "47:1"]
unsafe extern "C" fn get_name_from_os() -> *mut libc::c_char {
    let mut pw: *mut passwd = 0 as *mut passwd;
    pw = getpwuid(getuid());
    return if !pw.is_null() { (*pw).pw_name } else { 0 as *mut libc::c_char };
}
/* HAVE_PWD_H */
/* HAVE_PWD_H */
#[c2rust::src_loc = "79:14"]
static mut progname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/*
 * If struct[2] == NULL, then long_getopt acts as if the short flag struct[3]
 * were specified.  If struct[2] != NULL, then struct[3] is stored in
 * *(struct[2]), the array index which was specified is stored in *index, and
 * long_getopt() returns 0.
 */
#[no_mangle]
#[c2rust::src_loc = "135:13"]
pub static mut shopts: *const libc::c_char =
    b"r:fpFPn54aAVl:s:c:kit:T:RS:vX:CEI:\x00" as *const u8 as
        *const libc::c_char;
#[c2rust::src_loc = "139:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"Usage: %s [-V] [-l lifetime] [-s start_time] \n\t[-r renewable_life] [-f | -F | --forwardable | --noforwardable] \n\t[-p | -P | --proxiable | --noproxiable] \n\t-n [-a | -A | --addresses | --noaddresses] \n\t[--request-pac | --no-request-pac] \n\t[-C | --canonicalize] \n\t[-E | --enterprise] \n\t[-v] [-R] [-k [-i|-t keytab_file]] [-c cachename] \n\t[-S service_name] [-T ticket_armor_cache]\n\t[-X <attribute>[=<value>]] [principal]\n\n\x00"
                as *const u8 as *const libc::c_char, progname);
    fprintf(stderr,
            b"    options:\n\x00" as *const u8 as *const libc::c_char);
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-V verbose\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-l lifetime\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-s start time\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-r renewable lifetime\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-f forwardable\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-F not forwardable\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-p proxiable\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-P not proxiable\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-n anonymous\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-a include addresses\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-A do not include addresses\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-v validate\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-R renew\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-C canonicalize\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-E client is enterprise principal name\n\x00" as
                         *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-k use keytab\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-i use default client keytab (with -k)\n\x00" as
                         *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-t filename of keytab to use\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-c Kerberos 5 cache name\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-S service\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-T armor credential cache\n\x00" as *const u8 as
                         *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t-X <attribute>[=<value>]\n\x00" as *const u8 as
                         *const libc::c_char));
    exit(2 as libc::c_int);
}
#[c2rust::src_loc = "195:21"]
static mut errctx: krb5_context =
    0 as *const _krb5_context as *mut _krb5_context;
#[c2rust::src_loc = "196:1"]
unsafe extern "C" fn extended_com_err_fn(mut myprog: *const libc::c_char,
                                         mut code: errcode_t,
                                         mut fmt: *const libc::c_char,
                                         mut args: ::std::ffi::VaList) {
    let mut emsg: *const libc::c_char = 0 as *const libc::c_char;
    emsg = krb5_get_error_message(errctx, code as krb5_error_code);
    fprintf(stderr, b"%s: %s \x00" as *const u8 as *const libc::c_char,
            myprog, emsg);
    krb5_free_error_message(errctx, emsg);
    vfprintf(stderr, fmt, args.as_va_list());
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
}
#[c2rust::src_loc = "209:1"]
unsafe extern "C" fn add_preauth_opt(mut opts: *mut k_opts,
                                     mut av: *mut libc::c_char)
 -> libc::c_int {
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut krb5_gic_opt_pa_data = 0 as *mut krb5_gic_opt_pa_data;
    let mut x: *mut krb5_gic_opt_pa_data = 0 as *mut krb5_gic_opt_pa_data;
    let mut newsize: size_t =
        (((*opts).num_pa_opts + 1 as libc::c_int) as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_gic_opt_pa_data>()
                                             as libc::c_ulong);
    x =
        realloc((*opts).pa_opts as *mut libc::c_void, newsize) as
            *mut krb5_gic_opt_pa_data;
    if x.is_null() { return 12 as libc::c_int }
    (*opts).pa_opts = x;
    p =
        &mut *(*opts).pa_opts.offset((*opts).num_pa_opts as isize) as
            *mut krb5_gic_opt_pa_data;
    sep = strchr(av, '=' as i32);
    if !sep.is_null() {
        *sep = '\u{0}' as i32 as libc::c_char;
        sep = sep.offset(1);
        v = sep;
        (*p).value = v
    } else {
        (*p).value =
            b"yes\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    }
    (*p).attr = av;
    (*opts).num_pa_opts += 1;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "235:1"]
unsafe extern "C" fn parse_options(mut argc: libc::c_int,
                                   mut argv: *mut *mut libc::c_char,
                                   mut opts: *mut k_opts)
 -> *mut libc::c_char {
    let mut long_options: [option; 11] =
        [{
             let mut init =
                 option{name:
                            b"noforwardable\x00" as *const u8 as
                                *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: 'F' as i32,};
             init
         },
         {
             let mut init =
                 option{name:
                            b"noproxiable\x00" as *const u8 as
                                *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: 'P' as i32,};
             init
         },
         {
             let mut init =
                 option{name:
                            b"addresses\x00" as *const u8 as
                                *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: 'a' as i32,};
             init
         },
         {
             let mut init =
                 option{name:
                            b"forwardable\x00" as *const u8 as
                                *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: 'f' as i32,};
             init
         },
         {
             let mut init =
                 option{name:
                            b"proxiable\x00" as *const u8 as
                                *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: 'p' as i32,};
             init
         },
         {
             let mut init =
                 option{name:
                            b"noaddresses\x00" as *const u8 as
                                *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: 'A' as i32,};
             init
         },
         {
             let mut init =
                 option{name:
                            b"canonicalize\x00" as *const u8 as
                                *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: 'C' as i32,};
             init
         },
         {
             let mut init =
                 option{name:
                            b"enterprise\x00" as *const u8 as
                                *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: 'E' as i32,};
             init
         },
         {
             let mut init =
                 option{name:
                            b"request-pac\x00" as *const u8 as
                                *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: &mut (*opts).request_pac,
                        val: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 option{name:
                            b"no-request-pac\x00" as *const u8 as
                                *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: &mut (*opts).not_request_pac,
                        val: 1 as libc::c_int,};
             init
         },
         {
             let mut init =
                 option{name: 0 as *const libc::c_char,
                        has_arg: 0 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: 0 as libc::c_int,};
             init
         }];
    let mut ret: krb5_error_code = 0;
    let mut errflg: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    loop  {
        i =
            getopt_long(argc, argv, shopts, long_options.as_mut_ptr(),
                        0 as *mut libc::c_int);
        if !(i != -(1 as libc::c_int)) { break ; }
        match i {
            86 => { (*opts).verbose = 1 as libc::c_int }
            108 => {
                /* Lifetime */
                ret = krb5_string_to_deltat(optarg, &mut (*opts).lifetime);
                if ret != 0 || (*opts).lifetime == 0 as libc::c_int {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Bad lifetime value %s\n\x00" as
                                         *const u8 as *const libc::c_char),
                            optarg);
                    errflg += 1
                }
            }
            114 => {
                /* Renewable Time */
                ret = krb5_string_to_deltat(optarg, &mut (*opts).rlife);
                if ret != 0 || (*opts).rlife == 0 as libc::c_int {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Bad lifetime value %s\n\x00" as
                                         *const u8 as *const libc::c_char),
                            optarg);
                    errflg += 1
                }
            }
            102 => { (*opts).forwardable = 1 as libc::c_int }
            70 => { (*opts).not_forwardable = 1 as libc::c_int }
            112 => { (*opts).proxiable = 1 as libc::c_int }
            80 => { (*opts).not_proxiable = 1 as libc::c_int }
            110 => { (*opts).anonymous = 1 as libc::c_int }
            97 => { (*opts).addresses = 1 as libc::c_int }
            65 => { (*opts).no_addresses = 1 as libc::c_int }
            115 => {
                ret = krb5_string_to_deltat(optarg, &mut (*opts).starttime);
                if ret != 0 || (*opts).starttime == 0 as libc::c_int {
                    /* Parse as an absolute time; intentionally undocumented
                 * but left for backwards compatibility. */
                    let mut abs_starttime: krb5_timestamp = 0;
                    ret =
                        krb5_string_to_timestamp(optarg, &mut abs_starttime);
                    if ret != 0 || abs_starttime == 0 as libc::c_int {
                        fprintf(stderr,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"Bad start time value %s\n\x00" as
                                             *const u8 as
                                             *const libc::c_char), optarg);
                        errflg += 1
                    } else {
                        (*opts).starttime =
                            ts_delta(abs_starttime,
                                     time(0 as *mut time_t) as krb5_timestamp)
                    }
                }
            }
            83 => { (*opts).service_name = optarg }
            107 => { (*opts).action = INIT_KT }
            105 => { (*opts).use_client_keytab = 1 as libc::c_int }
            116 => {
                if !(*opts).keytab_name.is_null() {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Only one -t option allowed.\n\x00" as
                                         *const u8 as *const libc::c_char));
                    errflg += 1
                } else { (*opts).keytab_name = optarg }
            }
            84 => {
                if !(*opts).armor_ccache.is_null() {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Only one armor_ccache\n\x00" as
                                         *const u8 as *const libc::c_char));
                    errflg += 1
                } else { (*opts).armor_ccache = optarg }
            }
            82 => { (*opts).action = RENEW }
            118 => { (*opts).action = VALIDATE }
            99 => {
                if !(*opts).k5_out_cache_name.is_null() {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Only one -c option allowed\n\x00" as
                                         *const u8 as *const libc::c_char));
                    errflg += 1
                } else { (*opts).k5_out_cache_name = optarg }
            }
            73 => {
                if !(*opts).k5_in_cache_name.is_null() {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Only one -I option allowed\n\x00" as
                                         *const u8 as *const libc::c_char));
                    errflg += 1
                } else { (*opts).k5_in_cache_name = optarg }
            }
            88 => {
                ret = add_preauth_opt(opts, optarg);
                if ret != 0 {
                    com_err(progname, ret as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while adding preauth option\x00" as
                                         *const u8 as *const libc::c_char));
                    errflg += 1
                }
            }
            67 => { (*opts).canonicalize = 1 as libc::c_int }
            69 => { (*opts).enterprise = 1 as libc::c_int }
            52 => {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Kerberos 4 is no longer supported\n\x00" as
                                     *const u8 as *const libc::c_char));
                exit(3 as libc::c_int);
            }
            53 | 0 => { }
            _ => { errflg += 1 }
        }
    }
    if (*opts).forwardable != 0 && (*opts).not_forwardable != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Only one of -f and -F allowed\n\x00" as *const u8
                             as *const libc::c_char));
        errflg += 1
    }
    if (*opts).proxiable != 0 && (*opts).not_proxiable != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Only one of -p and -P allowed\n\x00" as *const u8
                             as *const libc::c_char));
        errflg += 1
    }
    if (*opts).request_pac != 0 && (*opts).not_request_pac != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Only one of --request-pac and --no-request-pac allowed\n\x00"
                             as *const u8 as *const libc::c_char));
        errflg += 1
    }
    if (*opts).addresses != 0 && (*opts).no_addresses != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Only one of -a and -A allowed\n\x00" as *const u8
                             as *const libc::c_char));
        errflg += 1
    }
    if !(*opts).keytab_name.is_null() &&
           (*opts).use_client_keytab == 1 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Only one of -t and -i allowed\n\x00" as *const u8
                             as *const libc::c_char));
        errflg += 1
    }
    if (!(*opts).keytab_name.is_null() ||
            (*opts).use_client_keytab == 1 as libc::c_int) &&
           (*opts).action as libc::c_uint !=
               INIT_KT as libc::c_int as libc::c_uint {
        (*opts).action = INIT_KT;
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"keytab specified, forcing -k\n\x00" as *const u8 as
                             *const libc::c_char));
    }
    if argc - optind > 1 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Extra arguments (starting with \"%s\").\n\x00" as
                             *const u8 as *const libc::c_char),
                *argv.offset((optind + 1 as libc::c_int) as isize));
        errflg += 1
    }
    if errflg != 0 { usage(); }
    (*opts).principal_name =
        if optind == argc - 1 as libc::c_int {
            *argv.offset(optind as isize)
        } else { 0 as *mut libc::c_char };
    return (*opts).principal_name;
}
#[c2rust::src_loc = "427:1"]
unsafe extern "C" fn k5_begin(mut opts: *mut k_opts, mut k5: *mut k5_data)
 -> libc::c_int {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut success: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int =
        if (*opts).enterprise != 0 {
            0x4 as libc::c_int
        } else { 0 as libc::c_int };
    let mut defcache: krb5_ccache = 0 as krb5_ccache;
    let mut defcache_princ: krb5_principal = 0 as krb5_principal;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut keytab: krb5_keytab = 0 as *mut _krb5_kt;
    let mut deftype: *const libc::c_char = 0 as *const libc::c_char;
    let mut defrealm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    ret = krb5_init_context(&mut (*k5).ctx);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing Kerberos 5 library\x00" as
                             *const u8 as *const libc::c_char));
        return 0 as libc::c_int
    }
    errctx = (*k5).ctx;
    if !(*opts).k5_out_cache_name.is_null() {
        ret =
            krb5_cc_resolve((*k5).ctx, (*opts).k5_out_cache_name,
                            &mut (*k5).out_cc);
        if ret != 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"resolving ccache %s\x00" as *const u8 as
                                 *const libc::c_char),
                    (*opts).k5_out_cache_name);
            current_block = 2086477769549425743;
        } else {
            if (*opts).verbose != 0 {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Using specified cache: %s\n\x00" as
                                     *const u8 as *const libc::c_char),
                        (*opts).k5_out_cache_name);
            }
            current_block = 11194104282611034094;
        }
    } else {
        /* Resolve the default ccache and get its type and default principal
         * (if it is initialized). */
        ret = krb5_cc_default((*k5).ctx, &mut defcache);
        if ret != 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while getting default ccache\x00" as *const u8
                                 as *const libc::c_char));
            current_block = 2086477769549425743;
        } else {
            deftype = krb5_cc_get_type((*k5).ctx, defcache);
            if krb5_cc_get_principal((*k5).ctx, defcache, &mut defcache_princ)
                   != 0 as libc::c_int {
                defcache_princ = 0 as krb5_principal
            }
            current_block = 11194104282611034094;
        }
    }
    match current_block {
        11194104282611034094 =>
        /* Choose a client principal name. */
        {
            if !(*opts).principal_name.is_null() {
                /* Use the specified principal name. */
                ret =
                    krb5_parse_name_flags((*k5).ctx, (*opts).principal_name,
                                          flags, &mut (*k5).me);
                if ret != 0 {
                    com_err(progname, ret as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"when parsing name %s\x00" as *const u8
                                         as *const libc::c_char),
                            (*opts).principal_name);
                    current_block = 2086477769549425743;
                } else { current_block = 6281126495347172768; }
            } else if (*opts).anonymous != 0 {
                /* Use the anonymous principal for the local realm. */
                ret = krb5_get_default_realm((*k5).ctx, &mut defrealm);
                if ret != 0 {
                    com_err(progname, ret as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while getting default realm\x00" as
                                         *const u8 as *const libc::c_char));
                    current_block = 2086477769549425743;
                } else {
                    ret =
                        krb5_build_principal_ext((*k5).ctx,
                                                 &mut (*k5).me as
                                                     *mut krb5_principal,
                                                 strlen(defrealm) as
                                                     libc::c_uint, defrealm,
                                                 strlen(b"WELLKNOWN\x00" as
                                                            *const u8 as
                                                            *const libc::c_char),
                                                 b"WELLKNOWN\x00" as *const u8
                                                     as *const libc::c_char,
                                                 strlen(b"ANONYMOUS\x00" as
                                                            *const u8 as
                                                            *const libc::c_char),
                                                 b"ANONYMOUS\x00" as *const u8
                                                     as *const libc::c_char,
                                                 0 as libc::c_int);
                    krb5_free_default_realm((*k5).ctx, defrealm);
                    if ret != 0 {
                        com_err(progname, ret as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"while building principal\x00" as
                                             *const u8 as
                                             *const libc::c_char));
                        current_block = 2086477769549425743;
                    } else { current_block = 6281126495347172768; }
                }
            } else if (*opts).action as libc::c_uint ==
                          INIT_KT as libc::c_int as libc::c_uint &&
                          (*opts).use_client_keytab != 0 {
                /* Use the first entry from the client keytab. */
                ret = krb5_kt_client_default((*k5).ctx, &mut keytab);
                if ret != 0 {
                    com_err(progname, ret as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"When resolving the default client keytab\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    current_block = 2086477769549425743;
                } else {
                    ret =
                        k5_kt_get_principal((*k5).ctx, keytab, &mut (*k5).me);
                    krb5_kt_close((*k5).ctx, keytab);
                    if ret != 0 {
                        com_err(progname, ret as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"When determining client principal name from keytab\x00"
                                             as *const u8 as
                                             *const libc::c_char));
                        current_block = 2086477769549425743;
                    } else { current_block = 6281126495347172768; }
                }
            } else if (*opts).action as libc::c_uint ==
                          INIT_KT as libc::c_int as libc::c_uint {
                /* Use the default host/service name. */
                ret =
                    krb5_sname_to_principal((*k5).ctx,
                                            0 as *const libc::c_char,
                                            0 as *const libc::c_char,
                                            3 as libc::c_int, &mut (*k5).me);
                if ret != 0 {
                    com_err(progname, ret as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"when creating default server principal name\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    current_block = 2086477769549425743;
                } else if *(*(*k5).me).realm.data.offset(0 as libc::c_int as
                                                             isize) as
                              libc::c_int == 0 as libc::c_int {
                    ret =
                        krb5_unparse_name((*k5).ctx,
                                          (*k5).me as krb5_const_principal,
                                          &mut (*k5).name);
                    if ret == 0 as libc::c_int {
                        com_err(progname, -(1765328167 as libc::c_long),
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"(principal %s)\x00" as *const u8 as
                                             *const libc::c_char),
                                (*k5).name);
                    } else {
                        com_err(progname, -(1765328167 as libc::c_long),
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"for local services\x00" as
                                             *const u8 as
                                             *const libc::c_char));
                    }
                    current_block = 2086477769549425743;
                } else { current_block = 6281126495347172768; }
            } else {
                if !(*k5).out_cc.is_null() {
                    /* If the output ccache is initialized, use its principal. */
                    if krb5_cc_get_principal((*k5).ctx, (*k5).out_cc,
                                             &mut princ) == 0 as libc::c_int {
                        (*k5).me = princ
                    }
                } else if !defcache_princ.is_null() {
                    /* Use the default cache's principal, and use the default cache as the
         * output cache. */
                    (*k5).out_cc = defcache;
                    defcache = 0 as krb5_ccache;
                    (*k5).me = defcache_princ;
                    defcache_princ = 0 as krb5_principal
                }
                current_block = 6281126495347172768;
            }
            match current_block {
                2086477769549425743 => { }
                _ =>
                /* If we still haven't chosen, use the local username. */
                {
                    if (*k5).me.is_null() {
                        name = get_name_from_os();
                        if name.is_null() {
                            fprintf(stderr,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"Unable to identify user\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char));
                            current_block = 2086477769549425743;
                        } else {
                            ret =
                                krb5_parse_name_flags((*k5).ctx, name, flags,
                                                      &mut (*k5).me);
                            if ret != 0 {
                                com_err(progname, ret as errcode_t,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"when parsing name %s\x00"
                                                     as *const u8 as
                                                     *const libc::c_char),
                                        name);
                                current_block = 2086477769549425743;
                            } else { current_block = 11052029508375673978; }
                        }
                    } else { current_block = 11052029508375673978; }
                    match current_block {
                        2086477769549425743 => { }
                        _ => {
                            if (*k5).out_cc.is_null() &&
                                   krb5_cc_support_switch((*k5).ctx, deftype)
                                       != 0 {
                                /* Use an existing cache for the client principal if we can. */
                                ret =
                                    krb5_cc_cache_match((*k5).ctx, (*k5).me,
                                                        &mut (*k5).out_cc);
                                if ret != 0 &&
                                       ret as libc::c_long !=
                                           -(1765328243 as libc::c_long) {
                                    com_err(progname, ret as errcode_t,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"while searching for ccache for %s\x00"
                                                         as *const u8 as
                                                         *const libc::c_char),
                                            (*opts).principal_name);
                                    current_block = 2086477769549425743;
                                } else if ret == 0 {
                                    if (*opts).verbose != 0 {
                                        fprintf(stderr,
                                                dgettext(b"mit-krb5\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"Using existing cache: %s\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char),
                                                krb5_cc_get_name((*k5).ctx,
                                                                 (*k5).out_cc));
                                    }
                                    (*k5).switch_to_cache =
                                        1 as libc::c_int as krb5_boolean;
                                    current_block = 18425699056680496821;
                                } else if !defcache_princ.is_null() {
                                    /* Create a new cache to avoid overwriting the initialized default
             * cache. */
                                    ret =
                                        krb5_cc_new_unique((*k5).ctx, deftype,
                                                           0 as
                                                               *const libc::c_char,
                                                           &mut (*k5).out_cc);
                                    if ret != 0 {
                                        com_err(progname, ret as errcode_t,
                                                dgettext(b"mit-krb5\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"while generating new ccache\x00"
                                                             as *const u8 as
                                                             *const libc::c_char));
                                        current_block = 2086477769549425743;
                                    } else {
                                        if (*opts).verbose != 0 {
                                            fprintf(stderr,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"Using new cache: %s\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char),
                                                    krb5_cc_get_name((*k5).ctx,
                                                                     (*k5).out_cc));
                                        }
                                        (*k5).switch_to_cache =
                                            1 as libc::c_int as krb5_boolean;
                                        current_block = 18425699056680496821;
                                    }
                                } else {
                                    current_block = 18425699056680496821;
                                }
                            } else { current_block = 18425699056680496821; }
                            match current_block {
                                2086477769549425743 => { }
                                _ => {
                                    /* Use the default cache if we haven't picked one yet. */
                                    if (*k5).out_cc.is_null() {
                                        (*k5).out_cc = defcache;
                                        defcache = 0 as krb5_ccache;
                                        if (*opts).verbose != 0 {
                                            fprintf(stderr,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"Using default cache: %s\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char),
                                                    krb5_cc_get_name((*k5).ctx,
                                                                     (*k5).out_cc));
                                        }
                                    }
                                    if !(*opts).k5_in_cache_name.is_null() {
                                        ret =
                                            krb5_cc_resolve((*k5).ctx,
                                                            (*opts).k5_in_cache_name,
                                                            &mut (*k5).in_cc);
                                        if ret != 0 {
                                            com_err(progname,
                                                    ret as errcode_t,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"resolving ccache %s\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char),
                                                    (*opts).k5_in_cache_name);
                                            current_block =
                                                2086477769549425743;
                                        } else {
                                            if (*opts).verbose != 0 {
                                                fprintf(stderr,
                                                        dgettext(b"mit-krb5\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"Using specified input cache: %s\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char),
                                                        (*opts).k5_in_cache_name);
                                            }
                                            current_block =
                                                2463987395154258233;
                                        }
                                    } else {
                                        current_block = 2463987395154258233;
                                    }
                                    match current_block {
                                        2086477769549425743 => { }
                                        _ => {
                                            ret =
                                                krb5_unparse_name((*k5).ctx,
                                                                  (*k5).me as
                                                                      krb5_const_principal,
                                                                  &mut (*k5).name);
                                            if ret != 0 {
                                                com_err(progname,
                                                        ret as errcode_t,
                                                        dgettext(b"mit-krb5\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"when unparsing name\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char));
                                            } else {
                                                if (*opts).verbose != 0 {
                                                    fprintf(stderr,
                                                            dgettext(b"mit-krb5\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"Using principal: %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char),
                                                            (*k5).name);
                                                }
                                                (*opts).principal_name =
                                                    (*k5).name;
                                                success = 1 as libc::c_int
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
    if !defcache.is_null() { krb5_cc_close((*k5).ctx, defcache); }
    krb5_free_principal((*k5).ctx, defcache_princ);
    return success;
}
#[c2rust::src_loc = "632:1"]
unsafe extern "C" fn k5_end(mut k5: *mut k5_data) {
    krb5_free_unparsed_name((*k5).ctx, (*k5).name);
    krb5_free_principal((*k5).ctx, (*k5).me);
    if !(*k5).in_cc.is_null() { krb5_cc_close((*k5).ctx, (*k5).in_cc); }
    if !(*k5).out_cc.is_null() { krb5_cc_close((*k5).ctx, (*k5).out_cc); }
    krb5_free_context((*k5).ctx);
    errctx = 0 as krb5_context;
    memset(k5 as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<k5_data>() as libc::c_ulong);
}
#[c2rust::src_loc = "646:1"]
unsafe extern "C" fn kinit_prompter(mut ctx: krb5_context,
                                    mut data: *mut libc::c_void,
                                    mut name: *const libc::c_char,
                                    mut banner: *const libc::c_char,
                                    mut num_prompts: libc::c_int,
                                    mut prompts: *mut krb5_prompt)
 -> krb5_error_code {
    let mut pwprompt: *mut krb5_boolean = data as *mut krb5_boolean;
    let mut ptypes: *mut krb5_prompt_type = 0 as *mut krb5_prompt_type;
    let mut i: libc::c_int = 0;
    /* Make a note if we receive a password prompt. */
    ptypes = krb5_get_prompt_types(ctx);
    i = 0 as libc::c_int;
    while i < num_prompts {
        if !ptypes.is_null() &&
               *ptypes.offset(i as isize) == 0x1 as libc::c_int {
            *pwprompt = 1 as libc::c_int as krb5_boolean
        }
        i += 1
    }
    return krb5_prompter_posix(ctx, data, name, banner, num_prompts, prompts);
}
#[c2rust::src_loc = "663:1"]
unsafe extern "C" fn k5_kinit(mut opts: *mut k_opts, mut k5: *mut k5_data)
 -> libc::c_int {
    let mut current_block: u64;
    let mut notix: libc::c_int = 1 as libc::c_int;
    let mut keytab: krb5_keytab = 0 as krb5_keytab;
    let mut my_creds: krb5_creds =
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
    let mut ret: krb5_error_code = 0;
    let mut options: *mut krb5_get_init_creds_opt =
        0 as *mut krb5_get_init_creds_opt;
    let mut pwprompt: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut addresses: *mut *mut krb5_address = 0 as *mut *mut krb5_address;
    let mut i: libc::c_int = 0;
    memset(&mut my_creds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    ret = krb5_get_init_creds_opt_alloc((*k5).ctx, &mut options);
    if !(ret != 0) {
        if (*opts).lifetime != 0 {
            krb5_get_init_creds_opt_set_tkt_life(options, (*opts).lifetime);
        }
        if (*opts).rlife != 0 {
            krb5_get_init_creds_opt_set_renew_life(options, (*opts).rlife);
        }
        if (*opts).forwardable != 0 {
            krb5_get_init_creds_opt_set_forwardable(options,
                                                    1 as libc::c_int);
        }
        if (*opts).not_forwardable != 0 {
            krb5_get_init_creds_opt_set_forwardable(options,
                                                    0 as libc::c_int);
        }
        if (*opts).proxiable != 0 {
            krb5_get_init_creds_opt_set_proxiable(options, 1 as libc::c_int);
        }
        if (*opts).not_proxiable != 0 {
            krb5_get_init_creds_opt_set_proxiable(options, 0 as libc::c_int);
        }
        if (*opts).canonicalize != 0 {
            krb5_get_init_creds_opt_set_canonicalize(options,
                                                     1 as libc::c_int);
        }
        if (*opts).anonymous != 0 {
            krb5_get_init_creds_opt_set_anonymous(options, 1 as libc::c_int);
        }
        if (*opts).addresses != 0 {
            ret = krb5_os_localaddr((*k5).ctx, &mut addresses);
            if ret != 0 {
                com_err(progname, ret as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"getting local addresses\x00" as *const u8
                                     as *const libc::c_char));
                current_block = 17864888867068740914;
            } else {
                krb5_get_init_creds_opt_set_address_list(options, addresses);
                current_block = 15925075030174552612;
            }
        } else { current_block = 15925075030174552612; }
        match current_block {
            17864888867068740914 => { }
            _ => {
                if (*opts).no_addresses != 0 {
                    krb5_get_init_creds_opt_set_address_list(options,
                                                             0 as
                                                                 *mut *mut krb5_address);
                }
                if !(*opts).armor_ccache.is_null() {
                    krb5_get_init_creds_opt_set_fast_ccache_name((*k5).ctx,
                                                                 options,
                                                                 (*opts).armor_ccache);
                }
                if (*opts).request_pac != 0 {
                    krb5_get_init_creds_opt_set_pac_request((*k5).ctx,
                                                            options,
                                                            1 as libc::c_int
                                                                as
                                                                krb5_boolean);
                }
                if (*opts).not_request_pac != 0 {
                    krb5_get_init_creds_opt_set_pac_request((*k5).ctx,
                                                            options,
                                                            0 as libc::c_int
                                                                as
                                                                krb5_boolean);
                }
                if (*opts).action as libc::c_uint ==
                       INIT_KT as libc::c_int as libc::c_uint &&
                       !(*opts).keytab_name.is_null() {
                    if strncmp((*opts).keytab_name,
                               b"KDB:\x00" as *const u8 as
                                   *const libc::c_char,
                               4 as libc::c_int as libc::c_ulong) ==
                           0 as libc::c_int {
                        ret =
                            kinit_kdb_init(&mut (*k5).ctx,
                                           (*(*k5).me).realm.data);
                        errctx = (*k5).ctx;
                        if ret != 0 {
                            com_err(progname, ret as errcode_t,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"while setting up KDB keytab for realm %s\x00"
                                                 as *const u8 as
                                                 *const libc::c_char),
                                    (*(*k5).me).realm.data);
                            current_block = 17864888867068740914;
                        } else { current_block = 15512526488502093901; }
                    } else { current_block = 15512526488502093901; }
                    match current_block {
                        17864888867068740914 => { }
                        _ => {
                            ret =
                                krb5_kt_resolve((*k5).ctx,
                                                (*opts).keytab_name,
                                                &mut keytab);
                            if ret != 0 {
                                com_err(progname, ret as errcode_t,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"resolving keytab %s\x00" as
                                                     *const u8 as
                                                     *const libc::c_char),
                                        (*opts).keytab_name);
                                current_block = 17864888867068740914;
                            } else {
                                if (*opts).verbose != 0 {
                                    fprintf(stderr,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"Using keytab: %s\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char),
                                            (*opts).keytab_name);
                                }
                                current_block = 1724319918354933278;
                            }
                        }
                    }
                } else if (*opts).action as libc::c_uint ==
                              INIT_KT as libc::c_int as libc::c_uint &&
                              (*opts).use_client_keytab != 0 {
                    ret = krb5_kt_client_default((*k5).ctx, &mut keytab);
                    if ret != 0 {
                        com_err(progname, ret as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"resolving default client keytab\x00"
                                             as *const u8 as
                                             *const libc::c_char));
                        current_block = 17864888867068740914;
                    } else { current_block = 1724319918354933278; }
                } else { current_block = 1724319918354933278; }
                match current_block {
                    17864888867068740914 => { }
                    _ => {
                        i = 0 as libc::c_int;
                        loop  {
                            if !(i < (*opts).num_pa_opts) {
                                current_block = 17075014677070940716;
                                break ;
                            }
                            ret =
                                krb5_get_init_creds_opt_set_pa((*k5).ctx,
                                                               options,
                                                               (*(*opts).pa_opts.offset(i
                                                                                            as
                                                                                            isize)).attr,
                                                               (*(*opts).pa_opts.offset(i
                                                                                            as
                                                                                            isize)).value);
                            if ret != 0 {
                                com_err(progname, ret as errcode_t,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"while setting \'%s\'=\'%s\'\x00"
                                                     as *const u8 as
                                                     *const libc::c_char),
                                        (*(*opts).pa_opts.offset(i as
                                                                     isize)).attr,
                                        (*(*opts).pa_opts.offset(i as
                                                                     isize)).value);
                                current_block = 17864888867068740914;
                                break ;
                            } else {
                                if (*opts).verbose != 0 {
                                    fprintf(stderr,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"PA Option %s = %s\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char),
                                            (*(*opts).pa_opts.offset(i as
                                                                         isize)).attr,
                                            (*(*opts).pa_opts.offset(i as
                                                                         isize)).value);
                                }
                                i += 1
                            }
                        }
                        match current_block {
                            17864888867068740914 => { }
                            _ => {
                                if !(*k5).in_cc.is_null() {
                                    ret =
                                        krb5_get_init_creds_opt_set_in_ccache((*k5).ctx,
                                                                              options,
                                                                              (*k5).in_cc);
                                    if ret != 0 {
                                        current_block = 17864888867068740914;
                                    } else {
                                        current_block = 7494008139977416618;
                                    }
                                } else {
                                    current_block = 7494008139977416618;
                                }
                                match current_block {
                                    17864888867068740914 => { }
                                    _ => {
                                        ret =
                                            krb5_get_init_creds_opt_set_out_ccache((*k5).ctx,
                                                                                   options,
                                                                                   (*k5).out_cc);
                                        if !(ret != 0) {
                                            match (*opts).action as
                                                      libc::c_uint {
                                                0 => {
                                                    ret =
                                                        krb5_get_init_creds_password((*k5).ctx,
                                                                                     &mut my_creds,
                                                                                     (*k5).me,
                                                                                     0
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     Some(kinit_prompter
                                                                                              as
                                                                                              unsafe extern "C" fn(_:
                                                                                                                       krb5_context,
                                                                                                                   _:
                                                                                                                       *mut libc::c_void,
                                                                                                                   _:
                                                                                                                       *const libc::c_char,
                                                                                                                   _:
                                                                                                                       *const libc::c_char,
                                                                                                                   _:
                                                                                                                       libc::c_int,
                                                                                                                   _:
                                                                                                                       *mut krb5_prompt)
                                                                                                  ->
                                                                                                      krb5_error_code),
                                                                                     &mut pwprompt
                                                                                         as
                                                                                         *mut krb5_boolean
                                                                                         as
                                                                                         *mut libc::c_void,
                                                                                     (*opts).starttime,
                                                                                     (*opts).service_name,
                                                                                     options)
                                                }
                                                1 => {
                                                    ret =
                                                        krb5_get_init_creds_keytab((*k5).ctx,
                                                                                   &mut my_creds,
                                                                                   (*k5).me,
                                                                                   keytab,
                                                                                   (*opts).starttime,
                                                                                   (*opts).service_name,
                                                                                   options)
                                                }
                                                3 => {
                                                    ret =
                                                        krb5_get_validated_creds((*k5).ctx,
                                                                                 &mut my_creds,
                                                                                 (*k5).me,
                                                                                 (*k5).out_cc,
                                                                                 (*opts).service_name)
                                                }
                                                2 => {
                                                    ret =
                                                        krb5_get_renewed_creds((*k5).ctx,
                                                                               &mut my_creds,
                                                                               (*k5).me,
                                                                               (*k5).out_cc,
                                                                               (*opts).service_name)
                                                }
                                                _ => { }
                                            }
                                            if ret != 0 {
                                                let mut doing:
                                                        *mut libc::c_char =
                                                    0 as *mut libc::c_char;
                                                match (*opts).action as
                                                          libc::c_uint {
                                                    0 | 1 => {
                                                        doing =
                                                            dgettext(b"mit-krb5\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"getting initial credentials\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char)
                                                    }
                                                    3 => {
                                                        doing =
                                                            dgettext(b"mit-krb5\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"validating credentials\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char)
                                                    }
                                                    2 => {
                                                        doing =
                                                            dgettext(b"mit-krb5\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"renewing credentials\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char)
                                                    }
                                                    _ => { }
                                                }
                                                /* If reply decryption failed, or if pre-authentication failed and we
         * were prompted for a password, assume the password was wrong. */
                                                if ret as libc::c_long ==
                                                       -(1765328353 as
                                                             libc::c_long) ||
                                                       pwprompt != 0 &&
                                                           ret as libc::c_long
                                                               ==
                                                               -(1765328360 as
                                                                     libc::c_long)
                                                   {
                                                    fprintf(stderr,
                                                            dgettext(b"mit-krb5\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"%s: Password incorrect while %s\n\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char),
                                                            progname, doing);
                                                } else {
                                                    com_err(progname,
                                                            ret as errcode_t,
                                                            dgettext(b"mit-krb5\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"while %s\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char),
                                                            doing);
                                                }
                                            } else {
                                                if (*opts).action as
                                                       libc::c_uint !=
                                                       INIT_PW as libc::c_int
                                                           as libc::c_uint &&
                                                       (*opts).action as
                                                           libc::c_uint !=
                                                           INIT_KT as
                                                               libc::c_int as
                                                               libc::c_uint {
                                                    ret =
                                                        krb5_cc_initialize((*k5).ctx,
                                                                           (*k5).out_cc,
                                                                           if (*opts).canonicalize
                                                                                  !=
                                                                                  0
                                                                              {
                                                                               my_creds.client
                                                                           } else {
                                                                               (*k5).me
                                                                           });
                                                    if ret != 0 {
                                                        com_err(progname,
                                                                ret as
                                                                    errcode_t,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"when initializing cache %s\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char),
                                                                if !(*opts).k5_out_cache_name.is_null()
                                                                   {
                                                                    (*opts).k5_out_cache_name
                                                                        as
                                                                        *const libc::c_char
                                                                } else {
                                                                    b"\x00" as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                });
                                                        current_block =
                                                            17864888867068740914;
                                                    } else {
                                                        if (*opts).verbose !=
                                                               0 {
                                                            fprintf(stderr,
                                                                    dgettext(b"mit-krb5\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char,
                                                                             b"Initialized cache\n\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char));
                                                        }
                                                        ret =
                                                            krb5_cc_store_cred((*k5).ctx,
                                                                               (*k5).out_cc,
                                                                               &mut my_creds);
                                                        if ret != 0 {
                                                            com_err(progname,
                                                                    ret as
                                                                        errcode_t,
                                                                    dgettext(b"mit-krb5\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char,
                                                                             b"while storing credentials\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char));
                                                            current_block =
                                                                17864888867068740914;
                                                        } else {
                                                            if (*opts).verbose
                                                                   != 0 {
                                                                fprintf(stderr,
                                                                        dgettext(b"mit-krb5\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"Stored credentials\n\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
                                                            }
                                                            current_block =
                                                                15622658527355336244;
                                                        }
                                                    }
                                                } else {
                                                    current_block =
                                                        15622658527355336244;
                                                }
                                                match current_block {
                                                    17864888867068740914 => {
                                                    }
                                                    _ => {
                                                        notix =
                                                            0 as libc::c_int;
                                                        if (*k5).switch_to_cache
                                                               != 0 {
                                                            ret =
                                                                krb5_cc_switch((*k5).ctx,
                                                                               (*k5).out_cc);
                                                            if ret != 0 {
                                                                com_err(progname,
                                                                        ret as
                                                                            errcode_t,
                                                                        dgettext(b"mit-krb5\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"while switching to new ccache\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
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
    kinit_kdb_fini();
    if !options.is_null() {
        krb5_get_init_creds_opt_free((*k5).ctx, options);
    }
    if my_creds.client == (*k5).me { my_creds.client = 0 as krb5_principal }
    if !(*opts).pa_opts.is_null() {
        free((*opts).pa_opts as *mut libc::c_void);
        (*opts).pa_opts = 0 as *mut krb5_gic_opt_pa_data;
        (*opts).num_pa_opts = 0 as libc::c_int
    }
    krb5_free_cred_contents((*k5).ctx, &mut my_creds);
    if !keytab.is_null() { krb5_kt_close((*k5).ctx, keytab); }
    return if notix != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
#[c2rust::src_loc = "867:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut opts: k_opts =
        k_opts{starttime: 0,
               lifetime: 0,
               rlife: 0,
               forwardable: 0,
               proxiable: 0,
               request_pac: 0,
               anonymous: 0,
               addresses: 0,
               not_forwardable: 0,
               not_proxiable: 0,
               not_request_pac: 0,
               no_addresses: 0,
               verbose: 0,
               principal_name: 0 as *mut libc::c_char,
               service_name: 0 as *mut libc::c_char,
               keytab_name: 0 as *mut libc::c_char,
               k5_in_cache_name: 0 as *mut libc::c_char,
               k5_out_cache_name: 0 as *mut libc::c_char,
               armor_ccache: 0 as *mut libc::c_char,
               action: INIT_PW,
               use_client_keytab: 0,
               num_pa_opts: 0,
               pa_opts: 0 as *mut krb5_gic_opt_pa_data,
               canonicalize: 0,
               enterprise: 0,};
    let mut k5: k5_data =
        k5_data{ctx: 0 as *mut _krb5_context,
                in_cc: 0 as *mut _krb5_ccache,
                out_cc: 0 as *mut _krb5_ccache,
                me: 0 as *mut krb5_principal_data,
                name: 0 as *mut libc::c_char,
                switch_to_cache: 0,};
    let mut authed_k5: libc::c_int = 0 as libc::c_int;
    setlocale(6 as libc::c_int, b"\x00" as *const u8 as *const libc::c_char);
    progname =
        if !strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).is_null() {
            strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).offset(1 as libc::c_int as isize)
        } else { *argv.offset(0 as libc::c_int as isize) };
    /* Ensure we can be driven from a pipe */
    if isatty(fileno(stdin)) == 0 {
        setvbuf(stdin, 0 as *mut libc::c_char, 2 as libc::c_int,
                0 as libc::c_int as size_t);
    }
    if isatty(fileno(stdout)) == 0 {
        setvbuf(stdout, 0 as *mut libc::c_char, 2 as libc::c_int,
                0 as libc::c_int as size_t);
    }
    if isatty(fileno(stderr)) == 0 {
        setvbuf(stderr, 0 as *mut libc::c_char, 2 as libc::c_int,
                0 as libc::c_int as size_t);
    }
    memset(&mut opts as *mut k_opts as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<k_opts>() as libc::c_ulong);
    opts.action = INIT_PW;
    memset(&mut k5 as *mut k5_data as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<k5_data>() as libc::c_ulong);
    set_com_err_hook(Some(extended_com_err_fn as
                              unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: errcode_t,
                                                   _: *const libc::c_char,
                                                   _: ::std::ffi::VaList)
                                  -> ()));
    parse_options(argc, argv, &mut opts);
    if k5_begin(&mut opts, &mut k5) != 0 {
        authed_k5 = k5_kinit(&mut opts, &mut k5)
    }
    if authed_k5 != 0 && opts.verbose != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Authenticated to Kerberos v5\n\x00" as *const u8 as
                             *const libc::c_char));
    }
    k5_end(&mut k5);
    if authed_k5 == 0 { exit(1 as libc::c_int); }
    return 0 as libc::c_int;
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
