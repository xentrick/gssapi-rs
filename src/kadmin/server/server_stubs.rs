use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:7"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:7"]
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
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:7"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:7"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/sys/types.h:7"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_short, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/unistd.h:7"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:8"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:8"]
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
#[c2rust::header_src = "/usr/include/sys/socket.h:8"]
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
    use super::socket_h::sockaddr;
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::unistd_h::socklen_t;
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
        #[c2rust::src_loc = "130:1"]
        pub fn getpeername(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                           __len: *mut socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:8"]
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
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:9"]
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
    pub type gss_OID_desc = gss_OID_desc_struct;
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
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
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
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\01\x05\x06\x03"},
 * corresponding to an object identifier value of
 * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
 * 6(nametypes), 3(gss-anonymous-name)}.  The constant
 * and GSS_C_NT_ANONYMOUS should be initialized to point
 * to that gss_OID_desc.
 */
        #[no_mangle]
        #[c2rust::src_loc = "404:27"]
        pub static mut GSS_C_NT_ANONYMOUS: gss_OID;
        /* mech_set */
        #[no_mangle]
        #[c2rust::src_loc = "546:1"]
        pub fn gss_compare_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_name_t, _: *mut libc::c_int)
         -> OM_uint32;
        /* name_equal */
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:10"]
pub mod krb5_h {
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
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
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
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
    /*
 * Per V5 spec on definition of principal types
 */
    /* *<  Name type not known */
    /* *< Just the name of the principal
                                      as in DCE, or for users */
    /* *< Service and other unique instance (krbtgt) */
    /* *< Service with host name as instance
                                      (telnet, rcommands) */
    /* *< Service with host as remaining components */
    /* *< Unique ID */
    /* *< PKINIT */
    /* *< Name in form of SMTP email name */
    /* *< Windows 2000 UPN */
    /* *< Well-known (special) principal */
    /* *< First component of
                                                NT_WELLKNOWN principals */
    /* *< Windows 2000 UPN and SID */
    /* *< NT 4 style name */
    /* *< NT 4 style name and SID */
    /* * Constant version of krb5_principal_data */
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    extern "C" {
        /* per Kerberos v5 protocol spec */
        /* not yet in the spec... */
        /* macros to determine if a type is a local type */
        /*
 * end "hostaddr.h"
 */
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        /* *
 * Convert a string principal name to a krb5_principal structure.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [out] principal_out   New principal
 *
 * Convert a string representation of a principal name to a krb5_principal
 * structure.
 *
 * A string representation of a Kerberos name consists of one or more principal
 * name components, separated by slashes, optionally followed by the \@
 * character and a realm name.  If the realm name is not specified, the local
 * realm is used.
 *
 * To use the slash and \@ symbols as part of a component (quoted) instead of
 * using them as a component separator or as a realm prefix), put a backslash
 * (\) character in front of the symbol.  Similarly, newline, tab, backspace,
 * and NULL characters can be included in a component by using @c n, @c t, @c b
 * or @c 0, respectively.
 *
 * @note The realm in a Kerberos @a name cannot contain slash, colon,
 * or NULL characters.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        /* *
 * Convert a krb5_principal structure to a string representation.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [out] name            String representation of principal name
 *
 * The resulting string representation uses the format and quoting conventions
 * described for krb5_parse_name().
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        /* *
 * Compare two principals.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
        #[no_mangle]
        #[c2rust::src_loc = "3664:1"]
        pub fn krb5_principal_compare(context: krb5_context,
                                      princ1: krb5_const_principal,
                                      princ2: krb5_const_principal)
         -> krb5_boolean;
        /* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
        /* *
 * Get the (possibly extended) error message for a code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 *
 * The behavior of krb5_get_error_message() is only defined the first time it
 * is called after a failed call to a krb5 function using the same context, and
 * only when the error code passed in is the same as that returned by the krb5
 * function.
 *
 * This function never returns NULL, so its result may be used unconditionally
 * as a C string.
 *
 * The string returned by this function must be freed using
 * krb5_free_error_message()
 *
 * @note Future versions may return the same string for the second
 * and following calls.
 */
        #[no_mangle]
        #[c2rust::src_loc = "8023:1"]
        pub fn krb5_get_error_message(ctx: krb5_context,
                                      code: krb5_error_code)
         -> *const libc::c_char;
        /* *
 * Free an error message generated by krb5_get_error_message().
 *
 * @param [in] ctx              Library context
 * @param [in] msg              Pointer to error message
 */
        #[no_mangle]
        #[c2rust::src_loc = "8032:1"]
        pub fn krb5_free_error_message(ctx: krb5_context,
                                       msg: *const libc::c_char);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:12"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "88:1"]
    pub type rpcprog_t = uint32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:12"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:12"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:12"]
pub mod rpc_msg_h {
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
        pub ru: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed_0 {
        pub RM_cmb: call_body,
        pub RM_rmb: reply_body,
    }
    /*
 * Body of a reply to an rpc request.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:8"]
    pub struct reply_body {
        pub rp_stat: reply_stat,
        pub ru: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_1 {
        pub RP_ar: accepted_reply,
        pub RP_dr: rejected_reply,
    }
    /*
 * Reply to an rpc request that was rejected by the server.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rejected_reply {
        pub rj_stat: reject_stat,
        pub ru: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:2"]
    pub union C2RustUnnamed_2 {
        pub RJ_versions: C2RustUnnamed_3,
        pub RJ_why: auth_stat,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:3"]
    pub struct C2RustUnnamed_3 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type reject_stat = libc::c_uint;
    #[c2rust::src_loc = "76:2"]
    pub const AUTH_ERROR: reject_stat = 1;
    #[c2rust::src_loc = "75:2"]
    pub const RPC_MISMATCH: reject_stat = 0;
    /*
 * Reply part of an rpc exchange
 */
    /*
 * Reply to an rpc request that was accepted by the server.
 * Note: there could be an error even though the request was
 * accepted.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "88:8"]
    pub struct accepted_reply {
        pub ar_verf: opaque_auth,
        pub ar_stat: accept_stat,
        pub ru: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:2"]
    pub union C2RustUnnamed_4 {
        pub AR_versions: C2RustUnnamed_6,
        pub AR_results: C2RustUnnamed_5,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:3"]
    pub struct C2RustUnnamed_5 {
        pub where_0: caddr_t,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:3"]
    pub struct C2RustUnnamed_6 {
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
    #[c2rust::src_loc = "60:1"]
    pub type reply_stat = libc::c_uint;
    #[c2rust::src_loc = "62:2"]
    pub const MSG_DENIED: reply_stat = 1;
    #[c2rust::src_loc = "61:2"]
    pub const MSG_ACCEPTED: reply_stat = 0;
    /*
 * Body of an rpc request call.
 */
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc.h:12"]
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
    use super::sys_types_h::u_short;
    use super::in_h::sockaddr_in;
    use super::svc_auth_h::SVCAUTH;
    use super::rpc_msg_h::rpc_msg;
    use super::xdr_h::xdrproc_t;
    /* !defined(GSSRPC_SVC_H) */
    /* XXX add auth_gsapi_log_*? */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc_auth.h:12"]
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
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:12"]
pub mod kdb_h {
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
    #[c2rust::src_loc = "156:16"]
    pub struct krb5_string_attr_st {
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    #[c2rust::src_loc = "156:1"]
    pub type krb5_string_attr = krb5_string_attr_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_data,
                        krb5_enctype, krb5_int32};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:12"]
pub mod admin_h {
    #[c2rust::src_loc = "70:1"]
    pub type kadm5_policy_t = *mut libc::c_char;
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "190:16"]
    pub struct _kadm5_principal_ent_t {
        pub principal: krb5_principal,
        pub princ_expire_time: krb5_timestamp,
        pub last_pwd_change: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub max_life: krb5_deltat,
        pub mod_name: krb5_principal,
        pub mod_date: krb5_timestamp,
        pub attributes: krb5_flags,
        pub kvno: krb5_kvno,
        pub mkvno: krb5_kvno,
        pub policy: *mut libc::c_char,
        pub aux_attributes: libc::c_long,
        pub max_renewable_life: krb5_deltat,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_key_data: krb5_int16,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    #[c2rust::src_loc = "190:1"]
    pub type kadm5_principal_ent_rec = _kadm5_principal_ent_t;
    #[c2rust::src_loc = "190:1"]
    pub type kadm5_principal_ent_t = *mut _kadm5_principal_ent_t;
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
    #[c2rust::src_loc = "215:1"]
    pub type kadm5_policy_ent_t = *mut _kadm5_policy_ent_t;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "284:16"]
    pub struct _kadm5_key_data {
        pub kvno: krb5_kvno,
        pub key: krb5_keyblock,
        pub salt: krb5_keysalt,
    }
    #[c2rust::src_loc = "284:1"]
    pub type kadm5_key_data = _kadm5_key_data;
    use super::krb5_h::{krb5_principal, krb5_timestamp, krb5_deltat,
                        krb5_flags, krb5_kvno, krb5_int16, krb5_enctype,
                        krb5_int32, krb5_keyblock, _krb5_context,
                        krb5_context, krb5_principal_data, krb5_boolean};
    use super::kdb_h::{krb5_tl_data, krb5_key_data, krb5_key_salt_tuple,
                       krb5_keysalt, krb5_string_attr};
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        /* version 2 fields */
        /* no longer used */
        /* version 3 fields */
        /* version 4 fields */
        /*
 * Data structure returned by kadm5_get_config_params()
 */
        /* Novell */ /* ABI change? */
        /* Deprecated except for db2 backwards compatibility.  Don't add
       new uses except as fallbacks for parameters that should be
       specified in the database module section of the config
       file.  */
        /*    char *            iprop_server;*/
        /*
 * functions
 */
        /*
 * For all initialization functions, the caller must first initialize
 * a context with kadm5_init_krb5_context which will survive as long
 * as the resulting handle.  The caller should free the context with
 * krb5_free_context.
 */
        #[no_mangle]
        #[c2rust::src_loc = "493:1"]
        pub fn kadm5_free_kadm5_key_data(context: krb5_context,
                                         n_key_data: libc::c_int,
                                         key_data: *mut kadm5_key_data)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "484:1"]
        pub fn kadm5_set_string(server_handle: *mut libc::c_void,
                                principal: krb5_principal,
                                key: *const libc::c_char,
                                value: *const libc::c_char) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "479:1"]
        pub fn kadm5_get_strings(server_handle: *mut libc::c_void,
                                 principal: krb5_principal,
                                 strings_out: *mut *mut krb5_string_attr,
                                 count_out: *mut libc::c_int) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "475:1"]
        pub fn kadm5_purgekeys(server_handle: *mut libc::c_void,
                               principal: krb5_principal,
                               keepkvno: libc::c_int) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "357:1"]
        pub fn kadm5_create_principal(server_handle: *mut libc::c_void,
                                      ent: kadm5_principal_ent_t,
                                      mask: libc::c_long,
                                      pass: *mut libc::c_char) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn kadm5_create_principal_3(server_handle: *mut libc::c_void,
                                        ent: kadm5_principal_ent_t,
                                        mask: libc::c_long,
                                        n_ks_tuple: libc::c_int,
                                        ks_tuple: *mut krb5_key_salt_tuple,
                                        pass: *mut libc::c_char)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn kadm5_delete_principal(server_handle: *mut libc::c_void,
                                      principal: krb5_principal)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "368:1"]
        pub fn kadm5_modify_principal(server_handle: *mut libc::c_void,
                                      ent: kadm5_principal_ent_t,
                                      mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "371:1"]
        pub fn kadm5_rename_principal(server_handle: *mut libc::c_void,
                                      _: krb5_principal, _: krb5_principal)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "373:1"]
        pub fn kadm5_get_principal(server_handle: *mut libc::c_void,
                                   principal: krb5_principal,
                                   ent: kadm5_principal_ent_t,
                                   mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "377:1"]
        pub fn kadm5_chpass_principal(server_handle: *mut libc::c_void,
                                      principal: krb5_principal,
                                      pass: *mut libc::c_char) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn kadm5_chpass_principal_3(server_handle: *mut libc::c_void,
                                        principal: krb5_principal,
                                        keepold: krb5_boolean,
                                        n_ks_tuple: libc::c_int,
                                        ks_tuple: *mut krb5_key_salt_tuple,
                                        pass: *mut libc::c_char)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "386:1"]
        pub fn kadm5_randkey_principal(server_handle: *mut libc::c_void,
                                       principal: krb5_principal,
                                       keyblocks: *mut *mut krb5_keyblock,
                                       n_keys: *mut libc::c_int)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "390:1"]
        pub fn kadm5_randkey_principal_3(server_handle: *mut libc::c_void,
                                         principal: krb5_principal,
                                         keepold: krb5_boolean,
                                         n_ks_tuple: libc::c_int,
                                         ks_tuple: *mut krb5_key_salt_tuple,
                                         keyblocks: *mut *mut krb5_keyblock,
                                         n_keys: *mut libc::c_int)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "398:1"]
        pub fn kadm5_setkey_principal(server_handle: *mut libc::c_void,
                                      principal: krb5_principal,
                                      keyblocks: *mut krb5_keyblock,
                                      n_keys: libc::c_int) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "403:1"]
        pub fn kadm5_setkey_principal_3(server_handle: *mut libc::c_void,
                                        principal: krb5_principal,
                                        keepold: krb5_boolean,
                                        n_ks_tuple: libc::c_int,
                                        ks_tuple: *mut krb5_key_salt_tuple,
                                        keyblocks: *mut krb5_keyblock,
                                        n_keys: libc::c_int) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "411:1"]
        pub fn kadm5_setkey_principal_4(server_handle: *mut libc::c_void,
                                        principal: krb5_principal,
                                        keepold: krb5_boolean,
                                        key_data: *mut kadm5_key_data,
                                        n_key_data: libc::c_int)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "423:1"]
        pub fn kadm5_create_policy(server_handle: *mut libc::c_void,
                                   ent: kadm5_policy_ent_t,
                                   mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "426:1"]
        pub fn kadm5_delete_policy(server_handle: *mut libc::c_void,
                                   policy: kadm5_policy_t) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "428:1"]
        pub fn kadm5_modify_policy(server_handle: *mut libc::c_void,
                                   ent: kadm5_policy_ent_t,
                                   mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "431:1"]
        pub fn kadm5_get_policy(server_handle: *mut libc::c_void,
                                policy: kadm5_policy_t,
                                ent: kadm5_policy_ent_t) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "434:1"]
        pub fn kadm5_get_privs(server_handle: *mut libc::c_void,
                               privs: *mut libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "444:1"]
        pub fn kadm5_free_principal_ent(server_handle: *mut libc::c_void,
                                        ent: kadm5_principal_ent_t)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "450:1"]
        pub fn kadm5_get_principals(server_handle: *mut libc::c_void,
                                    exp: *mut libc::c_char,
                                    princs: *mut *mut *mut libc::c_char,
                                    count: *mut libc::c_int) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "454:1"]
        pub fn kadm5_get_policies(server_handle: *mut libc::c_void,
                                  exp: *mut libc::c_char,
                                  pols: *mut *mut *mut libc::c_char,
                                  count: *mut libc::c_int) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "469:1"]
        pub fn kadm5_get_principal_keys(server_handle: *mut libc::c_void,
                                        principal: krb5_principal,
                                        kvno: krb5_kvno,
                                        key_data: *mut *mut kadm5_key_data,
                                        n_key_data: *mut libc::c_int)
         -> kadm5_ret_t;
    }
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/kadm_rpc.h:13"]
pub mod kadm_rpc_h {
    /* -*- mode: c; c-file-style: "bsd"; indent-tabs-mode: t -*- */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "10:8"]
    pub struct cprinc_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_principal_ent_rec,
        pub mask: libc::c_long,
        pub passwd: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "18:8"]
    pub struct cprinc3_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_principal_ent_rec,
        pub mask: libc::c_long,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
        pub passwd: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:8"]
    pub struct generic_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:8"]
    pub struct dprinc_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:8"]
    pub struct mprinc_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_principal_ent_rec,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:8"]
    pub struct rprinc_arg {
        pub api_version: krb5_ui_4,
        pub src: krb5_principal,
        pub dest: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:8"]
    pub struct gprincs_arg {
        pub api_version: krb5_ui_4,
        pub exp: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct gprincs_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub princs: *mut *mut libc::c_char,
        pub count: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:8"]
    pub struct chpass_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub pass: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "75:8"]
    pub struct chpass3_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
        pub pass: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:8"]
    pub struct setkey_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keyblocks: *mut krb5_keyblock,
        pub n_keys: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "93:8"]
    pub struct setkey3_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
        pub keyblocks: *mut krb5_keyblock,
        pub n_keys: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "104:8"]
    pub struct setkey4_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub key_data: *mut kadm5_key_data,
        pub n_key_data: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:8"]
    pub struct chrand_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "119:8"]
    pub struct chrand3_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepold: krb5_boolean,
        pub n_ks_tuple: libc::c_int,
        pub ks_tuple: *mut krb5_key_salt_tuple,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "128:8"]
    pub struct chrand_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub key: krb5_keyblock,
        pub keys: *mut krb5_keyblock,
        pub n_keys: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "137:8"]
    pub struct gprinc_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "144:8"]
    pub struct gprinc_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub rec: kadm5_principal_ent_rec,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "151:8"]
    pub struct cpol_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_policy_ent_rec,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "158:8"]
    pub struct dpol_arg {
        pub api_version: krb5_ui_4,
        pub name: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "164:8"]
    pub struct mpol_arg {
        pub api_version: krb5_ui_4,
        pub rec: kadm5_policy_ent_rec,
        pub mask: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "171:8"]
    pub struct gpol_arg {
        pub api_version: krb5_ui_4,
        pub name: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:8"]
    pub struct gpol_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub rec: kadm5_policy_ent_rec,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "184:8"]
    pub struct gpols_arg {
        pub api_version: krb5_ui_4,
        pub exp: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "190:8"]
    pub struct gpols_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub pols: *mut *mut libc::c_char,
        pub count: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "198:8"]
    pub struct getprivs_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub privs: libc::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "205:8"]
    pub struct purgekeys_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub keepkvno: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct gstrings_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "218:8"]
    pub struct gstrings_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub strings: *mut krb5_string_attr,
        pub count: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "226:8"]
    pub struct sstring_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "234:8"]
    pub struct getpkeys_arg {
        pub api_version: krb5_ui_4,
        pub princ: krb5_principal,
        pub kvno: krb5_kvno,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:8"]
    pub struct getpkeys_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
        pub key_data: *mut kadm5_key_data,
        pub n_key_data: libc::c_int,
    }
    use super::krb5_h::{krb5_ui_4, krb5_principal, krb5_boolean,
                        krb5_keyblock, krb5_kvno};
    use super::admin_h::{kadm5_principal_ent_rec, kadm5_ret_t, kadm5_key_data,
                         kadm5_policy_ent_rec};
    use super::kdb_h::{krb5_key_salt_tuple, krb5_string_attr};
    /* __KADM_RPC_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/server_internal.h:14"]
pub mod server_internal_h {
    #[c2rust::src_loc = "42:1"]
    pub type kadm5_server_handle_t = *mut _kadm5_server_handle_t;
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
    #[c2rust::src_loc = "40:1"]
    pub type kadm5_hook_handle = *mut kadm5_hook_handle_st;
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
    use super::krb5_h::{krb5_ui_4, krb5_context, krb5_principal};
    use super::admin_h::kadm5_config_params;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type kadm5_hook_handle_st;
        #[c2rust::src_loc = "38:16"]
        pub type pwqual_handle_st;
    }
    /* __KADM5_SERVER_INTERNAL_H__ */
    /* * @}*/
}
#[c2rust::header_src = "/usr/include/stdlib.h:7"]
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
#[c2rust::header_src = "/usr/include/stdio.h:7"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:7"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:7"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/arpa/inet.h:8"]
pub mod inet_h {
    use super::unistd_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn inet_ntop(__af: libc::c_int, __cp: *const libc::c_void,
                         __buf: *mut libc::c_char, __len: socklen_t)
         -> *const libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/socket-utils.h:8"]
pub mod socket_utils_h {
    #[inline]
    #[c2rust::src_loc = "87:1"]
    pub unsafe extern "C" fn ss2sin6(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr_in6 {
        return ss as *mut sockaddr_in6;
    }
    #[inline]
    #[c2rust::src_loc = "83:1"]
    pub unsafe extern "C" fn ss2sin(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr_in {
        return ss as *mut sockaddr_in;
    }
    #[inline]
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn ss2sa(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr {
        return ss as *mut sockaddr;
    }
    use super::socket_h::{sockaddr_storage, sockaddr};
    use super::in_h::{sockaddr_in6, sockaddr_in};
    /* SOCKET_UTILS_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:10"]
pub mod gssapi_ext_h {
    use super::gssapi_h::{gss_OID_desc, gss_const_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn gss_oid_equal(_: gss_const_OID, _: gss_const_OID)
         -> libc::c_int;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:10"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, OM_uint32,
                          gss_ctx_id_struct, gss_ctx_id_t};
    use super::krb5_h::krb5_flags;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "40:33"]
        pub static GSS_KRB5_NT_PRINCIPAL_NAME: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "181:1"]
        pub fn gss_krb5_get_tkt_flags(minor_status: *mut OM_uint32,
                                      context_handle: gss_ctx_id_t,
                                      ticket_flags: *mut krb5_flags)
         -> OM_uint32;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:16"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/server/misc.h:17"]
pub mod misc_h {
    use super::stddef_h::size_t;
    use super::krb5_h::{krb5_principal_data, krb5_principal};
    use super::admin_h::kadm5_ret_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "28:1"]
        pub fn trunc_name(len: *mut size_t, dots: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "22:1"]
        pub fn check_min_life(server_handle: *mut libc::c_void,
                              principal: krb5_principal,
                              msg_ret: *mut libc::c_char,
                              msg_len: libc::c_uint) -> kadm5_ret_t;
    }
    /* _MISC_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/server/auth.h:18"]
pub mod server_auth_h {
    use super::krb5_h::{_krb5_context, krb5_context, krb5_principal_data,
                        krb5_const_principal, krb5_boolean};
    use super::admin_h::{kadm5_policy_ent_rec, _kadm5_principal_ent_t,
                         kadm5_principal_ent_t};
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
        /* Authorize an add-principal or modify-principal operation, and apply
 * restrictions to ent and mask if any modules supply them. */
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn auth_restrict(context: krb5_context, opcode: libc::c_int,
                             client: krb5_const_principal,
                             ent: kadm5_principal_ent_t,
                             mask: *mut libc::c_long) -> krb5_boolean;
        /* Notify modules that the most recent authorized operation has ended. */
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn auth_end(context: krb5_context);
    }
    /* AUTH_H */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__u_short, __u_int, __uint8_t, __int16_t, __uint16_t,
                        __int32_t, __uint32_t, __caddr_t, __socklen_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::sys_types_h::{u_short, u_int, caddr_t};
pub use self::unistd_h::socklen_t;
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage};
pub use self::sys_socket_h::{__SOCKADDR_ARG, sockaddr_x25, sockaddr_un,
                             sockaddr_ns, sockaddr_iso, sockaddr_ipx,
                             sockaddr_inarp, sockaddr_eon, sockaddr_dl,
                             sockaddr_ax25, sockaddr_at, getpeername};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t};
pub use self::gssapi_h::{gss_name_t, gss_ctx_id_t, gss_uint32, OM_uint32,
                         gss_OID_desc_struct, gss_OID_desc, gss_OID,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_const_OID, gss_name_struct,
                         gss_ctx_id_struct, GSS_C_NT_ANONYMOUS,
                         gss_compare_name, gss_display_name, gss_release_name,
                         gss_release_buffer, gss_inquire_context};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_context, _krb5_keyblock,
                       krb5_keyblock, _krb5_context, krb5_parse_name,
                       krb5_unparse_name, krb5_principal_compare,
                       krb5_free_principal, krb5_free_keyblock_contents,
                       krb5_get_error_message, krb5_free_error_message};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, opaque_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed_0, reply_body,
                          C2RustUnnamed_1, rejected_reply, C2RustUnnamed_2,
                          C2RustUnnamed_3, reject_stat, AUTH_ERROR,
                          RPC_MISMATCH, accepted_reply, C2RustUnnamed_4,
                          C2RustUnnamed_5, C2RustUnnamed_6, accept_stat,
                          SYSTEM_ERR, GARBAGE_ARGS, PROC_UNAVAIL,
                          PROG_MISMATCH, PROG_UNAVAIL, SUCCESS, reply_stat,
                          MSG_DENIED, MSG_ACCEPTED, call_body, msg_type,
                          REPLY, CALL};
