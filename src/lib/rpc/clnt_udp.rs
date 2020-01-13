use ::libc;
use ::c2rust_asm_casts;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:42"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:42"]
pub mod types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __u_short = libc::c_ushort;
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
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
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:42"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:42"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdio.h:42"]
pub mod stdio_h {
    #[c2rust::src_loc = "77:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:43"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::{__socklen_t, __pid_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "628:1"]
        pub fn getpid() -> __pid_t;
    }
}
#[c2rust::header_src = "/usr/include/sys/types.h:44"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_short, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:44"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:44"]
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
#[c2rust::header_src = "/usr/include/sys/select.h:44"]
pub mod select_h {
    #[c2rust::src_loc = "49:1"]
    pub type __fd_mask = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:9"]
    pub struct fd_set {
        pub __fds_bits: [__fd_mask; 16],
    }
    use super::struct_timeval_h::timeval;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "101:1"]
        pub fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
                      __writefds: *mut fd_set, __exceptfds: *mut fd_set,
                      __timeout: *mut timeval) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/time.h:44"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:44"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:44"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:44"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:44"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:44"]
pub mod in_h {
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "40:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "92:5"]
    pub const IPPROTO_MAX: C2RustUnnamed = 256;
    #[c2rust::src_loc = "90:5"]
    pub const IPPROTO_RAW: C2RustUnnamed = 255;
    #[c2rust::src_loc = "88:5"]
    pub const IPPROTO_MPLS: C2RustUnnamed = 137;
    #[c2rust::src_loc = "86:5"]
    pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
    #[c2rust::src_loc = "84:5"]
    pub const IPPROTO_SCTP: C2RustUnnamed = 132;
    #[c2rust::src_loc = "82:5"]
    pub const IPPROTO_COMP: C2RustUnnamed = 108;
    #[c2rust::src_loc = "80:5"]
    pub const IPPROTO_PIM: C2RustUnnamed = 103;
    #[c2rust::src_loc = "78:5"]
    pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
    #[c2rust::src_loc = "76:5"]
    pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
    #[c2rust::src_loc = "74:5"]
    pub const IPPROTO_MTP: C2RustUnnamed = 92;
    #[c2rust::src_loc = "72:5"]
    pub const IPPROTO_AH: C2RustUnnamed = 51;
    #[c2rust::src_loc = "70:5"]
    pub const IPPROTO_ESP: C2RustUnnamed = 50;
    #[c2rust::src_loc = "68:5"]
    pub const IPPROTO_GRE: C2RustUnnamed = 47;
    #[c2rust::src_loc = "66:5"]
    pub const IPPROTO_RSVP: C2RustUnnamed = 46;
    #[c2rust::src_loc = "64:5"]
    pub const IPPROTO_IPV6: C2RustUnnamed = 41;
    #[c2rust::src_loc = "62:5"]
    pub const IPPROTO_DCCP: C2RustUnnamed = 33;
    #[c2rust::src_loc = "60:5"]
    pub const IPPROTO_TP: C2RustUnnamed = 29;
    #[c2rust::src_loc = "58:5"]
    pub const IPPROTO_IDP: C2RustUnnamed = 22;
    #[c2rust::src_loc = "56:5"]
    pub const IPPROTO_UDP: C2RustUnnamed = 17;
    #[c2rust::src_loc = "54:5"]
    pub const IPPROTO_PUP: C2RustUnnamed = 12;
    #[c2rust::src_loc = "52:5"]
    pub const IPPROTO_EGP: C2RustUnnamed = 8;
    #[c2rust::src_loc = "50:5"]
    pub const IPPROTO_TCP: C2RustUnnamed = 6;
    #[c2rust::src_loc = "48:5"]
    pub const IPPROTO_IPIP: C2RustUnnamed = 4;
    #[c2rust::src_loc = "46:5"]
    pub const IPPROTO_IGMP: C2RustUnnamed = 2;
    #[c2rust::src_loc = "44:5"]
    pub const IPPROTO_ICMP: C2RustUnnamed = 1;
    #[c2rust::src_loc = "42:5"]
    pub const IPPROTO_IP: C2RustUnnamed = 0;
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
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::sockaddr_h::sa_family_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:44"]
pub mod gssrpc_types_h {
    /* @(#)types.h	2.3 88/08/15 4.0 RPCSRC */
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
 *     * Neither the name of the “Oracle America, Inc.” nor the names of
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
/*      @(#)types.h 1.18 87/07/24 SMI      */
    /*
 * Rpc additions to <sys/types.h>
 */
    /*
 * Try to get MAXHOSTNAMELEN from somewhere.
 */
    /* #include <netdb.h> */
    /* Get htonl(), ntohl(), etc. */
    /* Define if we need to fake up some BSD type aliases. */
    /* Allow application to override. */
    /* #undef GSSRPC__BSD_TYPEALIASES */
    #[c2rust::src_loc = "88:1"]
    pub type rpcprog_t = uint32_t;
    #[c2rust::src_loc = "89:1"]
    pub type rpcvers_t = uint32_t;
    #[c2rust::src_loc = "90:1"]
    pub type rpcprot_t = uint32_t;
    #[c2rust::src_loc = "91:1"]
    pub type rpcproc_t = uint32_t;
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::stdint_intn_h::int32_t;
    /* !defined(GSSRPC_TYPES_H) */
    /*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
    /* XXX namespace */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:44"]
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
    /*
 * The XDR handle.
 * Contains operation which is being applied to the stream,
 * an operations vector for the paticular implementation (e.g. see xdr_mem.c),
 * and two private fields for the use of the particular impelementation.
 */
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
        /*
 * In-line routines for fast encode/decode of primitve data types.
 * Caveat emptor: these use single memory cycles to get the
 * data from the underlying buffer, and will fail to operate
 * properly if the data is not aligned.  The standard way to use these
 * is to say:
 *	if ((buf = XDR_INLINE(xdrs, count)) == NULL)
 *		return (FALSE);
 *	<<< macro calls >>>
 * where ``count'' is the number of bytes of data occupied
 * by the primitive data types.
 *
 * N.B. and frozen for all time: each data type here uses 4 bytes
 * of external representation.
 */
        /*
 * These are the "generic" xdr routines.
 */
        #[no_mangle]
        #[c2rust::src_loc = "251:1"]
        pub fn gssrpc_xdr_void(_: *mut XDR, _: *mut libc::c_void)
         -> libc::c_int;
        /* XDR using memory buffers */
        #[no_mangle]
        #[c2rust::src_loc = "315:1"]
        pub fn gssrpc_xdrmem_create(_: *mut XDR, _: caddr_t, _: u_int,
                                    _: xdr_op);
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:44"]
pub mod auth_h {
    /* @(#)auth.h	2.3 88/08/07 4.0 RPCSRC; from 1.17 88/02/08 SMI */
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
 * auth.h, Authentication interface.
 *
 * The data structures are completely opaque to the client.  The client
 * is required to pass a AUTH * to routines that create rpc
 * "sessions".
 */
    /* maximum length of network user's name */
    /*
 * Status returned from authentication check
 */
    #[c2rust::src_loc = "55:1"]
    pub type auth_stat = libc::c_uint;
    #[c2rust::src_loc = "74:2"]
    pub const RPCSEC_GSS_CTXPROBLEM: auth_stat = 14;
    /* some unknown reason */
    /*
	 * RPCSEC_GSS errors
	 */
    #[c2rust::src_loc = "73:2"]
    pub const RPCSEC_GSS_CREDPROBLEM: auth_stat = 13;
    /* bogus response verifier */
    #[c2rust::src_loc = "69:2"]
    pub const AUTH_FAILED: auth_stat = 7;
    /* rejected due to security reasons */
    /*
	 * failed locally
	*/
    #[c2rust::src_loc = "68:2"]
    pub const AUTH_INVALIDRESP: auth_stat = 6;
    /* verifier expired or was replayed */
    #[c2rust::src_loc = "64:2"]
    pub const AUTH_TOOWEAK: auth_stat = 5;
    /* bogus verifier (seal broken) */
    #[c2rust::src_loc = "63:2"]
    pub const AUTH_REJECTEDVERF: auth_stat = 4;
    /* client should begin new session */
    #[c2rust::src_loc = "62:2"]
    pub const AUTH_BADVERF: auth_stat = 3;
    /* bogus credentials (seal broken) */
    #[c2rust::src_loc = "61:2"]
    pub const AUTH_REJECTEDCRED: auth_stat = 2;
    /*
	 * failed at remote end
	 */
    #[c2rust::src_loc = "60:2"]
    pub const AUTH_BADCRED: auth_stat = 1;
    #[c2rust::src_loc = "56:2"]
    pub const AUTH_OK: auth_stat = 0;
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
    /* not to exceed MAX_AUTH_BYTES */
    /*
 * Auth handle, interface to client side authenticators.
 */
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
    use super::rpc_msg_h::rpc_msg;
    extern "C" {
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
        /* takes no parameters */
        #[no_mangle]
        #[c2rust::src_loc = "192:1"]
        pub fn gssrpc_authnone_create() -> *mut AUTH;
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
    /* unix style (uid, gids) */
    /* backward compatibility */
    /* no authentication */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:44"]
pub mod rpc_msg_h {
    /* @(#)rpc_msg.h	2.1 88/07/29 4.0 RPCSRC */
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
/*      @(#)rpc_msg.h 1.7 86/07/16 SMI      */
    /*
 * rpc_msg.h
 * rpc message definition
 */
    /*
 * Bottom up definition of an rpc message.
 * NOTE: call and reply use the same overall stuct but
 * different parts of unions within it.
 */
    /*
 * Reply part of an rpc exchange
 */
    /*
 * Reply to an rpc request that was accepted by the server.
 * Note: there could be an error even though the request was
 * accepted.
 */
    /* and many other null cases */
    /*
 * Reply to an rpc request that was rejected by the server.
 */
    /* why authentication did not work */
    /*
 * Body of a reply to an rpc request.
 */
    /*
 * Body of an rpc request call.
 */
    /* must be equal to two */
    /* protocol specific - provided by client */
    /*
 * The rpc message
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:8"]
    pub struct rpc_msg {
        pub rm_xid: uint32_t,
        pub rm_direction: msg_type,
        pub ru: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed_0 {
        pub RM_cmb: call_body,
        pub RM_rmb: reply_body,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:8"]
    pub struct reply_body {
        pub rp_stat: reply_stat,
        pub ru: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_1 {
        pub RP_ar: accepted_reply,
        pub RP_dr: rejected_reply,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rejected_reply {
        pub rj_stat: reject_stat,
        pub ru: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:2"]
    pub union C2RustUnnamed_2 {
        pub RJ_versions: C2RustUnnamed_3,
        pub RJ_why: auth_stat,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:3"]
    pub struct C2RustUnnamed_3 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type reject_stat = libc::c_uint;
    #[c2rust::src_loc = "76:2"]
    pub const AUTH_ERROR: reject_stat = 1;
    #[c2rust::src_loc = "75:2"]
    pub const RPC_MISMATCH: reject_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "88:8"]
    pub struct accepted_reply {
        pub ar_verf: opaque_auth,
        pub ar_stat: accept_stat,
        pub ru: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:2"]
    pub union C2RustUnnamed_4 {
        pub AR_versions: C2RustUnnamed_6,
        pub AR_results: C2RustUnnamed_5,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:3"]
    pub struct C2RustUnnamed_5 {
        pub where_0: caddr_t,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:3"]
    pub struct C2RustUnnamed_6 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "65:1"]
    pub type accept_stat = libc::c_uint;
    #[c2rust::src_loc = "71:2"]
    pub const SYSTEM_ERR: accept_stat = 5;
    #[c2rust::src_loc = "70:2"]
    pub const GARBAGE_ARGS: accept_stat = 4;
    #[c2rust::src_loc = "69:2"]
    pub const PROC_UNAVAIL: accept_stat = 3;
    #[c2rust::src_loc = "68:2"]
    pub const PROG_MISMATCH: accept_stat = 2;
    #[c2rust::src_loc = "67:2"]
    pub const PROG_UNAVAIL: accept_stat = 1;
    #[c2rust::src_loc = "66:2"]
    pub const SUCCESS: accept_stat = 0;
    #[c2rust::src_loc = "60:1"]
    pub type reply_stat = libc::c_uint;
    #[c2rust::src_loc = "62:2"]
    pub const MSG_DENIED: reply_stat = 1;
    #[c2rust::src_loc = "61:2"]
    pub const MSG_ACCEPTED: reply_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:8"]
    pub struct call_body {
        pub cb_rpcvers: rpcvers_t,
        pub cb_prog: rpcprog_t,
        pub cb_vers: rpcvers_t,
        pub cb_proc: rpcproc_t,
        pub cb_cred: opaque_auth,
        pub cb_verf: opaque_auth,
    }
    #[c2rust::src_loc = "55:1"]
    pub type msg_type = libc::c_uint;
    #[c2rust::src_loc = "57:2"]
    pub const REPLY: msg_type = 1;
    #[c2rust::src_loc = "56:2"]
    pub const CALL: msg_type = 0;
    use super::stdint_uintn_h::uint32_t;
    use super::auth_h::{auth_stat, opaque_auth};
    use super::gssrpc_types_h::{rpcvers_t, rpcprog_t, rpcproc_t};
    use super::sys_types_h::caddr_t;
    use super::xdr_h::{xdrproc_t, XDR};
    use super::clnt_h::rpc_err;
    extern "C" {
        /*
 * XDR routine to handle a rpc message.
 * xdr_callmsg(xdrs, cmsg)
 * 	XDR *xdrs;
 * 	struct rpc_msg *cmsg;
 */
        /*
 * XDR routine to pre-serialize the static part of a rpc message.
 * xdr_callhdr(xdrs, cmsg)
 * 	XDR *xdrs;
 * 	struct rpc_msg *cmsg;
 */
        #[no_mangle]
        #[c2rust::src_loc = "178:1"]
        pub fn gssrpc_xdr_callhdr(_: *mut XDR, _: *mut rpc_msg)
         -> libc::c_int;
        /*
 * XDR routine to handle a rpc reply.
 * xdr_replymsg(xdrs, rmsg)
 * 	XDR *xdrs;
 * 	struct rpc_msg *rmsg;
 */
        #[no_mangle]
        #[c2rust::src_loc = "186:1"]
        pub fn gssrpc_xdr_replymsg(_: *mut XDR, _: *mut rpc_msg)
         -> libc::c_int;
        /*
 * Fills in the error part of a reply message.
 * _seterr_reply(msg, error)
 * 	struct rpc_msg *msg;
 * 	struct rpc_err *error;
 */
/*
 * RENAMED: should be _seterr_reply or __seterr_reply if we can use
 * reserved namespace.
 */
        #[no_mangle]
        #[c2rust::src_loc = "198:1"]
        pub fn gssrpc__seterr_reply(_: *mut rpc_msg, _: *mut rpc_err);
    }
    /* !defined(GSSRPC_RPC_MSG_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:44"]
pub mod clnt_h {
    /* @(#)clnt.h	2.1 88/07/29 4.0 RPCSRC; from 1.31 88/02/08 SMI*/
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
 * clnt.h - Client side remote procedure call interface.
 */
    /*
 * Rpc calls return an enum clnt_stat.  This should be looked at more,
 * since each implementation is required to live with this (implementation
 * independent) list of errors.
 */
    #[c2rust::src_loc = "48:1"]
    pub type clnt_stat = libc::c_uint;
    /* remote program is not registered */
    /*
	 * unspecified error
	 */
    #[c2rust::src_loc = "83:2"]
    pub const RPC_FAILED: clnt_stat = 16;
    /* the pmapper failed in its call */
    #[c2rust::src_loc = "79:2"]
    pub const RPC_PROGNOTREGISTERED: clnt_stat = 15;
    /* unknown protocol */
    /*
	 * _ create errors
	 */
    #[c2rust::src_loc = "78:2"]
    pub const RPC_PMAPFAILURE: clnt_stat = 14;
    /* unknown host name */
    #[c2rust::src_loc = "73:2"]
    pub const RPC_UNKNOWNPROTO: clnt_stat = 17;
    /* generic "other problem" */
    /*
	 * callrpc & clnt_create errors
	 */
    #[c2rust::src_loc = "72:2"]
    pub const RPC_UNKNOWNHOST: clnt_stat = 13;
    /* decode arguments error */
    #[c2rust::src_loc = "67:2"]
    pub const RPC_SYSTEMERROR: clnt_stat = 12;
    /* procedure unavailable */
    #[c2rust::src_loc = "66:2"]
    pub const RPC_CANTDECODEARGS: clnt_stat = 11;
    /* program version mismatched */
    #[c2rust::src_loc = "65:2"]
    pub const RPC_PROCUNAVAIL: clnt_stat = 10;
    /* program not available */
    #[c2rust::src_loc = "64:2"]
    pub const RPC_PROGVERSMISMATCH: clnt_stat = 9;
    /* authentication error */
    #[c2rust::src_loc = "63:2"]
    pub const RPC_PROGUNAVAIL: clnt_stat = 8;
    /* rpc versions not compatible */
    #[c2rust::src_loc = "62:2"]
    pub const RPC_AUTHERROR: clnt_stat = 7;
    /* call timed out */
    /*
	 * remote errors
	 */
    #[c2rust::src_loc = "61:2"]
    pub const RPC_VERSMISMATCH: clnt_stat = 6;
    /* failure in receiving result */
    #[c2rust::src_loc = "57:2"]
    pub const RPC_TIMEDOUT: clnt_stat = 5;
    /* failure in sending call */
    #[c2rust::src_loc = "56:2"]
    pub const RPC_CANTRECV: clnt_stat = 4;
    /* can't decode results */
    #[c2rust::src_loc = "55:2"]
    pub const RPC_CANTSEND: clnt_stat = 3;
    /* can't encode arguments */
    #[c2rust::src_loc = "54:2"]
    pub const RPC_CANTDECODERES: clnt_stat = 2;
    /* call succeeded */
    /*
	 * local errors
	 */
    #[c2rust::src_loc = "53:2"]
    pub const RPC_CANTENCODEARGS: clnt_stat = 1;
    #[c2rust::src_loc = "49:2"]
    pub const RPC_SUCCESS: clnt_stat = 0;
    /*
 * Error info.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "90:8"]
    pub struct rpc_err {
        pub re_status: clnt_stat,
        pub ru: C2RustUnnamed_7,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:2"]
    pub union C2RustUnnamed_7 {
        pub RE_errno: libc::c_int,
        pub RE_why: auth_stat,
        pub RE_vers: C2RustUnnamed_9,
        pub RE_lb: C2RustUnnamed_8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "99:3"]
    pub struct C2RustUnnamed_8 {
        pub s1: int32_t,
        pub s2: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:3"]
    pub struct C2RustUnnamed_9 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    /*
 * Client rpc handle.
 * Created by individual implementations, see e.g. rpc_udp.c.
 * Client is responsible for initializing auth, see e.g. auth_none.c.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:16"]
    pub struct CLIENT {
        pub cl_auth: *mut AUTH,
        pub cl_ops: *mut clnt_ops,
        pub cl_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:9"]
    pub struct clnt_ops {
        pub cl_call: Option<unsafe extern "C" fn(_: *mut CLIENT, _: rpcproc_t,
                                                 _: xdrproc_t,
                                                 _: *mut libc::c_void,
                                                 _: xdrproc_t,
                                                 _: *mut libc::c_void,
                                                 _: timeval) -> clnt_stat>,
        pub cl_abort: Option<unsafe extern "C" fn(_: *mut CLIENT) -> ()>,
        pub cl_geterr: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                   _: *mut rpc_err) -> ()>,
        pub cl_freeres: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                    _: xdrproc_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
        pub cl_destroy: Option<unsafe extern "C" fn(_: *mut CLIENT) -> ()>,
        pub cl_control: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "327:8"]
    pub struct gssrpc_rpc_createrr {
        pub cf_stat: clnt_stat,
        pub cf_error: rpc_err,
    }
    use super::auth_h::{auth_stat, AUTH};
    use super::stdint_intn_h::int32_t;
    use super::gssrpc_types_h::{rpcvers_t, rpcproc_t};
    use super::xdr_h::xdrproc_t;
    use super::struct_timeval_h::timeval;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "332:29"]
        pub static mut gssrpc_rpc_createrr: gssrpc_rpc_createrr;
    }
    /* !defined(GSSRPC_CLNT_H) */
    /* a more reasonable packet size */
    /* rpc imposed limit on udp msg size */
}
#[c2rust::header_src = "/usr/include/sys/socket.h:44"]
pub mod sys_socket_h {
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    use super::stddef_h::size_t;
    use super::stdio_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn getsockname(__fd: libc::c_int, __addr: *mut sockaddr,
                           __len: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "126:1"]
        pub fn connect(__fd: libc::c_int, __addr: *const sockaddr,
                       __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "138:1"]
        pub fn send(__fd: libc::c_int, __buf: *const libc::c_void,
                    __n: size_t, __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "163:1"]
        pub fn recvfrom(__fd: libc::c_int, __buf: *mut libc::c_void,
                        __n: size_t, __flags: libc::c_int,
                        __addr: *mut sockaddr, __addr_len: *mut socklen_t)
         -> ssize_t;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:44"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc.h:44"]
pub mod rpc_h {
    use super::socket_h::sockaddr;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn gssrpc_bindresvport_sa(_: libc::c_int, _: *mut sockaddr)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn gssrpc__rpc_dtablesize() -> libc::c_int;
    }
    /* !defined(GSSRPC_RPC_H) */
}
#[c2rust::header_src = "/usr/include/sys/ioctl.h:46"]
pub mod ioctl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:51"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/pmap_clnt.h:53"]
pub mod pmap_clnt_h {
    use super::in_h::sockaddr_in;
    use super::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcprot_t};
    use super::sys_types_h::u_short;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn gssrpc_pmap_getport(_: *mut sockaddr_in, _: rpcprog_t,
                                   _: rpcvers_t, _: rpcprot_t) -> u_short;
    }
    /* !defined(GSSRPC_PMAP_CLNT_H) */
}
use c2rust_asm_casts::AsmCastTrait;
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __u_int, __uint16_t, __int32_t, __uint32_t,
                        __off_t, __off64_t, __pid_t, __time_t, __suseconds_t,
                        __ssize_t, __caddr_t, __socklen_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdio_h::{ssize_t, stderr, fprintf};
