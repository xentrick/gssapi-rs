use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:6"]
pub mod types_h {
    #[c2rust::src_loc = "31:1"]
    pub type __u_char = libc::c_uchar;
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
}
#[c2rust::header_src = "/usr/include/sys/types.h:6"]
pub mod sys_types_h {
    #[c2rust::src_loc = "33:1"]
    pub type u_char = __u_char;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_char, __u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:6"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:6"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:6"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:6"]
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
    use super::sys_types_h::{caddr_t, u_int, u_char};
    use super::gssrpc_types_h::rpc_inline_t;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "252:1"]
        pub fn gssrpc_xdr_int(_: *mut XDR, _: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "253:1"]
        pub fn gssrpc_xdr_u_int(_: *mut XDR, _: *mut u_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "254:1"]
        pub fn gssrpc_xdr_long(_: *mut XDR, _: *mut libc::c_long)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "258:1"]
        pub fn gssrpc_xdr_bool(_: *mut XDR, _: *mut libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn gssrpc_xdr_array(_: *mut XDR, _: *mut caddr_t, _: *mut u_int,
                                _: u_int, _: u_int, _: xdrproc_t)
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
        #[c2rust::src_loc = "268:1"]
        pub fn gssrpc_xdr_u_char(_: *mut XDR, _: *mut u_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "297:1"]
        pub fn gssrpc_xdr_int32(_: *mut XDR, _: *mut int32_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "298:1"]
        pub fn gssrpc_xdr_u_int32(_: *mut XDR, _: *mut uint32_t)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:7"]
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
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /*
 * end wordsize.h
 */
    /*
 * begin "base-defs.h"
 */
    /*
 * Basic definitions for Kerberos V5 library
 */
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
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:9"]
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
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "156:16"]
    pub struct krb5_string_attr_st {
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    /* NOT saved */
    /* String attributes (currently stored inside tl-data) map C string keys to
 * values.  They can be set via kadmin and consumed by KDC plugins. */
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
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /* # of array elements */
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
    /* Length, data */
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:9"]
pub mod admin_h {
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
    #[c2rust::src_loc = "284:16"]
    pub struct _kadm5_key_data {
        pub kvno: krb5_kvno,
        pub key: krb5_keyblock,
        pub salt: krb5_keysalt,
    }
    #[c2rust::src_loc = "284:1"]
    pub type kadm5_key_data = _kadm5_key_data;
    use super::krb5_h::{krb5_principal, krb5_timestamp, krb5_deltat,
                        krb5_flags, krb5_kvno, krb5_int16, krb5_keyblock,
                        krb5_context, krb5_error_code};
    use super::kdb_h::{krb5_tl_data, krb5_key_data, krb5_keysalt};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "465:1"]
        pub fn kadm5_init_krb5_context(_: *mut krb5_context)
         -> krb5_error_code;
    }
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/kadm_rpc.h:10"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:6"]
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
#[c2rust::header_src = "/usr/include/errno.h:8"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:11"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "90:14"]
        pub fn memchr(_: *const libc::c_void, _: libc::c_int,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
pub use self::types_h::{__u_char, __u_int, __uint8_t, __int16_t, __uint16_t,
                        __int32_t, __uint32_t, __caddr_t};
pub use self::sys_types_h::{u_char, u_int, caddr_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_int, gssrpc_xdr_u_int,
                      gssrpc_xdr_long, gssrpc_xdr_bool, gssrpc_xdr_array,
                      gssrpc_xdr_bytes, gssrpc_xdr_opaque, gssrpc_xdr_u_char,
                      gssrpc_xdr_int32, gssrpc_xdr_u_int32};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_context, _krb5_keyblock,
                       krb5_keyblock, _krb5_context, krb5_parse_name,
                       krb5_unparse_name, krb5_free_principal};
pub use self::kdb_h::{_krb5_tl_data, krb5_tl_data, krb5_string_attr_st,
                      krb5_string_attr, _krb5_key_data, krb5_key_data,
                      _krb5_keysalt, krb5_keysalt, __krb5_key_salt_tuple,
                      krb5_key_salt_tuple};
pub use self::admin_h::{kadm5_ret_t, _kadm5_principal_ent_t,
                        kadm5_principal_ent_rec, _kadm5_policy_ent_t,
                        kadm5_policy_ent_rec, _kadm5_key_data, kadm5_key_data,
                        kadm5_init_krb5_context};
pub use self::kadm_rpc_h::{cprinc_arg, cprinc3_arg, generic_ret, dprinc_arg,
                           mprinc_arg, rprinc_arg, gprincs_arg, gprincs_ret,
                           chpass_arg, chpass3_arg, setkey_arg, setkey3_arg,
                           setkey4_arg, chrand_arg, chrand3_arg, chrand_ret,
                           gprinc_arg, gprinc_ret, cpol_arg, dpol_arg,
                           mpol_arg, gpol_arg, gpol_ret, gpols_arg, gpols_ret,
                           getprivs_ret, purgekeys_arg, gstrings_arg,
                           gstrings_ret, sstring_arg, getpkeys_arg,
                           getpkeys_ret};
use self::stdlib_h::{malloc, free};
use self::errno_h::__errno_location;
use self::string_h::{memchr, strlen, memset};
/* -*- mode: c; c-file-style: "bsd"; indent-tabs-mode: t -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 *
 */
/*
 * Function: xdr_ui_4
 *
 * Purpose: XDR function which serves as a wrapper for xdr_u_int32,
 * to prevent compiler warnings about type clashes between u_int32
 * and krb5_ui_4.
 */