pub use self::svc_h::{svc_req, SVCXPRT, xp_ops, xprt_stat, XPRT_IDLE,
                      XPRT_MOREREQS, XPRT_DIED};
pub use self::svc_auth_h::{SVCAUTH, svc_auth_ops};
pub use self::kdb_h::{_krb5_tl_data, krb5_tl_data, krb5_string_attr_st,
                      krb5_string_attr, _krb5_key_data, krb5_key_data,
                      _krb5_keysalt, krb5_keysalt, __krb5_key_salt_tuple,
                      krb5_key_salt_tuple};
pub use self::admin_h::{kadm5_policy_t, kadm5_ret_t, _kadm5_principal_ent_t,
                        kadm5_principal_ent_rec, kadm5_principal_ent_t,
                        _kadm5_policy_ent_t, kadm5_policy_ent_rec,
                        kadm5_policy_ent_t, _kadm5_config_params,
                        kadm5_config_params, _kadm5_key_data, kadm5_key_data,
                        kadm5_free_kadm5_key_data, kadm5_set_string,
                        kadm5_get_strings, kadm5_purgekeys,
                        kadm5_create_principal, kadm5_create_principal_3,
                        kadm5_delete_principal, kadm5_modify_principal,
                        kadm5_rename_principal, kadm5_get_principal,
                        kadm5_chpass_principal, kadm5_chpass_principal_3,
                        kadm5_randkey_principal, kadm5_randkey_principal_3,
                        kadm5_setkey_principal, kadm5_setkey_principal_3,
                        kadm5_setkey_principal_4, kadm5_create_policy,
                        kadm5_delete_policy, kadm5_modify_policy,
                        kadm5_get_policy, kadm5_get_privs,
                        kadm5_free_principal_ent, kadm5_get_principals,
                        kadm5_get_policies, kadm5_get_principal_keys};
