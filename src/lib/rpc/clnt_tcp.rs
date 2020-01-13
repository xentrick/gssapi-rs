use ::libc;
use ::c2rust_asm_casts;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:55"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:55"]
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:55"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:55"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdio.h:55"]
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
#[c2rust::header_src = "/usr/include/unistd.h:56"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::{__socklen_t, __pid_t};
    use super::stddef_h::size_t;
    use super::stdio_h::ssize_t;
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
        #[no_mangle]
        #[c2rust::src_loc = "628:1"]
        pub fn getpid() -> __pid_t;
    }
}
#[c2rust::header_src = "/usr/include/sys/types.h:57"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_short, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:57"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:57"]
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
#[c2rust::header_src = "/usr/include/sys/select.h:57"]
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
#[c2rust::header_src = "/usr/include/sys/time.h:57"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:57"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:57"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:57"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:57"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:57"]
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
        #[c2rust::src_loc = "375:1"]
        pub fn ntohl(__netlong: uint32_t) -> uint32_t;
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:57"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:57"]
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
        /* XDR pseudo records for tcp */
        #[no_mangle]
        #[c2rust::src_loc = "321:1"]
        pub fn gssrpc_xdrrec_create(xdrs: *mut XDR, _: u_int, _: u_int,
                                    _: caddr_t,
                                    _:
                                        Option<unsafe extern "C" fn(_:
                                                                        caddr_t,
                                                                    _:
                                                                        caddr_t,
                                                                    _:
                                                                        libc::c_int)
                                                   -> libc::c_int>,
                                    _:
                                        Option<unsafe extern "C" fn(_:
                                                                        caddr_t,
                                                                    _:
                                                                        caddr_t,
                                                                    _:
                                                                        libc::c_int)
                                                   -> libc::c_int>);
        /* make end of xdr record */
        #[no_mangle]
        #[c2rust::src_loc = "326:1"]
        pub fn gssrpc_xdrrec_endofrecord(_: *mut XDR, _: libc::c_int)
         -> libc::c_int;
        /* move to beginning of next record */
        #[no_mangle]
        #[c2rust::src_loc = "329:1"]
        pub fn gssrpc_xdrrec_skiprecord(xdrs: *mut XDR) -> libc::c_int;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:57"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:57"]
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
        /*
 * XDR routine to handle a rpc reply.
 * xdr_replymsg(xdrs, rmsg)
 * 	XDR *xdrs;
 * 	struct rpc_msg *rmsg;
 */
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
        #[no_mangle]
        #[c2rust::src_loc = "186:1"]
        pub fn gssrpc_xdr_replymsg(_: *mut XDR, _: *mut rpc_msg)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "178:1"]
        pub fn gssrpc_xdr_callhdr(_: *mut XDR, _: *mut rpc_msg)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_RPC_MSG_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:57"]