#[no_mangle]
#[c2rust::src_loc = "28:1"]
pub unsafe extern "C" fn xdr_ui_4(mut xdrs: *mut XDR,
                                  mut objp: *mut krb5_ui_4) -> libc::c_int {
    /* Assumes that krb5_ui_4 and u_int32 are both four bytes long.
     This should not be a harmful assumption. */
    return gssrpc_xdr_u_int32(xdrs, objp as *mut uint32_t);
}
/*
 * Function: xdr_nullstring
 *
 * Purpose: XDR function for "strings" that are either NULL-terminated
 * or NULL.
 */
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn xdr_nullstring(mut xdrs: *mut XDR,
                                        mut objp: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut size: u_int = 0;
    if (*xdrs).x_op as libc::c_uint ==
           XDR_ENCODE as libc::c_int as libc::c_uint {
        if (*objp).is_null() {
            size = 0 as libc::c_int as u_int
        } else {
            size =
                strlen(*objp).wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as u_int
        }
    }
    if gssrpc_xdr_u_int(xdrs, &mut size) == 0 { return 0 as libc::c_int }
    match (*xdrs).x_op as libc::c_uint {
        1 => {
            if size == 0 as libc::c_int as libc::c_uint {
                *objp = 0 as *mut libc::c_char;
                return 1 as libc::c_int
            } else {
                if (*objp).is_null() {
                    *objp =
                        malloc(size as libc::c_ulong) as *mut libc::c_char;
                    if (*objp).is_null() {
                        *__errno_location() = 12 as libc::c_int;
                        return 0 as libc::c_int
                    }
                }
            }
            if gssrpc_xdr_opaque(xdrs, *objp, size) == 0 {
                return 0 as libc::c_int
            }
            /* Check that the unmarshalled bytes are a C string. */
            if *(*objp).offset(size.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint) as isize)
                   as libc::c_int != '\u{0}' as i32 {
                return 0 as libc::c_int
            }
            if !memchr(*objp as *const libc::c_void, '\u{0}' as i32,
                       size.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                           libc::c_ulong).is_null() {
                return 0 as libc::c_int
            }
            return 1 as libc::c_int
        }
        0 => {
            if size != 0 as libc::c_int as libc::c_uint {
                return gssrpc_xdr_opaque(xdrs, *objp, size)
            }
            return 1 as libc::c_int
        }
        2 => {
            if !(*objp).is_null() { free(*objp as *mut libc::c_void); }
            *objp = 0 as *mut libc::c_char;
            return 1 as libc::c_int
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
 * Function: xdr_nulltype
 *
 * Purpose: XDR function for arbitrary pointer types that are either
 * NULL or contain data.
 */
#[no_mangle]
#[c2rust::src_loc = "97:1"]
pub unsafe extern "C" fn xdr_nulltype(mut xdrs: *mut XDR,
                                      mut objp: *mut *mut libc::c_void,
                                      mut proc_0: xdrproc_t) -> libc::c_int {
    let mut null: libc::c_int = 0;
    match (*xdrs).x_op as libc::c_uint {
        1 => {
            if gssrpc_xdr_bool(xdrs, &mut null) == 0 {
                return 0 as libc::c_int
            }
            if null != 0 {
                *objp = 0 as *mut libc::c_void;
                return 1 as libc::c_int
            }
            return ::std::mem::transmute::<_,
                                           fn(_: _, _: _)
                                               ->
                                                   libc::c_int>(Some(proc_0.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                      objp)
        }
        0 => {
            if (*objp).is_null() {
                null = 1 as libc::c_int
            } else { null = 0 as libc::c_int }
            if gssrpc_xdr_bool(xdrs, &mut null) == 0 {
                return 0 as libc::c_int
            }
            if null == 0 as libc::c_int {
                return ::std::mem::transmute::<_,
                                               fn(_: _, _: _)
                                                   ->
                                                       libc::c_int>(Some(proc_0.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                          objp)
            }
            return 1 as libc::c_int
        }
        2 => {
            if !(*objp).is_null() {
                return ::std::mem::transmute::<_,
                                               fn(_: _, _: _)
                                                   ->
                                                       libc::c_int>(Some(proc_0.expect("non-null function pointer")).expect("non-null function pointer"))(xdrs,
                                                                                                                                                          objp)
            }
            return 1 as libc::c_int
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "131:1"]
pub unsafe extern "C" fn xdr_krb5_timestamp(mut xdrs: *mut XDR,
                                            mut objp: *mut krb5_timestamp)
 -> libc::c_int {
    /* This assumes that int32 and krb5_timestamp are the same size.
     This shouldn't be a problem, since we've got a unit test which
     checks for this. */
    if gssrpc_xdr_int32(xdrs, objp as *mut int32_t) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "143:1"]
pub unsafe extern "C" fn xdr_krb5_kvno(mut xdrs: *mut XDR,
                                       mut objp: *mut krb5_kvno)
 -> libc::c_int {
    return gssrpc_xdr_u_int(xdrs, objp);
}
#[no_mangle]
#[c2rust::src_loc = "149:1"]
pub unsafe extern "C" fn xdr_krb5_deltat(mut xdrs: *mut XDR,
                                         mut objp: *mut krb5_deltat)
 -> libc::c_int {
    /* This assumes that int32 and krb5_deltat are the same size.
     This shouldn't be a problem, since we've got a unit test which
     checks for this. */
    if gssrpc_xdr_int32(xdrs, objp as *mut int32_t) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "161:1"]
pub unsafe extern "C" fn xdr_krb5_flags(mut xdrs: *mut XDR,
                                        mut objp: *mut krb5_flags)
 -> libc::c_int {
    /* This assumes that int32 and krb5_flags are the same size.
     This shouldn't be a problem, since we've got a unit test which
     checks for this. */
    if gssrpc_xdr_int32(xdrs, objp as *mut int32_t) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "173:1"]
pub unsafe extern "C" fn xdr_krb5_ui_4(mut xdrs: *mut XDR,
                                       mut objp: *mut krb5_ui_4)
 -> libc::c_int {
    if gssrpc_xdr_u_int32(xdrs, objp as *mut uint32_t) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub unsafe extern "C" fn xdr_krb5_int16(mut xdrs: *mut XDR,
                                        mut objp: *mut krb5_int16)
 -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = *objp as libc::c_int;
    if gssrpc_xdr_int(xdrs, &mut tmp) == 0 { return 0 as libc::c_int }
    *objp = tmp as krb5_int16;
    return 1 as libc::c_int;
}
/*
 * Function: xdr_krb5_ui_2
 *
 * Purpose: XDR function which serves as a wrapper for xdr_u_int,
 * to prevent compiler warnings about type clashes between u_int
 * and krb5_ui_2.
 */
#[no_mangle]
#[c2rust::src_loc = "204:1"]
pub unsafe extern "C" fn xdr_krb5_ui_2(mut xdrs: *mut XDR,
                                       mut objp: *mut krb5_ui_2)
 -> libc::c_int {
    let mut tmp: libc::c_uint = 0;
    tmp = *objp as libc::c_uint;
    if gssrpc_xdr_u_int(xdrs, &mut tmp) == 0 { return 0 as libc::c_int }
    *objp = tmp as krb5_ui_2;
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "221:1"]
unsafe extern "C" fn xdr_krb5_boolean(mut xdrs: *mut XDR,
                                      mut kbool: *mut krb5_boolean)
 -> libc::c_int {
    let mut val: libc::c_int = 0;
    match (*xdrs).x_op as libc::c_uint {
        1 => {
            if gssrpc_xdr_bool(xdrs, &mut val) == 0 {
                return 0 as libc::c_int
            }
            *kbool =
                if val == 0 as libc::c_int {
                    0 as libc::c_int
                } else { 1 as libc::c_int } as krb5_boolean;
            return 1 as libc::c_int
        }
        0 => {
            val =
                if *kbool != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
            return gssrpc_xdr_bool(xdrs, &mut val)
        }
        2 => { return 1 as libc::c_int }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "244:1"]
pub unsafe extern "C" fn xdr_krb5_key_data_nocontents(mut xdrs: *mut XDR,
                                                      mut objp:
                                                          *mut krb5_key_data)
 -> libc::c_int {
    /*
      * Note that this function intentionally DOES NOT tranfer key
      * length or contents!  xdr_krb5_key_data in adb_xdr.c does, but
      * that is only for use within the server-side library.
      */
    let mut tmp: libc::c_uint = 0;
    if (*xdrs).x_op as libc::c_uint ==
           XDR_DECODE as libc::c_int as libc::c_uint {
        memset(objp as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<krb5_key_data>() as libc::c_ulong);
    }
    if xdr_krb5_int16(xdrs, &mut (*objp).key_data_ver) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_ui_2(xdrs, &mut (*objp).key_data_kvno) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_int16(xdrs,
                      &mut *(*objp).key_data_type.as_mut_ptr().offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize))
           == 0 {
        return 0 as libc::c_int
    }
    if (*objp).key_data_ver as libc::c_int > 1 as libc::c_int {
        if xdr_krb5_int16(xdrs,
                          &mut *(*objp).key_data_type.as_mut_ptr().offset(1 as
                                                                              libc::c_int
                                                                              as
                                                                              isize))
               == 0 {
            return 0 as libc::c_int
        }
    }
    /*
      * kadm5_get_principal on the server side allocates and returns
      * key contents when asked.  Even though this function refuses to
      * transmit that data, it still has to *free* the data at the
      * appropriate time to avoid a memory leak.
      */
    if (*xdrs).x_op as libc::c_uint == XDR_FREE as libc::c_int as libc::c_uint
       {
        tmp =
            (*objp).key_data_length[0 as libc::c_int as usize] as
                libc::c_uint;
        if gssrpc_xdr_bytes(xdrs,
                            &mut *(*objp).key_data_contents.as_mut_ptr().offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)
                                as *mut *mut krb5_octet as
                                *mut *mut libc::c_char, &mut tmp,
                            !(0 as libc::c_int) as u_int) == 0 {
            return 0 as libc::c_int
        }
        tmp =
            (*objp).key_data_length[1 as libc::c_int as usize] as
                libc::c_uint;
        if gssrpc_xdr_bytes(xdrs,
                            &mut *(*objp).key_data_contents.as_mut_ptr().offset(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)
                                as *mut *mut krb5_octet as
                                *mut *mut libc::c_char, &mut tmp,
                            !(0 as libc::c_int) as u_int) == 0 {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "292:1"]
pub unsafe extern "C" fn xdr_krb5_key_salt_tuple(mut xdrs: *mut XDR,
                                                 mut objp:
                                                     *mut krb5_key_salt_tuple)
 -> libc::c_int {
    if xdr_krb5_enctype(xdrs, &mut (*objp).ks_enctype) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_salttype(xdrs, &mut (*objp).ks_salttype) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "302:1"]
pub unsafe extern "C" fn xdr_krb5_tl_data(mut xdrs: *mut XDR,
                                          mut tl_data_head:
                                              *mut *mut krb5_tl_data)
 -> libc::c_int {
    let mut tl: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut tl2: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut more: libc::c_int = 0;
    let mut len: libc::c_uint = 0;
    match (*xdrs).x_op as libc::c_uint {
        2 => {
            tl2 = *tl_data_head;
            tl = tl2;
            while !tl.is_null() {
                tl2 = (*tl).tl_data_next;
                free((*tl).tl_data_contents as *mut libc::c_void);
                free(tl as *mut libc::c_void);
                tl = tl2
            }
            *tl_data_head = 0 as *mut krb5_tl_data
        }
        0 => {
            tl = *tl_data_head;
            loop  {
                more =
                    (tl != 0 as *mut libc::c_void as *mut krb5_tl_data) as
                        libc::c_int;
                if gssrpc_xdr_bool(xdrs, &mut more) == 0 {
                    return 0 as libc::c_int
                }
                if tl.is_null() { break ; }
                if xdr_krb5_int16(xdrs, &mut (*tl).tl_data_type) == 0 {
                    return 0 as libc::c_int
                }
                len = (*tl).tl_data_length as libc::c_uint;
                if gssrpc_xdr_bytes(xdrs,
                                    &mut (*tl).tl_data_contents as
                                        *mut *mut krb5_octet as
                                        *mut *mut libc::c_char, &mut len,
                                    !(0 as libc::c_int) as u_int) == 0 {
                    return 0 as libc::c_int
                }
                tl = (*tl).tl_data_next
            }
        }
        1 => {
            tl = 0 as *mut krb5_tl_data;
            loop  {
                if gssrpc_xdr_bool(xdrs, &mut more) == 0 {
                    return 0 as libc::c_int
                }
                if more == 0 as libc::c_int { break ; }
                tl2 =
                    malloc(::std::mem::size_of::<krb5_tl_data>() as
                               libc::c_ulong) as *mut krb5_tl_data;
                if tl2.is_null() { return 0 as libc::c_int }
                memset(tl2 as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<krb5_tl_data>() as
                           libc::c_ulong);
                if xdr_krb5_int16(xdrs, &mut (*tl2).tl_data_type) == 0 {
                    return 0 as libc::c_int
                }
                if gssrpc_xdr_bytes(xdrs,
                                    &mut (*tl2).tl_data_contents as
                                        *mut *mut krb5_octet as
                                        *mut *mut libc::c_char, &mut len,
                                    !(0 as libc::c_int) as u_int) == 0 {
                    return 0 as libc::c_int
                }
                (*tl2).tl_data_length = len as krb5_ui_2;
                (*tl2).tl_data_next = tl;
                tl = tl2
            }
            *tl_data_head = tl
        }
        _ => { }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "365:1"]
pub unsafe extern "C" fn xdr_kadm5_ret_t(mut xdrs: *mut XDR,
                                         mut objp: *mut kadm5_ret_t)
 -> libc::c_int {
    let mut tmp: uint32_t = 0;
    if (*xdrs).x_op as libc::c_uint ==
           XDR_ENCODE as libc::c_int as libc::c_uint {
        tmp = *objp as uint32_t
    }
    if gssrpc_xdr_u_int32(xdrs, &mut tmp) == 0 { return 0 as libc::c_int }
    if (*xdrs).x_op as libc::c_uint ==
           XDR_DECODE as libc::c_int as libc::c_uint {
        *objp = tmp as kadm5_ret_t
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "382:1"]
pub unsafe extern "C" fn xdr_kadm5_principal_ent_rec(mut xdrs: *mut XDR,
                                                     mut objp:
                                                         *mut kadm5_principal_ent_rec)
 -> libc::c_int {
    return _xdr_kadm5_principal_ent_rec(xdrs, objp,
                                        0x12345700 as libc::c_int |
                                            0x3 as libc::c_int);
}
/* -*- mode: c; c-file-style: "bsd"; indent-tabs-mode: t -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 */
#[c2rust::src_loc = "388:1"]
unsafe extern "C" fn _xdr_kadm5_principal_ent_rec(mut xdrs: *mut XDR,
                                                  mut objp:
                                                      *mut kadm5_principal_ent_rec,
                                                  mut v: libc::c_int)
 -> libc::c_int {
    let mut n: libc::c_uint = 0;
    if xdr_krb5_principal(xdrs, &mut (*objp).principal) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_timestamp(xdrs, &mut (*objp).princ_expire_time) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_timestamp(xdrs, &mut (*objp).last_pwd_change) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_timestamp(xdrs, &mut (*objp).pw_expiration) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_deltat(xdrs, &mut (*objp).max_life) == 0 {
        return 0 as libc::c_int
    }
    if xdr_nulltype(xdrs,
                    &mut (*objp).mod_name as *mut krb5_principal as
                        *mut *mut libc::c_void,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut XDR,
                                                                        _:
                                                                            *mut krb5_principal)
                                                       -> libc::c_int>,
                                            xdrproc_t>(Some(xdr_krb5_principal
                                                                as
                                                                unsafe extern "C" fn(_:
                                                                                         *mut XDR,
                                                                                     _:
                                                                                         *mut krb5_principal)
                                                                    ->
                                                                        libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_timestamp(xdrs, &mut (*objp).mod_date) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_flags(xdrs, &mut (*objp).attributes) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_kvno(xdrs, &mut (*objp).kvno) == 0 { return 0 as libc::c_int }
    if xdr_krb5_kvno(xdrs, &mut (*objp).mkvno) == 0 {
        return 0 as libc::c_int
    }
    if xdr_nullstring(xdrs, &mut (*objp).policy) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).aux_attributes) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_deltat(xdrs, &mut (*objp).max_renewable_life) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_timestamp(xdrs, &mut (*objp).last_success) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_timestamp(xdrs, &mut (*objp).last_failed) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_kvno(xdrs, &mut (*objp).fail_auth_count) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_int16(xdrs, &mut (*objp).n_key_data) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_int16(xdrs, &mut (*objp).n_tl_data) == 0 {
        return 0 as libc::c_int
    }
    if xdr_nulltype(xdrs,
                    &mut (*objp).tl_data as *mut *mut krb5_tl_data as
                        *mut *mut libc::c_void,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut XDR,
                                                                        _:
                                                                            *mut *mut krb5_tl_data)
                                                       -> libc::c_int>,
                                            xdrproc_t>(Some(xdr_krb5_tl_data
                                                                as
                                                                unsafe extern "C" fn(_:
                                                                                         *mut XDR,
                                                                                     _:
                                                                                         *mut *mut krb5_tl_data)
                                                                    ->
                                                                        libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    n = (*objp).n_key_data as libc::c_uint;
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).key_data as *mut *mut krb5_key_data as
                            *mut caddr_t, &mut n,
                        !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<krb5_key_data>() as
                            libc::c_ulong as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut krb5_key_data)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_krb5_key_data_nocontents
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut krb5_key_data)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "463:1"]
unsafe extern "C" fn _xdr_kadm5_policy_ent_rec(mut xdrs: *mut XDR,
                                               mut objp:
                                                   *mut kadm5_policy_ent_rec,
                                               mut vers: libc::c_int)
 -> libc::c_int {
    if xdr_nullstring(xdrs, &mut (*objp).policy) == 0 {
        return 0 as libc::c_int
    }
    /* these all used to be u_int32, but it's stupid for sized types
	   to be exposed at the api, and they're the same as longs on the
	   wire. */
    if gssrpc_xdr_long(xdrs, &mut (*objp).pw_min_life) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).pw_max_life) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).pw_min_length) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).pw_min_classes) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).pw_history_num) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).policy_refcnt) == 0 {
        return 0 as libc::c_int
    }
    if (*xdrs).x_op as libc::c_uint ==
           XDR_DECODE as libc::c_int as libc::c_uint {
        (*objp).pw_max_fail = 0 as libc::c_int as krb5_kvno;
        (*objp).pw_failcnt_interval = 0 as libc::c_int;
        (*objp).pw_lockout_duration = 0 as libc::c_int;
        (*objp).attributes = 0 as libc::c_int;
        (*objp).max_life = 0 as libc::c_int;
        (*objp).max_renewable_life = 0 as libc::c_int;
        (*objp).allowed_keysalts = 0 as *mut libc::c_char;
        (*objp).n_tl_data = 0 as libc::c_int as krb5_int16;
        (*objp).tl_data = 0 as *mut krb5_tl_data
    }
    if vers >= 0x12345700 as libc::c_int | 0x3 as libc::c_int {
        if xdr_krb5_kvno(xdrs, &mut (*objp).pw_max_fail) == 0 {
            return 0 as libc::c_int
        }
        if xdr_krb5_deltat(xdrs, &mut (*objp).pw_failcnt_interval) == 0 {
            return 0 as libc::c_int
        }
        if xdr_krb5_deltat(xdrs, &mut (*objp).pw_lockout_duration) == 0 {
            return 0 as libc::c_int
        }
    }
    if vers >= 0x12345700 as libc::c_int | 0x4 as libc::c_int {
        if xdr_krb5_flags(xdrs, &mut (*objp).attributes) == 0 {
            return 0 as libc::c_int
        }
        if xdr_krb5_deltat(xdrs, &mut (*objp).max_life) == 0 {
            return 0 as libc::c_int
        }
        if xdr_krb5_deltat(xdrs, &mut (*objp).max_renewable_life) == 0 {
            return 0 as libc::c_int
        }
        if xdr_nullstring(xdrs, &mut (*objp).allowed_keysalts) == 0 {
            return 0 as libc::c_int
        }
        if xdr_krb5_int16(xdrs, &mut (*objp).n_tl_data) == 0 {
            return 0 as libc::c_int
        }
        if xdr_nulltype(xdrs,
                        &mut (*objp).tl_data as *mut *mut krb5_tl_data as
                            *mut *mut libc::c_void,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut *mut krb5_tl_data)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_krb5_tl_data
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut *mut krb5_tl_data)
                                                                        ->
                                                                            libc::c_int)))
               == 0 {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "533:1"]
