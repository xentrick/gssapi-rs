use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:32"]
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
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
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
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    extern "C" {
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[no_mangle]
        #[c2rust::src_loc = "309:1"]
        pub fn krb5_anonymous_principal() -> krb5_const_principal;
        /* *
 * Compare two principals ignoring realm components.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * Similar to krb5_principal_compare(), but do not compare the realm
 * components of the principals.
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
        #[no_mangle]
        #[c2rust::src_loc = "3682:1"]
        pub fn krb5_principal_compare_any_realm(context: krb5_context,
                                                princ1: krb5_const_principal,
                                                princ2: krb5_const_principal)
         -> krb5_boolean;
        /* Error reporting */
/* *
 * Set an extended error message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] ...              printf(3) style parameters
 */
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/clpreauth_plugin.h:32"]
pub mod clpreauth_plugin_h {
    #[c2rust::src_loc = "75:1"]
    pub type krb5_clpreauth_rock = *mut krb5_clpreauth_rock_st;
    /* Before using a callback after version 1, modules must check the vers
 * field of the callback structure. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:16"]
    pub struct krb5_clpreauth_callbacks_st {
        pub vers: libc::c_int,
        pub get_etype: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_clpreauth_rock)
                                  -> krb5_enctype>,
        pub fast_armor: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_clpreauth_rock)
                                   -> *mut krb5_keyblock>,
        pub get_as_key: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_clpreauth_rock,
                                                    _:
                                                        *mut *mut krb5_keyblock)
                                   -> krb5_error_code>,
        pub set_as_key: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_clpreauth_rock,
                                                    _: *const krb5_keyblock)
                                   -> krb5_error_code>,
        pub get_preauth_time: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_clpreauth_rock,
                                                          _: krb5_boolean,
                                                          _:
                                                              *mut krb5_timestamp,
                                                          _: *mut krb5_int32)
                                         -> krb5_error_code>,
        pub ask_responder_question: Option<unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    krb5_clpreauth_rock,
                                                                _:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const libc::c_char)
                                               -> krb5_error_code>,
        pub get_responder_answer: Option<unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  krb5_clpreauth_rock,
                                                              _:
                                                                  *const libc::c_char)
                                             -> *const libc::c_char>,
        pub need_as_key: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: krb5_clpreauth_rock)
                                    -> ()>,
        pub get_cc_config: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: krb5_clpreauth_rock,
                                                       _: *const libc::c_char)
                                      -> *const libc::c_char>,
        pub set_cc_config: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: krb5_clpreauth_rock,
                                                       _: *const libc::c_char,
                                                       _: *const libc::c_char)
                                      -> krb5_error_code>,
        pub disable_fallback: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_clpreauth_rock)
                                         -> ()>,
    }
    #[c2rust::src_loc = "83:1"]
    pub type krb5_clpreauth_callbacks = *mut krb5_clpreauth_callbacks_st;
    use super::krb5_h::{krb5_enctype, krb5_context, krb5_keyblock,
                        krb5_error_code, krb5_boolean, krb5_timestamp,
                        krb5_int32};
    extern "C" {
        /* End of version 3 clpreauth callbacks (added in 1.17). */
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (c) 2006 Red Hat, Inc.
 * Portions copyright (c) 2006, 2011 Massachusetts Institute of Technology
 * All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *  * Neither the name of Red Hat, Inc., nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
 * OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
        /*
 * Declarations for clpreauth plugin module implementors.
 *
 * The clpreauth interface has a single supported major version, which is
 * 1.  Major version 1 has a current minor version of 2.  clpreauth modules
 * should define a function named clpreauth_<modulename>_initvt, matching
 * the signature:
 *
 *   krb5_error_code
 *   clpreauth_modname_initvt(krb5_context context, int maj_ver,
 *                            int min_ver, krb5_plugin_vtable vtable);
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for the interface and maj_ver:
 *     maj_ver == 1: Cast to krb5_clpreauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
        /* clpreauth mechanism property flags */
        /* Provides a real answer which we can send back to the KDC.  The client
 * assumes that one real answer will be enough. */
        /* Doesn't provide a real answer, but must be given a chance to run before any
 * REAL mechanism callbacks. */
        /* Abstract type for a client request information handle. */
        #[c2rust::src_loc = "75:16"]
        pub type krb5_clpreauth_rock_st;
    }
    /* KRB5_CLPREAUTH_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkcs11.h:32"]
pub mod pkcs11_h {
    #[c2rust::src_loc = "208:1"]
    pub type CK_SLOT_ID = libc::c_ulong;
    /* PKCS11_H */
    /* System dependencies.  */
    /* CRYPTOKI_COMPAT */
    /* Delete the helper macros defined at the top of the file.  */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit.h:32"]
pub mod pkinit_h {
    #[c2rust::src_loc = "128:1"]
    pub type pkinit_plg_crypto_context = *mut _pkinit_plg_crypto_context;
    #[c2rust::src_loc = "134:1"]
    pub type pkinit_req_crypto_context = *mut _pkinit_req_crypto_context;
    #[c2rust::src_loc = "141:1"]
    pub type pkinit_identity_crypto_context
        =
        *mut _pkinit_identity_crypto_context;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "175:16"]
    pub struct _pkinit_identity_opts {
        pub identity: *mut libc::c_char,
        pub identity_alt: *mut *mut libc::c_char,
        pub anchors: *mut *mut libc::c_char,
        pub intermediates: *mut *mut libc::c_char,
        pub crls: *mut *mut libc::c_char,
        pub idtype: libc::c_int,
        pub cert_filename: *mut libc::c_char,
        pub key_filename: *mut libc::c_char,
        pub p11_module_name: *mut libc::c_char,
        pub slotid: CK_SLOT_ID,
        pub token_label: *mut libc::c_char,
        pub cert_id_string: *mut libc::c_char,
        pub cert_label: *mut libc::c_char,
    }
    #[c2rust::src_loc = "175:1"]
    pub type pkinit_identity_opts = _pkinit_identity_opts;
    #[c2rust::src_loc = "315:1"]
    pub type pkinit_deferred_id = *mut _pkinit_deferred_id;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "309:8"]
    pub struct _pkinit_deferred_id {
        pub magic: libc::c_int,
        pub identity: *mut libc::c_char,
        pub ck_flags: libc::c_ulong,
        pub password: *mut libc::c_char,
    }
    #[inline]
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn pkiDebug(mut fmt: *const libc::c_char,
                                      mut args: ...) {
    }
    use super::pkcs11_h::CK_SLOT_ID;
    use super::krb5_h::{_krb5_context, krb5_context, krb5_principal_data,
                        krb5_principal, krb5_error_code};
    extern "C" {
        #[c2rust::src_loc = "128:16"]
        pub type _pkinit_plg_crypto_context;
        #[c2rust::src_loc = "134:16"]
        pub type _pkinit_req_crypto_context;
        #[c2rust::src_loc = "141:16"]
        pub type _pkinit_identity_crypto_context;
        #[no_mangle]
        #[c2rust::src_loc = "292:1"]
        pub fn pkinit_cert_matching(context: krb5_context,
                                    plg_cryptoctx: pkinit_plg_crypto_context,
                                    req_cryptoctx: pkinit_req_crypto_context,
                                    id_cryptoctx:
                                        pkinit_identity_crypto_context,
                                    princ: krb5_principal) -> krb5_error_code;
    }
    /* _PKINIT_H */
    /*
 * Now get crypto function declarations
 */
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "345:1"]
        pub fn strtok_r(__s: *mut libc::c_char, __delim: *const libc::c_char,
                        __save_ptr: *mut *mut libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "176:17"]
        pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
                      _: libc::c_int) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn secure_getenv(__name: *const libc::c_char)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:32"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:32"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit_crypto.h:32"]