pub use self::kadm_rpc_h::{cprinc_arg, cprinc3_arg, generic_ret, dprinc_arg,
                           mprinc_arg, rprinc_arg, gprincs_arg, gprincs_ret,
                           chpass_arg, chpass3_arg, setkey_arg, setkey3_arg,
                           setkey4_arg, chrand_arg, chrand3_arg, chrand_ret,
                           gprinc_arg, gprinc_ret, cpol_arg, dpol_arg,
                           mpol_arg, gpol_arg, gpol_ret, gpols_arg, gpols_ret,
                           getprivs_ret, purgekeys_arg, gstrings_arg,
                           gstrings_ret, sstring_arg, getpkeys_arg,
                           getpkeys_ret};
pub use self::server_internal_h::{kadm5_server_handle_t,
                                  _kadm5_server_handle_t, kadm5_hook_handle,
                                  pwqual_handle, kadm5_hook_handle_st,
                                  pwqual_handle_st};
use self::stdlib_h::{malloc, free};
use self::stdio_h::asprintf;
use self::string_h::{strlen, strcmp, memcmp, memset};
use self::libintl_h::dgettext;
use self::inet_h::inet_ntop;
pub use self::socket_utils_h::{ss2sin6, ss2sin, ss2sa};
use self::gssapi_ext_h::gss_oid_equal;
use self::gssapi_krb5_h::{GSS_KRB5_NT_PRINCIPAL_NAME, gss_krb5_get_tkt_flags};
use self::adm_proto_h::krb5_klog_syslog;
use self::misc_h::{trunc_name, check_min_life};
use self::server_auth_h::{auth, auth_restrict, auth_end};
extern "C" {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 */
    #[no_mangle]
    #[c2rust::src_loc = "20:41"]
    pub static mut gss_changepw_name: gss_name_t;
    #[no_mangle]
    #[c2rust::src_loc = "21:41"]
    pub static mut gss_oldchangepw_name: gss_name_t;
    #[no_mangle]
    #[c2rust::src_loc = "22:41"]
    pub static mut global_server_handle: *mut libc::c_void;
}
#[c2rust::src_loc = "40:1"]
unsafe extern "C" fn cmp_gss_names(mut n1: gss_name_t, mut n2: gss_name_t)
 -> libc::c_int {
    let mut emin: OM_uint32 = 0;
    let mut equal: libc::c_int = 0;
    if gss_compare_name(&mut emin, n1, n2, &mut equal) &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        return 0 as libc::c_int
    }
    return equal;
}
/* Does a comparison of the names and then releases the first entity */
/* For use above in CHANGEPW_SERVICE */
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn cmp_gss_names_rel_1(mut n1: gss_name_t,
                                         mut n2: gss_name_t) -> libc::c_int {
    let mut min_stat: OM_uint32 = 0;
    let mut ret: libc::c_int = 0;
    ret = cmp_gss_names(n1, n2);
    if !n1.is_null() { gss_release_name(&mut min_stat, &mut n1); }
    return ret;
}
/*
 * Function check_handle
 *
 * Purpose: Check a server handle and return a com_err code if it is
 * invalid or 0 if it is valid.
 *
 * Arguments:
 *
 *      handle          The server handle.
 */
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn check_handle(mut handle: *mut libc::c_void)
 -> libc::c_int {
    let mut srvr: kadm5_server_handle_t = handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long as libc::c_int }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long as libc::c_int
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long as libc::c_int
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long as libc::c_int
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long as libc::c_int
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long as libc::c_int
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long as libc::c_int
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long as libc::c_int
    }
    let mut srvr_0: kadm5_server_handle_t = handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() {
        return 43787551 as libc::c_long as libc::c_int
    }
    if (*srvr_0).lhandle.is_null() {
        return 43787551 as libc::c_long as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
 * Function: new_server_handle
 *
 * Purpose: Constructs a server handle suitable for passing into the
 * server library API functions, by folding the client's API version
 * and calling principal into the server handle returned by
 * kadm5_init.
 *
 * Arguments:
 *      api_version     (input) The API version specified by the client
 *      rqstp           (input) The RPC request
 *      handle          (output) The returned handle
 *      <return value>  (output) An error code, or 0 if no error occurred
 *
 * Effects:
 *      Returns a pointer to allocated storage containing the server
 *      handle.  If an error occurs, then no allocated storage is
 *      returned, and the return value of the function will be a
 *      non-zero com_err code.
 *
 *      The allocated storage for the handle should be freed with
 *      free_server_handle (see below) when it is no longer needed.
 */
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn new_server_handle(mut api_version: krb5_ui_4,
                                       mut rqstp: *mut svc_req,
                                       mut out_handle:
                                           *mut kadm5_server_handle_t)
 -> kadm5_ret_t {
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    *out_handle = 0 as kadm5_server_handle_t;
    handle =
        malloc(::std::mem::size_of::<_kadm5_server_handle_t>() as
                   libc::c_ulong) as kadm5_server_handle_t;
    if handle.is_null() { return 12 as libc::c_int as kadm5_ret_t }
    *handle = *(global_server_handle as kadm5_server_handle_t);
    (*handle).api_version = api_version;
    if gss_to_krb5_name(handle, rqst2name(rqstp),
                        &mut (*handle).current_caller) == 0 {
        free(handle as *mut libc::c_void);
        return 43787520 as libc::c_long
    }
    *out_handle = handle;
    return 0 as libc::c_int as kadm5_ret_t;
}
/*
 * Function: free_server_handle
 *
 * Purpose: Free handle memory allocated by new_server_handle
 *
 * Arguments:
 *      handle          (input/output) The handle to free
 */
#[c2rust::src_loc = "138:1"]
unsafe extern "C" fn free_server_handle(mut handle: kadm5_server_handle_t) {
    if handle.is_null() { return }
    krb5_free_principal((*handle).context, (*handle).current_caller);
    free(handle as *mut libc::c_void);
}
/* Result is stored in a static buffer and is invalidated by the next call. */
#[no_mangle]
#[c2rust::src_loc = "147:1"]
pub unsafe extern "C" fn client_addr(mut xprt: *mut SVCXPRT)
 -> *const libc::c_char {
    static mut abuf: [libc::c_char; 128] = [0; 128];
    let mut ss: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut len: socklen_t =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if getpeername((*xprt).xp_sock,
                   __SOCKADDR_ARG{__sockaddr__: ss2sa(&mut ss),}, &mut len) !=
           0 as libc::c_int {
        return b"(unknown)\x00" as *const u8 as *const libc::c_char
    }
    if (*ss2sa(&mut ss)).sa_family as libc::c_int == 2 as libc::c_int {
        p =
            inet_ntop(2 as libc::c_int,
                      &mut (*(ss2sin as
                                  unsafe extern "C" fn(_:
                                                           *mut sockaddr_storage)
                                      -> *mut sockaddr_in)(&mut ss)).sin_addr
                          as *mut in_addr as *const libc::c_void,
                      abuf.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 128]>() as
                          libc::c_ulong as socklen_t)
    } else if (*ss2sa(&mut ss)).sa_family as libc::c_int == 10 as libc::c_int
     {
        p =
            inet_ntop(10 as libc::c_int,
                      &mut (*(ss2sin6 as
                                  unsafe extern "C" fn(_:
                                                           *mut sockaddr_storage)
                                      ->
                                          *mut sockaddr_in6)(&mut ss)).sin6_addr
                          as *mut in6_addr as *const libc::c_void,
                      abuf.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 128]>() as
                          libc::c_ulong as socklen_t)
    }
    return if p.is_null() {
               b"(unknown)\x00" as *const u8 as *const libc::c_char
           } else { p };
}
/*
 * Function: setup_gss_names
 *
 * Purpose: Create printable representations of the client and server
 * names.
 *
 * Arguments:
 *      rqstp           (r) the RPC request
 *      client_name     (w) the gss_buffer_t for the client name
 *      server_name     (w) the gss_buffer_t for the server name
 *
 * Effects:
 *
 * Unparses the client and server names into client_name and
 * server_name, both of which must be freed by the caller.  Returns 0
 * on success and -1 on failure.
 */