pub use self::unistd_h::{socklen_t, close, getpid};
pub use self::sys_types_h::{u_short, u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::struct_timeval_h::timeval;
pub use self::select_h::{__fd_mask, fd_set, select};
pub use self::time_h::{timezone, __timezone_ptr_t, gettimeofday};
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::sockaddr;
pub use self::in_h::{in_addr_t, in_addr, C2RustUnnamed, IPPROTO_MAX,
                     IPPROTO_RAW, IPPROTO_MPLS, IPPROTO_UDPLITE, IPPROTO_SCTP,
                     IPPROTO_COMP, IPPROTO_PIM, IPPROTO_ENCAP, IPPROTO_BEETPH,
                     IPPROTO_MTP, IPPROTO_AH, IPPROTO_ESP, IPPROTO_GRE,
                     IPPROTO_RSVP, IPPROTO_IPV6, IPPROTO_DCCP, IPPROTO_TP,
                     IPPROTO_IDP, IPPROTO_UDP, IPPROTO_PUP, IPPROTO_EGP,
                     IPPROTO_TCP, IPPROTO_IPIP, IPPROTO_IGMP, IPPROTO_ICMP,
                     IPPROTO_IP, in_port_t, sockaddr_in, htons};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcprot_t, rpcproc_t,
                               rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdrmem_create};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, des_block,
                       opaque_auth, AUTH, auth_ops, gssrpc__null_auth,
                       gssrpc_authnone_create, gssrpc_xdr_opaque_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed_0, reply_body,
                          C2RustUnnamed_1, rejected_reply, C2RustUnnamed_2,
                          C2RustUnnamed_3, reject_stat, AUTH_ERROR,
                          RPC_MISMATCH, accepted_reply, C2RustUnnamed_4,
                          C2RustUnnamed_5, C2RustUnnamed_6, accept_stat,
                          SYSTEM_ERR, GARBAGE_ARGS, PROC_UNAVAIL,
                          PROG_MISMATCH, PROG_UNAVAIL, SUCCESS, reply_stat,
                          MSG_DENIED, MSG_ACCEPTED, call_body, msg_type,
                          REPLY, CALL, gssrpc_xdr_callhdr,
                          gssrpc_xdr_replymsg, gssrpc__seterr_reply};
