use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:10"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:10"]
pub mod types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __u_short = libc::c_ushort;
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
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
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:10"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:10"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/sys/types.h:10"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_short, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:10"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:10"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/signal.h:11"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn signal(__sig: libc::c_int, __handler: __sighandler_t)
         -> __sighandler_t;
    }
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:16"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:16"]
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
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:16"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:16"]
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
        #[c2rust::src_loc = "298:1"]
        pub fn gssrpc_xdr_u_int32(_: *mut XDR, _: *mut uint32_t)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:16"]
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
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:16"]
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
        pub ru: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed {
        pub RM_cmb: call_body,
        pub RM_rmb: reply_body,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:8"]
    pub struct reply_body {
        pub rp_stat: reply_stat,
        pub ru: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_0 {
        pub RP_ar: accepted_reply,
        pub RP_dr: rejected_reply,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rejected_reply {
        pub rj_stat: reject_stat,
        pub ru: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:2"]
    pub union C2RustUnnamed_1 {
        pub RJ_versions: C2RustUnnamed_2,
        pub RJ_why: auth_stat,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:3"]
    pub struct C2RustUnnamed_2 {
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
        pub ru: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:2"]
    pub union C2RustUnnamed_3 {
        pub AR_versions: C2RustUnnamed_5,
        pub AR_results: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:3"]
    pub struct C2RustUnnamed_4 {
        pub where_0: caddr_t,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:3"]
    pub struct C2RustUnnamed_5 {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:16"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
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
        /* This is the gssapi.h prologue. */
/* no xom.h */
/* End of gssapi.h prologue. */
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
        /*
 * Determine platform-dependent configuration.
 */
        /* __cplusplus */
        /*
 * First, include stddef.h to get size_t defined.
 */
        /*
 * POSIX says that sys/types.h is where size_t is defined.
 */
        /*
 * $Id$
 */
        /*
 * First, define the three platform-dependent pointer types.
 */
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* output_name */
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* mechanisms */
        /* Last argument new for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "594:1"]
        pub fn gss_inquire_context(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: *mut gss_name_t, _: *mut gss_name_t,
                                   _: *mut OM_uint32, _: *mut gss_OID,
                                   _: *mut OM_uint32, _: *mut libc::c_int,
                                   _: *mut libc::c_int) -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc.h:16"]
pub mod svc_h {
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
    use super::sys_types_h::{u_short, caddr_t};
    use super::in_h::sockaddr_in;
    use super::svc_auth_h::SVCAUTH;
    use super::rpc_msg_h::rpc_msg;
    use super::xdr_h::xdrproc_t;
    extern "C" {
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
        #[no_mangle]
        #[c2rust::src_loc = "258:1"]
        pub fn gssrpc_svc_sendreply(_: *mut SVCXPRT, _: xdrproc_t, _: caddr_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "259:1"]
        pub fn gssrpc_svcerr_decode(_: *mut SVCXPRT);
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn gssrpc_svcerr_weakauth(_: *mut SVCXPRT);
        #[no_mangle]
        #[c2rust::src_loc = "261:1"]
        pub fn gssrpc_svcerr_noproc(_: *mut SVCXPRT);
        #[no_mangle]
        #[c2rust::src_loc = "265:1"]
        pub fn gssrpc_svcerr_systemerr(_: *mut SVCXPRT);
    }
    /* !defined(GSSRPC_SVC_H) */
    /* XXX add auth_gsapi_log_*? */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc_auth.h:16"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:16"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
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
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    extern "C" {
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:16"]
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
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:16"]
pub mod kdb_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1990, 1991, 2016 by the Massachusetts Institute of Technology.
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
/*
 * Copyright 2009 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* KDC Database interface definitions */
    /* This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
    /* This version will be incremented when incompatible changes are made to the
 * KDB API, and will be kept in sync with the libkdb major version. */
    /* Salt types */
    /* #define KRB5_KDB_SALTTYPE_V4            1 */
    /* #define KRB5_KDB_SALTTYPE_AFS3          5 */
    /* Attributes */
    /* S4U2Self OK */
    /* Creation flags */
    /* Entry get flags */
/* Name canonicalization requested */
    /* Include authorization data generated by backend */
    /* Is AS-REQ (client referrals only) */
    /* Map cross-realm principals */
    /* Protocol transition */
    /* Constrained delegation */
    /* User-to-user */
    /* Cross-realm */
    /* Issuing referral */
    /* KDB iteration flags */
    /* String attribute names recognized by krb5 */
    /*
 * Note --- these structures cannot be modified without changing the
 * database version number in libkdb.a, but should be expandable by
 * adding new tl_data types.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_enctype,
                        krb5_int32};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:16"]
pub mod admin_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _kadm5_policy_ent_t {
        pub policy: *mut libc::c_char,
        pub pw_min_life: libc::c_long,
        pub pw_max_life: libc::c_long,
        pub pw_min_length: libc::c_long,
        pub pw_min_classes: libc::c_long,
        pub pw_history_num: libc::c_long,
        pub policy_refcnt: libc::c_long,
        pub pw_max_fail: krb5_kvno,
        pub pw_failcnt_interval: krb5_deltat,
        pub pw_lockout_duration: krb5_deltat,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    #[c2rust::src_loc = "215:1"]
    pub type kadm5_policy_ent_rec = _kadm5_policy_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _kadm5_config_params {
        pub mask: libc::c_long,
        pub realm: *mut libc::c_char,
        pub kadmind_port: libc::c_int,
        pub kpasswd_port: libc::c_int,
        pub admin_server: *mut libc::c_char,
        pub dbname: *mut libc::c_char,
        pub acl_file: *mut libc::c_char,
        pub dict_file: *mut libc::c_char,
        pub mkey_from_kbd: libc::c_int,
        pub stash_file: *mut libc::c_char,
        pub mkey_name: *mut libc::c_char,
        pub enctype: krb5_enctype,
        pub max_life: krb5_deltat,
        pub max_rlife: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub flags: krb5_flags,
        pub keysalts: *mut krb5_key_salt_tuple,
        pub num_keysalts: krb5_int32,
        pub kvno: krb5_kvno,
        pub iprop_enabled: libc::c_int,
        pub iprop_ulogsize: uint32_t,
        pub iprop_poll_time: krb5_deltat,
        pub iprop_logfile: *mut libc::c_char,
        pub iprop_port: libc::c_int,
        pub iprop_resync_timeout: libc::c_int,
        pub kadmind_listen: *mut libc::c_char,
        pub kpasswd_listen: *mut libc::c_char,
        pub iprop_listen: *mut libc::c_char,
    }
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    use super::krb5_h::{krb5_kvno, krb5_deltat, krb5_flags, krb5_int16,
                        krb5_enctype, krb5_timestamp, krb5_int32};
    use super::kdb_h::{krb5_tl_data, krb5_key_salt_tuple};
    use super::stdint_uintn_h::uint32_t;
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/server_internal.h:18"]
pub mod server_internal_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
    /*
 * This header file is used internally by the Admin API server
 * libraries and Admin server.  IF YOU THINK YOU NEED TO USE THIS FILE
 * FOR ANYTHING, YOU'RE ALMOST CERTAINLY WRONG.
 */
    /*
 * This is the history key version for a newly created DB.  We use this value
 * for principals which have no password history yet to avoid having to look up
 * the history key.  Values other than 2 will cause compatibility issues with
 * pre-1.8 libkadm5 code; the older code will reject key changes when it sees
 * an unexpected value of admin_history_kvno.
 */
    /* A pwqual_handle represents a password quality plugin module. */
    #[c2rust::src_loc = "38:1"]
    pub type pwqual_handle = *mut pwqual_handle_st;
    #[c2rust::src_loc = "40:1"]
    pub type kadm5_hook_handle = *mut kadm5_hook_handle_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:16"]
    pub struct _kadm5_server_handle_t {
        pub magic_number: krb5_ui_4,
        pub struct_version: krb5_ui_4,
        pub api_version: krb5_ui_4,
        pub context: krb5_context,
        pub current_caller: krb5_principal,
        pub params: kadm5_config_params,
        pub lhandle: *mut _kadm5_server_handle_t,
        pub db_args: *mut *mut libc::c_char,
        pub qual_handles: *mut pwqual_handle,
        pub hook_handles: *mut kadm5_hook_handle,
    }
    #[c2rust::src_loc = "42:1"]
    pub type kadm5_server_handle_t = *mut _kadm5_server_handle_t;
    use super::krb5_h::{krb5_ui_4, krb5_context, krb5_principal};
    use super::admin_h::kadm5_config_params;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type pwqual_handle_st;
        #[c2rust::src_loc = "40:16"]
        pub type kadm5_hook_handle_st;
    }
    /* __KADM5_SERVER_INTERNAL_H__ */
    /* * @}*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:26"]
pub mod iprop_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:9"]
    pub struct utf8str_t {
        pub utf8str_t_len: u_int,
        pub utf8str_t_val: *mut libc::c_char,
    }
    #[c2rust::src_loc = "22:1"]
    pub type kdb_sno_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:8"]
    pub struct kdbe_time_t {
        pub seconds: uint32_t,
        pub useconds: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct kdbe_key_t {
        pub k_ver: int32_t,
        pub k_kvno: int32_t,
        pub k_enctype: C2RustUnnamed_7,
        pub k_contents: C2RustUnnamed_6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:2"]
    pub struct C2RustUnnamed_6 {
        pub k_contents_len: u_int,
        pub k_contents_val: *mut utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:2"]
    pub struct C2RustUnnamed_7 {
        pub k_enctype_len: u_int,
        pub k_enctype_val: *mut int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:8"]
    pub struct kdbe_data_t {
        pub k_magic: int32_t,
        pub k_data: utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct kdbe_princ_t {
        pub k_realm: utf8str_t,
        pub k_components: C2RustUnnamed_8,
        pub k_nametype: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub struct C2RustUnnamed_8 {
        pub k_components_len: u_int,
        pub k_components_val: *mut kdbe_data_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct kdbe_tl_t {
        pub tl_type: int16_t,
        pub tl_data: C2RustUnnamed_9,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:2"]
    pub struct C2RustUnnamed_9 {
        pub tl_data_len: u_int,
        pub tl_data_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "69:9"]
    pub struct kdbe_pw_hist_t {
        pub kdbe_pw_hist_t_len: u_int,
        pub kdbe_pw_hist_t_val: *mut kdbe_key_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type kdbe_attr_type_t = libc::c_uint;
    #[c2rust::src_loc = "94:2"]
    pub const AT_PW_HIST: kdbe_attr_type_t = 19;
    #[c2rust::src_loc = "93:2"]
    pub const AT_PW_HIST_KVNO: kdbe_attr_type_t = 18;
    #[c2rust::src_loc = "92:2"]
    pub const AT_PW_POLICY_SWITCH: kdbe_attr_type_t = 17;
    #[c2rust::src_loc = "91:2"]
    pub const AT_PW_POLICY: kdbe_attr_type_t = 16;
    #[c2rust::src_loc = "90:2"]
    pub const AT_PW_LAST_CHANGE: kdbe_attr_type_t = 15;
    #[c2rust::src_loc = "89:2"]
    pub const AT_MOD_WHERE: kdbe_attr_type_t = 14;
    #[c2rust::src_loc = "88:2"]
    pub const AT_MOD_TIME: kdbe_attr_type_t = 13;
    #[c2rust::src_loc = "87:2"]
    pub const AT_MOD_PRINC: kdbe_attr_type_t = 12;
    #[c2rust::src_loc = "86:2"]
    pub const AT_LEN: kdbe_attr_type_t = 11;
    #[c2rust::src_loc = "85:2"]
    pub const AT_TL_DATA: kdbe_attr_type_t = 10;
    #[c2rust::src_loc = "84:2"]
    pub const AT_KEYDATA: kdbe_attr_type_t = 9;
    #[c2rust::src_loc = "83:2"]
    pub const AT_PRINC: kdbe_attr_type_t = 8;
    #[c2rust::src_loc = "82:2"]
    pub const AT_FAIL_AUTH_COUNT: kdbe_attr_type_t = 7;
    #[c2rust::src_loc = "81:2"]
    pub const AT_LAST_FAILED: kdbe_attr_type_t = 6;
    #[c2rust::src_loc = "80:2"]
    pub const AT_LAST_SUCCESS: kdbe_attr_type_t = 5;
    #[c2rust::src_loc = "79:2"]
    pub const AT_PW_EXP: kdbe_attr_type_t = 4;
    #[c2rust::src_loc = "78:2"]
    pub const AT_EXP: kdbe_attr_type_t = 3;
    #[c2rust::src_loc = "77:2"]
    pub const AT_MAX_RENEW_LIFE: kdbe_attr_type_t = 2;
    #[c2rust::src_loc = "76:2"]
    pub const AT_MAX_LIFE: kdbe_attr_type_t = 1;
    #[c2rust::src_loc = "75:2"]
    pub const AT_ATTRFLAGS: kdbe_attr_type_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:8"]
    pub struct kdbe_val_t {
        pub av_type: kdbe_attr_type_t,
        pub kdbe_val_t_u: C2RustUnnamed_10,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:2"]
    pub union C2RustUnnamed_10 {
        pub av_attrflags: uint32_t,
        pub av_max_life: uint32_t,
        pub av_max_renew_life: uint32_t,
        pub av_exp: uint32_t,
        pub av_pw_exp: uint32_t,
        pub av_last_success: uint32_t,
        pub av_last_failed: uint32_t,
        pub av_fail_auth_count: uint32_t,
        pub av_princ: kdbe_princ_t,
        pub av_keydata: C2RustUnnamed_14,
        pub av_tldata: C2RustUnnamed_13,
        pub av_len: int16_t,
        pub av_pw_last_change: uint32_t,
        pub av_mod_princ: kdbe_princ_t,
        pub av_mod_time: uint32_t,
        pub av_mod_where: utf8str_t,
        pub av_pw_policy: utf8str_t,
        pub av_pw_policy_switch: libc::c_int,
        pub av_pw_hist_kvno: uint32_t,
        pub av_pw_hist: C2RustUnnamed_12,
        pub av_extension: C2RustUnnamed_11,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "130:3"]
    pub struct C2RustUnnamed_11 {
        pub av_extension_len: u_int,
        pub av_extension_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:3"]
    pub struct C2RustUnnamed_12 {
        pub av_pw_hist_len: u_int,
        pub av_pw_hist_val: *mut kdbe_pw_hist_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:3"]
    pub struct C2RustUnnamed_13 {
        pub av_tldata_len: u_int,
        pub av_tldata_val: *mut kdbe_tl_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:3"]
    pub struct C2RustUnnamed_14 {
        pub av_keydata_len: u_int,
        pub av_keydata_val: *mut kdbe_key_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:9"]
    pub struct kdbe_t {
        pub kdbe_t_len: u_int,
        pub kdbe_t_val: *mut kdbe_val_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "143:8"]
    pub struct kdb_incr_update_t {
        pub kdb_princ_name: utf8str_t,
        pub kdb_entry_sno: kdb_sno_t,
        pub kdb_time: kdbe_time_t,
        pub kdb_update: kdbe_t,
        pub kdb_deleted: libc::c_int,
        pub kdb_commit: libc::c_int,
        pub kdb_kdcs_seen_by: C2RustUnnamed_16,
        pub kdb_futures: C2RustUnnamed_15,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "154:2"]
    pub struct C2RustUnnamed_15 {
        pub kdb_futures_len: u_int,
        pub kdb_futures_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:2"]
    pub struct C2RustUnnamed_16 {
        pub kdb_kdcs_seen_by_len: u_int,
        pub kdb_kdcs_seen_by_val: *mut utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "161:9"]
    pub struct kdb_ulog_t {
        pub kdb_ulog_t_len: u_int,
        pub kdb_ulog_t_val: *mut kdb_incr_update_t,
    }
    #[c2rust::src_loc = "166:1"]
    pub type update_status_t = libc::c_uint;
    #[c2rust::src_loc = "172:2"]
    pub const UPDATE_PERM_DENIED: update_status_t = 5;
    #[c2rust::src_loc = "171:2"]
    pub const UPDATE_NIL: update_status_t = 4;
    #[c2rust::src_loc = "170:2"]
    pub const UPDATE_BUSY: update_status_t = 3;
    #[c2rust::src_loc = "169:2"]
    pub const UPDATE_FULL_RESYNC_NEEDED: update_status_t = 2;
    #[c2rust::src_loc = "168:2"]
    pub const UPDATE_ERROR: update_status_t = 1;
    #[c2rust::src_loc = "167:2"]
    pub const UPDATE_OK: update_status_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "176:8"]
    pub struct kdb_last_t {
        pub last_sno: kdb_sno_t,
        pub last_time: kdbe_time_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "182:8"]
    pub struct kdb_incr_result_t {
        pub lastentry: kdb_last_t,
        pub updates: kdb_ulog_t,
        pub ret: update_status_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "189:8"]
    pub struct kdb_fullresync_result_t {
        pub lastentry: kdb_last_t,
        pub ret: update_status_t,
    }
    use super::sys_types_h::u_int;
    use super::stdint_uintn_h::uint32_t;
    use super::stdint_intn_h::{int32_t, int16_t};
    use super::xdr_h::XDR;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "246:1"]
        pub fn xdr_kdb_last_t(_: *mut XDR, _: *mut kdb_last_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "247:1"]
        pub fn xdr_kdb_incr_result_t(_: *mut XDR, _: *mut kdb_incr_result_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "248:1"]
        pub fn xdr_kdb_fullresync_result_t(_: *mut XDR,
                                           _: *mut kdb_fullresync_result_t)
         -> libc::c_int;
    }
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
#[c2rust::header_src = "/usr/include/string.h:10"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:10"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:10"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
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
        #[no_mangle]
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "800:1"]
        pub fn popen(__command: *const libc::c_char,
                     __modes: *const libc::c_char) -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "806:1"]
        pub fn pclose(__stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:10"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:10"]
pub mod unistd_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "573:1"]
        pub fn execl(__path: *const libc::c_char, __arg: *const libc::c_char,
                     _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "603:13"]
        pub fn _exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "756:1"]
        pub fn fork() -> __pid_t;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:10"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:19"]
pub mod adm_proto_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn krb5_klog_syslog(_: libc::c_int, _: *const libc::c_char,
                                _: ...) -> libc::c_int;
    }
    /* KRB5_ADM_PROTO_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:26"]
pub mod kdb_log_h {
    use super::krb5_h::{_krb5_context, krb5_context, krb5_error_code};
    use super::iprop_h::{kdb_last_t, kdb_incr_result_t, kdb_incr_update_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "65:1"]
        pub fn ulog_get_entries(context: krb5_context,
                                last: *const kdb_last_t,
                                ulog_handle: *mut kdb_incr_result_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn ulog_free_entries(updates: *mut kdb_incr_update_t,
                                 no_of_updates: libc::c_int);
    }
    /* !_KDB_LOG_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/server/auth.h:27"]
pub mod server_auth_h {
    use super::krb5_h::{_krb5_context, krb5_context, krb5_principal_data,
                        krb5_const_principal, krb5_boolean};
    use super::admin_h::kadm5_policy_ent_rec;
    extern "C" {
        /* Authorize the operation given by opcode, using the appropriate subset of p1,
 * p2, s1, s2, polent, and mask. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn auth(context: krb5_context, opcode: libc::c_int,
                    client: krb5_const_principal, p1: krb5_const_principal,
                    p2: krb5_const_principal, s1: *const libc::c_char,
                    s2: *const libc::c_char,
                    polent: *const kadm5_policy_ent_rec, mask: libc::c_long)
         -> krb5_boolean;
    }
    /* AUTH_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/server/misc.h:28"]
pub mod misc_h {
    use super::svc_h::{svc_req, SVCXPRT};
    use super::krb5_h::{_krb5_context, krb5_context, krb5_principal};
    use super::gssapi_h::{gss_name_struct, gss_name_t, gss_buffer_desc_struct,
                          gss_buffer_t, gss_buffer_desc, OM_uint32};
    use super::stddef_h::size_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1994 OpenVision Technologies, Inc., All Rights Reserved
 *
 */
        #[no_mangle]
        #[c2rust::src_loc = "30:1"]
        pub fn gss_to_krb5_name_1(rqstp: *mut svc_req, ctx: krb5_context,
                                  gss_name: gss_name_t,
                                  princ: *mut krb5_principal,
                                  gss_str: gss_buffer_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "28:1"]
        pub fn trunc_name(len: *mut size_t, dots: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "12:1"]
        pub fn setup_gss_names(_: *mut svc_req, _: *mut gss_buffer_desc,
                               _: *mut gss_buffer_desc) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn log_badauth(major: OM_uint32, minor: OM_uint32,
                           xprt: *mut SVCXPRT, data: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn client_addr(xprt: *mut SVCXPRT) -> *const libc::c_char;
    }
    /* _MISC_H */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __u_int, __uint8_t, __int16_t, __uint16_t,
                        __int32_t, __uint32_t, __off_t, __off64_t, __pid_t,
                        __caddr_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::sys_types_h::{u_short, u_int, caddr_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::signal_h::{__sighandler_t, signal};
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{in_port_t, sockaddr_in, in_addr, in_addr_t};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdr_u_int32};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, opaque_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed, reply_body, C2RustUnnamed_0,
                          rejected_reply, C2RustUnnamed_1, C2RustUnnamed_2,
                          reject_stat, AUTH_ERROR, RPC_MISMATCH,
                          accepted_reply, C2RustUnnamed_3, C2RustUnnamed_4,
                          C2RustUnnamed_5, accept_stat, SYSTEM_ERR,
                          GARBAGE_ARGS, PROC_UNAVAIL, PROG_MISMATCH,
                          PROG_UNAVAIL, SUCCESS, reply_stat, MSG_DENIED,
                          MSG_ACCEPTED, call_body, msg_type, REPLY, CALL};
pub use self::gssapi_h::{gss_name_t, gss_ctx_id_t, gss_uint32, OM_uint32,
                         gss_OID_desc_struct, gss_OID, gss_buffer_desc_struct,
                         gss_buffer_desc, gss_buffer_t, gss_name_struct,
                         gss_ctx_id_struct, gss_release_name,
                         gss_release_buffer, gss_inquire_context};
pub use self::svc_h::{svc_req, SVCXPRT, xp_ops, xprt_stat, XPRT_IDLE,
                      XPRT_MOREREQS, XPRT_DIED, gssrpc_svc_sendreply,
                      gssrpc_svcerr_decode, gssrpc_svcerr_weakauth,
                      gssrpc_svcerr_noproc, gssrpc_svcerr_systemerr};
pub use self::svc_auth_h::{SVCAUTH, svc_auth_ops};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_context, _krb5_context,
                       krb5_parse_name, krb5_free_principal};
pub use self::com_err_h::{errcode_t, error_message};
pub use self::kdb_h::{_krb5_tl_data, krb5_tl_data, __krb5_key_salt_tuple,
                      krb5_key_salt_tuple};
pub use self::admin_h::{_kadm5_policy_ent_t, kadm5_policy_ent_rec,
                        _kadm5_config_params, kadm5_config_params};
pub use self::server_internal_h::{pwqual_handle, kadm5_hook_handle,
                                  _kadm5_server_handle_t,
                                  kadm5_server_handle_t, pwqual_handle_st,
                                  kadm5_hook_handle_st};
pub use self::iprop_h::{utf8str_t, kdb_sno_t, kdbe_time_t, kdbe_key_t,
                        C2RustUnnamed_6, C2RustUnnamed_7, kdbe_data_t,
                        kdbe_princ_t, C2RustUnnamed_8, kdbe_tl_t,
                        C2RustUnnamed_9, kdbe_pw_hist_t, kdbe_attr_type_t,
                        AT_PW_HIST, AT_PW_HIST_KVNO, AT_PW_POLICY_SWITCH,
                        AT_PW_POLICY, AT_PW_LAST_CHANGE, AT_MOD_WHERE,
                        AT_MOD_TIME, AT_MOD_PRINC, AT_LEN, AT_TL_DATA,
                        AT_KEYDATA, AT_PRINC, AT_FAIL_AUTH_COUNT,
                        AT_LAST_FAILED, AT_LAST_SUCCESS, AT_PW_EXP, AT_EXP,
                        AT_MAX_RENEW_LIFE, AT_MAX_LIFE, AT_ATTRFLAGS,
                        kdbe_val_t, C2RustUnnamed_10, C2RustUnnamed_11,
                        C2RustUnnamed_12, C2RustUnnamed_13, C2RustUnnamed_14,
                        kdbe_t, kdb_incr_update_t, C2RustUnnamed_15,
                        C2RustUnnamed_16, kdb_ulog_t, update_status_t,
                        UPDATE_PERM_DENIED, UPDATE_NIL, UPDATE_BUSY,
                        UPDATE_FULL_RESYNC_NEEDED, UPDATE_ERROR, UPDATE_OK,
                        kdb_last_t, kdb_incr_result_t,
                        kdb_fullresync_result_t, xdr_kdb_last_t,
                        xdr_kdb_incr_result_t, xdr_kdb_fullresync_result_t};
use self::string_h::{strncmp, strchr, memcpy, memset};
use self::stdlib_h::{malloc, free, exit};
use self::stdio_h::{printf, snprintf, asprintf, perror, popen, pclose};
use self::errno_h::__errno_location;
use self::unistd_h::{execl, _exit, fork};
use self::libintl_h::dgettext;
use self::adm_proto_h::krb5_klog_syslog;
use self::kdb_log_h::{ulog_get_entries, ulog_free_entries};
use self::server_auth_h::auth;
use self::misc_h::{gss_to_krb5_name_1, trunc_name, setup_gss_names,
                   log_badauth, client_addr};
extern "C" {
    #[no_mangle]
    #[c2rust::src_loc = "33:14"]
    pub static mut global_server_handle: *mut libc::c_void;
    #[no_mangle]
    #[c2rust::src_loc = "34:12"]
    pub static mut nofork: libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "36:14"]
    pub static mut kdb5_util: *mut libc::c_char;
    #[no_mangle]
    #[c2rust::src_loc = "37:14"]
    pub static mut kprop: *mut libc::c_char;
    #[no_mangle]
    #[c2rust::src_loc = "38:14"]
    pub static mut dump_file: *mut libc::c_char;
    #[no_mangle]
    #[c2rust::src_loc = "39:14"]
    pub static mut kprop_port: *mut libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "534:5"]
pub union C2RustUnnamed_17 {
    pub iprop_get_updates_1_arg: kdb_last_t,
}
#[c2rust::src_loc = "41:14"]
static mut reply_ok_str: *mut libc::c_char =
    b"UPDATE_OK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "42:14"]
static mut reply_err_str: *mut libc::c_char =
    b"UPDATE_ERROR\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[c2rust::src_loc = "43:14"]
static mut reply_fr_str: *mut libc::c_char =
    b"UPDATE_FULL_RESYNC_NEEDED\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[c2rust::src_loc = "44:14"]
static mut reply_busy_str: *mut libc::c_char =
    b"UPDATE_BUSY\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[c2rust::src_loc = "45:14"]
static mut reply_nil_str: *mut libc::c_char =
    b"UPDATE_NIL\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[c2rust::src_loc = "46:14"]
static mut reply_perm_str: *mut libc::c_char =
    b"UPDATE_PERM_DENIED\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[c2rust::src_loc = "47:14"]
static mut reply_unknown_str: *mut libc::c_char =
    b"<UNKNOWN_CODE>\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[c2rust::src_loc = "67:1"]
unsafe extern "C" fn debprret(mut w: *mut libc::c_char,
                              mut ret: update_status_t, mut sno: kdb_sno_t) {
    match ret as libc::c_uint {
        0 => {
            printf(b"%s: end (OK, sno=%u)\n\x00" as *const u8 as
                       *const libc::c_char, w, sno);
        }
        1 => {
            printf(b"%s: end (ERROR)\n\x00" as *const u8 as
                       *const libc::c_char, w);
        }
        2 => {
            printf(b"%s: end (FR NEEDED)\n\x00" as *const u8 as
                       *const libc::c_char, w);
        }
        3 => {
            printf(b"%s: end (BUSY)\n\x00" as *const u8 as
                       *const libc::c_char, w);
        }
        4 => {
            printf(b"%s: end (NIL)\n\x00" as *const u8 as *const libc::c_char,
                   w);
        }
        5 => {
            printf(b"%s: end (PERM)\n\x00" as *const u8 as
                       *const libc::c_char, w);
        }
        _ => {
            printf(b"%s: end (UNKNOWN return code (%d))\n\x00" as *const u8 as
                       *const libc::c_char, w, ret as libc::c_uint);
        }
    };
}
#[c2rust::src_loc = "95:1"]
unsafe extern "C" fn replystr(mut ret: update_status_t) -> *mut libc::c_char {
    match ret as libc::c_uint {
        0 => { return reply_ok_str }
        1 => { return reply_err_str }
        2 => { return reply_fr_str }
        3 => { return reply_busy_str }
        4 => { return reply_nil_str }
        5 => { return reply_perm_str }
        _ => { return reply_unknown_str }
    };
}
/* Returns null on allocation failure.
   Regardless of success or failure, frees the input buffer.  */
#[c2rust::src_loc = "118:1"]
unsafe extern "C" fn buf_to_string(mut b: *mut gss_buffer_desc)
 -> *mut libc::c_char {
    let mut min_stat: OM_uint32 = 0;
    let mut s: *mut libc::c_char =
        malloc((*b).length.wrapping_add(1 as libc::c_int as libc::c_ulong)) as
            *mut libc::c_char;
    if !s.is_null() {
        memcpy(s as *mut libc::c_void, (*b).value, (*b).length);
        *s.offset((*b).length as isize) = 0 as libc::c_int as libc::c_char
    }
    gss_release_buffer(&mut min_stat, b);
    return s;
}
#[c2rust::src_loc = "132:1"]
unsafe extern "C" fn iprop_acl_check(mut context: krb5_context,
                                     mut client_name: *const libc::c_char)
 -> krb5_boolean {
    let mut client_princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut result: krb5_boolean = 0;
    if krb5_parse_name(context, client_name, &mut client_princ) !=
           0 as libc::c_int {
        return 0 as libc::c_int as krb5_boolean
    }
    result =
        auth(context, 19 as libc::c_int, client_princ as krb5_const_principal,
             0 as krb5_const_principal, 0 as krb5_const_principal,
             0 as *const libc::c_char, 0 as *const libc::c_char,
             0 as *const kadm5_policy_ent_rec,
             0 as libc::c_int as libc::c_long);
    krb5_free_principal(context, client_princ);
    return result;
}
#[no_mangle]
#[c2rust::src_loc = "146:1"]
pub unsafe extern "C" fn iprop_get_updates_1_svc(mut arg: *mut kdb_last_t,
                                                 mut rqstp: *mut svc_req)
 -> *mut kdb_incr_result_t {
    static mut ret: kdb_incr_result_t =
        kdb_incr_result_t{lastentry:
                              kdb_last_t{last_sno: 0,
                                         last_time:
                                             kdbe_time_t{seconds: 0,
                                                         useconds: 0,},},
                          updates:
                              kdb_ulog_t{kdb_ulog_t_len: 0,
                                         kdb_ulog_t_val:
                                             0 as *const kdb_incr_update_t as
                                                 *mut kdb_incr_update_t,},
                          ret: UPDATE_OK,};
    let mut whoami: *mut libc::c_char =
        b"iprop_get_updates_1\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    let mut kret: libc::c_int = 0;
    let mut handle: kadm5_server_handle_t =
        global_server_handle as kadm5_server_handle_t;
    let mut client_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut service_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obuf: [libc::c_char; 256] =
        [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    /* default return code */
    ret.ret = UPDATE_ERROR;
    if handle.is_null() {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"%s: server handle is NULL\x00" as
                                      *const u8 as *const libc::c_char),
                         whoami);
    } else {
        let mut client_desc: gss_buffer_desc =
            gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
        let mut service_desc: gss_buffer_desc =
            gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
        if setup_gss_names(rqstp, &mut client_desc, &mut service_desc) <
               0 as libc::c_int {
            krb5_klog_syslog(3 as libc::c_int,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"%s: setup_gss_names failed\x00" as
                                          *const u8 as *const libc::c_char),
                             whoami);
        } else {
            client_name = buf_to_string(&mut client_desc);
            service_name = buf_to_string(&mut service_desc);
            if client_name.is_null() || service_name.is_null() {
                krb5_klog_syslog(3 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"%s: out of memory recording principal names\x00"
                                              as *const u8 as
                                              *const libc::c_char), whoami);
            } else if iprop_acl_check((*handle).context, client_name) == 0 {
                ret.ret = UPDATE_PERM_DENIED;
                krb5_klog_syslog(5 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Unauthorized request: %s, client=%s, service=%s, addr=%s\x00"
                                              as *const u8 as
                                              *const libc::c_char), whoami,
                                 client_name, service_name,
                                 client_addr((*rqstp).rq_xprt));
            } else {
                kret = ulog_get_entries((*handle).context, arg, &mut ret);
                if ret.ret as libc::c_uint ==
                       UPDATE_OK as libc::c_int as libc::c_uint {
                    snprintf(obuf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 256]>() as
                                 libc::c_ulong,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"%s; Incoming SerialNo=%lu; Outgoing SerialNo=%lu\x00"
                                          as *const u8 as
                                          *const libc::c_char),
                             replystr(ret.ret),
                             (*arg).last_sno as libc::c_ulong,
                             ret.lastentry.last_sno as libc::c_ulong);
                } else {
                    snprintf(obuf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 256]>() as
                                 libc::c_ulong,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"%s; Incoming SerialNo=%lu; Outgoing SerialNo=N/A\x00"
                                          as *const u8 as
                                          *const libc::c_char),
                             replystr(ret.ret),
                             (*arg).last_sno as libc::c_ulong);
                }
                krb5_klog_syslog(5 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Request: %s, %s, %s, client=%s, service=%s, addr=%s\x00"
                                              as *const u8 as
                                              *const libc::c_char), whoami,
                                 obuf.as_mut_ptr(),
                                 if kret == 0 as libc::c_int {
                                     b"success\x00" as *const u8 as
                                         *const libc::c_char
                                 } else { error_message(kret as errcode_t) },
                                 client_name, service_name,
                                 client_addr((*rqstp).rq_xprt));
            }
        }
    }
    if nofork != 0 { debprret(whoami, ret.ret, ret.lastentry.last_sno); }
    free(client_name as *mut libc::c_void);
    free(service_name as *mut libc::c_void);
    return &mut ret;
}
/*
 * Given a client princ (foo/fqdn@R), copy (in arg cl) the fqdn substring.
 * Return arg cl str ptr on success, else NULL.
 */
