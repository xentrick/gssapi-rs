use ::libc;
use ::c2rust_asm_casts;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:46"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:46"]
pub mod types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __u_short = libc::c_ushort;
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
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
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:46"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:46"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/sys/types.h:46"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_short, __u_int, __ssize_t, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:46"]
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
#[c2rust::header_src = "/usr/include/sys/select.h:46"]
pub mod select_h {
    #[c2rust::src_loc = "49:1"]
    pub type __fd_mask = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:9"]
    pub struct fd_set {
        pub fds_bits: [__fd_mask; 16],
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:46"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:46"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/unistd.h:46"]
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
#[c2rust::header_src = "/usr/include/bits/socket_type.h:48"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:48"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:48"]
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
#[c2rust::header_src = "/usr/include/sys/socket.h:48"]
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
        #[c2rust::src_loc = "112:1"]
        pub fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                    __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                           __len: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "222:1"]
        pub fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "232:1"]
        pub fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                      __addr_len: *mut socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:48"]
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
    #[c2rust::src_loc = "40:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "92:5"]
    pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
    #[c2rust::src_loc = "90:5"]
    pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
    #[c2rust::src_loc = "88:5"]
    pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
    #[c2rust::src_loc = "86:5"]
    pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
    #[c2rust::src_loc = "84:5"]
    pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
    #[c2rust::src_loc = "82:5"]
    pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
    #[c2rust::src_loc = "80:5"]
    pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
    #[c2rust::src_loc = "78:5"]
    pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
    #[c2rust::src_loc = "76:5"]
    pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
    #[c2rust::src_loc = "74:5"]
    pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
    #[c2rust::src_loc = "72:5"]
    pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
    #[c2rust::src_loc = "70:5"]
    pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
    #[c2rust::src_loc = "68:5"]
    pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
    #[c2rust::src_loc = "66:5"]
    pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
    #[c2rust::src_loc = "64:5"]
    pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
    #[c2rust::src_loc = "62:5"]
    pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
    #[c2rust::src_loc = "60:5"]
    pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
    #[c2rust::src_loc = "58:5"]
    pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
    #[c2rust::src_loc = "56:5"]
    pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
    #[c2rust::src_loc = "54:5"]
    pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
    #[c2rust::src_loc = "52:5"]
    pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
    #[c2rust::src_loc = "50:5"]
    pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
    #[c2rust::src_loc = "48:5"]
    pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
    #[c2rust::src_loc = "46:5"]
    pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "44:5"]
    pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "42:5"]
    pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "376:1"]
        pub fn ntohs(__netshort: uint16_t) -> uint16_t;
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:48"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:48"]
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
        /* true if no more input */
        #[no_mangle]
        #[c2rust::src_loc = "332:1"]
        pub fn gssrpc_xdrrec_eof(xdrs: *mut XDR) -> libc::c_int;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:48"]
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
    use super::sys_types_h::{caddr_t, u_int};
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:48"]
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
        pub ru: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed_1 {
        pub RM_cmb: call_body,
        pub RM_rmb: reply_body,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:8"]
    pub struct reply_body {
        pub rp_stat: reply_stat,
        pub ru: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_2 {
        pub RP_ar: accepted_reply,
        pub RP_dr: rejected_reply,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rejected_reply {
        pub rj_stat: reject_stat,
        pub ru: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:2"]
    pub union C2RustUnnamed_3 {
        pub RJ_versions: C2RustUnnamed_4,
        pub RJ_why: auth_stat,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:3"]
    pub struct C2RustUnnamed_4 {
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
        pub ru: C2RustUnnamed_5,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:2"]
    pub union C2RustUnnamed_5 {
        pub AR_versions: C2RustUnnamed_7,
        pub AR_results: C2RustUnnamed_6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:3"]
    pub struct C2RustUnnamed_6 {
        pub where_0: caddr_t,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:3"]
    pub struct C2RustUnnamed_7 {
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
    extern "C" {
        /*
 * XDR routine to handle a rpc message.
 * xdr_callmsg(xdrs, cmsg)
 * 	XDR *xdrs;
 * 	struct rpc_msg *cmsg;
 */
        #[no_mangle]
        #[c2rust::src_loc = "170:1"]
        pub fn gssrpc_xdr_callmsg(_: *mut XDR, _: *mut rpc_msg)
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
    }
    /* !defined(GSSRPC_RPC_MSG_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc.h:48"]
pub mod svc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:16"]
    pub struct SVCXPRT {
        pub xp_sock: libc::c_int,
        pub xp_port: u_short,
        pub xp_ops: *mut xp_ops,
        pub xp_addrlen: libc::c_int,
        pub xp_raddr: sockaddr_in,
        pub xp_verf: opaque_auth,
        pub xp_auth: *mut SVCAUTH,
        pub xp_p1: *mut libc::c_void,
        pub xp_p2: *mut libc::c_void,
        pub xp_laddrlen: libc::c_int,
        pub xp_laddr: sockaddr_in,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:9"]
    pub struct xp_ops {
        pub xp_recv: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                 _: *mut rpc_msg)
                                -> libc::c_int>,
        pub xp_stat: Option<unsafe extern "C" fn(_: *mut SVCXPRT)
                                -> xprt_stat>,
        pub xp_getargs: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                    _: xdrproc_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
        pub xp_reply: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                  _: *mut rpc_msg)
                                 -> libc::c_int>,
        pub xp_freeargs: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                     _: xdrproc_t,
                                                     _: *mut libc::c_void)
                                    -> libc::c_int>,
        pub xp_destroy: Option<unsafe extern "C" fn(_: *mut SVCXPRT) -> ()>,
    }
    #[c2rust::src_loc = "67:1"]
    pub type xprt_stat = libc::c_uint;
    #[c2rust::src_loc = "70:2"]
    pub const XPRT_IDLE: xprt_stat = 2;
    #[c2rust::src_loc = "69:2"]
    pub const XPRT_MOREREQS: xprt_stat = 1;
    #[c2rust::src_loc = "68:2"]
    pub const XPRT_DIED: xprt_stat = 0;
    use super::sys_types_h::u_short;
    use super::in_h::sockaddr_in;
    use super::auth_h::opaque_auth;
    use super::svc_auth_h::SVCAUTH;
    use super::rpc_msg_h::rpc_msg;
    use super::xdr_h::xdrproc_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "220:1"]
        pub fn gssrpc_xprt_register(_: *mut SVCXPRT);
        #[no_mangle]
        #[c2rust::src_loc = "228:1"]
        pub fn gssrpc_xprt_unregister(_: *mut SVCXPRT);
    }
    /* !defined(GSSRPC_SVC_H) */
    /* XXX add auth_gsapi_log_*? */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc_auth.h:48"]
pub mod svc_auth_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:16"]
    pub struct SVCAUTH {
        pub svc_ah_ops: *mut svc_auth_ops,
        pub svc_ah_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:9"]
    pub struct svc_auth_ops {
        pub svc_ah_wrap: Option<unsafe extern "C" fn(_: *mut SVCAUTH,
                                                     _: *mut XDR,
                                                     _: xdrproc_t, _: caddr_t)
                                    -> libc::c_int>,
        pub svc_ah_unwrap: Option<unsafe extern "C" fn(_: *mut SVCAUTH,
                                                       _: *mut XDR,
                                                       _: xdrproc_t,
                                                       _: caddr_t)
                                      -> libc::c_int>,
        pub svc_ah_destroy: Option<unsafe extern "C" fn(_: *mut SVCAUTH)
                                       -> libc::c_int>,
    }
    use super::xdr_h::{XDR, xdrproc_t};
    use super::sys_types_h::caddr_t;
    /* !defined(GSSRPC_SVC_AUTH_H) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:46"]
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
#[c2rust::header_src = "/usr/include/stdio.h:46"]
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
#[c2rust::header_src = "/usr/include/fcntl.h:46"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:46"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:46"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc.h:48"]
pub mod rpc_h {
    use super::socket_h::sockaddr;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn gssrpc_bindresvport_sa(_: libc::c_int, _: *mut sockaddr)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_RPC_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/socket-utils.h:51"]
pub mod socket_utils_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2001,2005 by the Massachusetts Institute of Technology,
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
    /* Some useful stuff cross-platform for manipulating socket addresses.
   We assume at least ipv4 sockaddr_in support.  The sockaddr_storage
   stuff comes from the ipv6 socket api enhancements; socklen_t is
   provided on some systems; the rest is just convenience for internal
   use in the krb5 tree.

   Do NOT install this file.  */
    /* for HAVE_SOCKLEN_T etc */
    /* for sockaddr_storage */
    /* for "inline" if needed */
    /*
 * There's a lot of confusion between pointers to different sockaddr
 * types, and pointers with different degrees of indirection, as in
 * the locate_kdc type functions.  Use these function to ensure we
 * don't do something silly like cast a "sockaddr **" to a
 * "sockaddr_in *".
 *
 * The casts to (void *) are to get GCC to shut up about alignment
 * increasing.
 */
    #[inline]
    #[c2rust::src_loc = "71:1"]
    pub unsafe extern "C" fn sa2sin(mut sa: *mut sockaddr)
     -> *mut sockaddr_in {
        return sa as *mut libc::c_void as *mut sockaddr_in;
    }
    #[inline]
    #[c2rust::src_loc = "75:1"]
    pub unsafe extern "C" fn sa2sin6(mut sa: *mut sockaddr)
     -> *mut sockaddr_in6 {
        return sa as *mut libc::c_void as *mut sockaddr_in6;
    }
    /* Set the IPv4 or IPv6 port on sa to port.  Do nothing if sa is not an
 * Internet socket. */
    #[inline]
    #[c2rust::src_loc = "94:1"]
    pub unsafe extern "C" fn sa_setport(mut sa: *mut sockaddr,
                                        mut port: uint16_t) {
        if (*sa).sa_family as libc::c_int == 2 as libc::c_int {
            (*sa2sin(sa)).sin_port = htons(port)
        } else if (*sa).sa_family as libc::c_int == 10 as libc::c_int {
            (*sa2sin6(sa)).sin6_port = htons(port)
        };
    }
    /* Get the Internet port number of sa, or 0 if it is not an Internet socket. */
    #[inline]
    #[c2rust::src_loc = "104:1"]
    pub unsafe extern "C" fn sa_getport(mut sa: *mut sockaddr) -> uint16_t {
        if (*sa).sa_family as libc::c_int == 2 as libc::c_int {
            return ntohs((*sa2sin(sa)).sin_port)
        } else if (*sa).sa_family as libc::c_int == 10 as libc::c_int {
            return ntohs((*sa2sin6(sa)).sin6_port)
        } else { return 0 as libc::c_int as uint16_t };
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
    use super::socket_h::sockaddr;
    use super::in_h::{sockaddr_in, sockaddr_in6, htons, in_port_t, ntohs};
    use super::stdint_uintn_h::uint16_t;
    use super::sockaddr_h::sa_family_t;
    use super::unistd_h::socklen_t;
    use super::stdlib_h::abort;
    /* SOCKET_UTILS_H */
}
use c2rust_asm_casts::AsmCastTrait;
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __u_int, __uint8_t, __uint16_t, __int32_t,
                        __uint32_t, __off_t, __off64_t, __time_t,
                        __suseconds_t, __ssize_t, __caddr_t, __socklen_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::sys_types_h::{u_short, u_int, ssize_t, caddr_t};
pub use self::struct_timeval_h::timeval;
pub use self::select_h::{__fd_mask, fd_set, select};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::unistd_h::{socklen_t, close, read, write};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage};
pub use self::sys_socket_h::{__SOCKADDR_ARG, __CONST_SOCKADDR_ARG,
                             sockaddr_x25, sockaddr_un, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, bind, getsockname, listen,
                             accept};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, C2RustUnnamed_0,
                     IPPROTO_MAX, IPPROTO_RAW, IPPROTO_MPLS, IPPROTO_UDPLITE,
                     IPPROTO_SCTP, IPPROTO_COMP, IPPROTO_PIM, IPPROTO_ENCAP,
                     IPPROTO_BEETPH, IPPROTO_MTP, IPPROTO_AH, IPPROTO_ESP,
                     IPPROTO_GRE, IPPROTO_RSVP, IPPROTO_IPV6, IPPROTO_DCCP,
                     IPPROTO_TP, IPPROTO_IDP, IPPROTO_UDP, IPPROTO_PUP,
                     IPPROTO_EGP, IPPROTO_TCP, IPPROTO_IPIP, IPPROTO_IGMP,
                     IPPROTO_ICMP, IPPROTO_IP, ntohs, htons};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdrrec_create,
                      gssrpc_xdrrec_endofrecord, gssrpc_xdrrec_skiprecord,
                      gssrpc_xdrrec_eof};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, opaque_auth,
                       gssrpc__null_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed_1, reply_body,
                          C2RustUnnamed_2, rejected_reply, C2RustUnnamed_3,
                          C2RustUnnamed_4, reject_stat, AUTH_ERROR,
                          RPC_MISMATCH, accepted_reply, C2RustUnnamed_5,
                          C2RustUnnamed_6, C2RustUnnamed_7, accept_stat,
                          SYSTEM_ERR, GARBAGE_ARGS, PROC_UNAVAIL,
                          PROG_MISMATCH, PROG_UNAVAIL, SUCCESS, reply_stat,
                          MSG_DENIED, MSG_ACCEPTED, call_body, msg_type,
                          REPLY, CALL, gssrpc_xdr_callmsg,
                          gssrpc_xdr_replymsg};
pub use self::svc_h::{SVCXPRT, xp_ops, xprt_stat, XPRT_IDLE, XPRT_MOREREQS,
                      XPRT_DIED, gssrpc_xprt_register,
                      gssrpc_xprt_unregister};
pub use self::svc_auth_h::{SVCAUTH, svc_auth_ops};
use self::stdlib_h::{malloc, free, abort};
use self::stdio_h::{stderr, fprintf, perror};
use self::fcntl_h::fcntl;
use self::errno_h::__errno_location;
use self::string_h::memset;
use self::rpc_h::gssrpc_bindresvport_sa;
pub use self::socket_utils_h::{sa2sin, sa2sin6, sa_setport, sa_getport,
                               sa_socklen};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "110:8"]
pub struct tcp_conn {
    pub strm_stat: xprt_stat,
    pub x_id: uint32_t,
    pub xdrs: XDR,
    pub verf_body: [libc::c_char; 400],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "105:8"]
pub struct tcp_rendezvous {
    pub sendsize: u_int,
    pub recvsize: u_int,
}
#[c2rust::src_loc = "74:22"]
static mut svctcp_op: xp_ops =
    unsafe {
        {
            let mut init =
                xp_ops{xp_recv:
                           Some(svctcp_recv as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: *mut rpc_msg)
                                        -> libc::c_int),
                       xp_stat:
                           Some(svctcp_stat as
                                    unsafe extern "C" fn(_: *mut SVCXPRT)
                                        -> xprt_stat),
                       xp_getargs:
                           Some(svctcp_getargs as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: xdrproc_t,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int),
                       xp_reply:
                           Some(svctcp_reply as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: *mut rpc_msg)
                                        -> libc::c_int),
                       xp_freeargs:
                           Some(svctcp_freeargs as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: xdrproc_t,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int),
                       xp_destroy:
                           Some(svctcp_destroy as
                                    unsafe extern "C" fn(_: *mut SVCXPRT)
                                        -> ()),};
            init
        }
    };
#[c2rust::src_loc = "93:22"]
static mut svctcp_rendezvous_op: xp_ops =
    unsafe {
        {
            let mut init =
                xp_ops{xp_recv:
                           Some(rendezvous_request as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: *mut rpc_msg)
                                        -> libc::c_int),
                       xp_stat:
                           Some(rendezvous_stat as
                                    unsafe extern "C" fn(_: *mut SVCXPRT)
                                        -> xprt_stat),
                       xp_getargs:
                           Some(abortx_getargs as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: xdrproc_t,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int),
                       xp_reply:
                           Some(abortx_reply as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: *mut rpc_msg)
                                        -> libc::c_int),
                       xp_freeargs:
                           Some(abortx_freeargs as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: xdrproc_t,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int),
                       xp_destroy:
                           Some(svctcp_destroy as
                                    unsafe extern "C" fn(_: *mut SVCXPRT)
                                        -> ()),};
            init
        }
    };
/*
 * Usage:
 *	xprt = svctcp_create(sock, send_buf_size, recv_buf_size);
 *
 * Creates, registers, and returns a (rpc) tcp based transporter.
 * Once *xprt is initialized, it is registered as a transporter
 * see (svc.h, xprt_register).  This routine returns
 * a NULL if a problem occurred.
 *
 * If sock<0 then a socket is created, else sock is used.
 * If the socket, sock is not bound to a port then svctcp_create
 * binds it to an arbitrary port.  The routine then starts a tcp
 * listener on the socket's associated port.  In any (successful) case,
 * xprt->xp_sock is the registered socket number and xprt->xp_port is the
 * associated port number.
 *
 * Since tcp streams do buffered io similar to stdio, the caller can specify
 * how big the send and receive buffers are via the second and third parms;
 * 0 => use the system default.
 */
#[no_mangle]
#[c2rust::src_loc = "137:1"]
pub unsafe extern "C" fn gssrpc_svctcp_create(mut sock: libc::c_int,
                                              mut sendsize: u_int,
                                              mut recvsize: u_int)
 -> *mut SVCXPRT {
    let mut madesock: libc::c_int = 0 as libc::c_int;
    let mut xprt: *mut SVCXPRT = 0 as *mut SVCXPRT;
    let mut r: *mut tcp_rendezvous = 0 as *mut tcp_rendezvous;
    let mut ss: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut sa: *mut sockaddr =
        &mut ss as *mut sockaddr_storage as *mut sockaddr;
    let mut len: socklen_t = 0;
    if sock == -(1 as libc::c_int) {
        sock =
            socket(2 as libc::c_int, SOCK_STREAM as libc::c_int,
                   IPPROTO_TCP as libc::c_int);
        if sock < 0 as libc::c_int {
            perror(b"svctcp_.c - udp socket creation problem\x00" as *const u8
                       as *const libc::c_char);
            return 0 as *mut libc::c_void as *mut SVCXPRT
        }
        fcntl(sock, 2 as libc::c_int, 1 as libc::c_int);
        madesock = 1 as libc::c_int;
        memset(&mut ss as *mut sockaddr_storage as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong);
        (*sa).sa_family = 2 as libc::c_int as sa_family_t
    } else {
        len =
            ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
                socklen_t;
        if getsockname(sock, __SOCKADDR_ARG{__sockaddr__: sa,}, &mut len) !=
               0 as libc::c_int {
            perror(b"svc_tcp.c - cannot getsockname\x00" as *const u8 as
                       *const libc::c_char);
            return 0 as *mut libc::c_void as *mut SVCXPRT
        }
    }
    if gssrpc_bindresvport_sa(sock, sa) != 0 {
        sa_setport(sa, 0 as libc::c_int as uint16_t);
        bind(sock, __CONST_SOCKADDR_ARG{__sockaddr__: sa,}, sa_socklen(sa));
    }
    len =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    if getsockname(sock, __SOCKADDR_ARG{__sockaddr__: sa,}, &mut len) !=
           0 as libc::c_int {
        perror(b"svc_tcp.c - cannot getsockname\x00" as *const u8 as
                   *const libc::c_char);
        if madesock != 0 { close(sock); }
        return 0 as *mut libc::c_void as *mut SVCXPRT
    }
    if listen(sock, 2 as libc::c_int) != 0 as libc::c_int {
        perror(b"svctcp_.c - cannot listen\x00" as *const u8 as
                   *const libc::c_char);
        if madesock != 0 { close(sock); }
        return 0 as *mut libc::c_void as *mut SVCXPRT
    }
    r =
        malloc(::std::mem::size_of::<tcp_rendezvous>() as libc::c_ulong) as
            *mut tcp_rendezvous;
    if r.is_null() {
        fprintf(stderr,
                b"svctcp_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as *mut SVCXPRT
    }
    (*r).sendsize = sendsize;
    (*r).recvsize = recvsize;
    xprt =
        malloc(::std::mem::size_of::<SVCXPRT>() as libc::c_ulong) as
            *mut SVCXPRT;
    if xprt.is_null() {
        fprintf(stderr,
                b"svctcp_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as *mut SVCXPRT
    }
    (*xprt).xp_p2 = 0 as *mut libc::c_void;
    (*xprt).xp_p1 = r as caddr_t as *mut libc::c_void;
    (*xprt).xp_auth = 0 as *mut SVCAUTH;
    (*xprt).xp_verf = gssrpc__null_auth;
    (*xprt).xp_ops = &mut svctcp_rendezvous_op;
    (*xprt).xp_port = sa_getport(sa);
    (*xprt).xp_sock = sock;
    (*xprt).xp_laddrlen = 0 as libc::c_int;
    gssrpc_xprt_register(xprt);
    return xprt;
}
/* @(#)svc.h	2.2 88/07/29 4.0 RPCSRC; from 1.20 88/02/08 SMI */
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
 * svc.h, Server-side remote procedure call interface.
 */
/*
 * This interface must manage two items concerning remote procedure calling:
 *
 * 1) An arbitrary number of transport connections upon which rpc requests
 * are received.  The two most notable transports are TCP and UDP;  they are
 * created and registered by routines in svc_tcp.c and svc_udp.c, respectively;
 * they in turn call xprt_register and xprt_unregister.
 *
 * 2) An arbitrary number of locally registered services.  Services are
 * described by the following four data: program number, version number,
 * "service dispatch" function, a transport handle, and a boolean that
 * indicates whether or not the exported program should be registered with a
 * local binder service;  if true the program's number and version and the
 * port number from the transport handle are registered with the binder.
 * These data are registered with the rpc svc system via svc_register.
 *
 * A service's dispatch function is called whenever an rpc request comes in
 * on a transport.  The request's program and version numbers must match
 * those of the registered service.  The dispatch function is passed two
 * parameters, struct svc_req * and SVCXPRT *, defined below.
 */
/*
 * Server side transport handle
 */
/* associated port number */
/* receive incomming requests */
/* get transport status */
/* get arguments */
/* send reply */
/* free mem allocated for args */
/* destroy this struct */
/* length of remote address */
/* remote address */
/* raw response verifier */
/* auth flavor of current req */
/* private */
/* private */
/* lenght of local address */
/* local address */
/*
 *  Approved way of getting address of caller
 */
/*
 * Operations defined on an SVCXPRT handle
 *
 * SVCXPRT		*xprt;
 * struct rpc_msg	*msg;
 * xdrproc_t		 xargs;
 * caddr_t		 argsp;
 */
/*
 * Service request
 */
/* service program number */
/* service protocol version */
/* the desired procedure */
/* raw creds from the wire */
/* read only cooked client cred */
/* read only svc cred/context */
/* read only client name */
/* associated transport */
/* The request's auth flavor *should* be here, but the svc_req 	*/
	/* isn't passed around everywhere it is necessary.  The 	*/
	/* transport *is* passed around, so the auth flavor it stored 	*/
	/* there.  This means that the transport must be single 	*/
	/* threaded, but other parts of SunRPC already require that. 	*/
	/*SVCAUTH		*rq_auth;	 associated auth flavor */
/*
 * Service registration
 *
 * svc_register(xprt, prog, vers, dispatch, protocol)
 *	SVCXPRT *xprt;
 *	rpcprog_t prog;
 *	rpcvers_t vers;
 *	void (*dispatch)();
 *	int protocol;  like IPPROTO_TCP or _UDP; zero means do not register
 *
 * registerrpc(prog, vers, proc, routine, inproc, outproc)
 * 	returns 0 upon success, -1 if error.
 */
/*
 * Service un-registration
 *
 * svc_unregister(prog, vers)
 *	rpcprog_t prog;
 *	rpcvers_t vers;
 */
/*
 * Transport registration.
 *
 * xprt_register(xprt)
 *	SVCXPRT *xprt;
 */
/*
 * Transport un-register
 *
 * xprt_unregister(xprt)
 *	SVCXPRT *xprt;
 */
/*
 * When the service routine is called, it must first check to see if
 * it knows about the procedure; if not, it should call svcerr_noproc
 * and return.  If so, it should deserialize its arguments via
 * SVC_GETARGS or the new SVC_GETARGS_REQ (both defined above).  If
 * the deserialization does not work, svcerr_decode should be called
 * followed by a return.  Successful decoding of the arguments should
 * be followed the execution of the procedure's code and a call to
 * svc_sendreply or the new svc_sendreply_req.
 *
 * Also, if the service refuses to execute the procedure due to too-
 * weak authentication parameters, svcerr_weakauth should be called.
 * Note: do not confuse access-control failure with weak authentication!
 *
 * NB: In pure implementations of rpc, the caller always waits for a reply
 * msg.  This message is sent when svc_sendreply is called.
 * Therefore pure service implementations should always call
 * svc_sendreply even if the function logically returns void;  use
 * xdr.h - xdr_void for the xdr routine.  HOWEVER, tcp based rpc allows
 * for the abuse of pure rpc via batched calling or pipelining.  In the
 * case of a batched call, svc_sendreply should NOT be called since
 * this would send a return message, which is what batching tries to avoid.
 * It is the service/protocol writer's responsibility to know which calls are
 * batched and which are not.  Warning: responding to batch calls may
 * deadlock the caller and server processes!
 */
/*
 * Lowest level dispatching -OR- who owns this process anyway.
 * Somebody has to wait for incoming requests and then call the correct
 * service routine.  The routine svc_run does infinite waiting; i.e.,
 * svc_run never returns.
 * Since another (co-existant) package may wish to selectively wait for
 * incoming calls or other events outside of the rpc architecture, the
 * routine svc_getreq is provided.  It must be passed readfds, the
 * "in-place" results of a select system call (see select, section 2).
 */
/*
 * Global keeper of rpc service descriptors in use
 * dynamic; must be inspected before each call to select
 */
/* RENAMED */
/* compatibility */
/* def FD_SETSIZE */
/*
 * a small program implemented by the svc_rpc implementation itself;
 * also see clnt.h for protocol numbers.
 */
/* takes fdset instead of int */
/* never returns */
/*
 * Socket to use on svcxxx_create call to get default socket
 */
/*
 * These are the existing service side transport implementations
 */
/*
 * Memory based rpc for testing and timing.
 */
/*
 * Udp based rpc.
 */
/*
 * Tcp based rpc.
 */
/*
 * Like svtcp_create(), except the routine takes any *open* UNIX file
 * descriptor as its first input.
 */
/*
 * Like svtcp_create(), except the routine takes any *open* UNIX file
 * descriptor as its first input.
 */
#[no_mangle]
#[c2rust::src_loc = "212:1"]
pub unsafe extern "C" fn gssrpc_svcfd_create(mut fd: libc::c_int,
                                             mut sendsize: u_int,
                                             mut recvsize: u_int)
 -> *mut SVCXPRT {
    return makefd_xprt(fd, sendsize, recvsize); /* truely deals with calls */
}
#[c2rust::src_loc = "222:1"]
unsafe extern "C" fn makefd_xprt(mut fd: libc::c_int, mut sendsize: u_int,
                                 mut recvsize: u_int) -> *mut SVCXPRT {
    let mut xprt: *mut SVCXPRT =
        0 as *mut SVCXPRT; /* this is a connection, not a rendezvouser */
    let mut cd: *mut tcp_conn = 0 as *mut tcp_conn;
    if fd >= 1024 as libc::c_int {
        fprintf(stderr,
                b"svc_tcp: makefd_xprt: fd too high\n\x00" as *const u8 as
                    *const libc::c_char);
        xprt = 0 as *mut SVCXPRT
    } else {
        xprt =
            malloc(::std::mem::size_of::<SVCXPRT>() as libc::c_ulong) as
                *mut SVCXPRT;
        if xprt.is_null() {
            fprintf(stderr,
                    b"svc_tcp: makefd_xprt: out of memory\n\x00" as *const u8
                        as *const libc::c_char);
        } else {
            cd =
                malloc(::std::mem::size_of::<tcp_conn>() as libc::c_ulong) as
                    *mut tcp_conn;
            if cd.is_null() {
                fprintf(stderr,
                        b"svc_tcp: makefd_xprt: out of memory\n\x00" as
                            *const u8 as *const libc::c_char);
                free(xprt as *mut libc::c_char as *mut libc::c_void);
                xprt = 0 as *mut libc::c_void as *mut SVCXPRT
            } else {
                (*cd).strm_stat = XPRT_IDLE;
                gssrpc_xdrrec_create(&mut (*cd).xdrs, sendsize, recvsize,
                                     xprt as caddr_t,
                                     Some(readtcp as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char,
                                                                   _: caddr_t,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int),
                                     Some(writetcp as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_char,
                                                                   _: caddr_t,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int));
                (*xprt).xp_p2 = 0 as *mut libc::c_void;
                (*xprt).xp_p1 = cd as caddr_t as *mut libc::c_void;
                (*xprt).xp_auth = 0 as *mut SVCAUTH;
                (*xprt).xp_verf.oa_base = (*cd).verf_body.as_mut_ptr();
                (*xprt).xp_addrlen = 0 as libc::c_int;
                (*xprt).xp_laddrlen = 0 as libc::c_int;
                (*xprt).xp_ops = &mut svctcp_op;
                (*xprt).xp_port = 0 as libc::c_int as u_short;
                (*xprt).xp_sock = fd;
                gssrpc_xprt_register(xprt);
            }
        }
    }
    return xprt;
}
/*
 * Ops vector for TCP/IP rendezvous handler
 */
#[c2rust::src_loc = "273:1"]
unsafe extern "C" fn rendezvous_request(mut xprt: *mut SVCXPRT,
                                        mut msg: *mut rpc_msg)
 -> libc::c_int {
    let mut sock: libc::c_int = 0;
    let mut r: *mut tcp_rendezvous = 0 as *mut tcp_rendezvous;
    let mut addr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut laddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut len: socklen_t = 0;
    let mut llen: socklen_t = 0;
    r = (*xprt).xp_p1 as *mut tcp_rendezvous;
    loop  {
        llen =
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t;
        len = llen;
        sock =
            accept((*xprt).xp_sock,
                   __SOCKADDR_ARG{__sockaddr__:
                                      &mut addr as *mut sockaddr_in as
                                          *mut sockaddr,}, &mut len);
        if sock < 0 as libc::c_int {
            if *__errno_location() == 4 as libc::c_int { continue ; }
            return 0 as libc::c_int
        } else {
            fcntl(sock, 2 as libc::c_int, 1 as libc::c_int);
            if getsockname(sock,
                           __SOCKADDR_ARG{__sockaddr__:
                                              &mut laddr as *mut sockaddr_in
                                                  as *mut sockaddr,},
                           &mut llen) < 0 as libc::c_int {
                return 0 as libc::c_int
            }
            /*
	 * make a new transporter (re-uses xprt)
	 */
            xprt = makefd_xprt(sock, (*r).sendsize, (*r).recvsize);
            if xprt.is_null() { close(sock); return 0 as libc::c_int }
            (*xprt).xp_raddr = addr;
            (*xprt).xp_addrlen = len as libc::c_int;
            (*xprt).xp_laddr = laddr;
            (*xprt).xp_laddrlen = llen as libc::c_int;
            return 0 as libc::c_int
        }
    };
    /* there is never an rpc msg to be processed */
}
#[c2rust::src_loc = "311:1"]
unsafe extern "C" fn rendezvous_stat(mut xprt: *mut SVCXPRT) -> xprt_stat {
    return XPRT_IDLE;
}
#[c2rust::src_loc = "318:1"]
unsafe extern "C" fn svctcp_destroy(mut xprt: *mut SVCXPRT) {
    let mut cd: *mut tcp_conn = (*xprt).xp_p1 as *mut tcp_conn;
    gssrpc_xprt_unregister(xprt);
    close((*xprt).xp_sock);
    if (*xprt).xp_port as libc::c_int != 0 as libc::c_int {
        /* a rendezvouser socket */
        (*xprt).xp_port = 0 as libc::c_int as u_short
    } else if (*(*cd).xdrs.x_ops).x_destroy.is_some() {
        Some((*(*cd).xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut (*cd).xdrs);
    }
    if !(*xprt).xp_auth.is_null() {
        Some((*(*(*xprt).xp_auth).svc_ah_ops).svc_ah_destroy.expect("non-null function pointer")).expect("non-null function pointer")((*xprt).xp_auth);
        (*xprt).xp_auth = 0 as *mut SVCAUTH
    }
    free(cd as caddr_t as *mut libc::c_void);
    free(xprt as caddr_t as *mut libc::c_void);
}
/* an actual connection socket */
/*
 * All read operations timeout after 35 seconds.
 * A timeout is fatal for the connection.
 */
#[c2rust::src_loc = "344:23"]
static mut wait_per_try: timeval =
    {
        let mut init =
            timeval{tv_sec: 35 as libc::c_int as __time_t,
                    tv_usec: 0 as libc::c_int as __suseconds_t,};
        init
    };
/*
 * reads data from the tcp conection.
 * any error is fatal and the connection is closed.
 * (And a read of zero bytes is a half closed stream => error.)
 */
#[c2rust::src_loc = "351:1"]
unsafe extern "C" fn readtcp(mut xprtptr: *mut libc::c_char, mut buf: caddr_t,
                             mut len: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut xprt: *mut SVCXPRT = xprtptr as *mut libc::c_void as *mut SVCXPRT;
    let mut sock: libc::c_int = (*xprt).xp_sock;
    let mut tout: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut mask: fd_set = fd_set{fds_bits: [0; 16],};
    let mut readfds: fd_set = fd_set{fds_bits: [0; 16],};
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = &mut __d1;
    let fresh3;
    let fresh4 =
        (::std::mem::size_of::<fd_set>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong);
    let fresh5 =
        &mut *mask.fds_bits.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh1), "={di}" (fresh3) : "{ax}"
         (0 as libc::c_int), "0"
         (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh4)), "1"
         (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh5)) : "memory" :
         "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh4, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh5, fresh3);
    mask.fds_bits[(sock /
                       (8 as libc::c_int *
                            ::std::mem::size_of::<__fd_mask>() as
                                libc::c_ulong as libc::c_int)) as usize] |=
        ((1 as libc::c_ulong) <<
             sock %
                 (8 as libc::c_int *
                      ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as
                          libc::c_int)) as __fd_mask;
    loop 
         /* def FD_SETSIZE */
         {
        readfds = mask;
        tout = wait_per_try;
        if select(sock + 1 as libc::c_int, &mut readfds,
                  0 as *mut libc::c_void as *mut fd_set,
                  0 as *mut libc::c_void as *mut fd_set, &mut tout) <=
               0 as libc::c_int {
            if !(*__errno_location() == 4 as libc::c_int) {
                current_block = 7577583414753802095;
                break ;
            }
        }
        if readfds.fds_bits[(sock /
                                 (8 as libc::c_int *
                                      ::std::mem::size_of::<__fd_mask>() as
                                          libc::c_ulong as libc::c_int)) as
                                usize] &
               ((1 as libc::c_ulong) <<
                    sock %
                        (8 as libc::c_int *
                             ::std::mem::size_of::<__fd_mask>() as
                                 libc::c_ulong as libc::c_int)) as __fd_mask
               != 0 as libc::c_int as libc::c_long {
            current_block = 12599329904712511516;
            break ;
        }
    }
    match current_block {
        12599329904712511516 => {
            len =
                read(sock, buf as *mut libc::c_void, len as size_t) as
                    libc::c_int;
            if len > 0 as libc::c_int { return len }
        }
        _ => { }
    }
    (*((*xprt).xp_p1 as *mut tcp_conn)).strm_stat = XPRT_DIED;
    return -(1 as libc::c_int);
}
/*
 * writes data to the tcp connection.
 * Any error is fatal and the connection is closed.
 */
#[c2rust::src_loc = "398:1"]
unsafe extern "C" fn writetcp(mut xprtptr: *mut libc::c_char,
                              mut buf: caddr_t, mut len: libc::c_int)
 -> libc::c_int {
    let mut xprt: *mut SVCXPRT = xprtptr as *mut libc::c_void as *mut SVCXPRT;
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    cnt = len;
    while cnt > 0 as libc::c_int {
        i =
            write((*xprt).xp_sock, buf as *const libc::c_void, cnt as size_t)
                as libc::c_int;
        if i < 0 as libc::c_int {
            (*((*xprt).xp_p1 as *mut tcp_conn)).strm_stat = XPRT_DIED;
            return -(1 as libc::c_int)
        }
        cnt -= i;
        buf = buf.offset(i as isize)
    }
    return len;
}
#[c2rust::src_loc = "417:1"]
unsafe extern "C" fn svctcp_stat(mut xprt: *mut SVCXPRT) -> xprt_stat {
    let mut cd: *mut tcp_conn = (*xprt).xp_p1 as *mut tcp_conn;
    if (*cd).strm_stat as libc::c_uint ==
           XPRT_DIED as libc::c_int as libc::c_uint {
        return XPRT_DIED
    }
    if gssrpc_xdrrec_eof(&mut (*cd).xdrs) == 0 { return XPRT_MOREREQS }
    return XPRT_IDLE;
}
/* @(#)svc_tcp.c	2.2 88/08/01 4.0 RPCSRC */
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
 * svc_tcp.c, Server side for TCP/IP based RPC.
 *
 * Actually implements two flavors of transporter -
 * a tcp rendezvouser (a listner and connection establisher)
 * and a record/tcp stream.
 */
/*extern bool_t abort();
extern errno;
*/
/*
 * Ops vector for TCP/IP based rpc service handle
 */
#[c2rust::src_loc = "429:1"]
unsafe extern "C" fn svctcp_recv(mut xprt: *mut SVCXPRT,
                                 mut msg: *mut rpc_msg) -> libc::c_int {
    let mut cd: *mut tcp_conn = (*xprt).xp_p1 as *mut tcp_conn;
    let mut xdrs: *mut XDR = &mut (*cd).xdrs;
    (*xdrs).x_op = XDR_DECODE;
    gssrpc_xdrrec_skiprecord(xdrs);
    if gssrpc_xdr_callmsg(xdrs, msg) != 0 {
        (*cd).x_id = (*msg).rm_xid;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "446:1"]
unsafe extern "C" fn svctcp_getargs(mut xprt: *mut SVCXPRT,
                                    mut xdr_args: xdrproc_t,
                                    mut args_ptr: *mut libc::c_void)
 -> libc::c_int {
    if Some((*(*(*xprt).xp_auth).svc_ah_ops).svc_ah_unwrap.expect("non-null function pointer")).expect("non-null function pointer")((*xprt).xp_auth,
                                                                                                                                    &mut (*((*xprt).xp_p1
                                                                                                                                                as
                                                                                                                                                *mut tcp_conn)).xdrs,
                                                                                                                                    xdr_args,
                                                                                                                                    args_ptr
                                                                                                                                        as
                                                                                                                                        caddr_t)
           == 0 {
        svctcp_freeargs(xprt, xdr_args, args_ptr);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "461:1"]
unsafe extern "C" fn svctcp_freeargs(mut xprt: *mut SVCXPRT,
                                     mut xdr_args: xdrproc_t,
                                     mut args_ptr: *mut libc::c_void)
 -> libc::c_int {
    let mut xdrs: *mut XDR = &mut (*((*xprt).xp_p1 as *mut tcp_conn)).xdrs;
    (*xdrs).x_op = XDR_FREE;
    return ::std::mem::transmute::<_,
                                   fn(_: _, _: _)
                                       ->
                                           libc::c_int>(Some(xdr_args.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                args_ptr);
}
#[c2rust::src_loc = "473:1"]
unsafe extern "C" fn svctcp_reply(mut xprt: *mut SVCXPRT,
                                  mut msg: *mut rpc_msg) -> libc::c_int {
    let mut cd: *mut tcp_conn = (*xprt).xp_p1 as *mut tcp_conn;
    let mut xdrs: *mut XDR = &mut (*cd).xdrs;
    let mut stat: libc::c_int = 0;
    let mut xdr_results: xdrproc_t = None;
    let mut xdr_location: caddr_t = 0 as caddr_t;
    let mut has_args: libc::c_int = 0;
    if (*msg).ru.RM_rmb.rp_stat as libc::c_uint ==
           MSG_ACCEPTED as libc::c_int as libc::c_uint &&
           (*msg).ru.RM_rmb.ru.RP_ar.ar_stat as libc::c_uint ==
               SUCCESS as libc::c_int as libc::c_uint {
        has_args = 1 as libc::c_int;
        xdr_results = (*msg).ru.RM_rmb.ru.RP_ar.ru.AR_results.proc_0;
        xdr_location = (*msg).ru.RM_rmb.ru.RP_ar.ru.AR_results.where_0;
        (*msg).ru.RM_rmb.ru.RP_ar.ru.AR_results.proc_0 =
            ::std::mem::transmute::<Option<unsafe extern "C" fn(_: *mut XDR,
                                                                _:
                                                                    *mut libc::c_void)
                                               -> libc::c_int>,
                                    xdrproc_t>(Some(gssrpc_xdr_void as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut XDR,
                                                                             _:
                                                                                 *mut libc::c_void)
                                                            -> libc::c_int));
        (*msg).ru.RM_rmb.ru.RP_ar.ru.AR_results.where_0 = 0 as caddr_t
    } else { has_args = 0 as libc::c_int }
    (*xdrs).x_op = XDR_ENCODE;
    (*msg).rm_xid = (*cd).x_id;
    stat = 0 as libc::c_int;
    if gssrpc_xdr_replymsg(xdrs, msg) != 0 &&
           (has_args == 0 ||
                Some((*(*(*xprt).xp_auth).svc_ah_ops).svc_ah_wrap.expect("non-null function pointer")).expect("non-null function pointer")((*xprt).xp_auth,
                                                                                                                                           xdrs,
                                                                                                                                           xdr_results,
                                                                                                                                           xdr_location)
                    != 0) {
        stat = 1 as libc::c_int
    }
    gssrpc_xdrrec_endofrecord(xdrs, 1 as libc::c_int);
    return stat;
}
#[c2rust::src_loc = "508:1"]
unsafe extern "C" fn abortx() -> libc::c_int { abort(); }
#[c2rust::src_loc = "514:1"]
unsafe extern "C" fn abortx_getargs(mut xprt: *mut SVCXPRT,
                                    mut proc_0: xdrproc_t,
                                    mut info: *mut libc::c_void)
 -> libc::c_int {
    return abortx();
}
#[c2rust::src_loc = "522:1"]
unsafe extern "C" fn abortx_reply(mut xprt: *mut SVCXPRT,
                                  mut msg: *mut rpc_msg) -> libc::c_int {
    return abortx();
}
#[c2rust::src_loc = "527:1"]
unsafe extern "C" fn abortx_freeargs(mut xprt: *mut SVCXPRT,
                                     mut proc_0: xdrproc_t,
                                     mut info: *mut libc::c_void)
 -> libc::c_int {
    return abortx();
}
