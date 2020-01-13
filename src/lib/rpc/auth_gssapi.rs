use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:6"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:6"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:10"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:10"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/sys/types.h:10"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:10"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:10"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "122:16"]
    pub struct gss_channel_bindings_struct {
        pub initiator_addrtype: OM_uint32,
        pub initiator_address: gss_buffer_desc,
        pub acceptor_addrtype: OM_uint32,
        pub acceptor_address: gss_buffer_desc,
        pub application_data: gss_buffer_desc,
    }
    #[c2rust::src_loc = "122:1"]
    pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        /* cred_handle */
        #[no_mangle]
        #[c2rust::src_loc = "437:1"]
        pub fn gss_init_sec_context(_: *mut OM_uint32, _: gss_cred_id_t,
                                    _: *mut gss_ctx_id_t, _: gss_name_t,
                                    _: gss_OID, _: OM_uint32, _: OM_uint32,
                                    _: gss_channel_bindings_t,
                                    _: gss_buffer_t, _: *mut gss_OID,
                                    _: gss_buffer_t, _: *mut OM_uint32,
                                    _: *mut OM_uint32) -> OM_uint32;
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        /* output_name_type */
        #[no_mangle]
        #[c2rust::src_loc = "562:1"]
        pub fn gss_import_name(_: *mut OM_uint32, _: gss_buffer_t, _: gss_OID,
                               _: *mut gss_name_t) -> OM_uint32;
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
        /* output_message_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "760:1"]
        pub fn gss_unseal(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_buffer_t,
                          _: gss_buffer_t, _: *mut libc::c_int,
                          _: *mut libc::c_int) -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:16"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:16"]
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "194:1"]
        pub fn gssrpc_xdr_opaque_auth(_: *mut XDR, _: *mut opaque_auth)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_AUTH_H) */
    /* RPCSEC_GSS */
    /* GSS-API style */
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
        /* stderr */
        #[no_mangle]
        #[c2rust::src_loc = "322:1"]
        pub fn gssrpc_clnt_sperror(_: *mut CLIENT, _: *mut libc::c_char)
         -> *mut libc::c_char;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gssapi.h:17"]
pub mod auth_gssapi_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:16"]
    pub struct _auth_gssapi_creds {
        pub version: uint32_t,
        pub auth_msg: libc::c_int,
        pub client_handle: gss_buffer_desc,
    }
    #[c2rust::src_loc = "32:1"]
    pub type auth_gssapi_creds = _auth_gssapi_creds;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _auth_gssapi_init_arg {
        pub version: uint32_t,
        pub token: gss_buffer_desc,
    }
    #[c2rust::src_loc = "38:1"]
    pub type auth_gssapi_init_arg = _auth_gssapi_init_arg;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:16"]
    pub struct _auth_gssapi_init_res {
        pub version: uint32_t,
        pub client_handle: gss_buffer_desc,
        pub gss_major: OM_uint32,
        pub gss_minor: OM_uint32,
        pub token: gss_buffer_desc,
        pub signed_isn: gss_buffer_desc,
    }
    #[c2rust::src_loc = "43:1"]
    pub type auth_gssapi_init_res = _auth_gssapi_init_res;
    use super::stdint_uintn_h::uint32_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32, gss_ctx_id_struct,
                          gss_ctx_id_t, gss_buffer_desc_struct, gss_buffer_t};
    use super::xdr_h::XDR;
    use super::sys_types_h::caddr_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn gssrpc_xdr_authgssapi_creds(_: *mut XDR,
                                           _: *mut auth_gssapi_creds)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn gssrpc_xdr_authgssapi_init_arg(_: *mut XDR,
                                              _: *mut auth_gssapi_init_arg)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn gssrpc_xdr_authgssapi_init_res(_: *mut XDR,
                                              _: *mut auth_gssapi_init_res)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn gssrpc_auth_gssapi_wrap_data(major: *mut OM_uint32,
                                            minor: *mut OM_uint32,
                                            context: gss_ctx_id_t,
                                            seq_num: uint32_t,
                                            out_xdrs: *mut XDR,
                                            xdr_func:
                                                Option<unsafe extern "C" fn()
                                                           -> libc::c_int>,
                                            xdr_ptr: caddr_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn gssrpc_auth_gssapi_unwrap_data(major: *mut OM_uint32,
                                              minor: *mut OM_uint32,
                                              context: gss_ctx_id_t,
                                              seq_num: uint32_t,
                                              in_xdrs: *mut XDR,
                                              xdr_func:
                                                  Option<unsafe extern "C" fn()
                                                             -> libc::c_int>,
                                              xdr_ptr: caddr_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn gssrpc_auth_gssapi_display_status(msg: *mut libc::c_char,
                                                 major: OM_uint32,
                                                 minor: OM_uint32);
        #[no_mangle]
        #[c2rust::src_loc = "114:1"]
        pub fn gssrpc_auth_gssapi_seal_seq(context: gss_ctx_id_t,
                                           seq_num: uint32_t,
                                           out_buf: gss_buffer_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "117:1"]
        pub fn gssrpc_auth_gssapi_unseal_seq(context: gss_ctx_id_t,
                                             in_buf: gss_buffer_t,
                                             seq_num: *mut uint32_t)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_AUTH_GSSAPI_H) */
}
#[c2rust::header_src = "/usr/include/string.h:7"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_generic.h:11"]
pub mod gssapi_generic_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "54:27"]
        pub static mut gss_nt_service_name: gss_OID;
    }
    /* _GSSAPI_GENERIC_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:13"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:13"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "81:33"]
        pub static gss_mech_krb5: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "82:33"]
        pub static gss_mech_krb5_old: gss_OID;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/rpc/gssrpcint.h:19"]
pub mod gssrpcint_h {
    extern "C" {
        /* lib/rpc/gssrpcint.h */
/*
 * Copyright (C) 2008 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
        #[no_mangle]
        #[c2rust::src_loc = "29:1"]
        pub fn gssrpcint_printf(format: *const libc::c_char, _: ...);
    }
    /* __GSSRPCINT_H__ */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_int, __uint16_t, __int32_t, __uint32_t, __time_t,
                        __suseconds_t, __caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint16_t, uint32_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::struct_timeval_h::timeval;
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         gss_release_cred, gss_init_sec_context,
                         gss_delete_sec_context, gss_import_name,
                         gss_release_name, gss_release_buffer, gss_unseal};
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{in_addr_t, in_addr, in_port_t, sockaddr_in, ntohl};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdrmem_create,
                      gssrpc_xdr_free};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, des_block,
                       opaque_auth, AUTH, auth_ops, gssrpc_xdr_opaque_auth};
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
                       C2RustUnnamed_8, CLIENT, clnt_ops, gssrpc_rpc_createrr,
                       gssrpc_clnt_sperror};