pub use self::clnt_h::{clnt_stat, RPC_FAILED, RPC_PROGNOTREGISTERED,
                       RPC_PMAPFAILURE, RPC_UNKNOWNPROTO, RPC_UNKNOWNHOST,
                       RPC_SYSTEMERROR, RPC_CANTDECODEARGS, RPC_PROCUNAVAIL,
                       RPC_PROGVERSMISMATCH, RPC_PROGUNAVAIL, RPC_AUTHERROR,
                       RPC_VERSMISMATCH, RPC_TIMEDOUT, RPC_CANTRECV,
                       RPC_CANTSEND, RPC_CANTDECODERES, RPC_CANTENCODEARGS,
                       RPC_SUCCESS, rpc_err, C2RustUnnamed_7, C2RustUnnamed_8,
                       C2RustUnnamed_9, CLIENT, clnt_ops,
                       gssrpc_rpc_createrr};
use self::sys_socket_h::{socket, getsockname, connect, send, recvfrom};
use self::stdlib_h::{malloc, free};
use self::rpc_h::{gssrpc_bindresvport_sa, gssrpc__rpc_dtablesize};
use self::ioctl_h::ioctl;
use self::errno_h::__errno_location;
use self::pmap_clnt_h::gssrpc_pmap_getport;
/*
 * Private data kept per client handle
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "84:8"]
pub struct cu_data {
    pub cu_sock: libc::c_int,
    pub cu_closeit: libc::c_int,
    pub cu_raddr: sockaddr_in,
    pub cu_rlen: libc::c_int,
    pub cu_laddr: sockaddr_in,
    pub cu_llen: socklen_t,
    pub cu_wait: timeval,
    pub cu_total: timeval,
    pub cu_error: rpc_err,
    pub cu_outxdrs: XDR,
    pub cu_xdrpos: u_int,
    pub cu_sendsz: u_int,
    pub cu_outbuf: *mut libc::c_char,
    pub cu_recvsz: u_int,
    pub cu_inbuf: [libc::c_char; 1],
}
#[c2rust::src_loc = "72:24"]
static mut udp_ops: clnt_ops =
    unsafe {
        {
            let mut init =
                clnt_ops{cl_call:
                             Some(clntudp_call as
                                      unsafe extern "C" fn(_: *mut CLIENT,
                                                           _: rpcproc_t,
                                                           _: xdrproc_t,
                                                           _:
                                                               *mut libc::c_void,
                                                           _: xdrproc_t,
                                                           _:
                                                               *mut libc::c_void,
                                                           _: timeval)
                                          -> clnt_stat),
                         cl_abort:
                             Some(clntudp_abort as
                                      unsafe extern "C" fn(_: *mut CLIENT)
                                          -> ()),
                         cl_geterr:
                             Some(clntudp_geterr as
                                      unsafe extern "C" fn(_: *mut CLIENT,
                                                           _: *mut rpc_err)
                                          -> ()),
                         cl_freeres:
                             Some(clntudp_freeres as
                                      unsafe extern "C" fn(_: *mut CLIENT,
                                                           _: xdrproc_t,
                                                           _:
                                                               *mut libc::c_void)
                                          -> libc::c_int),
                         cl_destroy:
                             Some(clntudp_destroy as
                                      unsafe extern "C" fn(_: *mut CLIENT)
                                          -> ()),
                         cl_control:
                             Some(clntudp_control as
                                      unsafe extern "C" fn(_: *mut CLIENT,
                                                           _: libc::c_int,
                                                           _:
                                                               *mut libc::c_void)
                                          -> libc::c_int),};
            init
        }
    };
/*
 * Create a UDP based client handle.
 * If *sockp<0, *sockp is set to a newly created UPD socket.
 * If raddr->sin_port is 0 a binder on the remote machine
 * is consulted for the correct port number.
 * NB: It is the clients responsibility to close *sockp.
 * NB: The rpch->cl_auth is initialized to null authentication.
 *     Caller may wish to set this something more useful.
 *
 * wait is the amount of time used between retransmitting a call if
 * no response has been heard;  retransmition occurs until the actual
 * rpc call times out.
 *
 * sendsz and recvsz are the maximum allowable packet sizes that can be
 * sent and received.
 */