pub unsafe extern "C" fn xdr_kadm5_policy_ent_rec(mut xdrs: *mut XDR,
                                                  mut objp:
                                                      *mut kadm5_policy_ent_rec)
 -> libc::c_int {
    return _xdr_kadm5_policy_ent_rec(xdrs, objp,
                                     0x12345700 as libc::c_int |
                                         0x4 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "539:1"]
pub unsafe extern "C" fn xdr_cprinc_arg(mut xdrs: *mut XDR,
                                        mut objp: *mut cprinc_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if _xdr_kadm5_principal_ent_rec(xdrs, &mut (*objp).rec,
                                    (*objp).api_version as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).mask) == 0 {
        return 0 as libc::c_int
    }
    if xdr_nullstring(xdrs, &mut (*objp).passwd) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "558:1"]
pub unsafe extern "C" fn xdr_cprinc3_arg(mut xdrs: *mut XDR,
                                         mut objp: *mut cprinc3_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if _xdr_kadm5_principal_ent_rec(xdrs, &mut (*objp).rec,
                                    (*objp).api_version as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).mask) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).ks_tuple as *mut *mut krb5_key_salt_tuple
                            as *mut caddr_t,
                        &mut (*objp).n_ks_tuple as *mut libc::c_int as
                            *mut libc::c_uint, !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<krb5_key_salt_tuple>() as
                            libc::c_ulong as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut krb5_key_salt_tuple)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_krb5_key_salt_tuple
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut krb5_key_salt_tuple)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    if xdr_nullstring(xdrs, &mut (*objp).passwd) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "583:1"]
pub unsafe extern "C" fn xdr_generic_ret(mut xdrs: *mut XDR,
                                         mut objp: *mut generic_ret)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kadm5_ret_t(xdrs, &mut (*objp).code) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "596:1"]
