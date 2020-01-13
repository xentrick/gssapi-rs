use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:49"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:49"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:49"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:51"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:51"]
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
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:51"]
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
    /*
 * Operations defined on a XDR handle
 *
 * XDR		*xdrs;
 * int32_t	*longp;
 * caddr_t	 addr;
 * u_int	 len;
 * u_int	 pos;
 */
    /*
 * Support struct for discriminated unions.
 * You create an array of xdrdiscrim structures, terminated with
 * a entry with a null procedure pointer.  The xdr_union routine gets
 * the discriminant value and then searches the array of structures
 * for a matching value.  If a match is found the associated xdr routine
 * is called to handle that part of the union.  If there is
 * no match, then a default routine may be called.
 * If there is no match and no default routine it is an error.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:8"]
    pub struct xdr_discrim {
        pub value: libc::c_int,
        pub proc_0: xdrproc_t,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::gssrpc_types_h::rpc_inline_t;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "259:1"]
        pub fn gssrpc_xdr_enum(_: *mut XDR, _: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "262:1"]
        pub fn gssrpc_xdr_bytes(_: *mut XDR, _: *mut *mut libc::c_char,
                                _: *mut u_int, _: u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "263:1"]
        pub fn gssrpc_xdr_opaque(_: *mut XDR, _: caddr_t, _: u_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "265:1"]
        pub fn gssrpc_xdr_union(_: *mut XDR, _: *mut libc::c_int,
                                _: *mut libc::c_char, _: *mut xdr_discrim,
                                _: xdrproc_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "298:1"]
        pub fn gssrpc_xdr_u_int32(_: *mut XDR, _: *mut uint32_t)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:51"]
pub mod auth_h {
    #[c2rust::src_loc = "55:1"]
    pub type auth_stat = libc::c_uint;
    #[c2rust::src_loc = "74:2"]
    pub const RPCSEC_GSS_CTXPROBLEM: auth_stat = 14;
    #[c2rust::src_loc = "73:2"]
    pub const RPCSEC_GSS_CREDPROBLEM: auth_stat = 13;
    #[c2rust::src_loc = "69:2"]
    pub const AUTH_FAILED: auth_stat = 7;
    #[c2rust::src_loc = "68:2"]
    pub const AUTH_INVALIDRESP: auth_stat = 6;
    #[c2rust::src_loc = "64:2"]
    pub const AUTH_TOOWEAK: auth_stat = 5;
    #[c2rust::src_loc = "63:2"]
    pub const AUTH_REJECTEDVERF: auth_stat = 4;
    #[c2rust::src_loc = "62:2"]
    pub const AUTH_BADVERF: auth_stat = 3;
    #[c2rust::src_loc = "61:2"]
    pub const AUTH_REJECTEDCRED: auth_stat = 2;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:51"]
pub mod rpc_msg_h {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:51"]
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
    use super::auth_h::auth_stat;
    use super::stdint_intn_h::int32_t;
    use super::gssrpc_types_h::rpcvers_t;
    /* !defined(GSSRPC_CLNT_H) */
    /* a more reasonable packet size */
    /* rpc imposed limit on udp msg size */
    /* string */
}
pub use self::types_h::{__u_int, __int32_t, __uint32_t, __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, xdr_discrim, gssrpc_xdr_enum,
                      gssrpc_xdr_bytes, gssrpc_xdr_opaque, gssrpc_xdr_union,
                      gssrpc_xdr_u_int32};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, des_block,
                       opaque_auth};
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
                       C2RustUnnamed_8};
/*
 * Authentication info.  Opaque to client.
 */
/* flavor of auth */
/* address of more auth stuff */
/* not to exceed MAX_AUTH_BYTES */
/*
 * Auth handle, interface to client side authenticators.
 */
/* nextverf & serialize */
/* validate varifier */
/* refresh credentials */
/* destroy this structure */
/* encode data for wire */
/* decode data from wire */
/*
 * Authentication ops.
 * The ops and the auth handle provide the interface to the authenticators.
 *
 * AUTH	*auth;
 * XDR	*xdrs;
 * struct opaque_auth verf;
 */
/* RENAMED: should be _null_auth if we can use reserved namespace. */
/*
 * These are the various implementations of client side authenticators.
 */
