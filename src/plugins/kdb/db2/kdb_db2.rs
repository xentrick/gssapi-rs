use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:54"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
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
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:54"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "85:1"]
    pub type off_t = __off_t;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    #[c2rust::src_loc = "160:1"]
    pub type u_int32_t = __uint32_t;
    use super::types_h::{__u_int, __u_long, __off_t, __ssize_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:54"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:54"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:54"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:54"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:54"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:54"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:54"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:54"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:54"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:54"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:54"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    use super::pthreadtypes_h::pthread_mutex_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "424:1"]
        pub fn krb5int_mutex_lock(_: *mut k5_mutex_t);
        #[no_mangle]
        #[c2rust::src_loc = "425:1"]
        pub fn krb5int_mutex_unlock(_: *mut k5_mutex_t);
    }
    /* multiple inclusion? */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:54"]
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
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* * Structure for auth data */
    #[c2rust::src_loc = "1946:1"]
    pub type krb5_authdata = _krb5_authdata;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1946:16"]
    pub struct _krb5_authdata {
        pub magic: krb5_magic,
        pub ad_type: krb5_authdatatype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
    /* * C representation of KDC-REQ protocol message, including KDC-REQ-BODY */
    #[c2rust::src_loc = "2054:1"]
    pub type krb5_kdc_req = _krb5_kdc_req;
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
 * Ticket structure.
 *
 * The C representation of the ticket message, with a pointer to the
 * C representation of the encrypted part.
 */
    #[c2rust::src_loc = "1979:1"]
    pub type krb5_ticket = _krb5_ticket;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1979:16"]
    pub struct _krb5_ticket {
        pub magic: krb5_magic,
        pub server: krb5_principal,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_tkt_part,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* * Encrypted part of ticket. */
    #[c2rust::src_loc = "1961:1"]
    pub type krb5_enc_tkt_part = _krb5_enc_tkt_part;
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
    /* to-be-encrypted portion */
    /* *< flags */
    /* *< session key: includes enctype */
    /* *< client name/realm */
    /* *< list of transited realms */
    /* *< auth, start, end, renew_till */
    /* *< array of ptrs to addresses */
    /* *< auth data */
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
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    /* *< Time at which KDC issued the initial ticket that corresponds to this ticket */
    /* XXX ? should ktime in KDC_REP == authtime
       in ticket? otherwise client can't get this */
    /* *< optional in ticket, if not present, use @a authtime */
    /* *< Ticket expiration time */
    /* *< Latest time at which renewal of ticket can be valid */
    /* * Structure for transited encoding */
    #[c2rust::src_loc = "1954:1"]
    pub type krb5_transited = _krb5_transited;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1954:16"]
    pub struct _krb5_transited {
        pub magic: krb5_magic,
        pub tr_type: krb5_octet,
        pub tr_contents: krb5_data,
    }
    #[c2rust::src_loc = "398:1"]
    pub type krb5_enc_data = _krb5_enc_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "398:16"]
    pub struct _krb5_enc_data {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub kvno: krb5_kvno,
        pub ciphertext: krb5_data,
    }
    /* *< Transited encoding type */
    /* *< Contents */
    /* * Pre-authentication data */
    #[c2rust::src_loc = "2038:1"]
    pub type krb5_pa_data = _krb5_pa_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* *< Preauthentication data type */
        /* *< Length of data */
        /* *< Data */
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
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "7926:1"]
        pub fn krb5_prepend_error_message(ctx: krb5_context,
                                          code: krb5_error_code,
                                          fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "8043:1"]
        pub fn krb5_clear_error_message(ctx: krb5_context);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:54"]
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
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_context, krb5_error_code, krb5_data};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::kdb5_h::_kdb5_dal_handle;
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
        #[no_mangle]
        #[c2rust::src_loc = "614:1"]
        pub fn krb5_lock_file(_: krb5_context, _: libc::c_int, _: libc::c_int)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:54"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:54"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/kdb/kdb5.h:64"]
pub mod kdb5_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct _kdb5_dal_handle {
        pub db_context: *mut libc::c_void,
        pub lib_handle: db_library,
        pub master_keylist: *mut krb5_keylist_node,
        pub master_princ: krb5_principal,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    #[c2rust::src_loc = "19:1"]
    pub type db_library = *mut _db_library;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "19:16"]
    pub struct _db_library {
        pub name: [libc::c_char; 128],
        pub reference_cnt: libc::c_int,
        pub dl_dir_handle: plugin_dir_handle,
        pub vftabl: kdb_vftabl,
        pub next: *mut _db_library,
        pub prev: *mut _db_library,
    }
    use super::kdb_h::{krb5_keylist_node, kdb_vftabl};
    use super::krb5_h::krb5_principal;
    use super::k5_plugin_h::plugin_dir_handle;
    /* end of _KRB5_KDB5_H_ */
    /* typedef kdb5_dal_handle is in k5-int.h now */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:64"]
