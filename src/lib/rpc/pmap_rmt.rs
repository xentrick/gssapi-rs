use ::libc;
use ::c2rust_asm_casts;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:44"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:44"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:44"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:44"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/sys/types.h:44"]
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
#[c2rust::header_src = "/usr/include/unistd.h:44"]
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
#[c2rust::header_src = "/usr/include/sys/time.h:46"]
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
#[c2rust::header_src = "/usr/include/bits/socket_type.h:46"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:46"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:46"]
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
#[c2rust::header_src = "/usr/include/sys/socket.h:46"]
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
    use super::stddef_h::size_t;
    use super::unistd_h::socklen_t;
    use super::sys_types_h::ssize_t;
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
        #[c2rust::src_loc = "152:1"]
        pub fn sendto(__fd: libc::c_int, __buf: *const libc::c_void,
                      __n: size_t, __flags: libc::c_int,
                      __addr: __CONST_SOCKADDR_ARG, __addr_len: socklen_t)
         -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "163:1"]
        pub fn recvfrom(__fd: libc::c_int, __buf: *mut libc::c_void,
                        __n: size_t, __flags: libc::c_int,
                        __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t)
         -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int,
                          __optval: *const libc::c_void, __optlen: socklen_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:46"]
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
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:46"]
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
    #[c2rust::src_loc = "92:1"]
    pub type rpcport_t = uint32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:46"]
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
    use super::stdint_uintn_h::uint32_t;
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
        #[no_mangle]
        #[c2rust::src_loc = "272:1"]
        pub fn gssrpc_xdr_reference(_: *mut XDR, _: *mut caddr_t, _: u_int,
                                    _: xdrproc_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "298:1"]
        pub fn gssrpc_xdr_u_int32(_: *mut XDR, _: *mut uint32_t)
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:46"]
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
        #[no_mangle]
        #[c2rust::src_loc = "191:1"]
        pub fn gssrpc_authunix_create_default() -> *mut AUTH;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:46"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:46"]
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
        pub ru: C2RustUnnamed_8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:2"]
    pub union C2RustUnnamed_8 {
        pub RE_errno: libc::c_int,
        pub RE_why: auth_stat,
        pub RE_vers: C2RustUnnamed_10,
        pub RE_lb: C2RustUnnamed_9,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "99:3"]
    pub struct C2RustUnnamed_9 {
        pub s1: int32_t,
        pub s2: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:3"]
    pub struct C2RustUnnamed_10 {
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
    use super::auth_h::{auth_stat, AUTH};
    use super::stdint_intn_h::int32_t;
    use super::gssrpc_types_h::{rpcvers_t, rpcproc_t, rpcprog_t};
    use super::xdr_h::xdrproc_t;
    use super::struct_timeval_h::timeval;
    use super::in_h::sockaddr_in;
    use super::types_h::{__time_t, __suseconds_t};
    extern "C" {
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
        #[c2rust::src_loc = "301:1"]
        pub fn gssrpc_clntudp_create(_: *mut sockaddr_in, _: rpcprog_t,
                                     _: rpcvers_t, _: timeval,
                                     _: *mut libc::c_int) -> *mut CLIENT;
    }
    /* !defined(GSSRPC_CLNT_H) */
    /* a more reasonable packet size */
    /* rpc imposed limit on udp msg size */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/pmap_rmt.h:49"]
pub mod pmap_rmt_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:8"]
    pub struct rmtcallres {
        pub port_ptr: *mut rpcport_t,
        pub resultslen: uint32_t,
        pub results_ptr: caddr_t,
        pub xdr_results: xdrproc_t,
    }
    /* @(#)pmap_rmt.h	2.1 88/07/29 4.0 RPCSRC; from 1.2 88/02/08 SMI */
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
 * Structures and XDR routines for parameters to and replies from
 * the portmapper remote-call-service.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:8"]
    pub struct rmtcallargs {
        pub prog: rpcprog_t,
        pub vers: rpcvers_t,
        pub proc_0: rpcproc_t,
        pub arglen: uint32_t,
        pub args_ptr: caddr_t,
        pub xdr_args: xdrproc_t,
    }
    use super::gssrpc_types_h::{rpcport_t, rpcprog_t, rpcvers_t, rpcproc_t};
    use super::stdint_uintn_h::uint32_t;
    use super::sys_types_h::caddr_t;
    use super::xdr_h::xdrproc_t;
    /* !defined(GSSRPC_PMAP_RMT_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/pmap_clnt.h:48"]
pub mod pmap_clnt_h {
    #[c2rust::src_loc = "74:1"]
    pub type resultproc_t
        =
        Option<unsafe extern "C" fn(_: caddr_t, _: *mut sockaddr_in)
                   -> libc::c_int>;
    use super::sys_types_h::caddr_t;
    use super::in_h::sockaddr_in;
    /* !defined(GSSRPC_PMAP_CLNT_H) */
}
#[c2rust::header_src = "/usr/include/net/if.h:58"]
pub mod if_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "135:5"]
    pub union C2RustUnnamed_11 {
        pub ifru_addr: sockaddr,
        pub ifru_dstaddr: sockaddr,
        pub ifru_broadaddr: sockaddr,
        pub ifru_netmask: sockaddr,
        pub ifru_hwaddr: sockaddr,
        pub ifru_flags: libc::c_short,
        pub ifru_ivalue: libc::c_int,
        pub ifru_mtu: libc::c_int,
        pub ifru_map: ifmap,
        pub ifru_slave: [libc::c_char; 16],
        pub ifru_newname: [libc::c_char; 16],
        pub ifru_data: __caddr_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:8"]
    pub struct ifmap {
        pub mem_start: libc::c_ulong,
        pub mem_end: libc::c_ulong,
        pub base_addr: libc::c_ushort,
        pub irq: libc::c_uchar,
        pub dma: libc::c_uchar,
        pub port: libc::c_uchar,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:8"]
    pub struct ifreq {
        pub ifr_ifrn: C2RustUnnamed_12,
        pub ifr_ifru: C2RustUnnamed_11,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "130:5"]
    pub union C2RustUnnamed_12 {
        pub ifrn_name: [libc::c_char; 16],
    }
    #[c2rust::src_loc = "44:5"]
    pub const IFF_UP: C2RustUnnamed_14 = 1;
    #[c2rust::src_loc = "46:5"]
    pub const IFF_BROADCAST: C2RustUnnamed_14 = 2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "176:8"]
    pub struct ifconf {
        pub ifc_len: libc::c_int,
        pub ifc_ifcu: C2RustUnnamed_13,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "179:5"]
    pub union C2RustUnnamed_13 {
        pub ifcu_buf: __caddr_t,
        pub ifcu_req: *mut ifreq,
    }
    #[c2rust::src_loc = "42:1"]
    pub type C2RustUnnamed_14 = libc::c_uint;
    #[c2rust::src_loc = "79:5"]
    pub const IFF_DYNAMIC: C2RustUnnamed_14 = 32768;
    #[c2rust::src_loc = "77:5"]
    pub const IFF_AUTOMEDIA: C2RustUnnamed_14 = 16384;
    #[c2rust::src_loc = "75:5"]
    pub const IFF_PORTSEL: C2RustUnnamed_14 = 8192;
    #[c2rust::src_loc = "72:5"]
    pub const IFF_MULTICAST: C2RustUnnamed_14 = 4096;
    #[c2rust::src_loc = "69:5"]
    pub const IFF_SLAVE: C2RustUnnamed_14 = 2048;
    #[c2rust::src_loc = "67:5"]
    pub const IFF_MASTER: C2RustUnnamed_14 = 1024;
    #[c2rust::src_loc = "64:5"]
    pub const IFF_ALLMULTI: C2RustUnnamed_14 = 512;
    #[c2rust::src_loc = "60:5"]
    pub const IFF_PROMISC: C2RustUnnamed_14 = 256;
    #[c2rust::src_loc = "58:5"]
    pub const IFF_NOARP: C2RustUnnamed_14 = 128;
    #[c2rust::src_loc = "56:5"]
    pub const IFF_RUNNING: C2RustUnnamed_14 = 64;
    #[c2rust::src_loc = "54:5"]
    pub const IFF_NOTRAILERS: C2RustUnnamed_14 = 32;
    #[c2rust::src_loc = "52:5"]
    pub const IFF_POINTOPOINT: C2RustUnnamed_14 = 16;
    #[c2rust::src_loc = "50:5"]
    pub const IFF_LOOPBACK: C2RustUnnamed_14 = 8;
    #[c2rust::src_loc = "48:5"]
    pub const IFF_DEBUG: C2RustUnnamed_14 = 4;
    use super::socket_h::sockaddr;
    use super::types_h::__caddr_t;
}
#[c2rust::header_src = "/usr/include/stdio.h:44"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:44"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:44"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:44"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc.h:46"]
pub mod rpc_h {
    extern "C" {
        /* @(#)rpc.h	2.3 88/08/10 4.0 RPCSRC; from 1.9 88/02/08 SMI */
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
 * rpc.h, Just includes the billions of rpc header files necessary to
 * do remote procedure calling.
 */
        /* external data representation interfaces */
        /* Client side only authentication */
        /* Client side (mostly) remote procedure call */
        /* semi-private protocol headers */
        /* Server side only remote procedure callee */
        /*
 * get the local host's IP address without consulting
 * name service library functions
 */
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn gssrpc__rpc_dtablesize() -> libc::c_int;
    }
    /* !defined(GSSRPC_RPC_H) */
}
#[c2rust::header_src = "/usr/include/sys/ioctl.h:59"]
pub mod ioctl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/socket-utils.h:63"]
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
    use super::socket_h::sockaddr;
    use super::in_h::sockaddr_in;
    /* SOCKET_UTILS_H */
}
use c2rust_asm_casts::AsmCastTrait;
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __u_int, __uint8_t, __uint16_t, __int32_t,
                        __uint32_t, __pid_t, __time_t, __suseconds_t,
                        __ssize_t, __caddr_t, __socklen_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::sys_types_h::{u_short, u_int, ssize_t, caddr_t};
