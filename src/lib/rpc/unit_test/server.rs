use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:8"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:8"]
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
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "156:1"]
    pub type __clock_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:8"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:8"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/sys/types.h:8"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_short, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/types/__sigset_t.h:8"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/types/sigset_t.h:8"]
pub mod sigset_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type sigset_t = __sigset_t;
    use super::__sigset_t_h::__sigset_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:8"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:8"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/__sigval_t.h:17"]
pub mod __sigval_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type __sigval_t = sigval;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:7"]
    pub union sigval {
        pub sival_int: libc::c_int,
        pub sival_ptr: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/bits/types/siginfo_t.h:17"]
pub mod siginfo_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub struct siginfo_t {
        pub si_signo: libc::c_int,
        pub si_errno: libc::c_int,
        pub si_code: libc::c_int,
        pub __pad0: libc::c_int,
        pub _sifields: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:5"]
    pub union C2RustUnnamed {
        pub _pad: [libc::c_int; 28],
        pub _kill: C2RustUnnamed_8,
        pub _timer: C2RustUnnamed_7,
        pub _rt: C2RustUnnamed_6,
        pub _sigchld: C2RustUnnamed_5,
        pub _sigfault: C2RustUnnamed_2,
        pub _sigpoll: C2RustUnnamed_1,
        pub _sigsys: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:2"]
    pub struct C2RustUnnamed_0 {
        pub _call_addr: *mut libc::c_void,
        pub _syscall: libc::c_int,
        pub _arch: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_1 {
        pub si_band: libc::c_long,
        pub si_fd: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:2"]
    pub struct C2RustUnnamed_2 {
        pub si_addr: *mut libc::c_void,
        pub si_addr_lsb: libc::c_short,
        pub _bounds: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:6"]
    pub union C2RustUnnamed_3 {
        pub _addr_bnd: C2RustUnnamed_4,
        pub _pkey: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:3"]
    pub struct C2RustUnnamed_4 {
        pub _lower: *mut libc::c_void,
        pub _upper: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:2"]
    pub struct C2RustUnnamed_5 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_status: libc::c_int,
        pub si_utime: __clock_t,
        pub si_stime: __clock_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:2"]
    pub struct C2RustUnnamed_6 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:2"]
    pub struct C2RustUnnamed_7 {
        pub si_tid: libc::c_int,
        pub si_overrun: libc::c_int,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:2"]
    pub struct C2RustUnnamed_8 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
    }
    use super::types_h::{__uint32_t, __pid_t, __uid_t, __clock_t};
    use super::__sigval_t_h::__sigval_t;
}
#[c2rust::header_src = "/usr/include/signal.h:17"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    use super::sigset_t_h::sigset_t;
    use super::sigaction_h::sigaction;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "240:1"]
        pub fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                         __oact: *mut sigaction) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/sigaction.h:17"]
pub mod sigaction_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct sigaction {
        pub __sigaction_handler: C2RustUnnamed_9,
        pub sa_mask: __sigset_t,
        pub sa_flags: libc::c_int,
        pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:5"]
    pub union C2RustUnnamed_9 {
        pub sa_handler: __sighandler_t,
        pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut siginfo_t,
                                                      _: *mut libc::c_void)
                                     -> ()>,
    }
    use super::__sigset_t_h::__sigset_t;
    use super::signal_h::__sighandler_t;
    use super::siginfo_t_h::siginfo_t;
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:18"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:18"]
pub mod in_h {
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
    pub type C2RustUnnamed_10 = libc::c_uint;
    #[c2rust::src_loc = "92:5"]
    pub const IPPROTO_MAX: C2RustUnnamed_10 = 256;
    #[c2rust::src_loc = "90:5"]
    pub const IPPROTO_RAW: C2RustUnnamed_10 = 255;
    #[c2rust::src_loc = "88:5"]
    pub const IPPROTO_MPLS: C2RustUnnamed_10 = 137;
    #[c2rust::src_loc = "86:5"]
    pub const IPPROTO_UDPLITE: C2RustUnnamed_10 = 136;
    #[c2rust::src_loc = "84:5"]
    pub const IPPROTO_SCTP: C2RustUnnamed_10 = 132;
    #[c2rust::src_loc = "82:5"]
    pub const IPPROTO_COMP: C2RustUnnamed_10 = 108;
    #[c2rust::src_loc = "80:5"]
    pub const IPPROTO_PIM: C2RustUnnamed_10 = 103;
    #[c2rust::src_loc = "78:5"]
    pub const IPPROTO_ENCAP: C2RustUnnamed_10 = 98;
    #[c2rust::src_loc = "76:5"]
    pub const IPPROTO_BEETPH: C2RustUnnamed_10 = 94;
    #[c2rust::src_loc = "74:5"]
    pub const IPPROTO_MTP: C2RustUnnamed_10 = 92;
    #[c2rust::src_loc = "72:5"]
    pub const IPPROTO_AH: C2RustUnnamed_10 = 51;
    #[c2rust::src_loc = "70:5"]
    pub const IPPROTO_ESP: C2RustUnnamed_10 = 50;
    #[c2rust::src_loc = "68:5"]
    pub const IPPROTO_GRE: C2RustUnnamed_10 = 47;
    #[c2rust::src_loc = "66:5"]
    pub const IPPROTO_RSVP: C2RustUnnamed_10 = 46;
    #[c2rust::src_loc = "64:5"]
    pub const IPPROTO_IPV6: C2RustUnnamed_10 = 41;
    #[c2rust::src_loc = "62:5"]
    pub const IPPROTO_DCCP: C2RustUnnamed_10 = 33;
    #[c2rust::src_loc = "60:5"]
    pub const IPPROTO_TP: C2RustUnnamed_10 = 29;
    #[c2rust::src_loc = "58:5"]
    pub const IPPROTO_IDP: C2RustUnnamed_10 = 22;
    #[c2rust::src_loc = "56:5"]
    pub const IPPROTO_UDP: C2RustUnnamed_10 = 17;
    #[c2rust::src_loc = "54:5"]
    pub const IPPROTO_PUP: C2RustUnnamed_10 = 12;
    #[c2rust::src_loc = "52:5"]
    pub const IPPROTO_EGP: C2RustUnnamed_10 = 8;
    #[c2rust::src_loc = "50:5"]
    pub const IPPROTO_TCP: C2RustUnnamed_10 = 6;
    #[c2rust::src_loc = "48:5"]
    pub const IPPROTO_IPIP: C2RustUnnamed_10 = 4;
    #[c2rust::src_loc = "46:5"]
    pub const IPPROTO_IGMP: C2RustUnnamed_10 = 2;
    #[c2rust::src_loc = "44:5"]
    pub const IPPROTO_ICMP: C2RustUnnamed_10 = 1;
    #[c2rust::src_loc = "42:5"]
    pub const IPPROTO_IP: C2RustUnnamed_10 = 0;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::sockaddr_h::sa_family_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "376:1"]
        pub fn ntohs(__netshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:18"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:18"]
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
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:18"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:18"]
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
        pub ru: C2RustUnnamed_11,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed_11 {
        pub RM_cmb: call_body,
        pub RM_rmb: reply_body,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:8"]
    pub struct reply_body {
        pub rp_stat: reply_stat,
        pub ru: C2RustUnnamed_12,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_12 {
        pub RP_ar: accepted_reply,
        pub RP_dr: rejected_reply,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rejected_reply {
        pub rj_stat: reject_stat,
        pub ru: C2RustUnnamed_13,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:2"]
    pub union C2RustUnnamed_13 {
        pub RJ_versions: C2RustUnnamed_14,
        pub RJ_why: auth_stat,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:3"]
    pub struct C2RustUnnamed_14 {
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
        pub ru: C2RustUnnamed_15,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:2"]
    pub union C2RustUnnamed_15 {
        pub AR_versions: C2RustUnnamed_17,
        pub AR_results: C2RustUnnamed_16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:3"]
    pub struct C2RustUnnamed_16 {
        pub where_0: caddr_t,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:3"]
    pub struct C2RustUnnamed_17 {
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
    use super::xdr_h::xdrproc_t;
    /* !defined(GSSRPC_RPC_MSG_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:18"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "530:1"]
        pub fn gss_display_status(_: *mut OM_uint32, _: OM_uint32,
                                  _: libc::c_int, _: gss_OID,
                                  _: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc.h:18"]
pub mod svc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:8"]
    pub struct svc_req {
        pub rq_prog: rpcprog_t,
        pub rq_vers: rpcvers_t,
        pub rq_proc: rpcproc_t,
        pub rq_cred: opaque_auth,
        pub rq_clntcred: *mut libc::c_void,
        pub rq_svccred: *mut libc::c_void,
        pub rq_clntname: *mut libc::c_void,
        pub rq_xprt: *mut SVCXPRT,
    }
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
    use super::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t};
    use super::auth_h::opaque_auth;
    use super::sys_types_h::{u_short, u_int};
    use super::in_h::sockaddr_in;
    use super::svc_auth_h::SVCAUTH;
    use super::rpc_msg_h::rpc_msg;
    use super::xdr_h::xdrproc_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "331:1"]
        pub fn gssrpc_svctcp_create(_: libc::c_int, _: u_int, _: u_int)
         -> *mut SVCXPRT;
        #[no_mangle]
        #[c2rust::src_loc = "324:1"]
        pub fn gssrpc_svcudp_create(_: libc::c_int) -> *mut SVCXPRT;
        #[no_mangle]
        #[c2rust::src_loc = "305:1"]
        pub fn gssrpc_svc_run();
        #[no_mangle]
        #[c2rust::src_loc = "198:1"]
        pub fn gssrpc_svc_register(_: *mut SVCXPRT, _: rpcprog_t,
                                   _: rpcvers_t,
                                   _:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut svc_req,
                                                                   _:
                                                                       *mut SVCXPRT)
                                                  -> ()>, _: libc::c_int)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_SVC_H) */
    /* XXX add auth_gsapi_log_*? */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc_auth.h:18"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gssapi.h:23"]
pub mod auth_gssapi_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:16"]
    pub struct _auth_gssapi_name {
        pub name: *mut libc::c_char,
        pub type_0: gss_OID,
    }
    /* include/gssrpc/auth_gssapi.h - GSS-API style auth parameters for RPC */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved.
 */
    /*
 * Yuck.  Some sys/types.h files leak symbols
 */
    #[c2rust::src_loc = "27:1"]
    pub type auth_gssapi_name = _auth_gssapi_name;
    #[c2rust::src_loc = "51:1"]
    pub type auth_gssapi_log_badauth_func
        =
        Option<unsafe extern "C" fn(_: OM_uint32, _: OM_uint32,
                                    _: *mut sockaddr_in, _: caddr_t) -> ()>;
    #[c2rust::src_loc = "65:1"]
    pub type auth_gssapi_log_badverf_func
        =
        Option<unsafe extern "C" fn(_: gss_name_t, _: gss_name_t,
                                    _: *mut svc_req, _: *mut rpc_msg,
                                    _: caddr_t) -> ()>;
    #[c2rust::src_loc = "72:1"]
    pub type auth_gssapi_log_miscerr_func
        =
        Option<unsafe extern "C" fn(_: *mut svc_req, _: *mut rpc_msg,
                                    _: *mut libc::c_char, _: caddr_t) -> ()>;
    use super::gssapi_h::{gss_OID, OM_uint32, gss_name_t};
    use super::in_h::sockaddr_in;
    use super::sys_types_h::caddr_t;
    use super::svc_h::svc_req;
    use super::rpc_msg_h::rpc_msg;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "120:1"]
        pub fn gssrpc_svcauth_gssapi_set_names(names: *mut auth_gssapi_name,
                                               num: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "125:1"]
        pub fn gssrpc_svcauth_gssapi_set_log_badauth_func(func:
                                                              auth_gssapi_log_badauth_func,
                                                          data: caddr_t);
        #[no_mangle]
        #[c2rust::src_loc = "131:1"]
        pub fn gssrpc_svcauth_gssapi_set_log_badverf_func(func:
                                                              auth_gssapi_log_badverf_func,
                                                          data: caddr_t);
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn gssrpc_svcauth_gssapi_set_log_miscerr_func(func:
                                                              auth_gssapi_log_miscerr_func,
                                                          data: caddr_t);
    }
    /* !defined(GSSRPC_AUTH_GSSAPI_H) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:8"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:8"]
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
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:8"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "50:12"]
        pub static mut optind: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
                      __shortopts: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/pmap_clnt.h:19"]
pub mod pmap_clnt_h {
    use super::gssrpc_types_h::{rpcprog_t, rpcvers_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn gssrpc_pmap_unset(_: rpcprog_t, _: rpcvers_t) -> libc::c_int;
    }
    /* !defined(GSSRPC_PMAP_CLNT_H) */
}
#[c2rust::header_src = "/usr/include/arpa/inet.h:20"]
pub mod inet_h {
    use super::in_h::{in_addr, in_addr_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_generic.h:22"]
pub mod gssapi_generic_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "54:27"]
        pub static mut gss_nt_service_name: gss_OID;
    }
    /* _GSSAPI_GENERIC_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/rpc/unit-test/rpc_test.h:25"]
pub mod rpc_test_h {
    use super::svc_h::{svc_req, SVCXPRT};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "11:1"]
        pub fn rpc_test_prog_1_svc(_: *mut svc_req, _: *mut SVCXPRT);
    }
    /* !_RPC_TEST_H_RPCGEN */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __u_int, __uint16_t, __int32_t, __uint32_t,
                        __uid_t, __off_t, __off64_t, __pid_t, __clock_t,
                        __caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::sys_types_h::{u_short, u_int, caddr_t};
pub use self::__sigset_t_h::__sigset_t;
pub use self::sigset_t_h::sigset_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::__sigval_t_h::{__sigval_t, sigval};
pub use self::siginfo_t_h::{siginfo_t, C2RustUnnamed, C2RustUnnamed_0,
                            C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
                            C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6,
                            C2RustUnnamed_7, C2RustUnnamed_8};
pub use self::signal_h::{__sighandler_t, sigemptyset, sigaction};
pub use self::sigaction_h::{sigaction, C2RustUnnamed_9};
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{in_port_t, sockaddr_in, in_addr, in_addr_t,
                     C2RustUnnamed_10, IPPROTO_MAX, IPPROTO_RAW, IPPROTO_MPLS,
                     IPPROTO_UDPLITE, IPPROTO_SCTP, IPPROTO_COMP, IPPROTO_PIM,
                     IPPROTO_ENCAP, IPPROTO_BEETPH, IPPROTO_MTP, IPPROTO_AH,
                     IPPROTO_ESP, IPPROTO_GRE, IPPROTO_RSVP, IPPROTO_IPV6,
                     IPPROTO_DCCP, IPPROTO_TP, IPPROTO_IDP, IPPROTO_UDP,
                     IPPROTO_PUP, IPPROTO_EGP, IPPROTO_TCP, IPPROTO_IPIP,
                     IPPROTO_IGMP, IPPROTO_ICMP, IPPROTO_IP, ntohs};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, opaque_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed_11, reply_body,
                          C2RustUnnamed_12, rejected_reply, C2RustUnnamed_13,
                          C2RustUnnamed_14, reject_stat, AUTH_ERROR,
                          RPC_MISMATCH, accepted_reply, C2RustUnnamed_15,
                          C2RustUnnamed_16, C2RustUnnamed_17, accept_stat,
                          SYSTEM_ERR, GARBAGE_ARGS, PROC_UNAVAIL,
                          PROG_MISMATCH, PROG_UNAVAIL, SUCCESS, reply_stat,
                          MSG_DENIED, MSG_ACCEPTED, call_body, msg_type,
                          REPLY, CALL};
pub use self::gssapi_h::{gss_name_t, gss_uint32, OM_uint32,
                         gss_OID_desc_struct, gss_OID, gss_buffer_desc_struct,
                         gss_buffer_desc, gss_buffer_t, gss_name_struct,
                         gss_release_buffer, gss_display_name,
                         gss_display_status};
pub use self::svc_h::{svc_req, SVCXPRT, xp_ops, xprt_stat, XPRT_IDLE,
                      XPRT_MOREREQS, XPRT_DIED, gssrpc_svctcp_create,
                      gssrpc_svcudp_create, gssrpc_svc_run,
                      gssrpc_svc_register};
pub use self::svc_auth_h::{SVCAUTH, svc_auth_ops};
pub use self::auth_gssapi_h::{_auth_gssapi_name, auth_gssapi_name,
                              auth_gssapi_log_badauth_func,
                              auth_gssapi_log_badverf_func,
                              auth_gssapi_log_miscerr_func,
                              gssrpc_svcauth_gssapi_set_names,
                              gssrpc_svcauth_gssapi_set_log_badauth_func,
                              gssrpc_svcauth_gssapi_set_log_badverf_func,
                              gssrpc_svcauth_gssapi_set_log_miscerr_func};
use self::stdlib_h::{atoi, free, exit};
use self::stdio_h::{stderr, fprintf, printf, asprintf};
use self::getopt_core_h::{optind, getopt};
use self::pmap_clnt_h::gssrpc_pmap_unset;
use self::inet_h::inet_ntoa;
use self::gssapi_generic_h::gss_nt_service_name;
use self::rpc_test_h::rpc_test_prog_1_svc;
extern "C" {
    /*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved.
 *
 * $Id$
 * $Source$
 */
    /* inet_ntoa */
    /* MAXHOSTNAMELEN */
    #[no_mangle]
    #[c2rust::src_loc = "27:12"]
    pub static mut gssrpc_svc_debug_gssapi: libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "27:30"]
    pub static mut gssrpc_misc_debug_gssapi: libc::c_int;
}
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            b"Usage: server {-t|-u} [svc-debug] [misc-debug]\n\x00" as
                *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "50:1"]
unsafe extern "C" fn handlesig(mut dummy: libc::c_int) {
    exit(0 as libc::c_int);
}
#[c2rust::src_loc = "58:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut prot: libc::c_int = 0;
    let mut names: [auth_gssapi_name; 2] =
        [auth_gssapi_name{name: 0 as *mut libc::c_char,
                          type_0: 0 as *mut gss_OID_desc_struct,}; 2];
    let mut transp: *mut SVCXPRT = 0 as *mut SVCXPRT;
    extern "C" {
        #[link_name = "optind"]
        pub static mut optind_0: libc::c_int;
    }
    let mut sa: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_9{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    names[0 as libc::c_int as usize].name =
        b"server\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    names[0 as libc::c_int as usize].type_0 = gss_nt_service_name;
    names[1 as libc::c_int as usize].name = 0 as *mut libc::c_char;
    names[1 as libc::c_int as usize].type_0 = 0 as gss_OID;
    prot = 0 as libc::c_int;
    loop  {
        c = getopt(argc, argv, b"tu\x00" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) { break ; }
        match c {
            116 => { prot = IPPROTO_TCP as libc::c_int }
            117 => { prot = IPPROTO_UDP as libc::c_int }
            63 => { usage(); }
            _ => { }
        }
    }
    if prot == 0 as libc::c_int { usage(); }
    argv = argv.offset(optind as isize);
    argc -= optind;
    let mut current_block_18: u64;
    match argc {
        2 => {
            gssrpc_misc_debug_gssapi =
                atoi(*argv.offset(1 as libc::c_int as isize));
            current_block_18 = 9400601522379909684;
        }
        1 => { current_block_18 = 9400601522379909684; }
        0 => { current_block_18 = 2719512138335094285; }
        _ => { usage(); exit(1 as libc::c_int); }
    }
    match current_block_18 {
        9400601522379909684 => {
            gssrpc_svc_debug_gssapi =
                atoi(*argv.offset(0 as libc::c_int as isize))
        }
        _ => { }
    }
    gssrpc_pmap_unset(1000001 as libc::c_int as libc::c_ulong as rpcprog_t,
                      1 as libc::c_int as libc::c_ulong as rpcvers_t);
    if prot == IPPROTO_TCP as libc::c_int {
        transp =
            gssrpc_svctcp_create(-(1 as libc::c_int),
                                 0 as libc::c_int as u_int,
                                 0 as libc::c_int as u_int)
    } else { transp = gssrpc_svcudp_create(-(1 as libc::c_int)) }
    if transp.is_null() {
        fprintf(stderr,
                b"cannot create tcp service.\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if gssrpc_svc_register(transp,
                           1000001 as libc::c_int as libc::c_ulong as
                               rpcprog_t,
                           1 as libc::c_int as libc::c_ulong as rpcvers_t,
                           Some(rpc_test_prog_1_svc as
                                    unsafe extern "C" fn(_: *mut svc_req,
                                                         _: *mut SVCXPRT)
                                        -> ()), 0 as libc::c_int) == 0 {
        fprintf(stderr,
                b"unable to register (RPC_TEST_PROG, RPC_TEST_VERS_1, %s).\x00"
                    as *const u8 as *const libc::c_char,
                if prot == IPPROTO_TCP as libc::c_int {
                    b"tcp\x00" as *const u8 as *const libc::c_char
                } else { b"udp\x00" as *const u8 as *const libc::c_char });
        exit(1 as libc::c_int);
    }
    printf(b"port: %d\n\x00" as *const u8 as *const libc::c_char,
           (*transp).xp_port as libc::c_int);
    if gssrpc_svcauth_gssapi_set_names(names.as_mut_ptr(), 0 as libc::c_int)
           == 0 as libc::c_int {
        fprintf(stderr,
                b"unable to set gssapi names\n\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    gssrpc_svcauth_gssapi_set_log_badauth_func(Some(rpc_test_badauth as
                                                        unsafe extern "C" fn(_:
                                                                                 OM_uint32,
                                                                             _:
                                                                                 OM_uint32,
                                                                             _:
                                                                                 *mut sockaddr_in,
                                                                             _:
                                                                                 caddr_t)
                                                            -> ()),
                                               0 as caddr_t);
    gssrpc_svcauth_gssapi_set_log_badverf_func(Some(rpc_test_badverf as
                                                        unsafe extern "C" fn(_:
                                                                                 gss_name_t,
                                                                             _:
                                                                                 gss_name_t,
                                                                             _:
                                                                                 *mut svc_req,
                                                                             _:
                                                                                 *mut rpc_msg,
                                                                             _:
                                                                                 caddr_t)
                                                            -> ()),
                                               0 as caddr_t);
    gssrpc_svcauth_gssapi_set_log_miscerr_func(Some(log_miscerr as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut svc_req,
                                                                             _:
                                                                                 *mut rpc_msg,
                                                                             _:
                                                                                 *mut libc::c_char,
                                                                             _:
                                                                                 *mut libc::c_char)
                                                            -> ()),
                                               0 as caddr_t);
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = 0 as libc::c_int;
    sa.__sigaction_handler.sa_handler =
        Some(handlesig as unsafe extern "C" fn(_: libc::c_int) -> ());
    sigaction(1 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction);
    sigaction(15 as libc::c_int, &mut sa, 0 as *mut sigaction);
    printf(b"running\n\x00" as *const u8 as *const libc::c_char);
    gssrpc_svc_run();
    fprintf(stderr,
            b"svc_run returned\x00" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
    /* NOTREACHED */
}
#[no_mangle]
#[c2rust::src_loc = "154:1"]
pub unsafe extern "C" fn rpc_test_echo_1_svc(mut arg: *mut *mut libc::c_char,
                                             mut h: *mut svc_req)
 -> *mut *mut libc::c_char {
    static mut res: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    if !res.is_null() { free(res as *mut libc::c_void); }
    asprintf(&mut res as *mut *mut libc::c_char,
             b"Echo: %s\x00" as *const u8 as *const libc::c_char, *arg);
    return &mut res;
}
#[c2rust::src_loc = "164:1"]
unsafe extern "C" fn rpc_test_badverf(mut client: gss_name_t,
                                      mut server: gss_name_t,
                                      mut rqst: *mut svc_req,
                                      mut msg: *mut rpc_msg,
                                      mut data: caddr_t) {
    let mut minor_stat: OM_uint32 = 0;
    let mut type_0: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut client_name: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut server_name: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    gss_display_name(&mut minor_stat, client, &mut client_name, &mut type_0);
    gss_display_name(&mut minor_stat, server, &mut server_name, &mut type_0);
    printf(b"rpc_test server: bad verifier from %.*s at %s:%d for %.*s\n\x00"
               as *const u8 as *const libc::c_char,
           client_name.length as libc::c_int,
           client_name.value as *mut libc::c_char,
           inet_ntoa((*(*rqst).rq_xprt).xp_raddr.sin_addr),
           ntohs((*(*rqst).rq_xprt).xp_raddr.sin_port) as libc::c_int,
           server_name.length as libc::c_int,
           server_name.value as *mut libc::c_char);
    gss_release_buffer(&mut minor_stat, &mut client_name);
    gss_release_buffer(&mut minor_stat, &mut server_name);
}
/*
 * Function: log_badauth
 *
 * Purpose: Callback from GSS-API Sun RPC for authentication
 * failures/errors.
 *
 * Arguments:
 * 	major 		(r) GSS-API major status
 * 	minor		(r) GSS-API minor status
 * 	addr		(r) originating address
 * 	data		(r) arbitrary data (NULL), not used
 *
 * Effects:
 *
 * Logs the GSS-API error to stdout.
 */
#[no_mangle]
#[c2rust::src_loc = "201:1"]
pub unsafe extern "C" fn rpc_test_badauth(mut major: OM_uint32,
                                          mut minor: OM_uint32,
                                          mut addr: *mut sockaddr_in,
                                          mut data: caddr_t) {
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Authentication attempt failed: <IP address>, <GSS-API error */
     /* strings> */
    a = inet_ntoa((*addr).sin_addr);
    printf(b"rpc_test server: Authentication attempt failed: %s\x00" as
               *const u8 as *const libc::c_char, a);
    log_badauth_display_status(major, minor);
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "216:1"]
pub unsafe extern "C" fn log_miscerr(mut rqst: *mut svc_req,
                                     mut msg: *mut rpc_msg,
                                     mut error: *mut libc::c_char,
                                     mut data: *mut libc::c_char) {
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    a = inet_ntoa((*(*rqst).rq_xprt).xp_raddr.sin_addr);
    printf(b"Miscellaneous RPC error: %s, %s\n\x00" as *const u8 as
               *const libc::c_char, a, error);
}
#[no_mangle]
#[c2rust::src_loc = "225:1"]
pub unsafe extern "C" fn log_badauth_display_status(mut major: OM_uint32,
                                                    mut minor: OM_uint32) {
    log_badauth_display_status_1(major, 1 as libc::c_int, 0 as libc::c_int);
    log_badauth_display_status_1(minor, 2 as libc::c_int, 0 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "231:1"]
pub unsafe extern "C" fn log_badauth_display_status_1(mut code: OM_uint32,
                                                      mut type_0: libc::c_int,
                                                      mut rec: libc::c_int) {
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut msg_ctx: OM_uint32 = 0;
    let mut msg: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    msg_ctx = 0 as libc::c_int as OM_uint32;
    loop  {
        gssstat =
            gss_display_status(&mut minor_stat, code, type_0, 0 as gss_OID,
                               &mut msg_ctx, &mut msg);
        if gssstat != 0 as libc::c_int as libc::c_uint {
            if rec == 0 {
                log_badauth_display_status_1(gssstat, 1 as libc::c_int,
                                             1 as libc::c_int);
                log_badauth_display_status_1(minor_stat, 2 as libc::c_int,
                                             1 as libc::c_int);
            } else {
                printf(b"GSS-API authentication error %.*s: recursive failure!\n\x00"
                           as *const u8 as *const libc::c_char,
                       msg.length as libc::c_int,
                       msg.value as *mut libc::c_char);
            }
            return
        }
        printf(b", %.*s\x00" as *const u8 as *const libc::c_char,
               msg.length as libc::c_int, msg.value as *mut libc::c_char);
        gss_release_buffer(&mut minor_stat, &mut msg);
        if msg_ctx == 0 { break ; }
    };
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
