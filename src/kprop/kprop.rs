use ::libc;
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
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:27"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
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
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/stat.h:27"]
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
#[c2rust::header_src = "/usr/include/unistd.h:27"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
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
    }
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
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
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
    /* * Error message structure */
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
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
    /* * Cleartext that is encrypted and put into @c _krb5_ap_rep.  */
    #[c2rust::src_loc = "2140:1"]
    pub type krb5_ap_rep_enc_part = _krb5_ap_rep_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2246:16"]
    pub struct krb5_replay_data {
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq: krb5_ui_4,
    }
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    /* *< Client time, seconds portion */
    /* *< Client time, microseconds portion */
    /* *< Subkey (optional) */
    /* *< Sequence number */
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
    use super::stdint_uintn_h::{uint8_t, uint32_t};
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
        #[c2rust::src_loc = "353:8"]
        pub type _krb5_auth_context;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
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
 * Format and encode a @c KRB_ERROR message.
 *
 * @param [in]  context         Library context
 * @param [in]  dec_err         Error structure to be encoded
 * @param [out] enc_err         Encoded error structure
 *
 * This function creates a @c KRB_ERROR message in @a enc_err.  Use
 * krb5_free_data_contents() to free @a enc_err when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3292:1"]
        pub fn krb5_mk_error(context: krb5_context,
                             dec_err: *const krb5_error,
                             enc_err: *mut krb5_data) -> krb5_error_code;
        /* *
 * Decode a @c KRB-ERROR message.
 *
 * @param [in]  context         Library context
 * @param [in]  enc_errbuf      Encoded error message
 * @param [out] dec_error       Decoded error message
 *
 * This function processes @c KRB-ERROR message @a enc_errbuf and returns
 * an allocated structure @a dec_error containing the error message.
 * Use krb5_free_error() to free @a dec_error when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3309:1"]
        pub fn krb5_rd_error(context: krb5_context,
                             enc_errbuf: *const krb5_data,
                             dec_error: *mut *mut krb5_error)
         -> krb5_error_code;
        /* *
 * Process @c KRB-SAFE message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           @c KRB-SAFE message to be parsed
 * @param [out] userdata_out    Data parsed from @c KRB-SAFE message
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function parses a @c KRB-SAFE message, verifies its integrity, and
 * stores its data into @a userdata_out.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If @a auth_context has a remote address set, the address will be used to
 * verify the sender address in the KRB-SAFE message.  If @a auth_context has a
 * local address set, it will be used to verify the receiver address in the
 * KRB-SAFE message if the message contains one.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag is set in @a auth_context, the
 * sequence number of the KRB-SAFE message is checked against the remote
 * sequence number field of @a auth_context.  Otherwise, the sequence number is
 * not used.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, then the
 * timestamp in the message is verified to be within the permitted clock skew
 * of the current time, and the message is checked against an in-memory replay
 * cache to detect reflections or replays.
 *
 * Use krb5_free_data_contents() to free @a userdata_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3349:1"]
        pub fn krb5_rd_safe(context: krb5_context,
                            auth_context: krb5_auth_context,
                            inbuf: *const krb5_data,
                            userdata_out: *mut krb5_data,
                            rdata_out: *mut krb5_replay_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3547:1"]
        pub fn krb5_unparse_name_flags(context: krb5_context,
                                       principal: krb5_const_principal,
                                       flags: libc::c_int,
                                       name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4173:1"]
        pub fn krb5_kt_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               ktid: *mut krb5_keytab) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4655:1"]
        pub fn krb5_free_error(context: krb5_context, val: *mut krb5_error);
        #[no_mangle]
        #[c2rust::src_loc = "4677:1"]
        pub fn krb5_free_cred_contents(context: krb5_context,
                                       val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "4732:1"]
        pub fn krb5_free_ap_rep_enc_part(context: krb5_context,
                                         val: *mut krb5_ap_rep_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "4758:1"]
        pub fn krb5_free_data_contents(context: krb5_context,
                                       val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "4819:1"]
        pub fn krb5_us_timeofday(context: krb5_context,
                                 seconds: *mut krb5_timestamp,
                                 microseconds: *mut krb5_int32)
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
        #[c2rust::src_loc = "5293:1"]
        pub fn krb5_mk_safe(context: krb5_context,
                            auth_context: krb5_auth_context,
                            userdata: *const krb5_data,
                            der_out: *mut krb5_data,
                            rdata_out: *mut krb5_replay_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5337:1"]
        pub fn krb5_mk_priv(context: krb5_context,
                            auth_context: krb5_auth_context,
                            userdata: *const krb5_data,
                            der_out: *mut krb5_data,
                            rdata_out: *mut krb5_replay_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5391:1"]
        pub fn krb5_sendauth(context: krb5_context,
                             auth_context: *mut krb5_auth_context,
                             fd: krb5_pointer,
                             appl_version: *mut libc::c_char,
                             client: krb5_principal, server: krb5_principal,
                             ap_req_options: krb5_flags,
                             in_data: *mut krb5_data,
                             in_creds: *mut krb5_creds, ccache: krb5_ccache,
                             error: *mut *mut krb5_error,
                             rep_result: *mut *mut krb5_ap_rep_enc_part,
                             out_creds: *mut *mut krb5_creds)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5613:1"]
        pub fn krb5_auth_con_init(context: krb5_context,
                                  auth_context: *mut krb5_auth_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5644:1"]
        pub fn krb5_auth_con_setflags(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      flags: krb5_int32) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5718:1"]
        pub fn krb5_auth_con_setaddrs(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      local_addr: *mut krb5_address,
                                      remote_addr: *mut krb5_address)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5987:1"]
        pub fn krb5_auth_con_initivector(context: krb5_context,
                                         auth_context: krb5_auth_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7661:1"]
        pub fn krb5_get_init_creds_keytab(context: krb5_context,
                                          creds_0: *mut krb5_creds,
                                          client: krb5_principal,
                                          arg_keytab: krb5_keytab,
                                          start_time: krb5_deltat,
                                          in_tkt_service: *const libc::c_char,
                                          k5_gic_options:
                                              *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
pub mod k5_int_h {
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor, krb5_data};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
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
        #[c2rust::src_loc = "2214:1"]
        pub fn krb5_read_message(_: krb5_context, _: krb5_pointer,
                                 _: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2215:1"]
        pub fn krb5_write_message(_: krb5_context, _: krb5_pointer,
                                  _: *mut krb5_data) -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:27"]
pub mod socket_type_h {
    #[c2rust::src_loc = "24:1"]
    pub type __socket_type = libc::c_uint;
    #[c2rust::src_loc = "52:3"]
    pub const SOCK_NONBLOCK: __socket_type = 2048;
    #[c2rust::src_loc = "49:3"]
    pub const SOCK_CLOEXEC: __socket_type = 524288;
    #[c2rust::src_loc = "41:3"]
    pub const SOCK_PACKET: __socket_type = 10;
    #[c2rust::src_loc = "39:3"]
    pub const SOCK_DCCP: __socket_type = 6;
    #[c2rust::src_loc = "36:3"]
    pub const SOCK_SEQPACKET: __socket_type = 5;
    #[c2rust::src_loc = "34:3"]
    pub const SOCK_RDM: __socket_type = 4;
    #[c2rust::src_loc = "32:3"]
    pub const SOCK_RAW: __socket_type = 3;
    #[c2rust::src_loc = "29:3"]
    pub const SOCK_DGRAM: __socket_type = 2;
    #[c2rust::src_loc = "26:3"]
    pub const SOCK_STREAM: __socket_type = 1;
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
#[c2rust::header_src = "/usr/include/sys/socket.h:27"]
pub mod sys_socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:9"]
    pub union __SOCKADDR_ARG {
        pub __sockaddr__: *mut sockaddr,
        pub __sockaddr_at__: *mut sockaddr_at,
        pub __sockaddr_ax25__: *mut sockaddr_ax25,
        pub __sockaddr_dl__: *mut sockaddr_dl,
        pub __sockaddr_eon__: *mut sockaddr_eon,
        pub __sockaddr_in__: *mut sockaddr_in,
        pub __sockaddr_in6__: *mut sockaddr_in6,
        pub __sockaddr_inarp__: *mut sockaddr_inarp,
        pub __sockaddr_ipx__: *mut sockaddr_ipx,
        pub __sockaddr_iso__: *mut sockaddr_iso,
        pub __sockaddr_ns__: *mut sockaddr_ns,
        pub __sockaddr_un__: *mut sockaddr_un,
        pub __sockaddr_x25__: *mut sockaddr_x25,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:9"]
    pub union __CONST_SOCKADDR_ARG {
        pub __sockaddr__: *const sockaddr,
        pub __sockaddr_at__: *const sockaddr_at,
        pub __sockaddr_ax25__: *const sockaddr_ax25,
        pub __sockaddr_dl__: *const sockaddr_dl,
        pub __sockaddr_eon__: *const sockaddr_eon,
        pub __sockaddr_in__: *const sockaddr_in,
        pub __sockaddr_in6__: *const sockaddr_in6,
        pub __sockaddr_inarp__: *const sockaddr_inarp,
        pub __sockaddr_ipx__: *const sockaddr_ipx,
        pub __sockaddr_iso__: *const sockaddr_iso,
        pub __sockaddr_ns__: *const sockaddr_ns,
        pub __sockaddr_un__: *const sockaddr_un,
        pub __sockaddr_x25__: *const sockaddr_x25,
    }
    use super::socket_h::sockaddr;
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::unistd_h::socklen_t;
    extern "C" {
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_x25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_un;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ns;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_iso;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ipx;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_inarp;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_eon;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_dl;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ax25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_at;
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                           __len: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "126:1"]
        pub fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                       __len: socklen_t) -> libc::c_int;
    }
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn ntohl(__netlong: uint32_t) -> uint32_t;
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
    }
}
#[c2rust::header_src = "/usr/include/netdb.h:27"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "565:8"]
    pub struct addrinfo {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut libc::c_char,
        pub ai_next: *mut addrinfo,
    }
    use super::unistd_h::socklen_t;
    use super::socket_h::sockaddr;
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:27"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:27"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:27"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "36:14"]
        pub static mut optarg: *mut libc::c_char;
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
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
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
#[c2rust::header_src = "/usr/include/sys/stat.h:27"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn stat(__file: *const libc::c_char, __buf: *mut stat)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/locale.h:28"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/fake-addrinfo.h:41"]
pub mod fake_addrinfo_h {
    use super::netdb_h::addrinfo;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2001,2002,2003,2004 by the Massachusetts Institute of Technology,
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
        /* Approach overview:

   If a system version is available but buggy, save handles to it (via
   inline functions in a support library), redefine the names to refer
   to library functions, and in those functions, call the system
   versions and fix up the returned data.  Use the native data
   structures and flag values.

   If no system version exists, use gethostby* and fake it.  Define
   the data structures and flag values locally.


   On macOS, getaddrinfo results aren't cached (though
   gethostbyname results are), so we need to build a cache here.  Now
   things are getting really messy.  Because the cache is in use, we
   use getservbyname, and throw away thread safety.  (Not that the
   cache is thread safe, but when we get locking support, that'll be
   dealt with.)  This code needs tearing down and rebuilding, soon.


   Note that recent Windows developers' code has an interesting hack:
   When you include the right header files, with the right set of
   macros indicating system versions, you'll get an inline function
   that looks for getaddrinfo (or whatever) in the system library, and
   calls it if it's there.  If it's not there, it fakes it with
   gethostby* calls.

   We're taking a simpler approach: A system provides these routines or
   it does not.

   Someday, we may want to take into account different versions (say,
   different revs of GNU libc) where some are broken in one way, and
   some work or are broken in another way.  Cross that bridge when we
   come to it.  */
        /* To do, maybe:

   + For AIX 4.3.3, using the RFC 2133 definition: Implement
   AI_NUMERICHOST.  It's not defined in the header file.

   For certain (old?) versions of GNU libc, AI_NUMERICHOST is
   defined but not implemented.

   + Use gethostbyname2, inet_aton and other IPv6 or thread-safe
   functions if available.  But, see
   https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=135182 for one
   gethostbyname2 problem on Linux.  And besides, if a platform is
   supporting IPv6 at all, they really should be doing getaddrinfo
   by now.

   + inet_ntop, inet_pton

   + Conditionally export/import the function definitions, so a
   library can have a single copy instead of multiple.

   + Upgrade host requirements to include working implementations of
   these functions, and throw all this away.  Pleeease?  :-)  */
        /* ! HAVE_GETADDRINFO */
        /* Fudge things on older gai implementations.  */