pub use self::struct_timeval_h::timeval;
pub use self::select_h::{__fd_mask, fd_set, select};
pub use self::unistd_h::{socklen_t, close, getpid};
pub use self::time_h::{timezone, __timezone_ptr_t, gettimeofday};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::sockaddr;
pub use self::sys_socket_h::{__SOCKADDR_ARG, __CONST_SOCKADDR_ARG,
                             sockaddr_x25, sockaddr_un, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, sendto, recvfrom,
                             setsockopt};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, C2RustUnnamed_0,
                     IPPROTO_MAX, IPPROTO_RAW, IPPROTO_MPLS, IPPROTO_UDPLITE,
                     IPPROTO_SCTP, IPPROTO_COMP, IPPROTO_PIM, IPPROTO_ENCAP,
                     IPPROTO_BEETPH, IPPROTO_MTP, IPPROTO_AH, IPPROTO_ESP,
                     IPPROTO_GRE, IPPROTO_RSVP, IPPROTO_IPV6, IPPROTO_DCCP,
                     IPPROTO_TP, IPPROTO_IDP, IPPROTO_UDP, IPPROTO_PUP,
                     IPPROTO_EGP, IPPROTO_TCP, IPPROTO_IPIP, IPPROTO_IGMP,
                     IPPROTO_ICMP, IPPROTO_IP, htonl, htons};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpcport_t,
                               rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdr_reference,
                      gssrpc_xdr_u_int32, gssrpc_xdrmem_create};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, des_block,
                       opaque_auth, AUTH, auth_ops, gssrpc__null_auth,
                       gssrpc_authunix_create_default};
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
pub use self::clnt_h::{clnt_stat, RPC_FAILED, RPC_PROGNOTREGISTERED,
                       RPC_PMAPFAILURE, RPC_UNKNOWNPROTO, RPC_UNKNOWNHOST,
                       RPC_SYSTEMERROR, RPC_CANTDECODEARGS, RPC_PROCUNAVAIL,
                       RPC_PROGVERSMISMATCH, RPC_PROGUNAVAIL, RPC_AUTHERROR,
                       RPC_VERSMISMATCH, RPC_TIMEDOUT, RPC_CANTRECV,
                       RPC_CANTSEND, RPC_CANTDECODERES, RPC_CANTENCODEARGS,
                       RPC_SUCCESS, rpc_err, C2RustUnnamed_8, C2RustUnnamed_9,
                       C2RustUnnamed_10, CLIENT, clnt_ops,
                       gssrpc_clntudp_create};
