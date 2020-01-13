use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:49"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:49"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
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
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:49"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:49"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/unistd.h:50"]
pub mod unistd_h {
    #[c2rust::src_loc = "232:1"]
    pub type gid_t = __gid_t;
    use super::types_h::{__gid_t, __uid_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "678:1"]
        pub fn geteuid() -> __uid_t;
        #[no_mangle]
        #[c2rust::src_loc = "684:1"]
        pub fn getegid() -> __gid_t;
        #[no_mangle]
        #[c2rust::src_loc = "689:1"]
        pub fn getgroups(__size: libc::c_int, __list: *mut __gid_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "877:1"]
        pub fn gethostname(__name: *mut libc::c_char, __len: size_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/types.h:53"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:53"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:53"]
pub mod struct_timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__time_t, __suseconds_t};
}
#[c2rust::header_src = "/usr/include/sys/time.h:53"]
pub mod time_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:8"]
    pub struct timezone {
        pub tz_minuteswest: libc::c_int,
        pub tz_dsttime: libc::c_int,
    }
    #[c2rust::src_loc = "58:1"]
    pub type __timezone_ptr_t = *mut timezone;
    use super::struct_timeval_h::timeval;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:53"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:53"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
    use super::stdint_intn_h::int32_t;
    /* !defined(GSSRPC_TYPES_H) */
    /*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
    /* XXX namespace */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:54"]
