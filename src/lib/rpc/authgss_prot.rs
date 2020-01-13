use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:37"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:37"]
pub mod types_h {
    #[c2rust::src_loc = "31:1"]
    pub type __u_char = libc::c_uchar;
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:38"]
pub mod sys_types_h {
    #[c2rust::src_loc = "33:1"]
    pub type u_char = __u_char;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_char, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:38"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:41"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:41"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:41"]
pub mod xdr_h {
    #[c2rust::src_loc = "81:1"]
    pub type xdr_op = libc::c_uint;
    #[c2rust::src_loc = "84:2"]
    pub const XDR_FREE: xdr_op = 2;
    #[c2rust::src_loc = "83:2"]
    pub const XDR_DECODE: xdr_op = 1;
    #[c2rust::src_loc = "82:2"]
    pub const XDR_ENCODE: xdr_op = 0;
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
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "251:1"]
        pub fn gssrpc_xdr_void(_: *mut XDR, _: *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "253:1"]
        pub fn gssrpc_xdr_u_int(_: *mut XDR, _: *mut u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "259:1"]
        pub fn gssrpc_xdr_enum(_: *mut XDR, _: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "262:1"]
        pub fn gssrpc_xdr_bytes(_: *mut XDR, _: *mut *mut libc::c_char,
                                _: *mut u_int, _: u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "298:1"]
        pub fn gssrpc_xdr_u_int32(_: *mut XDR, _: *mut uint32_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "306:1"]
        pub fn gssrpc_xdralloc_create(_: *mut XDR, _: xdr_op);
        #[no_mangle]
        #[c2rust::src_loc = "315:1"]
        pub fn gssrpc_xdrmem_create(_: *mut XDR, _: caddr_t, _: u_int,
                                    _: xdr_op);
        #[no_mangle]
        #[c2rust::src_loc = "312:1"]
        pub fn gssrpc_xdralloc_getdata(_: *mut XDR) -> caddr_t;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:41"]
pub mod gssapi_h {
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
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    /*
 * For now, define a QOP-type as an OM_uint32 (pending resolution of ongoing
 * discussions).
 */
    #[c2rust::src_loc = "134:1"]
    pub type gss_qop_t = OM_uint32;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* time_rec */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "489:1"]
        pub fn gss_get_mic(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_qop_t,
                           _: gss_buffer_t, _: gss_buffer_t) -> OM_uint32;
        /* message_token */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn gss_verify_mic(_: *mut OM_uint32, _: gss_ctx_id_t,
                              _: gss_buffer_t, _: gss_buffer_t,
                              _: *mut gss_qop_t) -> OM_uint32;
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "508:1"]
        pub fn gss_wrap(_: *mut OM_uint32, _: gss_ctx_id_t, _: libc::c_int,
                        _: gss_qop_t, _: gss_buffer_t, _: *mut libc::c_int,
                        _: gss_buffer_t) -> OM_uint32;
        /* output_message_buffer */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "520:1"]
        pub fn gss_unwrap(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_buffer_t,
                          _: gss_buffer_t, _: *mut libc::c_int,
                          _: *mut gss_qop_t) -> OM_uint32;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gss.h:41"]