#[no_mangle]
#[c2rust::src_loc = "181:1"]
pub unsafe extern "C" fn setup_gss_names(mut rqstp: *mut svc_req,
                                         mut client_name:
                                             *mut gss_buffer_desc,
                                         mut server_name:
                                             *mut gss_buffer_desc)
 -> libc::c_int {
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut server_gss_name: gss_name_t = 0 as *mut gss_name_struct;
    if gss_name_to_string(rqst2name(rqstp), client_name) != 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    maj_stat =
        gss_inquire_context(&mut min_stat,
                            (*rqstp).rq_svccred as gss_ctx_id_t,
                            0 as *mut gss_name_t, &mut server_gss_name,
                            0 as *mut OM_uint32, 0 as *mut gss_OID,
                            0 as *mut OM_uint32, 0 as *mut libc::c_int,
                            0 as *mut libc::c_int);
    if maj_stat != 0 as libc::c_int as libc::c_uint {
        gss_release_buffer(&mut min_stat, client_name);
        gss_release_name(&mut min_stat, &mut server_gss_name);
        return -(1 as libc::c_int)
    }
    if gss_name_to_string(server_gss_name, server_name) != 0 as libc::c_int {
        gss_release_buffer(&mut min_stat, client_name);
        gss_release_name(&mut min_stat, &mut server_gss_name);
        return -(1 as libc::c_int)
    }
    gss_release_name(&mut min_stat, &mut server_gss_name);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "207:1"]
unsafe extern "C" fn acceptor_name(mut context: gss_ctx_id_t) -> gss_name_t {
    let mut maj_stat: OM_uint32 = 0;
    let mut min_stat: OM_uint32 = 0;
    let mut name: gss_name_t = 0 as *mut gss_name_struct;
    maj_stat =
        gss_inquire_context(&mut min_stat, context, 0 as *mut gss_name_t,
                            &mut name, 0 as *mut OM_uint32, 0 as *mut gss_OID,
                            0 as *mut OM_uint32, 0 as *mut libc::c_int,
                            0 as *mut libc::c_int);
    if maj_stat != 0 as libc::c_int as libc::c_uint { return 0 as gss_name_t }
    return name;
}
#[c2rust::src_loc = "219:1"]
unsafe extern "C" fn gss_to_krb5_name(mut handle: kadm5_server_handle_t,
                                      mut gss_name: gss_name_t,
                                      mut princ: *mut krb5_principal)
 -> libc::c_int {
    let mut minor_stat: OM_uint32 = 0;
    let mut gss_str: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut success: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if gss_name_to_string(gss_name, &mut gss_str) != 0 as libc::c_int {
        return 0 as libc::c_int
    }
    if asprintf(&mut s as *mut *mut libc::c_char,
                b"%.*s\x00" as *const u8 as *const libc::c_char,
                gss_str.length as libc::c_int,
                gss_str.value as *mut libc::c_char) < 0 as libc::c_int {
        gss_release_buffer(&mut minor_stat, &mut gss_str);
        return 0 as libc::c_int
    }
    success =
        (krb5_parse_name((*handle).context, s, princ) == 0 as libc::c_int) as
            libc::c_int;
    free(s as *mut libc::c_void);
    gss_release_buffer(&mut minor_stat, &mut gss_str);
    return success;
}
#[c2rust::src_loc = "239:1"]
unsafe extern "C" fn gss_name_to_string(mut gss_name: gss_name_t,
                                        mut str: *mut gss_buffer_desc)
 -> libc::c_int {
    let mut status: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut gss_type: gss_OID = 0 as *mut gss_OID_desc_struct;
    let pref: [libc::c_char; 21] =
        *::std::mem::transmute::<&[u8; 21],
                                 &[libc::c_char; 21]>(b"WELLKNOWN/ANONYMOUS@\x00");
    let preflen: size_t =
        (::std::mem::size_of::<[libc::c_char; 21]>() as
             libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    status = gss_display_name(&mut minor_stat, gss_name, str, &mut gss_type);
    if status != 0 as libc::c_int as libc::c_uint { return 1 as libc::c_int }
    if gss_oid_equal(gss_type as gss_const_OID,
                     GSS_C_NT_ANONYMOUS as gss_const_OID) != 0 {
        /* Guard against non-krb5 mechs with different anonymous displays. */
        if (*str).length < preflen ||
               memcmp((*str).value, pref.as_ptr() as *const libc::c_void,
                      preflen) != 0 as libc::c_int {
            return 1 as libc::c_int
        }
    } else if gss_oid_equal(gss_type as gss_const_OID,
                            GSS_KRB5_NT_PRINCIPAL_NAME as gss_const_OID) == 0
     {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
 * Perform common initialization for server stub functions.  A subset of the
 * output arguments may be set on failure; the caller is responsible for
 * initializing outputs and calling stub_cleanup() on success or failure.
 * princ and princ_str_out may be NULL to omit unparsing a principal name.
 */
#[c2rust::src_loc = "266:1"]
unsafe extern "C" fn stub_setup(mut api_version: krb5_ui_4,
                                mut rqstp: *mut svc_req,
                                mut princ: krb5_principal,
                                mut handle_out: *mut kadm5_server_handle_t,
                                mut api_version_out: *mut krb5_ui_4,
                                mut client_name_out: gss_buffer_t,
                                mut service_name_out: gss_buffer_t,
                                mut princ_str_out: *mut *mut libc::c_char)
 -> kadm5_ret_t {
    let mut ret: kadm5_ret_t = 0;
    ret = new_server_handle(api_version, rqstp, handle_out);
    if ret != 0 { return ret }
    ret = check_handle(*handle_out as *mut libc::c_void) as kadm5_ret_t;
    if ret != 0 { return ret }
    *api_version_out = (**handle_out).api_version;
    if setup_gss_names(rqstp, client_name_out, service_name_out) <
           0 as libc::c_int {
        return 43787520 as libc::c_long
    }
    if !princ_str_out.is_null() {
        if princ.is_null() { return 43787538 as libc::c_long }
        if krb5_unparse_name((**handle_out).context,
                             princ as krb5_const_principal, princ_str_out) !=
               0 {
            return 43787538 as libc::c_long
        }
    }
    return 0 as libc::c_int as kadm5_ret_t;
}
/* Perform common cleanup for server stub functions. */
#[c2rust::src_loc = "298:1"]
unsafe extern "C" fn stub_cleanup(mut handle: kadm5_server_handle_t,
                                  mut princ_str: *mut libc::c_char,
                                  mut client_name: gss_buffer_t,
                                  mut service_name: gss_buffer_t) {
    let mut minor_stat: OM_uint32 = 0;
    auth_end((*handle).context);
    free_server_handle(handle);
    free(princ_str as *mut libc::c_void);
    gss_release_buffer(&mut minor_stat, client_name);
    gss_release_buffer(&mut minor_stat, service_name);
}
#[c2rust::src_loc = "311:1"]
unsafe extern "C" fn stub_auth(mut handle: kadm5_server_handle_t,
                               mut opcode: libc::c_int,
                               mut p1: krb5_const_principal,
                               mut p2: krb5_const_principal,
                               mut s1: *const libc::c_char,
                               mut s2: *const libc::c_char) -> krb5_boolean {
    return auth((*handle).context, opcode,
                (*handle).current_caller as krb5_const_principal, p1, p2, s1,
                s2, 0 as *const kadm5_policy_ent_rec,
                0 as libc::c_int as libc::c_long);
}
#[c2rust::src_loc = "319:1"]
unsafe extern "C" fn stub_auth_pol(mut handle: kadm5_server_handle_t,
                                   mut opcode: libc::c_int,
                                   mut policy: *const libc::c_char,
                                   mut polent: *const kadm5_policy_ent_rec,
                                   mut mask: libc::c_long) -> krb5_boolean {
    return auth((*handle).context, opcode,
                (*handle).current_caller as krb5_const_principal,
                0 as krb5_const_principal, 0 as krb5_const_principal, policy,
                0 as *const libc::c_char, polent, mask);
}
#[c2rust::src_loc = "327:1"]
unsafe extern "C" fn stub_auth_restrict(mut handle: kadm5_server_handle_t,
                                        mut opcode: libc::c_int,
                                        mut ent: kadm5_principal_ent_t,
                                        mut mask: *mut libc::c_long)
 -> krb5_boolean {
    return auth_restrict((*handle).context, opcode,
                         (*handle).current_caller as krb5_const_principal,
                         ent, mask);
}
/* Return true if the client authenticated to kadmin/changepw and princ is not
 * the client principal. */
#[c2rust::src_loc = "337:1"]
unsafe extern "C" fn changepw_not_self(mut handle: kadm5_server_handle_t,
                                       mut rqstp: *mut svc_req,
                                       mut princ: krb5_const_principal)
 -> krb5_boolean {
    return (cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                  gss_ctx_id_t),
                                gss_changepw_name) |
                (!gss_oldchangepw_name.is_null() &&
                     cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                           gss_ctx_id_t),
                                         gss_oldchangepw_name) != 0) as
                    libc::c_int != 0 &&
                krb5_principal_compare((*handle).context,
                                       (*handle).current_caller as
                                           krb5_const_principal, princ) == 0)
               as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "346:1"]