pub mod xdr_h {
    /* @(#)xdr.h	2.2 88/07/29 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the "Oracle America, Inc." nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)xdr.h 1.19 87/04/22 SMI      */
    /*
 * xdr.h, External Data Representation Serialization Routines.
 */
    /* for FILE */
    /*
 * XDR provides a conventional way for converting between C data
 * types and an external bit-string representation.  Library supplied
 * routines provide for the conversion on built-in C data types.  These
 * routines and utility routines defined here are used to help implement
 * a type encode/decode routine for each user-defined type.
 *
 * Each data type provides a single procedure which takes two arguments:
 *
 *	bool_t
 *	xdrproc(xdrs, argresp)
 *		XDR *xdrs;
 *		<type> *argresp;
 *
 * xdrs is an instance of a XDR handle, to which or from which the data
 * type is to be converted.  argresp is a pointer to the structure to be
 * converted.  The XDR handle contains an operation field which indicates
 * which of the operations (ENCODE, DECODE * or FREE) is to be performed.
 *
 * XDR_DECODE may allocate space if the pointer argresp is null.  This
 * data can be freed with the XDR_FREE operation.
 *
 * We write only one procedure per data type to make it easy
 * to keep the encode and decode procedures for a data type consistent.
 * In many cases the same code performs all operations on a user defined type,
 * because all the hard work is done in the component type routines.
 * decode as a series of calls on the nested data types.
 */
    /*
 * Xdr operations.  XDR_ENCODE causes the type to be encoded into the
 * stream.  XDR_DECODE causes the type to be extracted from the stream.
 * XDR_FREE can be used to release the space allocated by an XDR_DECODE
 * request.
 */
    #[c2rust::src_loc = "81:1"]
    pub type xdr_op = libc::c_uint;
    #[c2rust::src_loc = "84:2"]
    pub const XDR_FREE: xdr_op = 2;
    #[c2rust::src_loc = "83:2"]
    pub const XDR_DECODE: xdr_op = 1;
    #[c2rust::src_loc = "82:2"]
    pub const XDR_ENCODE: xdr_op = 0;
    /*
 * This is the number of bytes per unit of external data.
 */
    /*
 * A xdrproc_t exists for each data type which is to be encoded or decoded.
 *
 * The second argument to the xdrproc_t is a pointer to an opaque pointer.
 * The opaque pointer generally points to a structure of the data type
 * to be decoded.  If this pointer is 0, then the type routines should
 * allocate dynamic storage of the appropriate size and return it.
 * bool_t	(*xdrproc_t)(XDR *, caddr_t *);
 *
 * XXX can't actually prototype it, because some take three args!!!
 */
    #[c2rust::src_loc = "105:1"]
    pub type xdrproc_t = Option<unsafe extern "C" fn() -> libc::c_int>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct XDR {
        pub x_op: xdr_op,
        pub x_ops: *mut xdr_ops,
        pub x_public: caddr_t,
        pub x_private: *mut libc::c_void,
        pub x_base: caddr_t,
        pub x_handy: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "115:9"]
    pub struct xdr_ops {
        pub x_getlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_putlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_getbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_putbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_getpostn: Option<unsafe extern "C" fn(_: *mut XDR) -> u_int>,
        pub x_setpostn: Option<unsafe extern "C" fn(_: *mut XDR, _: u_int)
                                   -> libc::c_int>,
        pub x_inline: Option<unsafe extern "C" fn(_: *mut XDR, _: libc::c_int)
                                 -> *mut rpc_inline_t>,
        pub x_destroy: Option<unsafe extern "C" fn(_: *mut XDR) -> ()>,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::gssrpc_types_h::rpc_inline_t;
    extern "C" {
        /* XDR using memory buffers */
        #[no_mangle]
        #[c2rust::src_loc = "315:1"]
        pub fn gssrpc_xdrmem_create(_: *mut XDR, _: caddr_t, _: u_int,
                                    _: xdr_op);
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:55"]
pub mod auth_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:7"]
    pub union des_block {
        pub c: [libc::c_char; 8],
    }
    /*
 * Authentication info.  Opaque to client.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:8"]
    pub struct opaque_auth {
        pub oa_flavor: libc::c_int,
        pub oa_base: caddr_t,
        pub oa_length: u_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:16"]
    pub struct AUTH {
        pub ah_cred: opaque_auth,
        pub ah_verf: opaque_auth,
        pub ah_key: des_block,
        pub ah_ops: *mut auth_ops,
        pub ah_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:9"]
    pub struct auth_ops {
        pub ah_nextverf: Option<unsafe extern "C" fn(_: *mut AUTH) -> ()>,
        pub ah_marshal: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR)
                                   -> libc::c_int>,
        pub ah_validate: Option<unsafe extern "C" fn(_: *mut AUTH,
                                                     _: *mut opaque_auth)
                                    -> libc::c_int>,
        pub ah_refresh: Option<unsafe extern "C" fn(_: *mut AUTH,
                                                    _: *mut rpc_msg)
                                   -> libc::c_int>,
        pub ah_destroy: Option<unsafe extern "C" fn(_: *mut AUTH) -> ()>,
        pub ah_wrap: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR,
                                                 _: xdrproc_t, _: caddr_t)
                                -> libc::c_int>,
        pub ah_unwrap: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR,
                                                   _: xdrproc_t, _: caddr_t)
                                  -> libc::c_int>,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::xdr_h::{XDR, xdrproc_t};
    extern "C" {
        /* not to exceed MAX_AUTH_BYTES */
        /*
 * Auth handle, interface to client side authenticators.
 */
        #[c2rust::src_loc = "96:8"]
        pub type rpc_msg;
        /*
 * Authentication ops.
 * The ops and the auth handle provide the interface to the authenticators.
 *
 * AUTH	*auth;
 * XDR	*xdrs;
 * struct opaque_auth verf;
 */
        /* RENAMED: should be _null_auth if we can use reserved namespace. */
        #[no_mangle]
        #[c2rust::src_loc = "173:27"]
        pub static mut gssrpc__null_auth: opaque_auth;
        #[no_mangle]
        #[c2rust::src_loc = "194:1"]
        pub fn gssrpc_xdr_opaque_auth(_: *mut XDR, _: *mut opaque_auth)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_AUTH_H) */
    /* RPCSEC_GSS */
    /* GSS-API style */
    /* des style (encrypted timestamps) */
    /* short hand unix style */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_unix.h:56"]
pub mod auth_unix_h {
    /*
 * Unix style credentials.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct authunix_parms {
        pub aup_time: uint32_t,
        pub aup_machname: *mut libc::c_char,
        pub aup_uid: libc::c_int,
        pub aup_gid: libc::c_int,
        pub aup_len: u_int,
        pub aup_gids: *mut libc::c_int,
    }
    use super::stdint_uintn_h::uint32_t;
    use super::sys_types_h::u_int;
    use super::xdr_h::XDR;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn gssrpc_xdr_authunix_parms(_: *mut XDR, _: *mut authunix_parms)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_AUTH_UNIX_H) */
}
#[c2rust::header_src = "/usr/include/stdio.h:49"]
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
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
    }
}
#[c2rust::header_src = "/usr/include/string.h:51"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:53"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_int, __int32_t, __uint32_t, __uid_t, __gid_t,
                        __off_t, __off64_t, __time_t, __suseconds_t,
                        __caddr_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::unistd_h::{gid_t, geteuid, getegid, getgroups, gethostname};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::struct_timeval_h::timeval;
