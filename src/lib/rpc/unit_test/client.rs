use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:9"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:9"]
pub mod types_h {
    #[c2rust::src_loc = "31:1"]
    pub type __u_char = libc::c_uchar;
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
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:9"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:9"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:11"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/sys/types.h:11"]
pub mod sys_types_h {
    #[c2rust::src_loc = "33:1"]
    pub type u_char = __u_char;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_char, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:11"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:11"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:11"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:11"]
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "376:1"]
        pub fn ntohs(__netshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src = "/usr/include/netdb.h:11"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:8"]
    pub struct hostent {
        pub h_name: *mut libc::c_char,
        pub h_aliases: *mut *mut libc::c_char,
        pub h_addrtype: libc::c_int,
        pub h_length: libc::c_int,
        pub h_addr_list: *mut *mut libc::c_char,
    }
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "142:1"]
        pub fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    }
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
    /* This is for rpc/netdb.h */
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "274:1"]
        pub fn gssrpc_xdr_wrapstring(_: *mut XDR, _: *mut *mut libc::c_char)
         -> libc::c_int;
        /* free memory buffers for xdr */
        #[no_mangle]
        #[c2rust::src_loc = "335:1"]
        pub fn gssrpc_xdr_free(_: xdrproc_t, _: *mut libc::c_void);
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:16"]
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
        pub ru: C2RustUnnamed_6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:2"]
    pub union C2RustUnnamed_6 {
        pub RE_errno: libc::c_int,
        pub RE_why: auth_stat,
        pub RE_vers: C2RustUnnamed_8,
        pub RE_lb: C2RustUnnamed_7,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "99:3"]
    pub struct C2RustUnnamed_7 {
        pub s1: int32_t,
        pub s2: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:3"]
    pub struct C2RustUnnamed_8 {
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
    use super::sys_types_h::u_int;
    use super::types_h::{__time_t, __suseconds_t};
    extern "C" {
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
        #[no_mangle]
        #[c2rust::src_loc = "277:1"]
        pub fn gssrpc_clnttcp_create(_: *mut sockaddr_in, _: rpcprog_t,
                                     _: rpcvers_t, _: *mut libc::c_int,
                                     _: u_int, _: u_int) -> *mut CLIENT;
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
        /*
 * Print why creation failed
 */
        #[no_mangle]
        #[c2rust::src_loc = "310:1"]
        pub fn gssrpc_clnt_pcreateerror(_: *mut libc::c_char);
        /* stderr */
        /*
 * Print an English error message, given the client error code
 */
        #[no_mangle]
        #[c2rust::src_loc = "321:1"]
        pub fn gssrpc_clnt_perror(_: *mut CLIENT, _: *mut libc::c_char);
        /* stderr */
        #[no_mangle]
        #[c2rust::src_loc = "322:1"]
        pub fn gssrpc_clnt_sperror(_: *mut CLIENT, _: *mut libc::c_char)
         -> *mut libc::c_char;
    }
    /* !defined(GSSRPC_CLNT_H) */
    /* a more reasonable packet size */
    /* rpc imposed limit on udp msg size */
    /* string */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:16"]
pub mod gssapi_h {
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:9"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "521:1"]
        pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:10"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:14"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
                      __shortopts: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "50:12"]
        pub static mut optind: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "36:14"]
        pub static mut optarg: *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:16"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gssapi.h:20"]
pub mod auth_gssapi_h {
    use super::clnt_h::CLIENT;
    use super::auth_h::AUTH;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "107:1"]
        pub fn gssrpc_auth_gssapi_create_default(clnt: *mut CLIENT,
                                                 service_name:
                                                     *mut libc::c_char)
         -> *mut AUTH;
    }
    /* !defined(GSSRPC_AUTH_GSSAPI_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/rpc/unit-test/rpc_test.h:21"]
pub mod rpc_test_h {
    use super::clnt_h::CLIENT;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "10:1"]
        pub fn rpc_test_echo_1(_: *mut *mut libc::c_char, _: *mut CLIENT)
         -> *mut *mut libc::c_char;
    }
    /* !_RPC_TEST_H_RPCGEN */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_char, __u_int, __uint16_t, __int32_t, __uint32_t,
                        __off_t, __off64_t, __time_t, __suseconds_t,
                        __caddr_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::sys_types_h::{u_char, u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::struct_timeval_h::timeval;
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{in_port_t, sockaddr_in, in_addr, in_addr_t, ntohs};
pub use self::netdb_h::{hostent, gethostbyname};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_wrapstring, gssrpc_xdr_free};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, des_block,
                       opaque_auth, AUTH, auth_ops};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed, reply_body, C2RustUnnamed_0,
                          rejected_reply, C2RustUnnamed_1, C2RustUnnamed_2,
                          reject_stat, AUTH_ERROR, RPC_MISMATCH,
                          accepted_reply, C2RustUnnamed_3, C2RustUnnamed_4,
                          C2RustUnnamed_5, accept_stat, SYSTEM_ERR,
                          GARBAGE_ARGS, PROC_UNAVAIL, PROG_MISMATCH,
                          PROG_UNAVAIL, SUCCESS, reply_stat, MSG_DENIED,
                          MSG_ACCEPTED, call_body, msg_type, REPLY, CALL};