pub unsafe extern "C" fn xdr_dprinc_arg(mut xdrs: *mut XDR,
                                        mut objp: *mut dprinc_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "608:1"]
pub unsafe extern "C" fn xdr_mprinc_arg(mut xdrs: *mut XDR,
                                        mut objp: *mut mprinc_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if _xdr_kadm5_principal_ent_rec(xdrs, &mut (*objp).rec,
                                    (*objp).api_version as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).mask) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "624:1"]
pub unsafe extern "C" fn xdr_rprinc_arg(mut xdrs: *mut XDR,
                                        mut objp: *mut rprinc_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).src) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).dest) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "639:1"]
pub unsafe extern "C" fn xdr_gprincs_arg(mut xdrs: *mut XDR,
                                         mut objp: *mut gprincs_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_nullstring(xdrs, &mut (*objp).exp) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "651:1"]
pub unsafe extern "C" fn xdr_gprincs_ret(mut xdrs: *mut XDR,
                                         mut objp: *mut gprincs_ret)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kadm5_ret_t(xdrs, &mut (*objp).code) == 0 {
        return 0 as libc::c_int
    }
    if (*objp).code == 0 as libc::c_int as libc::c_long {
        if gssrpc_xdr_int(xdrs, &mut (*objp).count) == 0 {
            return 0 as libc::c_int
        }
        if gssrpc_xdr_array(xdrs,
                            &mut (*objp).princs as *mut *mut *mut libc::c_char
                                as *mut caddr_t,
                            &mut (*objp).count as *mut libc::c_int as
                                *mut libc::c_uint,
                            !(0 as libc::c_int) as u_int,
                            ::std::mem::size_of::<*mut libc::c_char>() as
                                libc::c_ulong as u_int,
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    *mut XDR,
                                                                                _:
                                                                                    *mut *mut libc::c_char)
                                                               ->
                                                                   libc::c_int>,
                                                    xdrproc_t>(Some(xdr_nullstring
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut XDR,
                                                                                             _:
                                                                                                 *mut *mut libc::c_char)
                                                                            ->
                                                                                libc::c_int)))
               == 0 {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "674:1"]