pub use self::time_h::{timezone, __timezone_ptr_t, gettimeofday};
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdrmem_create};
pub use self::auth_h::{des_block, opaque_auth, AUTH, auth_ops, rpc_msg,
                       gssrpc__null_auth, gssrpc_xdr_opaque_auth};
pub use self::auth_unix_h::{authunix_parms, gssrpc_xdr_authunix_parms};
use self::stdio_h::{stderr, fprintf, perror};
use self::string_h::memmove;
use self::stdlib_h::{malloc, free, abort};
/*
 * This struct is pointed to by the ah_private field of an auth_handle.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "81:8"]
pub struct audata {
    pub au_origcred: opaque_auth,
    pub au_shcred: opaque_auth,
    pub au_shfaults: uint32_t,
    pub au_marshed: [libc::c_char; 400],
    pub au_mpos: u_int,
}
#[c2rust::src_loc = "68:24"]
static mut auth_unix_ops: auth_ops =
    unsafe {
        {
            let mut init =
                auth_ops{ah_nextverf:
                             Some(authunix_nextverf as
                                      unsafe extern "C" fn(_: *mut AUTH)
                                          -> ()),
                         ah_marshal:
                             Some(authunix_marshal as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR)
                                          -> libc::c_int),
                         ah_validate:
                             Some(authunix_validate as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _:
                                                               *mut opaque_auth)
                                          -> libc::c_int),
                         ah_refresh:
                             Some(authunix_refresh as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut rpc_msg)
                                          -> libc::c_int),
                         ah_destroy:
                             Some(authunix_destroy as
                                      unsafe extern "C" fn(_: *mut AUTH)
                                          -> ()),
                         ah_wrap:
                             Some(authunix_wrap as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR,
                                                           _: xdrproc_t,
                                                           _: caddr_t)
                                          -> libc::c_int),
                         ah_unwrap:
                             Some(authunix_wrap as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR,
                                                           _: xdrproc_t,
                                                           _: caddr_t)
                                          -> libc::c_int),};
            init
        }
    };
/*
 * These are the various implementations of client side authenticators.
 */
/*
 * Unix style authentication
 * AUTH *authunix_create(machname, uid, gid, len, aup_gids)
 *	char *machname;
 *	int uid;
 *	int gid;
 *	int len;
 *	int *aup_gids;
 */
/*
 * Create a unix style authenticator.
 * Returns an auth handle with the given stuff in it.
 */
