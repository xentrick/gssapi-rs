use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:26"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:26"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:26"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:26"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_iovec.h:26"]
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
#[c2rust::header_src = "/usr/include/unistd.h:26"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:26"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
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
    use super::stdint_intn_h::int32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:26"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:26"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "275:8"]
    pub struct cmsghdr {
        pub cmsg_len: size_t,
        pub cmsg_level: libc::c_int,
        pub cmsg_type: libc::c_int,
        pub __cmsg_data: [libc::c_uchar; 0],
    }
    use super::sockaddr_h::sa_family_t;
    use super::unistd_h::socklen_t;
    use super::struct_iovec_h::iovec;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "305:1"]
        pub fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr)
         -> *mut cmsghdr;
    }
}
#[c2rust::header_src = "/usr/include/sys/socket.h:26"]
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
        #[c2rust::src_loc = "173:1"]
        pub fn sendmsg(__fd: libc::c_int, __message: *const msghdr,
                       __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "191:1"]
        pub fn recvmsg(__fd: libc::c_int, __message: *mut msghdr,
                       __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int,
                          __optval: *const libc::c_void, __optlen: socklen_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:26"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "537:8"]
    pub struct in6_pktinfo {
        pub ipi6_addr: in6_addr,
        pub ipi6_ifindex: libc::c_uint,
    }
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
    }
}
#[c2rust::header_src = "/usr/include/bits/in.h:26"]
pub mod bits_in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "157:8"]
    pub struct in_pktinfo {
        pub ipi_ifindex: libc::c_int,
        pub ipi_spec_dst: in_addr,
        pub ipi_addr: in_addr,
    }
    use super::in_h::in_addr;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/apputils/udppktinfo.h:26"]
pub mod udppktinfo_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:15"]
    pub union aux_addressing_info {
        pub ipv6_ifindex: libc::c_int,
    }
    /* UDPPKTINFO_H */
}
#[c2rust::header_src = "/usr/include/errno.h:26"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:26"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/socket-utils.h:26"]
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
    #[inline]
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn ss2sa(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr {
        return ss as *mut sockaddr;
    }
    /* Return true if sa is an IPv4 or IPv6 socket address. */
    #[inline]
    #[c2rust::src_loc = "116:1"]
    pub unsafe extern "C" fn sa_is_inet(mut sa: *mut sockaddr)
     -> libc::c_int {
        return ((*sa).sa_family as libc::c_int == 2 as libc::c_int ||
                    (*sa).sa_family as libc::c_int == 10 as libc::c_int) as
                   libc::c_int;
    }
    /* Return true if sa is an IPv4 or IPv6 wildcard address. */
    #[inline]
    #[c2rust::src_loc = "123:1"]
    pub unsafe extern "C" fn sa_is_wildcard(mut sa: *mut sockaddr)
     -> libc::c_int {
        if (*sa).sa_family as libc::c_int == 10 as libc::c_int {
            return ({
                        let mut __a: *const in6_addr =
                            &mut (*(sa2sin6 as
                                        unsafe extern "C" fn(_: *mut sockaddr)
                                            ->
                                                *mut sockaddr_in6)(sa)).sin6_addr
                                as *mut in6_addr as *const in6_addr;
                        ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                             == 0 as libc::c_int as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                            usize] ==
                                 0 as libc::c_int as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                            usize] ==
                                 0 as libc::c_int as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                            usize] ==
                                 0 as libc::c_int as libc::c_uint) as
                            libc::c_int
                    })
        } else {
            if (*sa).sa_family as libc::c_int == 2 as libc::c_int {
                return ((*sa2sin(sa)).sin_addr.s_addr ==
                            0 as libc::c_int as in_addr_t) as libc::c_int
            }
        }
        return 0 as libc::c_int;
    }
    use super::socket_h::{sockaddr, sockaddr_storage};
    use super::in_h::{sockaddr_in, sockaddr_in6, in6_addr, in_addr_t};
    use super::sockaddr_h::sa_family_t;
    /* SOCKET_UTILS_H */
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __ssize_t, __socklen_t};
pub use self::sys_types_h::ssize_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_iovec_h::iovec;
pub use self::unistd_h::socklen_t;
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage, msghdr, cmsghdr,
                         __cmsg_nxthdr};