pub use self::pmap_rmt_h::{rmtcallres, rmtcallargs};
pub use self::pmap_clnt_h::resultproc_t;
pub use self::if_h::{C2RustUnnamed_11, ifmap, ifreq, C2RustUnnamed_12, IFF_UP,
                     IFF_BROADCAST, ifconf, C2RustUnnamed_13,
                     C2RustUnnamed_14, IFF_DYNAMIC, IFF_AUTOMEDIA,
                     IFF_PORTSEL, IFF_MULTICAST, IFF_SLAVE, IFF_MASTER,
                     IFF_ALLMULTI, IFF_PROMISC, IFF_NOARP, IFF_RUNNING,
                     IFF_NOTRAILERS, IFF_POINTOPOINT, IFF_LOOPBACK,
                     IFF_DEBUG};
use self::stdio_h::perror;
use self::fcntl_h::fcntl;
use self::errno_h::__errno_location;
use self::string_h::memset;
use self::rpc_h::gssrpc__rpc_dtablesize;
use self::ioctl_h::ioctl;
pub use self::socket_utils_h::sa2sin;
#[c2rust::src_loc = "65:23"]
static mut timeout: timeval =
    {
        let mut init =
            timeval{tv_sec: 3 as libc::c_int as __time_t,
                    tv_usec: 0 as libc::c_int as __suseconds_t,};
        init
    };