pub use self::auth_gssapi_h::{_auth_gssapi_creds, auth_gssapi_creds,
                              _auth_gssapi_init_arg, auth_gssapi_init_arg,
                              _auth_gssapi_init_res, auth_gssapi_init_res,
                              gssrpc_xdr_authgssapi_creds,
                              gssrpc_xdr_authgssapi_init_arg,
                              gssrpc_xdr_authgssapi_init_res,
                              gssrpc_auth_gssapi_wrap_data,
                              gssrpc_auth_gssapi_unwrap_data,
                              gssrpc_auth_gssapi_display_status,
                              gssrpc_auth_gssapi_seal_seq,
                              gssrpc_auth_gssapi_unseal_seq};
use self::string_h::{memcpy, memset, memcmp, strlen};
use self::gssapi_generic_h::gss_nt_service_name;
use self::stdlib_h::{malloc, free};
use self::gssapi_krb5_h::{gss_mech_krb5, gss_mech_krb5_old};
use self::gssrpcint_h::gssrpcint_printf;
/*
 * the ah_private data structure for an auth_handle
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "61:8"]
pub struct auth_gssapi_data {
    pub established: libc::c_int,
    pub clnt: *mut CLIENT,
    pub context: gss_ctx_id_t,
    pub client_handle: gss_buffer_desc,
    pub seq_num: uint32_t,
    pub def_cred: libc::c_int,
    pub cred_buf: [libc::c_uchar; 400],
    pub cred_len: uint32_t,
}
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved.
 *
 */
#[no_mangle]
#[c2rust::src_loc = "26:5"]
pub static mut gssrpc_auth_debug_gssapi: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "48:24"]
static mut auth_gssapi_ops: auth_ops =
    unsafe {
        {
            let mut init =
                auth_ops{ah_nextverf:
                             Some(auth_gssapi_nextverf as
                                      unsafe extern "C" fn(_: *mut AUTH)
                                          -> ()),
                         ah_marshal:
                             Some(auth_gssapi_marshall as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR)
                                          -> libc::c_int),
                         ah_validate:
                             Some(auth_gssapi_validate as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _:
                                                               *mut opaque_auth)
                                          -> libc::c_int),
                         ah_refresh:
                             Some(auth_gssapi_refresh as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut rpc_msg)
                                          -> libc::c_int),
                         ah_destroy:
                             Some(auth_gssapi_destroy as
                                      unsafe extern "C" fn(_: *mut AUTH)
                                          -> ()),
                         ah_wrap:
                             Some(auth_gssapi_wrap as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR,
                                                           _: xdrproc_t,
                                                           _: caddr_t)
                                          -> libc::c_int),
                         ah_unwrap:
                             Some(auth_gssapi_unwrap as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR,
                                                           _: xdrproc_t,
                                                           _: caddr_t)
                                          -> libc::c_int),};
            init
        }
    };
/*
 * Function: auth_gssapi_create_default
 *
 * Purpose:  Create a GSS-API style authenticator, with default
 * options, and return the handle.
 *
 * Effects: See design document, section XXX.
 */
#[no_mangle]
#[c2rust::src_loc = "83:1"]
pub unsafe extern "C" fn gssrpc_auth_gssapi_create_default(mut clnt:
                                                               *mut CLIENT,
                                                           mut service_name:
                                                               *mut libc::c_char)
 -> *mut AUTH {
    let mut auth: *mut AUTH = 0 as *mut AUTH;
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut input_name: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut target_name: gss_name_t = 0 as *mut gss_name_struct;
    input_name.value = service_name as *mut libc::c_void;
    input_name.length =
        strlen(service_name).wrapping_add(1 as libc::c_int as libc::c_ulong);
    gssstat =
        gss_import_name(&mut minor_stat, &mut input_name, gss_nt_service_name,
                        &mut target_name);
    if gssstat != 0 as libc::c_int as libc::c_uint {
        if gssrpc_auth_debug_gssapi != 0 {
            gssrpc_auth_gssapi_display_status(b"parsing name\x00" as *const u8
                                                  as *const libc::c_char as
                                                  *mut libc::c_char, gssstat,
                                              minor_stat);
        }
        gssrpc_rpc_createrr.cf_stat = RPC_SYSTEMERROR;
        gssrpc_rpc_createrr.cf_error.ru.RE_errno = 12 as libc::c_int;
        return 0 as *mut AUTH
    }
    auth =
        gssrpc_auth_gssapi_create(clnt, &mut gssstat, &mut minor_stat,
                                  0 as gss_cred_id_t, target_name,
                                  0 as gss_OID,
                                  (2 as libc::c_int | 4 as libc::c_int) as
                                      OM_uint32,
                                  0 as libc::c_int as OM_uint32,
                                  0 as *mut gss_OID, 0 as *mut OM_uint32,
                                  0 as *mut OM_uint32);
    gss_release_name(&mut minor_stat, &mut target_name);
    return auth;
}
/*
 * Function: auth_gssapi_create
 *
 * Purpose: Create a GSS-API style authenticator, with all the
 * options, and return the handle.
 *
 * Effects: See design document, section XXX.
 */