pub use self::sys_socket_h::{__SOCKADDR_ARG, __CONST_SOCKADDR_ARG,
                             sockaddr_x25, sockaddr_un, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, getsockname, sendto, recvfrom,
                             sendmsg, recvmsg, setsockopt};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, C2RustUnnamed_0,
                     IPPROTO_MAX, IPPROTO_RAW, IPPROTO_MPLS, IPPROTO_UDPLITE,
                     IPPROTO_SCTP, IPPROTO_COMP, IPPROTO_PIM, IPPROTO_ENCAP,
                     IPPROTO_BEETPH, IPPROTO_MTP, IPPROTO_AH, IPPROTO_ESP,
                     IPPROTO_GRE, IPPROTO_RSVP, IPPROTO_IPV6, IPPROTO_DCCP,
                     IPPROTO_TP, IPPROTO_IDP, IPPROTO_UDP, IPPROTO_PUP,
                     IPPROTO_EGP, IPPROTO_TCP, IPPROTO_IPIP, IPPROTO_IGMP,
                     IPPROTO_ICMP, IPPROTO_IP, in6_pktinfo, htonl};
pub use self::bits_in_h::in_pktinfo;
pub use self::udppktinfo_h::aux_addressing_info;
use self::errno_h::__errno_location;
use self::string_h::memset;
pub use self::socket_utils_h::{sa2sin, sa2sin6, ss2sa, sa_is_inet,
                               sa_is_wildcard};
#[inline]
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn set_ipv4_recvpktinfo(mut sock: libc::c_int)
 -> krb5_error_code {
    let mut sockopt: libc::c_int = 1 as libc::c_int;
    return setsockopt(sock, IPPROTO_IP as libc::c_int, 8 as libc::c_int,
                      &mut sockopt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t);
}
#[inline]
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn set_ipv6_recvpktinfo(mut sock: libc::c_int)
 -> krb5_error_code {
    let mut sockopt: libc::c_int = 1 as libc::c_int;
    return setsockopt(sock, IPPROTO_IPV6 as libc::c_int, 49 as libc::c_int,
                      &mut sockopt as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t);
}
/* HAVE_IPV6_PKTINFO */
/* HAVE_IPV6_PKTINFO */
/*
 * Set pktinfo option on a socket. Takes a socket and the socket address family
 * as arguments.
 *
 * Returns 0 on success, EINVAL if pktinfo is not supported for the address
 * family.
 */
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn set_pktinfo(mut sock: libc::c_int,
                                     mut family: libc::c_int)
 -> krb5_error_code {
    match family {
        2 => { return set_ipv4_recvpktinfo(sock) }
        10 => { return set_ipv6_recvpktinfo(sock) }
        _ => { return 22 as libc::c_int }
    };
}
/*
 * Check if a socket is bound to a wildcard address.
 * Returns 1 if it is, 0 if it's bound to a specific address, or -1 on error
 * with errno set to the error.
 */