unsafe extern "C" fn ticket_is_initial(mut rqstp: *mut svc_req)
 -> krb5_boolean {
    let mut status: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut flags: krb5_flags = 0;
    status =
        gss_krb5_get_tkt_flags(&mut minor_stat,
                               (*rqstp).rq_svccred as gss_ctx_id_t,
                               &mut flags);
    if status != 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as krb5_boolean
    }
    return (flags & 0x400000 as libc::c_int != 0 as libc::c_int) as
               libc::c_int as krb5_boolean;
}
/* If a key change request is for the client's own principal, verify that the
 * client used an initial ticket and enforce the policy min_life. */
#[c2rust::src_loc = "360:1"]
unsafe extern "C" fn check_self_keychange(mut handle: kadm5_server_handle_t,
                                          mut rqstp: *mut svc_req,
                                          mut princ: krb5_principal)
 -> kadm5_ret_t {
    if krb5_principal_compare((*handle).context,
                              (*handle).current_caller as
                                  krb5_const_principal,
                              princ as krb5_const_principal) == 0 {
        return 0 as libc::c_int as kadm5_ret_t
    }
    if ticket_is_initial(rqstp) == 0 { return 43787582 as libc::c_long }
    return check_min_life(handle as *mut libc::c_void, princ,
                          0 as *mut libc::c_char,
                          0 as libc::c_int as libc::c_uint);
}
#[c2rust::src_loc = "374:1"]
unsafe extern "C" fn log_unauth(mut op: *mut libc::c_char,
                                mut target: *mut libc::c_char,
                                mut client: gss_buffer_t,
                                mut server: gss_buffer_t,
                                mut rqstp: *mut svc_req) -> libc::c_int {
    let mut tlen: size_t = 0;
    let mut clen: size_t = 0;
    let mut slen: size_t = 0;
    let mut tdots: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cdots: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sdots: *mut libc::c_char = 0 as *mut libc::c_char;
    tlen = strlen(target);
    trunc_name(&mut tlen, &mut tdots);
    clen = (*client).length;
    trunc_name(&mut clen, &mut cdots);
    slen = (*server).length;
    trunc_name(&mut slen, &mut sdots);
    /* okay to cast lengths to int because trunc_name limits max value */
    return krb5_klog_syslog(5 as libc::c_int,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Unauthorized request: %s, %.*s%s, client=%.*s%s, service=%.*s%s, addr=%s\x00"
                                         as *const u8 as *const libc::c_char),
                            op, tlen as libc::c_int, target, tdots,
                            clen as libc::c_int,
                            (*client).value as *mut libc::c_char, cdots,
                            slen as libc::c_int,
                            (*server).value as *mut libc::c_char, sdots,
                            client_addr((*rqstp).rq_xprt));
}
#[c2rust::src_loc = "402:1"]
unsafe extern "C" fn log_done(mut op: *mut libc::c_char,
                              mut target: *mut libc::c_char,
                              mut errmsg: *const libc::c_char,
                              mut client: gss_buffer_t,
                              mut server: gss_buffer_t,
                              mut rqstp: *mut svc_req) -> libc::c_int {
    let mut tlen: size_t = 0;
    let mut clen: size_t = 0;
    let mut slen: size_t = 0;
    let mut tdots: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cdots: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sdots: *mut libc::c_char = 0 as *mut libc::c_char;
    if errmsg.is_null() {
        errmsg =
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"success\x00" as *const u8 as *const libc::c_char)
    }
    tlen = strlen(target);
    trunc_name(&mut tlen, &mut tdots);
    clen = (*client).length;
    trunc_name(&mut clen, &mut cdots);
    slen = (*server).length;
    trunc_name(&mut slen, &mut sdots);
    /* okay to cast lengths to int because trunc_name limits max value */
    return krb5_klog_syslog(5 as libc::c_int,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Request: %s, %.*s%s, %s, client=%.*s%s, service=%.*s%s, addr=%s\x00"
                                         as *const u8 as *const libc::c_char),
                            op, tlen as libc::c_int, target, tdots, errmsg,
                            clen as libc::c_int,
                            (*client).value as *mut libc::c_char, cdots,
                            slen as libc::c_int,
                            (*server).value as *mut libc::c_char, sdots,
                            client_addr((*rqstp).rq_xprt));
}
#[no_mangle]
#[c2rust::src_loc = "433:1"]
pub unsafe extern "C" fn create_principal_2_svc(mut arg: *mut cprinc_arg,
                                                mut ret: *mut generic_ret,
                                                mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).rec.principal,
                   &mut handle, &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth_restrict(handle, 1 as libc::c_int, &mut (*arg).rec,
                                  &mut (*arg).mask) == 0 {
            (*ret).code = 43787522 as libc::c_long;
            log_unauth(b"kadm5_create_principal\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
        } else {
            (*ret).code =
                kadm5_create_principal(handle as *mut libc::c_void,
                                       &mut (*arg).rec, (*arg).mask,
                                       (*arg).passwd);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_create_principal\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "473:1"]
pub unsafe extern "C" fn create_principal3_2_svc(mut arg: *mut cprinc3_arg,
                                                 mut ret: *mut generic_ret,
                                                 mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).rec.principal,
                   &mut handle, &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth_restrict(handle, 1 as libc::c_int, &mut (*arg).rec,
                                  &mut (*arg).mask) == 0 {
            (*ret).code = 43787522 as libc::c_long;
            log_unauth(b"kadm5_create_principal\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
        } else {
            (*ret).code =
                kadm5_create_principal_3(handle as *mut libc::c_void,
                                         &mut (*arg).rec, (*arg).mask,
                                         (*arg).n_ks_tuple, (*arg).ks_tuple,
                                         (*arg).passwd);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_create_principal\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
/* Return KADM5_PROTECT_KEYS if KRB5_KDB_LOCKDOWN_KEYS is set for princ. */
#[c2rust::src_loc = "514:1"]
unsafe extern "C" fn check_lockdown_keys(mut handle: kadm5_server_handle_t,
                                         mut princ: krb5_principal)
 -> kadm5_ret_t {
    let mut rec: kadm5_principal_ent_rec =
        kadm5_principal_ent_rec{principal: 0 as *mut krb5_principal_data,
                                princ_expire_time: 0,
                                last_pwd_change: 0,
                                pw_expiration: 0,
                                max_life: 0,
                                mod_name: 0 as *mut krb5_principal_data,
                                mod_date: 0,
                                attributes: 0,
                                kvno: 0,
                                mkvno: 0,
                                policy: 0 as *mut libc::c_char,
                                aux_attributes: 0,
                                max_renewable_life: 0,
                                last_success: 0,
                                last_failed: 0,
                                fail_auth_count: 0,
                                n_key_data: 0,
                                n_tl_data: 0,
                                tl_data: 0 as *mut krb5_tl_data,
                                key_data: 0 as *mut krb5_key_data,};
    let mut ret: kadm5_ret_t = 0;
    ret =
        kadm5_get_principal(handle as *mut libc::c_void, princ, &mut rec,
                            0x10 as libc::c_int as libc::c_long);
    if ret != 0 { return ret }
    ret =
        if rec.attributes & 0x800000 as libc::c_int != 0 {
            43787581 as libc::c_long
        } else { 0 as libc::c_int as libc::c_long };
    kadm5_free_principal_ent(handle as *mut libc::c_void, &mut rec);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "528:1"]
pub unsafe extern "C" fn delete_principal_2_svc(mut arg: *mut dprinc_arg,
                                                mut ret: *mut generic_ret,
                                                mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth(handle, 8 as libc::c_int,
                         (*arg).princ as krb5_const_principal,
                         0 as krb5_const_principal, 0 as *const libc::c_char,
                         0 as *const libc::c_char) == 0 {
            (*ret).code = 43787524 as libc::c_long;
            log_unauth(b"kadm5_delete_principal\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
        } else {
            (*ret).code = check_lockdown_keys(handle, (*arg).princ);
            if (*ret).code == 43787581 as libc::c_long {
                log_unauth(b"kadm5_delete_principal\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           prime_arg, &mut client_name, &mut service_name,
                           rqstp);
                (*ret).code = 43787524 as libc::c_long
            }
        }
        if (*ret).code == 0 as libc::c_int as libc::c_long {
            (*ret).code =
                kadm5_delete_principal(handle as *mut libc::c_void,
                                       (*arg).princ)
        }
        if (*ret).code != 43787524 as libc::c_long {
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_delete_principal\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "577:1"]
pub unsafe extern "C" fn modify_principal_2_svc(mut arg: *mut mprinc_arg,
                                                mut ret: *mut generic_ret,
                                                mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).rec.principal,
                   &mut handle, &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth_restrict(handle, 2 as libc::c_int, &mut (*arg).rec,
                                  &mut (*arg).mask) == 0 {
            (*ret).code = 43787523 as libc::c_long;
            log_unauth(b"kadm5_modify_principal\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
        } else if (*arg).mask & 0x10 as libc::c_int as libc::c_long != 0 &&
                      (*arg).rec.attributes & 0x800000 as libc::c_int == 0 {
            (*ret).code = check_lockdown_keys(handle, (*arg).rec.principal);
            if (*ret).code == 43787581 as libc::c_long {
                log_unauth(b"kadm5_modify_principal\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           prime_arg, &mut client_name, &mut service_name,
                           rqstp);
                (*ret).code = 43787523 as libc::c_long
            }
        }
        if (*ret).code == 0 as libc::c_int as libc::c_long {
            (*ret).code =
                kadm5_modify_principal(handle as *mut libc::c_void,
                                       &mut (*arg).rec, (*arg).mask);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_modify_principal\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "625:1"]
pub unsafe extern "C" fn rename_principal_2_svc(mut arg: *mut rprinc_arg,
                                                mut ret: *mut generic_ret,
                                                mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prime_arg2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut tlen1: size_t = 0;
    let mut tlen2: size_t = 0;
    let mut clen: size_t = 0;
    let mut slen: size_t = 0;
    let mut tdots1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tdots2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cdots: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sdots: *mut libc::c_char = 0 as *mut libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, 0 as krb5_principal,
                   &mut handle, &mut (*ret).api_version, &mut client_name,
                   &mut service_name, 0 as *mut *mut libc::c_char);
    if !((*ret).code != 0) {
        if krb5_unparse_name((*handle).context,
                             (*arg).src as krb5_const_principal,
                             &mut prime_arg1) != 0 ||
               krb5_unparse_name((*handle).context,
                                 (*arg).dest as krb5_const_principal,
                                 &mut prime_arg2) != 0 {
            (*ret).code = 43787538 as libc::c_long
        } else {
            tlen1 = strlen(prime_arg1);
            trunc_name(&mut tlen1, &mut tdots1);
            tlen2 = strlen(prime_arg2);
            trunc_name(&mut tlen2, &mut tdots2);
            clen = client_name.length;
            trunc_name(&mut clen, &mut cdots);
            slen = service_name.length;
            trunc_name(&mut slen, &mut sdots);
            if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                     gss_ctx_id_t),
                                   gss_changepw_name) |
                   (!gss_oldchangepw_name.is_null() &&
                        cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred
                                                              as
                                                              gss_ctx_id_t),
                                            gss_oldchangepw_name) != 0) as
                       libc::c_int != 0 ||
                   stub_auth(handle, 9 as libc::c_int,
                             (*arg).src as krb5_const_principal,
                             (*arg).dest as krb5_const_principal,
                             0 as *const libc::c_char,
                             0 as *const libc::c_char) == 0 {
                (*ret).code = 43787525 as libc::c_long;
                log_unauth(b"kadm5_rename_principal\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           prime_arg1, &mut client_name, &mut service_name,
                           rqstp);
            } else {
                (*ret).code = check_lockdown_keys(handle, (*arg).src);
                if (*ret).code == 43787581 as libc::c_long {
                    log_unauth(b"kadm5_rename_principal\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               prime_arg1, &mut client_name,
                               &mut service_name, rqstp);
                    (*ret).code = 43787524 as libc::c_long
                }
            }
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                /* okay to cast lengths to int because trunc_name limits max value */
                krb5_klog_syslog(5 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Unauthorized request: kadm5_rename_principal, %.*s%s to %.*s%s, client=%.*s%s, service=%.*s%s, addr=%s\x00"
                                              as *const u8 as
                                              *const libc::c_char),
                                 tlen1 as libc::c_int, prime_arg1, tdots1,
                                 tlen2 as libc::c_int, prime_arg2, tdots2,
                                 clen as libc::c_int,
                                 client_name.value as *mut libc::c_char,
                                 cdots, slen as libc::c_int,
                                 service_name.value as *mut libc::c_char,
                                 sdots, client_addr((*rqstp).rq_xprt));
            } else {
                (*ret).code =
                    kadm5_rename_principal(handle as *mut libc::c_void,
                                           (*arg).src, (*arg).dest);
                if (*ret).code != 0 as libc::c_int as libc::c_long {
                    errmsg =
                        krb5_get_error_message((*handle).context,
                                               (*ret).code as krb5_error_code)
                }
                /* okay to cast lengths to int because trunc_name limits max value */
                krb5_klog_syslog(5 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Request: kadm5_rename_principal, %.*s%s to %.*s%s, %s, client=%.*s%s, service=%.*s%s, addr=%s\x00"
                                              as *const u8 as
                                              *const libc::c_char),
                                 tlen1 as libc::c_int, prime_arg1, tdots1,
                                 tlen2 as libc::c_int, prime_arg2, tdots2,
                                 if !errmsg.is_null() {
                                     errmsg
                                 } else {
                                     dgettext(b"mit-krb5\x00" as *const u8 as
                                                  *const libc::c_char,
                                              b"success\x00" as *const u8 as
                                                  *const libc::c_char) as
                                         *const libc::c_char
                                 }, clen as libc::c_int,
                                 client_name.value as *mut libc::c_char,
                                 cdots, slen as libc::c_int,
                                 service_name.value as *mut libc::c_char,
                                 sdots, client_addr((*rqstp).rq_xprt));
                if !errmsg.is_null() {
                    krb5_free_error_message((*handle).context, errmsg);
                }
            }
        }
    }
    free(prime_arg1 as *mut libc::c_void);
    free(prime_arg2 as *mut libc::c_void);
    stub_cleanup(handle, 0 as *mut libc::c_char, &mut client_name,
                 &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "709:1"]