pub mod kdb_h {
    #[c2rust::src_loc = "294:1"]
    pub type krb5_keylist_node = _krb5_keylist_node;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "294:16"]
    pub struct _krb5_keylist_node {
        pub keyblock: krb5_keyblock,
        pub kvno: krb5_kvno,
        pub next: *mut _krb5_keylist_node,
    }
    /*
 * This number indicates the date of the last incompatible change to the DAL.
 * The maj_ver field of the module's vtable structure must match this version.
 */
    /*
 * A krb5_context can hold one database object.  Modules should use
 * krb5_db_set_context and krb5_db_get_context to store state associated with
 * the database object.
 *
 * Some module functions are mandatory for KDC operation; others are optional
 * or apply only to administrative operations.  If a function is optional, a
 * module can leave the function pointer as NULL.  Alternatively, modules can
 * return KRB5_PLUGIN_OP_NOTSUPP when asked to perform an inapplicable action.
 *
 * Some module functions have default implementations which will call back into
 * the vtable interface.  Leave these functions as NULL to use the default
 * implementations.
 *
 * The documentation in these comments describes the DAL as it is currently
 * implemented and used, not as it should be.  So if anything seems off, that
 * probably means the current state of things is off.
 *
 * Modules must allocate memory for principal entries, policy entries, and
 * other structures using an allocator compatible with malloc() as seen by
 * libkdb5 and libkrb5.  Modules may link against libkdb5 and call
 * krb5_db_alloc() to be certain that the same malloc implementation is used.
 */
    #[c2rust::src_loc = "923:1"]
    pub type kdb_vftabl = _kdb_vftabl;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "923:16"]
    pub struct _kdb_vftabl {
        pub maj_ver: libc::c_short,
        pub min_ver: libc::c_short,
        pub init_library: Option<unsafe extern "C" fn() -> krb5_error_code>,
        pub fini_library: Option<unsafe extern "C" fn() -> krb5_error_code>,
        pub init_module: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: *mut libc::c_char,
                                                     _:
                                                         *mut *mut libc::c_char,
                                                     _: libc::c_int)
                                    -> krb5_error_code>,
        pub fini_module: Option<unsafe extern "C" fn(_: krb5_context)
                                    -> krb5_error_code>,
        pub create: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: *mut libc::c_char,
                                                _: *mut *mut libc::c_char)
                               -> krb5_error_code>,
        pub destroy: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _: *mut *mut libc::c_char)
                                -> krb5_error_code>,
        pub get_age: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _: *mut time_t)
                                -> krb5_error_code>,
        pub lock: Option<unsafe extern "C" fn(_: krb5_context, _: libc::c_int)
                             -> krb5_error_code>,
        pub unlock: Option<unsafe extern "C" fn(_: krb5_context)
                               -> krb5_error_code>,
        pub get_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _:
                                                           krb5_const_principal,
                                                       _: libc::c_uint,
                                                       _:
                                                           *mut *mut krb5_db_entry)
                                      -> krb5_error_code>,
        pub put_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: *mut krb5_db_entry,
                                                       _:
                                                           *mut *mut libc::c_char)
                                      -> krb5_error_code>,
        pub delete_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_const_principal)
                                         -> krb5_error_code>,
        pub rename_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_const_principal,
                                                          _:
                                                              krb5_const_principal)
                                         -> krb5_error_code>,
        pub iterate: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _:
                                                     Option<unsafe extern "C" fn(_:
                                                                                     krb5_pointer,
                                                                                 _:
                                                                                     *mut krb5_db_entry)
                                                                ->
                                                                    libc::c_int>,
                                                 _: krb5_pointer,
                                                 _: krb5_flags)
                                -> krb5_error_code>,
        pub create_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: osa_policy_ent_t)
                                      -> krb5_error_code>,
        pub get_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut libc::c_char,
                                                    _: *mut osa_policy_ent_t)
                                   -> krb5_error_code>,
        pub put_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: osa_policy_ent_t)
                                   -> krb5_error_code>,
        pub iter_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: *mut libc::c_char,
                                                     _:
                                                         osa_adb_iter_policy_func,
                                                     _: *mut libc::c_void)
                                    -> krb5_error_code>,
        pub delete_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: *mut libc::c_char)
                                      -> krb5_error_code>,
        pub fetch_master_key: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _: krb5_principal,
                                                          _:
                                                              *mut krb5_keyblock,
                                                          _: *mut krb5_kvno,
                                                          _:
                                                              *mut libc::c_char)
                                         -> krb5_error_code>,
        pub fetch_master_key_list: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   krb5_principal,
                                                               _:
                                                                   *const krb5_keyblock,
                                                               _:
                                                                   *mut *mut krb5_keylist_node)
                                              -> krb5_error_code>,
        pub store_master_key_list: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   krb5_principal,
                                                               _:
                                                                   *mut krb5_keylist_node,
                                                               _:
                                                                   *mut libc::c_char)
                                              -> krb5_error_code>,
        pub dbe_search_enctype: Option<unsafe extern "C" fn(_: krb5_context,
                                                            _:
                                                                *mut krb5_db_entry,
                                                            _:
                                                                *mut krb5_int32,
                                                            _: krb5_int32,
                                                            _: krb5_int32,
                                                            _: krb5_int32,
                                                            _:
                                                                *mut *mut krb5_key_data)
                                           -> krb5_error_code>,
        pub change_pwd: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut krb5_keyblock,
                                                    _:
                                                        *mut krb5_key_salt_tuple,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_char,
                                                    _: libc::c_int,
                                                    _: krb5_boolean,
                                                    _: *mut krb5_db_entry)
                                   -> krb5_error_code>,
        pub promote_db: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut libc::c_char,
                                                    _: *mut *mut libc::c_char)
                                   -> krb5_error_code>,
        pub decrypt_key_data: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_key_data,
                                                          _:
                                                              *mut krb5_keyblock,
                                                          _:
                                                              *mut krb5_keysalt)
                                         -> krb5_error_code>,
        pub encrypt_key_data: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_keysalt,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut krb5_key_data)
                                         -> krb5_error_code>,
        pub sign_authdata: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: libc::c_uint,
                                                       _:
                                                           krb5_const_principal,
                                                       _:
                                                           krb5_const_principal,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: krb5_timestamp,
                                                       _:
                                                           *mut *mut krb5_authdata,
                                                       _: *mut libc::c_void,
                                                       _:
                                                           *mut *mut *mut krb5_data,
                                                       _:
                                                           *mut *mut *mut krb5_authdata)
                                      -> krb5_error_code>,
        pub check_transited_realms: Option<unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    *const krb5_data)
                                               -> krb5_error_code>,
        pub check_policy_as: Option<unsafe extern "C" fn(_: krb5_context,
                                                         _: *mut krb5_kdc_req,
                                                         _:
                                                             *mut krb5_db_entry,
                                                         _:
                                                             *mut krb5_db_entry,
                                                         _: krb5_timestamp,
                                                         _:
                                                             *mut *const libc::c_char,
                                                         _:
                                                             *mut *mut *mut krb5_pa_data)
                                        -> krb5_error_code>,
        pub check_policy_tgs: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *mut krb5_kdc_req,
                                                          _:
                                                              *mut krb5_db_entry,
                                                          _: *mut krb5_ticket,
                                                          _:
                                                              *mut *const libc::c_char,
                                                          _:
                                                              *mut *mut *mut krb5_pa_data)
                                         -> krb5_error_code>,
        pub audit_as_req: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: *mut krb5_kdc_req,
                                                      _: *const krb5_address,
                                                      _: *const krb5_address,
                                                      _: *mut krb5_db_entry,
                                                      _: *mut krb5_db_entry,
                                                      _: krb5_timestamp,
                                                      _: krb5_error_code)
                                     -> ()>,
        pub refresh_config: Option<unsafe extern "C" fn(_: krb5_context)
                                       -> ()>,
        pub check_allowed_to_delegate: Option<unsafe extern "C" fn(_:
                                                                       krb5_context,
                                                                   _:
                                                                       krb5_const_principal,
                                                                   _:
                                                                       *const krb5_db_entry,
                                                                   _:
                                                                       krb5_const_principal)
                                                  -> krb5_error_code>,
        pub free_principal_e_data: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_octet)
                                              -> ()>,
        pub get_s4u_x509_principal: Option<unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    krb5_const_principal,
                                                                _:
                                                                    libc::c_uint,
                                                                _:
                                                                    *mut *mut krb5_db_entry)
                                               -> krb5_error_code>,
        pub allowed_to_delegate_from: Option<unsafe extern "C" fn(_:
                                                                      krb5_context,
                                                                  _:
                                                                      krb5_const_principal,
                                                                  _:
                                                                      krb5_const_principal,
                                                                  _:
                                                                      *mut libc::c_void,
                                                                  _:
                                                                      *const krb5_db_entry)
                                                 -> krb5_error_code>,
        pub get_authdata_info: Option<unsafe extern "C" fn(_: krb5_context,
                                                           _: libc::c_uint,
                                                           _:
                                                               *mut *mut krb5_authdata,
                                                           _:
                                                               krb5_const_principal,
                                                           _:
                                                               krb5_const_principal,
                                                           _:
                                                               *mut krb5_keyblock,
                                                           _:
                                                               *mut krb5_keyblock,
                                                           _:
                                                               *mut krb5_db_entry,
                                                           _: krb5_timestamp,
                                                           _:
                                                               *mut *mut libc::c_void,
                                                           _:
                                                               *mut krb5_principal)
                                          -> krb5_error_code>,
        pub free_authdata_info: Option<unsafe extern "C" fn(_: krb5_context,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()>,
    }
    /*
     * Mandatory: Invoked after the module library is loaded, when the first DB
     * using the module is opened, across all contexts.
     */
    /*
     * Mandatory: Invoked before the module library is unloaded, after the last
     * DB using the module is closed, across all contexts.
     */
    /*
     * Mandatory: Initialize a database object.  Profile settings should be
     * read from conf_section inside KDB_MODULE_SECTION.  db_args communicates
     * command-line arguments for module-specific flags.  mode will be one of
     * KRB5_KDB_OPEN_{RW,RO} or'd with one of
     * KRB5_KDB_SRV_TYPE_{KDC,ADMIN,PASSWD,OTHER}.
     */
    /*
     * Mandatory: Finalize the database object contained in a context.  Free
     * any state contained in the db_context pointer and null it out.
     */
    /*
     * Optional: Initialize a database object while creating the underlying
     * database.  conf_section and db_args have the same meaning as in
     * init_module.  This function may return an error if the database already
     * exists.  Used by kdb5_util create.
     *
     * If db_args contains the value "temporary", the module should create an
     * exclusively locked side copy of the database suitable for loading in a
     * propagation from master to replica.  This side copy will later be
     * promoted with promote_db, allowing complete updates of the DB with no
     * loss in read availability.  If the module cannot comply with this
     * architecture, it should return an error.
     */
    /*
     * Optional: Destroy a database.  conf_section and db_args have the same
     * meaning as in init_module.  Used by kdb5_util destroy.  In current
     * usage, the database is destroyed while open, so the module should handle
     * that.
     */
    /*
     * Deprecated: No longer used as of krb5 1.10; can be removed in the next
     * DAL revision.  Modules should leave as NULL.
     */
    /*
     * Optional: Lock the database, with semantics depending on the mode
     * argument:
     *
     * KRB5_DB_LOCKMODE_SHARED: Lock may coexist with other shared locks.
     * KRB5_DB_LOCKMODE_EXCLUSIVE: Lock may not coexist with other locks.
     * KRB5_DB_LOCKMODE_PERMANENT: Exclusive lock surviving process exit.
     *
     * Used by the "kadmin lock" command, incremental propagation, and
     * kdb5_util dump.  Incremental propagation support requires shared locks
     * to operate.  kdb5_util dump will continue unlocked if the module returns
     * KRB5_PLUGIN_OP_NOTSUPP.
     */
    /* Optional: Release a lock created with db_lock. */
    /*
     * Mandatory: Set *entry to an allocated entry for the principal
     * search_for.  If the principal is not found, return KRB5_KDB_NOENTRY.
     *
     * The meaning of flags are as follows:
     *
     * KRB5_KDB_FLAG_CANONICALIZE: Set by the KDC when looking up entries for
     *     an AS or TGS request with canonicalization requested.  Determines
     *     whether the module should return out-of-realm referrals.
     *
     * KRB5_KDB_FLAG_INCLUDE_PAC: Set by the KDC during an AS request when the
     *     client requested PAC information during padata, and during most TGS
     *     requests.  Indicates that the module should include PAC information
     *     when its sign_authdata method is invoked.
     *
     * KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY: Set by the KDC when looking up the
     *     client entry in an AS request.  Affects how the module should return
     *     out-of-realm referrals.
     *
     * KRB5_KDB_FLAG_MAP_PRINCIPALS: Set by the KDC when looking up the client
     *     entry during TGS requests, except for S4U TGS requests and requests
     *     where the server entry has the KRB5_KDB_NO_AUTH_DATA_REQUIRED
     *     attribute.  Indicates that the module should map foreign principals
     *     to local principals if it supports doing so.
     *
     * KRB5_KDB_FLAG_PROTOCOL_TRANSITION: Set by the KDC when looking up the
     *     client entry during an S4U2Self TGS request.  This affects the PAC
     *     information which should be included when authorization data is
     *     generated; see the Microsoft S4U specification for details.
     *
     * KRB5_KDB_FLAG_CONSTRAINED_DELEGATION: Set by the KDC when looking up the
     *     client entry during an S4U2Proxy TGS request.  Also affects PAC
     *     generation.
     *
     * KRB5_KDB_FLAG_CROSS_REALM: Set by the KDC after looking up a server
     *     entry during a TGS request, if the header ticket was issued by a
     *     different realm.
     *
     * KRB5_KDB_FLAG_ISSUING_REFERRAL: Set by the KDC after looking up a server
     *     entry during a TGS request, if the requested server principal is not
     *     part of the realm being served, and a referral or alternate TGT will
     *     be issued instead.
     *
     * A module may return an in-realm alias by setting (*entry)->princ to the
     * canonical name.  The KDC will decide based on the request whether to use
     * the requested name or the canonical name in the issued ticket.
     *
     * A module can return a referral to another realm if
     * KRB5_KDB_FLAG_CANONICALIZE is set, or if
     * KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY is set and search_for->type is
     * KRB5_NT_ENTERPRISE_PRINCIPAL.  If KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY is
     * set, the module should return a referral by simply filling in an
     * out-of-realm name in (*entry)->princ and setting all other fields to
     * NULL.  Otherwise, the module should return the entry for the cross-realm
     * TGS of the referred-to realm.  For TGS referals, the module can also
     * include tl-data of type KRB5_TL_SERVER_REFERRAL containing ASN.1-encoded
     * Windows referral data as documented in
     * draft-ietf-krb-wg-kerberos-referrals-11 appendix A; this will be
     * returned to the client as encrypted padata.
     */
    /*
     * Optional: Create or modify a principal entry.  db_args communicates
     * command-line arguments for module-specific flags.
     *
     * The mask field of an entry indicates the changed fields.  Mask values
     * are defined in kadmin's admin.h header.  If KADM5_PRINCIPAL is set in
     * the mask, the entry is new; otherwise it already exists.  All fields of
     * an entry are expected to contain correct values, regardless of whether
     * they are specified in the mask, so it is acceptable for a module to
     * ignore the mask and update the entire entry.
     */
    /*
     * Optional: Delete the entry for the principal search_for.  If the
     * principal did not exist, return KRB5_KDB_NOENTRY.
     */
    /*
     * Optional with default: Rename a principal.  If the source principal does
     * not exist, return KRB5_KDB_NOENTRY.  If the target exists, return an
     * error.
     *
     * NOTE: If the module chooses to implement a custom function for renaming
     * a principal instead of using the default, then rename operations will
     * fail if iprop logging is enabled.
     */
    /*
     * Optional: For each principal entry in the database, invoke func with the
     * argments func_arg and the entry data.  If match_entry is specified, the
     * module may narrow the iteration to principal names matching that regular
     * expression; a module may alternatively ignore match_entry.
     */
    /*
     * Optional: Create a password policy entry.  Return an error if the policy
     * already exists.
     */
    /*
     * Optional: Set *policy to the policy entry of the specified name.  If the
     * entry does not exist, return KRB5_KDB_NOENTRY.
     */
    /*
     * Optional: Modify an existing password policy entry to match the values
     * in policy.  Return an error if the policy does not already exist.
     */
    /*
     * Optional: For each password policy entry in the database, invoke func
     * with the argments data and the entry data.  If match_entry is specified,
     * the module may narrow the iteration to policy names matching that
     * regular expression; a module may alternatively ignore match_entry.
     */
    /*
     * Optional: Delete the password policy entry with the name policy.  Return
     * an error if the entry does not exist.
     */
    /*
     * Optional with default: Retrieve a master keyblock from the stash file
     * db_args, filling in *key and *kvno.  mname is the name of the master
     * principal for the realm.
     *
     * The default implementation reads the master keyblock from a keytab or
     * old-format stash file.
     */
    /*
     * Optional with default: Given a keyblock for some version of the
     * database's master key, fetch the decrypted master key values from the
     * database and store the list into *mkeys_list.  The caller will free
     * *mkeys_list using a libkdb5 function which uses the standard free()
     * function, so the module must not use a custom allocator.
     *
     * The caller may not know the version number of the master key it has, in
     * which case it will pass IGNORE_VNO.
     *
     * The default implementation ignores kvno and tries the key against the
     * current master key data and all KRB5_TL_MKEY_AUX values, which contain
     * copies of the master keys encrypted with old master keys.
     */
    /*
     * Optional with default: Save a list of master keyblocks, obtained from
     * fetch_master_key_list, into the stash file db_arg.  The caller will set
     * master_pwd to NULL, so the module should just ignore it.  mname is the
     * name of the master principal for the realm.
     *
     * The default implementation saves the list of master keys in a
     * keytab-format file.
     */
    /*
     * Optional with default: Starting at position *start, scan the key data of
     * a database entry for a key matching the enctype ktype, the salt type
     * stype, and the version kvno.  Store the resulting key into *kdatap and
     * set *start to the position after the key found.  If ktype is negative,
     * match any enctype.  If stype is negative, match any salt type.  If kvno
     * is zero or negative, find the most recent key version satisfying the
     * other constraints.
     */
    /*
     * Optional with default: Change the key data for db_entry to include keys
     * derived from the password passwd in each of the specified key-salt
     * types, at version new_kvno.  Discard the old key data if keepold is not
     * set.
     *
     * The default implementation uses the keyblock master_key to encrypt each
     * new key, via the function encrypt_key_data.
     */
    /*
     * Optional: Promote a temporary database to be the live one.  context must
     * be initialized with an exclusively locked database created with the
     * "temporary" db_arg.  On success, the database object contained in
     * context will be finalized.
     *
     * This method is used by kdb5_util load to replace the live database with
     * minimal loss of read availability.
     */
    /*
     * Optional with default: Decrypt the key in key_data with master keyblock
     * mkey, placing the result into dbkey.  Copy the salt from key_data, if
     * any, into keysalt.  Either dbkey or keysalt may be left unmodified on
     * successful return if key_data does not contain key or salt information.
     *
     * The default implementation expects the encrypted key (in krb5_c_encrypt
     * format) to be stored in key_data_contents[0], with length given by
     * key_data_length[0].  If key_data_ver is 2, it expects the salt to be
     * stored, unencrypted, in key_data_contents[1], with length given by
     * key_data_length[1].
     */
    /*
     * Optional with default: Encrypt dbkey with master keyblock mkey, placing
     * the result into key_data along with keysalt.
     *
     * The default implementation stores the encrypted key (in krb5_c_encrypt
     * format) in key_data_contents[0] and the length in key_data_length[0].
     * If keysalt is specified, it sets key_data_ver to 2, and stores the salt
     * in key_data_contents[1] and its length in key_data_length[1].  If
     * keysalt is not specified, key_data_ver is set to 1.
     */
    /*
     * Optional: Generate signed authorization data, such as a Windows PAC, for
     * the ticket to be returned to the client.  Place the signed authorization
     * data, if any, in *signed_auth_data.  This function will be invoked for
     * an AS request if the client included padata requesting a PAC.  This
     * function will be invoked for a TGS request if there is authorization
     * data in the TGT, if the client is from another realm, or if the TGS
     * request is an S4U2Self or S4U2Proxy request.  This function will not be
     * invoked during TGS requests if the server principal has the
     * no_auth_data_required attribute set.  Input parameters are:
     *
     *   flags: The flags used to look up the client principal.
     *
     *   client_princ: For S4U2Self and S4U2Proxy TGS requests, the client
     *     principal requested by the service; for regular TGS requests, the
     *     possibly-canonicalized client principal.
     *
     *   server_princ: The server principal in the request.
     *
     *   client: The DB entry of the client if it is in the local realm, NULL
     *     if not.  For S4U2Self and S4U2Proxy TGS requests, this is the DB
     *     entry for the client principal requested by the service.
     *
     *   server: The DB entry of the service principal, or of a cross-realm
     *     krbtgt principal in case of referral.
     *
     *   header_server: For S4U2Proxy requests, the DB entry of the second
     *     ticket server.  For other TGS requests, the DB entry of the header
     *     ticket server.  For AS requests, NULL.
     *
     *   local_tgt: the DB entry of the local krbtgt principal.
     *
     *   client_key: The reply key for the KDC request, before any FAST armor
     *     is applied.  For AS requests, this may be the client's long-term key
     *     or a key chosen by a preauth mechanism.  For TGS requests, this may
     *     be the subkey found in the AP-REQ or the session key of the TGT.
     *
     *   server_key: The server key used to encrypt the returned ticket.
     *
     *   header_key: For S4U2Proxy requests, the key used to decrypt the second
     *     ticket.  For TGS requests, the key used to decrypt the header
     *     ticket.  For AS requests, NULL.
     *
     *   local_tgt_key: The decrypted first key of local_tgt.
     *
     *   session_key: The session key of the ticket being granted to the
     *     requestor.
     *
     *   authtime: The timestamp of the original client authentication time.
     *     For AS requests, this is the current time.  For TGS requests, this
     *     is the authtime of the subject ticket (TGT or S4U2Proxy evidence
     *     ticket).
     *
     *   tgt_auth_data: For TGS requests, the authorization data present in the
     *     subject ticket.  For AS requests, NULL.
     *
     *   ad_info: For TGS requests, the parsed authorization data if obtained
     *     by get_authdata_info method from the authorization data present in
     *     the subject ticket.  Otherwise NULL.
     *
     *   auth_indicators: Points to NULL or a null-terminated list of krb5_data
     *     pointers, each containing an authentication indicator (RFC 8129).
     *     The method may modify this list, or free it and replace
     *     *auth_indicators with NULL, to change which auth indicators will be
     *     included in the ticket.
     */
    /*
     * Optional: Perform a policy check on a cross-realm ticket's transited
     * field.  Return 0 if the check authoritatively succeeds,
     * KRB5_PLUGIN_NO_HANDLE to use the core transited-checking mechanisms, or
     * another error (other than KRB5_PLUGIN_OP_NOTSUPP) if the check fails.
     */
    /*
     * Optional: Perform a policy check on an AS request, in addition to the
     * standard policy checks.  Return 0 if the AS request is allowed.  If the
     * AS request is not allowed:
     *   - Place a short string literal into *status.
     *   - If desired, place data into e_data.  Any data placed here will be
     *     freed by the caller using the standard free function.
     *   - Return an appropriate error (such as KRB5KDC_ERR_POLICY).
     */
    /*
     * Optional: Perform a policy check on a TGS request, in addition to the
     * standard policy checks.  Return 0 if the TGS request is allowed.  If the
     * TGS request is not allowed:
     *   - Place a short string literal into *status.
     *   - If desired, place data into e_data.  Any data placed here will be
     *     freed by the caller using the standard free function.
     *   - Return an appropriate error (such as KRB5KDC_ERR_POLICY).
     * The input parameter ticket contains the TGT used in the TGS request.
     */
    /*
     * Optional: This method informs the module of a successful or unsuccessful
     * AS request.
     */
    /* Note: there is currently no method for auditing TGS requests. */
    /*
     * Optional: This method informs the module of a request to reload
     * configuration or other state (that is, the KDC received a SIGHUP).
     */
    /*
     * Optional: Perform a policy check on server being allowed to obtain
     * tickets from client to proxy.  (Note that proxy is the target of the
     * delegation, not the delegating service; the term "proxy" is from the
     * viewpoint of the delegating service asking another service to perform
     * some of its work in the authentication context of the client.  This
     * terminology comes from the Microsoft S4U protocol documentation.)
     * Return 0 if policy allows it, or an appropriate error (such as
     * KRB5KDC_ERR_POLICY) if not.  If this method is not implemented, all
     * S4U2Proxy delegation requests will be rejected.
     */
    /*
     * Optional: Free the e_data pointer of a database entry.  If this method
     * is not implemented, the e_data pointer in principal entries will be
     * freed with free() as seen by libkdb5.
     */
    /*
     * Optional: get a principal entry for S4U2Self based on X509 certificate.
     *
     * If flags include KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY, princ->realm
     * indicates the request realm, but the data components should be ignored.
     * The module can return an out-of-realm client referral as it would for
     * get_principal().
     *
     * If flags does not include KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY, princ is
     * from PA-S4U-X509-USER.  If it contains data components (and not just a
     * realm), the module should verify that it is the same as the lookup
     * result for client_cert.  The module should not return a referral.
     */
    /*
     * Optional: Perform a policy check on server being allowed to obtain
     * tickets from client to proxy.  This method is similar to
     * check_allowed_to_delegate, but it operates on the target server DB entry
     * (called "proxy" here as in Microsoft's protocol documentation) rather
     * than the intermediate server entry.  server_ad_info represents the
     * authdata of the intermediate server, as returned by the
     * get_authdata_info method on the header ticket.  Return 0 if policy
     * allows the delegation, or an appropriate error (such as
     * KRB5KDC_ERR_POLICY) if not.
     *
     * This method is called for S4U2Proxy requests and implements the
     * resource-based constrained delegation variant, which can support
     * cross-realm delegation.  If this method is not implemented or if it
     * returns a policy error, the KDC will fall back to
     * check_allowed_to_delegate if the intermediate and target servers are in
     * the same realm and the evidence ticket is forwardable.
     */
    /*
     * Optional: Perform verification and policy checks on authorization data,
     * such as a Windows PAC, based on the request client lookup flags.  Return
     * 0 if all checks have passed.  Optionally return a representation of the
     * authdata in *ad_info_out, to be consumed by allowed_to_delegate_from and
     * sign_authdata.  If client_out is not NULL, set *client_out to the client
     * name in the PAC; this indicates the requested client principal for a
     * cross-realm S4U2Proxy request.
     *
     * This method is called for TGS requests on the authorization data from
     * the header ticket.  For S4U2Proxy requests it is also called on the
     * authorization data from the evidence ticket.  If the
     * KRB5_KDB_FLAG_PROTOCOL_TRANSITION bit is set in flags, the authdata is
     * from the header ticket of an S4U2Self referral request, and the supplied
     * client_princ is the requested client.
     */
    /* End of minor version 0 for major version 8. */
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
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
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
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    /* NOT saved */
    /* # of array elements */
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "237:1"]
    pub type osa_adb_iter_policy_func
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: osa_policy_ent_t)
                   -> ()>;
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_t = *mut _osa_policy_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _osa_policy_ent_t {
        pub version: libc::c_int,
        pub name: *mut libc::c_char,
        pub pw_min_life: krb5_ui_4,
        pub pw_max_life: krb5_ui_4,
        pub pw_min_length: krb5_ui_4,
        pub pw_min_classes: krb5_ui_4,
        pub pw_history_num: krb5_ui_4,
        pub policy_refcnt: krb5_ui_4,
        pub pw_max_fail: krb5_ui_4,
        pub pw_failcnt_interval: krb5_ui_4,
        pub pw_lockout_duration: krb5_ui_4,
        pub attributes: krb5_ui_4,
        pub max_life: krb5_ui_4,
        pub max_renewable_life: krb5_ui_4,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    use super::krb5_h::{krb5_keyblock, krb5_kvno, krb5_error_code,
                        krb5_context, krb5_const_principal, krb5_pointer,
                        krb5_flags, krb5_principal, krb5_int32, krb5_boolean,
                        krb5_timestamp, krb5_authdata, krb5_data,
                        krb5_kdc_req, krb5_pa_data, krb5_ticket, krb5_address,
                        krb5_octet, krb5_magic, krb5_ui_2, krb5_ui_4,
                        krb5_deltat, krb5_int16, krb5_enctype};
    use super::time_t_h::time_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn krb5_db_free_principal(kcontext: krb5_context,
                                      entry: *mut krb5_db_entry);
    }
    /* Length, data */
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:54"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    extern "C" {
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
        #[c2rust::src_loc = "89:1"]
        pub fn profile_get_boolean(profile: profile_t,
                                   name: *const libc::c_char,
                                   subname: *const libc::c_char,
                                   subsubname: *const libc::c_char,
                                   def_val: libc::c_int,
                                   ret_default: *mut libc::c_int)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn profile_release_string(str: *mut libc::c_char);
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/kdb_db2.h:65"]
pub mod kdb_db2_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/kdb/db2/kdb_db2.h */
/*
 * Copyright 1997 by the Massachusetts Institute of Technology.
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
 * KDC Database backend definitions for Berkely DB.
 */
    #[c2rust::src_loc = "36:1"]
    pub type krb5_db2_context = _krb5_db2_context;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:16"]
    pub struct _krb5_db2_context {
        pub db_inited: krb5_boolean,
        pub db_name: *mut libc::c_char,
        pub db: *mut DB,
        pub hashfirst: krb5_boolean,
        pub db_lf_name: *mut libc::c_char,
        pub db_lf_file: libc::c_int,
        pub db_locks_held: libc::c_int,
        pub db_lock_mode: libc::c_int,
        pub db_nb_locks: krb5_boolean,
        pub policy_db: osa_adb_policy_t,
        pub tempdb: krb5_boolean,
        pub disable_last_success: krb5_boolean,
        pub disable_lockout: krb5_boolean,
        pub unlockiter: krb5_boolean,
    }
    use super::krb5_h::{krb5_boolean, krb5_context, krb5_timestamp,
                        krb5_error_code};
    use super::db_h::DB;
    use super::policy_db_h::osa_adb_policy_t;
    use super::k5_int_h::_krb5_context;
    use super::kdb_h::krb5_db_entry;
    use super::k5_thread_h::k5_mutex_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "123:1"]
        pub fn krb5_db2_lockout_audit(context: krb5_context,
                                      entry: *mut krb5_db_entry,
                                      stamp: krb5_timestamp,
                                      status: krb5_error_code)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "118:1"]
        pub fn krb5_db2_lockout_check_policy(context: krb5_context,
                                             entry: *mut krb5_db_entry,
                                             stamp: krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "115:20"]
        pub static mut krb5_db2_mutex: *mut k5_mutex_t;
    }
    /* Context initialized          */
    /* Name of database             */
    /* DB handle                    */
    /* Try hash database type first */
    /* Name of lock file            */
    /* File descriptor of lock file */
    /* Number of times locked       */
    /* Last lock mode, e.g. greatest*/
    /* [Non]Blocking lock modes     */
    /* KRB5_KDB_DB2_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/policy_db.h:65"]
pub mod policy_db_h {
    #[c2rust::src_loc = "63:1"]
    pub type osa_adb_policy_t = *mut _osa_adb_db_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:16"]
    pub struct _osa_adb_db_ent_t {
        pub magic: libc::c_int,
        pub db: *mut DB,
        pub info: HASHINFO,
        pub btinfo: BTREEINFO,
        pub filename: *mut libc::c_char,
        pub lock: osa_adb_lock_t,
        pub opencnt: libc::c_int,
    }
    #[c2rust::src_loc = "56:1"]
    pub type osa_adb_lock_t = *mut _osa_adb_db_lock_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:16"]
    pub struct _osa_adb_db_lock_ent_t {
        pub lockfile: *mut FILE,
        pub filename: *mut libc::c_char,
        pub refcnt: libc::c_int,
        pub lockmode: libc::c_int,
        pub lockcnt: libc::c_int,
        pub context: krb5_context,
    }
    #[c2rust::src_loc = "63:1"]
    pub type osa_adb_db_t = *mut _osa_adb_db_ent_t;
    use super::db_h::{DB, HASHINFO, BTREEINFO};
    use super::FILE_h::FILE;
    use super::krb5_h::{krb5_context, krb5_error_code};
    use super::kdb_h::{_osa_policy_ent_t, osa_policy_ent_t,
                       osa_adb_iter_policy_func};
    extern "C" {
        /*
 * Return Code (the rest are in adb_err.h)
 */
        /*
 * Functions
 */
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn osa_adb_create_db(filename: *mut libc::c_char,
                                 lockfile: *mut libc::c_char,
                                 magic: libc::c_int) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn osa_adb_destroy_db(filename: *mut libc::c_char,
                                  lockfile: *mut libc::c_char,
                                  magic: libc::c_int) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "85:1"]
        pub fn osa_adb_init_db(dbp: *mut osa_adb_db_t,
                               filename: *mut libc::c_char,
                               lockfile: *mut libc::c_char,
                               magic: libc::c_int) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "87:1"]
        pub fn osa_adb_fini_db(db: osa_adb_db_t, magic: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn osa_adb_get_lock(db: osa_adb_db_t, mode: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn osa_adb_release_lock(db: osa_adb_db_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "92:1"]
        pub fn osa_adb_create_policy(db: osa_adb_policy_t,
                                     entry: osa_policy_ent_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn osa_adb_destroy_policy(db: osa_adb_policy_t,
                                      name: *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "96:1"]
        pub fn osa_adb_get_policy(db: osa_adb_policy_t,
                                  name: *mut libc::c_char,
                                  entry: *mut osa_policy_ent_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "98:1"]
        pub fn osa_adb_put_policy(db: osa_adb_policy_t,
                                  entry: osa_policy_ent_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn osa_adb_iter_policy(db: osa_adb_policy_t,
                                   func: osa_adb_iter_policy_func,
                                   data: *mut libc::c_void)
         -> krb5_error_code;
    }
    /* __ADB_H__ */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/db.h:60"]
pub mod db_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:9"]
    pub struct BTREEINFO {
        pub flags: u_long,
        pub cachesize: u_int,
        pub maxkeypage: libc::c_int,
        pub minkeypage: libc::c_int,
        pub psize: u_int,
        pub compare: Option<unsafe extern "C" fn(_: *const DBT, _: *const DBT)
                                -> libc::c_int>,
        pub prefix: Option<unsafe extern "C" fn(_: *const DBT, _: *const DBT)
                               -> size_t>,
        pub lorder: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:9"]
    pub struct DBT {
        pub data: *mut libc::c_void,
        pub size: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:9"]
    pub struct HASHINFO {
        pub bsize: u_int,
        pub ffactor: u_int,
        pub nelem: u_int,
        pub cachesize: u_int,
        pub hash: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                              _: size_t) -> u_int32_t>,
        pub lorder: libc::c_int,
    }
    /*
 * !!!
 * The following flags are included in the dbopen(3) call as part of the
 * open(2) flags.  In order to avoid conflicts with the open flags, start
 * at the top of the 16 or 32-bit number space and work our way down.  If
 * the open flags were significantly expanded in the future, it could be
 * a problem.  Wish I'd left another flags word in the dbopen call.
 *
 * !!!
 * None of this stuff is implemented yet.  The only reason that it's here
 * is so that the access methods can skip copying the key/data pair when
 * the DB_LOCK flag isn't set.
 */
    /* Do locking. */
    /* Use shared memory. */
    /* Do transactions. */
    /* deal with turning prototypes on and off */
    /* no __P from system */
    /* Access method description structure. */
    #[c2rust::src_loc = "119:1"]
    pub type DB = __db;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "119:16"]
    pub struct __db {
        pub type_0: DBTYPE,
        pub close: Option<unsafe extern "C" fn(_: *mut __db) -> libc::c_int>,
        pub del: Option<unsafe extern "C" fn(_: *const __db, _: *const DBT,
                                             _: u_int) -> libc::c_int>,
        pub get: Option<unsafe extern "C" fn(_: *const __db, _: *const DBT,
                                             _: *mut DBT, _: u_int)
                            -> libc::c_int>,
        pub put: Option<unsafe extern "C" fn(_: *const __db, _: *mut DBT,
                                             _: *const DBT, _: u_int)
                            -> libc::c_int>,
        pub seq: Option<unsafe extern "C" fn(_: *const __db, _: *mut DBT,
                                             _: *mut DBT, _: u_int)
                            -> libc::c_int>,
        pub sync: Option<unsafe extern "C" fn(_: *const __db, _: u_int)
                             -> libc::c_int>,
        pub internal: *mut libc::c_void,
        pub fd: Option<unsafe extern "C" fn(_: *const __db) -> libc::c_int>,
    }
    #[c2rust::src_loc = "83:9"]
    pub type DBTYPE = libc::c_uint;
    #[c2rust::src_loc = "83:35"]
    pub const DB_RECNO: DBTYPE = 2;
    #[c2rust::src_loc = "83:26"]
    pub const DB_HASH: DBTYPE = 1;
    #[c2rust::src_loc = "83:16"]
    pub const DB_BTREE: DBTYPE = 0;
    use super::sys_types_h::{u_long, u_int, u_int32_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "187:1"]
        pub fn kdb2_dbopen(_: *const libc::c_char, _: libc::c_int,
                           _: libc::c_int, _: DBTYPE, _: *const libc::c_void)
         -> *mut DB;
    }
    /* Underlying db type. */
    /* Access method private. */
    /* !_DB_H_ */
}
#[c2rust::header_src = "/usr/include/utime.h:63"]
pub mod utime_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:8"]
    pub struct utimbuf {
        pub actime: __time_t,
        pub modtime: __time_t,
    }
    use super::types_h::__time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn utime(__file: *const libc::c_char,
                     __file_times: *const utimbuf) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:54"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:54"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "148:1"]
        pub fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:54"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:54"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:54"]
