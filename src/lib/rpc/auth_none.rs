use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:44"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:44"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:44"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:44"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
    use super::stdint_intn_h::int32_t;
    /* !defined(GSSRPC_TYPES_H) */
    /*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
    /* XXX namespace */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:45"]
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
    extern "C" {
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
        #[c2rust::src_loc = "194:1"]
        pub fn gssrpc_xdr_opaque_auth(_: *mut XDR, _: *mut opaque_auth)
         -> libc::c_int;
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
#[c2rust::header_src = "/usr/include/stdlib.h:44"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
pub use self::types_h::{__u_int, __int32_t, __caddr_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdrmem_create};
pub use self::auth_h::{des_block, opaque_auth, AUTH, auth_ops, rpc_msg,
                       gssrpc__null_auth, gssrpc_xdr_opaque_auth};
use self::stdlib_h::calloc;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "70:15"]
pub struct authnone_private {
    pub no_client: AUTH,
    pub marshalled_client: [libc::c_char; 20],
    pub mcnt: u_int,
}
#[c2rust::src_loc = "60:24"]
static mut ops: auth_ops =
    unsafe {
        {
            let mut init =
                auth_ops{ah_nextverf:
                             Some(authnone_verf as
                                      unsafe extern "C" fn(_: *mut AUTH)
                                          -> ()),
                         ah_marshal:
                             Some(authnone_marshal as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR)
                                          -> libc::c_int),
                         ah_validate:
                             Some(authnone_validate as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _:
                                                               *mut opaque_auth)
                                          -> libc::c_int),
                         ah_refresh:
                             Some(authnone_refresh as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut rpc_msg)
                                          -> libc::c_int),
                         ah_destroy:
                             Some(authnone_destroy as
                                      unsafe extern "C" fn(_: *mut AUTH)
                                          -> ()),
                         ah_wrap:
                             Some(authnone_wrap as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR,
                                                           _: xdrproc_t,
                                                           _: caddr_t)
                                          -> libc::c_int),
                         ah_unwrap:
                             Some(authnone_wrap as
                                      unsafe extern "C" fn(_: *mut AUTH,
                                                           _: *mut XDR,
                                                           _: xdrproc_t,
                                                           _: caddr_t)
                                          -> libc::c_int),};
            init
        }
    };
#[c2rust::src_loc = "74:4"]
static mut authnone_private: *mut authnone_private =
    0 as *const authnone_private as *mut authnone_private;
/* takes no parameters */
#[no_mangle]
#[c2rust::src_loc = "76:1"]
pub unsafe extern "C" fn gssrpc_authnone_create() -> *mut AUTH {
    let mut ap: *mut authnone_private = authnone_private;
    let mut xdr_stream: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut xdrs: *mut XDR = 0 as *mut XDR;
    if ap.is_null() {
        ap =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<authnone_private>() as libc::c_ulong)
                as *mut authnone_private;
        if ap.is_null() { return 0 as *mut AUTH }
        authnone_private = ap
    }
    if (*ap).mcnt == 0 {
        (*ap).no_client.ah_verf = gssrpc__null_auth;
        (*ap).no_client.ah_cred = (*ap).no_client.ah_verf;
        (*ap).no_client.ah_ops = &mut ops;
        xdrs = &mut xdr_stream;
        gssrpc_xdrmem_create(xdrs, (*ap).marshalled_client.as_mut_ptr(),
                             20 as libc::c_int as u_int, XDR_ENCODE);
        gssrpc_xdr_opaque_auth(xdrs, &mut (*ap).no_client.ah_cred);
        gssrpc_xdr_opaque_auth(xdrs, &mut (*ap).no_client.ah_verf);
        (*ap).mcnt =
            Some((*(*xdrs).x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(xdrs);
        if (*(*xdrs).x_ops).x_destroy.is_some() {
            Some((*(*xdrs).x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(xdrs);
        }
    }
    return &mut (*ap).no_client;
}
/*ARGSUSED*/
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn authnone_marshal(mut client: *mut AUTH,
                                      mut xdrs: *mut XDR) -> libc::c_int {
    let mut ap: *mut authnone_private = authnone_private;
    if ap.is_null() { return 0 as libc::c_int }
    return Some((*(*xdrs).x_ops).x_putbytes.expect("non-null function pointer")).expect("non-null function pointer")(xdrs,
                                                                                                                     (*ap).marshalled_client.as_mut_ptr(),
                                                                                                                     (*ap).mcnt);
}
/*
 * Authenticator operations routines
 */
/*ARGSUSED*/
#[c2rust::src_loc = "116:1"]
unsafe extern "C" fn authnone_verf(mut auth: *mut AUTH) { }
/*ARGSUSED*/
#[c2rust::src_loc = "122:1"]
unsafe extern "C" fn authnone_validate(mut auth: *mut AUTH,
                                       mut verf: *mut opaque_auth)
 -> libc::c_int {
    return 1 as libc::c_int;
}
/*ARGSUSED*/
#[c2rust::src_loc = "130:1"]
unsafe extern "C" fn authnone_refresh(mut auth: *mut AUTH,
                                      mut msg: *mut rpc_msg) -> libc::c_int {
    return 0 as libc::c_int;
}
/*ARGSUSED*/
#[c2rust::src_loc = "138:1"]
unsafe extern "C" fn authnone_destroy(mut auth: *mut AUTH) { }
#[c2rust::src_loc = "143:1"]
unsafe extern "C" fn authnone_wrap(mut auth: *mut AUTH, mut xdrs: *mut XDR,
                                   mut xfunc: xdrproc_t, mut xwhere: caddr_t)
 -> libc::c_int {
    return ::std::mem::transmute::<_,
                                   fn(_: _, _: _)
                                       ->
                                           libc::c_int>(Some(xfunc.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                             xwhere);
}