/* AIX 4.3.3 is based on RFC 2133; no AI_NUMERICHOST.  */
        /* Partial RFC 2553 implementations may not have AI_ADDRCONFIG and
   friends, which RFC 3493 says are now part of the getaddrinfo
   interface, and we'll want to use.  */
        /* Call out to stuff defined in libkrb5support.  */
        #[no_mangle]
        #[c2rust::src_loc = "214:1"]
        pub fn krb5int_getaddrinfo(node: *const libc::c_char,
                                   service: *const libc::c_char,
                                   hints: *const addrinfo,
                                   aip: *mut *mut addrinfo) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn krb5int_freeaddrinfo(ai: *mut addrinfo);
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn krb5int_gai_strerror(err: libc::c_int) -> *const libc::c_char;
    }
    /* FAI_DEFINED */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/kprop/kprop.h:42"]
pub mod kprop_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_address, krb5_principal,
                        krb5_error_code};
    use super::socket_h::sockaddr;
    extern "C" {
        /* pathnames are in osconf.h, included via k5-int.h */
        #[no_mangle]
        #[c2rust::src_loc = "38:1"]
        pub fn sockaddr2krbaddr(context: krb5_context, family: libc::c_int,
                                sa: *mut sockaddr,
                                dest: *mut *mut krb5_address) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn sn2princ_realm(context: krb5_context,
                              hostname: *const libc::c_char,
                              sname: *const libc::c_char,
                              realm_0: *const libc::c_char,
                              princ_out: *mut krb5_principal)
         -> krb5_error_code;
    }
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t, __dev_t,
                        __uid_t, __gid_t, __ino_t, __mode_t, __nlink_t,
                        __off_t, __off64_t, __time_t, __blksize_t, __blkcnt_t,
                        __ssize_t, __syscall_slong_t, __socklen_t};