pub mod clnt_h {
    #[c2rust::src_loc = "48:1"]
    pub type clnt_stat = libc::c_uint;
    #[c2rust::src_loc = "83:2"]
    pub const RPC_FAILED: clnt_stat = 16;
    #[c2rust::src_loc = "79:2"]
    pub const RPC_PROGNOTREGISTERED: clnt_stat = 15;
    #[c2rust::src_loc = "78:2"]
    pub const RPC_PMAPFAILURE: clnt_stat = 14;
    #[c2rust::src_loc = "73:2"]
    pub const RPC_UNKNOWNPROTO: clnt_stat = 17;
    #[c2rust::src_loc = "72:2"]
    pub const RPC_UNKNOWNHOST: clnt_stat = 13;
    #[c2rust::src_loc = "67:2"]
    pub const RPC_SYSTEMERROR: clnt_stat = 12;
    #[c2rust::src_loc = "66:2"]
    pub const RPC_CANTDECODEARGS: clnt_stat = 11;
    #[c2rust::src_loc = "65:2"]
    pub const RPC_PROCUNAVAIL: clnt_stat = 10;
    #[c2rust::src_loc = "64:2"]
    pub const RPC_PROGVERSMISMATCH: clnt_stat = 9;
    #[c2rust::src_loc = "63:2"]
    pub const RPC_PROGUNAVAIL: clnt_stat = 8;
    #[c2rust::src_loc = "62:2"]
    pub const RPC_AUTHERROR: clnt_stat = 7;
    #[c2rust::src_loc = "61:2"]
    pub const RPC_VERSMISMATCH: clnt_stat = 6;
    #[c2rust::src_loc = "57:2"]
    pub const RPC_TIMEDOUT: clnt_stat = 5;
    #[c2rust::src_loc = "56:2"]
    pub const RPC_CANTRECV: clnt_stat = 4;
    #[c2rust::src_loc = "55:2"]
    pub const RPC_CANTSEND: clnt_stat = 3;
    #[c2rust::src_loc = "54:2"]
    pub const RPC_CANTDECODERES: clnt_stat = 2;
    #[c2rust::src_loc = "53:2"]
    pub const RPC_CANTENCODEARGS: clnt_stat = 1;
    #[c2rust::src_loc = "49:2"]
    pub const RPC_SUCCESS: clnt_stat = 0;
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
    /*
 * Print why creation failed
 */
    /* stderr */
    /* string */
    /*
 * Like clnt_perror(), but is more verbose in its output
 */
    /* stderr */
    /*
 * Print an English error message, given the client error code
 */
    /* stderr */
    /* string */
    /*
 * If a creation fails, the following allows the user to figure out why.
 */
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
        /* useful when cf_stat == RPC_PMAPFAILURE */
        #[no_mangle]
        #[c2rust::src_loc = "332:29"]
        pub static mut gssrpc_rpc_createrr: gssrpc_rpc_createrr;
    }
    /* !defined(GSSRPC_CLNT_H) */
    /* a more reasonable packet size */
    /* rpc imposed limit on udp msg size */
    /* string */
}
#[c2rust::header_src = "/usr/include/sys/socket.h:57"]
pub mod sys_socket_h {
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
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
        #[c2rust::src_loc = "130:1"]
        pub fn getpeername(__fd: libc::c_int, __addr: *mut sockaddr,
                           __len: *mut socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:57"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc.h:57"]
pub mod rpc_h {
    use super::socket_h::sockaddr;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn gssrpc__rpc_dtablesize() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn gssrpc_bindresvport_sa(_: libc::c_int, _: *mut sockaddr)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_RPC_H) */
}
#[c2rust::header_src = "/usr/include/errno.h:60"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:61"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/pmap_clnt.h:62"]
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
pub use self::unistd_h::{socklen_t, close, read, write, getpid};
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
                     IPPROTO_IP, in_port_t, sockaddr_in, ntohl, htons};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcprot_t, rpcproc_t,
                               rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdrmem_create,
                      gssrpc_xdrrec_create, gssrpc_xdrrec_endofrecord,
                      gssrpc_xdrrec_skiprecord};
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
                          REPLY, CALL, gssrpc__seterr_reply,
                          gssrpc_xdr_replymsg, gssrpc_xdr_callhdr};
pub use self::clnt_h::{clnt_stat, RPC_FAILED, RPC_PROGNOTREGISTERED,
                       RPC_PMAPFAILURE, RPC_UNKNOWNPROTO, RPC_UNKNOWNHOST,
                       RPC_SYSTEMERROR, RPC_CANTDECODEARGS, RPC_PROCUNAVAIL,
                       RPC_PROGVERSMISMATCH, RPC_PROGUNAVAIL, RPC_AUTHERROR,
                       RPC_VERSMISMATCH, RPC_TIMEDOUT, RPC_CANTRECV,
                       RPC_CANTSEND, RPC_CANTDECODERES, RPC_CANTENCODEARGS,
                       RPC_SUCCESS, rpc_err, C2RustUnnamed_7, C2RustUnnamed_8,
                       C2RustUnnamed_9, CLIENT, clnt_ops,
                       gssrpc_rpc_createrr};
use self::sys_socket_h::{socket, getsockname, connect, getpeername};
use self::stdlib_h::{malloc, free};
use self::rpc_h::{gssrpc__rpc_dtablesize, gssrpc_bindresvport_sa};
use self::errno_h::__errno_location;
use self::string_h::memset;
use self::pmap_clnt_h::gssrpc_pmap_getport;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "90:8"]
pub struct ct_data {
    pub ct_sock: libc::c_int,
    pub ct_closeit: libc::c_int,
    pub ct_wait: timeval,
    pub ct_waitset: libc::c_int,
    pub ct_addr: sockaddr_in,
    pub ct_error: rpc_err,
    pub ct_u: C2RustUnnamed_10,
    pub ct_mpos: u_int,
    pub ct_xdrs: XDR,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "97:2"]