/*
 * Unix style authentication
 * AUTH *authunix_create(machname, uid, gid, len, aup_gids)
 *	char *machname;
 *	int uid;
 *	int gid;
 *	int len;
 *	int *aup_gids;
 */
/* takes no parameters */
/* takes no parameters */
/* @(#)rpc_prot.c	2.3 88/08/07 4.0 RPCSRC */
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
 * rpc_prot.c
 *
 * This set of routines implements the rpc message definition,
 * its serializer and some common rpc utility routines.
 * The routines are meant for various implementations of rpc -
 * they are NOT for the rpc client or rpc service implementations!
 * Because authentication stuff is easy and is part of rpc, the opaque
 * routines are also in this program.
 */
/* * * * * * * * * * * * * * XDR Authentication * * * * * * * * * * * */
/*
 * XDR an opaque authentication struct
 * (see auth.h)
 */
#[no_mangle]
#[c2rust::src_loc = "59:1"]
pub unsafe extern "C" fn gssrpc_xdr_opaque_auth(mut xdrs: *mut XDR,
                                                mut ap: *mut opaque_auth)
 -> libc::c_int {
    if gssrpc_xdr_enum(xdrs, &mut (*ap).oa_flavor) != 0 {
        return gssrpc_xdr_bytes(xdrs, &mut (*ap).oa_base,
                                &mut (*ap).oa_length,
                                400 as libc::c_int as u_int)
    }
    return 0 as libc::c_int;
}
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
/*
	 * failed at remote end
	 */
/* bogus credentials (seal broken) */
/* client should begin new session */
/* bogus verifier (seal broken) */
/* verifier expired or was replayed */
/* rejected due to security reasons */
/*
	 * failed locally
	*/
/* bogus response verifier */
/* some unknown reason */
/*
	 * RPCSEC_GSS errors
	 */
/*
 * XDR a DES block
 */
#[no_mangle]
#[c2rust::src_loc = "72:1"]
pub unsafe extern "C" fn gssrpc_xdr_des_block(mut xdrs: *mut XDR,
                                              mut blkp: *mut des_block)
 -> libc::c_int {
    return gssrpc_xdr_opaque(xdrs, blkp as caddr_t,
                             ::std::mem::size_of::<des_block>() as
                                 libc::c_ulong as u_int);
}
/* * * * * * * * * * * * * * XDR RPC MESSAGE * * * * * * * * * * * * * * * */
/*
 * XDR the MSG_ACCEPTED part of a reply message union
 */