pub use self::sys_types_h::ssize_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::struct_timespec_h::timespec;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stat_h::stat;
pub use self::unistd_h::{socklen_t, close, read, write};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_authdatatype, krb5_preauthtype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_pointer,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, krb5_auth_context, _krb5_keyblock,
                       krb5_keyblock, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_creds, krb5_creds,
                       _krb5_error, krb5_error, _krb5_ap_rep_enc_part,
                       krb5_ap_rep_enc_part, krb5_replay_data, krb5_ccache,
                       krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab,
                       _krb5_get_init_creds_opt, krb5_get_init_creds_opt,
                       _profile_t, _krb5_auth_context, _krb5_ccache,
                       krb5_kt_close, krb5_init_context, krb5_mk_error,
                       krb5_rd_error, krb5_rd_safe, krb5_unparse_name_flags,
                       krb5_kt_resolve, krb5_free_principal, krb5_free_error,
                       krb5_free_cred_contents, krb5_free_ap_rep_enc_part,
                       krb5_free_data_contents, krb5_free_unparsed_name,
                       krb5_us_timeofday, krb5_get_default_realm,
                       krb5_free_default_realm, krb5_mk_safe, krb5_mk_priv,
                       krb5_sendauth, krb5_auth_con_init,
                       krb5_auth_con_setflags, krb5_auth_con_setaddrs,
                       krb5_auth_con_initivector, krb5_get_init_creds_keytab};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_lock_file, krb5_read_message,
                         krb5_write_message};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, com_err, error_message};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage};
