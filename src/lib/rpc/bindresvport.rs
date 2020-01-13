use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:36"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:37"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/unistd.h:38"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::{__socklen_t, __pid_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "628:1"]
        pub fn getpid() -> __pid_t;
    }
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:41"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:41"]
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
#[c2rust::header_src = "/usr/include/netinet/in.h:42"]
pub mod in_h {
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "119:1"]
    pub type in_port_t = uint16_t;
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
    #[c2rust::src_loc = "253:8"]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: uint32_t,
    }
    use super::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
    use super::sockaddr_h::sa_family_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:36"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:40"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/socket.h:41"]
pub mod sys_socket_h {
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn bind(__fd: libc::c_int, __addr: *const sockaddr,
                    __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn getsockname(__fd: libc::c_int, __addr: *mut sockaddr,
                           __len: *mut socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/socket-utils.h:45"]
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
    #[c2rust::src_loc = "75:1"]
    pub unsafe extern "C" fn sa2sin6(mut sa: *mut sockaddr)
     -> *mut sockaddr_in6 {
        return sa as *mut libc::c_void as *mut sockaddr_in6;
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
    #[inline]
    #[c2rust::src_loc = "116:1"]
    pub unsafe extern "C" fn sa_is_inet(mut sa: *mut sockaddr)
     -> libc::c_int {
        return ((*sa).sa_family as libc::c_int == 2 as libc::c_int ||
                    (*sa).sa_family as libc::c_int == 10 as libc::c_int) as
                   libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn ss2sa(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr {
        return ss as *mut sockaddr;
    }
    #[inline]
    #[c2rust::src_loc = "71:1"]
    pub unsafe extern "C" fn sa2sin(mut sa: *mut sockaddr)
     -> *mut sockaddr_in {
        return sa as *mut libc::c_void as *mut sockaddr_in;
    }
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
    use super::socket_h::{sockaddr, sockaddr_storage};
    use super::in_h::{sockaddr_in6, sockaddr_in, htons, in_port_t};
    use super::unistd_h::socklen_t;
    use super::sockaddr_h::sa_family_t;
    use super::stdlib_h::abort;
    use super::stdint_uintn_h::uint16_t;
    /* SOCKET_UTILS_H */
}
pub use self::types_h::{__uint8_t, __uint16_t, __uint32_t, __pid_t,
                        __socklen_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::unistd_h::{socklen_t, getpid};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage};
pub use self::in_h::{in_addr_t, in_addr, in_port_t, in6_addr, C2RustUnnamed,
                     sockaddr_in, sockaddr_in6, htons};
use self::stdlib_h::abort;
use self::errno_h::__errno_location;
use self::sys_socket_h::{bind, getsockname};
pub use self::socket_utils_h::{sa2sin6, sa_socklen, sa_is_inet, ss2sa, sa2sin,
                               sa_setport};
/* lib/rpc/bindresvport.c */
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
 * Bind a socket to a privileged IP port
 */
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn gssrpc_bindresvport_sa(mut sd: libc::c_int,
                                                mut sa: *mut sockaddr)
 -> libc::c_int {
    let mut res: libc::c_int = 0;
    static mut port: libc::c_short = 0;
    let mut myaddr: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut salen: socklen_t = 0;
    let mut i: libc::c_int = 0;
    if sa.is_null() {
        salen =
            ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
                socklen_t;
        sa = ss2sa(&mut myaddr);
        res = getsockname(sd, sa, &mut salen);
        if res < 0 as libc::c_int { return -(1 as libc::c_int) }
    }
    if sa_is_inet(sa) == 0 {
        *__errno_location() = 96 as libc::c_int;
        return -(1 as libc::c_int)
    }
    if port as libc::c_int == 0 as libc::c_int {
        port =
            (getpid() %
                 (1024 as libc::c_int - 1 as libc::c_int - 600 as libc::c_int
                      + 1 as libc::c_int) + 600 as libc::c_int) as
                libc::c_short
    }
    res = -(1 as libc::c_int);
    *__errno_location() = 98 as libc::c_int;
    i = 0 as libc::c_int;
    while i <
              1024 as libc::c_int - 1 as libc::c_int - 600 as libc::c_int +
                  1 as libc::c_int && res < 0 as libc::c_int &&
              *__errno_location() == 98 as libc::c_int {
        let fresh0 = port;
        port = port + 1;
        sa_setport(sa, fresh0 as uint16_t);
        if port as libc::c_int > 1024 as libc::c_int - 1 as libc::c_int {
            port = 600 as libc::c_int as libc::c_short
        }
        res = bind(sd, sa, sa_socklen(sa));
        i += 1
    }
    return res;
}
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
#[c2rust::src_loc = "88:1"]
pub unsafe extern "C" fn gssrpc_bindresvport(mut sd: libc::c_int,
                                             mut sockin: *mut sockaddr_in)
 -> libc::c_int {
    return gssrpc_bindresvport_sa(sd, sockin as *mut sockaddr);
}