#[no_mangle]
#[c2rust::src_loc = "127:1"]
pub unsafe extern "C" fn gssrpc_auth_gssapi_create(mut clnt: *mut CLIENT,
                                                   mut gssstat:
                                                       *mut OM_uint32,
                                                   mut minor_stat:
                                                       *mut OM_uint32,
                                                   mut claimant_cred_handle:
                                                       gss_cred_id_t,
                                                   mut target_name:
                                                       gss_name_t,
                                                   mut mech_type: gss_OID,
                                                   mut req_flags: OM_uint32,
                                                   mut time_req: OM_uint32,
                                                   mut actual_mech_type:
                                                       *mut gss_OID,
                                                   mut ret_flags:
                                                       *mut OM_uint32,
                                                   mut time_rec:
                                                       *mut OM_uint32)
 -> *mut AUTH {
    let mut err: rpc_err =
        rpc_err{re_status: RPC_SUCCESS, ru: C2RustUnnamed_6{RE_errno: 0,},};
    let mut current_block: u64;
    let mut auth: *mut AUTH = 0 as *mut AUTH;
    let mut save_auth: *mut AUTH = 0 as *mut AUTH;
    let mut pdata: *mut auth_gssapi_data = 0 as *mut auth_gssapi_data;
    let mut bindings: gss_channel_bindings_struct =
        gss_channel_bindings_struct{initiator_addrtype: 0,
                                    initiator_address:
                                        gss_buffer_desc{length: 0,
                                                        value:
                                                            0 as
                                                                *mut libc::c_void,},
                                    acceptor_addrtype: 0,
                                    acceptor_address:
                                        gss_buffer_desc{length: 0,
                                                        value:
                                                            0 as
                                                                *mut libc::c_void,},
                                    application_data:
                                        gss_buffer_desc{length: 0,
                                                        value:
                                                            0 as
                                                                *mut libc::c_void,},};
    let mut bindp: *mut gss_channel_bindings_struct =
        0 as *mut gss_channel_bindings_struct;
    let mut laddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut raddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut callstat: clnt_stat = RPC_SUCCESS;
    let mut timeout: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut bindings_failed: libc::c_int = 0;
    let mut init_func: rpcproc_t = 0;
    let mut call_arg: auth_gssapi_init_arg =
        auth_gssapi_init_arg{version: 0,
                             token:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as
                                                         *mut libc::c_void,},};
    let mut call_res: auth_gssapi_init_res =
        auth_gssapi_init_res{version: 0,
                             client_handle:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as *mut libc::c_void,},
                             gss_major: 0,
                             gss_minor: 0,
                             token:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as *mut libc::c_void,},
                             signed_isn:
                                 gss_buffer_desc{length: 0,
                                                 value:
                                                     0 as
                                                         *mut libc::c_void,},};
    let mut input_token: *mut gss_buffer_desc = 0 as *mut gss_buffer_desc;
    let mut isn_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    memset(&mut gssrpc_rpc_createrr as *mut gssrpc_rpc_createrr as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<gssrpc_rpc_createrr>() as libc::c_ulong);
    /* this timeout is only used if clnt_control(clnt, CLSET_TIMEOUT) */
     /* has not already been called.. therefore, we can just pick */
     /* something reasonable-sounding.. */
    timeout.tv_sec = 30 as libc::c_int as __time_t;
    timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
    auth = 0 as *mut AUTH;
    pdata = 0 as *mut auth_gssapi_data;
    /* don't assume the caller will want to change clnt->cl_auth */
    save_auth = (*clnt).cl_auth;
    auth =
        malloc(::std::mem::size_of::<AUTH>() as libc::c_ulong) as *mut AUTH;
    pdata =
        malloc(::std::mem::size_of::<auth_gssapi_data>() as libc::c_ulong) as
            *mut auth_gssapi_data;
    if auth.is_null() || pdata.is_null() {
        /* They needn't both have failed; clean up.  */
        free(auth as *mut libc::c_void);
        free(pdata as *mut libc::c_void);
        auth = 0 as *mut AUTH;
        pdata = 0 as *mut auth_gssapi_data;
        gssrpc_rpc_createrr.cf_stat = RPC_SYSTEMERROR;
        gssrpc_rpc_createrr.cf_error.ru.RE_errno = 12 as libc::c_int
    } else {
        memset(auth as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<AUTH>() as libc::c_ulong);
        memset(pdata as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<auth_gssapi_data>() as libc::c_ulong);
        (*auth).ah_ops = &mut auth_gssapi_ops;
        (*auth).ah_private = pdata as caddr_t as *mut libc::c_void;
        /* initial creds are auth_msg TRUE and no handle */
        marshall_new_creds(auth, 1 as libc::c_int, 0 as gss_buffer_t);
        /* initial verifier is empty */
        (*auth).ah_verf.oa_flavor = 300001 as libc::c_int;
        (*auth).ah_verf.oa_base = 0 as caddr_t;
        (*auth).ah_verf.oa_length = 0 as libc::c_int as u_int;
        (*((*auth).ah_private as *mut auth_gssapi_data)).established =
            0 as libc::c_int;
        let ref mut fresh0 =
            (*((*auth).ah_private as *mut auth_gssapi_data)).clnt;
        *fresh0 = clnt;
        (*((*auth).ah_private as *mut auth_gssapi_data)).def_cred =
            (claimant_cred_handle == 0 as gss_cred_id_t) as libc::c_int;
        (*clnt).cl_auth = auth;
        /* start by trying latest version */
        call_arg.version = 4 as libc::c_int as uint32_t;
        bindings_failed = 0 as libc::c_int;
        'c_8109:
            loop  {
                /* set state for initial call to init_sec_context */
                input_token = 0 as gss_buffer_t;
                let ref mut fresh1 =
                    (*((*auth).ah_private as *mut auth_gssapi_data)).context;
                *fresh1 = 0 as gss_ctx_id_t;
                init_func = 1 as libc::c_int as rpcproc_t;
                /*
      * OV servers up to version 3 used the old mech id.  Beta 7
      * servers used version 3 with the new mech id; however, the beta
      * 7 gss-api accept_sec_context accepts either mech id.  Thus, if
      * any server rejects version 4, we fall back to version 3 with
      * the old mech id; for the OV server it will be right, and for
      * the beta 7 server it will be accepted.  Not ideal, but it
      * works.
      */
                if call_arg.version < 4 as libc::c_int as libc::c_uint &&
                       (mech_type == gss_mech_krb5 || mech_type.is_null()) {
                    mech_type = gss_mech_krb5_old
                }
                if bindings_failed == 0 &&
                       call_arg.version >= 3 as libc::c_int as libc::c_uint {
                    if Some((*(*clnt).cl_ops).cl_control.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                                  6
                                                                                                                                      as
                                                                                                                                      libc::c_int,
                                                                                                                                  &mut laddr
                                                                                                                                      as
                                                                                                                                      *mut sockaddr_in
                                                                                                                                      as
                                                                                                                                      *mut libc::c_void)
                           == 0 as libc::c_int {
                        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
                            gssrpcint_printf(b"gssapi_create: CLGET_LOCAL_ADDR failed\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                        }
                        break ;
                    } else if Some((*(*clnt).cl_ops).cl_control.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                                         3
                                                                                                                                             as
                                                                                                                                             libc::c_int,
                                                                                                                                         &mut raddr
                                                                                                                                             as
                                                                                                                                             *mut sockaddr_in
                                                                                                                                             as
                                                                                                                                             *mut libc::c_void)
                                  == 0 as libc::c_int {
                        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
                            gssrpcint_printf(b"gssapi_create: CLGET_SERVER_ADDR failed\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                        }
                        break ;
                    } else {
                        memset(&mut bindings as
                                   *mut gss_channel_bindings_struct as
                                   *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<gss_channel_bindings_struct>()
                                   as libc::c_ulong);
                        bindings.application_data.length =
                            0 as libc::c_int as size_t;
                        bindings.initiator_addrtype =
                            2 as libc::c_int as OM_uint32;
                        bindings.initiator_address.length =
                            4 as libc::c_int as size_t;
                        bindings.initiator_address.value =
                            &mut laddr.sin_addr.s_addr as *mut in_addr_t as
                                *mut libc::c_void;
                        bindings.acceptor_addrtype =
                            2 as libc::c_int as OM_uint32;
                        bindings.acceptor_address.length =
                            4 as libc::c_int as size_t;
                        bindings.acceptor_address.value =
                            &mut raddr.sin_addr.s_addr as *mut in_addr_t as
                                *mut libc::c_void;
                        bindp = &mut bindings
                    }
                } else { bindp = 0 as *mut gss_channel_bindings_struct }
                memset(&mut call_res as *mut auth_gssapi_init_res as
                           *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<auth_gssapi_init_res>() as
                           libc::c_ulong);
                loop  {
                    *gssstat =
                        gss_init_sec_context(minor_stat, claimant_cred_handle,
                                             &mut (*((*auth).ah_private as
                                                         *mut auth_gssapi_data)).context,
                                             target_name, mech_type,
                                             req_flags, time_req, bindp,
                                             input_token, actual_mech_type,
                                             &mut call_arg.token, ret_flags,
                                             time_rec);
                    if *gssstat != 0 as libc::c_int as libc::c_uint &&
                           *gssstat !=
                               ((1 as libc::c_int) <<
                                    0 as libc::c_int + 0 as libc::c_int) as
                                   libc::c_uint {
                        if gssrpc_auth_debug_gssapi != 0 {
                            gssrpc_auth_gssapi_display_status(b"initializing context\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char,
                                                              *gssstat,
                                                              *minor_stat);
                        }
                        break 'c_8109 ;
                    } else {
                        /* if we got a token, pass it on */
                        if call_arg.token.length !=
                               0 as libc::c_int as libc::c_ulong {
                            /*
	   * sanity check: if we received a signed isn in the last
	   * response then there *cannot* be another token to send
	   */
                            if call_res.signed_isn.length !=
                                   0 as libc::c_int as libc::c_ulong {
                                if gssrpc_auth_debug_gssapi >=
                                       99 as libc::c_int {
                                    gssrpcint_printf(b"gssapi_create: unexpected token from init_sec\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                }
                                break 'c_8109 ;
                            } else {
                                if gssrpc_auth_debug_gssapi >=
                                       99 as libc::c_int {
                                    gssrpcint_printf(b"gssapi_create: calling GSSAPI_INIT (%d)\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     init_func);
                                }
                                gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                        *mut XDR,
                                                                                                    _:
                                                                                                        *mut auth_gssapi_init_res)
                                                                                   ->
                                                                                       libc::c_int>,
                                                                        xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_res
                                                                                            as
                                                                                            unsafe extern "C" fn(_:
                                                                                                                     *mut XDR,
                                                                                                                 _:
                                                                                                                     *mut auth_gssapi_init_res)
                                                                                                ->
                                                                                                    libc::c_int)),
                                                &mut call_res as
                                                    *mut auth_gssapi_init_res
                                                    as *mut libc::c_void);
                                memset(&mut call_res as
                                           *mut auth_gssapi_init_res as
                                           *mut libc::c_void,
                                       0 as libc::c_int,
                                       ::std::mem::size_of::<auth_gssapi_init_res>()
                                           as libc::c_ulong);
                                callstat =
                                    Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                                            init_func,
                                                                                                                                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                                                    *mut XDR,
                                                                                                                                                                                                _:
                                                                                                                                                                                                    *mut auth_gssapi_init_arg)
                                                                                                                                                                               ->
                                                                                                                                                                                   libc::c_int>,
                                                                                                                                                                    xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_arg
                                                                                                                                                                                        as
                                                                                                                                                                                        unsafe extern "C" fn(_:
                                                                                                                                                                                                                 *mut XDR,
                                                                                                                                                                                                             _:
                                                                                                                                                                                                                 *mut auth_gssapi_init_arg)
                                                                                                                                                                                            ->
                                                                                                                                                                                                libc::c_int)),
                                                                                                                                            &mut call_arg
                                                                                                                                                as
                                                                                                                                                *mut auth_gssapi_init_arg
                                                                                                                                                as
                                                                                                                                                *mut libc::c_void,
                                                                                                                                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                                                    *mut XDR,
                                                                                                                                                                                                _:
                                                                                                                                                                                                    *mut auth_gssapi_init_res)
                                                                                                                                                                               ->
                                                                                                                                                                                   libc::c_int>,
                                                                                                                                                                    xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_res
                                                                                                                                                                                        as
                                                                                                                                                                                        unsafe extern "C" fn(_:
                                                                                                                                                                                                                 *mut XDR,
                                                                                                                                                                                                             _:
                                                                                                                                                                                                                 *mut auth_gssapi_init_res)
                                                                                                                                                                                            ->
                                                                                                                                                                                                libc::c_int)),
                                                                                                                                            &mut call_res
                                                                                                                                                as
                                                                                                                                                *mut auth_gssapi_init_res
                                                                                                                                                as
                                                                                                                                                *mut libc::c_void,
                                                                                                                                            timeout);
                                gss_release_buffer(minor_stat,
                                                   &mut call_arg.token);
                                if callstat as libc::c_uint !=
                                       RPC_SUCCESS as libc::c_int as
                                           libc::c_uint {
                                    err =
                                        rpc_err{re_status: RPC_SUCCESS,
                                                ru:
                                                    C2RustUnnamed_6{RE_errno:
                                                                        0,},};
                                    Some((*(*clnt).cl_ops).cl_geterr.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                                              &mut err);
                                    if callstat as libc::c_uint ==
                                           RPC_AUTHERROR as libc::c_int as
                                               libc::c_uint &&
                                           (err.ru.RE_why as libc::c_uint ==
                                                AUTH_BADCRED as libc::c_int as
                                                    libc::c_uint ||
                                                err.ru.RE_why as libc::c_uint
                                                    ==
                                                    AUTH_FAILED as libc::c_int
                                                        as libc::c_uint) &&
                                           call_arg.version >=
                                               1 as libc::c_int as
                                                   libc::c_uint {
                                        current_block = 5684854171168229155;
                                        break ;
                                    } else {
                                        current_block = 15462640364611497761;
                                        break ;
                                    }
                                } else if call_res.version != call_arg.version
                                              &&
                                              !(call_arg.version ==
                                                    2 as libc::c_int as
                                                        libc::c_uint &&
                                                    call_res.version ==
                                                        1 as libc::c_int as
                                                            libc::c_uint) {
                                    /*
		* The Secure 1.1 servers always respond with version
		* 1.  Thus, if we just tried a version >=3, fall all
		* the way back to version 1 since that is all they
		* understand
		*/
                                    if call_arg.version >
                                           2 as libc::c_int as libc::c_uint &&
                                           call_res.version ==
                                               1 as libc::c_int as
                                                   libc::c_uint {
                                        current_block = 10007731352114176167;
                                        break ;
                                    } else {
                                        current_block = 17711149709958600598;
                                        break ;
                                    }
                                } else if call_res.gss_major !=
                                              0 as libc::c_int as libc::c_uint
                                 {
                                    if gssrpc_auth_debug_gssapi != 0 {
                                        gssrpc_auth_gssapi_display_status(b"in response from server\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char
                                                                              as
                                                                              *mut libc::c_char,
                                                                          call_res.gss_major,
                                                                          call_res.gss_minor);
                                    }
                                    break 'c_8109 ;
                                } else {
                                    if gssrpc_auth_debug_gssapi >=
                                           99 as libc::c_int {
                                        gssrpcint_printf(b"gssapi_create: GSSAPI_INIT (%d) succeeded\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         init_func);
                                    }
                                    init_func = 2 as libc::c_int as rpcproc_t;
                                    /* check for client_handle */
                                    if (*((*auth).ah_private as
                                              *mut auth_gssapi_data)).client_handle.length
                                           ==
                                           0 as libc::c_int as libc::c_ulong {
                                        if call_res.client_handle.length ==
                                               0 as libc::c_int as
                                                   libc::c_ulong {
                                            if gssrpc_auth_debug_gssapi >=
                                                   99 as libc::c_int {
                                                gssrpcint_printf(b"gssapi_create: expected client_handle\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
                                            }
                                            break 'c_8109 ;
                                        } else {
                                            if gssrpc_auth_debug_gssapi >=
                                                   99 as libc::c_int {
                                                gssrpcint_printf(b"gssapi_create: got client_handle %d\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 *(call_res.client_handle.value
                                                                       as
                                                                       *mut uint32_t));
                                            }
                                            (*((*auth).ah_private as
                                                   *mut auth_gssapi_data)).client_handle.length
                                                =
                                                call_res.client_handle.length;
                                            let ref mut fresh2 =
                                                (*((*auth).ah_private as
                                                       *mut auth_gssapi_data)).client_handle.value;
                                            *fresh2 =
                                                malloc((*((*auth).ah_private
                                                              as
                                                              *mut auth_gssapi_data)).client_handle.length);
                                            memcpy((*((*auth).ah_private as
                                                          *mut auth_gssapi_data)).client_handle.value,
                                                   call_res.client_handle.value,
                                                   (*((*auth).ah_private as
                                                          *mut auth_gssapi_data)).client_handle.length);
                                            /* auth_msg is TRUE; there may be more tokens */
                                            marshall_new_creds(auth,
                                                               1 as
                                                                   libc::c_int,
                                                               &mut (*((*auth).ah_private
                                                                           as
                                                                           *mut auth_gssapi_data)).client_handle);
                                        }
                                    } else if !((*((*auth).ah_private as
                                                       *mut auth_gssapi_data)).client_handle.length
                                                    ==
                                                    call_res.client_handle.length
                                                    &&
                                                    memcmp((*((*auth).ah_private
                                                                  as
                                                                  *mut auth_gssapi_data)).client_handle.value,
                                                           call_res.client_handle.value,
                                                           (*((*auth).ah_private
                                                                  as
                                                                  *mut auth_gssapi_data)).client_handle.length)
                                                        == 0) {
                                        if gssrpc_auth_debug_gssapi >=
                                               99 as libc::c_int {
                                            gssrpcint_printf(b"gssapi_create: got different client_handle\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char);
                                        }
                                        break 'c_8109 ;
                                    }
                                    /* check for token */
                                    if call_res.token.length ==
                                           0 as libc::c_int as libc::c_ulong
                                           &&
                                           *gssstat ==
                                               ((1 as libc::c_int) <<
                                                    0 as libc::c_int +
                                                        0 as libc::c_int) as
                                                   libc::c_uint {
                                        if gssrpc_auth_debug_gssapi >=
                                               99 as libc::c_int {
                                            gssrpcint_printf(b"gssapi_create: expected token\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char);
                                        }
                                        break 'c_8109 ;
                                    } else if call_res.token.length !=
                                                  0 as libc::c_int as
                                                      libc::c_ulong {
                                        if *gssstat ==
                                               0 as libc::c_int as
                                                   libc::c_uint {
                                            if gssrpc_auth_debug_gssapi >=
                                                   99 as libc::c_int {
                                                gssrpcint_printf(b"gssapi_create: got unexpected token\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
                                            }
                                            break 'c_8109 ;
                                        } else {
                                            /* assumes call_res is safe until init_sec_context */
                                            input_token = &mut call_res.token;
                                            if gssrpc_auth_debug_gssapi >=
                                                   99 as libc::c_int {
                                                gssrpcint_printf(b"gssapi_create: got new token\n\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        /* check for isn */
                        if *gssstat == 0 as libc::c_int as libc::c_uint {
                            if call_res.signed_isn.length ==
                                   0 as libc::c_int as libc::c_ulong {
                                if gssrpc_auth_debug_gssapi >=
                                       99 as libc::c_int {
                                    gssrpcint_printf(b"gssapi_created: expected signed isn\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                }
                                break 'c_8109 ;
                            } else {
                                if gssrpc_auth_debug_gssapi >=
                                       99 as libc::c_int {
                                    gssrpcint_printf(b"gssapi_create: processing signed isn\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                }
                                /* don't check conf (integ only) or qop (accpet default) */
                                *gssstat =
                                    gss_unseal(minor_stat,
                                               (*((*auth).ah_private as
                                                      *mut auth_gssapi_data)).context,
                                               &mut call_res.signed_isn,
                                               &mut isn_buf,
                                               0 as *mut libc::c_int,
                                               0 as *mut libc::c_int);
                                if *gssstat !=
                                       0 as libc::c_int as libc::c_uint {
                                    if gssrpc_auth_debug_gssapi != 0 {
                                        gssrpc_auth_gssapi_display_status(b"unsealing isn\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char
                                                                              as
                                                                              *mut libc::c_char,
                                                                          *gssstat,
                                                                          *minor_stat);
                                    }
                                    break 'c_8109 ;
                                } else if isn_buf.length !=
                                              ::std::mem::size_of::<uint32_t>()
                                                  as libc::c_ulong {
                                    if gssrpc_auth_debug_gssapi >=
                                           99 as libc::c_int {
                                        gssrpcint_printf(b"gssapi_create: gss_unseal gave %d bytes\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         isn_buf.length as
                                                             libc::c_int);
                                    }
                                    break 'c_8109 ;
                                } else {
                                    (*((*auth).ah_private as
                                           *mut auth_gssapi_data)).seq_num =
                                        ntohl(*(isn_buf.value as
                                                    *mut uint32_t));
                                    *gssstat =
                                        gss_release_buffer(minor_stat,
                                                           &mut isn_buf);
                                    if *gssstat !=
                                           0 as libc::c_int as libc::c_uint {
                                        if gssrpc_auth_debug_gssapi != 0 {
                                            gssrpc_auth_gssapi_display_status(b"releasing unsealed isn\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char
                                                                                  as
                                                                                  *mut libc::c_char,
                                                                              *gssstat,
                                                                              *minor_stat);
                                        }
                                        break 'c_8109 ;
                                    } else if gssrpc_auth_debug_gssapi >=
                                                  99 as libc::c_int {
                                        gssrpcint_printf(b"gssapi_create: isn is %d\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char,
                                                         (*((*auth).ah_private
                                                                as
                                                                *mut auth_gssapi_data)).seq_num);
                                    }
                                }
                            }
                        } else if call_res.signed_isn.length !=
                                      0 as libc::c_int as libc::c_ulong {
                            if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
                                gssrpcint_printf(b"gssapi_create: got signed isn, can\'t check yet\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char);
                            }
                        }
                        /* results were okay.. continue if necessary */
                        if *gssstat ==
                               ((1 as libc::c_int) <<
                                    0 as libc::c_int + 0 as libc::c_int) as
                                   libc::c_uint {
                            if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
                                gssrpcint_printf(b"gssapi_create: not done, continuing\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char);
                            }
                        } else {
                            /*
      * Done!  Context is established, we have client_handle and isn.
      */
                            (*((*auth).ah_private as
                                   *mut auth_gssapi_data)).established =
                                1 as libc::c_int;
                            marshall_new_creds(auth, 0 as libc::c_int,
                                               &mut (*((*auth).ah_private as
                                                           *mut auth_gssapi_data)).client_handle);
                            if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
                                gssrpcint_printf(b"gssapi_create: done. client_handle %#x, isn %d\n\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 *((*((*auth).ah_private as
                                                          *mut auth_gssapi_data)).client_handle.value
                                                       as *mut uint32_t),
                                                 (*((*auth).ah_private as
                                                        *mut auth_gssapi_data)).seq_num);
                            }
                            /* don't assume the caller will want to change clnt->cl_auth */
                            (*clnt).cl_auth = save_auth;
                            gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                    *mut XDR,
                                                                                                _:
                                                                                                    *mut auth_gssapi_init_res)
                                                                               ->
                                                                                   libc::c_int>,
                                                                    xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_res
                                                                                        as
                                                                                        unsafe extern "C" fn(_:
                                                                                                                 *mut XDR,
                                                                                                             _:
                                                                                                                 *mut auth_gssapi_init_res)
                                                                                            ->
                                                                                                libc::c_int)),
                                            &mut call_res as
                                                *mut auth_gssapi_init_res as
                                                *mut libc::c_void);
                            return auth
                        }
                    }
                }
                match current_block {
                    17711149709958600598 => {
                        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
                            gssrpcint_printf(b"gssapi_create: invalid call_res vers %d\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             call_res.version);
                        }
                        break ;
                    }
                    10007731352114176167 => {
                        if gssrpc_auth_debug_gssapi >= 1 as libc::c_int {
                            gssrpcint_printf(b"Talking to Secure 1.1 server, using version 1.\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                        }
                        call_arg.version = 1 as libc::c_int as uint32_t
                    }
                    15462640364611497761 => {
                        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
                            gssrpcint_printf(b"gssapi_create: GSSAPI_INIT (%d) failed, stat %d\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             init_func,
                                             callstat as libc::c_uint);
                        }
                        break ;
                    }
                    _ => {
                        if gssrpc_auth_debug_gssapi >= 1 as libc::c_int {
                            gssrpcint_printf(b"call_arg protocol version %d rejected, trying %d.\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             call_arg.version,
                                             call_arg.version.wrapping_sub(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint));
                        }
                        call_arg.version = call_arg.version.wrapping_sub(1)
                    }
                }
            }
    }
    /* *****************************************************************/
    if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_create: bailing\n\n\x00" as *const u8 as
                             *const libc::c_char);
    }
    if !auth.is_null() {
        if !((*auth).ah_private as *mut auth_gssapi_data).is_null() {
            auth_gssapi_destroy(auth);
        } else { free(auth as *mut libc::c_void); }
        auth = 0 as *mut AUTH
    }
    /* don't assume the caller will want to change clnt->cl_auth */
    (*clnt).cl_auth = save_auth;
    if gssrpc_rpc_createrr.cf_stat as libc::c_uint ==
           0 as libc::c_int as libc::c_uint {
        gssrpc_rpc_createrr.cf_stat = RPC_AUTHERROR
    }
    gssrpc_xdr_free(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut XDR,
                                                                        _:
                                                                            *mut auth_gssapi_init_res)
                                                       -> libc::c_int>,
                                            xdrproc_t>(Some(gssrpc_xdr_authgssapi_init_res
                                                                as
                                                                unsafe extern "C" fn(_:
                                                                                         *mut XDR,
                                                                                     _:
                                                                                         *mut auth_gssapi_init_res)
                                                                    ->
                                                                        libc::c_int)),
                    &mut call_res as *mut auth_gssapi_init_res as
                        *mut libc::c_void);
    return auth;
}
/*
 * Function: marshall_new_creds
 *
 * Purpose: (pre-)serialize auth_msg and client_handle fields of
 * auth_gssapi_creds into auth->cred_buf
 *
 * Arguments:
 *
 * 	auth		(r/w) the AUTH structure to modify
 * 	auth_msg	(r) the auth_msg field to serialize
 * 	client_handle	(r) the client_handle field to serialize, or
 * 			NULL
 *
 * Returns: TRUE if successful, FALSE if not
 *
 * Requires: auth must point to a valid GSS-API auth structure, auth_msg
 * must be TRUE or FALSE, client_handle must be a gss_buffer_t with a valid
 * value and length field or NULL.
 *
 * Effects: auth->ah_cred is set to the serialized auth_gssapi_creds
 * version 2 structure (stored in the cred_buf field of private data)
 * containing version, auth_msg and client_handle.
 * auth->ah_cred.oa_flavor is set to AUTH_GSSAPI.  If cliend_handle is
 * NULL, it is treated as if it had a length of 0 and a value of NULL.
 *
 * Modifies: auth
 */
