use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:42"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:42"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:42"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:42"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/sys/types.h:46"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __u_long, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:46"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:46"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:46"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "89:1"]
    pub type rpcvers_t = uint32_t;
    #[c2rust::src_loc = "91:1"]
    pub type rpcproc_t = uint32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:47"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:47"]
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
    extern "C" {
        /* not to exceed MAX_AUTH_BYTES */
        /*
 * Auth handle, interface to client side authenticators.
 */
        #[c2rust::src_loc = "96:8"]
        pub type rpc_msg;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:48"]
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
        pub ru: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:2"]
    pub union C2RustUnnamed {
        pub RE_errno: libc::c_int,
        pub RE_why: auth_stat,
        pub RE_vers: C2RustUnnamed_1,
        pub RE_lb: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "99:3"]
    pub struct C2RustUnnamed_0 {
        pub s1: int32_t,
        pub s2: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:3"]
    pub struct C2RustUnnamed_1 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
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
#[c2rust::header_src = "/usr/include/stdio.h:42"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:43"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "132:14"]
        pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:46"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_int, __u_long, __int32_t, __uint32_t, __off_t,
                        __off64_t, __time_t, __suseconds_t, __caddr_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::sys_types_h::{u_int, u_long, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::struct_timeval_h::timeval;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::{rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, des_block,
                       opaque_auth, AUTH, auth_ops, rpc_msg};
pub use self::clnt_h::{clnt_stat, RPC_FAILED, RPC_PROGNOTREGISTERED,
                       RPC_PMAPFAILURE, RPC_UNKNOWNPROTO, RPC_UNKNOWNHOST,
                       RPC_SYSTEMERROR, RPC_CANTDECODEARGS, RPC_PROCUNAVAIL,
                       RPC_PROGVERSMISMATCH, RPC_PROGUNAVAIL, RPC_AUTHERROR,
                       RPC_VERSMISMATCH, RPC_TIMEDOUT, RPC_CANTRECV,
                       RPC_CANTSEND, RPC_CANTDECODERES, RPC_CANTENCODEARGS,
                       RPC_SUCCESS, rpc_err, C2RustUnnamed, C2RustUnnamed_0,
                       C2RustUnnamed_1, CLIENT, clnt_ops,
                       gssrpc_rpc_createrr};
use self::stdio_h::{stderr, snprintf, fprintf};
use self::string_h::{strncpy, strncat, strlen, strerror};
use self::stdlib_h::malloc;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "185:8"]
pub struct rpc_errtab {
    pub status: clnt_stat,
    pub message: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "315:8"]
pub struct auth_errtab {
    pub status: auth_stat,
    pub message: *mut libc::c_char,
}
#[c2rust::src_loc = "62:14"]
static mut buf: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "64:1"]
unsafe extern "C" fn get_buf() -> *mut libc::c_char {
    if buf.is_null() {
        buf =
            malloc(8192 as libc::c_int as libc::c_ulong) as *mut libc::c_char
    }
    return buf;
}
/* stderr */
/*
 * Print reply error info
 */
#[no_mangle]
#[c2rust::src_loc = "75:1"]
pub unsafe extern "C" fn gssrpc_clnt_sperror(mut rpch: *mut CLIENT,
                                             mut s: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut e: rpc_err =
        rpc_err{re_status: RPC_SUCCESS, ru: C2RustUnnamed{RE_errno: 0,},};
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufstart: *mut libc::c_char = get_buf();
    let mut str: *mut libc::c_char = bufstart;
    let mut strstart: *mut libc::c_char = str;
    let mut strend: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() { return 0 as *mut libc::c_char }
    strend = str.offset(8192 as libc::c_int as isize);
    Some((*(*rpch).cl_ops).cl_geterr.expect("non-null function pointer")).expect("non-null function pointer")(rpch,
                                                                                                              &mut e);
    strncpy(str, s,
            (8192 as libc::c_int - 1 as libc::c_int) as libc::c_ulong);
    *str.offset((8192 as libc::c_int - 1 as libc::c_int) as isize) =
        0 as libc::c_int as libc::c_char;
    strncat(str, b": \x00" as *const u8 as *const libc::c_char,
            ((8192 as libc::c_int - 1 as libc::c_int) as
                 libc::c_ulong).wrapping_sub(strlen(bufstart)));
    str = str.offset(strlen(str) as isize);
    strncat(str, gssrpc_clnt_sperrno(e.re_status),
            ((8192 as libc::c_int - 1 as libc::c_int) as
                 libc::c_ulong).wrapping_sub(strlen(bufstart)));
    *strstart.offset((8192 as libc::c_int - 1 as libc::c_int) as isize) =
        '\u{0}' as i32 as libc::c_char;
    str = str.offset(strlen(str) as isize);
    match e.re_status as libc::c_uint {
        0 | 1 | 2 | 5 | 8 | 10 | 11 | 12 | 13 | 17 | 14 | 15 | 16 => { }
        3 | 4 => {
            /* 10 for the string */
            if ((str.wrapping_offset_from(bufstart) as libc::c_long +
                     10 as libc::c_int as libc::c_long) as
                    libc::c_ulong).wrapping_add(strlen(strerror(e.ru.RE_errno)))
                   < 8192 as libc::c_int as libc::c_ulong {
                snprintf(str,
                         strend.wrapping_offset_from(str) as libc::c_long as
                             libc::c_ulong,
                         b"; errno = %s\x00" as *const u8 as
                             *const libc::c_char, strerror(e.ru.RE_errno));
            }
            str = str.offset(strlen(str) as isize)
        }
        6 => {
            /* 33 for the string, 22 for the numbers */
            if (str.wrapping_offset_from(bufstart) as libc::c_long +
                    33 as libc::c_int as libc::c_long +
                    22 as libc::c_int as libc::c_long) <
                   8192 as libc::c_int as libc::c_long {
                snprintf(str,
                         strend.wrapping_offset_from(str) as libc::c_long as
                             libc::c_ulong,
                         b"; low version = %lu, high version = %lu\x00" as
                             *const u8 as *const libc::c_char,
                         e.ru.RE_vers.low as u_long,
                         e.ru.RE_vers.high as u_long);
            }
            str = str.offset(strlen(str) as isize)
        }
        7 => {
            err = auth_errmsg(e.ru.RE_why);
            /* 8 for the string */
            if (str.wrapping_offset_from(bufstart) as libc::c_long +
                    8 as libc::c_int as libc::c_long) <
                   8192 as libc::c_int as libc::c_long {
                snprintf(str,
                         strend.wrapping_offset_from(str) as libc::c_long as
                             libc::c_ulong,
                         b"; why = \x00" as *const u8 as *const libc::c_char);
            }
            str = str.offset(strlen(str) as isize);
            if !err.is_null() {
                if (str.wrapping_offset_from(bufstart) as libc::c_long as
                        libc::c_ulong).wrapping_add(strlen(err)) <
                       8192 as libc::c_int as libc::c_ulong {
                    snprintf(str,
                             strend.wrapping_offset_from(str) as libc::c_long
                                 as libc::c_ulong,
                             b"%s\x00" as *const u8 as *const libc::c_char,
                             err);
                }
            } else if (str.wrapping_offset_from(bufstart) as libc::c_long +
                           33 as libc::c_int as libc::c_long +
                           11 as libc::c_int as libc::c_long) <
                          8192 as libc::c_int as libc::c_long {
                snprintf(str,
                         strend.wrapping_offset_from(str) as libc::c_long as
                             libc::c_ulong,
                         b"(unknown authentication error - %d)\x00" as
                             *const u8 as *const libc::c_char,
                         e.ru.RE_why as libc::c_int);
            }
            str = str.offset(strlen(str) as isize)
        }
        9 => {
            /* 33 for the string, 11 for the number */
            /* 33 for the string, 22 for the numbers */
            if (str.wrapping_offset_from(bufstart) as libc::c_long +
                    33 as libc::c_int as libc::c_long +
                    22 as libc::c_int as libc::c_long) <
                   8192 as libc::c_int as libc::c_long {
                snprintf(str,
                         strend.wrapping_offset_from(str) as libc::c_long as
                             libc::c_ulong,
                         b"; low version = %lu, high version = %lu\x00" as
                             *const u8 as *const libc::c_char,
                         e.ru.RE_vers.low as u_long,
                         e.ru.RE_vers.high as u_long);
            }
            str = str.offset(strlen(str) as isize)
        }
        _ => {
            /* unknown */
            /* 14 for the string, 22 for the numbers */
            if (str.wrapping_offset_from(bufstart) as libc::c_long +
                    14 as libc::c_int as libc::c_long +
                    22 as libc::c_int as libc::c_long) <
                   8192 as libc::c_int as libc::c_long {
                snprintf(str,
                         strend.wrapping_offset_from(str) as libc::c_long as
                             libc::c_ulong,
                         b"; s1 = %lu, s2 = %lu\x00" as *const u8 as
                             *const libc::c_char, e.ru.RE_lb.s1 as u_long,
                         e.ru.RE_lb.s2 as u_long);
            }
            str = str.offset(strlen(str) as isize)
        }
    }
    if (str.wrapping_offset_from(bufstart) as libc::c_long +
            1 as libc::c_int as libc::c_long) <
           8192 as libc::c_int as libc::c_long {
        snprintf(str,
                 strend.wrapping_offset_from(str) as libc::c_long as
                     libc::c_ulong,
                 b"\n\x00" as *const u8 as *const libc::c_char);
    }
    return strstart;
}
/* stderr */
/*
 * Print an English error message, given the client error code
 */
#[no_mangle]
#[c2rust::src_loc = "178:1"]
pub unsafe extern "C" fn gssrpc_clnt_perror(mut rpch: *mut CLIENT,
                                            mut s: *mut libc::c_char) {
    fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
            gssrpc_clnt_sperror(rpch, s));
}
#[c2rust::src_loc = "190:27"]
static mut rpc_errlist: [rpc_errtab; 18] =
    [{
         let mut init =
             rpc_errtab{status: RPC_SUCCESS,
                        message:
                            b"RPC: Success\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_CANTENCODEARGS,
                        message:
                            b"RPC: Can\'t encode arguments\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_CANTDECODERES,
                        message:
                            b"RPC: Can\'t decode result\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_CANTSEND,
                        message:
                            b"RPC: Unable to send\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_CANTRECV,
                        message:
                            b"RPC: Unable to receive\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_TIMEDOUT,
                        message:
                            b"RPC: Timed out\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_VERSMISMATCH,
                        message:
                            b"RPC: Incompatible versions of RPC\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_AUTHERROR,
                        message:
                            b"RPC: Authentication error\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_PROGUNAVAIL,
                        message:
                            b"RPC: Program unavailable\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_PROGVERSMISMATCH,
                        message:
                            b"RPC: Program/version mismatch\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_PROCUNAVAIL,
                        message:
                            b"RPC: Procedure unavailable\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_CANTDECODEARGS,
                        message:
                            b"RPC: Server can\'t decode arguments\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_SYSTEMERROR,
                        message:
                            b"RPC: Remote system error\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_UNKNOWNHOST,
                        message:
                            b"RPC: Unknown host\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_UNKNOWNPROTO,
                        message:
                            b"RPC: Unknown protocol\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_PMAPFAILURE,
                        message:
                            b"RPC: Port mapper failure\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_PROGNOTREGISTERED,
                        message:
                            b"RPC: Program not registered\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             rpc_errtab{status: RPC_FAILED,
                        message:
                            b"RPC: Failed (unspecified error)\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char,};
         init
     }];
/*
 * Copy error message to buffer.
 */
/*
 * This interface for use by clntrpc
 */
#[no_mangle]
#[c2rust::src_loc = "233:1"]
pub unsafe extern "C" fn gssrpc_clnt_sperrno(mut stat: clnt_stat)
 -> *mut libc::c_char {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[rpc_errtab; 18]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<rpc_errtab>()
                                                   as libc::c_ulong) {
        if rpc_errlist[i as usize].status as libc::c_uint ==
               stat as libc::c_uint {
            return rpc_errlist[i as usize].message
        }
        i = i.wrapping_add(1)
    }
    return b"RPC: (unknown error code)\x00" as *const u8 as
               *const libc::c_char as *mut libc::c_char;
}
/* string */
/*
 * Like clnt_perror(), but is more verbose in its output
 */