#[c2rust::src_loc = "139:1"]
unsafe extern "C" fn is_socket_bound_to_wildcard(mut sock: libc::c_int)
 -> libc::c_int {
    let mut bound_addr: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut bound_addr_len: socklen_t =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    let mut sa: *mut sockaddr = ss2sa(&mut bound_addr);
    if getsockname(sock, __SOCKADDR_ARG{__sockaddr__: sa,},
                   &mut bound_addr_len) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if sa_is_inet(sa) == 0 {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int)
    }
    return sa_is_wildcard(sa);
}
#[inline]
#[c2rust::src_loc = "159:1"]
unsafe extern "C" fn cmsg2pktinfo(mut cmsgptr: *mut cmsghdr)
 -> *mut in_pktinfo {
    return (*cmsgptr).__cmsg_data.as_mut_ptr() as *mut libc::c_void as
               *mut in_pktinfo;
}
#[c2rust::src_loc = "166:1"]
unsafe extern "C" fn check_cmsg_ip_pktinfo(mut cmsgptr: *mut cmsghdr,
                                           mut to: *mut sockaddr,
                                           mut tolen: *mut socklen_t,
                                           mut auxaddr:
                                               *mut aux_addressing_info)
 -> libc::c_int {
    let mut pktinfo: *mut in_pktinfo = 0 as *mut in_pktinfo;
    if (*cmsgptr).cmsg_level == IPPROTO_IP as libc::c_int &&
           (*cmsgptr).cmsg_type == 8 as libc::c_int &&
           *tolen as libc::c_ulong >=
               ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong {
        memset(to as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
        pktinfo = cmsg2pktinfo(cmsgptr);
        (*sa2sin(to)).sin_addr = (*pktinfo).ipi_addr;
        (*sa2sin(to)).sin_family = 2 as libc::c_int as sa_family_t;
        *tolen =
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* HAVE_IP_PKTINFO */
/* HAVE_IP_PKTINFO || IP_RECVDSTADDR */
#[inline]
#[c2rust::src_loc = "220:1"]
unsafe extern "C" fn cmsg2pktinfo6(mut cmsgptr: *mut cmsghdr)
 -> *mut in6_pktinfo {
    return (*cmsgptr).__cmsg_data.as_mut_ptr() as *mut libc::c_void as
               *mut in6_pktinfo;
}
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn check_cmsg_ipv6_pktinfo(mut cmsgptr: *mut cmsghdr,
                                             mut to: *mut sockaddr,
                                             mut tolen: *mut socklen_t,
                                             mut auxaddr:
                                                 *mut aux_addressing_info)
 -> libc::c_int {
    let mut pktinfo: *mut in6_pktinfo = 0 as *mut in6_pktinfo;
    if (*cmsgptr).cmsg_level == IPPROTO_IPV6 as libc::c_int &&
           (*cmsgptr).cmsg_type == 50 as libc::c_int &&
           *tolen as libc::c_ulong >=
               ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong {
        memset(to as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong);
        pktinfo = cmsg2pktinfo6(cmsgptr);
        (*sa2sin6(to)).sin6_addr = (*pktinfo).ipi6_addr;
        (*sa2sin6(to)).sin6_family = 10 as libc::c_int as sa_family_t;
        *tolen =
            ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as
                socklen_t;
        (*auxaddr).ipv6_ifindex = (*pktinfo).ipi6_ifindex as libc::c_int;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* HAVE_IPV6_PKTINFO */
/* HAVE_IPV6_PKTINFO */
#[c2rust::src_loc = "251:1"]
unsafe extern "C" fn check_cmsg_pktinfo(mut cmsgptr: *mut cmsghdr,
                                        mut to: *mut sockaddr,
                                        mut tolen: *mut socklen_t,
                                        mut auxaddr: *mut aux_addressing_info)
 -> libc::c_int {
    return (check_cmsg_ip_pktinfo(cmsgptr, to, tolen, auxaddr) != 0 ||
                check_cmsg_ipv6_pktinfo(cmsgptr, to, tolen, auxaddr) != 0) as
               libc::c_int;
}
/*
 * Receive a message from a socket.
 *
 * Arguments:
 *  sock
 *  buf     - The buffer to store the message in.
 *  len     - buf length
 *  flags
 *  from    - Set to the address that sent the message
 *  fromlen
 *  to      - Set to the address that the message was sent to if possible.
 *            May not be set in certain cases such as if pktinfo support is
 *            missing. May be NULL.
 *  tolen
 *  auxaddr - Miscellaneous address information.
 *
 * Returns 0 on success, otherwise an error code.
 */
#[no_mangle]
#[c2rust::src_loc = "277:1"]
pub unsafe extern "C" fn recv_from_to(mut sock: libc::c_int,
                                      mut buf: *mut libc::c_void,
                                      mut len: size_t, mut flags: libc::c_int,
                                      mut from: *mut sockaddr,
                                      mut fromlen: *mut socklen_t,
                                      mut to: *mut sockaddr,
                                      mut tolen: *mut socklen_t,
                                      mut auxaddr: *mut aux_addressing_info)
 -> krb5_error_code {
    let mut r: libc::c_int = 0;
    let mut iov: iovec = iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,};
    let mut cmsg: [libc::c_char; 40] = [0; 40];
    let mut cmsgptr: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    /* Don't use pktinfo if the socket isn't bound to a wildcard address. */
    r = is_socket_bound_to_wildcard(sock);
    if r < 0 as libc::c_int { return *__errno_location() }
    if to.is_null() || tolen.is_null() || r == 0 {
        return recvfrom(sock, buf, len, flags,
                        __SOCKADDR_ARG{__sockaddr__: from,}, fromlen) as
                   krb5_error_code
    }
    /* Clobber with something recognizeable in case we can't extract the
     * address but try to use it anyways. */
    memset(to as *mut libc::c_void, 0x40 as libc::c_int,
           *tolen as libc::c_ulong);
    iov.iov_base = buf;
    iov.iov_len = len;
    memset(&mut msg as *mut msghdr as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<msghdr>() as libc::c_ulong);
    msg.msg_name = from as *mut libc::c_void;
    msg.msg_namelen = *fromlen;
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    msg.msg_control = cmsg.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen =
        ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong;
    r = recvmsg(sock, &mut msg, flags) as libc::c_int;
    if r < 0 as libc::c_int { return r }
    *fromlen = msg.msg_namelen;
    /*
     * On Darwin (and presumably all *BSD with KAME stacks), CMSG_FIRSTHDR
     * doesn't check for a non-zero controllen.  RFC 3542 recommends making
     * this check, even though the (new) spec for CMSG_FIRSTHDR says it's
     * supposed to do the check.
     */
    if msg.msg_controllen != 0 {
        cmsgptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msg.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        while !cmsgptr.is_null() {
            if check_cmsg_pktinfo(cmsgptr, to, tolen, auxaddr) != 0 {
                return r
            }
            cmsgptr = __cmsg_nxthdr(&mut msg, cmsgptr)
        }
    }
    /* No info about destination addr was available.  */
    *tolen = 0 as libc::c_int as socklen_t;
    return r;
}
#[c2rust::src_loc = "339:1"]
unsafe extern "C" fn set_msg_from_ip_pktinfo(mut msg: *mut msghdr,
                                             mut cmsgptr: *mut cmsghdr,
                                             mut from: *mut sockaddr,
                                             mut fromlen: socklen_t,
                                             mut auxaddr:
                                                 *mut aux_addressing_info)
 -> krb5_error_code {
    let mut p: *mut in_pktinfo = cmsg2pktinfo(cmsgptr);
    let mut from4: *const sockaddr_in = sa2sin(from);
    if fromlen as libc::c_ulong !=
           ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong {
        return 22 as libc::c_int
    }
    (*cmsgptr).cmsg_level = IPPROTO_IP as libc::c_int;
    (*cmsgptr).cmsg_type = 8 as libc::c_int;
    (*cmsgptr).cmsg_len =
        ((::std::mem::size_of::<cmsghdr>() as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as
                                              libc::c_ulong).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
             &
             !(::std::mem::size_of::<size_t>() as
                   libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong)).wrapping_add(::std::mem::size_of::<in_pktinfo>()
                                                                                    as
                                                                                    libc::c_ulong);
    (*p).ipi_spec_dst = (*from4).sin_addr;
    (*msg).msg_controllen =
        ((::std::mem::size_of::<in_pktinfo>() as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as
                                              libc::c_ulong).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
             &
             !(::std::mem::size_of::<size_t>() as
                   libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong)).wrapping_add((::std::mem::size_of::<cmsghdr>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
                                                                                                                     as
                                                                                                                     libc::c_ulong).wrapping_sub(1
                                                                                                                                                     as
                                                                                                                                                     libc::c_int
                                                                                                                                                     as
                                                                                                                                                     libc::c_ulong)
                                                                                    &
                                                                                    !(::std::mem::size_of::<size_t>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_sub(1
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong));
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "385:1"]
unsafe extern "C" fn set_msg_from_ipv6_pktinfo(mut msg: *mut msghdr,
                                               mut cmsgptr: *mut cmsghdr,
                                               mut from: *mut sockaddr,
                                               mut fromlen: socklen_t,
                                               mut auxaddr:
                                                   *mut aux_addressing_info)
 -> krb5_error_code {
    let mut p: *mut in6_pktinfo = cmsg2pktinfo6(cmsgptr);
    let mut from6: *const sockaddr_in6 = sa2sin6(from);
    if fromlen as libc::c_ulong !=
           ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong {
        return 22 as libc::c_int
    }
    (*cmsgptr).cmsg_level = IPPROTO_IPV6 as libc::c_int;
    (*cmsgptr).cmsg_type = 50 as libc::c_int;
    (*cmsgptr).cmsg_len =
        ((::std::mem::size_of::<cmsghdr>() as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as
                                              libc::c_ulong).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
             &
             !(::std::mem::size_of::<size_t>() as
                   libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong)).wrapping_add(::std::mem::size_of::<in6_pktinfo>()
                                                                                    as
                                                                                    libc::c_ulong);
    (*p).ipi6_addr = (*from6).sin6_addr;
    /*
     * Because of the possibility of asymmetric routing, we
     * normally don't want to specify an interface.  However,
     * macOS doesn't like sending from a link-local address
     * (which can come up in testing at least, if you wind up
     * with a "foo.local" name) unless we do specify the
     * interface.
     */
    if ({
            let mut __a: *const in6_addr =
                &(*from6).sin6_addr as *const in6_addr;
            ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] &
                 htonl(0xffc00000 as libc::c_uint) ==
                 htonl(0xfe800000 as libc::c_uint)) as libc::c_int
        }) != 0 {
        (*p).ipi6_ifindex = (*auxaddr).ipv6_ifindex as libc::c_uint
    }
    /* otherwise, already zero */
    (*msg).msg_controllen =
        ((::std::mem::size_of::<in6_pktinfo>() as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as
                                              libc::c_ulong).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong)
             &
             !(::std::mem::size_of::<size_t>() as
                   libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                   libc::c_ulong)).wrapping_add((::std::mem::size_of::<cmsghdr>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>()
                                                                                                                     as
                                                                                                                     libc::c_ulong).wrapping_sub(1
                                                                                                                                                     as
                                                                                                                                                     libc::c_int
                                                                                                                                                     as
                                                                                                                                                     libc::c_ulong)
                                                                                    &
                                                                                    !(::std::mem::size_of::<size_t>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_sub(1
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_ulong));
    return 0 as libc::c_int;
}
/* HAVE_IPV6_PKTINFO */
/* HAVE_IPV6_PKTINFO */
#[c2rust::src_loc = "420:1"]
unsafe extern "C" fn set_msg_from(mut family: libc::c_int,
                                  mut msg: *mut msghdr,
                                  mut cmsgptr: *mut cmsghdr,
                                  mut from: *mut sockaddr,
                                  mut fromlen: socklen_t,
                                  mut auxaddr: *mut aux_addressing_info)
 -> krb5_error_code {
    match family {
        2 => {
            return set_msg_from_ip_pktinfo(msg, cmsgptr, from, fromlen,
                                           auxaddr)
        }
        10 => {
            return set_msg_from_ipv6_pktinfo(msg, cmsgptr, from, fromlen,
                                             auxaddr)
        }
        _ => { }
    }
    return 22 as libc::c_int;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2016 by the Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
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
 * This holds whatever additional information might be needed to
 * properly send back to the client from the correct local address.
 *
 * In this case, we only need one datum so far: On macOS, the
 * kernel doesn't seem to like sending from link-local addresses
 * unless we specify the correct interface.
 */
/*
 * Send a message to an address.
 *
 * Arguments:
 *  sock
 *  buf     - The message to send.
 *  len     - buf length
 *  flags
 *  to      - The address to send the message to.
 *  tolen
 *  from    - The address to attempt to send the message from. May be NULL.
 *  fromlen
 *  auxaddr - Miscellaneous address information.
 *
 * Returns 0 on success, otherwise an error code.
 */
#[no_mangle]
#[c2rust::src_loc = "451:1"]
pub unsafe extern "C" fn send_to_from(mut sock: libc::c_int,
                                      mut buf: *mut libc::c_void,
                                      mut len: size_t, mut flags: libc::c_int,
                                      mut to: *const sockaddr,
                                      mut tolen: socklen_t,
                                      mut from: *mut sockaddr,
                                      mut fromlen: socklen_t,
                                      mut auxaddr: *mut aux_addressing_info)
 -> krb5_error_code {
    let mut r: libc::c_int = 0;
    let mut iov: iovec = iovec{iov_base: 0 as *mut libc::c_void, iov_len: 0,};
    let mut msg: msghdr =
        msghdr{msg_name: 0 as *mut libc::c_void,
               msg_namelen: 0,
               msg_iov: 0 as *mut iovec,
               msg_iovlen: 0,
               msg_control: 0 as *mut libc::c_void,
               msg_controllen: 0,
               msg_flags: 0,};
    let mut cmsgptr: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut cbuf: [libc::c_char; 40] = [0; 40];
    /* Don't use pktinfo if the socket isn't bound to a wildcard address. */
    r = is_socket_bound_to_wildcard(sock);
    if r < 0 as libc::c_int { return *__errno_location() }
    if !(from.is_null() || fromlen == 0 as libc::c_int as libc::c_uint ||
             (*from).sa_family as libc::c_int !=
                 (*to).sa_family as libc::c_int || r == 0) {
        iov.iov_base = buf;
        iov.iov_len = len;
        /* Truncation?  */
        if iov.iov_len != len { return 22 as libc::c_int }
        memset(cbuf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong);
        memset(&mut msg as *mut msghdr as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<msghdr>() as libc::c_ulong);
        msg.msg_name = to as *mut libc::c_void;
        msg.msg_namelen = tolen;
        msg.msg_iov = &mut iov;
        msg.msg_iovlen = 1 as libc::c_int as size_t;
        msg.msg_control = cbuf.as_mut_ptr() as *mut libc::c_void;
        /* CMSG_FIRSTHDR needs a non-zero controllen, or it'll return NULL on
     * Linux. */
        msg.msg_controllen =
            ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong;
        cmsgptr =
            if msg.msg_controllen >=
                   ::std::mem::size_of::<cmsghdr>() as libc::c_ulong {
                msg.msg_control as *mut cmsghdr
            } else { 0 as *mut cmsghdr };
        msg.msg_controllen = 0 as libc::c_int as size_t;
        if !(set_msg_from((*from).sa_family as libc::c_int, &mut msg, cmsgptr,
                          from, fromlen, auxaddr) != 0) {
            return sendmsg(sock, &mut msg, flags) as krb5_error_code
        }
    }
    return sendto(sock, buf, len, flags,
                  __CONST_SOCKADDR_ARG{__sockaddr__: to,}, tolen) as
               krb5_error_code;
}
/* HAVE_PKTINFO_SUPPORT && CMSG_SPACE */
/* HAVE_PKTINFO_SUPPORT && CMSG_SPACE */