#[c2rust::src_loc = "492:1"]
unsafe extern "C" fn marshall_new_creds(mut auth: *mut AUTH,
                                        mut auth_msg: libc::c_int,
                                        mut client_handle: gss_buffer_t)
 -> libc::c_int {
    let mut creds: auth_gssapi_creds =
        auth_gssapi_creds{version: 0,
                          auth_msg: 0,
                          client_handle:
                              gss_buffer_desc{length: 0,
                                              value:
                                                  0 as *mut libc::c_void,},};
    let mut xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"marshall_new_creds: starting\n\x00" as *const u8 as
                             *const libc::c_char);
    }
    creds.version = 2 as libc::c_int as uint32_t;
    creds.auth_msg = auth_msg;
    if !client_handle.is_null() {
        creds.client_handle.length = (*client_handle).length;
        creds.client_handle.value = (*client_handle).value
    } else {
        creds.client_handle.length = 0 as libc::c_int as size_t;
        creds.client_handle.value = 0 as *mut libc::c_void
    }
    gssrpc_xdrmem_create(&mut xdrs,
                         (*((*auth).ah_private as
                                *mut auth_gssapi_data)).cred_buf.as_mut_ptr()
                             as caddr_t, 400 as libc::c_int as u_int,
                         XDR_ENCODE);
    if gssrpc_xdr_authgssapi_creds(&mut xdrs, &mut creds) == 0 {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"marshall_new_creds: failed encoding auth_gssapi_creds\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        if (*xdrs.x_ops).x_destroy.is_some() {
            Some((*xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut xdrs);
        }
        return 0 as libc::c_int
    }
    (*((*auth).ah_private as *mut auth_gssapi_data)).cred_len =
        Some((*xdrs.x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(&mut xdrs);
    if (*xdrs.x_ops).x_destroy.is_some() {
        Some((*xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut xdrs);
    }
    if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"marshall_new_creds: auth_gssapi_creds is %d bytes\n\x00"
                             as *const u8 as *const libc::c_char,
                         (*((*auth).ah_private as
                                *mut auth_gssapi_data)).cred_len);
    }
    (*auth).ah_cred.oa_flavor = 300001 as libc::c_int;
    (*auth).ah_cred.oa_base =
        (*((*auth).ah_private as *mut auth_gssapi_data)).cred_buf.as_mut_ptr()
            as *mut libc::c_char;
    (*auth).ah_cred.oa_length =
        (*((*auth).ah_private as *mut auth_gssapi_data)).cred_len;
    if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"marshall_new_creds: succeeding\n\x00" as *const u8
                             as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
/*
 * Function: auth_gssapi_nextverf
 *
 * Purpose: None.
 *
 * Effects: None.  Never called.
 */
#[c2rust::src_loc = "542:1"]
unsafe extern "C" fn auth_gssapi_nextverf(mut auth: *mut AUTH) { }
/*
 * Function: auth_gssapi_marhsall
 *
 * Purpose: Marshall RPC credentials and verifier onto xdr stream.
 *
 * Arguments:
 *
 * 	auth		(r/w) AUTH structure for client
 * 	xdrs		(r/w) XDR stream to marshall to
 *
 * Returns: boolean indicating success/failure
 *
 * Effects:
 *
 * The pre-serialized credentials in cred_buf are serialized.  If the
 * context is established, the sealed sequence number is serialized as
 * the verifier.  If the context is not established, an empty verifier
 * is serialized.  The sequence number is *not* incremented, because
 * this function is called multiple times if retransmission is required.
 *
 * If this took all the header fields as arguments, it could sign
 * them.
 */
#[c2rust::src_loc = "569:1"]
unsafe extern "C" fn auth_gssapi_marshall(mut auth: *mut AUTH,
                                          mut xdrs: *mut XDR) -> libc::c_int {
    let mut minor_stat: OM_uint32 = 0;
    let mut out_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut seq_num: uint32_t = 0;
    if (*((*auth).ah_private as *mut auth_gssapi_data)).established ==
           1 as libc::c_int {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_marshall: starting\n\x00" as *const u8
                                 as *const libc::c_char);
        }
        seq_num =
            (*((*auth).ah_private as
                   *mut auth_gssapi_data)).seq_num.wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint);
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_marshall: sending seq_num %d\n\x00" as
                                 *const u8 as *const libc::c_char, seq_num);
        }
        if gssrpc_auth_gssapi_seal_seq((*((*auth).ah_private as
                                              *mut auth_gssapi_data)).context,
                                       seq_num, &mut out_buf) ==
               0 as libc::c_int {
            if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
                gssrpcint_printf(b"gssapi_marhshall: seal failed\n\x00" as
                                     *const u8 as *const libc::c_char);
            }
        }
        (*auth).ah_verf.oa_base = out_buf.value as caddr_t;
        (*auth).ah_verf.oa_length = out_buf.length as u_int;
        if gssrpc_xdr_opaque_auth(xdrs, &mut (*auth).ah_cred) == 0 ||
               gssrpc_xdr_opaque_auth(xdrs, &mut (*auth).ah_verf) == 0 {
            gss_release_buffer(&mut minor_stat, &mut out_buf);
            return 0 as libc::c_int
        }
        gss_release_buffer(&mut minor_stat, &mut out_buf);
    } else {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_marshall: not established, sending null verf\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        (*auth).ah_verf.oa_base = 0 as caddr_t;
        (*auth).ah_verf.oa_length = 0 as libc::c_int as u_int;
        if gssrpc_xdr_opaque_auth(xdrs, &mut (*auth).ah_cred) == 0 ||
               gssrpc_xdr_opaque_auth(xdrs, &mut (*auth).ah_verf) == 0 {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
/*
 * Function: auth_gssapi_validate
 *
 * Purpose: Validate RPC response verifier from server.
 *
 * Effects: See design document, section XXX.
 */
#[c2rust::src_loc = "620:1"]
unsafe extern "C" fn auth_gssapi_validate(mut auth: *mut AUTH,
                                          mut verf: *mut opaque_auth)
 -> libc::c_int {
    let mut in_buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut seq_num: uint32_t = 0;
    if (*((*auth).ah_private as *mut auth_gssapi_data)).established ==
           0 as libc::c_int {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_validate: not established, noop\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        return 1 as libc::c_int
    }
    if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_validate: starting\n\x00" as *const u8 as
                             *const libc::c_char);
    }
    in_buf.length = (*verf).oa_length as size_t;
    in_buf.value = (*verf).oa_base as *mut libc::c_void;
    if gssrpc_auth_gssapi_unseal_seq((*((*auth).ah_private as
                                            *mut auth_gssapi_data)).context,
                                     &mut in_buf, &mut seq_num) ==
           0 as libc::c_int {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_validate: failed unsealing verifier\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        return 0 as libc::c_int
    }
    /* we sent seq_num+1, so we should get back seq_num+2 */
    if (*((*auth).ah_private as
              *mut auth_gssapi_data)).seq_num.wrapping_add(2 as libc::c_int as
                                                               libc::c_uint)
           != seq_num {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_validate: expecting seq_num %d, got %d (%#x)\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*((*auth).ah_private as
                                    *mut auth_gssapi_data)).seq_num.wrapping_add(2
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint),
                             seq_num, seq_num);
        }
        return 0 as libc::c_int
    }
    if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_validate: seq_num %d okay\n\x00" as
                             *const u8 as *const libc::c_char, seq_num);
    }
    /* +1 for successful transmission, +1 for successful validation */
    let ref mut fresh3 =
        (*((*auth).ah_private as *mut auth_gssapi_data)).seq_num;
    *fresh3 =
        (*fresh3 as
             libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint) as
            uint32_t as uint32_t;
    if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_validate: succeeding\n\x00" as *const u8 as
                             *const libc::c_char);
    }
    return 1 as libc::c_int;
}
/*
 * Function: auth_gssapi_refresh
 *
 * Purpose: Attempts to resyncrhonize the sequence number.
 *
 * Effects:
 *
 * When the server receives a properly authenticated RPC call, it
 * increments the sequence number it is expecting from the client.
 * But if the server's response is lost for any reason, the client
 * can't know whether the server ever received it, assumes it didn't,
 * and does *not* increment its sequence number.  Thus, the client's
 * next call will fail with AUTH_REJECTEDCRED because the server will
 * think it is a replay attack.
 *
 * When an AUTH_REJECTEDCRED error arrives, this function attempts to
 * resyncrhonize by incrementing the client's sequence number and
 * returning TRUE.  If any other error arrives, it returns FALSE.
 */