#[c2rust::src_loc = "244:1"]
unsafe extern "C" fn getclhoststr(mut clprinc: *const libc::c_char,
                                  mut cl: *mut libc::c_char, mut len: size_t)
 -> *mut libc::c_char {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    s = strchr(clprinc, '/' as i32);
    if s.is_null() ||
           { s = s.offset(1); e = strchr(s, '@' as i32); e.is_null() } ||
           e.wrapping_offset_from(s) as libc::c_long as size_t >= len {
        return 0 as *mut libc::c_char
    }
    memcpy(cl as *mut libc::c_void, s as *const libc::c_void,
           e.wrapping_offset_from(s) as libc::c_long as libc::c_ulong);
    *cl.offset(e.wrapping_offset_from(s) as libc::c_long as isize) =
        '\u{0}' as i32 as libc::c_char;
    return cl;
}
#[c2rust::src_loc = "257:1"]
unsafe extern "C" fn ipropx_resync(mut vers: uint32_t,
                                   mut rqstp: *mut svc_req)
 -> *mut kdb_fullresync_result_t {
    static mut ret: kdb_fullresync_result_t =
        kdb_fullresync_result_t{lastentry:
                                    kdb_last_t{last_sno: 0,
                                               last_time:
                                                   kdbe_time_t{seconds: 0,
                                                               useconds:
                                                                   0,},},
                                ret: UPDATE_OK,};
    let mut ubuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clhost: [libc::c_char; 1025] =
        [0 as libc::c_int as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut pret: libc::c_int = 0;
    let mut fret: libc::c_int = 0;
    let mut p: *mut FILE = 0 as *mut FILE;
    let mut handle: kadm5_server_handle_t =
        global_server_handle as kadm5_server_handle_t;
    let mut client_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut service_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut whoami: *mut libc::c_char =
        b"iprop_full_resync_1\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    /*
     * vers contains the highest version number the client is
     * willing to accept. A client can always accept a lower
     * version: the version number is indicated in the dump
     * header.
     */
    /* default return code */
    ret.ret = UPDATE_ERROR;
    if handle.is_null() {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"%s: server handle is NULL\x00" as
                                      *const u8 as *const libc::c_char),
                         whoami);
    } else {
        let mut client_desc: gss_buffer_desc =
            gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
        let mut service_desc: gss_buffer_desc =
            gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
        if setup_gss_names(rqstp, &mut client_desc, &mut service_desc) <
               0 as libc::c_int {
            krb5_klog_syslog(3 as libc::c_int,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"%s: setup_gss_names failed\x00" as
                                          *const u8 as *const libc::c_char),
                             whoami);
        } else {
            client_name = buf_to_string(&mut client_desc);
            service_name = buf_to_string(&mut service_desc);
            if client_name.is_null() || service_name.is_null() {
                krb5_klog_syslog(3 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"%s: out of memory recording principal names\x00"
                                              as *const u8 as
                                              *const libc::c_char), whoami);
            } else if iprop_acl_check((*handle).context, client_name) == 0 {
                ret.ret = UPDATE_PERM_DENIED;
                krb5_klog_syslog(5 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Unauthorized request: %s, client=%s, service=%s, addr=%s\x00"
                                              as *const u8 as
                                              *const libc::c_char), whoami,
                                 client_name, service_name,
                                 client_addr((*rqstp).rq_xprt));
            } else if getclhoststr(client_name, clhost.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 1025]>()
                                       as libc::c_ulong).is_null() {
                krb5_klog_syslog(3 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"%s: getclhoststr failed\x00" as
                                              *const u8 as
                                              *const libc::c_char), whoami);
            } else if asprintf(&mut ubuf as *mut *mut libc::c_char,
                               b"%s -r %s dump -i%d -c %s\x00" as *const u8 as
                                   *const libc::c_char, kdb5_util,
                               (*handle).params.realm, vers, dump_file) <
                          0 as libc::c_int {
                krb5_klog_syslog(3 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"%s: cannot construct kdb5 util dump string too long; out of memory\x00"
                                              as *const u8 as
                                              *const libc::c_char), whoami);
            } else {
                /*
     * Note the -i; modified version of kdb5_util dump format
     * to include sno (serial number). This argument is now
     * versioned (-i0 for legacy dump format, -i1 for ipropx
     * version 1 format, etc).
     *
     * The -c option ("conditional") causes the dump to dump only if no
     * dump already exists or that dump is not in ipropx format, or the
     * sno and timestamp in the header of that dump are outside the
     * ulog.  This allows us to share a single global dump with all
     * replicas, since it's OK to share an older dump, as long as its
     * sno and timestamp are in the ulog (then the replicas can get the
     * subsequent updates very iprop).
     */
                /*
     * Fork to dump the db and xfer it to the replica.
     * (the fork allows parent to return quickly and the child
     * acts like a callback to the replica).
     */
                fret = fork();
                match fret {
                    -1 => {
                        /* error */
                        if nofork != 0 { perror(whoami); }
                        krb5_klog_syslog(3 as libc::c_int,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"%s: fork failed: %s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char),
                                         whoami,
                                         error_message(*__errno_location() as
                                                           errcode_t));
                    }
                    0 => {
                        /* child */
                        signal(17 as libc::c_int, None);
                        /* run kdb5_util(1M) dump for IProp */
                        p =
                            popen(ubuf,
                                  b"w\x00" as *const u8 as
                                      *const libc::c_char);
                        if p.is_null() {
                            krb5_klog_syslog(3 as libc::c_int,
                                             dgettext(b"mit-krb5\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      b"%s: popen failed: %s\x00"
                                                          as *const u8 as
                                                          *const libc::c_char),
                                             whoami,
                                             error_message(*__errno_location()
                                                               as errcode_t));
                            _exit(1 as libc::c_int);
                        }
                        pret = pclose(p);
                        if pret != 0 as libc::c_int {
                            /* XXX popen/pclose may not set errno
	       properly, and the error could be from the
	       subprocess anyways.  */
                            if nofork != 0 { perror(whoami); }
                            krb5_klog_syslog(3 as libc::c_int,
                                             dgettext(b"mit-krb5\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      b"%s: pclose(popen) failed: %s\x00"
                                                          as *const u8 as
                                                          *const libc::c_char),
                                             whoami,
                                             error_message(*__errno_location()
                                                               as errcode_t));
                            _exit(1 as libc::c_int);
                        }
                        if !kprop_port.is_null() {
                            pret =
                                execl(kprop,
                                      b"kprop\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"-r\x00" as *const u8 as
                                          *const libc::c_char,
                                      (*handle).params.realm,
                                      b"-f\x00" as *const u8 as
                                          *const libc::c_char, dump_file,
                                      b"-P\x00" as *const u8 as
                                          *const libc::c_char, kprop_port,
                                      clhost.as_mut_ptr(),
                                      0 as *mut libc::c_void)
                        } else {
                            pret =
                                execl(kprop,
                                      b"kprop\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"-r\x00" as *const u8 as
                                          *const libc::c_char,
                                      (*handle).params.realm,
                                      b"-f\x00" as *const u8 as
                                          *const libc::c_char, dump_file,
                                      clhost.as_mut_ptr(),
                                      0 as *mut libc::c_void)
                        }
                        perror(whoami);
                        krb5_klog_syslog(3 as libc::c_int,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"%s: exec failed: %s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char),
                                         whoami,
                                         error_message(*__errno_location() as
                                                           errcode_t));
                        _exit(1 as libc::c_int);
                    }
                    _ => {
                        /* parent */
                        ret.ret = UPDATE_OK;
                        /* not used by replica (sno is retrieved from kdb5_util dump) */
                        ret.lastentry.last_sno =
                            0 as libc::c_int as kdb_sno_t;
                        ret.lastentry.last_time.seconds =
                            0 as libc::c_int as uint32_t;
                        ret.lastentry.last_time.useconds =
                            0 as libc::c_int as uint32_t;
                        krb5_klog_syslog(5 as libc::c_int,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"Request: %s, spawned resync process %d, client=%s, service=%s, addr=%s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char),
                                         whoami, fret, client_name,
                                         service_name,
                                         client_addr((*rqstp).rq_xprt));
                    }
                }
            }
        }
    }
    if nofork != 0 {
        debprret(whoami, ret.ret, 0 as libc::c_int as kdb_sno_t);
    }
    free(client_name as *mut libc::c_void);
    free(service_name as *mut libc::c_void);
    free(ubuf as *mut libc::c_void);
    return &mut ret;
}
#[no_mangle]
#[c2rust::src_loc = "445:1"]
pub unsafe extern "C" fn iprop_full_resync_1_svc(mut argp: *mut libc::c_void,
                                                 mut rqstp: *mut svc_req)
 -> *mut kdb_fullresync_result_t {
    return ipropx_resync(0 as libc::c_int as uint32_t, rqstp);
}
#[no_mangle]
#[c2rust::src_loc = "451:1"]
pub unsafe extern "C" fn iprop_full_resync_ext_1_svc(mut argp: *mut uint32_t,
                                                     mut rqstp: *mut svc_req)
 -> *mut kdb_fullresync_result_t {
    return ipropx_resync(*argp, rqstp);
}
#[c2rust::src_loc = "457:1"]
unsafe extern "C" fn check_iprop_rpcsec_auth(mut rqstp: *mut svc_req)
 -> libc::c_int {
    /* XXX Since the client can authenticate against any principal in
       the database, we need to do a sanity check.  Only checking for
       "kiprop" now, but that means theoretically the client could be
       authenticating to kiprop on some other machine.  */
    /* Code taken from kadm_rpc_svc.c, tweaked.  */
    let mut ctx: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut kctx: krb5_context = 0 as *mut _krb5_context;
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut name: gss_name_t = 0 as *mut gss_name_struct;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut ret: libc::c_int = 0;
    let mut success: libc::c_int = 0;
    let mut c1: *mut krb5_data = 0 as *mut krb5_data;
    let mut realm: *mut krb5_data = 0 as *mut krb5_data;
    let mut gss_str: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut slen: size_t = 0;
    let mut sdots: *mut libc::c_char = 0 as *mut libc::c_char;
    success = 0 as libc::c_int;
    handle = global_server_handle as kadm5_server_handle_t;
    if (*rqstp).rq_cred.oa_flavor != 6 as libc::c_int {
        return 0 as libc::c_int
    }
    ctx = (*rqstp).rq_svccred as gss_ctx_id_t;
    maj_stat =
        gss_inquire_context(&mut min_stat, ctx, 0 as *mut gss_name_t,
                            &mut name, 0 as *mut OM_uint32, 0 as *mut gss_OID,
                            0 as *mut OM_uint32, 0 as *mut libc::c_int,
                            0 as *mut libc::c_int);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"check_rpcsec_auth: failed inquire_context, stat=%u\x00"
                                      as *const u8 as *const libc::c_char),
                         maj_stat);
        log_badauth(maj_stat, min_stat, (*rqstp).rq_xprt,
                    0 as *mut libc::c_char);
    } else {
        kctx = (*handle).context;
        ret = gss_to_krb5_name_1(rqstp, kctx, name, &mut princ, &mut gss_str);
        if !(ret == 0 as libc::c_int) {
            slen = gss_str.length;
            trunc_name(&mut slen, &mut sdots);
            /*
      * Since we accept with GSS_C_NO_NAME, the client can authenticate
      * against the entire kdb.  Therefore, ensure that the service
      * name is something reasonable.
      */
            if !((*princ).length != 2 as libc::c_int) {
                c1 =
                    if (0 as libc::c_int) < (*princ).length {
                        (*princ).data.offset(0 as libc::c_int as isize)
                    } else { 0 as *mut krb5_data };
                realm = &mut (*princ).realm;
                if strncmp((*handle).params.realm, (*realm).data,
                           (*realm).length as libc::c_ulong) ==
                       0 as libc::c_int &&
                       strncmp(b"kiprop\x00" as *const u8 as
                                   *const libc::c_char, (*c1).data,
                               (*c1).length as libc::c_ulong) ==
                           0 as libc::c_int {
                    success = 1 as libc::c_int
                }
            }
            if success == 0 {
                krb5_klog_syslog(3 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"bad service principal %.*s%s\x00"
                                              as *const u8 as
                                              *const libc::c_char),
                                 slen as libc::c_int,
                                 gss_str.value as *mut libc::c_char, sdots);
            }
            gss_release_buffer(&mut min_stat, &mut gss_str);
            krb5_free_principal(kctx, princ);
        }
    }
    gss_release_name(&mut min_stat, &mut name);
    return success;
}
/* network.c */
#[no_mangle]
#[c2rust::src_loc = "530:1"]
pub unsafe extern "C" fn krb5_iprop_prog_1(mut rqstp: *mut svc_req,
                                           mut transp: *mut SVCXPRT) {
    let mut argument: C2RustUnnamed_17 =
        C2RustUnnamed_17{iprop_get_updates_1_arg:
                             kdb_last_t{last_sno: 0,
                                        last_time:
                                            kdbe_time_t{seconds: 0,
                                                        useconds: 0,},},};
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _xdr_argument: Option<unsafe extern "C" fn() -> libc::c_int> =
        None;
    let mut _xdr_result: Option<unsafe extern "C" fn() -> libc::c_int> = None;
    let mut local: Option<unsafe extern "C" fn() -> *mut libc::c_void> = None;
    let mut whoami: *mut libc::c_char =
        b"krb5_iprop_prog_1\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    if check_iprop_rpcsec_auth(rqstp) == 0 {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"authentication attempt failed: %s, RPC authentication flavor %d\x00"
                                      as *const u8 as *const libc::c_char),
                         client_addr((*rqstp).rq_xprt),
                         (*rqstp).rq_cred.oa_flavor);
        gssrpc_svcerr_weakauth(transp);
        return
    }
    match (*rqstp).rq_proc {
        0 => {
            gssrpc_svc_sendreply(transp,
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
                                                                                     libc::c_int)),
                                 0 as *mut libc::c_void as *mut libc::c_char);
            return
        }
        1 => {
            _xdr_argument =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut XDR,
                                                                    _:
                                                                        *mut kdb_last_t)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(xdr_kdb_last_t
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut XDR,
                                                                                                   _:
                                                                                                       *mut kdb_last_t)
                                                                                  ->
                                                                                      libc::c_int));
            _xdr_result =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut XDR,
                                                                    _:
                                                                        *mut kdb_incr_result_t)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(xdr_kdb_incr_result_t
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut XDR,
                                                                                                   _:
                                                                                                       *mut kdb_incr_result_t)
                                                                                  ->
                                                                                      libc::c_int));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut kdb_last_t,
                                                                    _:
                                                                        *mut svc_req)
                                                   -> *mut kdb_incr_result_t>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       *mut libc::c_void>>(Some(iprop_get_updates_1_svc
                                                                                    as
                                                                                    unsafe extern "C" fn(_:
                                                                                                             *mut kdb_last_t,
                                                                                                         _:
                                                                                                             *mut svc_req)
                                                                                        ->
                                                                                            *mut kdb_incr_result_t))
        }
        2 => {
            _xdr_argument =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut XDR,
                                                                    _:
                                                                        *mut libc::c_void)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(gssrpc_xdr_void
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut XDR,
                                                                                                   _:
                                                                                                       *mut libc::c_void)
                                                                                  ->
                                                                                      libc::c_int));
            _xdr_result =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut XDR,
                                                                    _:
                                                                        *mut kdb_fullresync_result_t)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(xdr_kdb_fullresync_result_t
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut XDR,
                                                                                                   _:
                                                                                                       *mut kdb_fullresync_result_t)
                                                                                  ->
                                                                                      libc::c_int));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut libc::c_void,
                                                                    _:
                                                                        *mut svc_req)
                                                   ->
                                                       *mut kdb_fullresync_result_t>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       *mut libc::c_void>>(Some(iprop_full_resync_1_svc
                                                                                    as
                                                                                    unsafe extern "C" fn(_:
                                                                                                             *mut libc::c_void,
                                                                                                         _:
                                                                                                             *mut svc_req)
                                                                                        ->
                                                                                            *mut kdb_fullresync_result_t))
        }
        3 => {
            _xdr_argument =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut XDR,
                                                                    _:
                                                                        *mut uint32_t)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(gssrpc_xdr_u_int32
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut XDR,
                                                                                                   _:
                                                                                                       *mut uint32_t)
                                                                                  ->
                                                                                      libc::c_int));
            _xdr_result =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut XDR,
                                                                    _:
                                                                        *mut kdb_fullresync_result_t)
                                                   -> libc::c_int>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       libc::c_int>>(Some(xdr_kdb_fullresync_result_t
                                                                              as
                                                                              unsafe extern "C" fn(_:
                                                                                                       *mut XDR,
                                                                                                   _:
                                                                                                       *mut kdb_fullresync_result_t)
                                                                                  ->
                                                                                      libc::c_int));
            local =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut uint32_t,
                                                                    _:
                                                                        *mut svc_req)
                                                   ->
                                                       *mut kdb_fullresync_result_t>,
                                        Option<unsafe extern "C" fn()
                                                   ->
                                                       *mut libc::c_void>>(Some(iprop_full_resync_ext_1_svc
                                                                                    as
                                                                                    unsafe extern "C" fn(_:
                                                                                                             *mut uint32_t,
                                                                                                         _:
                                                                                                             *mut svc_req)
                                                                                        ->
                                                                                            *mut kdb_fullresync_result_t))
        }
        _ => {
            krb5_klog_syslog(3 as libc::c_int,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"RPC unknown request: %d (%s)\x00" as
                                          *const u8 as *const libc::c_char),
                             (*rqstp).rq_proc, whoami);
            gssrpc_svcerr_noproc(transp);
            return
        }
    }
    memset(&mut argument as *mut C2RustUnnamed_17 as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<C2RustUnnamed_17>() as libc::c_ulong);
    if Some((*(*transp).xp_ops).xp_getargs.expect("non-null function pointer")).expect("non-null function pointer")(transp,
                                                                                                                    _xdr_argument,
                                                                                                                    &mut argument
                                                                                                                        as
                                                                                                                        *mut C2RustUnnamed_17
                                                                                                                        as
                                                                                                                        caddr_t
                                                                                                                        as
                                                                                                                        *mut libc::c_void)
           == 0 {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"RPC svc_getargs failed (%s)\x00" as
                                      *const u8 as *const libc::c_char),
                         whoami);
        gssrpc_svcerr_decode(transp);
        return
    }
    result =
        ::std::mem::transmute::<_,
                                fn(_: _, _: _)
                                    ->
                                        *mut libc::c_void>(Some(local.expect("non-null function pointer")).expect("non-null function pointer"))(&mut argument,
                                                                                                                                                rqstp);
    if _xdr_result.is_some() && !result.is_null() &&
           gssrpc_svc_sendreply(transp, _xdr_result, result as caddr_t) == 0 {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"RPC svc_sendreply failed (%s)\x00" as
                                      *const u8 as *const libc::c_char),
                         whoami);
        gssrpc_svcerr_systemerr(transp);
    }
    if Some((*(*transp).xp_ops).xp_freeargs.expect("non-null function pointer")).expect("non-null function pointer")(transp,
                                                                                                                     _xdr_argument,
                                                                                                                     &mut argument
                                                                                                                         as
                                                                                                                         *mut C2RustUnnamed_17
                                                                                                                         as
                                                                                                                         caddr_t
                                                                                                                         as
                                                                                                                         *mut libc::c_void)
           == 0 {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"RPC svc_freeargs failed (%s)\x00" as
                                      *const u8 as *const libc::c_char),
                         whoami);
        exit(1 as libc::c_int);
    }
    if (*rqstp).rq_proc == 1 as libc::c_int as libc::c_uint {
        /* union XXX *, struct svc_req * */
        /* LINTED */
        let mut r: *mut kdb_incr_result_t = result as *mut kdb_incr_result_t;
        if (*r).ret as libc::c_uint ==
               UPDATE_OK as libc::c_int as libc::c_uint {
            ulog_free_entries((*r).updates.kdb_ulog_t_val,
                              (*r).updates.kdb_ulog_t_len as libc::c_int);
            (*r).updates.kdb_ulog_t_val = 0 as *mut kdb_incr_update_t;
            (*r).updates.kdb_ulog_t_len = 0 as libc::c_int as u_int
        }
    };
}