pub unsafe extern "C" fn xdr_chpass_arg(mut xdrs: *mut XDR,
                                        mut objp: *mut chpass_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    if xdr_nullstring(xdrs, &mut (*objp).pass) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "689:1"]
pub unsafe extern "C" fn xdr_chpass3_arg(mut xdrs: *mut XDR,
                                         mut objp: *mut chpass3_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_boolean(xdrs, &mut (*objp).keepold) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).ks_tuple as *mut *mut krb5_key_salt_tuple
                            as *mut caddr_t,
                        &mut (*objp).n_ks_tuple as *mut libc::c_int as
                            *mut libc::c_uint, !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<krb5_key_salt_tuple>() as
                            libc::c_ulong as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut krb5_key_salt_tuple)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_krb5_key_salt_tuple
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut krb5_key_salt_tuple)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    if xdr_nullstring(xdrs, &mut (*objp).pass) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "713:1"]
pub unsafe extern "C" fn xdr_setkey_arg(mut xdrs: *mut XDR,
                                        mut objp: *mut setkey_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).keyblocks as *mut *mut krb5_keyblock as
                            *mut caddr_t,
                        &mut (*objp).n_keys as *mut libc::c_int as
                            *mut libc::c_uint, !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<krb5_keyblock>() as
                            libc::c_ulong as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut krb5_keyblock)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_krb5_keyblock
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut krb5_keyblock)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "730:1"]
pub unsafe extern "C" fn xdr_setkey3_arg(mut xdrs: *mut XDR,
                                         mut objp: *mut setkey3_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_boolean(xdrs, &mut (*objp).keepold) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).ks_tuple as *mut *mut krb5_key_salt_tuple
                            as *mut caddr_t,
                        &mut (*objp).n_ks_tuple as *mut libc::c_int as
                            *mut libc::c_uint, !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<krb5_key_salt_tuple>() as
                            libc::c_ulong as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut krb5_key_salt_tuple)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_krb5_key_salt_tuple
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut krb5_key_salt_tuple)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).keyblocks as *mut *mut krb5_keyblock as
                            *mut caddr_t,
                        &mut (*objp).n_keys as *mut libc::c_int as
                            *mut libc::c_uint, !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<krb5_keyblock>() as
                            libc::c_ulong as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut krb5_keyblock)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_krb5_keyblock
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut krb5_keyblock)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "755:1"]
pub unsafe extern "C" fn xdr_setkey4_arg(mut xdrs: *mut XDR,
                                         mut objp: *mut setkey4_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_boolean(xdrs, &mut (*objp).keepold) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).key_data as *mut *mut kadm5_key_data as
                            *mut caddr_t,
                        &mut (*objp).n_key_data as *mut libc::c_int as
                            *mut libc::c_uint, !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<kadm5_key_data>() as
                            libc::c_ulong as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut kadm5_key_data)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_kadm5_key_data
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut kadm5_key_data)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "775:1"]
pub unsafe extern "C" fn xdr_chrand_arg(mut xdrs: *mut XDR,
                                        mut objp: *mut chrand_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "787:1"]