pub use self::sys_socket_h::{__SOCKADDR_ARG, __CONST_SOCKADDR_ARG,
                             sockaddr_x25, sockaddr_un, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, getsockname, connect};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, ntohl, htonl};
pub use self::netdb_h::addrinfo;
use self::stdlib_h::{free, exit};
use self::stdio_h::{stderr, fprintf, printf, snprintf, asprintf};
use self::fcntl_h::open;
use self::errno_h::__errno_location;
use self::getopt_core_h::{optarg, optind, getopt};
use self::libintl_h::dgettext;
use self::string_h::{strlen, strdup, memset, memcpy};
use self::sys_stat_h::{stat, fstat};
use self::locale_h::setlocale;
use self::fake_addrinfo_h::{krb5int_getaddrinfo, krb5int_freeaddrinfo,
                            krb5int_gai_strerror};
use self::kprop_h::{sockaddr2krbaddr, sn2princ_realm};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kprop/kprop.c */
/*
 * Copyright 1990,1991,2008 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "48:14"]
static mut kprop_version: *mut libc::c_char =
    b"kprop5_01\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "50:14"]
static mut progname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "51:12"]
static mut debug: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "52:14"]
static mut keytab_path: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "53:14"]
static mut replica_host: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "54:14"]
static mut realm: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "55:14"]
static mut def_realm: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "56:14"]
static mut file: *mut libc::c_char =
    b"/usr/local/var/krb5kdc/replica_datatrans\x00" as *const u8 as
        *const libc::c_char as *mut libc::c_char;
/* The Kerberos principal we'll be sending as, initialized in get_tickets. */
#[c2rust::src_loc = "59:23"]
static mut my_principal: krb5_principal =
    0 as *const krb5_principal_data as *mut krb5_principal_data;
#[c2rust::src_loc = "61:19"]
static mut creds: krb5_creds =
    krb5_creds{magic: 0,
               client:
                   0 as *const krb5_principal_data as
                       *mut krb5_principal_data,
               server:
                   0 as *const krb5_principal_data as
                       *mut krb5_principal_data,
               keyblock:
                   krb5_keyblock{magic: 0,
                                 enctype: 0,
                                 length: 0,
                                 contents:
                                     0 as *const krb5_octet as
                                         *mut krb5_octet,},
               times:
                   krb5_ticket_times{authtime: 0,
                                     starttime: 0,
                                     endtime: 0,
                                     renew_till: 0,},
               is_skey: 0,
               ticket_flags: 0,
               addresses:
                   0 as *const *mut krb5_address as *mut *mut krb5_address,
               ticket:
                   krb5_data{magic: 0,
                             length: 0,
                             data:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,},
               second_ticket:
                   krb5_data{magic: 0,
                             length: 0,
                             data:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,},
               authdata:
                   0 as *const *mut krb5_authdata as
                       *mut *mut krb5_authdata,};
#[c2rust::src_loc = "62:22"]
static mut sender_addr: *mut krb5_address =
    0 as *const krb5_address as *mut krb5_address;
#[c2rust::src_loc = "63:22"]
static mut receiver_addr: *mut krb5_address =
    0 as *const krb5_address as *mut krb5_address;
#[c2rust::src_loc = "64:20"]
static mut port: *const libc::c_char =
    b"krb5_prop\x00" as *const u8 as *const libc::c_char;