pub mod unistd_h {
    use super::types_h::__off_t;
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "334:1"]
        pub fn lseek(__fd: libc::c_int, __offset: __off_t,
                     __whence: libc::c_int) -> __off_t;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void,
                    __nbytes: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "825:1"]
        pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "954:1"]
        pub fn fsync(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/time.h:54"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:54"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:54"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
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
#[c2rust::header_src = "/usr/include/sys/stat.h:54"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/db2/kdb_xdr.h:66"]
pub mod kdb_xdr_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_data, krb5_principal_data,
                        krb5_const_principal, krb5_error_code};
    use super::kdb_h::krb5_db_entry;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
        #[no_mangle]
        #[c2rust::src_loc = "7:1"]
        pub fn krb5_encode_princ_dbkey(context: krb5_context,
                                       key: *mut krb5_data,
                                       principal: krb5_const_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "11:1"]
        pub fn krb5_decode_princ_entry(context: krb5_context,
                                       content: *mut krb5_data,
                                       entry: *mut *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "18:1"]
        pub fn krb5_encode_princ_entry(context: krb5_context,
                                       content: *mut krb5_data,
                                       entry: *mut krb5_db_entry)
         -> krb5_error_code;
    }
}
pub use self::types_h::{__u_int, __u_long, __uint8_t, __int16_t, __uint16_t,
                        __int32_t, __uint32_t, __dev_t, __uid_t, __gid_t,
                        __ino_t, __mode_t, __nlink_t, __off_t, __off64_t,
                        __time_t, __blksize_t, __blkcnt_t, __ssize_t,
                        __syscall_slong_t};