#[no_mangle]
#[c2rust::src_loc = "118:1"]
pub unsafe extern "C" fn gssrpc_clntudp_bufcreate(mut raddr: *mut sockaddr_in,
                                                  mut program: rpcprog_t,
                                                  mut version: rpcvers_t,
                                                  mut wait: timeval,
                                                  mut sockp: *mut libc::c_int,
                                                  mut sendsz: u_int,
                                                  mut recvsz: u_int)
 -> *mut CLIENT {
    let mut current_block: u64;
    let mut cl: *mut CLIENT = 0 as *mut CLIENT;
    let mut cu: *mut cu_data = 0 as *mut cu_data;
    let mut now: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut call_msg: rpc_msg =
        rpc_msg{rm_xid: 0,
                rm_direction: CALL,
                ru:
                    C2RustUnnamed_0{RM_cmb:
                                        call_body{cb_rpcvers: 0,
                                                  cb_prog: 0,
                                                  cb_vers: 0,
                                                  cb_proc: 0,
                                                  cb_cred:
                                                      opaque_auth{oa_flavor:
                                                                      0,
                                                                  oa_base:
                                                                      0 as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char,
                                                                  oa_length:
                                                                      0,},
                                                  cb_verf:
                                                      opaque_auth{oa_flavor:
                                                                      0,
                                                                  oa_base:
                                                                      0 as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char,
                                                                  oa_length:
                                                                      0,},},},};
    cl =
        malloc(::std::mem::size_of::<CLIENT>() as libc::c_ulong) as
            *mut CLIENT;
    if cl.is_null() {
        fprintf(stderr,
                b"clntudp_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        gssrpc_rpc_createrr.cf_stat = RPC_SYSTEMERROR;
        gssrpc_rpc_createrr.cf_error.ru.RE_errno = *__errno_location()
    } else {
        sendsz =
            sendsz.wrapping_add(3 as libc::c_int as
                                    libc::c_uint).wrapping_div(4 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint).wrapping_mul(4
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint);
        recvsz =
            recvsz.wrapping_add(3 as libc::c_int as
                                    libc::c_uint).wrapping_div(4 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint).wrapping_mul(4
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint);
        cu =
            malloc((::std::mem::size_of::<cu_data>() as
                        libc::c_ulong).wrapping_add(sendsz as
                                                        libc::c_ulong).wrapping_add(recvsz
                                                                                        as
                                                                                        libc::c_ulong))
                as *mut cu_data;
        if cu.is_null() {
            fprintf(stderr,
                    b"clntudp_create: out of memory\n\x00" as *const u8 as
                        *const libc::c_char);
            gssrpc_rpc_createrr.cf_stat = RPC_SYSTEMERROR;
            gssrpc_rpc_createrr.cf_error.ru.RE_errno = *__errno_location()
        } else {
            (*cu).cu_outbuf =
                &mut *(*cu).cu_inbuf.as_mut_ptr().offset(recvsz as isize) as
                    *mut libc::c_char;
            gettimeofday(&mut now, 0 as *mut timezone);
            if (*raddr).sin_port as libc::c_int == 0 as libc::c_int {
                let mut port: u_short = 0;
                port =
                    gssrpc_pmap_getport(raddr, program, version,
                                        IPPROTO_UDP as libc::c_int as
                                            rpcprot_t);
                if port as libc::c_int == 0 as libc::c_int {
                    current_block = 9429719425824199949;
                } else {
                    (*raddr).sin_port = htons(port);
                    current_block = 4808432441040389987;
                }
            } else { current_block = 4808432441040389987; }
            match current_block {
                9429719425824199949 => { }
                _ => {
                    (*cl).cl_ops = &mut udp_ops;
                    (*cl).cl_private = cu as caddr_t as *mut libc::c_void;
                    (*cu).cu_raddr = *raddr;
                    (*cu).cu_rlen =
                        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                            as libc::c_int;
                    (*cu).cu_wait = wait;
                    (*cu).cu_total.tv_sec = -(1 as libc::c_int) as __time_t;
                    (*cu).cu_total.tv_usec =
                        -(1 as libc::c_int) as __suseconds_t;
                    (*cu).cu_sendsz = sendsz;
                    (*cu).cu_recvsz = recvsz;
                    call_msg.rm_xid =
                        (getpid() as libc::c_long ^ now.tv_sec ^ now.tv_usec)
                            as uint32_t;
                    call_msg.rm_direction = CALL;
                    call_msg.ru.RM_cmb.cb_rpcvers =
                        2 as libc::c_int as uint32_t;
                    call_msg.ru.RM_cmb.cb_prog = program;
                    call_msg.ru.RM_cmb.cb_vers = version;
                    gssrpc_xdrmem_create(&mut (*cu).cu_outxdrs,
                                         (*cu).cu_outbuf, sendsz, XDR_ENCODE);
                    if !(gssrpc_xdr_callhdr(&mut (*cu).cu_outxdrs,
                                            &mut call_msg) == 0) {
                        (*cu).cu_xdrpos =
                            Some((*(*cu).cu_outxdrs.x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(&mut (*cu).cu_outxdrs);
                        if *sockp < 0 as libc::c_int {
                            let mut dontblock: libc::c_int = 1 as libc::c_int;
                            *sockp =
                                socket(2 as libc::c_int,
                                       SOCK_DGRAM as libc::c_int,
                                       IPPROTO_UDP as libc::c_int);
                            if *sockp < 0 as libc::c_int {
                                gssrpc_rpc_createrr.cf_stat = RPC_SYSTEMERROR;
                                gssrpc_rpc_createrr.cf_error.ru.RE_errno =
                                    *__errno_location();
                                current_block = 9429719425824199949;
                            } else {
                                /* attempt to bind to prov port */
                                gssrpc_bindresvport_sa(*sockp,
                                                       0 as *mut sockaddr);
                                /* the sockets rpc controls are non-blocking */
                                ioctl(*sockp,
                                      0x5421 as libc::c_int as libc::c_ulong,
                                      &mut dontblock as *mut libc::c_int as
                                          *mut libc::c_char);
                                (*cu).cu_closeit = 1 as libc::c_int;
                                current_block = 16799951812150840583;
                            }
                        } else {
                            (*cu).cu_closeit = 0 as libc::c_int;
                            current_block = 16799951812150840583;
                        }
                        match current_block {
                            9429719425824199949 => { }
                            _ => {
                                if !(connect(*sockp, raddr as *mut sockaddr,
                                             ::std::mem::size_of::<sockaddr_in>()
                                                 as libc::c_ulong as
                                                 socklen_t) <
                                         0 as libc::c_int) {
                                    (*cu).cu_llen =
                                        ::std::mem::size_of::<sockaddr_in>()
                                            as libc::c_ulong as socklen_t;
                                    if !(getsockname(*sockp,
                                                     &mut (*cu).cu_laddr as
                                                         *mut sockaddr_in as
                                                         *mut sockaddr,
                                                     &mut (*cu).cu_llen) <
                                             0 as libc::c_int) {
                                        (*cu).cu_sock = *sockp;
                                        (*cl).cl_auth =
                                            gssrpc_authnone_create();
                                        return cl
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !cu.is_null() { free(cu as caddr_t as *mut libc::c_void); }
    if !cl.is_null() { free(cl as caddr_t as *mut libc::c_void); }
    return 0 as *mut libc::c_void as *mut CLIENT;
}
/*
 * UDP based rpc.
 * CLIENT *
 * clntudp_create(raddr, program, version, wait, sockp)
 *	struct sockaddr_in *raddr;
 *	rpcprog_t program;
 *	rpcvers_t version;
 *	struct timeval wait;
 *	int *sockp;
 *
 * Same as above, but you specify max packet sizes.
 * CLIENT *
 * clntudp_bufcreate(raddr, program, version, wait, sockp, sendsz, recvsz)
 *	struct sockaddr_in *raddr;
 *	rpcprog_t program;
 *	rpcvers_t version;
 *	struct timeval wait;
 *	int *sockp;
 *	u_int sendsz;
 *	u_int recvsz;
 */
#[no_mangle]
#[c2rust::src_loc = "214:1"]
pub unsafe extern "C" fn gssrpc_clntudp_create(mut raddr: *mut sockaddr_in,
                                               mut program: rpcprog_t,
                                               mut version: rpcvers_t,
                                               mut wait: timeval,
                                               mut sockp: *mut libc::c_int)
 -> *mut CLIENT {
    return gssrpc_clntudp_bufcreate(raddr, program, version, wait, sockp,
                                    8800 as libc::c_int as u_int,
                                    8800 as libc::c_int as u_int);
}
/* @(#)clnt_udp.c	2.2 88/08/01 4.0 RPCSRC */
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
 * clnt_udp.c, Implements a UDP/IP based, client side RPC.
 */
/*
 * UDP bases client side rpc operations
 */
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn clntudp_call(mut cl: *mut CLIENT, mut proc_0: rpcproc_t,
                                  mut xargs: xdrproc_t,
                                  mut argsp: *mut libc::c_void,
                                  mut xresults: xdrproc_t,
                                  mut resultsp: *mut libc::c_void,
                                  mut utimeout: timeval) -> clnt_stat 
 /* seconds to wait before
					 * giving up */
 {
    let mut current_block: u64; /* Assumes recvfrom uses same type */
    let mut cu: *mut cu_data = (*cl).cl_private as *mut cu_data;
    let mut xdrs: *mut XDR = 0 as *mut XDR;
    let mut outlen: libc::c_int = 0;
    let mut inlen: ssize_t = 0;
    let mut fromlen: socklen_t = 0;
    let mut readfds: fd_set = fd_set{__fds_bits: [0; 16],};
    let mut mask: fd_set = fd_set{__fds_bits: [0; 16],};
    /* def FD_SETSIZE */
    let mut from: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],}; /* number of times to refresh cred */
    let mut reply_msg: rpc_msg =
        rpc_msg{rm_xid: 0,
                rm_direction: CALL,
                ru:
                    C2RustUnnamed_0{RM_cmb:
                                        call_body{cb_rpcvers: 0,
                                                  cb_prog: 0,
                                                  cb_vers: 0,
                                                  cb_proc: 0,
                                                  cb_cred:
                                                      opaque_auth{oa_flavor:
                                                                      0,
                                                                  oa_base:
                                                                      0 as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char,
                                                                  oa_length:
                                                                      0,},
                                                  cb_verf:
                                                      opaque_auth{oa_flavor:
                                                                      0,
                                                                  oa_base:
                                                                      0 as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char,
                                                                  oa_length:
                                                                      0,},},},};
    let mut reply_xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut time_waited: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut seltimeout: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut ok: libc::c_int = 0;
    let mut nrefreshes: libc::c_int = 2 as libc::c_int;
    let mut timeout: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut procl: libc::c_long = proc_0 as libc::c_long;
    if (*cu).cu_total.tv_usec == -(1 as libc::c_int) as libc::c_long {
        timeout = utimeout
        /* use supplied timeout */
    } else {
        timeout = (*cu).cu_total
        /* use default timeout */
    }
    time_waited.tv_sec = 0 as libc::c_int as __time_t;
    time_waited.tv_usec = 0 as libc::c_int as __suseconds_t;
    loop  {
        xdrs = &mut (*cu).cu_outxdrs;
        (*xdrs).x_op = XDR_ENCODE;
        Some((*(*xdrs).x_ops).x_setpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                  (*cu).cu_xdrpos);
        /*
	 * the transaction is the first thing in the out buffer
	 */
        let ref mut fresh0 =
            *((*cu).cu_outbuf as *mut libc::c_void as *mut uint32_t);
        *fresh0 = (*fresh0).wrapping_add(1);
        if Some((*(*xdrs).x_ops).x_putlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                    &mut procl)
               == 0 ||
               Some((*(*(*cl).cl_auth).ah_ops).ah_marshal.expect("non-null function pointer")).expect("non-null function pointer")((*cl).cl_auth,
                                                                                                                                   xdrs)
                   == 0 ||
               Some((*(*(*cl).cl_auth).ah_ops).ah_wrap.expect("non-null function pointer")).expect("non-null function pointer")((*cl).cl_auth,
                                                                                                                                xdrs,
                                                                                                                                xargs,
                                                                                                                                argsp
                                                                                                                                    as
                                                                                                                                    caddr_t)
                   == 0 {
            (*cu).cu_error.re_status = RPC_CANTENCODEARGS;
            return (*cu).cu_error.re_status
        }
        outlen =
            Some((*(*xdrs).x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs)
                as libc::c_int;
        'c_4410:
            loop  {
                if send((*cu).cu_sock, (*cu).cu_outbuf as *const libc::c_void,
                        outlen as u_int as size_t, 0 as libc::c_int) !=
                       outlen as libc::c_long {
                    (*cu).cu_error.ru.RE_errno = *__errno_location();
                    (*cu).cu_error.re_status = RPC_CANTSEND;
                    return (*cu).cu_error.re_status
                }
                /*
	 * Hack to provide rpc-based message passing
	 */
                if timeout.tv_sec == 0 as libc::c_int as libc::c_long &&
                       timeout.tv_usec == 0 as libc::c_int as libc::c_long {
                    (*cu).cu_error.re_status = RPC_TIMEDOUT;
                    return (*cu).cu_error.re_status
                }
                /*
	 * sub-optimal code appears here because we have
	 * some clock time to spare while the packets are in flight.
	 * (We assume that this is actually only executed once.)
	 */
                reply_msg.ru.RM_rmb.ru.RP_ar.ar_verf = gssrpc__null_auth;
                reply_msg.ru.RM_rmb.ru.RP_ar.ru.AR_results.where_0 =
                    0 as caddr_t;
                reply_msg.ru.RM_rmb.ru.RP_ar.ru.AR_results.proc_0 =
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut XDR,
                                                                        _:
                                                                            *mut libc::c_void)
                                                       -> libc::c_int>,
                                            xdrproc_t>(Some(gssrpc_xdr_void as
                                                                unsafe extern "C" fn(_:
                                                                                         *mut XDR,
                                                                                     _:
                                                                                         *mut libc::c_void)
                                                                    ->
                                                                        libc::c_int));
                let mut __d0: libc::c_int = 0;
                let mut __d1: libc::c_int = 0;
                let fresh1 = &mut __d0;
                let fresh2;
                let fresh3 = &mut __d1;
                let fresh4;
                let fresh5 =
                    (::std::mem::size_of::<fd_set>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>()
                                                         as libc::c_ulong);
                let fresh6 =
                    &mut *mask.__fds_bits.as_mut_ptr().offset(0 as libc::c_int
                                                                  as isize) as
                        *mut __fd_mask;
                asm!("cld; rep; stosq" : "={cx}" (fresh2), "={di}" (fresh4) :
                     "{ax}" (0 as libc::c_int), "0"
                     (c2rust_asm_casts::AsmCast::cast_in(fresh1, fresh5)), "1"
                     (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh6)) :
                     "memory" : "volatile");
                c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh5, fresh2);
                c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh6, fresh4);
                mask.__fds_bits[((*cu).cu_sock /
                                     (8 as libc::c_int *
                                          ::std::mem::size_of::<__fd_mask>()
                                              as libc::c_ulong as
                                              libc::c_int)) as usize] |=
                    ((1 as libc::c_ulong) <<
                         (*cu).cu_sock %
                             (8 as libc::c_int *
                                  ::std::mem::size_of::<__fd_mask>() as
                                      libc::c_ulong as libc::c_int)) as
                        __fd_mask;
                loop 
                     /* def FD_SETSIZE */
                     {
                    readfds = mask;
                    seltimeout = (*cu).cu_wait;
                    match select(gssrpc__rpc_dtablesize(), &mut readfds,
                                 0 as *mut libc::c_void as *mut fd_set,
                                 0 as *mut libc::c_void as *mut fd_set,
                                 &mut seltimeout) {
                        0 => {
                            time_waited.tv_sec += (*cu).cu_wait.tv_sec;
                            time_waited.tv_usec += (*cu).cu_wait.tv_usec;
                            while time_waited.tv_usec >=
                                      1000000 as libc::c_int as libc::c_long {
                                time_waited.tv_sec += 1;
                                time_waited.tv_usec -=
                                    1000000 as libc::c_int as libc::c_long
                            }
                            if time_waited.tv_sec < timeout.tv_sec ||
                                   time_waited.tv_sec == timeout.tv_sec &&
                                       time_waited.tv_usec < timeout.tv_usec {
                                continue 'c_4410 ;
                            }
                            (*cu).cu_error.re_status = RPC_TIMEDOUT;
                            return (*cu).cu_error.re_status
                        }
                        -1 => {
                            /*
		 * buggy in other cases because time_waited is not being
		 * updated.
		 */
                            if *__errno_location() == 4 as libc::c_int {
                                continue ;
                            }
                            (*cu).cu_error.ru.RE_errno = *__errno_location();
                            (*cu).cu_error.re_status = RPC_CANTRECV;
                            return (*cu).cu_error.re_status
                        }
                        _ => {
                            loop  {
                                fromlen =
                                    ::std::mem::size_of::<sockaddr>() as
                                        libc::c_ulong as socklen_t;
                                inlen =
                                    recvfrom((*cu).cu_sock,
                                             (*cu).cu_inbuf.as_mut_ptr() as
                                                 *mut libc::c_void,
                                             (*cu).cu_recvsz as size_t,
                                             0 as libc::c_int,
                                             &mut from as *mut sockaddr_in as
                                                 *mut sockaddr, &mut fromlen);
                                if !(inlen < 0 as libc::c_int as libc::c_long
                                         &&
                                         *__errno_location() ==
                                             4 as libc::c_int) {
                                    break ;
                                }
                            }
                            if inlen < 0 as libc::c_int as libc::c_long {
                                if *__errno_location() == 11 as libc::c_int {
                                    continue ;
                                }
                                (*cu).cu_error.ru.RE_errno =
                                    *__errno_location();
                                (*cu).cu_error.re_status = RPC_CANTRECV;
                                return (*cu).cu_error.re_status
                            } else {
                                if (inlen as size_t) <
                                       ::std::mem::size_of::<uint32_t>() as
                                           libc::c_ulong {
                                    continue ;
                                }
                                /* see if reply transaction id matches sent id */
                                if !(*((*cu).cu_inbuf.as_mut_ptr() as
                                           *mut libc::c_void as *mut uint32_t)
                                         !=
                                         *((*cu).cu_outbuf as
                                               *mut libc::c_void as
                                               *mut uint32_t)) {
                                    break ;
                                }
                            }
                        }
                    }
                }
                /* we now assume we have the proper reply */
                /*
	 * now decode and validate the response
	 */
                gssrpc_xdrmem_create(&mut reply_xdrs,
                                     (*cu).cu_inbuf.as_mut_ptr(),
                                     inlen as u_int, XDR_DECODE);
                ok = gssrpc_xdr_replymsg(&mut reply_xdrs, &mut reply_msg);
                /* XDR_DESTROY(&reply_xdrs);  save a few cycles on noop destroy */
                if ok != 0 {
                    current_block = 4888910987971495881;
                    break ;
                } else { current_block = 17075014677070940716; break ; }
            }
        match current_block {
            17075014677070940716 => {
                /*
		 * It's possible for xdr_replymsg() to fail partway
		 * through its attempt to decode the result from the
		 * server. If this happens, it will leave the reply
		 * structure partially populated with dynamically
		 * allocated memory. (This can happen if someone uses
		 * clntudp_bufcreate() to create a CLIENT handle and
		 * specifies a receive buffer size that is too small.)
		 * This memory must be free()ed to avoid a leak.
		 */
                let mut op: xdr_op =
                    reply_xdrs.x_op; /* end of valid reply message */
                reply_xdrs.x_op =
                    XDR_FREE; /* end of unsuccessful completion */
                gssrpc_xdr_replymsg(&mut reply_xdrs,
                                    &mut reply_msg); /* end successful completion */
                reply_xdrs.x_op = op;
                (*cu).cu_error.re_status = RPC_CANTDECODERES;
                current_block = 2310077433060450808;
                break ;
            }
            _ => {
                gssrpc__seterr_reply(&mut reply_msg, &mut (*cu).cu_error);
                if (*cu).cu_error.re_status as libc::c_uint ==
                       RPC_SUCCESS as libc::c_int as libc::c_uint {
                    if Some((*(*(*cl).cl_auth).ah_ops).ah_validate.expect("non-null function pointer")).expect("non-null function pointer")((*cl).cl_auth,
                                                                                                                                            &mut reply_msg.ru.RM_rmb.ru.RP_ar.ar_verf)
                           == 0 {
                        (*cu).cu_error.re_status = RPC_AUTHERROR;
                        (*cu).cu_error.ru.RE_why = AUTH_INVALIDRESP
                    } else if Some((*(*(*cl).cl_auth).ah_ops).ah_unwrap.expect("non-null function pointer")).expect("non-null function pointer")((*cl).cl_auth,
                                                                                                                                                 &mut reply_xdrs,
                                                                                                                                                 xresults,
                                                                                                                                                 resultsp
                                                                                                                                                     as
                                                                                                                                                     caddr_t)
                                  == 0 {
                        if (*cu).cu_error.re_status as libc::c_uint ==
                               RPC_SUCCESS as libc::c_int as libc::c_uint {
                            (*cu).cu_error.re_status = RPC_CANTDECODERES
                        }
                    }
                    current_block = 11796148217846552555;
                    break ;
                } else {
                    /* maybe our credentials need to be refreshed ... */
                    if !(nrefreshes > 0 as libc::c_int &&
                             Some((*(*(*cl).cl_auth).ah_ops).ah_refresh.expect("non-null function pointer")).expect("non-null function pointer")((*cl).cl_auth,
                                                                                                                                                 &mut reply_msg)
                                 != 0) {
                        current_block = 11796148217846552555;
                        break ;
                    }
                    nrefreshes -= 1
                }
            }
        }
    }
    match current_block {
        11796148217846552555 => {
            /* free verifier */
            if reply_msg.ru.RM_rmb.rp_stat as libc::c_uint ==
                   MSG_ACCEPTED as libc::c_int as libc::c_uint &&
                   !reply_msg.ru.RM_rmb.ru.RP_ar.ar_verf.oa_base.is_null() {
                (*xdrs).x_op = XDR_FREE;
                gssrpc_xdr_opaque_auth(xdrs,
                                       &mut reply_msg.ru.RM_rmb.ru.RP_ar.ar_verf);
            }
        }
        _ => { }
    }
    return (*cu).cu_error.re_status;
}
#[c2rust::src_loc = "414:1"]
unsafe extern "C" fn clntudp_geterr(mut cl: *mut CLIENT,
                                    mut errp: *mut rpc_err) {
    let mut cu: *mut cu_data = (*cl).cl_private as *mut cu_data;
    *errp = (*cu).cu_error;
}
#[c2rust::src_loc = "425:1"]
unsafe extern "C" fn clntudp_freeres(mut cl: *mut CLIENT,
                                     mut xdr_res: xdrproc_t,
                                     mut res_ptr: *mut libc::c_void)
 -> libc::c_int {
    let mut cu: *mut cu_data = (*cl).cl_private as *mut cu_data;
    let mut xdrs: *mut XDR = &mut (*cu).cu_outxdrs;
    (*xdrs).x_op = XDR_FREE;
    return ::std::mem::transmute::<_,
                                   fn(_: _, _: _)
                                       ->
                                           libc::c_int>(Some(xdr_res.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                               res_ptr);
}
/*ARGSUSED*/
#[c2rust::src_loc = "440:1"]
unsafe extern "C" fn clntudp_abort(mut h: *mut CLIENT) { }
#[c2rust::src_loc = "445:1"]
unsafe extern "C" fn clntudp_control(mut cl: *mut CLIENT,
                                     mut request: libc::c_int,
                                     mut info: *mut libc::c_void)
 -> libc::c_int {
    let mut cu: *mut cu_data = (*cl).cl_private as *mut cu_data;
    match request {
        1 => { (*cu).cu_total = *(info as *mut timeval) }
        2 => { *(info as *mut timeval) = (*cu).cu_total }
        4 => { (*cu).cu_wait = *(info as *mut timeval) }
        5 => { *(info as *mut timeval) = (*cu).cu_wait }
        3 => { *(info as *mut sockaddr_in) = (*cu).cu_raddr }
        6 => { *(info as *mut sockaddr_in) = (*cu).cu_laddr }
        _ => { return 0 as libc::c_int }
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "478:1"]
unsafe extern "C" fn clntudp_destroy(mut cl: *mut CLIENT) {
    let mut cu: *mut cu_data = (*cl).cl_private as *mut cu_data;
    if (*cu).cu_closeit != 0 { close((*cu).cu_sock); }
    if (*(*cu).cu_outxdrs.x_ops).x_destroy.is_some() {
        Some((*(*cu).cu_outxdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut (*cu).cu_outxdrs);
    }
    free(cu as caddr_t as *mut libc::c_void);
    free(cl as caddr_t as *mut libc::c_void);
}