#[no_mangle]
#[c2rust::src_loc = "83:1"]
pub unsafe extern "C" fn gssrpc_xdr_accepted_reply(mut xdrs: *mut XDR,
                                                   mut ar:
                                                       *mut accepted_reply)
 -> libc::c_int {
    /* personalized union, rather than calling xdr_union */
    if gssrpc_xdr_opaque_auth(xdrs, &mut (*ar).ar_verf) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_enum(xdrs,
                       &mut (*ar).ar_stat as *mut accept_stat as
                           *mut libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    match (*ar).ar_stat as libc::c_uint {
        0 => {
            return ::std::mem::transmute::<_,
                                           fn(_: _, _: _)
                                               ->
                                                   libc::c_int>(Some((*ar).ru.AR_results.proc_0.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                                          (*ar).ru.AR_results.where_0)
        }
        2 => {
            if gssrpc_xdr_u_int32(xdrs, &mut (*ar).ru.AR_versions.low) == 0 {
                return 0 as libc::c_int
            }
            return gssrpc_xdr_u_int32(xdrs, &mut (*ar).ru.AR_versions.high)
        }
        4 | 5 | 3 | 1 | _ => { }
    }
    return 1 as libc::c_int;
    /* TRUE => open ended set of problems */
}
/* XDR the MSG_ACCEPTED part of a reply message union */
/* XDR the MSG_DENIED part of a reply message union */
/*
 * XDR the MSG_DENIED part of a reply message union
 */
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn gssrpc_xdr_rejected_reply(mut xdrs: *mut XDR,
                                                   mut rr:
                                                       *mut rejected_reply)
 -> libc::c_int {
    /* personalized union, rather than calling xdr_union */
    if gssrpc_xdr_enum(xdrs,
                       &mut (*rr).rj_stat as *mut reject_stat as
                           *mut libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    match (*rr).rj_stat as libc::c_uint {
        0 => {
            if gssrpc_xdr_u_int32(xdrs, &mut (*rr).ru.RJ_versions.low) == 0 {
                return 0 as libc::c_int
            }
            return gssrpc_xdr_u_int32(xdrs, &mut (*rr).ru.RJ_versions.high)
        }
        1 => {
            return gssrpc_xdr_enum(xdrs,
                                   &mut (*rr).ru.RJ_why as *mut auth_stat as
                                       *mut libc::c_int)
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "134:27"]
static mut reply_dscrm: [xdr_discrim; 3] =
    unsafe {
        [{
             let mut init =
                 xdr_discrim{value: MSG_ACCEPTED as libc::c_int,
                             proc_0:
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *mut XDR,
                                                                                     _:
                                                                                         *mut accepted_reply)
                                                                    ->
                                                                        libc::c_int>,
                                                         xdrproc_t>(Some(gssrpc_xdr_accepted_reply
                                                                             as
                                                                             unsafe extern "C" fn(_:
                                                                                                      *mut XDR,
                                                                                                  _:
                                                                                                      *mut accepted_reply)
                                                                                 ->
                                                                                     libc::c_int)),};
             init
         },
         {
             let mut init =
                 xdr_discrim{value: MSG_DENIED as libc::c_int,
                             proc_0:
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *mut XDR,
                                                                                     _:
                                                                                         *mut rejected_reply)
                                                                    ->
                                                                        libc::c_int>,
                                                         xdrproc_t>(Some(gssrpc_xdr_rejected_reply
                                                                             as
                                                                             unsafe extern "C" fn(_:
                                                                                                      *mut XDR,
                                                                                                  _:
                                                                                                      *mut rejected_reply)
                                                                                 ->
                                                                                     libc::c_int)),};
             init
         },
         {
             let mut init =
                 xdr_discrim{value: -(1 as libc::c_int), proc_0: None,};
             init
         }]
    };
/*
 * XDR a reply message
 */
#[no_mangle]
#[c2rust::src_loc = "142:1"]
pub unsafe extern "C" fn gssrpc_xdr_replymsg(mut xdrs: *mut XDR,
                                             mut rmsg: *mut rpc_msg)
 -> libc::c_int {
    if gssrpc_xdr_u_int32(xdrs, &mut (*rmsg).rm_xid) != 0 &&
           gssrpc_xdr_enum(xdrs,
                           &mut (*rmsg).rm_direction as *mut msg_type as
                               *mut libc::c_int) != 0 &&
           (*rmsg).rm_direction as libc::c_uint ==
               REPLY as libc::c_int as libc::c_uint {
        return gssrpc_xdr_union(xdrs,
                                &mut (*rmsg).ru.RM_rmb.rp_stat as
                                    *mut reply_stat as *mut libc::c_int,
                                &mut (*rmsg).ru.RM_rmb.ru as
                                    *mut C2RustUnnamed_0 as caddr_t,
                                reply_dscrm.as_mut_ptr(), None)
    }
    return 0 as libc::c_int;
}
/*
 * Serializes the "static part" of a call message header.
 * The fields include: rm_xid, rm_direction, rpcvers, prog, and vers.
 * The rm_xid is not really static, but the user can easily munge on the fly.
 */
#[no_mangle]
#[c2rust::src_loc = "160:1"]
pub unsafe extern "C" fn gssrpc_xdr_callhdr(mut xdrs: *mut XDR,
                                            mut cmsg: *mut rpc_msg)
 -> libc::c_int {
    (*cmsg).rm_direction = CALL;
    (*cmsg).ru.RM_cmb.cb_rpcvers = 2 as libc::c_int as uint32_t;
    if (*xdrs).x_op as libc::c_uint ==
           XDR_ENCODE as libc::c_int as libc::c_uint &&
           gssrpc_xdr_u_int32(xdrs, &mut (*cmsg).rm_xid) != 0 &&
           gssrpc_xdr_enum(xdrs,
                           &mut (*cmsg).rm_direction as *mut msg_type as
                               *mut libc::c_int) != 0 &&
           gssrpc_xdr_u_int32(xdrs, &mut (*cmsg).ru.RM_cmb.cb_rpcvers) != 0 &&
           gssrpc_xdr_u_int32(xdrs, &mut (*cmsg).ru.RM_cmb.cb_prog) != 0 {
        return gssrpc_xdr_u_int32(xdrs, &mut (*cmsg).ru.RM_cmb.cb_vers)
    }
    return 0 as libc::c_int;
}
/* ************************** Client utility routine ************* */
#[c2rust::src_loc = "178:1"]
unsafe extern "C" fn accepted(mut acpt_stat: accept_stat,
                              mut error: *mut rpc_err) {
    match acpt_stat as libc::c_uint {
        1 => { (*error).re_status = RPC_PROGUNAVAIL; return }
        2 => { (*error).re_status = RPC_PROGVERSMISMATCH; return }
        3 => { (*error).re_status = RPC_PROCUNAVAIL; return }
        4 => { (*error).re_status = RPC_CANTDECODEARGS; return }
        5 => { (*error).re_status = RPC_SYSTEMERROR; return }
        0 => { (*error).re_status = RPC_SUCCESS; return }
        _ => { }
    }
    /* something's wrong, but we don't know what ... */
    (*error).re_status = RPC_FAILED;
    (*error).ru.RE_lb.s1 = MSG_ACCEPTED as libc::c_int;
    (*error).ru.RE_lb.s2 = acpt_stat as int32_t;
}
#[c2rust::src_loc = "214:1"]
unsafe extern "C" fn rejected(mut rjct_stat: reject_stat,
                              mut error: *mut rpc_err) {
    match rjct_stat as libc::c_uint {
        0 => { (*error).re_status = RPC_VERSMISMATCH; return }
        1 => { (*error).re_status = RPC_AUTHERROR; return }
        _ => { }
    }
    /* something's wrong, but we don't know what ... */
    (*error).re_status = RPC_FAILED;
    (*error).ru.RE_lb.s1 = MSG_DENIED as libc::c_int;
    (*error).ru.RE_lb.s2 = rjct_stat as int32_t;
}
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
/*
 * given a reply message, fills in the error
 */
#[no_mangle]
#[c2rust::src_loc = "237:1"]
pub unsafe extern "C" fn gssrpc__seterr_reply(mut msg: *mut rpc_msg,
                                              mut error: *mut rpc_err) {
    /* optimized for normal, SUCCESSful case */
    match (*msg).ru.RM_rmb.rp_stat as libc::c_uint {
        0 => {
            if (*msg).ru.RM_rmb.ru.RP_ar.ar_stat as libc::c_uint ==
                   SUCCESS as libc::c_int as libc::c_uint {
                (*error).re_status = RPC_SUCCESS;
                return
            }
            accepted((*msg).ru.RM_rmb.ru.RP_ar.ar_stat, error);
        }
        1 => { rejected((*msg).ru.RM_rmb.ru.RP_dr.rj_stat, error); }
        _ => {
            (*error).re_status = RPC_FAILED;
            (*error).ru.RE_lb.s1 = (*msg).ru.RM_rmb.rp_stat as int32_t
        }
    }
    match (*error).re_status as libc::c_uint {
        6 => {
            (*error).ru.RE_vers.low =
                (*msg).ru.RM_rmb.ru.RP_dr.ru.RJ_versions.low;
            (*error).ru.RE_vers.high =
                (*msg).ru.RM_rmb.ru.RP_dr.ru.RJ_versions.high
        }
        7 => { (*error).ru.RE_why = (*msg).ru.RM_rmb.ru.RP_dr.ru.RJ_why }
        9 => {
            (*error).ru.RE_vers.low =
                (*msg).ru.RM_rmb.ru.RP_ar.ru.AR_versions.low;
            (*error).ru.RE_vers.high =
                (*msg).ru.RM_rmb.ru.RP_ar.ru.AR_versions.high
        }
        16 | 0 | 15 | 14 | 17 | 13 | 12 | 11 | 10 | 8 | 5 | 4 | 3 | 2 | 1 | _
        => {
        }
    };
}