#[c2rust::src_loc = "65:14"]
static mut dbpathname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\nUsage: %s [-r realm] [-f file] [-d] [-P port] [-s keytab] replica_host\n\n\x00"
                         as *const u8 as *const libc::c_char), progname);
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "90:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut database_fd: libc::c_int = 0;
    let mut database_size: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut my_creds: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut auth_context: krb5_auth_context = 0 as *mut _krb5_auth_context;
    setlocale(6 as libc::c_int, b"\x00" as *const u8 as *const libc::c_char);
    retval = krb5_init_context(&mut context);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing krb5\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    parse_args(context, argc, argv);
    get_tickets(context);
    database_fd = open_database(context, file, &mut database_size);
    open_connection(context, replica_host, &mut fd);
    kerberos_authenticate(context, &mut auth_context, fd, my_principal,
                          &mut my_creds);
    xmit_database(context, auth_context, my_creds, fd, database_fd,
                  database_size);
    update_last_prop_file(replica_host, file);
    printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                    b"Database propagation to %s: SUCCEEDED\n\x00" as
                        *const u8 as *const libc::c_char), replica_host);
    krb5_free_cred_contents(context, my_creds);
    close_database(context, database_fd);
    krb5_free_default_realm(context, def_realm);
    exit(0 as libc::c_int);
}
#[c2rust::src_loc = "121:1"]
unsafe extern "C" fn parse_args(mut context: krb5_context,
                                mut argc: libc::c_int,
                                mut argv: *mut *mut libc::c_char) {
    let mut c: libc::c_int = 0;
    let mut ret: krb5_error_code = 0;
    progname = *argv.offset(0 as libc::c_int as isize);
    loop  {
        c =
            getopt(argc, argv,
                   b"r:f:dP:s:\x00" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) { break ; }
        match c {
            114 => { realm = optarg }
            102 => { file = optarg }
            100 => { debug += 1 }
            80 => { port = optarg }
            115 => { keytab_path = optarg }
            _ => { usage(); }
        }
    }
    if argc - optind != 1 as libc::c_int { usage(); }
    replica_host = *argv.offset(optind as isize);
    if realm.is_null() {
        ret = krb5_get_default_realm(context, &mut def_realm);
        if ret != 0 {
            com_err(progname, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while getting default realm\x00" as *const u8
                                 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
        realm = def_realm
    };
}
#[c2rust::src_loc = "163:1"]
unsafe extern "C" fn get_tickets(mut context: krb5_context) {
    let mut server: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: krb5_error_code = 0;
    let mut keytab: krb5_keytab = 0 as krb5_keytab;
    let mut server_princ: krb5_principal = 0 as krb5_principal;
    /* Figure out what tickets we'll be using to send. */
    retval =
        sn2princ_realm(context, 0 as *const libc::c_char,
                       b"host\x00" as *const u8 as *const libc::c_char, realm,
                       &mut my_principal);
    if retval != 0 {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while setting client principal name\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /* Construct the principal name for the replica host. */
    memset(&mut creds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    retval =
        sn2princ_realm(context, replica_host,
                       b"host\x00" as *const u8 as *const libc::c_char, realm,
                       &mut server_princ);
    if retval != 0 {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while setting server principal name\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    retval =
        krb5_unparse_name_flags(context, server_princ as krb5_const_principal,
                                0x2 as libc::c_int, &mut server);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while unparsing server name\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if !keytab_path.is_null() {
        retval = krb5_kt_resolve(context, keytab_path, &mut keytab);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while resolving keytab\x00" as *const u8 as
                                 *const libc::c_char));
            exit(1 as libc::c_int);
        }
    }
    retval =
        krb5_get_init_creds_keytab(context, &mut creds, my_principal, keytab,
                                   0 as libc::c_int, server,
                                   0 as *mut krb5_get_init_creds_opt);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting initial credentials\n\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if !keytab.is_null() { krb5_kt_close(context, keytab); }
    krb5_free_unparsed_name(context, server);
    krb5_free_principal(context, server_princ);
}
#[c2rust::src_loc = "215:1"]
unsafe extern "C" fn open_connection(mut context: krb5_context,
                                     mut host: *mut libc::c_char,
                                     mut fd_out: *mut libc::c_int) {
    let mut retval: krb5_error_code = 0;
    let mut socket_length: socklen_t = 0;
    let mut hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut res: *mut addrinfo = 0 as *mut addrinfo;
    let mut answers: *mut addrinfo = 0 as *mut addrinfo;
    let mut sa: *mut sockaddr = 0 as *mut sockaddr;
    let mut my_sin: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut s: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    *fd_out = -(1 as libc::c_int);
    memset(&mut hints as *mut addrinfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_flags = 0x20 as libc::c_int;
    error = krb5int_getaddrinfo(host, port, &mut hints, &mut answers);
    if error != 0 as libc::c_int {
        com_err(progname, 0 as libc::c_int as errcode_t,
                b"%s: %s\x00" as *const u8 as *const libc::c_char, host,
                krb5int_gai_strerror(error));
        exit(1 as libc::c_int);
    }
    s = -(1 as libc::c_int);
    retval = 22 as libc::c_int;
    res = answers;
    while !res.is_null() {
        s = socket((*res).ai_family, (*res).ai_socktype, (*res).ai_protocol);
        if s < 0 as libc::c_int {
            com_err(progname, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while creating socket\x00" as *const u8 as
                                 *const libc::c_char));
            exit(1 as libc::c_int);
        }
        if connect(s, __CONST_SOCKADDR_ARG{__sockaddr__: (*res).ai_addr,},
                   (*res).ai_addrlen) < 0 as libc::c_int {
            retval = *__errno_location();
            close(s);
            s = -(1 as libc::c_int);
            res = (*res).ai_next
        } else {
            /* We successfully connect()ed */
            *fd_out = s;
            retval =
                sockaddr2krbaddr(context, (*res).ai_family, (*res).ai_addr,
                                 &mut receiver_addr);
            if retval != 0 as libc::c_int {
                com_err(progname, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while converting server address\x00" as
                                     *const u8 as *const libc::c_char));
                exit(1 as libc::c_int);
            }
            break ;
        }
    }
    krb5int_freeaddrinfo(answers);
    if s == -(1 as libc::c_int) {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while connecting to server\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /* Set sender_addr. */
    socket_length =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    if getsockname(s,
                   __SOCKADDR_ARG{__sockaddr__:
                                      &mut my_sin as *mut sockaddr_storage as
                                          *mut sockaddr,}, &mut socket_length)
           < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting local socket address\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    sa = &mut my_sin as *mut sockaddr_storage as *mut sockaddr;
    if sockaddr2krbaddr(context, (*sa).sa_family as libc::c_int, sa,
                        &mut sender_addr) != 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while converting local address\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    };
}
#[c2rust::src_loc = "284:1"]
unsafe extern "C" fn kerberos_authenticate(mut context: krb5_context,
                                           mut auth_context:
                                               *mut krb5_auth_context,
                                           mut fd: libc::c_int,
                                           mut me: krb5_principal,
                                           mut new_creds:
                                               *mut *mut krb5_creds) {
    let mut retval: krb5_error_code = 0;
    let mut error: *mut krb5_error = 0 as *mut krb5_error;
    let mut rep_result: *mut krb5_ap_rep_enc_part =
        0 as *mut krb5_ap_rep_enc_part;
    retval = krb5_auth_con_init(context, auth_context);
    if retval != 0 { exit(1 as libc::c_int); }
    krb5_auth_con_setflags(context, *auth_context, 0x4 as libc::c_int);
    retval =
        krb5_auth_con_setaddrs(context, *auth_context, sender_addr,
                               receiver_addr);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"in krb5_auth_con_setaddrs\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    retval =
        krb5_sendauth(context, auth_context,
                      &mut fd as *mut libc::c_int as krb5_pointer,
                      kprop_version, me, creds.server,
                      0x20000000 as libc::c_int, 0 as *mut krb5_data,
                      &mut creds, 0 as krb5_ccache, &mut error,
                      &mut rep_result, new_creds);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while authenticating to server\x00" as *const u8 as
                             *const libc::c_char));
        if !error.is_null() {
            if (*error).error == 60 as libc::c_int as libc::c_uint {
                if !(*error).text.data.is_null() {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Generic remote error: %s\n\x00" as
                                         *const u8 as *const libc::c_char),
                            (*error).text.data);
                }
            } else if (*error).error != 0 {
                com_err(progname,
                        (*error).error as krb5_error_code as libc::c_long +
                            -(1765328384 as libc::c_long),
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"signalled from server\x00" as *const u8 as
                                     *const libc::c_char));
                if !(*error).text.data.is_null() {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Error text from server: %s\n\x00" as
                                         *const u8 as *const libc::c_char),
                            (*error).text.data);
                }
            }
            krb5_free_error(context, error);
        }
        exit(1 as libc::c_int);
    }
    krb5_free_ap_rep_enc_part(context, rep_result);
}
/*
 * Open the Kerberos database dump file.  Takes care of locking it
 * and making sure that the .ok file is more recent that the database
 * dump file itself.
 *
 * Returns the file descriptor of the database dump file.  Also fills
 * in the size of the database file.
 */