pub mod auth_gss_h {
    #[c2rust::src_loc = "51:9"]
    pub type rpc_gss_proc_t = libc::c_uint;
    #[c2rust::src_loc = "55:2"]
    pub const RPCSEC_GSS_DESTROY: rpc_gss_proc_t = 3;
    #[c2rust::src_loc = "54:2"]
    pub const RPCSEC_GSS_CONTINUE_INIT: rpc_gss_proc_t = 2;
    #[c2rust::src_loc = "53:2"]
    pub const RPCSEC_GSS_INIT: rpc_gss_proc_t = 1;
    #[c2rust::src_loc = "52:2"]
    pub const RPCSEC_GSS_DATA: rpc_gss_proc_t = 0;
    #[c2rust::src_loc = "59:9"]
    pub type rpc_gss_svc_t = libc::c_uint;
    #[c2rust::src_loc = "62:2"]
    pub const RPCSEC_GSS_SVC_PRIVACY: rpc_gss_svc_t = 3;
    #[c2rust::src_loc = "61:2"]
    pub const RPCSEC_GSS_SVC_INTEGRITY: rpc_gss_svc_t = 2;
    #[c2rust::src_loc = "60:2"]
    pub const RPCSEC_GSS_SVC_NONE: rpc_gss_svc_t = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:8"]
    pub struct rpc_gss_cred {
        pub gc_v: u_int,
        pub gc_proc: rpc_gss_proc_t,
        pub gc_seq: uint32_t,
        pub gc_svc: rpc_gss_svc_t,
        pub gc_ctx: gss_buffer_desc,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rpc_gss_init_res {
        pub gr_ctx: gss_buffer_desc,
        pub gr_major: uint32_t,
        pub gr_minor: uint32_t,
        pub gr_win: uint32_t,
        pub gr_token: gss_buffer_desc,
    }
    use super::sys_types_h::u_int;
    use super::stdint_uintn_h::uint32_t;
    use super::gssapi_h::gss_buffer_desc;
    /* !defined(GSSRPC_AUTH_GSS_H) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:38"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:40"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_char, __u_int, __int32_t, __uint32_t, __caddr_t};
pub use self::sys_types_h::{u_char, u_int, caddr_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdr_u_int,
                      gssrpc_xdr_enum, gssrpc_xdr_bytes, gssrpc_xdr_u_int32,
                      gssrpc_xdralloc_create, gssrpc_xdrmem_create,
                      gssrpc_xdralloc_getdata};
pub use self::gssapi_h::{gss_ctx_id_t, gss_uint32, OM_uint32,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_qop_t, gss_ctx_id_struct,
                         gss_get_mic, gss_verify_mic, gss_wrap, gss_unwrap,
                         gss_release_buffer};
pub use self::auth_gss_h::{rpc_gss_proc_t, RPCSEC_GSS_DESTROY,
                           RPCSEC_GSS_CONTINUE_INIT, RPCSEC_GSS_INIT,
                           RPCSEC_GSS_DATA, rpc_gss_svc_t,
                           RPCSEC_GSS_SVC_PRIVACY, RPCSEC_GSS_SVC_INTEGRITY,
                           RPCSEC_GSS_SVC_NONE, rpc_gss_cred,
                           rpc_gss_init_res};
use self::stdlib_h::free;
use self::string_h::memset;
/* lib/rpc/authgss_prot.c */
/*
  Copyright (c) 2000 The Regents of the University of Michigan.
  All rights reserved.

  Copyright (c) 2000 Dug Song <dugsong@UMICH.EDU>.
  All rights reserved, all wrongs reversed.

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions
  are met:

  1. Redistributions of source code must retain the above copyright
     notice, this list of conditions and the following disclaimer.
  2. Redistributions in binary form must reproduce the above copyright
     notice, this list of conditions and the following disclaimer in the
     documentation and/or other materials provided with the distribution.
  3. Neither the name of the University nor the names of its
     contributors may be used to endorse or promote products derived
     from this software without specific prior written permission.

  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
  DISCLAIMED. IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
  CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
  SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
  BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
  LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
  NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
  SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

  Id: authgss_prot.c,v 1.18 2000/09/01 04:14:03 dugsong Exp
*/
#[no_mangle]
#[c2rust::src_loc = "48:1"]
pub unsafe extern "C" fn gssrpc_xdr_rpc_gss_buf(mut xdrs: *mut XDR,
                                                mut buf: gss_buffer_t,
                                                mut maxsize: u_int)
 -> libc::c_int {
    let mut xdr_stat: libc::c_int = 0;
    let mut tmplen: u_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*xdrs).x_op as libc::c_uint !=
           XDR_DECODE as libc::c_int as libc::c_uint {
        if (*buf).length >
               (2147483647 as libc::c_int as
                    libc::c_uint).wrapping_mul(2 as
                                                   libc::c_uint).wrapping_add(1
                                                                                  as
                                                                                  libc::c_uint)
                   as libc::c_ulong {
            return 0 as libc::c_int
        } else { tmplen = (*buf).length as u_int }
    }
    cp = (*buf).value as *mut libc::c_char;
    xdr_stat = gssrpc_xdr_bytes(xdrs, &mut cp, &mut tmplen, maxsize);
    (*buf).value = cp as *mut libc::c_void;
    if xdr_stat != 0 &&
           (*xdrs).x_op as libc::c_uint ==
               XDR_DECODE as libc::c_int as libc::c_uint {
        (*buf).length = tmplen as size_t
    }
    return xdr_stat;
}
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub unsafe extern "C" fn gssrpc_xdr_rpc_gss_cred(mut xdrs: *mut XDR,
                                                 mut p: *mut rpc_gss_cred)
 -> libc::c_int {
    let mut xdr_stat: libc::c_int = 0;
    xdr_stat =
        (gssrpc_xdr_u_int(xdrs, &mut (*p).gc_v) != 0 &&
             gssrpc_xdr_enum(xdrs,
                             &mut (*p).gc_proc as *mut rpc_gss_proc_t as
                                 *mut libc::c_int) != 0 &&
             gssrpc_xdr_u_int32(xdrs, &mut (*p).gc_seq) != 0 &&
             gssrpc_xdr_enum(xdrs,
                             &mut (*p).gc_svc as *mut rpc_gss_svc_t as
                                 *mut libc::c_int) != 0 &&
             gssrpc_xdr_rpc_gss_buf(xdrs, &mut (*p).gc_ctx,
                                    400 as libc::c_int as u_int) != 0) as
            libc::c_int;
    gssrpc_log_debug(b"xdr_rpc_gss_cred: %s %s (v %d, proc %d, seq %d, svc %d, ctx %p:%d)\x00"
                         as *const u8 as *const libc::c_char,
                     if (*xdrs).x_op as libc::c_uint ==
                            XDR_ENCODE as libc::c_int as libc::c_uint {
                         b"encode\x00" as *const u8 as *const libc::c_char
                     } else {
                         b"decode\x00" as *const u8 as *const libc::c_char
                     },
                     if xdr_stat == 1 as libc::c_int {
                         b"success\x00" as *const u8 as *const libc::c_char
                     } else {
                         b"failure\x00" as *const u8 as *const libc::c_char
                     }, (*p).gc_v, (*p).gc_proc as libc::c_uint, (*p).gc_seq,
                     (*p).gc_svc as libc::c_uint, (*p).gc_ctx.value,
                     (*p).gc_ctx.length);
    return xdr_stat;
}
#[no_mangle]
#[c2rust::src_loc = "92:1"]
pub unsafe extern "C" fn gssrpc_xdr_rpc_gss_init_args(mut xdrs: *mut XDR,
                                                      mut p:
                                                          *mut gss_buffer_desc)
 -> libc::c_int {
    let mut xdr_stat: libc::c_int = 0;
    xdr_stat = gssrpc_xdr_rpc_gss_buf(xdrs, p, 2048 as libc::c_int as u_int);
    gssrpc_log_debug(b"xdr_rpc_gss_init_args: %s %s (token %p:%d)\x00" as
                         *const u8 as *const libc::c_char,
                     if (*xdrs).x_op as libc::c_uint ==
                            XDR_ENCODE as libc::c_int as libc::c_uint {
                         b"encode\x00" as *const u8 as *const libc::c_char
                     } else {
                         b"decode\x00" as *const u8 as *const libc::c_char
                     },
                     if xdr_stat == 1 as libc::c_int {
                         b"success\x00" as *const u8 as *const libc::c_char
                     } else {
                         b"failure\x00" as *const u8 as *const libc::c_char
                     }, (*p).value, (*p).length);
    return xdr_stat;
}
#[no_mangle]
#[c2rust::src_loc = "107:1"]
pub unsafe extern "C" fn gssrpc_xdr_rpc_gss_init_res(mut xdrs: *mut XDR,
                                                     mut p:
                                                         *mut rpc_gss_init_res)
 -> libc::c_int {
    let mut xdr_stat: libc::c_int = 0;
    xdr_stat =
        (gssrpc_xdr_rpc_gss_buf(xdrs, &mut (*p).gr_ctx,
                                2048 as libc::c_int as u_int) != 0 &&
             gssrpc_xdr_u_int32(xdrs, &mut (*p).gr_major) != 0 &&
             gssrpc_xdr_u_int32(xdrs, &mut (*p).gr_minor) != 0 &&
             gssrpc_xdr_u_int32(xdrs, &mut (*p).gr_win) != 0 &&
             gssrpc_xdr_rpc_gss_buf(xdrs, &mut (*p).gr_token,
                                    2048 as libc::c_int as u_int) != 0) as
            libc::c_int;
    gssrpc_log_debug(b"xdr_rpc_gss_init_res %s %s (ctx %p:%d, maj %d, min %d, win %d, token %p:%d)\x00"
                         as *const u8 as *const libc::c_char,
                     if (*xdrs).x_op as libc::c_uint ==
                            XDR_ENCODE as libc::c_int as libc::c_uint {
                         b"encode\x00" as *const u8 as *const libc::c_char
                     } else {
                         b"decode\x00" as *const u8 as *const libc::c_char
                     },
                     if xdr_stat == 1 as libc::c_int {
                         b"success\x00" as *const u8 as *const libc::c_char
                     } else {
                         b"failure\x00" as *const u8 as *const libc::c_char
                     }, (*p).gr_ctx.value, (*p).gr_ctx.length, (*p).gr_major,
                     (*p).gr_minor, (*p).gr_win, (*p).gr_token.value,
                     (*p).gr_token.length);
    return xdr_stat;
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn gssrpc_xdr_rpc_gss_wrap_data(mut xdrs: *mut XDR,
                                                      mut xdr_func: xdrproc_t,
                                                      mut xdr_ptr: caddr_t,
                                                      mut ctx: gss_ctx_id_t,
                                                      mut qop: gss_qop_t,
                                                      mut svc: rpc_gss_svc_t,
                                                      mut seq: uint32_t)
 -> libc::c_int {
    let mut tmpxdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut databuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut wrapbuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut conf_state: libc::c_int = 0;
    let mut xdr_stat: libc::c_int = 0;
    gssrpc_xdralloc_create(&mut tmpxdrs, XDR_ENCODE);
    xdr_stat = 0 as libc::c_int;
    /* Marshal rpc_gss_data_t (sequence number + arguments). */
    if !(gssrpc_xdr_u_int32(&mut tmpxdrs, &mut seq) == 0 ||
             ::std::mem::transmute::<_,
                                     fn(_: _, _: _)
                                         ->
                                             libc::c_int>(Some(xdr_func.expect("non-null function pointer")).expect("non-null function pointer"))(&mut tmpxdrs,
                                                                                                                                                  xdr_ptr)
                 == 0) {
        /* Set databuf to marshalled rpc_gss_data_t. */
        databuf.length =
            Some((*tmpxdrs.x_ops).x_getpostn.expect("non-null function pointer")).expect("non-null function pointer")(&mut tmpxdrs)
                as size_t;
        databuf.value =
            gssrpc_xdralloc_getdata(&mut tmpxdrs) as *mut libc::c_void;
        if svc as libc::c_uint ==
               RPCSEC_GSS_SVC_INTEGRITY as libc::c_int as libc::c_uint {
            if !(gssrpc_xdr_rpc_gss_buf(xdrs, &mut databuf,
                                        -(1 as libc::c_int) as libc::c_uint)
                     == 0) {
                /* Checksum rpc_gss_data_t. */
                maj_stat =
                    gss_get_mic(&mut min_stat, ctx, qop, &mut databuf,
                                &mut wrapbuf);
                if maj_stat != 0 as libc::c_int as libc::c_uint {
                    gssrpc_log_debug(b"gss_get_mic failed\x00" as *const u8 as
                                         *const libc::c_char);
                } else {
                    /* Marshal checksum. */
                    xdr_stat =
                        gssrpc_xdr_rpc_gss_buf(xdrs, &mut wrapbuf,
                                               -(1 as libc::c_int) as
                                                   libc::c_uint);
                    gss_release_buffer(&mut min_stat, &mut wrapbuf);
                }
            }
        } else if svc as libc::c_uint ==
                      RPCSEC_GSS_SVC_PRIVACY as libc::c_int as libc::c_uint {
            /* Encrypt rpc_gss_data_t. */
            maj_stat =
                gss_wrap(&mut min_stat, ctx, 1 as libc::c_int, qop,
                         &mut databuf, &mut conf_state, &mut wrapbuf);
            if maj_stat != 0 as libc::c_int as libc::c_uint {
                gssrpc_log_status(b"gss_wrap\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, maj_stat, min_stat);
            } else {
                /* Marshal databody_priv. */
                xdr_stat =
                    gssrpc_xdr_rpc_gss_buf(xdrs, &mut wrapbuf,
                                           -(1 as libc::c_int) as
                                               libc::c_uint);
                gss_release_buffer(&mut min_stat, &mut wrapbuf);
            }
        }
    }
    if (*tmpxdrs.x_ops).x_destroy.is_some() {
        Some((*tmpxdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut tmpxdrs);
    }
    return xdr_stat;
}
#[no_mangle]
#[c2rust::src_loc = "184:1"]
pub unsafe extern "C" fn gssrpc_xdr_rpc_gss_unwrap_data(mut xdrs: *mut XDR,
                                                        mut xdr_func:
                                                            xdrproc_t,
                                                        mut xdr_ptr: caddr_t,
                                                        mut ctx: gss_ctx_id_t,
                                                        mut qop: gss_qop_t,
                                                        mut svc:
                                                            rpc_gss_svc_t,
                                                        mut seq: uint32_t)
 -> libc::c_int {
    let mut tmpxdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut databuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut wrapbuf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut seq_num: uint32_t = 0;
    let mut conf_state: libc::c_int = 0;
    let mut qop_state: gss_qop_t = 0;
    let mut xdr_stat: libc::c_int = 0;
    if xdr_func ==
           ::std::mem::transmute::<Option<unsafe extern "C" fn(_: *mut XDR,
                                                               _:
                                                                   *mut libc::c_void)
                                              -> libc::c_int>,
                                   xdrproc_t>(Some(gssrpc_xdr_void as
                                                       unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut libc::c_void)
                                                           -> libc::c_int)) ||
           xdr_ptr.is_null() {
        return 1 as libc::c_int
    }
    memset(&mut databuf as *mut gss_buffer_desc as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<gss_buffer_desc>() as libc::c_ulong);
    memset(&mut wrapbuf as *mut gss_buffer_desc as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<gss_buffer_desc>() as libc::c_ulong);
    if svc as libc::c_uint ==
           RPCSEC_GSS_SVC_INTEGRITY as libc::c_int as libc::c_uint {
        /* Decode databody_integ. */
        if gssrpc_xdr_rpc_gss_buf(xdrs, &mut databuf,
                                  -(1 as libc::c_int) as libc::c_uint) == 0 {
            gssrpc_log_debug(b"xdr decode databody_integ failed\x00" as
                                 *const u8 as *const libc::c_char);
            return 0 as libc::c_int
        }
        /* Decode checksum. */
        if gssrpc_xdr_rpc_gss_buf(xdrs, &mut wrapbuf,
                                  -(1 as libc::c_int) as libc::c_uint) == 0 {
            gss_release_buffer(&mut min_stat, &mut databuf);
            gssrpc_log_debug(b"xdr decode checksum failed\x00" as *const u8 as
                                 *const libc::c_char);
            return 0 as libc::c_int
        }
        /* Verify checksum and QOP. */
        maj_stat =
            gss_verify_mic(&mut min_stat, ctx, &mut databuf, &mut wrapbuf,
                           &mut qop_state);
        free(wrapbuf.value);
        if maj_stat != 0 as libc::c_int as libc::c_uint || qop_state != qop {
            gss_release_buffer(&mut min_stat, &mut databuf);
            gssrpc_log_status(b"gss_verify_mic\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              maj_stat, min_stat);
            return 0 as libc::c_int
        }
    } else if svc as libc::c_uint ==
                  RPCSEC_GSS_SVC_PRIVACY as libc::c_int as libc::c_uint {
        /* Decode databody_priv. */
        if gssrpc_xdr_rpc_gss_buf(xdrs, &mut wrapbuf,
                                  -(1 as libc::c_int) as libc::c_uint) == 0 {
            gssrpc_log_debug(b"xdr decode databody_priv failed\x00" as
                                 *const u8 as *const libc::c_char);
            return 0 as libc::c_int
        }
        /* Decrypt databody. */
        maj_stat =
            gss_unwrap(&mut min_stat, ctx, &mut wrapbuf, &mut databuf,
                       &mut conf_state, &mut qop_state);
        free(wrapbuf.value);
        /* Verify encryption and QOP. */
        if maj_stat != 0 as libc::c_int as libc::c_uint || qop_state != qop ||
               conf_state != 1 as libc::c_int {
            gss_release_buffer(&mut min_stat, &mut databuf);
            gssrpc_log_status(b"gss_unwrap\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              maj_stat, min_stat);
            return 0 as libc::c_int
        }
    }
    /* Decode rpc_gss_data_t (sequence number + arguments). */
    gssrpc_xdrmem_create(&mut tmpxdrs, databuf.value as caddr_t,
                         databuf.length as u_int, XDR_DECODE);
    xdr_stat =
        (gssrpc_xdr_u_int32(&mut tmpxdrs, &mut seq_num) != 0 &&
             ::std::mem::transmute::<_,
                                     fn(_: _, _: _)
                                         ->
                                             libc::c_int>(Some(xdr_func.expect("non-null function pointer")).expect("non-null function pointer"))(&mut tmpxdrs,
                                                                                                                                                  xdr_ptr)
                 != 0) as libc::c_int;
    if (*tmpxdrs.x_ops).x_destroy.is_some() {
        Some((*tmpxdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut tmpxdrs);
    }
    gss_release_buffer(&mut min_stat, &mut databuf);
    /* Verify sequence number. */
    if xdr_stat == 1 as libc::c_int && seq_num != seq {
        gssrpc_log_debug(b"wrong sequence number in databody\x00" as *const u8
                             as *const libc::c_char);
        return 0 as libc::c_int
    }
    return xdr_stat;
}
#[no_mangle]
#[c2rust::src_loc = "261:1"]
pub unsafe extern "C" fn gssrpc_xdr_rpc_gss_data(mut xdrs: *mut XDR,
                                                 mut xdr_func: xdrproc_t,
                                                 mut xdr_ptr: caddr_t,
                                                 mut ctx: gss_ctx_id_t,
                                                 mut qop: gss_qop_t,
                                                 mut svc: rpc_gss_svc_t,
                                                 mut seq: uint32_t)
 -> libc::c_int {
    match (*xdrs).x_op as libc::c_uint {
        0 => {
            return gssrpc_xdr_rpc_gss_wrap_data(xdrs, xdr_func, xdr_ptr, ctx,
                                                qop, svc, seq)
        }
        1 => {
            return gssrpc_xdr_rpc_gss_unwrap_data(xdrs, xdr_func, xdr_ptr,
                                                  ctx, qop, svc, seq)
        }
        2 => { return 1 as libc::c_int }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "351:1"]
pub unsafe extern "C" fn gssrpc_log_debug(mut fmt: *const libc::c_char,
                                          mut args: ...) {
}
#[no_mangle]
#[c2rust::src_loc = "356:1"]
pub unsafe extern "C" fn gssrpc_log_status(mut m: *mut libc::c_char,
                                           mut maj_stat: OM_uint32,
                                           mut min_stat: OM_uint32) {
}
/* include/gssrpc/auth_gss.h */
/*
  Copyright (c) 2000 The Regents of the University of Michigan.
  All rights reserved.

  Copyright (c) 2000 Dug Song <dugsong@UMICH.EDU>.
  All rights reserved, all wrongs reversed.

  Redistribution and use in source and binary forms, with or without
  modification, are permitted provided that the following conditions
  are met:

  1. Redistributions of source code must retain the above copyright
     notice, this list of conditions and the following disclaimer.
  2. Redistributions in binary form must reproduce the above copyright
     notice, this list of conditions and the following disclaimer in the
     documentation and/or other materials provided with the distribution.
  3. Neither the name of the University nor the names of its
     contributors may be used to endorse or promote products derived
     from this software without specific prior written permission.

  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
  DISCLAIMED. IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
  CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
  SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
  BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
  LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
  NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
  SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

  Id: auth_gss.h,v 1.13 2002/05/08 16:54:33 andros Exp
*/
/* RPCSEC_GSS control procedures. */
/* RPCSEC_GSS services. */
/* RPCSEC_GSS security triple. */
/* mechanism */
/* quality of protection */
/* service */
/* cred handle */
/* req flags for init_sec_context */
/* Private data required for kernel implementation */
/* Session context handle */
/* Credentials context handle */
/* Sequence window */
/* Krb 5 default mechanism
#define KRB5OID  "1.2.840.113554.1.2.2"

gss_OID_desc krb5oid = {
	20, KRB5OID
};
 */
/*
struct rpc_gss_sec krb5mech = {
	(gss_OID)&krb5oid,
	GSS_QOP_DEFAULT,
	RPCSEC_GSS_SVC_NONE
};
*/
/* Credentials. */
/* version */
/* control procedure */
/* sequence number */
/* service */
/* context handle */
/* Context creation response. */
/* context handle */
/* major status */
/* minor status */
/* sequence window */
/* token */
/* Maximum sequence number value. */
/* Prototypes. */
#[no_mangle]
#[c2rust::src_loc = "361:1"]
pub unsafe extern "C" fn gssrpc_log_hexdump(mut buf: *const u_char,
                                            mut len: libc::c_int,
                                            mut offset: libc::c_int) {
}