pub use self::sys_types_h::{u_int, u_long, off_t, ssize_t, u_int32_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::struct_timespec_h::timespec;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stat_h::stat;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, krb5int_mutex_lock,
                            krb5int_mutex_unlock};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_authdatatype,
                       krb5_preauthtype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, krb5_keyblock,
                       _krb5_keyblock, krb5_authdata, _krb5_authdata,
                       krb5_kdc_req, _krb5_kdc_req, krb5_ticket, _krb5_ticket,
                       krb5_enc_tkt_part, _krb5_enc_tkt_part,
                       krb5_ticket_times, _krb5_ticket_times, krb5_transited,
                       _krb5_transited, krb5_enc_data, _krb5_enc_data,
                       krb5_pa_data, _krb5_pa_data, _profile_t,
                       krb5_free_data_contents, krb5_set_error_message,
                       krb5_prepend_error_message, krb5_clear_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, make_data, k5calloc, k5alloc,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         krb5_lock_file};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::kdb5_h::{_kdb5_dal_handle, db_library, _db_library};
pub use self::kdb_h::{krb5_keylist_node, _krb5_keylist_node, kdb_vftabl,
                      _kdb_vftabl, krb5_db_entry, _krb5_db_entry_new,
                      krb5_key_data, _krb5_key_data, krb5_tl_data,
                      _krb5_tl_data, krb5_keysalt, _krb5_keysalt,
                      krb5_key_salt_tuple, __krb5_key_salt_tuple,
                      osa_adb_iter_policy_func, osa_policy_ent_t,
                      _osa_policy_ent_t, krb5_db_free_principal};
pub use self::profile_h::{profile_t, profile_get_string, profile_get_boolean,
                          profile_release_string};
pub use self::kdb_db2_h::{krb5_db2_context, _krb5_db2_context,
                          krb5_db2_lockout_audit,
                          krb5_db2_lockout_check_policy, krb5_db2_mutex};
pub use self::policy_db_h::{osa_adb_policy_t, _osa_adb_db_ent_t,
                            osa_adb_lock_t, _osa_adb_db_lock_ent_t,
                            osa_adb_db_t, osa_adb_create_db,
                            osa_adb_destroy_db, osa_adb_init_db,
                            osa_adb_fini_db, osa_adb_get_lock,
                            osa_adb_release_lock, osa_adb_create_policy,
                            osa_adb_destroy_policy, osa_adb_get_policy,
                            osa_adb_put_policy, osa_adb_iter_policy};
pub use self::db_h::{BTREEINFO, DBT, HASHINFO, DB, __db, DBTYPE, DB_RECNO,
                     DB_HASH, DB_BTREE, kdb2_dbopen};
pub use self::utime_h::{utimbuf, utime};
use self::stdlib_h::{calloc, free, malloc};
use self::stdio_h::{rename, asprintf};
use self::fcntl_h::{fcntl, open};
use self::errno_h::__errno_location;
use self::unistd_h::{lseek, close, read, write, unlink, fsync};
use self::time_h::time;
use self::libintl_h::dgettext;
use self::string_h::{strchr, strdup, strcmp, memset, memcpy};
use self::sys_stat_h::fstat;
use self::kdb_xdr_h::{krb5_encode_princ_dbkey, krb5_decode_princ_entry,
                      krb5_encode_princ_entry};
#[c2rust::src_loc = "930:1"]
pub type ctx_iterate_cb
    =
    Option<unsafe extern "C" fn(_: krb5_pointer, _: *mut krb5_db_entry)
               -> krb5_error_code>;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "933:16"]
pub struct iter_curs {
    pub key: DBT,
    pub data: DBT,
    pub keycopy: DBT,
    pub startflag: libc::c_uint,
    pub stepflag: libc::c_uint,
    pub ctx: krb5_context,
    pub dbc: *mut krb5_db2_context,
    pub lockmode: libc::c_int,
    pub islocked: krb5_boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1350:8"]
pub struct nra_context {
    pub kcontext: krb5_context,
    pub db_context: *mut krb5_db2_context,
}
/*
 * Locking:
 *
 * There are two distinct locking protocols used.  One is designed to
 * lock against processes (the admin_server, for one) which make
 * incremental changes to the database; the other is designed to lock
 * against utilities (kdb5_edit, kpropd, kdb5_convert) which replace the
 * entire database in one fell swoop.
 *
 * The first locking protocol is implemented using flock() in the
 * krb_dbl_lock() and krb_dbl_unlock routines.
 *
 * The second locking protocol is necessary because DBM "files" are
 * actually implemented as two separate files, and it is impossible to
 * atomically rename two files simultaneously.  It assumes that the
 * database is replaced only very infrequently in comparison to the time
 * needed to do a database read operation.
 *
 * A third file is used as a "version" semaphore; the modification
 * time of this file is the "version number" of the database.
 * At the start of a read operation, the reader checks the version
 * number; at the end of the read operation, it checks again.  If the
 * version number changed, or if the semaphore was nonexistant at
 * either time, the reader sleeps for a second to let things
 * stabilize, and then tries again; if it does not succeed after
 * KRB5_DBM_MAX_RETRY attempts, it gives up.
 *
 * On update, the semaphore file is deleted (if it exists) before any
 * update takes place; at the end of the update, it is replaced, with
 * a version number strictly greater than the version number which
 * existed at the start of the update.
 *
 * If the system crashes in the middle of an update, the semaphore
 * file is not automatically created on reboot; this is a feature, not
 * a bug, since the database may be inconsistant.  Note that the
 * absence of a semaphore file does not prevent another _update_ from
 * taking place later.  Database replacements take place automatically
 * only on replica servers; a crash in the middle of an update will be
 * fixed by the next propagation.  A crash in the middle of an on the
 * master would be somewhat more serious, but this would likely be
 * noticed by an administrator, who could fix the problem and retry
 * the operation.
 */
/* Evaluate to true if the krb5_context c contains an initialized db2
 * context. */
#[c2rust::src_loc = "126:1"]
unsafe extern "C" fn get_db_opt(mut input: *mut libc::c_char,
                                mut opt: *mut *mut libc::c_char,
                                mut val: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut pos: *mut libc::c_char = strchr(input, '=' as i32);
    if pos.is_null() {
        *opt = 0 as *mut libc::c_char;
        *val = strdup(input);
        if (*val).is_null() { return 12 as libc::c_int }
    } else {
        *opt =
            malloc((pos.wrapping_offset_from(input) as libc::c_long +
                        1 as libc::c_int as libc::c_long) as libc::c_ulong) as
                *mut libc::c_char;
        *val = strdup(pos.offset(1 as libc::c_int as isize));
        if (*opt).is_null() || (*val).is_null() {
            free(*opt as *mut libc::c_void);
            *opt = 0 as *mut libc::c_char;
            free(*val as *mut libc::c_void);
            *val = 0 as *mut libc::c_char;
            return 12 as libc::c_int
        }
        memcpy(*opt as *mut libc::c_void, input as *const libc::c_void,
               pos.wrapping_offset_from(input) as libc::c_long as
                   libc::c_ulong);
        *(*opt).offset(pos.wrapping_offset_from(input) as libc::c_long as
                           isize) = '\u{0}' as i32 as libc::c_char
    }
    return 0 as libc::c_int;
}
/* Restore dbctx to the uninitialized state. */
#[c2rust::src_loc = "154:1"]
unsafe extern "C" fn ctx_clear(mut dbc: *mut krb5_db2_context) {
    /*
     * Free any dynamically allocated memory.  File descriptors and locks
     * are the caller's problem.
     */
    free((*dbc).db_lf_name as *mut libc::c_void);
    free((*dbc).db_name as *mut libc::c_void);
    /*
     * Clear the structure and reset the defaults.
     */
    memset(dbc as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_db2_context>() as libc::c_ulong);
    (*dbc).db = 0 as *mut DB;
    (*dbc).db_lf_name = 0 as *mut libc::c_char;
    (*dbc).db_lf_file = -(1 as libc::c_int);
    (*dbc).db_name = 0 as *mut libc::c_char;
    (*dbc).db_nb_locks = 0 as libc::c_int as krb5_boolean;
    (*dbc).tempdb = 0 as libc::c_int as krb5_boolean;
}
/* Set *dbc_out to the db2 database context for context.  If one does not
 * exist, create one in the uninitialized state. */
#[c2rust::src_loc = "177:1"]
unsafe extern "C" fn ctx_get(mut context: krb5_context,
                             mut dbc_out: *mut *mut krb5_db2_context)
 -> krb5_error_code {
    let mut dbc: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    let mut dal_handle: *mut kdb5_dal_handle = 0 as *mut kdb5_dal_handle;
    dal_handle = (*context).dal_handle;
    if (*dal_handle).db_context.is_null() {
        dbc =
            malloc(::std::mem::size_of::<krb5_db2_context>() as libc::c_ulong)
                as *mut krb5_db2_context;
        if dbc.is_null() {
            return 12 as libc::c_int
        } else {
            memset(dbc as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<krb5_db2_context>() as
                       libc::c_ulong);
            ctx_clear(dbc);
            (*dal_handle).db_context = dbc as *mut libc::c_void
        }
    }
    *dbc_out = (*dal_handle).db_context as *mut krb5_db2_context;
    return 0 as libc::c_int;
}
/* Using db_args and the profile, initialize the configurable parameters of the
 * DB context inside context. */
#[c2rust::src_loc = "201:1"]
unsafe extern "C" fn configure_context(mut context: krb5_context,
                                       mut conf_section: *mut libc::c_char,
                                       mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut status: krb5_error_code = 0;
    let mut dbc: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    let mut t_ptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut opt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut profile: profile_t = (*context).profile;
    let mut bval: libc::c_int = 0;
    status = ctx_get(context, &mut dbc);
    if status != 0 as libc::c_int { return status }
    /* Allow unlockiter to be overridden by command line db_args. */
    status =
        profile_get_boolean(profile,
                            b"dbmodules\x00" as *const u8 as
                                *const libc::c_char, conf_section,
                            b"unlockiter\x00" as *const u8 as
                                *const libc::c_char, 0 as libc::c_int,
                            &mut bval) as krb5_error_code;
    if !(status != 0 as libc::c_int) {
        (*dbc).unlockiter = bval as krb5_boolean;
        t_ptr = db_args;
        loop  {
            if !(!t_ptr.is_null() && !(*t_ptr).is_null()) {
                current_block = 1118134448028020070;
                break ;
            }
            free(opt as *mut libc::c_void);
            free(val as *mut libc::c_void);
            status = get_db_opt(*t_ptr, &mut opt, &mut val);
            if !opt.is_null() &&
                   strcmp(opt,
                          b"dbname\x00" as *const u8 as *const libc::c_char)
                       == 0 {
                (*dbc).db_name = strdup(val);
                if (*dbc).db_name.is_null() {
                    status = 12 as libc::c_int;
                    current_block = 12335079809486252666;
                    break ;
                }
            } else if opt.is_null() &&
                          strcmp(val,
                                 b"temporary\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                (*dbc).tempdb = 1 as libc::c_int as krb5_boolean
            } else if !(opt.is_null() &&
                            strcmp(val,
                                   b"merge_nra\x00" as *const u8 as
                                       *const libc::c_char) == 0) {
                if !opt.is_null() &&
                       strcmp(opt,
                              b"hash\x00" as *const u8 as *const libc::c_char)
                           == 0 {
                    (*dbc).hashfirst = 1 as libc::c_int as krb5_boolean
                } else if opt.is_null() &&
                              strcmp(val,
                                     b"unlockiter\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                    (*dbc).unlockiter = 1 as libc::c_int as krb5_boolean
                } else if opt.is_null() &&
                              strcmp(val,
                                     b"lockiter\x00" as *const u8 as
                                         *const libc::c_char) == 0 {
                    (*dbc).unlockiter = 0 as libc::c_int as krb5_boolean
                } else {
                    status = 22 as libc::c_int;
                    krb5_set_error_message(context, status,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"Unsupported argument \"%s\" for db2\x00"
                                                        as *const u8 as
                                                        *const libc::c_char),
                                           if !opt.is_null() {
                                               opt
                                           } else { val });
                    current_block = 12335079809486252666;
                    break ;
                }
            }
            t_ptr = t_ptr.offset(1)
        }
        match current_block {
            12335079809486252666 => { }
            _ => {
                if (*dbc).db_name.is_null() {
                    /* Check for database_name in the db_module section. */
                    status =
                        profile_get_string(profile,
                                           b"dbmodules\x00" as *const u8 as
                                               *const libc::c_char,
                                           conf_section,
                                           b"database_name\x00" as *const u8
                                               as *const libc::c_char,
                                           0 as *const libc::c_char,
                                           &mut pval) as krb5_error_code;
                    if status == 0 as libc::c_int && pval.is_null() {
                        /* For compatibility, check for database_name in the realm. */
                        status =
                            profile_get_string(profile,
                                               b"realms\x00" as *const u8 as
                                                   *const libc::c_char,
                                               (*context).default_realm,
                                               b"database_name\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               b"/usr/local/var/krb5kdc/principal\x00"
                                                   as *const u8 as
                                                   *const libc::c_char,
                                               &mut pval) as krb5_error_code
                    }
                    if status != 0 as libc::c_int {
                        current_block = 12335079809486252666;
                    } else {
                        (*dbc).db_name = strdup(pval);
                        current_block = 10891380440665537214;
                    }
                } else { current_block = 10891380440665537214; }
                match current_block {
                    12335079809486252666 => { }
                    _ => {
                        status =
                            profile_get_boolean(profile,
                                                b"dbmodules\x00" as *const u8
                                                    as *const libc::c_char,
                                                conf_section,
                                                b"disable_last_success\x00" as
                                                    *const u8 as
                                                    *const libc::c_char,
                                                0 as libc::c_int, &mut bval)
                                as krb5_error_code;
                        if !(status != 0 as libc::c_int) {
                            (*dbc).disable_last_success =
                                bval as krb5_boolean;
                            status =
                                profile_get_boolean(profile,
                                                    b"dbmodules\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    conf_section,
                                                    b"disable_lockout\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    0 as libc::c_int,
                                                    &mut bval) as
                                    krb5_error_code;
                            if !(status != 0 as libc::c_int) {
                                (*dbc).disable_lockout = bval as krb5_boolean
                            }
                        }
                    }
                }
            }
        }
    }
    free(opt as *mut libc::c_void);
    free(val as *mut libc::c_void);
    profile_release_string(pval);
    return status;
}
/*
 * Set *out to one of the filenames used for the DB described by dbc.  sfx
 * should be one of SUFFIX_DB, SUFFIX_LOCK, SUFFIX_POLICY, or
 * SUFFIX_POLICY_LOCK.
 */