#[c2rust::src_loc = "341:1"]
unsafe extern "C" fn open_database(mut context: krb5_context,
                                   mut data_fn: *mut libc::c_char,
                                   mut size: *mut libc::c_int)
 -> libc::c_int {
    let mut stbuf: stat =
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
    let mut stbuf_ok: stat =
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
    let mut data_ok_fn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    dbpathname = strdup(data_fn);
    if dbpathname.is_null() {
        com_err(progname, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"allocating database file name \'%s\'\x00" as
                             *const u8 as *const libc::c_char), data_fn);
        exit(1 as libc::c_int);
    }
    fd = open(dbpathname, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while trying to open %s\x00" as *const u8 as
                             *const libc::c_char), dbpathname);
        exit(1 as libc::c_int);
    }
    err =
        krb5_lock_file(context, fd, 0x1 as libc::c_int | 0x4 as libc::c_int);
    if err == 11 as libc::c_int || err == 11 as libc::c_int ||
           *__errno_location() == 13 as libc::c_int {
        com_err(progname, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"database locked\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    } else {
        if err != 0 {
            com_err(progname, err as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while trying to lock \'%s\'\x00" as *const u8
                                 as *const libc::c_char), dbpathname);
            exit(1 as libc::c_int);
        }
    }
    if fstat(fd, &mut stbuf) != 0 {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while trying to stat %s\x00" as *const u8 as
                             *const libc::c_char), data_fn);
        exit(1 as libc::c_int);
    }
    if asprintf(&mut data_ok_fn as *mut *mut libc::c_char,
                b"%s.dump_ok\x00" as *const u8 as *const libc::c_char,
                data_fn) < 0 as libc::c_int {
        com_err(progname, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while trying to malloc data_ok_fn\x00" as *const u8
                             as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if stat(data_ok_fn, &mut stbuf_ok) != 0 {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while trying to stat %s\x00" as *const u8 as
                             *const libc::c_char), data_ok_fn);
        free(data_ok_fn as *mut libc::c_void);
        exit(1 as libc::c_int);
    }
    if stbuf.st_mtim.tv_sec > stbuf_ok.st_mtim.tv_sec {
        com_err(progname, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"\'%s\' more recent than \'%s\'.\x00" as *const u8
                             as *const libc::c_char), data_fn, data_ok_fn);
        exit(1 as libc::c_int);
    }
    free(data_ok_fn as *mut libc::c_void);
    *size = stbuf.st_size as libc::c_int;
    return fd;
}
#[c2rust::src_loc = "392:1"]
unsafe extern "C" fn close_database(mut context: krb5_context,
                                    mut fd: libc::c_int) {
    let mut err: libc::c_int = 0;
    err = krb5_lock_file(context, fd, 0x8 as libc::c_int);
    if err != 0 {
        com_err(progname, err as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while unlocking database \'%s\'\x00" as *const u8
                             as *const libc::c_char), dbpathname);
    }
    free(dbpathname as *mut libc::c_void);
    close(fd);
}
/*
 * Now we send over the database.  We use the following protocol:
 * Send over a KRB_SAFE message with the size.  Then we send over the
 * database in blocks of KPROP_BLKSIZE, encrypted using KRB_PRIV.
 * Then we expect to see a KRB_SAFE message with the size sent back.
 *
 * At any point in the protocol, we may send a KRB_ERROR message; this
 * will abort the entire operation.
 */