#[c2rust::src_loc = "677:1"]
unsafe extern "C" fn auth_gssapi_refresh(mut auth: *mut AUTH,
                                         mut msg: *mut rpc_msg)
 -> libc::c_int {
    if (*msg).ru.RM_rmb.ru.RP_dr.rj_stat as libc::c_uint ==
           AUTH_ERROR as libc::c_int as libc::c_uint &&
           (*msg).ru.RM_rmb.ru.RP_dr.ru.RJ_why as libc::c_uint ==
               AUTH_REJECTEDVERF as libc::c_int as libc::c_uint {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_refresh: rejected verifier, incrementing\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        let ref mut fresh4 =
            (*((*auth).ah_private as *mut auth_gssapi_data)).seq_num;
        *fresh4 = (*fresh4).wrapping_add(1);
        return 1 as libc::c_int
    } else {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_refresh: failing\n\x00" as *const u8 as
                                 *const libc::c_char);
        }
        return 0 as libc::c_int
    };
}
/*
 * Function: auth_gssapi_destroy
 *
 * Purpose: Destroy a GSS-API authentication structure.
 *
 * Effects:  This function destroys the GSS-API authentication
 * context, and sends a message to the server instructing it to
 * invokte gss_process_token() and thereby destroy its corresponding
 * context.  Since the client doesn't really care whether the server
 * gets this message, no failures are reported.
 */