pub use self::clnt_h::{clnt_stat, RPC_FAILED, RPC_PROGNOTREGISTERED,
                       RPC_PMAPFAILURE, RPC_UNKNOWNPROTO, RPC_UNKNOWNHOST,
                       RPC_SYSTEMERROR, RPC_CANTDECODEARGS, RPC_PROCUNAVAIL,
                       RPC_PROGVERSMISMATCH, RPC_PROGUNAVAIL, RPC_AUTHERROR,
                       RPC_VERSMISMATCH, RPC_TIMEDOUT, RPC_CANTRECV,
                       RPC_CANTSEND, RPC_CANTDECODERES, RPC_CANTENCODEARGS,
                       RPC_SUCCESS, rpc_err, C2RustUnnamed_6, C2RustUnnamed_7,
                       C2RustUnnamed_8, CLIENT, clnt_ops,
                       gssrpc_clnttcp_create, gssrpc_clntudp_create,
                       gssrpc_clnt_pcreateerror, gssrpc_clnt_perror,
                       gssrpc_clnt_sperror};
pub use self::gssapi_h::{gss_ctx_id_t, gss_uint32, OM_uint32,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_ctx_id_struct};
use self::stdio_h::{stdout, stderr, fflush, fprintf, snprintf, fputc};
use self::string_h::{memmove, memset, strcmp, strncmp};
use self::getopt_core_h::{getopt, optind, optarg};
use self::stdlib_h::{atoi, exit};
use self::auth_gssapi_h::gssrpc_auth_gssapi_create_default;
use self::rpc_test_h::rpc_test_echo_1;
extern "C" {
    #[no_mangle]
    #[c2rust::src_loc = "39:12"]
    pub static mut gssrpc_auth_debug_gssapi: libc::c_int;
}
/* copied from auth_gssapi.c for hackery */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "25:8"]
pub struct auth_gssapi_data {
    pub established: libc::c_int,
    pub clnt: *mut CLIENT,
    pub context: gss_ctx_id_t,
    pub client_handle: gss_buffer_desc,
    pub seq_num: OM_uint32,
    pub def_cred: libc::c_int,
    pub cred_buf: [u_char; 400],
    pub cred_len: int32_t,
}
#[no_mangle]
#[c2rust::src_loc = "40:7"]
pub static mut whoami: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn usage() -> ! {
    fprintf(stderr,
            b"usage: %s {-t|-u} [-a] [-s num] [-m num] host service [count]\n\x00"
                as *const u8 as *const libc::c_char, whoami);
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "52:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut echo_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut echo_resp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut clnt: *mut CLIENT = 0 as *mut CLIENT;
    let mut tmp_auth: *mut AUTH = 0 as *mut AUTH;
    let mut e: rpc_err =
        rpc_err{re_status: RPC_SUCCESS, ru: C2RustUnnamed_6{RE_errno: 0,},};
    let mut auth_once: libc::c_int = 0;
    let mut sock: libc::c_int = 0;
    let mut use_tcp: libc::c_int = 0;
    let mut count: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    extern "C" {
        #[link_name = "optind"]
        pub static mut optind_0: libc::c_int;
    }
    extern "C" {
        #[link_name = "optarg"]
        pub static mut optarg_0: *mut libc::c_char;
    }
    extern "C" {
        #[no_mangle]
        pub static mut gssrpc_svc_debug_gssapi: libc::c_int;
    }
    extern "C" {
        #[no_mangle]
        pub static mut gssrpc_misc_debug_gssapi: libc::c_int;
    }
    extern "C" {
        #[link_name = "gssrpc_auth_debug_gssapi"]
        pub static mut gssrpc_auth_debug_gssapi_0: libc::c_int;
    }
    let mut c: libc::c_int = 0;
    let mut sin: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut h: *mut hostent = 0 as *mut hostent;
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    extern "C" {
        #[no_mangle]
        pub static mut krb5_gss_dbg_client_expcreds: libc::c_int;
    }
    krb5_gss_dbg_client_expcreds = 1 as libc::c_int;
    whoami = *argv.offset(0 as libc::c_int as isize);
    count = 1026 as libc::c_int as libc::c_uint;
    auth_once = 0 as libc::c_int;
    use_tcp = -(1 as libc::c_int);
    loop  {
        c =
            getopt(argc, argv,
                   b"a:m:os:tu\x00" as *const u8 as *const libc::c_char);
        if !(c != -(1 as libc::c_int)) { break ; }
        match c {
            97 => { gssrpc_auth_debug_gssapi = atoi(optarg) }
            109 => { gssrpc_misc_debug_gssapi = atoi(optarg) }
            111 => { auth_once += 1 }
            115 => { gssrpc_svc_debug_gssapi = atoi(optarg) }
            116 => { use_tcp = 1 as libc::c_int }
            117 => { use_tcp = 0 as libc::c_int }
            63 => { usage(); }
            _ => { }
        }
    }
    if use_tcp == -(1 as libc::c_int) { usage(); }
    argv = argv.offset(optind as isize);
    argc -= optind;
    match argc {
        4 => {
            count =
                atoi(*argv.offset(3 as libc::c_int as isize)) as libc::c_uint;
            if count >
                   (4096 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
                fprintf(stderr,
                        b"Test count cannot exceed %d.\n\x00" as *const u8 as
                            *const libc::c_char,
                        4096 as libc::c_int - 1 as libc::c_int);
                usage();
            }
        }
        3 => { }
        _ => { usage(); }
    }
    host = *argv.offset(0 as libc::c_int as isize);
    port = *argv.offset(1 as libc::c_int as isize);
    target = *argv.offset(2 as libc::c_int as isize);
    /* get server address */
    h = gethostbyname(host);
    if h.is_null() {
        fprintf(stderr,
                b"Can\'t resolve hostname %s\n\x00" as *const u8 as
                    *const libc::c_char, host);
        exit(1 as libc::c_int);
    }
    memset(&mut sin as *mut sockaddr_in as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
    sin.sin_family = (*h).h_addrtype as sa_family_t;
    sin.sin_port = ntohs(atoi(port) as uint16_t);
    memmove(&mut sin.sin_addr as *mut in_addr as *mut libc::c_void,
            *(*h).h_addr_list.offset(0 as libc::c_int as isize) as
                *const libc::c_void,
            ::std::mem::size_of::<in_addr>() as libc::c_ulong);
    /* client handle to rstat */
    sock = -(1 as libc::c_int);
    if use_tcp != 0 {
        clnt =
            gssrpc_clnttcp_create(&mut sin,
                                  1000001 as libc::c_int as libc::c_ulong as
                                      rpcprog_t,
                                  1 as libc::c_int as libc::c_ulong as
                                      rpcvers_t, &mut sock,
                                  0 as libc::c_int as u_int,
                                  0 as libc::c_int as u_int)
    } else {
        tv.tv_sec = 5 as libc::c_int as __time_t;
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        clnt =
            gssrpc_clntudp_create(&mut sin,
                                  1000001 as libc::c_int as libc::c_ulong as
                                      rpcprog_t,
                                  1 as libc::c_int as libc::c_ulong as
                                      rpcvers_t, tv, &mut sock)
    }
    if clnt.is_null() {
        gssrpc_clnt_pcreateerror(whoami);
        exit(1 as libc::c_int);
    }
    (*clnt).cl_auth = gssrpc_auth_gssapi_create_default(clnt, target);
    if (*clnt).cl_auth.is_null() {
        gssrpc_clnt_pcreateerror(whoami);
        exit(2 as libc::c_int);
    }
    /*
      * Call the echo service multiple times.
      */
    echo_arg = buf.as_mut_ptr();
    i = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        snprintf(buf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 4096]>() as
                     libc::c_ulong,
                 b"testing %d\n\x00" as *const u8 as *const libc::c_char, i);
        echo_resp = rpc_test_echo_1(&mut echo_arg, clnt);
        if echo_resp.is_null() {
            fprintf(stderr,
                    b"RPC_TEST_ECHO call %d%s\x00" as *const u8 as
                        *const libc::c_char, i,
                    gssrpc_clnt_sperror(clnt,
                                        b"\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char));
        }
        if strncmp(*echo_resp,
                   b"Echo: \x00" as *const u8 as *const libc::c_char,
                   6 as libc::c_int as libc::c_ulong) != 0 &&
               strcmp(echo_arg,
                      (*echo_resp).offset(6 as libc::c_int as isize)) !=
                   0 as libc::c_int {
            fprintf(stderr,
                    b"RPC_TEST_ECHO call %d response wrong: arg = %s, resp = %s\n\x00"
                        as *const u8 as *const libc::c_char, i, echo_arg,
                    *echo_resp);
        }
        gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut *mut libc::c_char)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(gssrpc_xdr_wrapstring
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut *mut libc::c_char)
                                                                        ->
                                                                            libc::c_int)),
                        echo_resp as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    /*
      * Make a call with an invalid verifier and check for error;
      * server should log error message.  It is important to
      *increment* seq_num here, since a decrement would be fixed (see
      * below).  Note that seq_num will be incremented (by
      * authg_gssapi_refresh) twice, so we need to decrement by three
      * to reset.
      */
    let ref mut fresh0 =
        (*((*(*clnt).cl_auth).ah_private as *mut auth_gssapi_data)).seq_num;
    *fresh0 = (*fresh0).wrapping_add(1);
    echo_arg =
        b"testing with bad verf\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    echo_resp = rpc_test_echo_1(&mut echo_arg, clnt);
    if echo_resp.is_null() {
        Some((*(*clnt).cl_ops).cl_geterr.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                  &mut e);
        if e.re_status as libc::c_uint !=
               RPC_AUTHERROR as libc::c_int as libc::c_uint ||
               e.ru.RE_why as libc::c_uint !=
                   AUTH_REJECTEDVERF as libc::c_int as libc::c_uint {
            gssrpc_clnt_perror(clnt, whoami);
        }
    } else {
        fprintf(stderr,
                b"bad seq didn\'t cause failure\n\x00" as *const u8 as
                    *const libc::c_char);
        gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut *mut libc::c_char)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(gssrpc_xdr_wrapstring
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut *mut libc::c_char)
                                                                        ->
                                                                            libc::c_int)),
                        echo_resp as *mut libc::c_void);
    }
    let ref mut fresh1 =
        (*((*(*clnt).cl_auth).ah_private as *mut auth_gssapi_data)).seq_num;
    *fresh1 =
        (*fresh1 as
             libc::c_uint).wrapping_sub(3 as libc::c_int as libc::c_uint) as
            OM_uint32 as OM_uint32;
    /*
      * Make sure we're resyncronized.
      */
    echo_arg =
        b"testing for reset\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    echo_resp = rpc_test_echo_1(&mut echo_arg, clnt);
    if echo_resp.is_null() {
        gssrpc_clnt_perror(clnt,
                           b"Sequence number improperly reset\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char);
    } else {
        gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut *mut libc::c_char)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(gssrpc_xdr_wrapstring
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut *mut libc::c_char)
                                                                        ->
                                                                            libc::c_int)),
                        echo_resp as *mut libc::c_void);
    }
    /*
      * Now simulate a lost server response, and see if
      * auth_gssapi_refresh recovers.
      */
    let ref mut fresh2 =
        (*((*(*clnt).cl_auth).ah_private as *mut auth_gssapi_data)).seq_num;
    *fresh2 = (*fresh2).wrapping_sub(1);
    echo_arg =
        b"forcing auto-resynchronization\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char;
    echo_resp = rpc_test_echo_1(&mut echo_arg, clnt);
    if echo_resp.is_null() {
        gssrpc_clnt_perror(clnt,
                           b"Auto-resynchronization failed\x00" as *const u8
                               as *const libc::c_char as *mut libc::c_char);
    } else {
        gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut *mut libc::c_char)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(gssrpc_xdr_wrapstring
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut *mut libc::c_char)
                                                                        ->
                                                                            libc::c_int)),
                        echo_resp as *mut libc::c_void);
    }
    /*
      * Now make sure auto-resyncrhonization actually worked
      */
    echo_arg =
        b"testing for resynchronization\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char;
    echo_resp = rpc_test_echo_1(&mut echo_arg, clnt);
    if echo_resp.is_null() {
        gssrpc_clnt_perror(clnt,
                           b"Auto-resynchronization did not work\x00" as
                               *const u8 as *const libc::c_char as
                               *mut libc::c_char);
    } else {
        gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut *mut libc::c_char)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(gssrpc_xdr_wrapstring
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut *mut libc::c_char)
                                                                        ->
                                                                            libc::c_int)),
                        echo_resp as *mut libc::c_void);
    }
    /*
      * Test fix for secure-rpc/586, part 1: btree keys must be
      * unique.  Create another context from the same credentials; it
      * should have the same expiration time and will cause the server
      * to abort if the clients are not differentiated.
      *
      * Test fix for secure-rpc/586, part 2: btree keys cannot be
      * mutated in place.  To test this: a second client, *with a
      * later expiration time*, must be run.  The second client should
      * destroy itself *after* the first one; if the key-mutating bug
      * is not fixed, the second client_data will be in the btree
      * before the first, but its key will be larger; thus, when the
      * first client calls AUTH_DESTROY, the server won't find it in
      * the btree and call abort.
      *
      * For unknown reasons, running just a second client didn't
      * tickle the bug; the btree code seemed to guess which node to
      * look at first.  Running a total of three clients does ticket
      * the bug.  Thus, the full test sequence looks like this:
      *
      * 	kinit -l 20m user && client server test@ddn 200
      * 	sleep 1
      * 	kini -l 30m user && client server test@ddn 300
      * 	sleep 1
      * 	kinit -l 40m user && client server test@ddn 400
      */
    if auth_once == 0 {
        tmp_auth = (*clnt).cl_auth;
        (*clnt).cl_auth = gssrpc_auth_gssapi_create_default(clnt, target);
        if (*clnt).cl_auth.is_null() {
            gssrpc_clnt_pcreateerror(whoami);
            exit(2 as libc::c_int);
        }
        Some((*(*(*clnt).cl_auth).ah_ops).ah_destroy.expect("non-null function pointer")).expect("non-null function pointer")((*clnt).cl_auth);
        (*clnt).cl_auth = tmp_auth
    }
    /*
      * Try RPC calls with argument/result lengths [0, 1025].  Do
      * this last, since it takes a while..
      */
    echo_arg = buf.as_mut_ptr();
    memset(buf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           count.wrapping_add(1 as libc::c_int as libc::c_uint) as
               libc::c_ulong);
    i = 0 as libc::c_int as libc::c_uint;
    while i < count {
        echo_resp = rpc_test_echo_1(&mut echo_arg, clnt);
        if echo_resp.is_null() {
            fprintf(stderr,
                    b"RPC_TEST_LENGTHS call %d%s\x00" as *const u8 as
                        *const libc::c_char, i,
                    gssrpc_clnt_sperror(clnt,
                                        b"\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char));
            break ;
        } else {
            if strncmp(*echo_resp,
                       b"Echo: \x00" as *const u8 as *const libc::c_char,
                       6 as libc::c_int as libc::c_ulong) != 0 &&
                   strcmp(echo_arg,
                          (*echo_resp).offset(6 as libc::c_int as isize)) !=
                       0 as libc::c_int {
                fprintf(stderr,
                        b"RPC_TEST_LENGTHS call %d response wrong\n\x00" as
                            *const u8 as *const libc::c_char, i);
            }
            gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    *mut XDR,
                                                                                _:
                                                                                    *mut *mut libc::c_char)
                                                               ->
                                                                   libc::c_int>,
                                                    xdrproc_t>(Some(gssrpc_xdr_wrapstring
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut XDR,
                                                                                             _:
                                                                                                 *mut *mut libc::c_char)
                                                                            ->
                                                                                libc::c_int)),
                            echo_resp as *mut libc::c_void);
            /* cycle from 1 to 255 */
            buf[i as usize] =
                i.wrapping_rem(255 as libc::c_int as
                                   libc::c_uint).wrapping_add(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                    as libc::c_char;
            if i.wrapping_rem(100 as libc::c_int as libc::c_uint) ==
                   0 as libc::c_int as libc::c_uint {
                fputc('.' as i32, stdout);
                fflush(stdout);
            }
            i = i.wrapping_add(1)
        }
    }
    fputc('\n' as i32, stdout);
    Some((*(*(*clnt).cl_auth).ah_ops).ah_destroy.expect("non-null function pointer")).expect("non-null function pointer")((*clnt).cl_auth);
    Some((*(*clnt).cl_ops).cl_destroy.expect("non-null function pointer")).expect("non-null function pointer")(clnt);
    exit(0 as libc::c_int);
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