#[c2rust::src_loc = "291:1"]
unsafe extern "C" fn ctx_dbsuffix(mut dbc: *mut krb5_db2_context,
                                  mut sfx: *const libc::c_char,
                                  mut out: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tilde: *const libc::c_char = 0 as *const libc::c_char;
    *out = 0 as *mut libc::c_char;
    tilde =
        if (*dbc).tempdb != 0 {
            b"~\x00" as *const u8 as *const libc::c_char
        } else { b"\x00" as *const u8 as *const libc::c_char };
    if asprintf(&mut result as *mut *mut libc::c_char,
                b"%s%s%s\x00" as *const u8 as *const libc::c_char,
                (*dbc).db_name, tilde, sfx) < 0 as libc::c_int {
        return 12 as libc::c_int
    }
    *out = result;
    return 0 as libc::c_int;
}
/* Generate all four files corresponding to dbc. */
#[c2rust::src_loc = "306:1"]
unsafe extern "C" fn ctx_allfiles(mut dbc: *mut krb5_db2_context,
                                  mut dbname_out: *mut *mut libc::c_char,
                                  mut lockname_out: *mut *mut libc::c_char,
                                  mut polname_out: *mut *mut libc::c_char,
                                  mut plockname_out: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    *plockname_out = 0 as *mut libc::c_char;
    *polname_out = *plockname_out;
    *lockname_out = *polname_out;
    *dbname_out = *lockname_out;
    if !(ctx_dbsuffix(dbc, b"\x00" as *const u8 as *const libc::c_char,
                      &mut a) != 0) {
        if !(ctx_dbsuffix(dbc, b".ok\x00" as *const u8 as *const libc::c_char,
                          &mut b) != 0) {
            if !(ctx_dbsuffix(dbc,
                              b".kadm5\x00" as *const u8 as
                                  *const libc::c_char, &mut c) != 0) {
                if !(ctx_dbsuffix(dbc,
                                  b".kadm5.lock\x00" as *const u8 as
                                      *const libc::c_char, &mut d) != 0) {
                    *dbname_out = a;
                    *lockname_out = b;
                    *polname_out = c;
                    *plockname_out = d;
                    return 0 as libc::c_int
                }
            }
        }
    }
    free(a as *mut libc::c_void);
    free(b as *mut libc::c_void);
    free(c as *mut libc::c_void);
    free(d as *mut libc::c_void);
    return 12 as libc::c_int;
}
/*
 * Open the DB2 database described by dbc, using the specified flags and mode,
 * and return the resulting handle.  Try both hash and btree database types;
 * dbc->hashfirst determines which is attempted first.  If dbc->hashfirst
 * indicated the wrong type, update it to indicate the correct type.
 */
#[c2rust::src_loc = "340:1"]
unsafe extern "C" fn open_db(mut context: krb5_context,
                             mut dbc: *mut krb5_db2_context,
                             mut flags: libc::c_int, mut mode: libc::c_int,
                             mut db_out: *mut *mut DB) -> krb5_error_code {
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut db: *mut DB = 0 as *mut DB;
    let mut bti: BTREEINFO =
        BTREEINFO{flags: 0,
                  cachesize: 0,
                  maxkeypage: 0,
                  minkeypage: 0,
                  psize: 0,
                  compare: None,
                  prefix: None,
                  lorder: 0,};
    let mut hashi: HASHINFO =
        HASHINFO{bsize: 0,
                 ffactor: 0,
                 nelem: 0,
                 cachesize: 0,
                 hash: None,
                 lorder: 0,};
    bti.flags = 0 as libc::c_int as u_long;
    bti.cachesize = 0 as libc::c_int as u_int;
    bti.psize = 4096 as libc::c_int as u_int;
    bti.lorder = 0 as libc::c_int;
    bti.minkeypage = 0 as libc::c_int;
    bti.compare = None;
    bti.prefix = None;
    *db_out = 0 as *mut DB;
    if ctx_dbsuffix(dbc, b"\x00" as *const u8 as *const libc::c_char,
                    &mut fname) != 0 as libc::c_int {
        return 12 as libc::c_int
    }
    hashi.bsize = 4096 as libc::c_int as u_int;
    hashi.cachesize = 0 as libc::c_int as u_int;
    hashi.ffactor = 40 as libc::c_int as u_int;
    hashi.hash = None;
    hashi.lorder = 0 as libc::c_int;
    hashi.nelem = 1 as libc::c_int as u_int;
    /* Try our best guess at the database type. */
    db =
        kdb2_dbopen(fname, flags, mode,
                    if (*dbc).hashfirst != 0 {
                        DB_HASH as libc::c_int
                    } else { DB_BTREE as libc::c_int } as DBTYPE,
                    if (*dbc).hashfirst != 0 {
                        &mut hashi as *mut HASHINFO as *mut libc::c_void
                    } else {
                        &mut bti as *mut BTREEINFO as *mut libc::c_void
                    });
    if db.is_null() && *__errno_location() == 22 as libc::c_int {
        db =
            kdb2_dbopen(fname, flags, mode,
                        if (*dbc).hashfirst != 0 {
                            DB_BTREE as libc::c_int
                        } else { DB_HASH as libc::c_int } as DBTYPE,
                        if (*dbc).hashfirst != 0 {
                            &mut bti as *mut BTREEINFO as *mut libc::c_void
                        } else {
                            &mut hashi as *mut HASHINFO as *mut libc::c_void
                        });
        /* If that worked, update our guess for next time. */
        if !db.is_null() {
            (*dbc).hashfirst =
                ((*dbc).hashfirst == 0) as libc::c_int as krb5_boolean
        }
    }
    /* Don't try unlocked iteration with a hash database. */
    if !db.is_null() && (*dbc).hashfirst != 0 {
        (*dbc).unlockiter = 0 as libc::c_int as krb5_boolean
    }
    if db.is_null() {
        krb5_prepend_error_message(context, *__errno_location(),
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Cannot open DB2 database \'%s\'\x00"
                                                as *const u8 as
                                                *const libc::c_char), fname);
    }
    *db_out = db;
    free(fname as *mut libc::c_void);
    return if db.is_null() { *__errno_location() } else { 0 as libc::c_int };
}
#[c2rust::src_loc = "396:1"]
unsafe extern "C" fn ctx_unlock(mut context: krb5_context,
                                mut dbc: *mut krb5_db2_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut retval2: krb5_error_code = 0;
    let mut db: *mut DB = 0 as *mut DB;
    retval = osa_adb_release_lock((*dbc).policy_db);
    if (*dbc).db_locks_held == 0 {
        /* lock already unlocked */
        return -(1780008437 as libc::c_long) as krb5_error_code
    }
    db = (*dbc).db;
    (*dbc).db_locks_held -= 1;
    if (*dbc).db_locks_held == 0 as libc::c_int {
        (*db).close.expect("non-null function pointer")(db);
        (*dbc).db = 0 as *mut DB;
        (*dbc).db_lock_mode = 0 as libc::c_int;
        retval2 =
            krb5_lock_file(context, (*dbc).db_lf_file, 0x8 as libc::c_int);
        if retval2 != 0 { return retval2 }
    }
    /* We may be unlocking because osa_adb_get_lock() failed. */
    if retval as libc::c_long == 28810251 as libc::c_long {
        return 0 as libc::c_int
    }
    return retval;
}
#[c2rust::src_loc = "425:1"]
unsafe extern "C" fn ctx_lock(mut context: krb5_context,
                              mut dbc: *mut krb5_db2_context,
                              mut lockmode: libc::c_int) -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut kmode: libc::c_int = 0;
    if lockmode == 0x8 as libc::c_int || lockmode == 0x2 as libc::c_int {
        kmode = 0x2 as libc::c_int
    } else if lockmode == 0x1 as libc::c_int {
        kmode = 0x1 as libc::c_int
    } else { return 22 as libc::c_int }
    if (*dbc).db_locks_held == 0 as libc::c_int || (*dbc).db_lock_mode < kmode
       {
        /* Acquire or upgrade the lock. */
        retval = krb5_lock_file(context, (*dbc).db_lf_file, kmode);
        /* Check if we tried to lock something not open for write. */
        if retval == 9 as libc::c_int && kmode == 0x2 as libc::c_int {
            return -(1780008424 as libc::c_long) as krb5_error_code
        } else {
            if retval == 13 as libc::c_int || retval == 11 as libc::c_int ||
                   retval == 11 as libc::c_int {
                return -(1780008424 as libc::c_long) as krb5_error_code
            } else { if retval != 0 { return retval } }
        }
        /* Open the DB (or re-open it for read/write). */
        if !(*dbc).db.is_null() {
            (*(*dbc).db).close.expect("non-null function pointer")((*dbc).db);
        }
        retval =
            open_db(context, dbc,
                    if kmode == 0x1 as libc::c_int {
                        0 as libc::c_int
                    } else { 0o2 as libc::c_int }, 0o600 as libc::c_int,
                    &mut (*dbc).db);
        if retval != 0 {
            (*dbc).db_locks_held = 0 as libc::c_int;
            (*dbc).db_lock_mode = 0 as libc::c_int;
            osa_adb_release_lock((*dbc).policy_db);
            krb5_lock_file(context, (*dbc).db_lf_file, 0x8 as libc::c_int);
            return retval
        }
        (*dbc).db_lock_mode = kmode
    }
    (*dbc).db_locks_held += 1;
    /* Acquire or upgrade the policy lock. */
    retval = osa_adb_get_lock((*dbc).policy_db, lockmode);
    if retval != 0 {
        ctx_unlock(context, dbc);
        if retval as libc::c_long == 28810253 as libc::c_long ||
               retval as libc::c_long == 28810250 as libc::c_long ||
               retval as libc::c_long == 28810252 as libc::c_long {
            retval = -(1780008424 as libc::c_long) as krb5_error_code
        }
    }
    return retval;
}
/* Initialize the lock file and policy database fields of dbc.  The db_name and
 * tempdb fields must already be set. */