/*
 * pmapper remote-call-service interface.
 * This routine is used to call the pmapper remote call service
 * which will look up a service program in the port maps, and then
 * remotely call that routine with the given parameters.  This allows
 * programs to do a lookup and call in one step.
*/
#[no_mangle]
#[c2rust::src_loc = "78:1"]
pub unsafe extern "C" fn gssrpc_pmap_rmtcall(mut addr: *mut sockaddr_in,
                                             mut prog: rpcprog_t,
                                             mut vers: rpcvers_t,
                                             mut proc_0: rpcproc_t,
                                             mut xdrargs: xdrproc_t,
                                             mut argsp: caddr_t,
                                             mut xdrres: xdrproc_t,
                                             mut resp: caddr_t,
                                             mut tout: timeval,
                                             mut port_ptr: *mut rpcport_t)
 -> clnt_stat {
    let mut sock: libc::c_int = !(0 as libc::c_int);
    let mut client: *mut CLIENT = 0 as *mut CLIENT;
    let mut a: rmtcallargs =
        rmtcallargs{prog: 0,
                    vers: 0,
                    proc_0: 0,
                    arglen: 0,
                    args_ptr: 0 as *mut libc::c_char,
                    xdr_args: None,};
    let mut r: rmtcallres =
        rmtcallres{port_ptr: 0 as *mut rpcport_t,
                   resultslen: 0,
                   results_ptr: 0 as *mut libc::c_char,
                   xdr_results: None,};
    let mut stat: clnt_stat = RPC_SUCCESS;
    (*addr).sin_port = htons(111 as libc::c_int as u_short);
    client =
        gssrpc_clntudp_create(addr, 100000 as libc::c_int as rpcprog_t,
                              2 as libc::c_int as rpcvers_t, timeout,
                              &mut sock);
    if !client.is_null() {
        a.prog = prog;
        a.vers = vers;
        a.proc_0 = proc_0;
        a.args_ptr = argsp;
        a.xdr_args = xdrargs;
        r.port_ptr = port_ptr;
        r.results_ptr = resp;
        r.xdr_results = xdrres;
        stat =
            Some((*(*client).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(client,
                                                                                                                      5
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          rpcproc_t,
                                                                                                                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                              *mut XDR,
                                                                                                                                                                          _:
                                                                                                                                                                              *mut rmtcallargs)
                                                                                                                                                         ->
                                                                                                                                                             libc::c_int>,
                                                                                                                                              xdrproc_t>(Some(gssrpc_xdr_rmtcall_args
                                                                                                                                                                  as
                                                                                                                                                                  unsafe extern "C" fn(_:
                                                                                                                                                                                           *mut XDR,
                                                                                                                                                                                       _:
                                                                                                                                                                                           *mut rmtcallargs)
                                                                                                                                                                      ->
                                                                                                                                                                          libc::c_int)),
                                                                                                                      &mut a
                                                                                                                          as
                                                                                                                          *mut rmtcallargs
                                                                                                                          as
                                                                                                                          *mut libc::c_void,
                                                                                                                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                              *mut XDR,
                                                                                                                                                                          _:
                                                                                                                                                                              *mut rmtcallres)
                                                                                                                                                         ->
                                                                                                                                                             libc::c_int>,
                                                                                                                                              xdrproc_t>(Some(gssrpc_xdr_rmtcallres
                                                                                                                                                                  as
                                                                                                                                                                  unsafe extern "C" fn(_:
                                                                                                                                                                                           *mut XDR,
                                                                                                                                                                                       _:
                                                                                                                                                                                           *mut rmtcallres)
                                                                                                                                                                      ->
                                                                                                                                                                          libc::c_int)),
                                                                                                                      &mut r
                                                                                                                          as
                                                                                                                          *mut rmtcallres
                                                                                                                          as
                                                                                                                          *mut libc::c_void,
                                                                                                                      tout);
        Some((*(*client).cl_ops).cl_destroy.expect("non-null function pointer")).expect("non-null function pointer")(client);
    } else { stat = RPC_FAILED }
    close(sock);
    (*addr).sin_port = 0 as libc::c_int as in_port_t;
    return stat;
}
/*
 * XDR remote call arguments
 * written for XDR_ENCODE direction only
 */
