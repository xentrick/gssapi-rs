use ::libc;
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
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:44"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:44"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_iovec.h:44"]
pub mod struct_iovec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:8"]
    pub struct iovec {
        pub iov_base: *mut libc::c_void,
        pub iov_len: size_t,
    }
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/usr/include/unistd.h:44"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:8"]
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        pub __ss_padding: [libc::c_char; 118],
        pub __ss_align: libc::c_ulong,
    }
    #[c2rust::src_loc = "200:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "248:5"]
    pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
    #[c2rust::src_loc = "245:5"]
    pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
    #[c2rust::src_loc = "243:5"]
    pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
    #[c2rust::src_loc = "241:5"]
    pub const MSG_BATCH: C2RustUnnamed = 262144;
    #[c2rust::src_loc = "239:5"]
    pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
    #[c2rust::src_loc = "237:5"]
    pub const MSG_MORE: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "235:5"]
    pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "233:5"]
    pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "231:5"]
    pub const MSG_RST: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "229:5"]
    pub const MSG_CONFIRM: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "227:5"]
    pub const MSG_SYN: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "225:5"]
    pub const MSG_FIN: C2RustUnnamed = 512;
    #[c2rust::src_loc = "223:5"]
    pub const MSG_WAITALL: C2RustUnnamed = 256;
    #[c2rust::src_loc = "221:5"]
    pub const MSG_EOR: C2RustUnnamed = 128;
    #[c2rust::src_loc = "219:5"]
    pub const MSG_DONTWAIT: C2RustUnnamed = 64;
    #[c2rust::src_loc = "217:5"]
    pub const MSG_TRUNC: C2RustUnnamed = 32;
    #[c2rust::src_loc = "215:5"]
    pub const MSG_PROXY: C2RustUnnamed = 16;
    #[c2rust::src_loc = "213:5"]
    pub const MSG_CTRUNC: C2RustUnnamed = 8;
    #[c2rust::src_loc = "210:5"]
    pub const MSG_TRYHARD: C2RustUnnamed = 4;
    #[c2rust::src_loc = "206:5"]
    pub const MSG_DONTROUTE: C2RustUnnamed = 4;
    #[c2rust::src_loc = "204:5"]
    pub const MSG_PEEK: C2RustUnnamed = 2;
    #[c2rust::src_loc = "202:5"]
    pub const MSG_OOB: C2RustUnnamed = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "257:8"]
    pub struct msghdr {
        pub msg_name: *mut libc::c_void,
        pub msg_namelen: socklen_t,
        pub msg_iov: *mut iovec,
        pub msg_iovlen: size_t,
        pub msg_control: *mut libc::c_void,
        pub msg_controllen: size_t,
        pub msg_flags: libc::c_int,
    }
    use super::sockaddr_h::sa_family_t;
    use super::unistd_h::socklen_t;
    use super::struct_iovec_h::iovec;
    use super::stddef_h::size_t;
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
    use super::socket_h::{sockaddr, msghdr};
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::unistd_h::socklen_t;
    use super::stddef_h::size_t;
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
        #[c2rust::src_loc = "112:1"]
        pub fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                    __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                           __len: *mut socklen_t) -> libc::c_int;
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
        #[c2rust::src_loc = "191:1"]
        pub fn recvmsg(__fd: libc::c_int, __message: *mut msghdr,
                       __flags: libc::c_int) -> ssize_t;
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
        pub __in6_u: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed_0 {
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
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "92:5"]
    pub const IPPROTO_MAX: C2RustUnnamed_1 = 256;
    #[c2rust::src_loc = "90:5"]
    pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
    #[c2rust::src_loc = "88:5"]
    pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
    #[c2rust::src_loc = "86:5"]
    pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
    #[c2rust::src_loc = "84:5"]
    pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
    #[c2rust::src_loc = "82:5"]
    pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
    #[c2rust::src_loc = "80:5"]
    pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
    #[c2rust::src_loc = "78:5"]
    pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
    #[c2rust::src_loc = "76:5"]
    pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
    #[c2rust::src_loc = "74:5"]
    pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
    #[c2rust::src_loc = "72:5"]
    pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
    #[c2rust::src_loc = "70:5"]
    pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
    #[c2rust::src_loc = "68:5"]
    pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
    #[c2rust::src_loc = "66:5"]
    pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
    #[c2rust::src_loc = "64:5"]
    pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
    #[c2rust::src_loc = "62:5"]
    pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
    #[c2rust::src_loc = "60:5"]
    pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
    #[c2rust::src_loc = "58:5"]
    pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
    #[c2rust::src_loc = "56:5"]
    pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
    #[c2rust::src_loc = "54:5"]
    pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
    #[c2rust::src_loc = "52:5"]
    pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
    #[c2rust::src_loc = "50:5"]
    pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
    #[c2rust::src_loc = "48:5"]
    pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "46:5"]
    pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "44:5"]
    pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
    #[c2rust::src_loc = "42:5"]
    pub const IPPROTO_IP: C2RustUnnamed_1 = 0;
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
        pub ru: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed_2 {
        pub RM_cmb: call_body,
        pub RM_rmb: reply_body,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:8"]
    pub struct reply_body {
        pub rp_stat: reply_stat,
        pub ru: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_3 {
        pub RP_ar: accepted_reply,
        pub RP_dr: rejected_reply,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rejected_reply {
        pub rj_stat: reject_stat,
        pub ru: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:2"]
    pub union C2RustUnnamed_4 {
        pub RJ_versions: C2RustUnnamed_5,
        pub RJ_why: auth_stat,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:3"]
    pub struct C2RustUnnamed_5 {
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
        pub ru: C2RustUnnamed_6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:2"]
    pub union C2RustUnnamed_6 {
        pub AR_versions: C2RustUnnamed_8,
        pub AR_results: C2RustUnnamed_7,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:3"]
    pub struct C2RustUnnamed_7 {
        pub where_0: caddr_t,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:3"]
    pub struct C2RustUnnamed_8 {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc.h:46"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc_auth.h:46"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:44"]
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
#[c2rust::header_src = "/usr/include/stdio.h:44"]
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
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc.h:46"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/socket-utils.h:52"]
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
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __u_int, __uint8_t, __uint16_t, __int32_t,
                        __uint32_t, __off_t, __off64_t, __ssize_t, __caddr_t,
                        __socklen_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::sys_types_h::{u_short, u_int, ssize_t, caddr_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::struct_iovec_h::iovec;
pub use self::unistd_h::{socklen_t, close};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage, C2RustUnnamed,
                         MSG_CMSG_CLOEXEC, MSG_FASTOPEN, MSG_ZEROCOPY,
                         MSG_BATCH, MSG_WAITFORONE, MSG_MORE, MSG_NOSIGNAL,
                         MSG_ERRQUEUE, MSG_RST, MSG_CONFIRM, MSG_SYN, MSG_FIN,
                         MSG_WAITALL, MSG_EOR, MSG_DONTWAIT, MSG_TRUNC,
                         MSG_PROXY, MSG_CTRUNC, MSG_TRYHARD, MSG_DONTROUTE,
                         MSG_PEEK, MSG_OOB, msghdr};
pub use self::sys_socket_h::{__SOCKADDR_ARG, __CONST_SOCKADDR_ARG,
                             sockaddr_x25, sockaddr_un, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, bind, getsockname, sendto,
                             recvfrom, recvmsg};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed_0, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, C2RustUnnamed_1,
                     IPPROTO_MAX, IPPROTO_RAW, IPPROTO_MPLS, IPPROTO_UDPLITE,
                     IPPROTO_SCTP, IPPROTO_COMP, IPPROTO_PIM, IPPROTO_ENCAP,
                     IPPROTO_BEETPH, IPPROTO_MTP, IPPROTO_AH, IPPROTO_ESP,
                     IPPROTO_GRE, IPPROTO_RSVP, IPPROTO_IPV6, IPPROTO_DCCP,
                     IPPROTO_TP, IPPROTO_IDP, IPPROTO_UDP, IPPROTO_PUP,
                     IPPROTO_EGP, IPPROTO_TCP, IPPROTO_IPIP, IPPROTO_IGMP,
                     IPPROTO_ICMP, IPPROTO_IP, ntohs, htons};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdrmem_create};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, opaque_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed_2, reply_body,
                          C2RustUnnamed_3, rejected_reply, C2RustUnnamed_4,
                          C2RustUnnamed_5, reject_stat, AUTH_ERROR,
                          RPC_MISMATCH, accepted_reply, C2RustUnnamed_6,
                          C2RustUnnamed_7, C2RustUnnamed_8, accept_stat,
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
use self::string_h::{memcmp, memset};
use self::rpc_h::gssrpc_bindresvport_sa;
pub use self::socket_utils_h::{sa2sin, sa2sin6, sa_setport, sa_getport,
                               sa_socklen};
/*
 * kept in xprt->xp_p2
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "87:8"]
pub struct svcudp_data {
    pub su_iosz: u_int,
    pub su_xid: uint32_t,
    pub su_xdrs: XDR,
    pub su_verfbody: [libc::c_char; 400],
    pub su_cache: *mut libc::c_void,
}
/* 75% sparse */
/*
 * An entry in the cache
 */
#[c2rust::src_loc = "356:1"]
pub type cache_ptr = *mut cache_node;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "357:8"]
pub struct cache_node {
    pub cache_xid: uint32_t,
    pub cache_proc: rpcproc_t,
    pub cache_vers: rpcvers_t,
    pub cache_prog: rpcprog_t,
    pub cache_addr: sockaddr_in,
    pub cache_reply: *mut libc::c_char,
    pub cache_replylen: uint32_t,
    pub cache_next: cache_ptr,
}
/*
 * The entire cache
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "382:8"]
pub struct udp_cache {
    pub uc_size: uint32_t,
    pub uc_entries: *mut cache_ptr,
    pub uc_fifo: *mut cache_ptr,
    pub uc_nextvictim: uint32_t,
    pub uc_prog: rpcprog_t,
    pub uc_vers: rpcvers_t,
    pub uc_proc: rpcproc_t,
    pub uc_addr: sockaddr_in,
}
#[c2rust::src_loc = "74:22"]
static mut svcudp_op: xp_ops =
    unsafe {
        {
            let mut init =
                xp_ops{xp_recv:
                           Some(svcudp_recv as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: *mut rpc_msg)
                                        -> libc::c_int),
                       xp_stat:
                           Some(svcudp_stat as
                                    unsafe extern "C" fn(_: *mut SVCXPRT)
                                        -> xprt_stat),
                       xp_getargs:
                           Some(svcudp_getargs as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: xdrproc_t,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int),
                       xp_reply:
                           Some(svcudp_reply as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: *mut rpc_msg)
                                        -> libc::c_int),
                       xp_freeargs:
                           Some(svcudp_freeargs as
                                    unsafe extern "C" fn(_: *mut SVCXPRT,
                                                         _: xdrproc_t,
                                                         _: *mut libc::c_void)
                                        -> libc::c_int),
                       xp_destroy:
                           Some(svcudp_destroy as
                                    unsafe extern "C" fn(_: *mut SVCXPRT)
                                        -> ()),};
            init
        }
    };
/* cached data, NULL if no cache */
/*
 * Usage:
 *	xprt = svcudp_create(sock);
 *
 * If sock<0 then a socket is created, else sock is used.
 * If the socket, sock is not bound to a port then svcudp_create
 * binds it to an arbitrary port.  In any (successful) case,
 * xprt->xp_sock is the registered socket number and xprt->xp_port is the
 * associated port number.
 * Once *xprt is initialized, it is registered as a transporter;
 * see (svc.h, xprt_register).
 * The routines returns NULL if a problem occurred.
 */
#[no_mangle]
#[c2rust::src_loc = "109:1"]
pub unsafe extern "C" fn gssrpc_svcudp_bufcreate(mut sock: libc::c_int,
                                                 mut sendsz: u_int,
                                                 mut recvsz: u_int)
 -> *mut SVCXPRT {
    let mut madesock: libc::c_int = 0 as libc::c_int;
    let mut xprt: *mut SVCXPRT = 0 as *mut SVCXPRT;
    let mut su: *mut svcudp_data = 0 as *mut svcudp_data;
    let mut ss: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut sa: *mut sockaddr =
        &mut ss as *mut sockaddr_storage as *mut sockaddr;
    let mut len: socklen_t = 0;
    if sock == -(1 as libc::c_int) {
        sock =
            socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int,
                   IPPROTO_UDP as libc::c_int);
        if sock < 0 as libc::c_int {
            perror(b"svcudp_create: socket creation problem\x00" as *const u8
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
        if getsockname(sock, __SOCKADDR_ARG{__sockaddr__: sa,}, &mut len) <
               0 as libc::c_int {
            perror(b"svcudp_create - cannot getsockname\x00" as *const u8 as
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
        perror(b"svcudp_create - cannot getsockname\x00" as *const u8 as
                   *const libc::c_char);
        if madesock != 0 { close(sock); }
        return 0 as *mut libc::c_void as *mut SVCXPRT
    }
    xprt =
        malloc(::std::mem::size_of::<SVCXPRT>() as libc::c_ulong) as
            *mut SVCXPRT;
    if xprt.is_null() {
        fprintf(stderr,
                b"svcudp_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as *mut SVCXPRT
    }
    su =
        malloc(::std::mem::size_of::<svcudp_data>() as libc::c_ulong) as
            *mut svcudp_data;
    if su.is_null() {
        fprintf(stderr,
                b"svcudp_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as *mut SVCXPRT
    }
    (*su).su_iosz =
        (if sendsz > recvsz {
             sendsz
         } else {
             recvsz
         }).wrapping_add(3 as libc::c_int as
                             libc::c_uint).wrapping_div(4 as libc::c_int as
                                                            libc::c_uint).wrapping_mul(4
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint);
    (*xprt).xp_p1 = malloc((*su).su_iosz as libc::c_ulong);
    if (*xprt).xp_p1.is_null() {
        fprintf(stderr,
                b"svcudp_create: out of memory\n\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as *mut SVCXPRT
    }
    gssrpc_xdrmem_create(&mut (*su).su_xdrs, (*xprt).xp_p1 as caddr_t,
                         (*su).su_iosz, XDR_DECODE);
    (*su).su_cache = 0 as *mut libc::c_void;
    (*xprt).xp_p2 = su as caddr_t as *mut libc::c_void;
    (*xprt).xp_auth = 0 as *mut SVCAUTH;
    (*xprt).xp_verf.oa_base = (*su).su_verfbody.as_mut_ptr();
    (*xprt).xp_ops = &mut svcudp_op;
    (*xprt).xp_port = sa_getport(sa);
    (*xprt).xp_sock = sock;
    gssrpc_xprt_register(xprt);
    return xprt;
}
#[no_mangle]
#[c2rust::src_loc = "178:1"]
pub unsafe extern "C" fn gssrpc_svcudp_create(mut sock: libc::c_int)
 -> *mut SVCXPRT {
    return gssrpc_svcudp_bufcreate(sock, 8800 as libc::c_int as u_int,
                                   8800 as libc::c_int as u_int);
}
#[c2rust::src_loc = "185:1"]
unsafe extern "C" fn svcudp_stat(mut xprt: *mut SVCXPRT) -> xprt_stat {
    return XPRT_IDLE;
}
/* @(#)svc_udp.c	2.2 88/07/29 4.0 RPCSRC */
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
 * svc_udp.c,
 * Server side for UDP/IP based RPC.  (Does some caching in the hopes of
 * achieving execute-at-most-once semantics.)
 */
#[c2rust::src_loc = "192:1"]
unsafe extern "C" fn svcudp_recv(mut xprt: *mut SVCXPRT,
                                 mut msg: *mut rpc_msg) -> libc::c_int {
    let mut dummy: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut dummy_iov: [iovec; 1] =
        [iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,}; 1];
    let mut su: *mut svcudp_data = (*xprt).xp_p2 as *mut svcudp_data;
    let mut xdrs: *mut XDR = &mut (*su).su_xdrs;
    let mut rlen: libc::c_int = 0;
    let mut reply: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut replylen: uint32_t = 0;
    let mut addrlen: socklen_t = 0;
    loop  {
        memset(&mut dummy as *mut msghdr as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<msghdr>() as libc::c_ulong);
        dummy_iov[0 as libc::c_int as usize].iov_base = (*xprt).xp_p1;
        dummy_iov[0 as libc::c_int as usize].iov_len =
            (*su).su_iosz as libc::c_int as size_t;
        dummy.msg_iov = dummy_iov.as_mut_ptr();
        dummy.msg_iovlen = 1 as libc::c_int as size_t;
        (*xprt).xp_laddrlen =
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                libc::c_int;
        dummy.msg_namelen = (*xprt).xp_laddrlen as socklen_t;
        dummy.msg_name =
            &mut (*xprt).xp_laddr as *mut sockaddr_in as *mut libc::c_char as
                *mut libc::c_void;
        rlen =
            recvmsg((*xprt).xp_sock, &mut dummy, MSG_PEEK as libc::c_int) as
                libc::c_int;
        if rlen == -(1 as libc::c_int) {
            if *__errno_location() == 4 as libc::c_int { continue ; }
            return 0 as libc::c_int
        } else {
            addrlen =
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                    socklen_t;
            rlen =
                recvfrom((*xprt).xp_sock, (*xprt).xp_p1,
                         (*su).su_iosz as libc::c_int as size_t,
                         0 as libc::c_int,
                         __SOCKADDR_ARG{__sockaddr__:
                                            &mut (*xprt).xp_raddr as
                                                *mut sockaddr_in as
                                                *mut sockaddr,}, &mut addrlen)
                    as libc::c_int;
            if rlen == -(1 as libc::c_int) &&
                   *__errno_location() == 4 as libc::c_int {
                continue ;
            }
            if rlen <
                   (4 as libc::c_int as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint32_t>()
                                                        as libc::c_ulong) as
                       libc::c_int {
                return 0 as libc::c_int
            }
            (*xprt).xp_addrlen = addrlen as libc::c_int;
            (*xdrs).x_op = XDR_DECODE;
            Some((*(*xdrs).x_ops).x_setpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                      0
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          u_int);
            if gssrpc_xdr_callmsg(xdrs, msg) == 0 { return 0 as libc::c_int }
            (*su).su_xid = (*msg).rm_xid;
            if !(*su).su_cache.is_null() {
                if cache_get(xprt, msg, &mut reply, &mut replylen) != 0 {
                    sendto((*xprt).xp_sock, reply as *const libc::c_void,
                           replylen as libc::c_int as size_t,
                           0 as libc::c_int,
                           __CONST_SOCKADDR_ARG{__sockaddr__:
                                                    &mut (*xprt).xp_raddr as
                                                        *mut sockaddr_in as
                                                        *mut sockaddr,},
                           (*xprt).xp_addrlen as socklen_t);
                    return 1 as libc::c_int
                }
            }
            return 1 as libc::c_int
        }
    };
}
#[c2rust::src_loc = "245:1"]
unsafe extern "C" fn svcudp_reply(mut xprt: *mut SVCXPRT,
                                  mut msg: *mut rpc_msg) -> libc::c_int {
    let mut su: *mut svcudp_data = (*xprt).xp_p2 as *mut svcudp_data;
    let mut xdrs: *mut XDR = &mut (*su).su_xdrs;
    let mut slen: libc::c_int = 0;
    let mut stat: libc::c_int = 0 as libc::c_int;
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
    Some((*(*xdrs).x_ops).x_setpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                              0
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  u_int);
    (*msg).rm_xid = (*su).su_xid;
    if gssrpc_xdr_replymsg(xdrs, msg) != 0 &&
           (has_args == 0 ||
                Some((*(*(*xprt).xp_auth).svc_ah_ops).svc_ah_wrap.expect("non-null function pointer")).expect("non-null function pointer")((*xprt).xp_auth,
                                                                                                                                           xdrs,
                                                                                                                                           xdr_results,
                                                                                                                                           xdr_location)
                    != 0) {
        slen =
            Some((*(*xdrs).x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs)
                as libc::c_int;
        if sendto((*xprt).xp_sock, (*xprt).xp_p1, slen as size_t,
                  0 as libc::c_int,
                  __CONST_SOCKADDR_ARG{__sockaddr__:
                                           &mut (*xprt).xp_raddr as
                                               *mut sockaddr_in as
                                               *mut sockaddr,},
                  (*xprt).xp_addrlen as socklen_t) == slen as libc::c_long {
            stat = 1 as libc::c_int;
            if !(*su).su_cache.is_null() && slen >= 0 as libc::c_int {
                cache_set(xprt, slen as uint32_t);
            }
        }
    }
    return stat;
}
#[c2rust::src_loc = "288:1"]
unsafe extern "C" fn svcudp_getargs(mut xprt: *mut SVCXPRT,
                                    mut xdr_args: xdrproc_t,
                                    mut args_ptr: *mut libc::c_void)
 -> libc::c_int {
    if Some((*(*(*xprt).xp_auth).svc_ah_ops).svc_ah_unwrap.expect("non-null function pointer")).expect("non-null function pointer")((*xprt).xp_auth,
                                                                                                                                    &mut (*((*xprt).xp_p2
                                                                                                                                                as
                                                                                                                                                *mut svcudp_data)).su_xdrs,
                                                                                                                                    xdr_args,
                                                                                                                                    args_ptr
                                                                                                                                        as
                                                                                                                                        caddr_t)
           == 0 {
        svcudp_freeargs(xprt, xdr_args, args_ptr);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "302:1"]
unsafe extern "C" fn svcudp_freeargs(mut xprt: *mut SVCXPRT,
                                     mut xdr_args: xdrproc_t,
                                     mut args_ptr: *mut libc::c_void)
 -> libc::c_int {
    let mut xdrs: *mut XDR =
        &mut (*((*xprt).xp_p2 as *mut svcudp_data)).su_xdrs;
    (*xdrs).x_op = XDR_FREE;
    return ::std::mem::transmute::<_,
                                   fn(_: _, _: _)
                                       ->
                                           libc::c_int>(Some(xdr_args.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                args_ptr);
}
#[c2rust::src_loc = "314:1"]
unsafe extern "C" fn svcudp_destroy(mut xprt: *mut SVCXPRT) {
    let mut su: *mut svcudp_data = (*xprt).xp_p2 as *mut svcudp_data;
    gssrpc_xprt_unregister(xprt);
    if (*xprt).xp_sock != !(0 as libc::c_int) { close((*xprt).xp_sock); }
    (*xprt).xp_sock = !(0 as libc::c_int);
    if !(*xprt).xp_auth.is_null() {
        Some((*(*(*xprt).xp_auth).svc_ah_ops).svc_ah_destroy.expect("non-null function pointer")).expect("non-null function pointer")((*xprt).xp_auth);
        (*xprt).xp_auth = 0 as *mut SVCAUTH
    }
    if (*(*su).su_xdrs.x_ops).x_destroy.is_some() {
        Some((*(*su).su_xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut (*su).su_xdrs);
    }
    free((*xprt).xp_p1);
    free(su as caddr_t as *mut libc::c_void);
    free(xprt as caddr_t as *mut libc::c_void);
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
/* saved caller's address */
/*
 * the hashing function
 */
/*
 * Enable use of the cache.
 * Note: there is no disable.
 */
#[no_mangle]
#[c2rust::src_loc = "405:1"]
pub unsafe extern "C" fn gssrpc_svcudp_enablecache(mut transp: *mut SVCXPRT,
                                                   mut size: uint32_t)
 -> libc::c_int {
    let mut su: *mut svcudp_data = (*transp).xp_p2 as *mut svcudp_data;
    let mut uc: *mut udp_cache = 0 as *mut udp_cache;
    if !(*su).su_cache.is_null() {
        fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                b"enablecache: cache already enabled\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as libc::c_int
    }
    uc =
        malloc((::std::mem::size_of::<udp_cache>() as
                    libc::c_ulong).wrapping_mul(1 as libc::c_int as
                                                    libc::c_ulong) as
                   libc::c_uint as libc::c_ulong) as *mut udp_cache;
    if uc.is_null() {
        fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                b"enablecache: could not allocate cache\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as libc::c_int
    }
    (*uc).uc_size = size;
    (*uc).uc_nextvictim = 0 as libc::c_int as uint32_t;
    (*uc).uc_entries =
        malloc((::std::mem::size_of::<cache_ptr>() as
                    libc::c_ulong).wrapping_mul(size.wrapping_mul(4 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                    as libc::c_ulong) as
                   libc::c_uint as libc::c_ulong) as *mut cache_ptr;
    if (*uc).uc_entries.is_null() {
        fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                b"enablecache: could not allocate cache data\x00" as *const u8
                    as *const libc::c_char);
        return 0 as libc::c_int
    }
    memset((*uc).uc_entries as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<cache_ptr>() as
                libc::c_ulong).wrapping_mul(size.wrapping_mul(4 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                                                as libc::c_int as
                                                libc::c_ulong));
    (*uc).uc_fifo =
        malloc((::std::mem::size_of::<cache_ptr>() as
                    libc::c_ulong).wrapping_mul(size as libc::c_ulong) as
                   libc::c_uint as libc::c_ulong) as *mut cache_ptr;
    if (*uc).uc_fifo.is_null() {
        fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                b"enablecache: could not allocate cache fifo\x00" as *const u8
                    as *const libc::c_char);
        return 0 as libc::c_int
    }
    memset((*uc).uc_fifo as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<cache_ptr>() as
                libc::c_ulong).wrapping_mul(size as libc::c_int as
                                                libc::c_ulong));
    (*su).su_cache = uc as *mut libc::c_char as *mut libc::c_void;
    return 1 as libc::c_int;
}
/*
 * Set an entry in the cache
 */
#[c2rust::src_loc = "444:1"]
unsafe extern "C" fn cache_set(mut xprt: *mut SVCXPRT,
                               mut replylen: uint32_t) {
    let mut victim: cache_ptr = 0 as *mut cache_node;
    let mut vicp: *mut cache_ptr = 0 as *mut cache_ptr;
    let mut su: *mut svcudp_data = (*xprt).xp_p2 as *mut svcudp_data;
    let mut uc: *mut udp_cache = (*su).su_cache as *mut udp_cache;
    let mut loc: u_int = 0;
    let mut newbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    /*
 	 * Find space for the new entry, either by
	 * reusing an old entry, or by mallocing a new one
	 */
    victim =
        *(*uc).uc_fifo.offset((*uc).uc_nextvictim as
                                  isize); /* remote from cache */
    if !victim.is_null() {
        loc =
            (*victim).cache_xid.wrapping_rem((4 as libc::c_int as
                                                  libc::c_uint).wrapping_mul((*((*((*xprt).xp_p2
                                                                                       as
                                                                                       *mut svcudp_data)).su_cache
                                                                                    as
                                                                                    *mut udp_cache)).uc_size));
        vicp = &mut *(*uc).uc_entries.offset(loc as isize) as *mut cache_ptr;
        while !(*vicp).is_null() && *vicp != victim {
            vicp = &mut (**vicp).cache_next
        }
        if (*vicp).is_null() {
            fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    b"cache_set: victim not found\x00" as *const u8 as
                        *const libc::c_char);
            return
        }
        *vicp = (*victim).cache_next;
        newbuf = (*victim).cache_reply
    } else {
        victim =
            malloc((::std::mem::size_of::<cache_node>() as
                        libc::c_ulong).wrapping_mul(1 as libc::c_int as
                                                        libc::c_ulong) as
                       libc::c_uint as libc::c_ulong) as *mut cache_node;
        if victim.is_null() {
            fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    b"cache_set: victim alloc failed\x00" as *const u8 as
                        *const libc::c_char);
            return
        }
        newbuf = malloc((*su).su_iosz as libc::c_ulong) as *mut libc::c_char;
        if newbuf.is_null() {
            fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                    b"cache_set: could not allocate new rpc_buffer\x00" as
                        *const u8 as *const libc::c_char);
            free(victim as *mut libc::c_void);
            return
        }
    }
    /*
	 * Store it away
	 */
    (*victim).cache_replylen = replylen;
    (*victim).cache_reply = (*xprt).xp_p1 as *mut libc::c_char;
    (*xprt).xp_p1 = newbuf as *mut libc::c_void;
    gssrpc_xdrmem_create(&mut (*su).su_xdrs, (*xprt).xp_p1 as caddr_t,
                         (*su).su_iosz, XDR_ENCODE);
    (*victim).cache_xid = (*su).su_xid;
    (*victim).cache_proc = (*uc).uc_proc;
    (*victim).cache_vers = (*uc).uc_vers;
    (*victim).cache_prog = (*uc).uc_prog;
    (*victim).cache_addr = (*uc).uc_addr;
    loc =
        (*victim).cache_xid.wrapping_rem((4 as libc::c_int as
                                              libc::c_uint).wrapping_mul((*((*((*xprt).xp_p2
                                                                                   as
                                                                                   *mut svcudp_data)).su_cache
                                                                                as
                                                                                *mut udp_cache)).uc_size));
    (*victim).cache_next = *(*uc).uc_entries.offset(loc as isize);
    let ref mut fresh0 = *(*uc).uc_entries.offset(loc as isize);
    *fresh0 = victim;
    let fresh1 = (*uc).uc_nextvictim;
    (*uc).uc_nextvictim = (*uc).uc_nextvictim.wrapping_add(1);
    let ref mut fresh2 = *(*uc).uc_fifo.offset(fresh1 as isize);
    *fresh2 = victim;
    (*uc).uc_nextvictim =
        ((*uc).uc_nextvictim as libc::c_uint).wrapping_rem((*uc).uc_size) as
            uint32_t as uint32_t;
}
/*
 * Try to get an entry from the cache
 * return 1 if found, 0 if not found
 */
#[c2rust::src_loc = "510:1"]
unsafe extern "C" fn cache_get(mut xprt: *mut SVCXPRT, mut msg: *mut rpc_msg,
                               mut replyp: *mut *mut libc::c_char,
                               mut replylenp: *mut uint32_t) -> libc::c_int {
    let mut loc: u_int = 0;
    let mut ent: cache_ptr = 0 as *mut cache_node;
    let mut su: *mut svcudp_data = (*xprt).xp_p2 as *mut svcudp_data;
    let mut uc: *mut udp_cache = (*su).su_cache as *mut udp_cache;
    loc =
        (*su).su_xid.wrapping_rem((4 as libc::c_int as
                                       libc::c_uint).wrapping_mul((*((*((*xprt).xp_p2
                                                                            as
                                                                            *mut svcudp_data)).su_cache
                                                                         as
                                                                         *mut udp_cache)).uc_size));
    ent = *(*uc).uc_entries.offset(loc as isize);
    while !ent.is_null() {
        if (*ent).cache_xid == (*su).su_xid &&
               (*ent).cache_proc == (*uc).uc_proc &&
               (*ent).cache_vers == (*uc).uc_vers &&
               (*ent).cache_prog == (*uc).uc_prog &&
               memcmp(&mut (*ent).cache_addr as *mut sockaddr_in as
                          *mut libc::c_char as *const libc::c_void,
                      &mut (*uc).uc_addr as *mut sockaddr_in as
                          *mut libc::c_char as *const libc::c_void,
                      ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong)
                   == 0 as libc::c_int {
            *replyp = (*ent).cache_reply;
            *replylenp = (*ent).cache_replylen;
            return 1 as libc::c_int
        }
        ent = (*ent).cache_next
    }
    /*
	 * Failed to find entry
	 * Remember a few things so we can do a set later
	 */
    (*uc).uc_proc = (*msg).ru.RM_cmb.cb_proc;
    (*uc).uc_vers = (*msg).ru.RM_cmb.cb_vers;
    (*uc).uc_prog = (*msg).ru.RM_cmb.cb_prog;
    (*uc).uc_addr = (*xprt).xp_raddr;
    return 0 as libc::c_int;
}