pub union C2RustUnnamed_10 {
    pub ct_mcall: [libc::c_char; 24],
    pub ct_mcalli: uint32_t,
}
#[c2rust::src_loc = "81:24"]
static mut tcp_ops: clnt_ops =
    unsafe {
        {
            let mut init =
                clnt_ops{cl_call:
                             Some(clnttcp_call as
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
                             Some(clnttcp_abort as
                                      unsafe extern "C" fn(_: *mut CLIENT)
                                          -> ()),
                         cl_geterr:
                             Some(clnttcp_geterr as
                                      unsafe extern "C" fn(_: *mut CLIENT,
                                                           _: *mut rpc_err)
                                          -> ()),
                         cl_freeres:
                             Some(clnttcp_freeres as
                                      unsafe extern "C" fn(_: *mut CLIENT,
                                                           _: xdrproc_t,
                                                           _:
                                                               *mut libc::c_void)
                                          -> libc::c_int),
                         cl_destroy:
                             Some(clnttcp_destroy as
                                      unsafe extern "C" fn(_: *mut CLIENT)
                                          -> ()),
                         cl_control:
                             Some(clnttcp_control as
                                      unsafe extern "C" fn(_: *mut CLIENT,
                                                           _: libc::c_int,
                                                           _:
                                                               *mut libc::c_void)
                                          -> libc::c_int),};
            init
        }
    };
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
/* call succeeded */
/*
	 * local errors
	 */
/* can't encode arguments */
/* can't decode results */
/* failure in sending call */
/* failure in receiving result */
/* call timed out */
/*
	 * remote errors
	 */
/* rpc versions not compatible */
/* authentication error */
/* program not available */
/* program version mismatched */
/* procedure unavailable */
/* decode arguments error */
/* generic "other problem" */
/*
	 * callrpc & clnt_create errors
	 */
/* unknown host name */
/* unknown protocol */
/*
	 * _ create errors
	 */
/* the pmapper failed in its call */
/* remote program is not registered */
/*
	 * unspecified error
	 */
/*
 * Error info.
 */
/* realated system error */
/* why the auth error occurred */
/* lowest verion supported */
/* highest verion supported */
/* maybe meaningful if RPC_FAILED */
/* life boot & debugging only */
/*
 * Client rpc handle.
 * Created by individual implementations, see e.g. rpc_udp.c.
 * Client is responsible for initializing auth, see e.g. auth_none.c.
 */
/* authenticator */
/* call remote procedure */
/* abort a call */
/* get specific error code */
/* frees results */
/* destroy this structure */
/* the ioctl() of rpc */
		/* XXX CITI makes 2nd arg take u_int */
/* private stuff */
/*
 * client side rpc interface ops
 *
 * Parameter types are:
 *
 */
/*
 * enum clnt_stat
 * CLNT_CALL(rh, proc, xargs, argsp, xres, resp, timeout)
 * 	CLIENT *rh;
 *	rpcproc_t proc;
 *	xdrproc_t xargs;
 *	caddr_t argsp;
 *	xdrproc_t xres;
 *	caddr_t resp;
 *	struct timeval timeout;
 */
/*
 * void
 * CLNT_ABORT(rh);
 * 	CLIENT *rh;
 */
/*
 * struct rpc_err
 * CLNT_GETERR(rh);
 * 	CLIENT *rh;
 */
/*
 * bool_t
 * CLNT_FREERES(rh, xres, resp);
 * 	CLIENT *rh;
 *	xdrproc_t xres;
 *	caddr_t resp;
 */
/*
 * bool_t
 * CLNT_CONTROL(cl, request, info)
 *      CLIENT *cl;
 *      u_int request;
 *      char *info;
 */
/*
 * control operations that apply to both udp and tcp transports
 */
/* set timeout (timeval) */
/* get timeout (timeval) */
/* get server's address (sockaddr) */
/*
 * udp only control operations
 */
/* set retry timeout (timeval) */
/* get retry timeout (timeval) */
/*
 * new control operations
 */
/* get local address (sockaddr, getsockname)*/
/*
 * void
 * CLNT_DESTROY(rh);
 * 	CLIENT *rh;
 */
/*
 * RPCTEST is a test program which is accessable on every rpc
 * transport/port.  It is used for testing, performance evaluation,
 * and network administration.
 */
/*
 * By convention, procedure 0 takes null arguments and returns them
 */
/*
 * Below are the client handle creation routines for the various
 * implementations of client side rpc.  They can return NULL if a
 * creation failure occurs.
 */
/*
 * Memory based rpc (for speed check and testing)
 * CLIENT *
 * clntraw_create(prog, vers)
 *	rpcprog_t prog;
 *	rpcvers_t vers;
 */
/*
 * Generic client creation routine. Supported protocols are "udp" and "tcp"
 */
/*
 * TCP based rpc
 * CLIENT *
 * clnttcp_create(raddr, prog, vers, sockp, sendsz, recvsz)
 *	struct sockaddr_in *raddr;
 *	rpcprog_t prog;
 *	rpcvers_t version;
 *	int *sockp;
 *	u_int sendsz;
 *	u_int recvsz;
 */
/*
 * Create a client handle for a tcp/ip connection.
 * If *sockp<0, *sockp is set to a newly created TCP socket and it is
 * connected to raddr.  If *sockp non-negative then
 * raddr is ignored.  The rpc/tcp package does buffering
 * similar to stdio, so the client must pick send and receive buffer sizes,];
 * 0 => use the default.
 * If raddr->sin_port is 0, then a binder on the remote machine is
 * consulted for the right port number.
 * NB: *sockp is copied into a private area.
 * NB: It is the clients responsibility to close *sockp.
 * NB: The rpch->cl_auth is set null authentication.  Caller may wish to set this
 * something more useful.
 */
#[no_mangle]
#[c2rust::src_loc = "123:1"]
pub unsafe extern "C" fn gssrpc_clnttcp_create(mut raddr: *mut sockaddr_in,
                                               mut prog: rpcprog_t,
                                               mut vers: rpcvers_t,
                                               mut sockp: *mut libc::c_int,
                                               mut sendsz: u_int,
                                               mut recvsz: u_int)
 -> *mut CLIENT {
    let mut current_block: u64;
    let mut h: *mut CLIENT = 0 as *mut CLIENT;
    let mut ct: *mut ct_data = 0 as *mut ct_data;
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
    h =
        malloc(::std::mem::size_of::<CLIENT>() as libc::c_ulong) as
            *mut CLIENT;
    if h.is_null() {
        fprintf(stderr,
                b"clnttcp_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        gssrpc_rpc_createrr.cf_stat = RPC_SYSTEMERROR;
        gssrpc_rpc_createrr.cf_error.ru.RE_errno = *__errno_location()
    } else {
        ct =
            malloc(::std::mem::size_of::<ct_data>() as libc::c_ulong) as
                *mut ct_data;
        if ct.is_null() {
            fprintf(stderr,
                    b"clnttcp_create: out of memory\n\x00" as *const u8 as
                        *const libc::c_char);
            gssrpc_rpc_createrr.cf_stat = RPC_SYSTEMERROR;
            gssrpc_rpc_createrr.cf_error.ru.RE_errno = *__errno_location()
        } else {
            /*
	 * If no port number given ask the pmap for one
	 */
            if !raddr.is_null() &&
                   (*raddr).sin_port as libc::c_int == 0 as libc::c_int {
                let mut port: u_short = 0;
                port =
                    gssrpc_pmap_getport(raddr, prog, vers,
                                        IPPROTO_TCP as libc::c_int as
                                            rpcprot_t);
                if port as libc::c_int == 0 as libc::c_int {
                    free(ct as caddr_t as *mut libc::c_void);
                    free(h as caddr_t as *mut libc::c_void);
                    return 0 as *mut libc::c_void as *mut CLIENT
                }
                (*raddr).sin_port = htons(port)
            }
            /*
	 * If no socket given, open one
	 */
            if *sockp < 0 as libc::c_int {
                *sockp =
                    socket(2 as libc::c_int, SOCK_STREAM as libc::c_int,
                           IPPROTO_TCP as libc::c_int);
                gssrpc_bindresvport_sa(*sockp, 0 as *mut sockaddr);
                if *sockp < 0 as libc::c_int || raddr.is_null() ||
                       connect(*sockp, raddr as *mut sockaddr,
                               ::std::mem::size_of::<sockaddr_in>() as
                                   libc::c_ulong as socklen_t) <
                           0 as libc::c_int {
                    gssrpc_rpc_createrr.cf_stat = RPC_SYSTEMERROR;
                    gssrpc_rpc_createrr.cf_error.ru.RE_errno =
                        *__errno_location();
                    close(*sockp);
                    current_block = 136211610275891755;
                } else {
                    (*ct).ct_closeit = 1 as libc::c_int;
                    current_block = 4488286894823169796;
                }
            } else {
                (*ct).ct_closeit = 0 as libc::c_int;
                current_block = 4488286894823169796;
            }
            match current_block {
                136211610275891755 => { }
                _ => {
                    /*
	 * Set up private data struct
	 */
                    (*ct).ct_sock = *sockp;
                    (*ct).ct_wait.tv_usec = 0 as libc::c_int as __suseconds_t;
                    (*ct).ct_waitset = 0 as libc::c_int;
                    if raddr.is_null() {
                        /* Get the remote address from the socket, if it's IPv4. */
                        let mut sin: sockaddr_in =
                            sockaddr_in{sin_family: 0,
                                        sin_port: 0,
                                        sin_addr: in_addr{s_addr: 0,},
                                        sin_zero: [0; 8],};
                        let mut len: socklen_t =
                            ::std::mem::size_of::<sockaddr_in>() as
                                libc::c_ulong as socklen_t;
                        let mut ret: libc::c_int =
                            getpeername((*ct).ct_sock,
                                        &mut sin as *mut sockaddr_in as
                                            *mut sockaddr, &mut len);
                        if ret == 0 as libc::c_int &&
                               len as libc::c_ulong ==
                                   ::std::mem::size_of::<sockaddr_in>() as
                                       libc::c_ulong &&
                               sin.sin_family as libc::c_int ==
                                   2 as libc::c_int {
                            (*ct).ct_addr = sin
                        } else {
                            memset(&mut (*ct).ct_addr as *mut sockaddr_in as
                                       *mut libc::c_void, 0 as libc::c_int,
                                   ::std::mem::size_of::<sockaddr_in>() as
                                       libc::c_ulong);
                        }
                    } else { (*ct).ct_addr = *raddr }
                    /*
	 * Initialize call message
	 */
                    gettimeofday(&mut now, 0 as *mut timezone);
                    call_msg.rm_xid =
                        (getpid() as libc::c_long ^ now.tv_sec ^ now.tv_usec)
                            as uint32_t;
                    call_msg.rm_direction = CALL;
                    call_msg.ru.RM_cmb.cb_rpcvers =
                        2 as libc::c_int as uint32_t;
                    call_msg.ru.RM_cmb.cb_prog = prog;
                    call_msg.ru.RM_cmb.cb_vers = vers;
                    /*
	 * pre-serialize the staic part of the call msg and stash it away
	 */
                    gssrpc_xdrmem_create(&mut (*ct).ct_xdrs,
                                         (*ct).ct_u.ct_mcall.as_mut_ptr(),
                                         24 as libc::c_int as u_int,
                                         XDR_ENCODE);
                    if gssrpc_xdr_callhdr(&mut (*ct).ct_xdrs, &mut call_msg)
                           == 0 {
                        if (*ct).ct_closeit != 0 { close(*sockp); }
                    } else {
                        (*ct).ct_mpos =
                            Some((*(*ct).ct_xdrs.x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(&mut (*ct).ct_xdrs);
                        if (*(*ct).ct_xdrs.x_ops).x_destroy.is_some() {
                            Some((*(*ct).ct_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut (*ct).ct_xdrs);
                        }
                        /*
	 * Create a client handle which uses xdrrec for serialization
	 * and authnone for authentication.
	 */
                        gssrpc_xdrrec_create(&mut (*ct).ct_xdrs, sendsz,
                                             recvsz, ct as caddr_t,
                                             Some(readtcp as
                                                      unsafe extern "C" fn(_:
                                                                               *mut libc::c_char,
                                                                           _:
                                                                               caddr_t,
                                                                           _:
                                                                               libc::c_int)
                                                          -> libc::c_int),
                                             Some(writetcp as
                                                      unsafe extern "C" fn(_:
                                                                               *mut libc::c_char,
                                                                           _:
                                                                               caddr_t,
                                                                           _:
                                                                               libc::c_int)
                                                          -> libc::c_int));
                        (*h).cl_ops = &mut tcp_ops;
                        (*h).cl_private = ct as caddr_t as *mut libc::c_void;
                        (*h).cl_auth = gssrpc_authnone_create();
                        return h
                    }
                }
            }
        }
    }
    /*
	 * Something goofed, free stuff and barf
	 */
    free(ct as caddr_t as *mut libc::c_void); /* yuk */
    free(h as caddr_t as *mut libc::c_void);
    return 0 as *mut libc::c_void as *mut CLIENT;
}
#[c2rust::src_loc = "245:1"]
unsafe extern "C" fn clnttcp_call(mut h: *mut CLIENT, mut proc_0: rpcproc_t,
                                  mut xdr_args: xdrproc_t,
                                  mut args_ptr: *mut libc::c_void,
                                  mut xdr_results: xdrproc_t,
                                  mut results_ptr: *mut libc::c_void,
                                  mut timeout: timeval) -> clnt_stat {
    let mut ct: *mut ct_data = (*h).cl_private as *mut ct_data;
    let mut xdrs: *mut XDR = &mut (*ct).ct_xdrs;
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
    let mut x_id: uint32_t = 0;
    let mut msg_x_id: *mut uint32_t = &mut (*ct).ct_u.ct_mcalli;
    let mut shipnow: libc::c_int = 0;
    let mut refreshes: libc::c_int = 2 as libc::c_int;
    let mut procl: libc::c_long = proc_0 as libc::c_long;
    if (*ct).ct_waitset == 0 { (*ct).ct_wait = timeout }
    shipnow =
        if xdr_results.is_none() &&
               timeout.tv_sec == 0 as libc::c_int as libc::c_long &&
               timeout.tv_usec == 0 as libc::c_int as libc::c_long {
            0 as libc::c_int
        } else { 1 as libc::c_int };
    loop  {
        (*xdrs).x_op = XDR_ENCODE;
        (*ct).ct_error.re_status = RPC_SUCCESS;
        *msg_x_id = (*msg_x_id).wrapping_sub(1);
        x_id = ntohl(*msg_x_id);
        if Some((*(*xdrs).x_ops).x_putbytes.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                     (*ct).ct_u.ct_mcall.as_mut_ptr(),
                                                                                                                     (*ct).ct_mpos)
               == 0 ||
               Some((*(*xdrs).x_ops).x_putlong.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                        &mut procl)
                   == 0 ||
               Some((*(*(*h).cl_auth).ah_ops).ah_marshal.expect("non-null function pointer")).expect("non-null function pointer")((*h).cl_auth,
                                                                                                                                  xdrs)
                   == 0 ||
               Some((*(*(*h).cl_auth).ah_ops).ah_wrap.expect("non-null function pointer")).expect("non-null function pointer")((*h).cl_auth,
                                                                                                                               xdrs,
                                                                                                                               xdr_args,
                                                                                                                               args_ptr
                                                                                                                                   as
                                                                                                                                   caddr_t)
                   == 0 {
            if (*ct).ct_error.re_status as libc::c_uint ==
                   RPC_SUCCESS as libc::c_int as libc::c_uint {
                (*ct).ct_error.re_status = RPC_CANTENCODEARGS
            }
            gssrpc_xdrrec_endofrecord(xdrs, 1 as libc::c_int);
            return (*ct).ct_error.re_status
        }
        if gssrpc_xdrrec_endofrecord(xdrs, shipnow) == 0 {
            (*ct).ct_error.re_status = RPC_CANTSEND;
            return (*ct).ct_error.re_status
        }
        if shipnow == 0 { return RPC_SUCCESS }
        /*
	 * Hack to provide rpc-based message passing
	 */
        if timeout.tv_sec == 0 as libc::c_int as libc::c_long &&
               timeout.tv_usec == 0 as libc::c_int as libc::c_long {
            (*ct).ct_error.re_status = RPC_TIMEDOUT;
            return (*ct).ct_error.re_status
        }
        /*
	 * Keep receiving until we get a valid transaction id
	 */
        (*xdrs).x_op = XDR_DECODE;
        loop  {
            reply_msg.ru.RM_rmb.ru.RP_ar.ar_verf = gssrpc__null_auth;
            reply_msg.ru.RM_rmb.ru.RP_ar.ru.AR_results.where_0 = 0 as caddr_t;
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
            if gssrpc_xdrrec_skiprecord(xdrs) == 0 {
                return (*ct).ct_error.re_status
            }
            /* now decode and validate the response header */
            if gssrpc_xdr_replymsg(xdrs, &mut reply_msg) == 0 {
                /*
			 * Free some stuff allocated by xdr_replymsg()
			 * to avoid leaks, since it may allocate
			 * memory from partially successful decodes.
			 */
                let mut op: xdr_op = (*xdrs).x_op;
                (*xdrs).x_op = XDR_FREE;
                gssrpc_xdr_replymsg(xdrs, &mut reply_msg);
                (*xdrs).x_op = op;
                if (*ct).ct_error.re_status as libc::c_uint ==
                       RPC_SUCCESS as libc::c_int as libc::c_uint {
                    continue ;
                }
                return (*ct).ct_error.re_status
            } else if reply_msg.rm_xid == x_id { break ; }
        }
        /*
	 * process header
	 */
        gssrpc__seterr_reply(&mut reply_msg,
                             &mut (*ct).ct_error); /* end of unsuccessful completion */
        if (*ct).ct_error.re_status as libc::c_uint ==
               RPC_SUCCESS as libc::c_int as libc::c_uint {
            if Some((*(*(*h).cl_auth).ah_ops).ah_validate.expect("non-null function pointer")).expect("non-null function pointer")((*h).cl_auth,
                                                                                                                                   &mut reply_msg.ru.RM_rmb.ru.RP_ar.ar_verf)
                   == 0 {
                (*ct).ct_error.re_status =
                    RPC_AUTHERROR; /* end successful completion */
                (*ct).ct_error.ru.RE_why = AUTH_INVALIDRESP
            } else if Some((*(*(*h).cl_auth).ah_ops).ah_unwrap.expect("non-null function pointer")).expect("non-null function pointer")((*h).cl_auth,
                                                                                                                                        xdrs,
                                                                                                                                        xdr_results,
                                                                                                                                        results_ptr
                                                                                                                                            as
                                                                                                                                            caddr_t)
                          == 0 {
                if (*ct).ct_error.re_status as libc::c_uint ==
                       RPC_SUCCESS as libc::c_int as libc::c_uint {
                    (*ct).ct_error.re_status = RPC_CANTDECODERES
                }
            }
            break ;
        } else {
            /* maybe our credentials need to be refreshed ... */
            let fresh0 = refreshes;
            refreshes = refreshes - 1;
            if !(fresh0 != 0 &&
                     Some((*(*(*h).cl_auth).ah_ops).ah_refresh.expect("non-null function pointer")).expect("non-null function pointer")((*h).cl_auth,
                                                                                                                                        &mut reply_msg)
                         != 0) {
                break ;
            }
        }
    }
    /* free verifier ... */
    if reply_msg.ru.RM_rmb.rp_stat as libc::c_uint ==
           MSG_ACCEPTED as libc::c_int as libc::c_uint &&
           !reply_msg.ru.RM_rmb.ru.RP_ar.ar_verf.oa_base.is_null() {
        (*xdrs).x_op = XDR_FREE;
        gssrpc_xdr_opaque_auth(xdrs,
                               &mut reply_msg.ru.RM_rmb.ru.RP_ar.ar_verf);
    }
    return (*ct).ct_error.re_status;
}
#[c2rust::src_loc = "354:1"]
unsafe extern "C" fn clnttcp_geterr(mut h: *mut CLIENT,
                                    mut errp: *mut rpc_err) {
    let mut ct: *mut ct_data = (*h).cl_private as *mut ct_data;
    *errp = (*ct).ct_error;
}
#[c2rust::src_loc = "364:1"]
unsafe extern "C" fn clnttcp_freeres(mut cl: *mut CLIENT,
                                     mut xdr_res: xdrproc_t,
                                     mut res_ptr: *mut libc::c_void)
 -> libc::c_int {
    let mut ct: *mut ct_data = (*cl).cl_private as *mut ct_data;
    let mut xdrs: *mut XDR = &mut (*ct).ct_xdrs;
    (*xdrs).x_op = XDR_FREE;
    return ::std::mem::transmute::<_,
                                   fn(_: _, _: _)
                                       ->
                                           libc::c_int>(Some(xdr_res.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                               res_ptr);
}
/*ARGSUSED*/
#[c2rust::src_loc = "378:1"]
unsafe extern "C" fn clnttcp_abort(mut cl: *mut CLIENT) { }
#[c2rust::src_loc = "383:1"]
unsafe extern "C" fn clnttcp_control(mut cl: *mut CLIENT,
                                     mut request: libc::c_int,
                                     mut info: *mut libc::c_void)
 -> libc::c_int {
    let mut ct: *mut ct_data = (*cl).cl_private as *mut ct_data;
    let mut len: socklen_t = 0;
    match request {
        1 => {
            (*ct).ct_wait = *(info as *mut timeval);
            (*ct).ct_waitset = 1 as libc::c_int
        }
        2 => { *(info as *mut timeval) = (*ct).ct_wait }
        3 => { *(info as *mut sockaddr_in) = (*ct).ct_addr }
        6 => {
            len =
                ::std::mem::size_of::<sockaddr>() as libc::c_ulong as
                    socklen_t;
            if getsockname((*ct).ct_sock, info as *mut sockaddr, &mut len) <
                   0 as libc::c_int {
                return 0 as libc::c_int
            } else { return 1 as libc::c_int }
        }
        _ => { return 0 as libc::c_int }
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "416:1"]
unsafe extern "C" fn clnttcp_destroy(mut h: *mut CLIENT) {
    let mut ct: *mut ct_data = (*h).cl_private as *mut ct_data;
    if (*ct).ct_closeit != 0 { close((*ct).ct_sock); }
    if (*(*ct).ct_xdrs.x_ops).x_destroy.is_some() {
        Some((*(*ct).ct_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut (*ct).ct_xdrs);
    }
    free(ct as caddr_t as *mut libc::c_void);
    free(h as caddr_t as *mut libc::c_void);
}
/*
 * Interface between xdr serializer and tcp connection.
 * Behaves like the system calls, read & write, but keeps some error state
 * around for the rpc level.
 */
#[c2rust::src_loc = "433:1"]
unsafe extern "C" fn readtcp(mut ctptr: *mut libc::c_char, mut buf: caddr_t,
                             mut len: libc::c_int) -> libc::c_int {
    let mut ct: *mut ct_data = ctptr as *mut libc::c_void as *mut ct_data;
    let mut tout: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut mask: fd_set = fd_set{__fds_bits: [0; 16],};
    let mut readfds: fd_set = fd_set{__fds_bits: [0; 16],};
    if len == 0 as libc::c_int { return 0 as libc::c_int }
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh1 = &mut __d0;
    let fresh2;
    let fresh3 = &mut __d1;
    let fresh4;
    let fresh5 =
        (::std::mem::size_of::<fd_set>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong);
    let fresh6 =
        &mut *mask.__fds_bits.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh2), "={di}" (fresh4) : "{ax}"
         (0 as libc::c_int), "0"
         (c2rust_asm_casts::AsmCast::cast_in(fresh1, fresh5)), "1"
         (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh6)) : "memory" :
         "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh5, fresh2);
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh6, fresh4);
    mask.__fds_bits[((*ct).ct_sock /
                         (8 as libc::c_int *
                              ::std::mem::size_of::<__fd_mask>() as
                                  libc::c_ulong as libc::c_int)) as usize] |=
        ((1 as libc::c_ulong) <<
             (*ct).ct_sock %
                 (8 as libc::c_int *
                      ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as
                          libc::c_int)) as __fd_mask;
    loop 
         /* def FD_SETSIZE */
         {
        readfds = mask;
        tout = (*ct).ct_wait;
        match select(gssrpc__rpc_dtablesize(), &mut readfds,
                     0 as *mut libc::c_void as *mut fd_set,
                     0 as *mut libc::c_void as *mut fd_set, &mut tout) {
            0 => {
                (*ct).ct_error.re_status = RPC_TIMEDOUT;
                return -(1 as libc::c_int)
            }
            -1 => {
                if *__errno_location() == 4 as libc::c_int { continue ; }
                (*ct).ct_error.re_status = RPC_CANTRECV;
                (*ct).ct_error.ru.RE_errno = *__errno_location();
                return -(1 as libc::c_int)
            }
            _ => { break ; }
        }
    }
    len =
        read((*ct).ct_sock, buf as *mut libc::c_void, len as size_t) as
            libc::c_int;
    match len {
        0 => {
            /* premature eof */
            (*ct).ct_error.ru.RE_errno =
                104 as libc::c_int; /* it's really an error */
            (*ct).ct_error.re_status = RPC_CANTRECV;
            len = -(1 as libc::c_int)
        }
        -1 => {
            (*ct).ct_error.ru.RE_errno = *__errno_location();
            (*ct).ct_error.re_status = RPC_CANTRECV
        }
        _ => { }
    }
    return len;
}
#[c2rust::src_loc = "492:1"]
unsafe extern "C" fn writetcp(mut ctptr: *mut libc::c_char, mut buf: caddr_t,
                              mut len: libc::c_int) -> libc::c_int {
    let mut ct: *mut ct_data = ctptr as *mut libc::c_void as *mut ct_data;
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    cnt = len;
    while cnt > 0 as libc::c_int {
        i =
            write((*ct).ct_sock, buf as *const libc::c_void, cnt as size_t) as
                libc::c_int;
        if i == -(1 as libc::c_int) {
            (*ct).ct_error.ru.RE_errno = *__errno_location();
            (*ct).ct_error.re_status = RPC_CANTSEND;
            return -(1 as libc::c_int)
        }
        cnt -= i;
        buf = buf.offset(i as isize)
    }
    return len;
}