pub unsafe extern "C" fn xdr_chrand3_arg(mut xdrs: *mut XDR,
                                         mut objp: *mut chrand3_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_boolean(xdrs, &mut (*objp).keepold) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_array(xdrs,
                        &mut (*objp).ks_tuple as *mut *mut krb5_key_salt_tuple
                            as *mut caddr_t,
                        &mut (*objp).n_ks_tuple as *mut libc::c_int as
                            *mut libc::c_uint, !(0 as libc::c_int) as u_int,
                        ::std::mem::size_of::<krb5_key_salt_tuple>() as
                            libc::c_ulong as u_int,
                        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                *mut XDR,
                                                                            _:
                                                                                *mut krb5_key_salt_tuple)
                                                           -> libc::c_int>,
                                                xdrproc_t>(Some(xdr_krb5_key_salt_tuple
                                                                    as
                                                                    unsafe extern "C" fn(_:
                                                                                             *mut XDR,
                                                                                         _:
                                                                                             *mut krb5_key_salt_tuple)
                                                                        ->
                                                                            libc::c_int)))
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "808:1"]
pub unsafe extern "C" fn xdr_chrand_ret(mut xdrs: *mut XDR,
                                        mut objp: *mut chrand_ret)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kadm5_ret_t(xdrs, &mut (*objp).code) == 0 {
        return 0 as libc::c_int
    }
    if (*objp).code == 0 as libc::c_int as libc::c_long {
        if gssrpc_xdr_array(xdrs,
                            &mut (*objp).keys as *mut *mut krb5_keyblock as
                                *mut *mut libc::c_char,
                            &mut (*objp).n_keys as *mut libc::c_int as
                                *mut libc::c_uint,
                            !(0 as libc::c_int) as u_int,
                            ::std::mem::size_of::<krb5_keyblock>() as
                                libc::c_ulong as u_int,
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    *mut XDR,
                                                                                _:
                                                                                    *mut krb5_keyblock)
                                                               ->
                                                                   libc::c_int>,
                                                    xdrproc_t>(Some(xdr_krb5_keyblock
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut XDR,
                                                                                             _:
                                                                                                 *mut krb5_keyblock)
                                                                            ->
                                                                                libc::c_int)))
               == 0 {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "827:1"]