#[no_mangle]
#[c2rust::src_loc = "97:1"]
pub unsafe extern "C" fn gssrpc_authunix_create(mut machname:
                                                    *mut libc::c_char,
                                                mut uid: libc::c_int,
                                                mut gid: libc::c_int,
                                                mut len: libc::c_int,
                                                mut aup_gids:
                                                    *mut libc::c_int)
 -> *mut AUTH {
    let mut aup: authunix_parms =
        authunix_parms{aup_time: 0,
                       aup_machname: 0 as *mut libc::c_char,
                       aup_uid: 0,
                       aup_gid: 0,
                       aup_len: 0,
                       aup_gids: 0 as *mut libc::c_int,};
    let mut mymem: [libc::c_char; 400] = [0; 400];
    let mut now: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut auth: *mut AUTH = 0 as *mut AUTH;
    let mut au: *mut audata = 0 as *mut audata;
    /*
	 * Allocate and set up auth handle
	 */
    auth =
        malloc(::std::mem::size_of::<AUTH>() as libc::c_ulong) as *mut AUTH;
    if auth.is_null() {
        fprintf(stderr,
                b"authunix_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as *mut AUTH
    }
    au =
        malloc(::std::mem::size_of::<audata>() as libc::c_ulong) as
            *mut audata;
    if au.is_null() {
        fprintf(stderr,
                b"authunix_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as *mut AUTH
    }
    (*auth).ah_ops = &mut auth_unix_ops;
    (*auth).ah_private = au as caddr_t as *mut libc::c_void;
    (*au).au_shcred = gssrpc__null_auth;
    (*auth).ah_verf = (*au).au_shcred;
    (*au).au_shfaults = 0 as libc::c_int as uint32_t;
    /*
	 * fill in param struct from the given params
	 */
    gettimeofday(&mut now, 0 as *mut timezone);
    aup.aup_time = now.tv_sec as uint32_t;
    aup.aup_machname = machname;
    aup.aup_uid = uid;
    aup.aup_gid = gid;
    aup.aup_len = len as u_int;
    aup.aup_gids = aup_gids;
    /*
	 * Serialize the parameters into origcred
	 */
    gssrpc_xdrmem_create(&mut xdrs, mymem.as_mut_ptr(),
                         400 as libc::c_int as u_int, XDR_ENCODE);
    if gssrpc_xdr_authunix_parms(&mut xdrs, &mut aup) == 0 { abort(); }
    len =
        Some((*xdrs.x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(&mut xdrs)
            as libc::c_int;
    (*au).au_origcred.oa_length = len as u_int;
    (*au).au_origcred.oa_flavor = 1 as libc::c_int;
    (*au).au_origcred.oa_base =
        malloc(len as u_int as libc::c_ulong) as caddr_t;
    if (*au).au_origcred.oa_base.is_null() {
        fprintf(stderr,
                b"authunix_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as *mut AUTH
    }
    memmove((*au).au_origcred.oa_base as *mut libc::c_void,
            mymem.as_mut_ptr() as *const libc::c_void,
            len as u_int as libc::c_ulong);
    /*
	 * set auth handle to reflect new cred.
	 */
    (*auth).ah_cred = (*au).au_origcred;
    marshal_new_auth(auth);
    return auth;
}
/*
 * Returns an auth handle with parameters determined by doing lots of
 * syscalls.
 */
#[no_mangle]
#[c2rust::src_loc = "175:1"]
pub unsafe extern "C" fn gssrpc_authunix_create_default() -> *mut AUTH {
    let mut len: libc::c_int = 0;
    let mut machname: [libc::c_char; 256] = [0; 256];
    let mut uid: libc::c_int = 0;
    let mut gid: libc::c_int = 0;
    let mut gids: [gid_t; 16] = [0; 16];
    let mut igids: [libc::c_int; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    if gethostname(machname.as_mut_ptr(), 255 as libc::c_int as size_t) ==
           -(1 as libc::c_int) {
        abort();
    }
    machname[255 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    uid = geteuid() as libc::c_int;
    gid = getegid() as libc::c_int;
    len = getgroups(16 as libc::c_int, gids.as_mut_ptr());
    if len < 0 as libc::c_int { abort(); }
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        igids[i as usize] = gids[i as usize] as libc::c_int;
        i += 1
    }
    return gssrpc_authunix_create(machname.as_mut_ptr(), uid, gid, len,
                                  igids.as_mut_ptr());
}
/* @(#)auth_unix.c	2.2 88/08/01 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the "Oracle America, Inc." nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*
 * auth_unix.c, Implements UNIX style authentication parameters.
 *
 * The system is very weak.  The client uses no encryption for its
 * credentials and only sends null verifiers.  The server sends backs
 * null verifiers or optionally a verifier that suggests a new short hand
 * for the credentials.
 *
 */
/*
 * Unix authenticator operations vector
 */
/*
 * authunix operations
 */
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn authunix_nextverf(mut auth: *mut AUTH) {
    /* no action necessary */
}
#[c2rust::src_loc = "208:1"]
unsafe extern "C" fn authunix_marshal(mut auth: *mut AUTH, mut xdrs: *mut XDR)
 -> libc::c_int {
    let mut au: *mut audata = (*auth).ah_private as *mut audata;
    return Some((*(*xdrs).x_ops).x_putbytes.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                     (*au).au_marshed.as_mut_ptr(),
                                                                                                                     (*au).au_mpos);
}
#[c2rust::src_loc = "216:1"]
unsafe extern "C" fn authunix_validate(mut auth: *mut AUTH,
                                       mut verf: *mut opaque_auth)
 -> libc::c_int {
    let mut au: *mut audata = 0 as *mut audata;
    let mut xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    if (*verf).oa_flavor == 2 as libc::c_int {
        au = (*auth).ah_private as *mut audata;
        gssrpc_xdrmem_create(&mut xdrs, (*verf).oa_base, (*verf).oa_length,
                             XDR_DECODE);
        if !(*au).au_shcred.oa_base.is_null() {
            free((*au).au_shcred.oa_base as *mut libc::c_void);
            (*au).au_shcred.oa_base = 0 as caddr_t
        }
        if gssrpc_xdr_opaque_auth(&mut xdrs, &mut (*au).au_shcred) != 0 {
            (*auth).ah_cred = (*au).au_shcred
        } else {
            xdrs.x_op = XDR_FREE;
            gssrpc_xdr_opaque_auth(&mut xdrs, &mut (*au).au_shcred);
            (*au).au_shcred.oa_base = 0 as caddr_t;
            (*auth).ah_cred = (*au).au_origcred
        }
        marshal_new_auth(auth);
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "244:1"]
unsafe extern "C" fn authunix_refresh(mut auth: *mut AUTH,
                                      mut msg: *mut rpc_msg) -> libc::c_int {
    let mut au: *mut audata = (*auth).ah_private as *mut audata;
    let mut aup: authunix_parms =
        authunix_parms{aup_time: 0,
                       aup_machname: 0 as *mut libc::c_char,
                       aup_uid: 0,
                       aup_gid: 0,
                       aup_len: 0,
                       aup_gids: 0 as *mut libc::c_int,};
    let mut now: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut stat: libc::c_int = 0;
    if (*auth).ah_cred.oa_base == (*au).au_origcred.oa_base {
        /* there is no hope.  Punt */
        return 0 as libc::c_int
    }
    (*au).au_shfaults = (*au).au_shfaults.wrapping_add(1);
    /* first deserialize the creds back into a struct authunix_parms */
    aup.aup_machname = 0 as *mut libc::c_char;
    aup.aup_gids = 0 as *mut libc::c_void as *mut libc::c_int;
    gssrpc_xdrmem_create(&mut xdrs, (*au).au_origcred.oa_base,
                         (*au).au_origcred.oa_length, XDR_DECODE);
    stat = gssrpc_xdr_authunix_parms(&mut xdrs, &mut aup);
    if !(stat == 0) {
        /* update the time and serialize in place */
        gettimeofday(&mut now, 0 as *mut timezone);
        aup.aup_time = now.tv_sec as uint32_t;
        xdrs.x_op = XDR_ENCODE;
        Some((*xdrs.x_ops).x_setpostn.expect("non-null function pointer")).expect("non-null function pointer")(&mut xdrs,
                                                                                                               0
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   u_int);
        stat = gssrpc_xdr_authunix_parms(&mut xdrs, &mut aup);
        if !(stat == 0) {
            (*auth).ah_cred = (*au).au_origcred;
            marshal_new_auth(auth);
        }
    }
    /* free the struct authunix_parms created by deserializing */
    xdrs.x_op = XDR_FREE;
    gssrpc_xdr_authunix_parms(&mut xdrs, &mut aup);
    if (*xdrs.x_ops).x_destroy.is_some() {
        Some((*xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut xdrs);
    }
    return stat;
}
#[c2rust::src_loc = "286:1"]
unsafe extern "C" fn authunix_destroy(mut auth: *mut AUTH) {
    let mut au: *mut audata = (*auth).ah_private as *mut audata;
    free((*au).au_origcred.oa_base as *mut libc::c_void);
    if !(*au).au_shcred.oa_base.is_null() {
        free((*au).au_shcred.oa_base as *mut libc::c_void);
    }
    free((*auth).ah_private);
    if !(*auth).ah_verf.oa_base.is_null() {
        free((*auth).ah_verf.oa_base as *mut libc::c_void);
    }
    free(auth as caddr_t as *mut libc::c_void);
}
/* xdr pos at end of marshed */
/*
 * Marshals (pre-serializes) an auth struct.
 * sets private data, au_marshed and au_mpos
 */
#[c2rust::src_loc = "308:1"]
unsafe extern "C" fn marshal_new_auth(mut auth: *mut AUTH) {
    let mut xdr_stream: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut xdrs: *mut XDR = &mut xdr_stream;
    let mut au: *mut audata = (*auth).ah_private as *mut audata;
    gssrpc_xdrmem_create(xdrs, (*au).au_marshed.as_mut_ptr(),
                         400 as libc::c_int as u_int, XDR_ENCODE);
    if gssrpc_xdr_opaque_auth(xdrs, &mut (*auth).ah_cred) == 0 ||
           gssrpc_xdr_opaque_auth(xdrs, &mut (*auth).ah_verf) == 0 {
        perror(b"auth_none.c - Fatal marshalling problem\x00" as *const u8 as
                   *const libc::c_char);
    } else {
        (*au).au_mpos =
            Some((*(*xdrs).x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs)
    }
    if (*(*xdrs).x_ops).x_destroy.is_some() {
        Some((*(*xdrs).x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(xdrs);
    };
}
#[c2rust::src_loc = "325:1"]
unsafe extern "C" fn authunix_wrap(mut auth: *mut AUTH, mut xdrs: *mut XDR,
                                   mut xfunc: xdrproc_t, mut xwhere: caddr_t)
 -> libc::c_int {
    return ::std::mem::transmute::<_,
                                   fn(_: _, _: _)
                                       ->
                                           libc::c_int>(Some(xfunc.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                             xwhere);
}