#[c2rust::src_loc = "703:1"]
unsafe extern "C" fn auth_gssapi_destroy(mut auth: *mut AUTH) {
    let mut timeout: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut cred: gss_cred_id_t = 0 as *mut gss_cred_id_struct;
    let mut callstat: libc::c_int = 0;
    if (*((*auth).ah_private as *mut auth_gssapi_data)).client_handle.length
           == 0 as libc::c_int as libc::c_ulong {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_destroy: no client_handle, not calling destroy\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
    } else {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_destroy: marshalling new creds\n\x00" as
                                 *const u8 as *const libc::c_char);
        }
        if marshall_new_creds(auth, 1 as libc::c_int,
                              &mut (*((*auth).ah_private as
                                          *mut auth_gssapi_data)).client_handle)
               == 0 {
            if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
                gssrpcint_printf(b"gssapi_destroy: marshall_new_creds failed\n\x00"
                                     as *const u8 as *const libc::c_char);
            }
        } else {
            if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
                gssrpcint_printf(b"gssapi_destroy: calling GSSAPI_DESTROY\n\x00"
                                     as *const u8 as *const libc::c_char);
            }
            timeout.tv_sec = 1 as libc::c_int as __time_t;
            timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
            callstat =
                Some((*(*(*((*auth).ah_private as
                                *mut auth_gssapi_data)).clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")((*((*auth).ah_private
                                                                                                                                                              as
                                                                                                                                                              *mut auth_gssapi_data)).clnt,
                                                                                                                                                       4
                                                                                                                                                           as
                                                                                                                                                           libc::c_int
                                                                                                                                                           as
                                                                                                                                                           rpcproc_t,
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
                                                                                                                                                       0
                                                                                                                                                           as
                                                                                                                                                           *mut libc::c_void,
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
                                                                                                                                                       0
                                                                                                                                                           as
                                                                                                                                                           *mut libc::c_void,
                                                                                                                                                       timeout)
                    as libc::c_int;
            if callstat != RPC_SUCCESS as libc::c_int {
                gssrpc_clnt_sperror((*((*auth).ah_private as
                                           *mut auth_gssapi_data)).clnt,
                                    b"gssapi_destroy: GSSAPI_DESTROY failed\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char);
            }
        }
    }
    if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_destroy: deleting context\n\x00" as
                             *const u8 as *const libc::c_char);
    }
    gssstat =
        gss_delete_sec_context(&mut minor_stat,
                               &mut (*((*auth).ah_private as
                                           *mut auth_gssapi_data)).context,
                               0 as gss_buffer_t);
    if gssstat != 0 as libc::c_int as libc::c_uint {
        if gssrpc_auth_debug_gssapi != 0 {
            gssrpc_auth_gssapi_display_status(b"deleting context\x00" as
                                                  *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char, gssstat,
                                              minor_stat);
        }
    }
    if (*((*auth).ah_private as *mut auth_gssapi_data)).def_cred != 0 {
        cred = 0 as gss_cred_id_t;
        gssstat = gss_release_cred(&mut minor_stat, &mut cred);
        if gssstat != 0 as libc::c_int as libc::c_uint {
            if gssrpc_auth_debug_gssapi != 0 {
                gssrpc_auth_gssapi_display_status(b"deleting default credential\x00"
                                                      as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char,
                                                  gssstat, minor_stat);
            }
        }
    }
    free((*((*auth).ah_private as
                *mut auth_gssapi_data)).client_handle.value);
    free((*auth).ah_private);
    free(auth as *mut libc::c_void);
    if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
        gssrpcint_printf(b"gssapi_destroy: done\n\x00" as *const u8 as
                             *const libc::c_char);
    };
}
/*
 * Function: auth_gssapi_wrap
 *
 * Purpose: encrypt the serialized arguments from xdr_func applied to
 * xdr_ptr and write the result to xdrs.
 *
 * Effects: See design doc, section XXX.
 */