pub unsafe extern "C" fn get_principal_2_svc(mut arg: *mut gprinc_arg,
                                             mut ret: *mut gprinc_ret,
                                             mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut funcname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        funcname =
            b"kadm5_get_principal\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        if changepw_not_self(handle, rqstp,
                             (*arg).princ as krb5_const_principal) != 0 ||
               stub_auth(handle, 10 as libc::c_int,
                         (*arg).princ as krb5_const_principal,
                         0 as krb5_const_principal, 0 as *const libc::c_char,
                         0 as *const libc::c_char) == 0 {
            (*ret).code = 43787521 as libc::c_long;
            log_unauth(funcname, prime_arg, &mut client_name,
                       &mut service_name, rqstp);
        } else {
            (*ret).code =
                kadm5_get_principal(handle as *mut libc::c_void, (*arg).princ,
                                    &mut (*ret).rec, (*arg).mask);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(funcname, prime_arg, errmsg, &mut client_name,
                     &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "750:1"]
pub unsafe extern "C" fn get_princs_2_svc(mut arg: *mut gprincs_arg,
                                          mut ret: *mut gprincs_ret,
                                          mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, 0 as krb5_principal,
                   &mut handle, &mut (*ret).api_version, &mut client_name,
                   &mut service_name, 0 as *mut *mut libc::c_char);
    if !((*ret).code != 0) {
        prime_arg = (*arg).exp;
        if prime_arg.is_null() {
            prime_arg =
                b"*\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth(handle, 13 as libc::c_int, 0 as krb5_const_principal,
                         0 as krb5_const_principal, 0 as *const libc::c_char,
                         0 as *const libc::c_char) == 0 {
            (*ret).code = 43787564 as libc::c_long;
            log_unauth(b"kadm5_get_principals\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
        } else {
            (*ret).code =
                kadm5_get_principals(handle as *mut libc::c_void, (*arg).exp,
                                     &mut (*ret).princs, &mut (*ret).count);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_get_principals\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, 0 as *mut libc::c_char, &mut client_name,
                 &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "793:1"]
pub unsafe extern "C" fn chpass_principal_2_svc(mut arg: *mut chpass_arg,
                                                mut ret: *mut generic_ret,
                                                mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        (*ret).code = check_lockdown_keys(handle, (*arg).princ);
        if (*ret).code != 0 as libc::c_int as libc::c_long {
            if (*ret).code == 43787581 as libc::c_long {
                log_unauth(b"kadm5_chpass_principal\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           prime_arg, &mut client_name, &mut service_name,
                           rqstp);
                (*ret).code = 43787565 as libc::c_long
            }
        } else if changepw_not_self(handle, rqstp,
                                    (*arg).princ as krb5_const_principal) != 0
                      ||
                      stub_auth(handle, 4 as libc::c_int,
                                (*arg).princ as krb5_const_principal,
                                0 as krb5_const_principal,
                                0 as *const libc::c_char,
                                0 as *const libc::c_char) == 0 {
            (*ret).code = 43787565 as libc::c_long;
            log_unauth(b"kadm5_chpass_principal\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
        } else {
            (*ret).code = check_self_keychange(handle, rqstp, (*arg).princ);
            if (*ret).code == 0 {
                (*ret).code =
                    kadm5_chpass_principal(handle as *mut libc::c_void,
                                           (*arg).princ, (*arg).pass)
            }
        }
        if (*ret).code != 43787565 as libc::c_long {
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_chpass_principal\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "843:1"]
pub unsafe extern "C" fn chpass_principal3_2_svc(mut arg: *mut chpass3_arg,
                                                 mut ret: *mut generic_ret,
                                                 mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        (*ret).code = check_lockdown_keys(handle, (*arg).princ);
        if (*ret).code != 0 as libc::c_int as libc::c_long {
            if (*ret).code == 43787581 as libc::c_long {
                log_unauth(b"kadm5_chpass_principal\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           prime_arg, &mut client_name, &mut service_name,
                           rqstp);
                (*ret).code = 43787565 as libc::c_long
            }
        } else if changepw_not_self(handle, rqstp,
                                    (*arg).princ as krb5_const_principal) != 0
                      ||
                      stub_auth(handle, 4 as libc::c_int,
                                (*arg).princ as krb5_const_principal,
                                0 as krb5_const_principal,
                                0 as *const libc::c_char,
                                0 as *const libc::c_char) == 0 {
            (*ret).code = 43787565 as libc::c_long;
            log_unauth(b"kadm5_chpass_principal\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
        } else {
            (*ret).code = check_self_keychange(handle, rqstp, (*arg).princ);
            if (*ret).code == 0 {
                (*ret).code =
                    kadm5_chpass_principal_3(handle as *mut libc::c_void,
                                             (*arg).princ, (*arg).keepold,
                                             (*arg).n_ks_tuple,
                                             (*arg).ks_tuple, (*arg).pass)
            }
        }
        if (*ret).code != 43787565 as libc::c_long {
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_chpass_principal\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "896:1"]
pub unsafe extern "C" fn setkey_principal_2_svc(mut arg: *mut setkey_arg,
                                                mut ret: *mut generic_ret,
                                                mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        (*ret).code = check_lockdown_keys(handle, (*arg).princ);
        if (*ret).code != 0 as libc::c_int as libc::c_long {
            if (*ret).code == 43787581 as libc::c_long {
                log_unauth(b"kadm5_setkey_principal\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           prime_arg, &mut client_name, &mut service_name,
                           rqstp);
                (*ret).code = 43787570 as libc::c_long
            }
        } else if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                        gss_ctx_id_t),
                                      gss_changepw_name) |
                      (!gss_oldchangepw_name.is_null() &&
                           cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred
                                                                 as
                                                                 gss_ctx_id_t),
                                               gss_oldchangepw_name) != 0) as
                          libc::c_int == 0 &&
                      stub_auth(handle, 6 as libc::c_int,
                                (*arg).princ as krb5_const_principal,
                                0 as krb5_const_principal,
                                0 as *const libc::c_char,
                                0 as *const libc::c_char) != 0 {
            (*ret).code =
                kadm5_setkey_principal(handle as *mut libc::c_void,
                                       (*arg).princ, (*arg).keyblocks,
                                       (*arg).n_keys)
        } else {
            log_unauth(b"kadm5_setkey_principal\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
            (*ret).code = 43787570 as libc::c_long
        }
        if (*ret).code != 43787570 as libc::c_long {
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_setkey_principal\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "945:1"]
pub unsafe extern "C" fn setkey_principal3_2_svc(mut arg: *mut setkey3_arg,
                                                 mut ret: *mut generic_ret,
                                                 mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        (*ret).code = check_lockdown_keys(handle, (*arg).princ);
        if (*ret).code != 0 as libc::c_int as libc::c_long {
            if (*ret).code == 43787581 as libc::c_long {
                log_unauth(b"kadm5_setkey_principal\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           prime_arg, &mut client_name, &mut service_name,
                           rqstp);
                (*ret).code = 43787570 as libc::c_long
            }
        } else if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                        gss_ctx_id_t),
                                      gss_changepw_name) |
                      (!gss_oldchangepw_name.is_null() &&
                           cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred
                                                                 as
                                                                 gss_ctx_id_t),
                                               gss_oldchangepw_name) != 0) as
                          libc::c_int == 0 &&
                      stub_auth(handle, 6 as libc::c_int,
                                (*arg).princ as krb5_const_principal,
                                0 as krb5_const_principal,
                                0 as *const libc::c_char,
                                0 as *const libc::c_char) != 0 {
            (*ret).code =
                kadm5_setkey_principal_3(handle as *mut libc::c_void,
                                         (*arg).princ, (*arg).keepold,
                                         (*arg).n_ks_tuple, (*arg).ks_tuple,
                                         (*arg).keyblocks, (*arg).n_keys)
        } else {
            log_unauth(b"kadm5_setkey_principal\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
            (*ret).code = 43787570 as libc::c_long
        }
        if (*ret).code != 43787570 as libc::c_long {
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_setkey_principal\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "995:1"]
pub unsafe extern "C" fn setkey_principal4_2_svc(mut arg: *mut setkey4_arg,
                                                 mut ret: *mut generic_ret,
                                                 mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        (*ret).code = check_lockdown_keys(handle, (*arg).princ);
        if (*ret).code != 0 as libc::c_int as libc::c_long {
            if (*ret).code == 43787581 as libc::c_long {
                log_unauth(b"kadm5_setkey_principal\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           prime_arg, &mut client_name, &mut service_name,
                           rqstp);
                (*ret).code = 43787570 as libc::c_long
            }
        } else if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                        gss_ctx_id_t),
                                      gss_changepw_name) |
                      (!gss_oldchangepw_name.is_null() &&
                           cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred
                                                                 as
                                                                 gss_ctx_id_t),
                                               gss_oldchangepw_name) != 0) as
                          libc::c_int == 0 &&
                      stub_auth(handle, 6 as libc::c_int,
                                (*arg).princ as krb5_const_principal,
                                0 as krb5_const_principal,
                                0 as *const libc::c_char,
                                0 as *const libc::c_char) != 0 {
            (*ret).code =
                kadm5_setkey_principal_4(handle as *mut libc::c_void,
                                         (*arg).princ, (*arg).keepold,
                                         (*arg).key_data, (*arg).n_key_data)
        } else {
            log_unauth(b"kadm5_setkey_principal\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
            (*ret).code = 43787570 as libc::c_long
        }
        if (*ret).code != 43787570 as libc::c_long {
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_setkey_principal\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
/* Empty out *keys / *nkeys if princ is protected with the lockdown
 * attribute, or if we fail to check. */
#[c2rust::src_loc = "1046:1"]
unsafe extern "C" fn chrand_check_lockdown(mut handle: kadm5_server_handle_t,
                                           mut princ: krb5_principal,
                                           mut keys: *mut *mut krb5_keyblock,
                                           mut nkeys: *mut libc::c_int)
 -> kadm5_ret_t {
    let mut ret: kadm5_ret_t = 0;
    let mut i: libc::c_int = 0;
    ret = check_lockdown_keys(handle, princ);
    if ret == 0 { return 0 as libc::c_int as kadm5_ret_t }
    i = 0 as libc::c_int;
    while i < *nkeys {
        krb5_free_keyblock_contents((*handle).context,
                                    &mut *(*keys).offset(i as isize));
        i += 1
    }
    free(*keys as *mut libc::c_void);
    *keys = 0 as *mut krb5_keyblock;
    *nkeys = 0 as libc::c_int;
    return if ret == 43787581 as libc::c_long {
               0 as libc::c_int as libc::c_long
           } else { ret };
}
#[no_mangle]
#[c2rust::src_loc = "1065:1"]
pub unsafe extern "C" fn chrand_principal_2_svc(mut arg: *mut chrand_arg,
                                                mut ret: *mut chrand_ret,
                                                mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut funcname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut k: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut nkeys: libc::c_int = 0;
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        funcname =
            b"kadm5_randkey_principal\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        if changepw_not_self(handle, rqstp,
                             (*arg).princ as krb5_const_principal) != 0 ||
               stub_auth(handle, 5 as libc::c_int,
                         (*arg).princ as krb5_const_principal,
                         0 as krb5_const_principal, 0 as *const libc::c_char,
                         0 as *const libc::c_char) == 0 {
            (*ret).code = 43787565 as libc::c_long;
            log_unauth(funcname, prime_arg, &mut client_name,
                       &mut service_name, rqstp);
        } else {
            (*ret).code = check_self_keychange(handle, rqstp, (*arg).princ);
            if (*ret).code == 0 {
                (*ret).code =
                    kadm5_randkey_principal(handle as *mut libc::c_void,
                                            (*arg).princ, &mut k, &mut nkeys)
            }
        }
        if (*ret).code == 0 as libc::c_int as libc::c_long {
            (*ret).code =
                chrand_check_lockdown(handle, (*arg).princ, &mut k,
                                      &mut nkeys);
            if (*ret).code == 43787581 as libc::c_long {
                (*ret).code = 0 as libc::c_int as kadm5_ret_t
            }
            (*ret).keys = k;
            (*ret).n_keys = nkeys
        }
        if (*ret).code != 43787565 as libc::c_long {
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(funcname, prime_arg, errmsg, &mut client_name,
                     &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1121:1"]
pub unsafe extern "C" fn chrand_principal3_2_svc(mut arg: *mut chrand3_arg,
                                                 mut ret: *mut chrand_ret,
                                                 mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut funcname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut k: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut nkeys: libc::c_int = 0;
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        funcname =
            b"kadm5_randkey_principal\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        if changepw_not_self(handle, rqstp,
                             (*arg).princ as krb5_const_principal) != 0 ||
               stub_auth(handle, 5 as libc::c_int,
                         (*arg).princ as krb5_const_principal,
                         0 as krb5_const_principal, 0 as *const libc::c_char,
                         0 as *const libc::c_char) == 0 {
            (*ret).code = 43787565 as libc::c_long;
            log_unauth(funcname, prime_arg, &mut client_name,
                       &mut service_name, rqstp);
        } else {
            (*ret).code = check_self_keychange(handle, rqstp, (*arg).princ);
            if (*ret).code == 0 {
                (*ret).code =
                    kadm5_randkey_principal_3(handle as *mut libc::c_void,
                                              (*arg).princ, (*arg).keepold,
                                              (*arg).n_ks_tuple,
                                              (*arg).ks_tuple, &mut k,
                                              &mut nkeys)
            }
        }
        if (*ret).code == 0 as libc::c_int as libc::c_long {
            (*ret).code =
                chrand_check_lockdown(handle, (*arg).princ, &mut k,
                                      &mut nkeys);
            if (*ret).code == 43787581 as libc::c_long {
                (*ret).code = 0 as libc::c_int as kadm5_ret_t
            }
            (*ret).keys = k;
            (*ret).n_keys = nkeys
        }
        if (*ret).code != 43787565 as libc::c_long {
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(funcname, prime_arg, errmsg, &mut client_name,
                     &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1180:1"]
pub unsafe extern "C" fn create_policy_2_svc(mut arg: *mut cpol_arg,
                                             mut ret: *mut generic_ret,
                                             mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, 0 as krb5_principal,
                   &mut handle, &mut (*ret).api_version, &mut client_name,
                   &mut service_name, 0 as *mut *mut libc::c_char);
    if !((*ret).code != 0) {
        prime_arg = (*arg).rec.policy;
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth_pol(handle, 14 as libc::c_int, (*arg).rec.policy,
                             &mut (*arg).rec, (*arg).mask) == 0 {
            (*ret).code = 43787522 as libc::c_long;
            log_unauth(b"kadm5_create_policy\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
        } else {
            (*ret).code =
                kadm5_create_policy(handle as *mut libc::c_void,
                                    &mut (*arg).rec, (*arg).mask);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_create_policy\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char,
                     if prime_arg.is_null() {
                         b"(null)\x00" as *const u8 as *const libc::c_char
                     } else { prime_arg as *const libc::c_char } as
                         *mut libc::c_char, errmsg, &mut client_name,
                     &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, 0 as *mut libc::c_char, &mut client_name,
                 &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1222:1"]
pub unsafe extern "C" fn delete_policy_2_svc(mut arg: *mut dpol_arg,
                                             mut ret: *mut generic_ret,
                                             mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, 0 as krb5_principal,
                   &mut handle, &mut (*ret).api_version, &mut client_name,
                   &mut service_name, 0 as *mut *mut libc::c_char);
    if !((*ret).code != 0) {
        prime_arg = (*arg).name;
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth(handle, 16 as libc::c_int, 0 as krb5_const_principal,
                         0 as krb5_const_principal, (*arg).name,
                         0 as *const libc::c_char) == 0 {
            log_unauth(b"kadm5_delete_policy\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
            (*ret).code = 43787524 as libc::c_long
        } else {
            (*ret).code =
                kadm5_delete_policy(handle as *mut libc::c_void, (*arg).name);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_delete_policy\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char,
                     if prime_arg.is_null() {
                         b"(null)\x00" as *const u8 as *const libc::c_char
                     } else { prime_arg as *const libc::c_char } as
                         *mut libc::c_char, errmsg, &mut client_name,
                     &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, 0 as *mut libc::c_char, &mut client_name,
                 &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1262:1"]
pub unsafe extern "C" fn modify_policy_2_svc(mut arg: *mut mpol_arg,
                                             mut ret: *mut generic_ret,
                                             mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, 0 as krb5_principal,
                   &mut handle, &mut (*ret).api_version, &mut client_name,
                   &mut service_name, 0 as *mut *mut libc::c_char);
    if !((*ret).code != 0) {
        prime_arg = (*arg).rec.policy;
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth_pol(handle, 15 as libc::c_int, (*arg).rec.policy,
                             &mut (*arg).rec, (*arg).mask) == 0 {
            log_unauth(b"kadm5_modify_policy\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
            (*ret).code = 43787523 as libc::c_long
        } else {
            (*ret).code =
                kadm5_modify_policy(handle as *mut libc::c_void,
                                    &mut (*arg).rec, (*arg).mask);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_modify_policy\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char,
                     if prime_arg.is_null() {
                         b"(null)\x00" as *const u8 as *const libc::c_char
                     } else { prime_arg as *const libc::c_char } as
                         *mut libc::c_char, errmsg, &mut client_name,
                     &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, 0 as *mut libc::c_char, &mut client_name,
                 &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1303:1"]
pub unsafe extern "C" fn get_policy_2_svc(mut arg: *mut gpol_arg,
                                          mut ret: *mut gpol_ret,
                                          mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut funcname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut ret2: kadm5_ret_t = 0;
    let mut caller_ent: kadm5_principal_ent_rec =
        kadm5_principal_ent_rec{principal: 0 as *mut krb5_principal_data,
                                princ_expire_time: 0,
                                last_pwd_change: 0,
                                pw_expiration: 0,
                                max_life: 0,
                                mod_name: 0 as *mut krb5_principal_data,
                                mod_date: 0,
                                attributes: 0,
                                kvno: 0,
                                mkvno: 0,
                                policy: 0 as *mut libc::c_char,
                                aux_attributes: 0,
                                max_renewable_life: 0,
                                last_success: 0,
                                last_failed: 0,
                                fail_auth_count: 0,
                                n_key_data: 0,
                                n_tl_data: 0,
                                tl_data: 0 as *mut krb5_tl_data,
                                key_data: 0 as *mut krb5_key_data,};
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut cpolicy: *const libc::c_char = 0 as *const libc::c_char;
    memset(&mut caller_ent as *mut kadm5_principal_ent_rec as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<kadm5_principal_ent_rec>() as libc::c_ulong);
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, 0 as krb5_principal,
                   &mut handle, &mut (*ret).api_version, &mut client_name,
                   &mut service_name, 0 as *mut *mut libc::c_char);
    if !((*ret).code != 0) {
        funcname =
            b"kadm5_get_policy\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        prime_arg = (*arg).name;
        /* Look up the client principal's policy value. */
        ret2 =
            kadm5_get_principal((*handle).lhandle as *mut libc::c_void,
                                (*handle).current_caller, &mut caller_ent,
                                0x41ffff as libc::c_int as libc::c_long);
        if ret2 == 0 as libc::c_int as libc::c_long &&
               caller_ent.aux_attributes &
                   0x800 as libc::c_int as libc::c_long != 0 {
            cpolicy = caller_ent.policy
        }
        (*ret).code = 43787521 as libc::c_long;
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 &&
               (cpolicy.is_null() ||
                    strcmp(cpolicy, (*arg).name) != 0 as libc::c_int) ||
               stub_auth(handle, 17 as libc::c_int, 0 as krb5_const_principal,
                         0 as krb5_const_principal, (*arg).name, cpolicy) == 0
           {
            (*ret).code = 43787521 as libc::c_long;
            log_unauth(funcname, prime_arg, &mut client_name,
                       &mut service_name, rqstp);
        } else {
            (*ret).code =
                kadm5_get_policy(handle as *mut libc::c_void, (*arg).name,
                                 &mut (*ret).rec);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(funcname,
                     if prime_arg.is_null() {
                         b"(null)\x00" as *const u8 as *const libc::c_char
                     } else { prime_arg as *const libc::c_char } as
                         *mut libc::c_char, errmsg, &mut client_name,
                     &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    kadm5_free_principal_ent((*handle).lhandle as *mut libc::c_void,
                             &mut caller_ent);
    stub_cleanup(handle, 0 as *mut libc::c_char, &mut client_name,
                 &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1356:1"]
pub unsafe extern "C" fn get_pols_2_svc(mut arg: *mut gpols_arg,
                                        mut ret: *mut gpols_ret,
                                        mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, 0 as krb5_principal,
                   &mut handle, &mut (*ret).api_version, &mut client_name,
                   &mut service_name, 0 as *mut *mut libc::c_char);
    if !((*ret).code != 0) {
        prime_arg = (*arg).exp;
        if prime_arg.is_null() {
            prime_arg =
                b"*\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth(handle, 18 as libc::c_int, 0 as krb5_const_principal,
                         0 as krb5_const_principal, 0 as *const libc::c_char,
                         0 as *const libc::c_char) == 0 {
            (*ret).code = 43787564 as libc::c_long;
            log_unauth(b"kadm5_get_policies\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
        } else {
            (*ret).code =
                kadm5_get_policies(handle as *mut libc::c_void, (*arg).exp,
                                   &mut (*ret).pols, &mut (*ret).count);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_get_policies\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, 0 as *mut libc::c_char, &mut client_name,
                 &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1398:1"]
pub unsafe extern "C" fn get_privs_2_svc(mut arg: *mut krb5_ui_4,
                                         mut ret: *mut getprivs_ret,
                                         mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup(*arg, rqstp, 0 as krb5_principal, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, 0 as *mut *mut libc::c_char);
    if !((*ret).code != 0) {
        (*ret).code =
            kadm5_get_privs(handle as *mut libc::c_void, &mut (*ret).privs);
        if (*ret).code != 0 as libc::c_int as libc::c_long {
            errmsg =
                krb5_get_error_message((*handle).context,
                                       (*ret).code as krb5_error_code)
        }
        log_done(b"kadm5_get_privs\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char,
                 client_name.value as *mut libc::c_char, errmsg,
                 &mut client_name, &mut service_name, rqstp);
        if !errmsg.is_null() {
            krb5_free_error_message((*handle).context, errmsg);
        }
    }
    stub_cleanup(handle, 0 as *mut libc::c_char, &mut client_name,
                 &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1426:1"]
pub unsafe extern "C" fn purgekeys_2_svc(mut arg: *mut purgekeys_arg,
                                         mut ret: *mut generic_ret,
                                         mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut funcname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        funcname =
            b"kadm5_purgekeys\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth(handle, 7 as libc::c_int,
                         (*arg).princ as krb5_const_principal,
                         0 as krb5_const_principal, 0 as *const libc::c_char,
                         0 as *const libc::c_char) == 0 {
            (*ret).code = 43787523 as libc::c_long;
            log_unauth(funcname, prime_arg, &mut client_name,
                       &mut service_name, rqstp);
        } else {
            (*ret).code =
                kadm5_purgekeys(handle as *mut libc::c_void, (*arg).princ,
                                (*arg).keepkvno);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(funcname, prime_arg, errmsg, &mut client_name,
                     &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1465:1"]
pub unsafe extern "C" fn get_strings_2_svc(mut arg: *mut gstrings_arg,
                                           mut ret: *mut gstrings_ret,
                                           mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth(handle, 11 as libc::c_int,
                         (*arg).princ as krb5_const_principal,
                         0 as krb5_const_principal, 0 as *const libc::c_char,
                         0 as *const libc::c_char) == 0 {
            (*ret).code = 43787521 as libc::c_long;
            log_unauth(b"kadm5_get_strings\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
        } else {
            (*ret).code =
                kadm5_get_strings(handle as *mut libc::c_void, (*arg).princ,
                                  &mut (*ret).strings, &mut (*ret).count);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_get_strings\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1503:1"]
pub unsafe extern "C" fn set_string_2_svc(mut arg: *mut sstring_arg,
                                          mut ret: *mut generic_ret,
                                          mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int != 0 ||
               stub_auth(handle, 3 as libc::c_int,
                         (*arg).princ as krb5_const_principal,
                         0 as krb5_const_principal, (*arg).key, (*arg).value)
                   == 0 {
            (*ret).code = 43787523 as libc::c_long;
            log_unauth(b"kadm5_mod_strings\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
        } else {
            (*ret).code =
                kadm5_set_string(handle as *mut libc::c_void, (*arg).princ,
                                 (*arg).key, (*arg).value);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_mod_strings\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1541:1"]
pub unsafe extern "C" fn init_2_svc(mut arg: *mut krb5_ui_4,
                                    mut ret: *mut generic_ret,
                                    mut rqstp: *mut svc_req) -> libc::c_int {
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut clen: size_t = 0;
    let mut slen: size_t = 0;
    let mut cdots: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sdots: *mut libc::c_char = 0 as *mut libc::c_char;
    (*ret).code =
        stub_setup(*arg, rqstp, 0 as krb5_principal, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, 0 as *mut *mut libc::c_char);
    if !((*ret).code != 0) {
        if (*ret).code != 0 as libc::c_int as libc::c_long {
            errmsg =
                krb5_get_error_message((*handle).context,
                                       (*ret).code as krb5_error_code)
        }
        clen = client_name.length;
        trunc_name(&mut clen, &mut cdots);
        slen = service_name.length;
        trunc_name(&mut slen, &mut sdots);
        /* okay to cast lengths to int because trunc_name limits max value */
        krb5_klog_syslog(5 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"Request: kadm5_init, %.*s%s, %s, client=%.*s%s, service=%.*s%s, addr=%s, vers=%d, flavor=%d\x00"
                                      as *const u8 as *const libc::c_char),
                         clen as libc::c_int,
                         client_name.value as *mut libc::c_char, cdots,
                         if !errmsg.is_null() {
                             errmsg
                         } else {
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"success\x00" as *const u8 as
                                          *const libc::c_char) as
                                 *const libc::c_char
                         }, clen as libc::c_int,
                         client_name.value as *mut libc::c_char, cdots,
                         slen as libc::c_int,
                         service_name.value as *mut libc::c_char, sdots,
                         client_addr((*rqstp).rq_xprt),
                         (*ret).api_version &
                             !(0x12345700 as libc::c_int) as libc::c_uint,
                         (*rqstp).rq_cred.oa_flavor);
        if !errmsg.is_null() {
            krb5_free_error_message((*handle).context, errmsg);
        }
    }
    stub_cleanup(handle, 0 as *mut libc::c_char, &mut client_name,
                 &mut service_name);
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1582:1"]
pub unsafe extern "C" fn rqst2name(mut rqstp: *mut svc_req) -> gss_name_t {
    if (*rqstp).rq_cred.oa_flavor == 6 as libc::c_int {
        return (*rqstp).rq_clntname as gss_name_t
    } else { return (*rqstp).rq_clntcred as gss_name_t };
}
#[no_mangle]
#[c2rust::src_loc = "1592:1"]
pub unsafe extern "C" fn get_principal_keys_2_svc(mut arg: *mut getpkeys_arg,
                                                  mut ret: *mut getpkeys_ret,
                                                  mut rqstp: *mut svc_req)
 -> libc::c_int {
    let mut prime_arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut client_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut service_name: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    (*ret).code =
        stub_setup((*arg).api_version, rqstp, (*arg).princ, &mut handle,
                   &mut (*ret).api_version, &mut client_name,
                   &mut service_name, &mut prime_arg);
    if !((*ret).code != 0) {
        if cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                 gss_ctx_id_t),
                               gss_changepw_name) |
               (!gss_oldchangepw_name.is_null() &&
                    cmp_gss_names_rel_1(acceptor_name((*rqstp).rq_svccred as
                                                          gss_ctx_id_t),
                                        gss_oldchangepw_name) != 0) as
                   libc::c_int == 0 &&
               stub_auth(handle, 12 as libc::c_int,
                         (*arg).princ as krb5_const_principal,
                         0 as krb5_const_principal, 0 as *const libc::c_char,
                         0 as *const libc::c_char) != 0 {
            (*ret).code =
                kadm5_get_principal_keys(handle as *mut libc::c_void,
                                         (*arg).princ, (*arg).kvno,
                                         &mut (*ret).key_data,
                                         &mut (*ret).n_key_data)
        } else {
            log_unauth(b"kadm5_get_principal_keys\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char,
                       prime_arg, &mut client_name, &mut service_name, rqstp);
            (*ret).code = 43787580 as libc::c_long
        }
        if (*ret).code == 0 as libc::c_int as libc::c_long {
            (*ret).code = check_lockdown_keys(handle, (*arg).princ);
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                kadm5_free_kadm5_key_data((*handle).context,
                                          (*ret).n_key_data, (*ret).key_data);
                (*ret).key_data = 0 as *mut kadm5_key_data;
                (*ret).n_key_data = 0 as libc::c_int
            }
            if (*ret).code == 43787581 as libc::c_long {
                log_unauth(b"kadm5_get_principal_keys\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           prime_arg, &mut client_name, &mut service_name,
                           rqstp);
                (*ret).code = 43787580 as libc::c_long
            }
        }
        if (*ret).code != 43787580 as libc::c_long {
            if (*ret).code != 0 as libc::c_int as libc::c_long {
                errmsg =
                    krb5_get_error_message((*handle).context,
                                           (*ret).code as krb5_error_code)
            }
            log_done(b"kadm5_get_principal_keys\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char, prime_arg,
                     errmsg, &mut client_name, &mut service_name, rqstp);
            if !errmsg.is_null() {
                krb5_free_error_message((*handle).context, errmsg);
            }
        }
    }
    stub_cleanup(handle, prime_arg, &mut client_name, &mut service_name);
    return 1 as libc::c_int;
}