#[no_mangle]
#[c2rust::src_loc = "246:1"]
pub unsafe extern "C" fn gssrpc_clnt_perrno(mut num: clnt_stat) {
    fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
            gssrpc_clnt_sperrno(num));
}
/* stderr */
#[no_mangle]
#[c2rust::src_loc = "253:1"]
pub unsafe extern "C" fn gssrpc_clnt_spcreateerror(mut s: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut str: *mut libc::c_char = get_buf();
    let mut strend: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() { return 0 as *mut libc::c_char }
    strend = str.offset(8192 as libc::c_int as isize);
    snprintf(str,
             strend.wrapping_offset_from(str) as libc::c_long as
                 libc::c_ulong,
             b"%s: \x00" as *const u8 as *const libc::c_char, s);
    *str.offset((8192 as libc::c_int - 1 as libc::c_int) as isize) =
        '\u{0}' as i32 as libc::c_char;
    strncat(str, gssrpc_clnt_sperrno(gssrpc_rpc_createrr.cf_stat),
            (8192 as libc::c_int - 1 as libc::c_int) as libc::c_ulong);
    match gssrpc_rpc_createrr.cf_stat as libc::c_uint {
        14 => {
            strncat(str, b" - \x00" as *const u8 as *const libc::c_char,
                    ((8192 as libc::c_int - 1 as libc::c_int) as
                         libc::c_ulong).wrapping_sub(strlen(str)));
            strncat(str,
                    gssrpc_clnt_sperrno(gssrpc_rpc_createrr.cf_error.re_status),
                    ((8192 as libc::c_int - 1 as libc::c_int) as
                         libc::c_ulong).wrapping_sub(strlen(str)));
        }
        12 => {
            strncat(str, b" - \x00" as *const u8 as *const libc::c_char,
                    ((8192 as libc::c_int - 1 as libc::c_int) as
                         libc::c_ulong).wrapping_sub(strlen(str)));
            let mut m: *const libc::c_char =
                strerror(gssrpc_rpc_createrr.cf_error.ru.RE_errno);
            if !m.is_null() {
                strncat(str, m,
                        ((8192 as libc::c_int - 1 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen(str)));
            } else {
                snprintf(&mut *str.offset((strlen as
                                               unsafe extern "C" fn(_:
                                                                        *const libc::c_char)
                                                   -> libc::c_ulong)(str) as
                                              isize) as *mut libc::c_char,
                         (8192 as libc::c_int as
                              libc::c_ulong).wrapping_sub(strlen(str)),
                         b"Error %d\x00" as *const u8 as *const libc::c_char,
                         gssrpc_rpc_createrr.cf_error.ru.RE_errno);
            }
        }
        3 | 2 | 1 | 0 | 17 | 15 | 16 | 13 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 |
        _ => {
        }
    }
    strncat(str, b"\n\x00" as *const u8 as *const libc::c_char,
            ((8192 as libc::c_int - 1 as libc::c_int) as
                 libc::c_ulong).wrapping_sub(strlen(str)));
    return str;
}
/*
 * Print why creation failed
 */
#[no_mangle]
#[c2rust::src_loc = "309:1"]
pub unsafe extern "C" fn gssrpc_clnt_pcreateerror(mut s: *mut libc::c_char) {
    fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
            gssrpc_clnt_spcreateerror(s));
}
#[c2rust::src_loc = "320:27"]
static mut auth_errlist: [auth_errtab; 8] =
    [{
         let mut init =
             auth_errtab{status: AUTH_OK,
                         message:
                             b"Authentication OK\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             auth_errtab{status: AUTH_BADCRED,
                         message:
                             b"Invalid client credential\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             auth_errtab{status: AUTH_REJECTEDCRED,
                         message:
                             b"Server rejected credential\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             auth_errtab{status: AUTH_BADVERF,
                         message:
                             b"Invalid client verifier\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             auth_errtab{status: AUTH_REJECTEDVERF,
                         message:
                             b"Server rejected verifier\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             auth_errtab{status: AUTH_TOOWEAK,
                         message:
                             b"Client credential too weak\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             auth_errtab{status: AUTH_INVALIDRESP,
                         message:
                             b"Invalid server verifier\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             auth_errtab{status: AUTH_FAILED,
                         message:
                             b"Failed (unspecified error)\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,};
         init
     }];
/* @(#)clnt_perror.c	2.1 88/07/29 4.0 RPCSRC */
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
 * clnt_perror.c
 */
/* HAVE_STRERROR */
#[c2rust::src_loc = "339:1"]
unsafe extern "C" fn auth_errmsg(mut stat: auth_stat) -> *mut libc::c_char {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[auth_errtab; 8]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<auth_errtab>()
                                                   as libc::c_ulong) {
        if auth_errlist[i as usize].status as libc::c_uint ==
               stat as libc::c_uint {
            return auth_errlist[i as usize].message
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut libc::c_char;
}