#[c2rust::src_loc = "760:1"]
unsafe extern "C" fn auth_gssapi_wrap(mut auth: *mut AUTH,
                                      mut out_xdrs: *mut XDR,
                                      mut xdr_func:
                                          Option<unsafe extern "C" fn()
                                                     -> libc::c_int>,
                                      mut xdr_ptr: caddr_t) -> libc::c_int {
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    if (*((*auth).ah_private as *mut auth_gssapi_data)).established == 0 {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_wrap: context not established, noop\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        return ::std::mem::transmute::<_,
                                       fn(_: _, _: _)
                                           ->
                                               libc::c_int>(Some(xdr_func.expect("non-null function pointer")).expect("non-null function pointer"))(out_xdrs,
                                                                                                                                                    xdr_ptr)
    } else if gssrpc_auth_gssapi_wrap_data(&mut gssstat, &mut minor_stat,
                                           (*((*auth).ah_private as
                                                  *mut auth_gssapi_data)).context,
                                           (*((*auth).ah_private as
                                                  *mut auth_gssapi_data)).seq_num.wrapping_add(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint),
                                           out_xdrs, xdr_func, xdr_ptr) == 0 {
        if gssstat != 0 as libc::c_int as libc::c_uint {
            if gssrpc_auth_debug_gssapi != 0 {
                gssrpc_auth_gssapi_display_status(b"encrypting function arguments\x00"
                                                      as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char,
                                                  gssstat, minor_stat);
            }
        }
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
/*
 * Function: auth_gssapi_unwrap
 *
 * Purpose: read encrypted arguments from xdrs, decrypt, and
 * deserialize with xdr_func into xdr_ptr.
 *
 * Effects: See design doc, section XXX.
 */
#[c2rust::src_loc = "791:1"]
unsafe extern "C" fn auth_gssapi_unwrap(mut auth: *mut AUTH,
                                        mut in_xdrs: *mut XDR,
                                        mut xdr_func:
                                            Option<unsafe extern "C" fn()
                                                       -> libc::c_int>,
                                        mut xdr_ptr: caddr_t) -> libc::c_int {
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    if (*((*auth).ah_private as *mut auth_gssapi_data)).established == 0 {
        if gssrpc_auth_debug_gssapi >= 99 as libc::c_int {
            gssrpcint_printf(b"gssapi_unwrap: context not established, noop\n\x00"
                                 as *const u8 as *const libc::c_char);
        }
        return ::std::mem::transmute::<_,
                                       fn(_: _, _: _)
                                           ->
                                               libc::c_int>(Some(xdr_func.expect("non-null function pointer")).expect("non-null function pointer"))(in_xdrs,
                                                                                                                                                    xdr_ptr)
    } else if gssrpc_auth_gssapi_unwrap_data(&mut gssstat, &mut minor_stat,
                                             (*((*auth).ah_private as
                                                    *mut auth_gssapi_data)).context,
                                             (*((*auth).ah_private as
                                                    *mut auth_gssapi_data)).seq_num,
                                             in_xdrs, xdr_func, xdr_ptr) == 0
     {
        if gssstat != 0 as libc::c_int as libc::c_uint {
            if gssrpc_auth_debug_gssapi != 0 {
                gssrpc_auth_gssapi_display_status(b"decrypting function arguments\x00"
                                                      as *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char,
                                                  gssstat, minor_stat);
            }
        }
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