pub unsafe extern "C" fn xdr_gprinc_arg(mut xdrs: *mut XDR,
                                        mut objp: *mut gprinc_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).mask) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "843:1"]
pub unsafe extern "C" fn xdr_gprinc_ret(mut xdrs: *mut XDR,
                                        mut objp: *mut gprinc_ret)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kadm5_ret_t(xdrs, &mut (*objp).code) == 0 {
        return 0 as libc::c_int
    }
    if (*objp).code == 0 as libc::c_int as libc::c_long {
        if _xdr_kadm5_principal_ent_rec(xdrs, &mut (*objp).rec,
                                        (*objp).api_version as libc::c_int) ==
               0 {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "862:1"]
pub unsafe extern "C" fn xdr_cpol_arg(mut xdrs: *mut XDR,
                                      mut objp: *mut cpol_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if _xdr_kadm5_policy_ent_rec(xdrs, &mut (*objp).rec,
                                 (*objp).api_version as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).mask) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "878:1"]
pub unsafe extern "C" fn xdr_dpol_arg(mut xdrs: *mut XDR,
                                      mut objp: *mut dpol_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_nullstring(xdrs, &mut (*objp).name) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "890:1"]
pub unsafe extern "C" fn xdr_mpol_arg(mut xdrs: *mut XDR,
                                      mut objp: *mut mpol_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if _xdr_kadm5_policy_ent_rec(xdrs, &mut (*objp).rec,
                                 (*objp).api_version as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_long(xdrs, &mut (*objp).mask) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "906:1"]
pub unsafe extern "C" fn xdr_gpol_arg(mut xdrs: *mut XDR,
                                      mut objp: *mut gpol_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_nullstring(xdrs, &mut (*objp).name) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "918:1"]
pub unsafe extern "C" fn xdr_gpol_ret(mut xdrs: *mut XDR,
                                      mut objp: *mut gpol_ret)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kadm5_ret_t(xdrs, &mut (*objp).code) == 0 {
        return 0 as libc::c_int
    }
    if (*objp).code == 0 as libc::c_int as libc::c_long {
        if _xdr_kadm5_policy_ent_rec(xdrs, &mut (*objp).rec,
                                     (*objp).api_version as libc::c_int) == 0
           {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "936:1"]
pub unsafe extern "C" fn xdr_gpols_arg(mut xdrs: *mut XDR,
                                       mut objp: *mut gpols_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_nullstring(xdrs, &mut (*objp).exp) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "948:1"]
pub unsafe extern "C" fn xdr_gpols_ret(mut xdrs: *mut XDR,
                                       mut objp: *mut gpols_ret)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kadm5_ret_t(xdrs, &mut (*objp).code) == 0 {
        return 0 as libc::c_int
    }
    if (*objp).code == 0 as libc::c_int as libc::c_long {
        if gssrpc_xdr_int(xdrs, &mut (*objp).count) == 0 {
            return 0 as libc::c_int
        }
        if gssrpc_xdr_array(xdrs,
                            &mut (*objp).pols as *mut *mut *mut libc::c_char
                                as *mut caddr_t,
                            &mut (*objp).count as *mut libc::c_int as
                                *mut libc::c_uint,
                            !(0 as libc::c_int) as u_int,
                            ::std::mem::size_of::<*mut libc::c_char>() as
                                libc::c_ulong as u_int,
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    *mut XDR,
                                                                                _:
                                                                                    *mut *mut libc::c_char)
                                                               ->
                                                                   libc::c_int>,
                                                    xdrproc_t>(Some(xdr_nullstring
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut XDR,
                                                                                             _:
                                                                                                 *mut *mut libc::c_char)
                                                                            ->
                                                                                libc::c_int)))
               == 0 {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "971:1"]
pub unsafe extern "C" fn xdr_getprivs_ret(mut xdrs: *mut XDR,
                                          mut objp: *mut getprivs_ret)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kadm5_ret_t(xdrs, &mut (*objp).code) == 0 ||
           gssrpc_xdr_long(xdrs, &mut (*objp).privs) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "983:1"]
pub unsafe extern "C" fn xdr_purgekeys_arg(mut xdrs: *mut XDR,
                                           mut objp: *mut purgekeys_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_int(xdrs, &mut (*objp).keepkvno) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "999:1"]