pub mod pkinit_crypto_h {
    use super::krb5_h::{_krb5_context, krb5_context, krb5_error_code,
                        krb5_principal_data, krb5_principal, krb5_boolean};
    use super::pkinit_h::{_pkinit_plg_crypto_context,
                          pkinit_plg_crypto_context,
                          _pkinit_req_crypto_context,
                          pkinit_req_crypto_context,
                          _pkinit_identity_crypto_context,
                          pkinit_identity_crypto_context,
                          pkinit_identity_opts};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "448:1"]
        pub fn crypto_free_cert_info(context: krb5_context,
                                     plg_cryptoctx: pkinit_plg_crypto_context,
                                     req_cryptoctx: pkinit_req_crypto_context,
                                     id_cryptoctx:
                                         pkinit_identity_crypto_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "436:1"]
        pub fn crypto_load_certs(context: krb5_context,
                                 plg_cryptoctx: pkinit_plg_crypto_context,
                                 req_cryptoctx: pkinit_req_crypto_context,
                                 idopts: *mut pkinit_identity_opts,
                                 id_cryptoctx: pkinit_identity_crypto_context,
                                 princ: krb5_principal,
                                 defer_id_prompts: krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "503:1"]
        pub fn crypto_load_cas_and_crls(context: krb5_context,
                                        plg_cryptoctx:
                                            pkinit_plg_crypto_context,
                                        req_cryptoctx:
                                            pkinit_req_crypto_context,
                                        idopts: *mut pkinit_identity_opts,
                                        id_cryptoctx:
                                            pkinit_identity_crypto_context,
                                        idtype: libc::c_int,
                                        catype: libc::c_int,
                                        id: *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "233:1"]
        pub fn crypto_retrieve_signer_identity(context: krb5_context,
                                               id_cryptoctx:
                                                   pkinit_identity_crypto_context,
                                               identity:
                                                   *mut *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "492:1"]
        pub fn crypto_cert_select_default(context: krb5_context,
                                          plg_cryptoctx:
                                              pkinit_plg_crypto_context,
                                          req_cryptoctx:
                                              pkinit_req_crypto_context,
                                          id_cryptoctx:
                                              pkinit_identity_crypto_context)
         -> krb5_error_code;
    }
    /* _PKINIT_CRYPTO_H */
}
pub use self::types_h::{__uint8_t, __int32_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_enctype,
                       krb5_timestamp, krb5_error_code, krb5_magic,
                       _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, krb5_const_principal, krb5_context,
                       _krb5_keyblock, krb5_keyblock, _krb5_context,
                       krb5_anonymous_principal,
                       krb5_principal_compare_any_realm,
                       krb5_set_error_message};
pub use self::clpreauth_plugin_h::{krb5_clpreauth_rock,
                                   krb5_clpreauth_callbacks_st,
                                   krb5_clpreauth_callbacks,
                                   krb5_clpreauth_rock_st};
pub use self::pkcs11_h::CK_SLOT_ID;
pub use self::pkinit_h::{pkinit_plg_crypto_context, pkinit_req_crypto_context,
                         pkinit_identity_crypto_context,
                         _pkinit_identity_opts, pkinit_identity_opts,
                         pkinit_deferred_id, _pkinit_deferred_id, pkiDebug,
                         _pkinit_plg_crypto_context,
                         _pkinit_req_crypto_context,
                         _pkinit_identity_crypto_context,
                         pkinit_cert_matching};