#[no_mangle]
#[c2rust::src_loc = "124:1"]
pub unsafe extern "C" fn gssrpc_xdr_rmtcall_args(mut xdrs: *mut XDR,
                                                 mut cap: *mut rmtcallargs)
 -> libc::c_int {
    let mut lenposition: u_int = 0;
    let mut argposition: u_int = 0;
    let mut position: u_int = 0;
    if gssrpc_xdr_u_int32(xdrs, &mut (*cap).prog) != 0 &&
           gssrpc_xdr_u_int32(xdrs, &mut (*cap).vers) != 0 &&
           gssrpc_xdr_u_int32(xdrs, &mut (*cap).proc_0) != 0 {
        lenposition =
            Some((*(*xdrs).x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs);
        if gssrpc_xdr_u_int32(xdrs, &mut (*cap).arglen) == 0 {
            return 0 as libc::c_int
        }
        argposition =
            Some((*(*xdrs).x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs);
        if ::std::mem::transmute::<_,
                                   fn(_: _, _: _)
                                       ->
                                           libc::c_int>(Some((*cap).xdr_args.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                       (*cap).args_ptr)
               == 0 {
            return 0 as libc::c_int
        }
        position =
            Some((*(*xdrs).x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs);
        (*cap).arglen = position.wrapping_sub(argposition);
        Some((*(*xdrs).x_ops).x_setpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                  lenposition);
        if gssrpc_xdr_u_int32(xdrs, &mut (*cap).arglen) == 0 {
            return 0 as libc::c_int
        }
        Some((*(*xdrs).x_ops).x_setpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                  position);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
 * XDR remote call results
 * written for XDR_DECODE direction only
 */
#[no_mangle]
#[c2rust::src_loc = "155:1"]
pub unsafe extern "C" fn gssrpc_xdr_rmtcallres(mut xdrs: *mut XDR,
                                               mut crp: *mut rmtcallres)
 -> libc::c_int {
    let mut port_ptr: caddr_t = 0 as *mut libc::c_char;
    port_ptr = (*crp).port_ptr as *mut libc::c_void as caddr_t;
    if gssrpc_xdr_reference(xdrs, &mut port_ptr,
                            ::std::mem::size_of::<uint32_t>() as libc::c_ulong
                                as u_int,
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    *mut XDR,
                                                                                _:
                                                                                    *mut uint32_t)
                                                               ->
                                                                   libc::c_int>,
                                                    xdrproc_t>(Some(gssrpc_xdr_u_int32
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut XDR,
                                                                                             _:
                                                                                                 *mut uint32_t)
                                                                            ->
                                                                                libc::c_int)))
           != 0 && gssrpc_xdr_u_int32(xdrs, &mut (*crp).resultslen) != 0 {
        (*crp).port_ptr = port_ptr as *mut libc::c_void as *mut uint32_t;
        return ::std::mem::transmute::<_,
                                       fn(_: _, _: _)
                                           ->
                                               libc::c_int>(Some((*crp).xdr_results.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                              (*crp).results_ptr)
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "180:1"]
unsafe extern "C" fn getbroadcastnets(mut addrs: *mut in_addr,
                                      mut sock: libc::c_int,
                                      mut buf: *mut libc::c_char)
 -> libc::c_int 
 /* why allocxate more when we can use existing... */
 {
    let mut ifc: ifconf =
        ifconf{ifc_len: 0,
               ifc_ifcu:
                   C2RustUnnamed_13{ifcu_buf: 0 as *mut libc::c_char,},};
    let mut ifreq: ifreq =
        ifreq{ifr_ifrn: C2RustUnnamed_12{ifrn_name: [0; 16],},
              ifr_ifru:
                  C2RustUnnamed_11{ifru_addr:
                                       sockaddr{sa_family: 0,
                                                sa_data: [0; 14],},},};
    let mut ifr: *mut ifreq = 0 as *mut ifreq;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    ifc.ifc_len =
        (256 as libc::c_int as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<ifconf>() as
                                             libc::c_ulong) as libc::c_int;
    ifc.ifc_ifcu.ifcu_buf = buf;
    memset(buf as *mut libc::c_void, 0 as libc::c_int,
           (256 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<ifconf>() as
                                                libc::c_ulong));
    if ioctl(sock, 0x8912 as libc::c_int as libc::c_ulong,
             &mut ifc as *mut ifconf as *mut libc::c_char) < 0 as libc::c_int
       {
        perror(b"broadcast: ioctl (get interface configuration)\x00" as
                   *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    ifr = ifc.ifc_ifcu.ifcu_req;
    i = 0 as libc::c_int;
    n =
        (ifc.ifc_len as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<ifreq>() as
                                             libc::c_ulong) as libc::c_int;
    while n > 0 as libc::c_int {
        ifreq = *ifr;
        if ioctl(sock, 0x8913 as libc::c_int as libc::c_ulong,
                 &mut ifreq as *mut ifreq as *mut libc::c_char) <
               0 as libc::c_int {
            perror(b"broadcast: ioctl (get interface flags)\x00" as *const u8
                       as *const libc::c_char);
        } else if ifreq.ifr_ifru.ifru_flags as libc::c_int &
                      IFF_BROADCAST as libc::c_int != 0 &&
                      ifreq.ifr_ifru.ifru_flags as libc::c_int &
                          IFF_UP as libc::c_int != 0 &&
                      (*ifr).ifr_ifru.ifru_addr.sa_family as libc::c_int ==
                          2 as libc::c_int {
            /* 4.3BSD */
            if ioctl(sock, 0x8919 as libc::c_int as libc::c_ulong,
                     &mut ifreq as *mut ifreq as *mut libc::c_char) <
                   0 as libc::c_int {
                let fresh0 = i;
                i = i + 1;
                (*addrs.offset(fresh0 as isize)).s_addr =
                    0 as libc::c_int as in_addr_t
            } else {
                let fresh1 = i;
                i = i + 1;
                *addrs.offset(fresh1 as isize) =
                    (*sa2sin(&mut ifreq.ifr_ifru.ifru_addr)).sin_addr
            }
            /* 4.2 BSD */
        }
        n -= 1;
        ifr = ifr.offset(1)
    }
    return i;
}
#[no_mangle]
#[c2rust::src_loc = "225:1"]
pub unsafe extern "C" fn gssrpc_clnt_broadcast(mut prog: rpcprog_t,
                                               mut vers: rpcvers_t,
                                               mut proc_0: rpcproc_t,
                                               mut xargs: xdrproc_t,
                                               mut argsp: caddr_t,
                                               mut xresults: xdrproc_t,
                                               mut resultsp: caddr_t,
                                               mut eachresult: resultproc_t)
 -> clnt_stat 
 /* call with each result obtained */
 {
    let mut stat: clnt_stat = RPC_SUCCESS;
    let mut unix_auth: *mut AUTH = gssrpc_authunix_create_default();
    let mut xdr_stream: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut xdrs: *mut XDR = &mut xdr_stream;
    let mut outlen: libc::c_int = 0;
    let mut nets: libc::c_int = 0;
    let mut inlen: ssize_t = 0;
    let mut fromlen: socklen_t = 0;
    let mut sock: libc::c_int = 0;
    let mut on: libc::c_int = 1 as libc::c_int;
    let mut mask: fd_set = fd_set{fds_bits: [0; 16],};
    let mut readfds: fd_set = fd_set{fds_bits: [0; 16],};
    /* def FD_SETSIZE */
    let mut i: libc::c_int = 0; /* broadcast and response addresses */
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut xid: uint32_t = 0;
    let mut port: rpcport_t = 0;
    let mut addrs: [in_addr; 20] = [in_addr{s_addr: 0,}; 20];
    let mut baddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut raddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut a: rmtcallargs =
        rmtcallargs{prog: 0,
                    vers: 0,
                    proc_0: 0,
                    arglen: 0,
                    args_ptr: 0 as *mut libc::c_char,
                    xdr_args: None,};
    let mut r: rmtcallres =
        rmtcallres{port_ptr: 0 as *mut rpcport_t,
                   resultslen: 0,
                   results_ptr: 0 as *mut libc::c_char,
                   xdr_results: None,};
    let mut msg: rpc_msg =
        rpc_msg{rm_xid: 0,
                rm_direction: CALL,
                ru:
                    C2RustUnnamed_1{RM_cmb:
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
    let mut t: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut t2: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut outbuf: [libc::c_char; 1400] = [0; 1400];
    let mut inbuf: [libc::c_char; 8800] = [0; 8800];
    /*
	 * initialization: create a socket, a broadcast address, and
	 * preserialize the arguments into a send buffer.
	 */
    sock =
        socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int,
               IPPROTO_UDP as libc::c_int);
    if sock < 0 as libc::c_int {
        perror(b"Cannot create socket for broadcast rpc\x00" as *const u8 as
                   *const libc::c_char);
        stat = RPC_CANTSEND
    } else {
        fcntl(sock, 2 as libc::c_int, 1 as libc::c_int);
        if setsockopt(sock, 1 as libc::c_int, 6 as libc::c_int,
                      &mut on as *mut libc::c_int as *mut libc::c_char as
                          *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) < 0 as libc::c_int {
            perror(b"Cannot set socket option SO_BROADCAST\x00" as *const u8
                       as *const libc::c_char);
            stat = RPC_CANTSEND
        } else {
            /* def SO_BROADCAST */
            let mut __d0: libc::c_int = 0;
            let mut __d1: libc::c_int = 0;
            let fresh2 = &mut __d0;
            let fresh3;
            let fresh4 = &mut __d1;
            let fresh5;
            let fresh6 =
                (::std::mem::size_of::<fd_set>() as
                     libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>()
                                                     as libc::c_ulong);
            let fresh7 =
                &mut *mask.fds_bits.as_mut_ptr().offset(0 as libc::c_int as
                                                            isize) as
                    *mut __fd_mask;
            asm!("cld; rep; stosq" : "={cx}" (fresh3), "={di}" (fresh5) :
                 "{ax}" (0 as libc::c_int), "0"
                 (c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh6)), "1"
                 (c2rust_asm_casts::AsmCast::cast_in(fresh4, fresh7)) :
                 "memory" : "volatile");
            c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh6, fresh3);
            c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh7, fresh5);
            mask.fds_bits[(sock /
                               (8 as libc::c_int *
                                    ::std::mem::size_of::<__fd_mask>() as
                                        libc::c_ulong as libc::c_int)) as
                              usize] |=
                ((1 as libc::c_ulong) <<
                     sock %
                         (8 as libc::c_int *
                              ::std::mem::size_of::<__fd_mask>() as
                                  libc::c_ulong as libc::c_int)) as __fd_mask;
            /* def FD_SETSIZE */
            nets =
                getbroadcastnets(addrs.as_mut_ptr(), sock,
                                 inbuf.as_mut_ptr());
            memset(&mut baddr as *mut sockaddr_in as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
            baddr.sin_family = 2 as libc::c_int as sa_family_t;
            baddr.sin_port = htons(111 as libc::c_int as u_short);
            baddr.sin_addr.s_addr = htonl(0 as libc::c_int as in_addr_t);
            /*	baddr.sin_addr.S_un.S_addr = htonl(INADDR_ANY); */
            gettimeofday(&mut t, 0 as *mut timezone);
            xid =
                (getpid() as libc::c_long ^ t.tv_sec ^ t.tv_usec) as uint32_t;
            msg.rm_xid = xid;
            t.tv_usec = 0 as libc::c_int as __suseconds_t;
            msg.rm_direction = CALL;
            msg.ru.RM_cmb.cb_rpcvers = 2 as libc::c_int as uint32_t;
            msg.ru.RM_cmb.cb_prog = 100000 as libc::c_int as rpcprog_t;
            msg.ru.RM_cmb.cb_vers = 2 as libc::c_int as rpcvers_t;
            msg.ru.RM_cmb.cb_proc = 5 as libc::c_int as rpcproc_t;
            msg.ru.RM_cmb.cb_cred = (*unix_auth).ah_cred;
            msg.ru.RM_cmb.cb_verf = (*unix_auth).ah_verf;
            a.prog = prog;
            a.vers = vers;
            a.proc_0 = proc_0;
            a.xdr_args = xargs;
            a.args_ptr = argsp;
            r.port_ptr = &mut port;
            r.xdr_results = xresults;
            r.results_ptr = resultsp;
            gssrpc_xdrmem_create(xdrs, outbuf.as_mut_ptr(),
                                 1400 as libc::c_int as u_int, XDR_ENCODE);
            if gssrpc_xdr_callmsg(xdrs, &mut msg) == 0 ||
                   gssrpc_xdr_rmtcall_args(xdrs, &mut a) == 0 {
                stat = RPC_CANTENCODEARGS
            } else {
                outlen =
                    Some((*(*xdrs).x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs)
                        as libc::c_int;
                if (*(*xdrs).x_ops).x_destroy.is_some() {
                    Some((*(*xdrs).x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(xdrs);
                }
                /*
	 * Basic loop: broadcast a packet and wait a while for response(s).
	 * The response timeout grows larger per iteration.
	 */
                t.tv_sec =
                    4 as libc::c_int as
                        __time_t; /* end of select results switch */
                's_254:
                    while t.tv_sec <= 14 as libc::c_int as libc::c_long {
                        i = 0 as libc::c_int;
                        while i < nets {
                            baddr.sin_addr = addrs[i as usize];
                            if sendto(sock,
                                      outbuf.as_mut_ptr() as
                                          *const libc::c_void,
                                      outlen as size_t, 0 as libc::c_int,
                                      __CONST_SOCKADDR_ARG{__sockaddr__:
                                                               &mut baddr as
                                                                   *mut sockaddr_in
                                                                   as
                                                                   *mut sockaddr,},
                                      ::std::mem::size_of::<sockaddr>() as
                                          libc::c_ulong as socklen_t) !=
                                   outlen as libc::c_long {
                                perror(b"Cannot send broadcast packet\x00" as
                                           *const u8 as *const libc::c_char);
                                stat = RPC_CANTSEND;
                                break 's_254 ;
                            } else { i += 1 }
                        }
                        if eachresult.is_none() {
                            stat = RPC_SUCCESS;
                            break ;
                        } else {
                            loop  {
                                msg.ru.RM_rmb.ru.RP_ar.ar_verf =
                                    gssrpc__null_auth;
                                msg.ru.RM_rmb.ru.RP_ar.ru.AR_results.where_0 =
                                    &mut r as *mut rmtcallres as caddr_t;
                                msg.ru.RM_rmb.ru.RP_ar.ru.AR_results.proc_0 =
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            *mut XDR,
                                                                                        _:
                                                                                            *mut rmtcallres)
                                                                       ->
                                                                           libc::c_int>,
                                                            xdrproc_t>(Some(gssrpc_xdr_rmtcallres
                                                                                as
                                                                                unsafe extern "C" fn(_:
                                                                                                         *mut XDR,
                                                                                                     _:
                                                                                                         *mut rmtcallres)
                                                                                    ->
                                                                                        libc::c_int));
                                readfds = mask;
                                t2 = t;
                                match select(gssrpc__rpc_dtablesize(),
                                             &mut readfds,
                                             0 as *mut libc::c_void as
                                                 *mut fd_set,
                                             0 as *mut libc::c_void as
                                                 *mut fd_set, &mut t2) {
                                    0 => {
                                        /* timed out */
                                        stat = RPC_TIMEDOUT;
                                        t.tv_sec +=
                                            2 as libc::c_int as libc::c_long;
                                        break ;
                                    }
                                    -1 => {
                                        /* some kind of error */
                                        if *__errno_location() ==
                                               4 as libc::c_int {
                                            continue ;
                                        }
                                        perror(b"Broadcast select problem\x00"
                                                   as *const u8 as
                                                   *const libc::c_char);
                                        stat = RPC_CANTRECV;
                                        break 's_254 ;
                                    }
                                    _ => {
                                        loop  {
                                            fromlen =
                                                ::std::mem::size_of::<sockaddr>()
                                                    as libc::c_ulong as
                                                    socklen_t;
                                            inlen =
                                                recvfrom(sock,
                                                         inbuf.as_mut_ptr() as
                                                             *mut libc::c_void,
                                                         8800 as libc::c_int
                                                             as size_t,
                                                         0 as libc::c_int,
                                                         __SOCKADDR_ARG{__sockaddr__:
                                                                            &mut raddr
                                                                                as
                                                                                *mut sockaddr_in
                                                                                as
                                                                                *mut sockaddr,},
                                                         &mut fromlen);
                                            if inlen <
                                                   0 as libc::c_int as
                                                       libc::c_long {
                                                if *__errno_location() ==
                                                       4 as libc::c_int {
                                                    continue ;
                                                }
                                                perror(b"Cannot receive reply to broadcast\x00"
                                                           as *const u8 as
                                                           *const libc::c_char);
                                                stat = RPC_CANTRECV;
                                                break 's_254 ;
                                            } else {
                                                if (inlen as size_t) <
                                                       ::std::mem::size_of::<uint32_t>()
                                                           as libc::c_ulong {
                                                    break ;
                                                }
                                                /*
		 * see if reply transaction id matches sent id.
		 * If so, decode the results.
		 */
                                                gssrpc_xdrmem_create(xdrs,
                                                                     inbuf.as_mut_ptr(),
                                                                     inlen as
                                                                         u_int,
                                                                     XDR_DECODE);
                                                if gssrpc_xdr_replymsg(xdrs,
                                                                       &mut msg)
                                                       != 0 {
                                                    if msg.rm_xid == xid &&
                                                           msg.ru.RM_rmb.rp_stat
                                                               as libc::c_uint
                                                               ==
                                                               MSG_ACCEPTED as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint
                                                           &&
                                                           msg.ru.RM_rmb.ru.RP_ar.ar_stat
                                                               as libc::c_uint
                                                               ==
                                                               SUCCESS as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint
                                                       {
                                                        raddr.sin_port =
                                                            htons(port as
                                                                      u_short);
                                                        done =
                                                            Some(eachresult.expect("non-null function pointer")).expect("non-null function pointer")(resultsp,
                                                                                                                                                     &mut raddr)
                                                    }
                                                    /* otherwise, we just ignore the errors ... */
                                                }
                                                (*xdrs).x_op = XDR_FREE;
                                                msg.ru.RM_rmb.ru.RP_ar.ru.AR_results.proc_0
                                                    =
                                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                            *mut XDR,
                                                                                                        _:
                                                                                                            *mut libc::c_void)
                                                                                       ->
                                                                                           libc::c_int>,
                                                                            xdrproc_t>(Some(gssrpc_xdr_void
                                                                                                as
                                                                                                unsafe extern "C" fn(_:
                                                                                                                         *mut XDR,
                                                                                                                     _:
                                                                                                                         *mut libc::c_void)
                                                                                                    ->
                                                                                                        libc::c_int));
                                                gssrpc_xdr_replymsg(xdrs,
                                                                    &mut msg);
                                                ::std::mem::transmute::<_,
                                                                        fn(_:
                                                                               _,
                                                                           _:
                                                                               _)
                                                                            ->
                                                                                libc::c_int>(Some(xresults.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                                                     resultsp);
                                                if (*(*xdrs).x_ops).x_destroy.is_some()
                                                   {
                                                    Some((*(*xdrs).x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(xdrs);
                                                }
                                                if !(done != 0) { break ; }
                                                stat = RPC_SUCCESS;
                                                break 's_254 ;
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
    close(sock);
    Some((*(*unix_auth).ah_ops).ah_destroy.expect("non-null function pointer")).expect("non-null function pointer")(unix_auth);
    return stat;
}