pub unsafe extern "C" fn xdr_gstrings_arg(mut xdrs: *mut XDR,
                                          mut objp: *mut gstrings_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1012:1"]
pub unsafe extern "C" fn xdr_gstrings_ret(mut xdrs: *mut XDR,
                                          mut objp: *mut gstrings_ret)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kadm5_ret_t(xdrs, &mut (*objp).code) == 0 {
        return 0 as libc::c_int
    }
    if (*objp).code == 0 as libc::c_int as libc::c_long {
        if gssrpc_xdr_int(xdrs, &mut (*objp).count) == 0 {
            return 0 as libc::c_int
        }
        if gssrpc_xdr_array(xdrs,
                            &mut (*objp).strings as *mut *mut krb5_string_attr
                                as *mut caddr_t,
                            &mut (*objp).count as *mut libc::c_int as
                                *mut libc::c_uint,
                            !(0 as libc::c_int) as u_int,
                            ::std::mem::size_of::<krb5_string_attr>() as
                                libc::c_ulong as u_int,
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    *mut XDR,
                                                                                _:
                                                                                    *mut krb5_string_attr)
                                                               ->
                                                                   libc::c_int>,
                                                    xdrproc_t>(Some(xdr_krb5_string_attr
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut XDR,
                                                                                             _:
                                                                                                 *mut krb5_string_attr)
                                                                            ->
                                                                                libc::c_int)))
               == 0 {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1036:1"]
pub unsafe extern "C" fn xdr_sstring_arg(mut xdrs: *mut XDR,
                                         mut objp: *mut sstring_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    if xdr_nullstring(xdrs, &mut (*objp).key) == 0 { return 0 as libc::c_int }
    if xdr_nullstring(xdrs, &mut (*objp).value) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1055:1"]
pub unsafe extern "C" fn xdr_krb5_principal(mut xdrs: *mut XDR,
                                            mut objp: *mut krb5_principal)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pr: krb5_principal = 0 as krb5_principal;
    static mut context: krb5_context =
        0 as *const _krb5_context as krb5_context;
    /* using a static context here is ugly, but should work
       ok, and the other solutions are even uglier */
    if context.is_null() && kadm5_init_krb5_context(&mut context) != 0 {
        return 0 as libc::c_int
    }
    match (*xdrs).x_op as libc::c_uint {
        0 => {
            if !(*objp).is_null() {
                ret =
                    krb5_unparse_name(context, *objp as krb5_const_principal,
                                      &mut p);
                if ret != 0 as libc::c_int { return 0 as libc::c_int }
            }
            if xdr_nullstring(xdrs, &mut p) == 0 { return 0 as libc::c_int }
            if !p.is_null() { free(p as *mut libc::c_void); }
        }
        1 => {
            if xdr_nullstring(xdrs, &mut p) == 0 { return 0 as libc::c_int }
            if !p.is_null() {
                ret = krb5_parse_name(context, p, &mut pr);
                if ret != 0 as libc::c_int { return 0 as libc::c_int }
                *objp = pr;
                free(p as *mut libc::c_void);
            } else { *objp = 0 as krb5_principal }
        }
        2 => {
            if !(*objp).is_null() { krb5_free_principal(context, *objp); }
            *objp = 0 as krb5_principal
        }
        _ => { }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1101:1"]
pub unsafe extern "C" fn xdr_krb5_octet(mut xdrs: *mut XDR,
                                        mut objp: *mut krb5_octet)
 -> libc::c_int {
    if gssrpc_xdr_u_char(xdrs, objp) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1109:1"]
pub unsafe extern "C" fn xdr_krb5_enctype(mut xdrs: *mut XDR,
                                          mut objp: *mut krb5_enctype)
 -> libc::c_int {
    if gssrpc_xdr_int32(xdrs, objp as *mut int32_t) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1117:1"]
pub unsafe extern "C" fn xdr_krb5_salttype(mut xdrs: *mut XDR,
                                           mut objp: *mut krb5_int32)
 -> libc::c_int {
    if gssrpc_xdr_int32(xdrs, objp as *mut int32_t) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1125:1"]
pub unsafe extern "C" fn xdr_krb5_keyblock(mut xdrs: *mut XDR,
                                           mut objp: *mut krb5_keyblock)
 -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* XXX This only works because free_keyblock assumes ->contents
      is allocated by malloc() */
    if xdr_krb5_enctype(xdrs, &mut (*objp).enctype) == 0 {
        return 0 as libc::c_int
    }
    cp = (*objp).contents as *mut libc::c_char;
    if gssrpc_xdr_bytes(xdrs, &mut cp, &mut (*objp).length,
                        !(0 as libc::c_int) as u_int) == 0 {
        return 0 as libc::c_int
    }
    (*objp).contents = cp as *mut uint8_t;
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1141:1"]
pub unsafe extern "C" fn xdr_krb5_string_attr(mut xdrs: *mut XDR,
                                              mut objp: *mut krb5_string_attr)
 -> libc::c_int {
    if xdr_nullstring(xdrs, &mut (*objp).key) == 0 { return 0 as libc::c_int }
    if xdr_nullstring(xdrs, &mut (*objp).value) == 0 {
        return 0 as libc::c_int
    }
    if (*xdrs).x_op as libc::c_uint ==
           XDR_DECODE as libc::c_int as libc::c_uint &&
           ((*objp).key.is_null() || (*objp).value.is_null()) {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1154:1"]
pub unsafe extern "C" fn xdr_kadm5_key_data(mut xdrs: *mut XDR,
                                            mut objp: *mut kadm5_key_data)
 -> libc::c_int {
    if xdr_krb5_kvno(xdrs, &mut (*objp).kvno) == 0 { return 0 as libc::c_int }
    if xdr_krb5_keyblock(xdrs, &mut (*objp).key) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_int16(xdrs, &mut (*objp).salt.type_0) == 0 {
        return 0 as libc::c_int
    }
    if gssrpc_xdr_bytes(xdrs, &mut (*objp).salt.data.data,
                        &mut (*objp).salt.data.length,
                        !(0 as libc::c_int) as u_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1169:1"]
pub unsafe extern "C" fn xdr_getpkeys_arg(mut xdrs: *mut XDR,
                                          mut objp: *mut getpkeys_arg)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_principal(xdrs, &mut (*objp).princ) == 0 {
        return 0 as libc::c_int
    }
    if xdr_krb5_kvno(xdrs, &mut (*objp).kvno) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1184:1"]
pub unsafe extern "C" fn xdr_getpkeys_ret(mut xdrs: *mut XDR,
                                          mut objp: *mut getpkeys_ret)
 -> libc::c_int {
    if xdr_ui_4(xdrs, &mut (*objp).api_version) == 0 {
        return 0 as libc::c_int
    }
    if xdr_kadm5_ret_t(xdrs, &mut (*objp).code) == 0 {
        return 0 as libc::c_int
    }
    if (*objp).code == 0 as libc::c_int as libc::c_long {
        if gssrpc_xdr_array(xdrs,
                            &mut (*objp).key_data as *mut *mut kadm5_key_data
                                as *mut caddr_t,
                            &mut (*objp).n_key_data as *mut libc::c_int as
                                *mut libc::c_uint,
                            !(0 as libc::c_int) as u_int,
                            ::std::mem::size_of::<kadm5_key_data>() as
                                libc::c_ulong as u_int,
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    *mut XDR,
                                                                                _:
                                                                                    *mut kadm5_key_data)
                                                               ->
                                                                   libc::c_int>,
                                                    xdrproc_t>(Some(xdr_kadm5_key_data
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut XDR,
                                                                                             _:
                                                                                                 *mut kadm5_key_data)
                                                                            ->
                                                                                libc::c_int)))
               == 0 {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