use self::string_h::{strdup, strchr, strtok_r, strncmp, strcmp};
use self::stdlib_h::{strtol, malloc, calloc, realloc, free, secure_getenv};
use self::errno_h::__errno_location;
use self::libintl_h::dgettext;
use self::pkinit_crypto_h::{crypto_free_cert_info, crypto_load_certs,
                            crypto_load_cas_and_crls,
                            crypto_retrieve_signer_identity,
                            crypto_cert_select_default};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * COPYRIGHT (C) 2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn free_list(mut list: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    if list.is_null() { return }
    i = 0 as libc::c_int;
    while !(*list.offset(i as isize)).is_null() {
        free(*list.offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    free(list as *mut libc::c_void);
}
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn copy_list(mut dst: *mut *mut *mut libc::c_char,
                               mut src: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut newlist: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if dst.is_null() { return 22 as libc::c_int }
    *dst = 0 as *mut *mut libc::c_char;
    if src.is_null() { return 0 as libc::c_int }
    i = 0 as libc::c_int;
    while !(*src.offset(i as isize)).is_null() { i += 1 }
    newlist =
        calloc(1 as libc::c_int as libc::c_ulong,
               ((i + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    if newlist.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int;
    loop  {
        if (*src.offset(i as isize)).is_null() {
            current_block = 7976072742316086414;
            break ;
        }
        let ref mut fresh0 = *newlist.offset(i as isize);
        *fresh0 = strdup(*src.offset(i as isize));
        if (*newlist.offset(i as isize)).is_null() {
            current_block = 15308638568240087505;
            break ;
        }
        i += 1
    }
    match current_block {
        15308638568240087505 => {
            free_list(newlist);
            return 12 as libc::c_int
        }
        _ => {
            let ref mut fresh1 = *newlist.offset(i as isize);
            *fresh1 = 0 as *mut libc::c_char;
            *dst = newlist;
            return 0 as libc::c_int
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn idtype2string(mut idtype: libc::c_int)
 -> *mut libc::c_char {
    match idtype {
        1 => {
            return b"FILE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        2 => {
            return b"DIR\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        3 => {
            return b"PKCS11\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        5 => {
            return b"PKCS12\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        4 => {
            return b"ENV\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        _ => {
            return b"INVALID\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "94:1"]
pub unsafe extern "C" fn catype2string(mut catype: libc::c_int)
 -> *mut libc::c_char {
    match catype {
        1 => {
            return b"ANCHORS\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        2 => {
            return b"INTERMEDIATES\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        3 => {
            return b"CRLS\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        _ => {
            return b"INVALID\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "105:1"]
pub unsafe extern "C" fn pkinit_init_identity_opts(mut idopts:
                                                       *mut *mut pkinit_identity_opts)
 -> krb5_error_code {
    let mut opts: *mut pkinit_identity_opts = 0 as *mut pkinit_identity_opts;
    *idopts = 0 as *mut pkinit_identity_opts;
    opts =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<pkinit_identity_opts>() as libc::c_ulong)
            as *mut pkinit_identity_opts;
    if opts.is_null() { return 12 as libc::c_int }
    (*opts).identity = 0 as *mut libc::c_char;
    (*opts).anchors = 0 as *mut *mut libc::c_char;
    (*opts).intermediates = 0 as *mut *mut libc::c_char;
    (*opts).crls = 0 as *mut *mut libc::c_char;
    (*opts).cert_filename = 0 as *mut libc::c_char;
    (*opts).key_filename = 0 as *mut libc::c_char;
    (*opts).p11_module_name = 0 as *mut libc::c_char;
    (*opts).slotid = 999999 as libc::c_int as CK_SLOT_ID;
    (*opts).token_label = 0 as *mut libc::c_char;
    (*opts).cert_id_string = 0 as *mut libc::c_char;
    (*opts).cert_label = 0 as *mut libc::c_char;
    *idopts = opts;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "135:1"]
pub unsafe extern "C" fn pkinit_dup_identity_opts(mut src_opts:
                                                      *mut pkinit_identity_opts,
                                                  mut dest_opts:
                                                      *mut *mut pkinit_identity_opts)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut newopts: *mut pkinit_identity_opts =
        0 as *mut pkinit_identity_opts;
    let mut retval: krb5_error_code = 0;
    *dest_opts = 0 as *mut pkinit_identity_opts;
    retval = pkinit_init_identity_opts(&mut newopts);
    if retval != 0 { return retval }
    retval = 12 as libc::c_int;
    if !(*src_opts).identity.is_null() {
        (*newopts).identity = strdup((*src_opts).identity);
        if (*newopts).identity.is_null() {
            current_block = 11470328316629907497;
        } else { current_block = 8515828400728868193; }
    } else { current_block = 8515828400728868193; }
    match current_block {
        8515828400728868193 => {
            retval = copy_list(&mut (*newopts).anchors, (*src_opts).anchors);
            if !(retval != 0) {
                retval =
                    copy_list(&mut (*newopts).intermediates,
                              (*src_opts).intermediates);
                if !(retval != 0) {
                    retval =
                        copy_list(&mut (*newopts).crls, (*src_opts).crls);
                    if !(retval != 0) {
                        if !(*src_opts).cert_filename.is_null() {
                            (*newopts).cert_filename =
                                strdup((*src_opts).cert_filename);
                            if (*newopts).cert_filename.is_null() {
                                current_block = 11470328316629907497;
                            } else { current_block = 5689001924483802034; }
                        } else { current_block = 5689001924483802034; }
                        match current_block {
                            11470328316629907497 => { }
                            _ => {
                                if !(*src_opts).key_filename.is_null() {
                                    (*newopts).key_filename =
                                        strdup((*src_opts).key_filename);
                                    if (*newopts).key_filename.is_null() {
                                        current_block = 11470328316629907497;
                                    } else {
                                        current_block = 5783071609795492627;
                                    }
                                } else {
                                    current_block = 5783071609795492627;
                                }
                                match current_block {
                                    11470328316629907497 => { }
                                    _ => {
                                        if !(*src_opts).p11_module_name.is_null()
                                           {
                                            (*newopts).p11_module_name =
                                                strdup((*src_opts).p11_module_name);
                                            if (*newopts).p11_module_name.is_null()
                                               {
                                                current_block =
                                                    11470328316629907497;
                                            } else {
                                                current_block =
                                                    16203760046146113240;
                                            }
                                        } else {
                                            current_block =
                                                16203760046146113240;
                                        }
                                        match current_block {
                                            11470328316629907497 => { }
                                            _ => {
                                                (*newopts).slotid =
                                                    (*src_opts).slotid;
                                                if !(*src_opts).token_label.is_null()
                                                   {
                                                    (*newopts).token_label =
                                                        strdup((*src_opts).token_label);
                                                    if (*newopts).token_label.is_null()
                                                       {
                                                        current_block =
                                                            11470328316629907497;
                                                    } else {
                                                        current_block =
                                                            5689316957504528238;
                                                    }
                                                } else {
                                                    current_block =
                                                        5689316957504528238;
                                                }
                                                match current_block {
                                                    11470328316629907497 => {
                                                    }
                                                    _ => {
                                                        if !(*src_opts).cert_id_string.is_null()
                                                           {
                                                            (*newopts).cert_id_string
                                                                =
                                                                strdup((*src_opts).cert_id_string);
                                                            if (*newopts).cert_id_string.is_null()
                                                               {
                                                                current_block
                                                                    =
                                                                    11470328316629907497;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    8704759739624374314;
                                                            }
                                                        } else {
                                                            current_block =
                                                                8704759739624374314;
                                                        }
                                                        match current_block {
                                                            11470328316629907497
                                                            => {
                                                            }
                                                            _ => {
                                                                if !(*src_opts).cert_label.is_null()
                                                                   {
                                                                    (*newopts).cert_label
                                                                        =
                                                                        strdup((*src_opts).cert_label);
                                                                    if (*newopts).cert_label.is_null()
                                                                       {
                                                                        current_block
                                                                            =
                                                                            11470328316629907497;
                                                                    } else {
                                                                        current_block
                                                                            =
                                                                            8180496224585318153;
                                                                    }
                                                                } else {
                                                                    current_block
                                                                        =
                                                                        8180496224585318153;
                                                                }
                                                                match current_block
                                                                    {
                                                                    11470328316629907497
                                                                    => {
                                                                    }
                                                                    _ => {
                                                                        *dest_opts
                                                                            =
                                                                            newopts;
                                                                        return 0
                                                                                   as
                                                                                   libc::c_int
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    pkinit_fini_identity_opts(newopts);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "215:1"]
pub unsafe extern "C" fn pkinit_fini_identity_opts(mut idopts:
                                                       *mut pkinit_identity_opts) {
    if idopts.is_null() { return }
    if !(*idopts).identity.is_null() {
        free((*idopts).identity as *mut libc::c_void);
    }
    free_list((*idopts).anchors);
    free_list((*idopts).intermediates);
    free_list((*idopts).crls);
    free_list((*idopts).identity_alt);
    free((*idopts).cert_filename as *mut libc::c_void);
    free((*idopts).key_filename as *mut libc::c_void);
    free((*idopts).p11_module_name as *mut libc::c_void);
    free((*idopts).token_label as *mut libc::c_void);
    free((*idopts).cert_id_string as *mut libc::c_void);
    free((*idopts).cert_label as *mut libc::c_void);
    free(idopts as *mut libc::c_void);
}
#[c2rust::src_loc = "240:1"]
unsafe extern "C" fn parse_pkcs11_options(mut context: krb5_context,
                                          mut idopts:
                                              *mut pkinit_identity_opts,
                                          mut residual: *const libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: krb5_error_code = 12 as libc::c_int;
    if residual.is_null() ||
           *residual.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\u{0}' as i32 {
        return 0 as libc::c_int
    }
    /* Split string into attr=value substrings */
    s = strdup(residual);
    if s.is_null() { return retval }
    cp = strtok_r(s, b":\x00" as *const u8 as *const libc::c_char, &mut save);
    loop  {
        if cp.is_null() { current_block = 11743904203796629665; break ; }
        vp = strchr(cp, '=' as i32);
        /* If there is no "=", this is a pkcs11 module name */
        if vp.is_null() {
            free((*idopts).p11_module_name as
                     *mut libc::c_void); /* skip past colon */
            (*idopts).p11_module_name = strdup(cp); /* skip past colon */
            if (*idopts).p11_module_name.is_null() {
                current_block = 6621008398210872980;
                break ;
            }
        } else {
            let fresh2 = vp;
            vp = vp.offset(1);
            *fresh2 = '\u{0}' as i32 as libc::c_char;
            if strcmp(cp,
                      b"module_name\x00" as *const u8 as *const libc::c_char)
                   == 0 {
                free((*idopts).p11_module_name as *mut libc::c_void);
                (*idopts).p11_module_name = strdup(vp);
                if (*idopts).p11_module_name.is_null() {
                    current_block = 6621008398210872980;
                    break ;
                }
            } else if strcmp(cp,
                             b"slotid\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                let mut slotid: libc::c_long =
                    strtol(vp, 0 as *mut *mut libc::c_char,
                           10 as libc::c_int);
                if (slotid ==
                        -(9223372036854775807 as libc::c_long) -
                            1 as libc::c_long ||
                        slotid == 9223372036854775807 as libc::c_long) &&
                       *__errno_location() != 0 as libc::c_int {
                    retval = 22 as libc::c_int;
                    current_block = 6621008398210872980;
                    break ;
                } else if slotid as libc::c_int as libc::c_long != slotid {
                    retval = 22 as libc::c_int;
                    current_block = 6621008398210872980;
                    break ;
                } else { (*idopts).slotid = slotid as CK_SLOT_ID }
            } else if strcmp(cp,
                             b"token\x00" as *const u8 as *const libc::c_char)
                          == 0 {
                free((*idopts).token_label as *mut libc::c_void);
                (*idopts).token_label = strdup(vp);
                if (*idopts).token_label.is_null() {
                    current_block = 6621008398210872980;
                    break ;
                }
            } else if strcmp(cp,
                             b"certid\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                free((*idopts).cert_id_string as *mut libc::c_void);
                (*idopts).cert_id_string = strdup(vp);
                if (*idopts).cert_id_string.is_null() {
                    current_block = 6621008398210872980;
                    break ;
                }
            } else if strcmp(cp,
                             b"certlabel\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                free((*idopts).cert_label as *mut libc::c_void);
                (*idopts).cert_label = strdup(vp);
                if (*idopts).cert_label.is_null() {
                    current_block = 6621008398210872980;
                    break ;
                }
            }
        }
        cp =
            strtok_r(0 as *mut libc::c_char,
                     b":\x00" as *const u8 as *const libc::c_char, &mut save)
    }
    match current_block {
        11743904203796629665 => { retval = 0 as libc::c_int }
        _ => { }
    }
    free(s as *mut libc::c_void);
    return retval;
}
#[c2rust::src_loc = "308:1"]
unsafe extern "C" fn parse_fs_options(mut context: krb5_context,
                                      mut idopts: *mut pkinit_identity_opts,
                                      mut residual: *const libc::c_char)
 -> krb5_error_code {
    let mut certname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keyname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cert_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: krb5_error_code = 12 as libc::c_int;
    if residual.is_null() ||
           *residual.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\u{0}' as i32 ||
           *residual.offset(0 as libc::c_int as isize) as libc::c_int ==
               ',' as i32 {
        return 22 as libc::c_int
    }
    certname = strdup(residual);
    if !certname.is_null() {
        certname =
            strtok_r(certname, b",\x00" as *const u8 as *const libc::c_char,
                     &mut save);
        if !certname.is_null() {
            keyname =
                strtok_r(0 as *mut libc::c_char,
                         b",\x00" as *const u8 as *const libc::c_char,
                         &mut save);
            cert_filename = strdup(certname);
            if !cert_filename.is_null() {
                key_filename =
                    strdup(if !keyname.is_null() {
                               keyname
                           } else { certname });
                if !key_filename.is_null() {
                    (*idopts).cert_filename = cert_filename;
                    (*idopts).key_filename = key_filename;
                    key_filename = 0 as *mut libc::c_char;
                    cert_filename = key_filename;
                    retval = 0 as libc::c_int
                }
            }
        }
    }
    free(certname as *mut libc::c_void);
    free(cert_filename as *mut libc::c_void);
    free(key_filename as *mut libc::c_void);
    return retval;
}
#[c2rust::src_loc = "349:1"]
unsafe extern "C" fn parse_pkcs12_options(mut context: krb5_context,
                                          mut idopts:
                                              *mut pkinit_identity_opts,
                                          mut residual: *const libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    if residual.is_null() ||
           *residual.offset(0 as libc::c_int as isize) as libc::c_int ==
               '\u{0}' as i32 {
        return 0 as libc::c_int
    }
    (*idopts).cert_filename = strdup(residual);
    if !(*idopts).cert_filename.is_null() {
        (*idopts).key_filename = strdup(residual);
        if !(*idopts).key_filename.is_null() {
            pkiDebug(b"%s: cert_filename \'%s\' key_filename \'%s\'\n\x00" as
                         *const u8 as *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 21],
                                               &[libc::c_char; 21]>(b"parse_pkcs12_options\x00")).as_ptr(),
                     (*idopts).cert_filename, (*idopts).key_filename);
            retval = 0 as libc::c_int
        }
    }
    return retval;
}
#[c2rust::src_loc = "375:1"]
unsafe extern "C" fn process_option_identity(mut context: krb5_context,
                                             mut plg_cryptoctx:
                                                 pkinit_plg_crypto_context,
                                             mut req_cryptoctx:
                                                 pkinit_req_crypto_context,
                                             mut idopts:
                                                 *mut pkinit_identity_opts,
                                             mut id_cryptoctx:
                                                 pkinit_identity_crypto_context,
                                             mut value: *const libc::c_char)
 -> krb5_error_code {
    let mut residual: *const libc::c_char = 0 as *const libc::c_char;
    let mut idtype: libc::c_int = 0;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    pkiDebug(b"%s: processing value \'%s\'\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 24],
                                       &[libc::c_char; 24]>(b"process_option_identity\x00")).as_ptr(),
             if !value.is_null() {
                 value
             } else { b"NULL\x00" as *const u8 as *const libc::c_char });
    if value.is_null() { return 22 as libc::c_int }
    residual = strchr(value, ':' as i32);
    if !residual.is_null() {
        let mut typelen: libc::c_uint = 0;
        residual = residual.offset(1);
        typelen =
            residual.wrapping_offset_from(value) as libc::c_long as
                libc::c_uint;
        if strncmp(value, b"FILE:\x00" as *const u8 as *const libc::c_char,
                   typelen as libc::c_ulong) == 0 as libc::c_int {
            idtype = 1 as libc::c_int
        } else if strncmp(value,
                          b"PKCS11:\x00" as *const u8 as *const libc::c_char,
                          typelen as libc::c_ulong) == 0 as libc::c_int {
            idtype = 3 as libc::c_int
        } else if strncmp(value,
                          b"PKCS12:\x00" as *const u8 as *const libc::c_char,
                          typelen as libc::c_ulong) == 0 as libc::c_int {
            idtype = 5 as libc::c_int
        } else if strncmp(value,
                          b"DIR:\x00" as *const u8 as *const libc::c_char,
                          typelen as libc::c_ulong) == 0 as libc::c_int {
            idtype = 2 as libc::c_int
        } else if strncmp(value,
                          b"ENV:\x00" as *const u8 as *const libc::c_char,
                          typelen as libc::c_ulong) == 0 as libc::c_int {
            idtype = 4 as libc::c_int
        } else {
            pkiDebug(b"%s: Unsupported type while processing \'%s\'\n\x00" as
                         *const u8 as *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 24],
                                               &[libc::c_char; 24]>(b"process_option_identity\x00")).as_ptr(),
                     value);
            krb5_set_error_message(context,
                                   -(1765328174 as libc::c_long) as
                                       krb5_error_code,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Unsupported type while processing \'%s\'\n\x00"
                                                as *const u8 as
                                                *const libc::c_char), value);
            return -(1765328174 as libc::c_long) as krb5_error_code
        }
    } else { idtype = 1 as libc::c_int; residual = value }
    (*idopts).idtype = idtype;
    pkiDebug(b"%s: idtype is %s\n\x00" as *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 24],
                                       &[libc::c_char; 24]>(b"process_option_identity\x00")).as_ptr(),
             idtype2string((*idopts).idtype));
    match idtype {
        4 => {
            return process_option_identity(context, plg_cryptoctx,
                                           req_cryptoctx, idopts,
                                           id_cryptoctx,
                                           secure_getenv(residual))
        }
        1 => { retval = parse_fs_options(context, idopts, residual) }
        5 => { retval = parse_pkcs12_options(context, idopts, residual) }
        3 => { retval = parse_pkcs11_options(context, idopts, residual) }
        2 => {
            (*idopts).cert_filename = strdup(residual);
            if (*idopts).cert_filename.is_null() {
                retval = 12 as libc::c_int
            }
        }
        _ => {
            krb5_set_error_message(context,
                                   -(1765328174 as libc::c_long) as
                                       krb5_error_code,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Internal error parsing X509_user_identity\n\x00"
                                                as *const u8 as
                                                *const libc::c_char));
            retval = 22 as libc::c_int
        }
    }
    return retval;
}
#[c2rust::src_loc = "456:1"]
unsafe extern "C" fn process_option_ca_crl(mut context: krb5_context,
                                           mut plg_cryptoctx:
                                               pkinit_plg_crypto_context,
                                           mut req_cryptoctx:
                                               pkinit_req_crypto_context,
                                           mut idopts:
                                               *mut pkinit_identity_opts,
                                           mut id_cryptoctx:
                                               pkinit_identity_crypto_context,
                                           mut value: *const libc::c_char,
                                           mut catype: libc::c_int)
 -> krb5_error_code {
    let mut residual: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut typelen: libc::c_uint = 0;
    let mut idtype: libc::c_int = 0;
    pkiDebug(b"%s: processing catype %s, value \'%s\'\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 22],
                                       &[libc::c_char; 22]>(b"process_option_ca_crl\x00")).as_ptr(),
             catype2string(catype), value);
    residual = strchr(value, ':' as i32);
    if residual.is_null() {
        pkiDebug(b"No type given for \'%s\'\n\x00" as *const u8 as
                     *const libc::c_char, value);
        return 22 as libc::c_int
    }
    residual = residual.offset(1);
    typelen =
        residual.wrapping_offset_from(value) as libc::c_long as libc::c_uint;
    if strncmp(value, b"FILE:\x00" as *const u8 as *const libc::c_char,
               typelen as libc::c_ulong) == 0 as libc::c_int {
        idtype = 1 as libc::c_int
    } else if strncmp(value, b"DIR:\x00" as *const u8 as *const libc::c_char,
                      typelen as libc::c_ulong) == 0 as libc::c_int {
        idtype = 2 as libc::c_int
    } else { return 95 as libc::c_int }
    return crypto_load_cas_and_crls(context, plg_cryptoctx, req_cryptoctx,
                                    idopts, id_cryptoctx, idtype, catype,
                                    residual);
}
/*
 * Load any identity information which doesn't require us to ask a controlling
 * user any questions, and record the names of anything else which would
 * require us to ask questions.
 */
#[no_mangle]
#[c2rust::src_loc = "497:1"]
pub unsafe extern "C" fn pkinit_identity_initialize(mut context: krb5_context,
                                                    mut plg_cryptoctx:
                                                        pkinit_plg_crypto_context,
                                                    mut req_cryptoctx:
                                                        pkinit_req_crypto_context,
                                                    mut idopts:
                                                        *mut pkinit_identity_opts,
                                                    mut id_cryptoctx:
                                                        pkinit_identity_crypto_context,
                                                    mut cb:
                                                        krb5_clpreauth_callbacks,
                                                    mut rock:
                                                        krb5_clpreauth_rock,
                                                    mut princ: krb5_principal)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 22 as libc::c_int;
    let mut i: libc::c_int = 0;
    pkiDebug(b"%s: %p %p %p\n\x00" as *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 27],
                                       &[libc::c_char; 27]>(b"pkinit_identity_initialize\x00")).as_ptr(),
             context, idopts, id_cryptoctx);
    if !(!princ.is_null() &&
             krb5_principal_compare_any_realm(context,
                                              princ as krb5_const_principal,
                                              krb5_anonymous_principal()) !=
                 0) {
        if !(idopts.is_null() || id_cryptoctx.is_null()) {
            /*
         * If identity was specified, use that.  (For the kdc, this
         * is specified as pkinit_identity in the kdc.conf.  For users,
         * this is specified on the command line via X509_user_identity.)
         * If a user did not specify identity on the command line,
         * then we will try alternatives which may have been specified
         * in the config file.
         */
            if !(*idopts).identity.is_null() {
                retval =
                    process_option_identity(context, plg_cryptoctx,
                                            req_cryptoctx, idopts,
                                            id_cryptoctx, (*idopts).identity);
                current_block = 13056961889198038528;
            } else if !(*idopts).identity_alt.is_null() {
                i = 0 as libc::c_int;
                while retval != 0 as libc::c_int &&
                          !(*(*idopts).identity_alt.offset(i as
                                                               isize)).is_null()
                      {
                    retval =
                        process_option_identity(context, plg_cryptoctx,
                                                req_cryptoctx, idopts,
                                                id_cryptoctx,
                                                *(*idopts).identity_alt.offset(i
                                                                                   as
                                                                                   isize));
                    i += 1
                }
                current_block = 13056961889198038528;
            } else {
                retval = -(1765328174 as libc::c_long) as krb5_error_code;
                krb5_set_error_message(context, retval,
                                       dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"No user identity options specified\x00"
                                                    as *const u8 as
                                                    *const libc::c_char));
                pkiDebug(b"%s: no user identity options specified\n\x00" as
                             *const u8 as *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 27],
                                                   &[libc::c_char; 27]>(b"pkinit_identity_initialize\x00")).as_ptr());
                current_block = 2446719291013035946;
            }
            match current_block {
                2446719291013035946 => { }
                _ => {
                    if !(retval != 0) {
                        retval =
                            crypto_load_certs(context, plg_cryptoctx,
                                              req_cryptoctx, idopts,
                                              id_cryptoctx, princ,
                                              1 as libc::c_int as
                                                  krb5_boolean);
                        if !(retval != 0) {
                            crypto_free_cert_info(context, plg_cryptoctx,
                                                  req_cryptoctx,
                                                  id_cryptoctx);
                        }
                    }
                }
            }
        }
    } else {
        /* We're the anonymous principal. */
        retval = 0 as libc::c_int
    }
    return retval;
}
/*
 * Load identity information, including that which requires us to ask a
 * controlling user any questions.  If we have PIN/password values which
 * correspond to a given identity, use that, otherwise, if one is available,
 * we'll use the prompter callback.
 */
#[no_mangle]
#[c2rust::src_loc = "568:1"]
pub unsafe extern "C" fn pkinit_identity_prompt(mut context: krb5_context,
                                                mut plg_cryptoctx:
                                                    pkinit_plg_crypto_context,
                                                mut req_cryptoctx:
                                                    pkinit_req_crypto_context,
                                                mut idopts:
                                                    *mut pkinit_identity_opts,
                                                mut id_cryptoctx:
                                                    pkinit_identity_crypto_context,
                                                mut cb:
                                                    krb5_clpreauth_callbacks,
                                                mut rock: krb5_clpreauth_rock,
                                                mut do_matching: libc::c_int,
                                                mut princ: krb5_principal)
 -> krb5_error_code {
    let mut current_block: u64; /* Not anonymous principal */
    let mut retval: krb5_error_code = 22 as libc::c_int;
    let mut signer_identity: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    pkiDebug(b"%s: %p %p %p\n\x00" as *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 23],
                                       &[libc::c_char; 23]>(b"pkinit_identity_prompt\x00")).as_ptr(),
             context, idopts, id_cryptoctx);
    if !(!princ.is_null() &&
             krb5_principal_compare_any_realm(context,
                                              princ as krb5_const_principal,
                                              krb5_anonymous_principal()) !=
                 0) {
        retval =
            crypto_load_certs(context, plg_cryptoctx, req_cryptoctx, idopts,
                              id_cryptoctx, princ,
                              0 as libc::c_int as krb5_boolean);
        if retval != 0 {
            current_block = 13810939424077242397;
        } else {
            if do_matching != 0 {
                /*
             * Try to select exactly one certificate based on matching
             * criteria.  Typical used for clients.
             */
                retval =
                    pkinit_cert_matching(context, plg_cryptoctx,
                                         req_cryptoctx, id_cryptoctx, princ);
                if retval != 0 {
                    crypto_free_cert_info(context, plg_cryptoctx,
                                          req_cryptoctx, id_cryptoctx);
                    current_block = 13810939424077242397;
                } else { current_block = 12349973810996921269; }
            } else {
                /*
             * Tell crypto code to use the "default" identity.  Typically used
             * for KDCs.
             */
                retval =
                    crypto_cert_select_default(context, plg_cryptoctx,
                                               req_cryptoctx, id_cryptoctx);
                if retval != 0 {
                    crypto_free_cert_info(context, plg_cryptoctx,
                                          req_cryptoctx, id_cryptoctx);
                    current_block = 13810939424077242397;
                } else { current_block = 12349973810996921269; }
            }
            match current_block {
                13810939424077242397 => { }
                _ => {
                    if !rock.is_null() && !cb.is_null() &&
                           retval == 0 as libc::c_int {
                        /* Save the signer identity if we're the client. */
                        if crypto_retrieve_signer_identity(context,
                                                           id_cryptoctx,
                                                           &mut signer_identity)
                               == 0 as libc::c_int {
                            (*cb).set_cc_config.expect("non-null function pointer")(context,
                                                                                    rock,
                                                                                    b"X509_user_identity\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char,
                                                                                    signer_identity);
                        }
                    }
                    retval =
                        crypto_free_cert_info(context, plg_cryptoctx,
                                              req_cryptoctx, id_cryptoctx);
                    if retval != 0 {
                        current_block = 13810939424077242397;
                    } else { current_block = 18317007320854588510; }
                }
            }
        }
    } else { current_block = 18317007320854588510; }
    match current_block {
        18317007320854588510 => {
            i = 0 as libc::c_int;
            loop  {
                if !(!(*idopts).anchors.is_null() &&
                         !(*(*idopts).anchors.offset(i as isize)).is_null()) {
                    current_block = 14818589718467733107;
                    break ;
                }
                retval =
                    process_option_ca_crl(context, plg_cryptoctx,
                                          req_cryptoctx, idopts, id_cryptoctx,
                                          *(*idopts).anchors.offset(i as
                                                                        isize),
                                          1 as libc::c_int);
                if retval != 0 {
                    current_block = 13810939424077242397;
                    break ;
                }
                i += 1
            }
            match current_block {
                13810939424077242397 => { }
                _ => {
                    i = 0 as libc::c_int;
                    loop  {
                        if !(!(*idopts).intermediates.is_null() &&
                                 !(*(*idopts).intermediates.offset(i as
                                                                       isize)).is_null())
                           {
                            current_block = 2891135413264362348;
                            break ;
                        }
                        retval =
                            process_option_ca_crl(context, plg_cryptoctx,
                                                  req_cryptoctx, idopts,
                                                  id_cryptoctx,
                                                  *(*idopts).intermediates.offset(i
                                                                                      as
                                                                                      isize),
                                                  2 as libc::c_int);
                        if retval != 0 {
                            current_block = 13810939424077242397;
                            break ;
                        }
                        i += 1
                    }
                    match current_block {
                        13810939424077242397 => { }
                        _ => {
                            i = 0 as libc::c_int;
                            while !(*idopts).crls.is_null() &&
                                      !(*(*idopts).crls.offset(i as
                                                                   isize)).is_null()
                                  {
                                retval =
                                    process_option_ca_crl(context,
                                                          plg_cryptoctx,
                                                          req_cryptoctx,
                                                          idopts,
                                                          id_cryptoctx,
                                                          *(*idopts).crls.offset(i
                                                                                     as
                                                                                     isize),
                                                          3 as libc::c_int);
                                if retval != 0 { break ; }
                                i += 1
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    return retval;
}
/*
 * Create an entry in the passed-in list for the named identity, optionally
 * with the specified token flag value and/or supplied password, replacing any
 * existing entry with the same identity name.
 */
#[no_mangle]
#[c2rust::src_loc = "666:1"]
pub unsafe extern "C" fn pkinit_set_deferred_id(mut identities:
                                                    *mut *mut pkinit_deferred_id,
                                                mut identity:
                                                    *const libc::c_char,
                                                mut ck_flags: libc::c_ulong,
                                                mut password:
                                                    *const libc::c_char)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut out: *mut pkinit_deferred_id = 0 as *mut pkinit_deferred_id;
    let mut ids: *mut pkinit_deferred_id = 0 as *mut pkinit_deferred_id;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Search for an entry that's already in the list. */
    ids = *identities;
    i = 0 as libc::c_int;
    while !ids.is_null() && !(*ids.offset(i as isize)).is_null() {
        if strcmp((**ids.offset(i as isize)).identity, identity) ==
               0 as libc::c_int {
            /* Replace its password value, then we're done. */
            tmp =
                if !password.is_null() {
                    strdup(password)
                } else { 0 as *mut libc::c_char };
            if !password.is_null() && tmp.is_null() {
                return 12 as libc::c_int
            }
            (**ids.offset(i as isize)).ck_flags = ck_flags;
            free((**ids.offset(i as isize)).password as *mut libc::c_void);
            let ref mut fresh3 = (**ids.offset(i as isize)).password;
            *fresh3 = tmp;
            return 0 as libc::c_int
        }
        i += 1
    }
    /* Resize the list. */
    out =
        realloc(ids as *mut libc::c_void,
                (::std::mem::size_of::<pkinit_deferred_id>() as
                     libc::c_ulong).wrapping_mul((i + 2 as libc::c_int) as
                                                     libc::c_ulong)) as
            *mut pkinit_deferred_id;
    if !out.is_null() {
        *identities = out;
        /* Allocate the new final entry. */
        let ref mut fresh4 = *out.offset(i as isize);
        *fresh4 =
            malloc(::std::mem::size_of::<_pkinit_deferred_id>() as
                       libc::c_ulong) as pkinit_deferred_id;
        if !(*out.offset(i as isize)).is_null() {
            /* Populate the new entry. */
            (**out.offset(i as isize)).magic = 0x3ca20d21 as libc::c_int;
            let ref mut fresh5 = (**out.offset(i as isize)).identity;
            *fresh5 = strdup(identity);
            if !(**out.offset(i as isize)).identity.is_null() {
                (**out.offset(i as isize)).ck_flags = ck_flags;
                let ref mut fresh6 = (**out.offset(i as isize)).password;
                *fresh6 =
                    if !password.is_null() {
                        strdup(password)
                    } else { 0 as *mut libc::c_char };
                if !(!password.is_null() &&
                         (**out.offset(i as isize)).password.is_null()) {
                    /* Terminate the list. */
                    let ref mut fresh7 =
                        *out.offset((i + 1 as libc::c_int) as isize);
                    *fresh7 = 0 as pkinit_deferred_id;
                    return 0 as libc::c_int
                }
            }
        }
    }
    if !out.is_null() && !(*out.offset(i as isize)).is_null() {
        free((**out.offset(i as isize)).identity as *mut libc::c_void);
        free(*out.offset(i as isize) as *mut libc::c_void);
        let ref mut fresh8 = *out.offset(i as isize);
        *fresh8 = 0 as pkinit_deferred_id
    }
    return 12 as libc::c_int;
}
/*
 * Return a password which we've associated with the named identity, if we've
 * stored one.  Otherwise return NULL.
 */
#[no_mangle]
#[c2rust::src_loc = "729:1"]
pub unsafe extern "C" fn pkinit_find_deferred_id(mut identities:
                                                     *mut pkinit_deferred_id,
                                                 mut identity:
                                                     *const libc::c_char)
 -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !identities.is_null() && !(*identities.offset(i as isize)).is_null()
          {
        if strcmp((**identities.offset(i as isize)).identity, identity) ==
               0 as libc::c_int {
            return (**identities.offset(i as isize)).password
        }
        i += 1
    }
    return 0 as *const libc::c_char;
}
/*
 * Return the flags associated with the specified identity, or 0 if we don't
 * have such an identity.
 */
#[no_mangle]
#[c2rust::src_loc = "746:1"]
pub unsafe extern "C" fn pkinit_get_deferred_id_flags(mut identities:
                                                          *mut pkinit_deferred_id,
                                                      mut identity:
                                                          *const libc::c_char)
 -> libc::c_ulong {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !identities.is_null() && !(*identities.offset(i as isize)).is_null()
          {
        if strcmp((**identities.offset(i as isize)).identity, identity) ==
               0 as libc::c_int {
            return (**identities.offset(i as isize)).ck_flags
        }
        i += 1
    }
    return 0 as libc::c_int as libc::c_ulong;
}
/*
 * COPYRIGHT (C) 2006,2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
/* pkinit_kdc_ocsp has been removed */
/* Make pkiDebug(fmt,...) print, or not.  */
/* Still evaluates for side effects.  */
/* This is better if the compiler doesn't inline variadic functions
   well, but gcc will warn about "left-hand operand of comma
   expression has no effect".  Still evaluates for side effects.  */
/* #define pkiDebug	(void) */
/* Solaris compiler doesn't grok __FUNCTION__
 * hack for now.  Fix all the uses eventually. */
/* Macros to deal with converting between various data types... */
/*
 * notes about crypto contexts:
 *
 * the basic idea is that there are crypto contexts that live at
 * both the plugin level and request level. the identity context (that
 * keeps info about your own certs and such) is separate because
 * it is needed at different levels for the kdc and and the client.
 * (the kdc's identity is at the plugin level, the client's identity
 * information could change per-request.)
 * the identity context is meant to have the entity's cert,
 * a list of trusted and intermediate cas, a list of crls, and any
 * pkcs11 information.  the req context is meant to have the
 * received certificate and the DH related information. the plugin
 * context is meant to have global crypto information, i.e., OIDs
 * and constant DH parameter information.
 */
/*
 * plugin crypto context should keep plugin common information,
 * eg., OIDs, known DHparams
 */
/*
 * request crypto context should keep reqyest common information,
 * eg., received credentials, DH parameters of this request
 */
/*
 * identity context should keep information about credentials
 * for the request, eg., my credentials, trusted ca certs,
 * intermediate ca certs, crls, pkcs11 info
 */
/*
 * this structure keeps information about the config options
 */
/* require EKU checking (default is true) */
/* accept secondary EKU (default is false) */
/* allow UPN-SAN instead of pkinit-SAN */
/* selects DH or RSA based pkinit */
/* require CRL for a CA (default is false) */
/* require freshness token (default is false) */
/* disable freshness token on client for testing */
/* minimum DH modulus size allowed */
/*
 * this structure keeps options used for a given request
 */
/* initial request DH modulus size (default=1024) */
/*
 * information about identity from config file or command line
 */
/*
 * Client's plugin context
 */
/*
 * Client's per-request context
 */
/*
 * KDC's (per-realm) plugin context
 */
/*
 * KDC's per-request context
 */
/*
 * Functions in pkinit_lib.c
 */
/*
 * Functions in pkinit_identity.c
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN/OUT */
/* IN/OUT */
/* IN/OUT */
/* IN (optional) */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN/OUT */
/* IN/OUT */
/* IN/OUT */
/* IN */
/* IN (optional) */
/*
 * Client's list of identities for which it needs PINs or passwords
 */
/*
 * Free a deferred_id list.
 */
#[no_mangle]
#[c2rust::src_loc = "762:1"]
pub unsafe extern "C" fn pkinit_free_deferred_ids(mut identities:
                                                      *mut pkinit_deferred_id) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !identities.is_null() && !(*identities.offset(i as isize)).is_null()
          {
        free((**identities.offset(i as isize)).identity as *mut libc::c_void);
        free((**identities.offset(i as isize)).password as *mut libc::c_void);
        free(*identities.offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    free(identities as *mut libc::c_void);
}