#[c2rust::src_loc = "482:1"]
unsafe extern "C" fn ctx_init(mut dbc: *mut krb5_db2_context)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut polname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plockname: *mut libc::c_char = 0 as *mut libc::c_char;
    retval =
        ctx_dbsuffix(dbc, b".ok\x00" as *const u8 as *const libc::c_char,
                     &mut (*dbc).db_lf_name);
    if retval != 0 { return retval }
    /*
     * should be opened read/write so that write locking can work with
     * POSIX systems
     */
    (*dbc).db_lf_file =
        open((*dbc).db_lf_name, 0o2 as libc::c_int, 0o666 as libc::c_int);
    if (*dbc).db_lf_file < 0 as libc::c_int {
        (*dbc).db_lf_file =
            open((*dbc).db_lf_name, 0 as libc::c_int, 0o666 as libc::c_int);
        if (*dbc).db_lf_file < 0 as libc::c_int {
            retval = *__errno_location();
            current_block = 9242331651186579826;
        } else { current_block = 7815301370352969686; }
    } else { current_block = 7815301370352969686; }
    match current_block {
        7815301370352969686 => {
            fcntl((*dbc).db_lf_file, 2 as libc::c_int, 1 as libc::c_int);
            (*dbc).db_inited = (*dbc).db_inited.wrapping_add(1);
            retval =
                ctx_dbsuffix(dbc,
                             b".kadm5\x00" as *const u8 as
                                 *const libc::c_char, &mut polname);
            if !(retval != 0) {
                retval =
                    ctx_dbsuffix(dbc,
                                 b".kadm5.lock\x00" as *const u8 as
                                     *const libc::c_char, &mut plockname);
                if !(retval != 0) {
                    retval =
                        osa_adb_init_db(&mut (*dbc).policy_db, polname,
                                        plockname, 0x12345a00 as libc::c_int)
                }
            }
        }
        _ => { }
    }
    free(polname as *mut libc::c_void);
    free(plockname as *mut libc::c_void);
    if retval != 0 { ctx_clear(dbc); }
    return retval;
}
#[c2rust::src_loc = "522:1"]
unsafe extern "C" fn ctx_fini(mut dbc: *mut krb5_db2_context) {
    if (*dbc).db_lf_file != -(1 as libc::c_int) { close((*dbc).db_lf_file); }
    if !(*dbc).policy_db.is_null() {
        osa_adb_fini_db((*dbc).policy_db, 0x12345a00 as libc::c_int);
    }
    ctx_clear(dbc);
    free(dbc as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "533:1"]
pub unsafe extern "C" fn krb5_db2_fini(mut context: krb5_context)
 -> krb5_error_code {
    if !(*(*context).dal_handle).db_context.is_null() {
        ctx_fini((*(*context).dal_handle).db_context as
                     *mut krb5_db2_context);
        (*(*context).dal_handle).db_context = 0 as *mut libc::c_void
    }
    return 0 as libc::c_int;
}
/* Return successfully if the db2 name set in context can be opened. */
#[c2rust::src_loc = "544:1"]
unsafe extern "C" fn check_openable(mut context: krb5_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut db: *mut DB = 0 as *mut DB;
    let mut dbc: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    dbc = (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    retval =
        open_db(context, dbc, 0 as libc::c_int, 0 as libc::c_int, &mut db);
    if retval != 0 { return retval }
    (*db).close.expect("non-null function pointer")(db);
    return 0 as libc::c_int;
}
/*
 * Return the last modification time of the database.
 *
 * Think about using fstat.
 */
#[no_mangle]
#[c2rust::src_loc = "565:1"]
pub unsafe extern "C" fn krb5_db2_get_age(mut context: krb5_context,
                                          mut db_name: *mut libc::c_char,
                                          mut age: *mut time_t)
 -> krb5_error_code {
    let mut dbc: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    let mut st: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if !(!(*(*context).dal_handle).db_context.is_null() &&
             (*((*(*context).dal_handle).db_context as
                    *mut krb5_db2_context)).db_inited != 0) {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    dbc = (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    if fstat((*dbc).db_lf_file, &mut st) < 0 as libc::c_int {
        *age = -(1 as libc::c_int) as time_t
    } else { *age = st.st_mtim.tv_sec }
    return 0 as libc::c_int;
}
/* Try to update the timestamp on dbc's lockfile. */
#[c2rust::src_loc = "583:1"]
unsafe extern "C" fn ctx_update_age(mut dbc: *mut krb5_db2_context) {
    let mut st: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut now: time_t = 0;
    let mut utbuf: utimbuf = utimbuf{actime: 0, modtime: 0,};
    now = time(0 as *mut libc::c_void as *mut time_t);
    if fstat((*dbc).db_lf_file, &mut st) != 0 as libc::c_int { return }
    if st.st_mtim.tv_sec >= now {
        utbuf.actime = st.st_mtim.tv_sec + 1 as libc::c_int as libc::c_long;
        utbuf.modtime = st.st_mtim.tv_sec + 1 as libc::c_int as libc::c_long;
        utime((*dbc).db_lf_name, &mut utbuf);
    } else {
        utime((*dbc).db_lf_name, 0 as *mut libc::c_void as *mut utimbuf);
    };
}
#[no_mangle]
#[c2rust::src_loc = "601:1"]
pub unsafe extern "C" fn krb5_db2_lock(mut context: krb5_context,
                                       mut lockmode: libc::c_int)
 -> krb5_error_code {
    if !(!(*(*context).dal_handle).db_context.is_null() &&
             (*((*(*context).dal_handle).db_context as
                    *mut krb5_db2_context)).db_inited != 0) {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    return ctx_lock(context,
                    (*(*context).dal_handle).db_context as
                        *mut krb5_db2_context, lockmode);
}
#[no_mangle]
#[c2rust::src_loc = "609:1"]
pub unsafe extern "C" fn krb5_db2_unlock(mut context: krb5_context)
 -> krb5_error_code {
    if !(!(*(*context).dal_handle).db_context.is_null() &&
             (*((*(*context).dal_handle).db_context as
                    *mut krb5_db2_context)).db_inited != 0) {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    return ctx_unlock(context,
                      (*(*context).dal_handle).db_context as
                          *mut krb5_db2_context);
}
/* Zero out and unlink filename. */
#[c2rust::src_loc = "618:1"]
unsafe extern "C" fn destroy_file(mut filename: *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut statb: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut dowrite: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nb: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut retval: libc::c_int = 0;
    let mut pos: off_t = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut zbuf: [libc::c_char; 8192] = [0; 8192];
    fd = open(filename, 0o2 as libc::c_int, 0 as libc::c_int);
    if fd < 0 as libc::c_int { return *__errno_location() }
    fcntl(fd, 2 as libc::c_int, 1 as libc::c_int);
    /* fstat() will probably not fail unless using a remote filesystem
     * (which is inappropriate for the kerberos database) so this check
     * is mostly paranoia.  */
    if !(fstat(fd, &mut statb) == -(1 as libc::c_int)) {
        /*
     * Stroll through the file, reading in BUFSIZ chunks.  If everything
     * is zero, then we're done for that block, otherwise, zero the block.
     * We would like to just blast through everything, but some DB
     * implementations make holey files and writing data to the holes
     * causes actual blocks to be allocated which is no good, since
     * we're just about to unlink it anyways.
     */
        memset(zbuf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               8192 as libc::c_int as libc::c_ulong);
        pos = 0 as libc::c_int as off_t;
        loop  {
            if !(pos < statb.st_size) {
                current_block = 13472856163611868459;
                break ;
            }
            dowrite = 0 as libc::c_int;
            nb =
                read(fd, buf.as_mut_ptr() as *mut libc::c_void,
                     8192 as libc::c_int as size_t) as libc::c_int;
            if nb < 0 as libc::c_int {
                current_block = 9845889062736777822;
                break ;
            }
            j = 0 as libc::c_int;
            while j < nb {
                if buf[j as usize] as libc::c_int != '\u{0}' as i32 {
                    dowrite = 1 as libc::c_int;
                    break ;
                } else { j += 1 }
            }
            /* For signedness */
            j = nb;
            if dowrite != 0 {
                lseek(fd, pos, 0 as libc::c_int);
                nb =
                    write(fd, zbuf.as_mut_ptr() as *const libc::c_void,
                          j as size_t) as libc::c_int;
                if nb < 0 as libc::c_int {
                    current_block = 9845889062736777822;
                    break ;
                }
            }
            pos += nb as libc::c_long
        }
        match current_block {
            9845889062736777822 => { }
            _ => {
                /* ??? Is fsync really needed?  I don't know of any non-networked
     * filesystem which will discard queued writes to disk if a file
     * is deleted after it is closed.  --jfc */
                fsync(fd);
                close(fd);
                if unlink(filename) != 0 { return *__errno_location() }
                return 0 as libc::c_int
            }
        }
    }
    retval = *__errno_location();
    close(fd);
    return retval;
}
/* Initialize dbc by locking and creating the DB.  If the DB already exists,
 * clear it out if dbc->tempdb is set; otherwise return EEXIST. */
#[c2rust::src_loc = "686:1"]
unsafe extern "C" fn ctx_create_db(mut context: krb5_context,
                                   mut dbc: *mut krb5_db2_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut dbname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut polname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plockname: *mut libc::c_char = 0 as *mut libc::c_char;
    retval =
        ctx_allfiles(dbc, &mut dbname, &mut (*dbc).db_lf_name, &mut polname,
                     &mut plockname);
    if retval != 0 { return retval }
    (*dbc).db_lf_file =
        open((*dbc).db_lf_name,
             0o100 as libc::c_int | 0o2 as libc::c_int |
                 0o1000 as libc::c_int, 0o600 as libc::c_int);
    if (*dbc).db_lf_file < 0 as libc::c_int {
        retval = *__errno_location()
    } else {
        retval =
            krb5_lock_file(context, (*dbc).db_lf_file, 0x2 as libc::c_int);
        if !(retval != 0 as libc::c_int) {
            fcntl((*dbc).db_lf_file, 2 as libc::c_int, 1 as libc::c_int);
            (*dbc).db_lock_mode = 0x2 as libc::c_int;
            (*dbc).db_locks_held = 1 as libc::c_int;
            if (*dbc).tempdb != 0 {
                /* Temporary DBs are locked for their whole lifetime.  Since we have
         * the lock, any remnant files can be safely destroyed. */
                destroy_file(dbname);
                unlink(polname);
                unlink(plockname);
            }
            retval =
                open_db(context, dbc,
                        0o2 as libc::c_int | 0o100 as libc::c_int |
                            0o200 as libc::c_int, 0o600 as libc::c_int,
                        &mut (*dbc).db);
            if !(retval != 0) {
                /* Create the policy database, initialize a handle to it, and lock it. */
                retval =
                    osa_adb_create_db(polname, plockname,
                                      0x12345a00 as libc::c_int);
                if !(retval != 0) {
                    retval =
                        osa_adb_init_db(&mut (*dbc).policy_db, polname,
                                        plockname, 0x12345a00 as libc::c_int);
                    if !(retval != 0) {
                        retval =
                            osa_adb_get_lock((*dbc).policy_db,
                                             0x2 as libc::c_int);
                        if !(retval != 0) {
                            (*dbc).db_inited =
                                1 as libc::c_int as krb5_boolean
                        }
                    }
                }
            }
        }
    }
    if retval != 0 {
        if !(*dbc).db.is_null() {
            (*(*dbc).db).close.expect("non-null function pointer")((*dbc).db);
        }
        if (*dbc).db_locks_held > 0 as libc::c_int {
            krb5_lock_file(context, (*dbc).db_lf_file, 0x8 as libc::c_int);
        }
        if (*dbc).db_lf_file >= 0 as libc::c_int { close((*dbc).db_lf_file); }
        ctx_clear(dbc);
    }
    free(dbname as *mut libc::c_void);
    free(polname as *mut libc::c_void);
    free(plockname as *mut libc::c_void);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "754:1"]
pub unsafe extern "C" fn krb5_db2_get_principal(mut context: krb5_context,
                                                mut searchfor:
                                                    krb5_const_principal,
                                                mut flags: libc::c_uint,
                                                mut entry:
                                                    *mut *mut krb5_db_entry)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut dbc: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    let mut retval: krb5_error_code = 0;
    let mut db: *mut DB = 0 as *mut DB;
    let mut key: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut contents: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut keydata: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut contdata: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut dbret: libc::c_int = 0;
    *entry = 0 as *mut krb5_db_entry;
    if !(!(*(*context).dal_handle).db_context.is_null() &&
             (*((*(*context).dal_handle).db_context as
                    *mut krb5_db2_context)).db_inited != 0) {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    dbc = (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    retval = ctx_lock(context, dbc, 0x1 as libc::c_int);
    if retval != 0 { return retval }
    /* XXX deal with wildcard lookups */
    retval =
        krb5_encode_princ_dbkey(context, &mut keydata,
                                searchfor); /* unlock read lock */
    if !(retval != 0) {
        key.data = keydata.data as *mut libc::c_void;
        key.size = keydata.length as size_t;
        db = (*dbc).db;
        dbret =
            Some((*db).get.expect("non-null function pointer")).expect("non-null function pointer")(db,
                                                                                                    &mut key,
                                                                                                    &mut contents,
                                                                                                    0
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        u_int);
        retval = *__errno_location();
        krb5_free_data_contents(context, &mut keydata);
        match dbret {
            1 => {
                current_block = 7416946810694630179;
                match current_block {
                    594737289688321538 => {
                        contdata.data = contents.data as *mut libc::c_char;
                        contdata.length = contents.size as libc::c_uint;
                        retval =
                            krb5_decode_princ_entry(context, &mut contdata,
                                                    entry)
                    }
                    _ => {
                        retval =
                            -(1780008443 as libc::c_long) as krb5_error_code
                    }
                }
            }
            0 => {
                current_block = 594737289688321538;
                match current_block {
                    594737289688321538 => {
                        contdata.data = contents.data as *mut libc::c_char;
                        contdata.length = contents.size as libc::c_uint;
                        retval =
                            krb5_decode_princ_entry(context, &mut contdata,
                                                    entry)
                    }
                    _ => {
                        retval =
                            -(1780008443 as libc::c_long) as krb5_error_code
                    }
                }
            }
            -1 | _ => { }
        }
    }
    krb5_db2_unlock(context);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "805:1"]
pub unsafe extern "C" fn krb5_db2_put_principal(mut context: krb5_context,
                                                mut entry: *mut krb5_db_entry,
                                                mut db_args:
                                                    *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut dbret: libc::c_int = 0;
    let mut db: *mut DB = 0 as *mut DB;
    let mut key: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut contents: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut contdata: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut keydata: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut retval: krb5_error_code = 0;
    let mut dbc: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    krb5_clear_error_message(context);
    if !db_args.is_null() {
        /* DB2 does not support db_args DB arguments for principal */
        krb5_set_error_message(context, 22 as libc::c_int,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Unsupported argument \"%s\" for db2\x00"
                                            as *const u8 as
                                            *const libc::c_char),
                               *db_args.offset(0 as libc::c_int as
                                                   isize)); /* unlock database */
        return 22 as libc::c_int
    }
    if !(!(*(*context).dal_handle).db_context.is_null() &&
             (*((*(*context).dal_handle).db_context as
                    *mut krb5_db2_context)).db_inited != 0) {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    dbc = (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    retval = ctx_lock(context, dbc, 0x2 as libc::c_int);
    if retval != 0 { return retval }
    db = (*dbc).db;
    retval = krb5_encode_princ_entry(context, &mut contdata, entry);
    if !(retval != 0) {
        contents.data = contdata.data as *mut libc::c_void;
        contents.size = contdata.length as size_t;
        retval =
            krb5_encode_princ_dbkey(context, &mut keydata,
                                    (*entry).princ as krb5_const_principal);
        if retval != 0 {
            krb5_free_data_contents(context, &mut contdata);
        } else {
            key.data = keydata.data as *mut libc::c_void;
            key.size = keydata.length as size_t;
            dbret =
                Some((*db).put.expect("non-null function pointer")).expect("non-null function pointer")(db,
                                                                                                        &mut key,
                                                                                                        &mut contents,
                                                                                                        0
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            u_int);
            retval =
                if dbret != 0 {
                    *__errno_location()
                } else { 0 as libc::c_int };
            krb5_free_data_contents(context, &mut keydata);
            krb5_free_data_contents(context, &mut contdata);
        }
    }
    ctx_update_age(dbc);
    krb5_db2_unlock(context);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "857:1"]
pub unsafe extern "C" fn krb5_db2_delete_principal(mut context: krb5_context,
                                                   mut searchfor:
                                                       krb5_const_principal)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut dbc: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    let mut db: *mut DB = 0 as *mut DB;
    let mut key: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut contents: DBT = DBT{data: 0 as *mut libc::c_void, size: 0,};
    let mut keydata: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut contdata: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut i: libc::c_int = 0;
    let mut dbret: libc::c_int = 0;
    if !(!(*(*context).dal_handle).db_context.is_null() &&
             (*((*(*context).dal_handle).db_context as
                    *mut krb5_db2_context)).db_inited != 0) {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    dbc = (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    retval = ctx_lock(context, dbc, 0x2 as libc::c_int);
    if retval != 0 { return retval }
    retval = krb5_encode_princ_dbkey(context, &mut keydata, searchfor);
    if !(retval != 0) {
        key.data = keydata.data as *mut libc::c_void;
        key.size = keydata.length as size_t;
        db = (*dbc).db;
        dbret =
            Some((*db).get.expect("non-null function pointer")).expect("non-null function pointer")(db,
                                                                                                    &mut key,
                                                                                                    &mut contents,
                                                                                                    0
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        u_int);
        retval = *__errno_location();
        match dbret {
            1 => { retval = -(1780008443 as libc::c_long) as krb5_error_code }
            0 => {
                contdata.data = contents.data as *mut libc::c_char;
                contdata.length = contents.size as libc::c_uint;
                retval =
                    krb5_decode_princ_entry(context, &mut contdata,
                                            &mut entry);
                if !(retval != 0) {
                    /* Clear encrypted key contents */
                    i = 0 as libc::c_int;
                    while i < (*entry).n_key_data as libc::c_int {
                        if (*(*entry).key_data.offset(i as
                                                          isize)).key_data_length[0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      usize]
                               != 0 {
                            memset((*(*entry).key_data.offset(i as
                                                                  isize)).key_data_contents[0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                usize]
                                       as *mut libc::c_void, 0 as libc::c_int,
                                   (*(*entry).key_data.offset(i as
                                                                  isize)).key_data_length[0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              usize]
                                       as libc::c_uint as libc::c_ulong);
                        }
                        i += 1
                    }
                    retval =
                        krb5_encode_princ_entry(context, &mut contdata,
                                                entry);
                    krb5_db_free_principal(context, entry);
                    if !(retval != 0) {
                        contents.data = contdata.data as *mut libc::c_void;
                        contents.size = contdata.length as size_t;
                        dbret =
                            Some((*db).put.expect("non-null function pointer")).expect("non-null function pointer")(db,
                                                                                                                    &mut key,
                                                                                                                    &mut contents,
                                                                                                                    0
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        u_int);
                        retval =
                            if dbret != 0 {
                                *__errno_location()
                            } else { 0 as libc::c_int };
                        krb5_free_data_contents(context, &mut contdata);
                        if !(retval != 0) {
                            dbret =
                                Some((*db).del.expect("non-null function pointer")).expect("non-null function pointer")(db,
                                                                                                                        &mut key,
                                                                                                                        0
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            u_int);
                            retval =
                                if dbret != 0 {
                                    *__errno_location()
                                } else { 0 as libc::c_int }
                        }
                    }
                }
            }
            -1 | _ => { }
        }
        /* Fall through. */
        krb5_free_data_contents(context,
                                &mut keydata); /* unlock write lock */
    }
    ctx_update_age(dbc);
    krb5_db2_unlock(context);
    return retval;
}
/* Lock DB handle of curs, updating curs->islocked. */
#[c2rust::src_loc = "946:1"]
unsafe extern "C" fn curs_lock(mut curs: *mut iter_curs) -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    retval = ctx_lock((*curs).ctx, (*curs).dbc, (*curs).lockmode);
    if retval != 0 { return retval }
    (*curs).islocked = 1 as libc::c_int as krb5_boolean;
    return 0 as libc::c_int;
}
/* Unlock DB handle of curs, updating curs->islocked. */
#[c2rust::src_loc = "959:1"]
unsafe extern "C" fn curs_unlock(mut curs: *mut iter_curs) {
    ctx_unlock((*curs).ctx, (*curs).dbc);
    (*curs).islocked = 0 as libc::c_int as krb5_boolean;
}
/* Set up curs and lock DB. */
#[c2rust::src_loc = "967:1"]
unsafe extern "C" fn curs_init(mut curs: *mut iter_curs,
                               mut ctx: krb5_context,
                               mut dbc: *mut krb5_db2_context,
                               mut iterflags: krb5_flags) -> krb5_error_code {
    let mut isrecurse: libc::c_int = iterflags & 0x4 as libc::c_int;
    let mut prevflag: libc::c_uint = 9 as libc::c_int as libc::c_uint;
    let mut nextflag: libc::c_uint = 7 as libc::c_int as libc::c_uint;
    (*curs).keycopy.size = 0 as libc::c_int as size_t;
    (*curs).keycopy.data = 0 as *mut libc::c_void;
    (*curs).islocked = 0 as libc::c_int as krb5_boolean;
    (*curs).ctx = ctx;
    (*curs).dbc = dbc;
    if iterflags & 0x1 as libc::c_int != 0 {
        (*curs).lockmode = 0x2 as libc::c_int
    } else { (*curs).lockmode = 0x1 as libc::c_int }
    if isrecurse != 0 {
        if (*dbc).hashfirst != 0 {
            krb5_set_error_message(ctx, 22 as libc::c_int,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Recursive iteration is not supported for hash databases\x00"
                                                as *const u8 as
                                                *const libc::c_char));
            return 22 as libc::c_int
        }
        prevflag = 129 as libc::c_int as libc::c_uint;
        nextflag = 128 as libc::c_int as libc::c_uint
    }
    if iterflags & 0x2 as libc::c_int != 0 {
        (*curs).startflag = 6 as libc::c_int as libc::c_uint;
        (*curs).stepflag = prevflag
    } else {
        (*curs).startflag = 3 as libc::c_int as libc::c_uint;
        (*curs).stepflag = nextflag
    }
    return curs_lock(curs);
}
/* Get initial entry. */
#[c2rust::src_loc = "1012:1"]
unsafe extern "C" fn curs_start(mut curs: *mut iter_curs) -> libc::c_int {
    let mut db: *mut DB = (*(*curs).dbc).db;
    return (*db).seq.expect("non-null function pointer")(db, &mut (*curs).key,
                                                         &mut (*curs).data,
                                                         (*curs).startflag);
}
/* Save iteration state so DB can be unlocked/closed. */
#[c2rust::src_loc = "1021:1"]
unsafe extern "C" fn curs_save(mut curs: *mut iter_curs) -> krb5_error_code {
    if (*(*curs).dbc).unlockiter == 0 { return 0 as libc::c_int }
    (*curs).keycopy.data = malloc((*curs).key.size);
    if (*curs).keycopy.data.is_null() { return 12 as libc::c_int }
    (*curs).keycopy.size = (*curs).key.size;
    memcpy((*curs).keycopy.data, (*curs).key.data, (*curs).key.size);
    return 0 as libc::c_int;
}
/* Free allocated cursor resources */
#[c2rust::src_loc = "1037:1"]
unsafe extern "C" fn curs_free(mut curs: *mut iter_curs) {
    free((*curs).keycopy.data);
    (*curs).keycopy.size = 0 as libc::c_int as size_t;
    (*curs).keycopy.data = 0 as *mut libc::c_void;
}
/* Move one step of iteration (forwards or backwards as requested).  Free
 * curs->keycopy as a side effect, if needed. */
#[c2rust::src_loc = "1047:1"]
unsafe extern "C" fn curs_step(mut curs: *mut iter_curs) -> libc::c_int {
    let mut dbret: libc::c_int = 0;
    let mut dbc: *mut krb5_db2_context = (*curs).dbc;
    if (*dbc).unlockiter != 0 {
        /* Reacquire libdb cursor using saved copy of key. */
        (*curs).key = (*curs).keycopy;
        dbret =
            (*(*dbc).db).seq.expect("non-null function pointer")((*dbc).db,
                                                                 &mut (*curs).key,
                                                                 &mut (*curs).data,
                                                                 1 as
                                                                     libc::c_int
                                                                     as
                                                                     u_int);
        curs_free(curs);
        if dbret != 0 { return dbret }
    }
    return (*(*dbc).db).seq.expect("non-null function pointer")((*dbc).db,
                                                                &mut (*curs).key,
                                                                &mut (*curs).data,
                                                                (*curs).stepflag);
}
/* Run one invocation of the callback, unlocking the mutex and possibly the DB
 * around the invocation. */
#[c2rust::src_loc = "1066:1"]
unsafe extern "C" fn curs_run_cb(mut curs: *mut iter_curs,
                                 mut func: ctx_iterate_cb,
                                 mut func_arg: krb5_pointer)
 -> krb5_error_code {
    let mut dbc: *mut krb5_db2_context = (*curs).dbc;
    let mut retval: krb5_error_code = 0;
    let mut lockerr: krb5_error_code = 0;
    let mut entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut ctx: krb5_context = (*curs).ctx;
    let mut contdata: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    contdata =
        make_data((*curs).data.data, (*curs).data.size as libc::c_uint);
    retval = krb5_decode_princ_entry(ctx, &mut contdata, &mut entry);
    if retval != 0 { return retval }
    /* Save libdb key across possible DB closure. */
    retval = curs_save(curs);
    if retval != 0 { return retval }
    if (*dbc).unlockiter != 0 { curs_unlock(curs); }
    krb5int_mutex_unlock(krb5_db2_mutex);
    retval =
        Some(func.expect("non-null function pointer")).expect("non-null function pointer")(func_arg,
                                                                                           entry);
    krb5_db_free_principal(ctx, entry);
    krb5int_mutex_lock(krb5_db2_mutex);
    if (*dbc).unlockiter != 0 {
        lockerr = curs_lock(curs);
        if lockerr != 0 { return lockerr }
    }
    return retval;
}
/* Free cursor resources and unlock the DB if needed. */
#[c2rust::src_loc = "1100:1"]
unsafe extern "C" fn curs_fini(mut curs: *mut iter_curs) {
    curs_free(curs);
    if (*curs).islocked != 0 { curs_unlock(curs); };
}
#[c2rust::src_loc = "1108:1"]
unsafe extern "C" fn ctx_iterate(mut context: krb5_context,
                                 mut dbc: *mut krb5_db2_context,
                                 mut func: ctx_iterate_cb,
                                 mut func_arg: krb5_pointer,
                                 mut iterflags: krb5_flags)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut dbret: libc::c_int = 0;
    let mut curs: iter_curs =
        iter_curs{key: DBT{data: 0 as *mut libc::c_void, size: 0,},
                  data: DBT{data: 0 as *mut libc::c_void, size: 0,},
                  keycopy: DBT{data: 0 as *mut libc::c_void, size: 0,},
                  startflag: 0,
                  stepflag: 0,
                  ctx: 0 as *mut _krb5_context,
                  dbc: 0 as *mut krb5_db2_context,
                  lockmode: 0,
                  islocked: 0,};
    retval = curs_init(&mut curs, context, dbc, iterflags);
    if retval != 0 { return retval }
    dbret = curs_start(&mut curs);
    loop  {
        if !(dbret == 0 as libc::c_int) {
            current_block = 13183875560443969876;
            break ;
        }
        retval = curs_run_cb(&mut curs, func, func_arg);
        if retval != 0 { current_block = 9730632343856994291; break ; }
        dbret = curs_step(&mut curs)
    }
    match current_block {
        13183875560443969876 => {
            match dbret {
                1 | 0 => { }
                -1 | _ => { retval = *__errno_location() }
            }
        }
        _ => { }
    }
    curs_fini(&mut curs);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "1139:1"]
pub unsafe extern "C" fn krb5_db2_iterate(mut context: krb5_context,
                                          mut match_expr: *mut libc::c_char,
                                          mut func: ctx_iterate_cb,
                                          mut func_arg: krb5_pointer,
                                          mut iterflags: krb5_flags)
 -> krb5_error_code {
    if !(!(*(*context).dal_handle).db_context.is_null() &&
             (*((*(*context).dal_handle).db_context as
                    *mut krb5_db2_context)).db_inited != 0) {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    return ctx_iterate(context,
                       (*(*context).dal_handle).db_context as
                           *mut krb5_db2_context, func, func_arg, iterflags);
}
#[no_mangle]
#[c2rust::src_loc = "1149:1"]
pub unsafe extern "C" fn krb5_db2_set_lockmode(mut context: krb5_context,
                                               mut mode: krb5_boolean)
 -> krb5_boolean {
    let mut old: krb5_boolean = 0;
    let mut dbc: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    dbc = (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    old = mode;
    if !dbc.is_null() { old = (*dbc).db_nb_locks; (*dbc).db_nb_locks = mode }
    return old;
}
/*
 *     DAL API functions
 */
#[no_mangle]
#[c2rust::src_loc = "1167:1"]
pub unsafe extern "C" fn krb5_db2_lib_init() -> krb5_error_code {
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1173:1"]
pub unsafe extern "C" fn krb5_db2_lib_cleanup() -> krb5_error_code {
    /* right now, no cleanup required */
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1180:1"]
pub unsafe extern "C" fn krb5_db2_open(mut context: krb5_context,
                                       mut conf_section: *mut libc::c_char,
                                       mut db_args: *mut *mut libc::c_char,
                                       mut mode: libc::c_int)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    krb5_clear_error_message(context);
    if !(*(*context).dal_handle).db_context.is_null() &&
           (*((*(*context).dal_handle).db_context as
                  *mut krb5_db2_context)).db_inited != 0 {
        return 0 as libc::c_int
    }
    status = configure_context(context, conf_section, db_args);
    if status != 0 as libc::c_int { return status }
    status = check_openable(context);
    if status != 0 as libc::c_int { return status }
    return ctx_init((*(*context).dal_handle).db_context as
                        *mut krb5_db2_context);
}
#[no_mangle]
#[c2rust::src_loc = "1201:1"]
pub unsafe extern "C" fn krb5_db2_create(mut context: krb5_context,
                                         mut conf_section: *mut libc::c_char,
                                         mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0 as libc::c_int;
    let mut dbc: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    krb5_clear_error_message(context);
    if !(*(*context).dal_handle).db_context.is_null() &&
           (*((*(*context).dal_handle).db_context as
                  *mut krb5_db2_context)).db_inited != 0 {
        return 0 as libc::c_int
    }
    status = configure_context(context, conf_section, db_args);
    if status != 0 as libc::c_int { return status }
    dbc = (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    status = ctx_create_db(context, dbc);
    if status != 0 as libc::c_int { return status }
    if (*dbc).tempdb == 0 { krb5_db2_unlock(context); }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1226:1"]
pub unsafe extern "C" fn krb5_db2_destroy(mut context: krb5_context,
                                          mut conf_section: *mut libc::c_char,
                                          mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut status: krb5_error_code = 0;
    let mut dbc: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    let mut dbname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lockname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut polname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plockname: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*(*context).dal_handle).db_context.is_null() &&
           (*((*(*context).dal_handle).db_context as
                  *mut krb5_db2_context)).db_inited != 0 {
        status = krb5_db2_fini(context);
        if status != 0 as libc::c_int { return status }
    }
    krb5_clear_error_message(context);
    status = configure_context(context, conf_section, db_args);
    if status != 0 as libc::c_int { return status }
    status = check_openable(context);
    if status != 0 as libc::c_int { return status }
    dbc = (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    status =
        ctx_allfiles(dbc, &mut dbname, &mut lockname, &mut polname,
                     &mut plockname);
    if !(status != 0) {
        status = destroy_file(dbname);
        if !(status != 0) {
            status = unlink(lockname);
            if !(status != 0) {
                status =
                    osa_adb_destroy_db(polname, plockname,
                                       0x12345a00 as libc::c_int);
                if status != 0 { return status }
                status = krb5_db2_fini(context)
            }
        }
    }
    free(dbname as *mut libc::c_void);
    free(lockname as *mut libc::c_void);
    free(polname as *mut libc::c_void);
    free(plockname as *mut libc::c_void);
    return status;
}
/* policy functions */
#[no_mangle]
#[c2rust::src_loc = "1274:1"]
pub unsafe extern "C" fn krb5_db2_create_policy(mut context: krb5_context,
                                                mut policy: osa_policy_ent_t)
 -> krb5_error_code {
    let mut dbc: *mut krb5_db2_context =
        (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    return osa_adb_create_policy((*dbc).policy_db, policy);
}
#[no_mangle]
#[c2rust::src_loc = "1282:1"]
pub unsafe extern "C" fn krb5_db2_get_policy(mut context: krb5_context,
                                             mut name: *mut libc::c_char,
                                             mut policy:
                                                 *mut osa_policy_ent_t)
 -> krb5_error_code {
    let mut dbc: *mut krb5_db2_context =
        (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    return osa_adb_get_policy((*dbc).policy_db, name, policy);
}
#[no_mangle]
#[c2rust::src_loc = "1291:1"]
pub unsafe extern "C" fn krb5_db2_put_policy(mut context: krb5_context,
                                             mut policy: osa_policy_ent_t)
 -> krb5_error_code {
    let mut dbc: *mut krb5_db2_context =
        (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    return osa_adb_put_policy((*dbc).policy_db, policy);
}
#[no_mangle]
#[c2rust::src_loc = "1299:1"]
pub unsafe extern "C" fn krb5_db2_iter_policy(mut context: krb5_context,
                                              mut match_entry:
                                                  *mut libc::c_char,
                                              mut func:
                                                  osa_adb_iter_policy_func,
                                              mut data: *mut libc::c_void)
 -> krb5_error_code {
    let mut dbc: *mut krb5_db2_context =
        (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    return osa_adb_iter_policy((*dbc).policy_db, func, data);
}
#[no_mangle]
#[c2rust::src_loc = "1309:1"]
pub unsafe extern "C" fn krb5_db2_delete_policy(mut context: krb5_context,
                                                mut policy: *mut libc::c_char)
 -> krb5_error_code {
    let mut dbc: *mut krb5_db2_context =
        (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    return osa_adb_destroy_policy((*dbc).policy_db, policy);
}
/*
 * Merge non-replicated attributes from src into dst, setting
 * changed to non-zero if dst was changed.
 *
 * Non-replicated attributes are: last_success, last_failed,
 * fail_auth_count, and any negative TL data values.
 */
#[c2rust::src_loc = "1324:1"]
unsafe extern "C" fn krb5_db2_merge_principal(mut context: krb5_context,
                                              mut src: *mut krb5_db_entry,
                                              mut dst: *mut krb5_db_entry,
                                              mut changed: *mut libc::c_int)
 -> krb5_error_code {
    *changed = 0 as libc::c_int;
    if (*dst).last_success != (*src).last_success {
        (*dst).last_success = (*src).last_success;
        *changed += 1
    }
    if (*dst).last_failed != (*src).last_failed {
        (*dst).last_failed = (*src).last_failed;
        *changed += 1
    }
    if (*dst).fail_auth_count != (*src).fail_auth_count {
        (*dst).fail_auth_count = (*src).fail_auth_count;
        *changed += 1
    }
    return 0 as libc::c_int;
}
/*
 * Iteration callback merges non-replicated attributes from
 * old database.
 */
#[c2rust::src_loc = "1359:1"]
unsafe extern "C" fn krb5_db2_merge_nra_iterator(mut ptr: krb5_pointer,
                                                 mut entry:
                                                     *mut krb5_db_entry)
 -> krb5_error_code {
    let mut nra: *mut nra_context = ptr as *mut nra_context;
    let mut dal_handle: *mut kdb5_dal_handle = (*(*nra).kcontext).dal_handle;
    let mut retval: krb5_error_code = 0;
    let mut changed: libc::c_int = 0;
    let mut s_entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut dst_db: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    memset(&mut s_entry as *mut *mut krb5_db_entry as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<*mut krb5_db_entry>() as libc::c_ulong);
    dst_db = (*dal_handle).db_context as *mut krb5_db2_context;
    (*dal_handle).db_context = (*nra).db_context as *mut libc::c_void;
    /* look up the new principal in the old DB */
    retval =
        krb5_db2_get_principal((*nra).kcontext,
                               (*entry).princ as krb5_const_principal,
                               0 as libc::c_int as libc::c_uint,
                               &mut s_entry);
    if retval != 0 as libc::c_int {
        /* principal may be newly created, so ignore */
        (*dal_handle).db_context = dst_db as *mut libc::c_void;
        return 0 as libc::c_int
    }
    /* merge non-replicated attributes from the old entry in */
    krb5_db2_merge_principal((*nra).kcontext, s_entry, entry, &mut changed);
    (*dal_handle).db_context = dst_db as *mut libc::c_void;
    /* if necessary, commit the modified new entry to the new DB */
    if changed != 0 {
        retval =
            krb5_db2_put_principal((*nra).kcontext, entry,
                                   0 as *mut *mut libc::c_char)
    } else { retval = 0 as libc::c_int }
    krb5_db_free_principal((*nra).kcontext, s_entry);
    return retval;
}
/*
 * Merge non-replicated attributes (that is, lockout-related
 * attributes and negative TL data types) from the real database
 * into the temporary one.
 */
#[c2rust::src_loc = "1403:1"]
unsafe extern "C" fn ctx_merge_nra(mut context: krb5_context,
                                   mut dbc_temp: *mut krb5_db2_context,
                                   mut dbc_real: *mut krb5_db2_context)
 -> krb5_error_code {
    let mut nra: nra_context =
        nra_context{kcontext: 0 as *mut _krb5_context,
                    db_context: 0 as *mut krb5_db2_context,};
    nra.kcontext = context;
    nra.db_context = dbc_real;
    return ctx_iterate(context, dbc_temp,
                       Some(krb5_db2_merge_nra_iterator as
                                unsafe extern "C" fn(_: krb5_pointer,
                                                     _: *mut krb5_db_entry)
                                    -> krb5_error_code),
                       &mut nra as *mut nra_context as krb5_pointer,
                       0 as libc::c_int);
}
/*
 * In the filesystem, promote the temporary database described by dbc_temp to
 * the real database described by dbc_real.  Both must be exclusively locked.
 */
#[c2rust::src_loc = "1418:1"]
unsafe extern "C" fn ctx_promote(mut context: krb5_context,
                                 mut dbc_temp: *mut krb5_db2_context,
                                 mut dbc_real: *mut krb5_db2_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut tdb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tlock: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tpol: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tplock: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rdb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rlock: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rpol: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rplock: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Generate all filenames of interest (including a few we don't need). */
    retval =
        ctx_allfiles(dbc_temp, &mut tdb, &mut tlock, &mut tpol, &mut tplock);
    if retval != 0 { return retval }
    retval =
        ctx_allfiles(dbc_real, &mut rdb, &mut rlock, &mut rpol, &mut rplock);
    if !(retval != 0) {
        /* Rename the principal and policy databases into place. */
        if rename(tdb, rdb) != 0 {
            retval = *__errno_location()
        } else if rename(tpol, rpol) != 0 {
            retval = *__errno_location()
        } else {
            ctx_update_age(dbc_real);
            /* Release and remove the temporary DB lockfiles. */
            unlink(tlock);
            unlink(tplock);
        }
    }
    free(tdb as *mut libc::c_void);
    free(tlock as *mut libc::c_void);
    free(tpol as *mut libc::c_void);
    free(tplock as *mut libc::c_void);
    free(rdb as *mut libc::c_void);
    free(rlock as *mut libc::c_void);
    free(rpol as *mut libc::c_void);
    free(rplock as *mut libc::c_void);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "1462:1"]
pub unsafe extern "C" fn krb5_db2_promote_db(mut context: krb5_context,
                                             mut conf_section:
                                                 *mut libc::c_char,
                                             mut db_args:
                                                 *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut merge_nra: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut real_locked: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut dbc_temp: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    let mut dbc_real: *mut krb5_db2_context = 0 as *mut krb5_db2_context;
    let mut db_argp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    /* context must be initialized with an exclusively locked temp DB. */
    if !(!(*(*context).dal_handle).db_context.is_null() &&
             (*((*(*context).dal_handle).db_context as
                    *mut krb5_db2_context)).db_inited != 0) {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    dbc_temp = (*(*context).dal_handle).db_context as *mut krb5_db2_context;
    if (*dbc_temp).db_lock_mode != 0x2 as libc::c_int {
        return -(1780008437 as libc::c_long) as krb5_error_code
    }
    if (*dbc_temp).tempdb == 0 { return 22 as libc::c_int }
    /* Check db_args for whether we should merge non-replicated attributes. */
    db_argp = db_args;
    while !(*db_argp).is_null() {
        if strcmp(*db_argp,
                  b"merge_nra\x00" as *const u8 as *const libc::c_char) == 0 {
            merge_nra = 1 as libc::c_int as krb5_boolean;
            break ;
        } else { db_argp = db_argp.offset(1) }
    }
    /* Make a db2 context for the real DB. */
    dbc_real =
        k5alloc(::std::mem::size_of::<krb5_db2_context>() as libc::c_ulong,
                &mut retval) as *mut krb5_db2_context;
    if dbc_real.is_null() { return retval }
    ctx_clear(dbc_real);
    /* Try creating the real DB. */
    (*dbc_real).db_name = strdup((*dbc_temp).db_name);
    if !(*dbc_real).db_name.is_null() {
        (*dbc_real).tempdb = 0 as libc::c_int as krb5_boolean;
        retval = ctx_create_db(context, dbc_real);
        if retval == 17 as libc::c_int {
            /* The real database already exists, so open and lock it. */
            (*dbc_real).db_name = strdup((*dbc_temp).db_name);
            if (*dbc_real).db_name.is_null() {
                current_block = 3074960247257659635;
            } else {
                (*dbc_real).tempdb = 0 as libc::c_int as krb5_boolean;
                retval = ctx_init(dbc_real);
                if retval != 0 {
                    current_block = 3074960247257659635;
                } else {
                    retval = ctx_lock(context, dbc_real, 0x2 as libc::c_int);
                    if retval != 0 {
                        current_block = 3074960247257659635;
                    } else { current_block = 5689316957504528238; }
                }
            }
        } else if retval != 0 {
            current_block = 3074960247257659635;
        } else { current_block = 5689316957504528238; }
        match current_block {
            3074960247257659635 => { }
            _ => {
                real_locked = 1 as libc::c_int as krb5_boolean;
                if merge_nra != 0 {
                    retval = ctx_merge_nra(context, dbc_temp, dbc_real);
                    if retval != 0 {
                        current_block = 3074960247257659635;
                    } else { current_block = 18153031941552419006; }
                } else { current_block = 18153031941552419006; }
                match current_block {
                    3074960247257659635 => { }
                    _ => {
                        /* Perform filesystem manipulations for the promotion. */
                        retval = ctx_promote(context, dbc_temp, dbc_real);
                        if !(retval != 0) {
                            /* Unlock and finalize context since the temp DB is gone. */
                            krb5_db2_unlock(context);
                            krb5_db2_fini(context);
                        }
                    }
                }
            }
        }
    }
    if real_locked != 0 { ctx_unlock(context, dbc_real); }
    if !dbc_real.is_null() { ctx_fini(dbc_real); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "1538:1"]
pub unsafe extern "C" fn krb5_db2_check_policy_as(mut kcontext: krb5_context,
                                                  mut request:
                                                      *mut krb5_kdc_req,
                                                  mut client:
                                                      *mut krb5_db_entry,
                                                  mut server:
                                                      *mut krb5_db_entry,
                                                  mut kdc_time:
                                                      krb5_timestamp,
                                                  mut status:
                                                      *mut *const libc::c_char,
                                                  mut e_data:
                                                      *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    retval = krb5_db2_lockout_check_policy(kcontext, client, kdc_time);
    if retval as libc::c_long == -(1765328366 as libc::c_long) {
        *status = b"LOCKED_OUT\x00" as *const u8 as *const libc::c_char
    }
    return retval;
}
/* policy management functions */
/* Thread-safety wrapper slapped on top of original implementation.  */
/* lockout */
#[no_mangle]
#[c2rust::src_loc = "1552:1"]
pub unsafe extern "C" fn krb5_db2_audit_as_req(mut kcontext: krb5_context,
                                               mut request: *mut krb5_kdc_req,
                                               mut local_addr:
                                                   *const krb5_address,
                                               mut remote_addr:
                                                   *const krb5_address,
                                               mut client: *mut krb5_db_entry,
                                               mut server: *mut krb5_db_entry,
                                               mut authtime: krb5_timestamp,
                                               mut error_code:
                                                   krb5_error_code) {
    krb5_db2_lockout_audit(kcontext, client, authtime, error_code);
}