#[c2rust::src_loc = "413:1"]
unsafe extern "C" fn xmit_database(mut context: krb5_context,
                                   mut auth_context: krb5_auth_context,
                                   mut my_creds: *mut krb5_creds,
                                   mut fd: libc::c_int,
                                   mut database_fd: libc::c_int,
                                   mut in_database_size: libc::c_int) {
    let mut n: krb5_int32 = 0;
    let mut inbuf: krb5_data =
        krb5_data{magic: 0,
                  length: 0,
                  data: 0 as *const libc::c_char as *mut libc::c_char,};
    let mut outbuf: krb5_data =
        krb5_data{magic: 0,
                  length: 0,
                  data: 0 as *const libc::c_char as *mut libc::c_char,};
    let mut buf: [libc::c_char; 32768] = [0; 32768];
    let mut retval: krb5_error_code = 0;
    let mut error: *mut krb5_error = 0 as *mut krb5_error;
    let mut database_size: krb5_ui_4 = in_database_size as krb5_ui_4;
    let mut send_size: krb5_ui_4 = 0;
    let mut sent_size: krb5_ui_4 = 0;
    /* Send over the size. */
    send_size = htonl(database_size); /* must be 4, really */
    inbuf.data = &mut send_size as *mut krb5_ui_4 as *mut libc::c_char;
    inbuf.length =
        ::std::mem::size_of::<krb5_ui_4>() as libc::c_ulong as libc::c_uint;
    /* KPROP_CKSUMTYPE */
    retval =
        krb5_mk_safe(context, auth_context, &mut inbuf, &mut outbuf,
                     0 as *mut krb5_replay_data);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while encoding database size\x00" as *const u8 as
                             *const libc::c_char));
        send_error(context, my_creds, fd,
                   dgettext(b"mit-krb5\x00" as *const u8 as
                                *const libc::c_char,
                            b"while encoding database size\x00" as *const u8
                                as *const libc::c_char), retval);
        exit(1 as libc::c_int);
    }
    retval =
        krb5_write_message(context,
                           &mut fd as *mut libc::c_int as krb5_pointer,
                           &mut outbuf);
    if retval != 0 {
        krb5_free_data_contents(context, &mut outbuf);
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while sending database size\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    krb5_free_data_contents(context, &mut outbuf);
    /* Initialize the initial vector. */
    retval = krb5_auth_con_initivector(context, auth_context);
    if retval != 0 {
        send_error(context, my_creds, fd,
                   b"failed while initializing i_vector\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char, retval);
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while allocating i_vector\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /* Send over the file, block by block. */
    inbuf.data = buf.as_mut_ptr();
    sent_size = 0 as libc::c_int as krb5_ui_4;
    loop  {
        n =
            read(database_fd, buf.as_mut_ptr() as *mut libc::c_void,
                 ::std::mem::size_of::<[libc::c_char; 32768]>() as
                     libc::c_ulong) as krb5_int32;
        if !(n != 0) { break ; }
        inbuf.length = n as libc::c_uint;
        retval =
            krb5_mk_priv(context, auth_context, &mut inbuf, &mut outbuf,
                         0 as *mut krb5_replay_data);
        if retval != 0 {
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 32768]>() as
                         libc::c_ulong,
                     b"while encoding database block starting at %d\x00" as
                         *const u8 as *const libc::c_char, sent_size);
            com_err(progname, retval as errcode_t,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr());
            send_error(context, my_creds, fd, buf.as_mut_ptr(), retval);
            exit(1 as libc::c_int);
        }
        retval =
            krb5_write_message(context,
                               &mut fd as *mut libc::c_int as krb5_pointer,
                               &mut outbuf);
        if retval != 0 {
            krb5_free_data_contents(context, &mut outbuf);
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while sending database block starting at %d\x00"
                                 as *const u8 as *const libc::c_char),
                    sent_size);
            exit(1 as libc::c_int);
        }
        krb5_free_data_contents(context, &mut outbuf);
        sent_size =
            (sent_size as libc::c_uint).wrapping_add(n as libc::c_uint) as
                krb5_ui_4 as krb5_ui_4;
        if debug != 0 {
            printf(b"%d bytes sent.\n\x00" as *const u8 as
                       *const libc::c_char, sent_size);
        }
    }
    if sent_size != database_size {
        com_err(progname, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Premature EOF found for database file!\x00" as
                             *const u8 as *const libc::c_char));
        send_error(context, my_creds, fd,
                   b"Premature EOF found for database file!\x00" as *const u8
                       as *const libc::c_char as *mut libc::c_char,
                   -(1765328324 as libc::c_long) as krb5_error_code);
        exit(1 as libc::c_int);
    }
    /*
     * OK, we've sent the database; now let's wait for a success
     * indication from the remote end.
     */
    retval =
        krb5_read_message(context,
                          &mut fd as *mut libc::c_int as krb5_pointer,
                          &mut inbuf);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while reading response from server\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /*
     * If we got an error response back from the server, display
     * the error message
     */
    if !(&mut inbuf as *mut krb5_data).is_null() && inbuf.length != 0 &&
           *inbuf.data.offset(0 as libc::c_int as isize) as libc::c_int &
               !(0x20 as libc::c_int) ==
               30 as libc::c_int | 0x40 as libc::c_int {
        retval = krb5_rd_error(context, &mut inbuf, &mut error);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while decoding error response from server\x00"
                                 as *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
        if (*error).error == 60 as libc::c_int as libc::c_uint {
            if !(*error).text.data.is_null() {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Generic remote error: %s\n\x00" as
                                     *const u8 as *const libc::c_char),
                        (*error).text.data);
            }
        } else if (*error).error != 0 {
            com_err(progname,
                    (*error).error as krb5_error_code as libc::c_long +
                        -(1765328384 as libc::c_long),
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"signalled from server\x00" as *const u8 as
                                 *const libc::c_char));
            if !(*error).text.data.is_null() {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Error text from server: %s\n\x00" as
                                     *const u8 as *const libc::c_char),
                        (*error).text.data);
            }
        }
        krb5_free_error(context, error);
        exit(1 as libc::c_int);
    }
    retval =
        krb5_rd_safe(context, auth_context, &mut inbuf, &mut outbuf,
                     0 as *mut krb5_replay_data);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                b"while decoding final size packet from server\x00" as
                    *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memcpy(&mut send_size as *mut krb5_ui_4 as *mut libc::c_void,
           outbuf.data as *const libc::c_void,
           ::std::mem::size_of::<krb5_ui_4>() as libc::c_ulong);
    send_size = ntohl(send_size);
    if send_size != database_size {
        com_err(progname, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Kpropd sent database size %d, expecting %d\x00" as
                             *const u8 as *const libc::c_char), send_size,
                database_size);
        exit(1 as libc::c_int);
    }
    free(inbuf.data as *mut libc::c_void);
    free(outbuf.data as *mut libc::c_void);
}
#[c2rust::src_loc = "547:1"]
unsafe extern "C" fn send_error(mut context: krb5_context,
                                mut my_creds: *mut krb5_creds,
                                mut fd: libc::c_int,
                                mut err_text: *mut libc::c_char,
                                mut err_code: krb5_error_code) {
    let mut error: krb5_error =
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
                                 data:
                                     0 as *const libc::c_char as
                                         *mut libc::c_char,},
                   e_data:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data:
                                     0 as *const libc::c_char as
                                         *mut libc::c_char,},};
    let mut text: *const libc::c_char = 0 as *const libc::c_char;
    let mut outbuf: krb5_data =
        krb5_data{magic: 0,
                  length: 0,
                  data: 0 as *const libc::c_char as *mut libc::c_char,};
    memset(&mut error as *mut krb5_error as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_error>() as libc::c_ulong);
    krb5_us_timeofday(context, &mut error.ctime, &mut error.cusec);
    error.server = (*my_creds).server;
    error.client = my_principal;
    error.error =
        (err_code as libc::c_long - -(1765328384 as libc::c_long)) as
            krb5_ui_4;
    if error.error > 127 as libc::c_int as libc::c_uint {
        error.error = 60 as libc::c_int as krb5_ui_4
    }
    text =
        if !err_text.is_null() {
            err_text as *const libc::c_char
        } else { error_message(err_code as errcode_t) };
    error.text.length =
        strlen(text).wrapping_add(1 as libc::c_int as libc::c_ulong) as
            libc::c_uint;
    error.text.data = strdup(text);
    if !error.text.data.is_null() {
        if krb5_mk_error(context, &mut error, &mut outbuf) == 0 {
            krb5_write_message(context,
                               &mut fd as *mut libc::c_int as krb5_pointer,
                               &mut outbuf);
            krb5_free_data_contents(context, &mut outbuf);
        }
        free(error.text.data as *mut libc::c_void);
    };
}
#[c2rust::src_loc = "574:1"]
unsafe extern "C" fn update_last_prop_file(mut hostname: *mut libc::c_char,
                                           mut file_name: *mut libc::c_char) {
    let mut file_last_prop: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    static mut last_prop: [libc::c_char; 11] =
        [46, 108, 97, 115, 116, 95, 112, 114, 111, 112, 0];
    if asprintf(&mut file_last_prop as *mut *mut libc::c_char,
                b"%s.%s%s\x00" as *const u8 as *const libc::c_char, file_name,
                hostname, last_prop.as_mut_ptr()) < 0 as libc::c_int {
        com_err(progname, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while allocating filename for update_last_prop_file\x00"
                             as *const u8 as *const libc::c_char));
        return
    }
    fd =
        open(file_last_prop,
             0o1 as libc::c_int | 0o100 as libc::c_int |
                 0o1000 as libc::c_int, 0o600 as libc::c_int);
    if fd < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while creating \'last_prop\' file, \'%s\'\x00" as
                             *const u8 as *const libc::c_char),
                file_last_prop);
        free(file_last_prop as *mut libc::c_void);
        return
    }
    write(fd,
          b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          1 as libc::c_int as size_t);
    free(file_last_prop as *mut libc::c_void);
    close(fd);
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
