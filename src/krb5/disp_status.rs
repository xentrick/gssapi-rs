use ::libc;

pub mod error_map_h {

    #[inline]

    pub unsafe extern "C" fn gsserrmap_init(
        mut head: *mut crate::error_map_h::gsserrmap__head,
    ) -> i32 {
        (*head).first = 0 as *mut crate::error_map_h::gsserrmap__element;
        return 0i32;
    }
    #[inline]

    pub unsafe extern "C" fn gsserrmap_destroy(mut head: *mut crate::error_map_h::gsserrmap__head) {
        let mut e = 0 as *mut crate::error_map_h::gsserrmap__element;
        let mut e_next = 0 as *mut crate::error_map_h::gsserrmap__element;
        let mut free_key: Option<unsafe extern "C" fn(_: crate::gssapi_h::OM_uint32) -> ()> = None;
        let mut free_value: Option<unsafe extern "C" fn(_: *mut i8) -> ()> =
            Some(free_string as unsafe extern "C" fn(_: *mut i8) -> ());
        e = (*head).first;
        while !e.is_null() {
            e_next = (*e).next;
            if free_key.is_some() {
                Some(free_key.expect("non-null function pointer"))
                    .expect("non-null function pointer")((*e).key);
            }
            if free_value.is_some() {
                Some(free_value.expect("non-null function pointer"))
                    .expect("non-null function pointer")((*e).value);
            }
            crate::stdlib::free(e as *mut libc::c_void);
            e = e_next
        }
        (*head).first = 0 as *mut crate::error_map_h::gsserrmap__element;
    }
    /* Returns pointer to linked-list entry, or null if key not found.  */
    #[inline]

    pub unsafe extern "C" fn gsserrmap__find_node(
        mut head: *mut crate::error_map_h::gsserrmap__head,
        mut key: crate::gssapi_h::OM_uint32,
    ) -> *mut crate::error_map_h::gsserrmap__element {
        let mut e = 0 as *mut crate::error_map_h::gsserrmap__element;
        e = (*head).first;
        while !e.is_null() {
            if compare_OM_uint32(key, (*e).key) == 0i32 {
                return e;
            }
            e = (*e).next
        }
        return 0 as *mut crate::error_map_h::gsserrmap__element;
    }
    /* Returns pointer to value, or null if key not found.  */
    #[inline]

    pub unsafe extern "C" fn gsserrmap_find(
        mut head: *mut crate::error_map_h::gsserrmap__head,
        mut key: crate::gssapi_h::OM_uint32,
    ) -> *mut *mut i8 {
        let mut e = gsserrmap__find_node(head, key);
        if !e.is_null() {
            return &mut (*e).value;
        }
        return 0 as *mut *mut i8;
    }
    /* Returns 0 or error code.  */
    #[inline]

    pub unsafe extern "C" fn gsserrmap__copy_key(
        mut out: *mut crate::gssapi_h::OM_uint32,
        mut in_0: crate::gssapi_h::OM_uint32,
    ) -> i32 {
        let mut copykey: Option<
            unsafe extern "C" fn(
                _: *mut crate::gssapi_h::OM_uint32,
                _: crate::gssapi_h::OM_uint32,
            ) -> i32,
        > = None;
        if copykey.is_none() {
            *out = in_0;
            return 0i32;
        } else {
            return Some(copykey.expect("non-null function pointer"))
                .expect("non-null function pointer")(out, in_0);
        };
    }
    /* Returns 0 or error code.  */
    #[inline]

    pub unsafe extern "C" fn gsserrmap_replace_or_insert(
        mut head: *mut crate::error_map_h::gsserrmap__head,
        mut key: crate::gssapi_h::OM_uint32,
        mut new_value: *mut i8,
    ) -> i32 {
        let mut e = gsserrmap__find_node(head, key);
        let mut ret: i32 = 0;
        if !e.is_null() {
            /* replace */
            let mut free_value: Option<unsafe extern "C" fn(_: *mut i8) -> ()> =
                Some(free_string as unsafe extern "C" fn(_: *mut i8) -> ());
            if free_value.is_some() {
                Some(free_value.expect("non-null function pointer"))
                    .expect("non-null function pointer")((*e).value);
            }
            (*e).value = new_value
        } else {
            /* insert */
            e = crate::stdlib::malloc(
                ::std::mem::size_of::<crate::error_map_h::gsserrmap__element>(),
            ) as *mut crate::error_map_h::gsserrmap__element;
            if e.is_null() {
                return 12i32;
            }
            ret = gsserrmap__copy_key(&mut (*e).key, key);
            if ret != 0 {
                crate::stdlib::free(e as *mut libc::c_void);
                return ret;
            }
            (*e).value = new_value;
            (*e).next = (*head).first;
            (*head).first = e
        }
        return 0i32;
    }

    use crate::src::krb5::disp_status::compare_OM_uint32;
    use crate::src::krb5::disp_status::free_string;
}

/* _GSSAPIP_KRB5_H_ */
pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_context;
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
pub use crate::k5_int_h::localauth_module_handle;
pub use crate::k5_int_h::plugin_interface;
pub use crate::k5_int_h::plugin_mapping;
pub use crate::k5_int_h::CANONHOST_FALLBACK;
pub use crate::k5_int_h::CANONHOST_FALSE;
pub use crate::k5_int_h::CANONHOST_TRUE;
pub use crate::k5_plugin_h::plugin_dir_handle;
pub use crate::k5_plugin_h::plugin_file_handle;
pub use crate::k5_thread_h::k5_key_t;
pub use crate::k5_thread_h::krb5int_getspecific;
pub use crate::k5_thread_h::krb5int_setspecific;
pub use crate::k5_thread_h::K5_KEY_COM_ERR;
pub use crate::k5_thread_h::K5_KEY_GSS_KRB5_CCACHE_NAME;
pub use crate::k5_thread_h::K5_KEY_GSS_KRB5_ERROR_MESSAGE;
pub use crate::k5_thread_h::K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME;
pub use crate::k5_thread_h::K5_KEY_GSS_SPNEGO_STATUS;
pub use crate::k5_thread_h::K5_KEY_MAX;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::_krb5_trace_info;
pub use crate::krb5_h::_profile_t;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_error_message;
pub use crate::krb5_h::krb5_get_error_message;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_set_error_message;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::profile_h::profile_t;

pub use crate::com_err_h::errcode_t;
pub use crate::com_err_h::error_message;
pub use crate::error_map_h::gsserrmap;
pub use crate::error_map_h::gsserrmap__element;
pub use crate::error_map_h::gsserrmap__head;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::src::krb5::disp_status::error_map_h::gsserrmap__copy_key;
pub use crate::src::krb5::disp_status::error_map_h::gsserrmap__find_node;
pub use crate::src::krb5::disp_status::error_map_h::gsserrmap_destroy;
pub use crate::src::krb5::disp_status::error_map_h::gsserrmap_find;
pub use crate::src::krb5::disp_status::error_map_h::gsserrmap_init;
pub use crate::src::krb5::disp_status::error_map_h::gsserrmap_replace_or_insert;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
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
/* XXXX internationalization!! */
#[inline]

unsafe extern "C" fn compare_OM_uint32(
    mut a: crate::gssapi_h::OM_uint32,
    mut b: crate::gssapi_h::OM_uint32,
) -> i32 {
    if a < b {
        return -(1i32);
    } else if a == b {
        return 0i32;
    } else {
        return 1i32;
    };
}
#[inline]

unsafe extern "C" fn free_string(mut s: *mut i8) {
    crate::stdlib::free(s as *mut libc::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_get_error_message(
    mut minor_code: crate::gssapi_h::OM_uint32,
) -> *mut i8 {
    let mut p =
        crate::k5_thread_h::krb5int_getspecific(crate::k5_thread_h::K5_KEY_GSS_KRB5_ERROR_MESSAGE)
            as *mut crate::error_map_h::gsserrmap;
    let mut msg = 0 as *mut i8;
    if !p.is_null() {
        let mut v = gsserrmap_find(p, minor_code);
        if !v.is_null() {
            msg = *v
        }
    }
    if msg.is_null() {
        msg = crate::com_err_h::error_message(
            minor_code as crate::krb5_h::krb5_error_code as crate::com_err_h::errcode_t,
        ) as *mut i8
    }
    return msg;
}

unsafe extern "C" fn gss_krb5_save_error_string_nocopy(
    mut minor_code: crate::gssapi_h::OM_uint32,
    mut msg: *mut i8,
) -> i32 {
    let mut current_block: u64;
    let mut p = 0 as *mut crate::error_map_h::gsserrmap;
    let mut ret: i32 = 0;
    p = crate::k5_thread_h::krb5int_getspecific(crate::k5_thread_h::K5_KEY_GSS_KRB5_ERROR_MESSAGE)
        as *mut crate::error_map_h::gsserrmap;
    if p.is_null() {
        p = crate::stdlib::malloc(::std::mem::size_of::<crate::error_map_h::gsserrmap>())
            as *mut crate::error_map_h::gsserrmap;
        if p.is_null() {
            ret = 1i32;
            current_block = 9353323475770310880;
        } else if gsserrmap_init(p) != 0i32 {
            crate::stdlib::free(p as *mut libc::c_void);
            p = 0 as *mut crate::error_map_h::gsserrmap;
            ret = 1i32;
            current_block = 9353323475770310880;
        } else if crate::k5_thread_h::krb5int_setspecific(
            crate::k5_thread_h::K5_KEY_GSS_KRB5_ERROR_MESSAGE,
            p as *mut libc::c_void,
        ) != 0i32
        {
            gsserrmap_destroy(p);
            crate::stdlib::free(p as *mut libc::c_void);
            p = 0 as *mut crate::error_map_h::gsserrmap;
            ret = 1i32;
            current_block = 9353323475770310880;
        } else {
            current_block = 1054647088692577877;
        }
    } else {
        current_block = 1054647088692577877;
    }
    match current_block {
        1054647088692577877 => ret = gsserrmap_replace_or_insert(p, minor_code, msg),
        _ => {}
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_save_error_string(
    mut minor_code: crate::gssapi_h::OM_uint32,
    mut msg: *mut i8,
) {
    let mut s = crate::stdlib::strdup(msg);
    if !s.is_null() {
        if gss_krb5_save_error_string_nocopy(minor_code, s) != 0i32 {
            crate::stdlib::free(s as *mut libc::c_void);
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_save_error_message(
    mut minor_code: crate::gssapi_h::OM_uint32,
    mut format: *const i8,
    mut args: ...
) {
    let mut s = 0 as *mut i8;
    let mut n: i32 = 0;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    n = crate::stdlib::vasprintf(&mut s, format, ap.as_va_list());
    if n >= 0i32 {
        if gss_krb5_save_error_string_nocopy(minor_code, s) != 0i32 {
            crate::stdlib::free(s as *mut libc::c_void);
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_save_error_info(
    mut minor_code: crate::gssapi_h::OM_uint32,
    mut ctx: crate::krb5_h::krb5_context,
) {
    let mut s = 0 as *mut i8;
    s = crate::krb5_h::krb5_get_error_message(ctx, minor_code as crate::krb5_h::krb5_error_code)
        as *mut i8;
    krb5_gss_save_error_string(minor_code, s);
    /* The get_error_message call above resets the error message in
    ctx.  Put it back, in case we make this call again *sigh*.  */
    crate::krb5_h::krb5_set_error_message(
        ctx,
        minor_code as crate::krb5_h::krb5_error_code,
        b"%s\x00" as *const u8 as *const i8,
        s,
    );
    crate::krb5_h::krb5_free_error_message(ctx, s);
}
/* minor_status */
/* mech_set */
/* minor_status */
/* name1 */
/* name2 */
/* name_equal */
/* minor_status */
/* input_name */
/* output_name_buffer */
/* output_name_type */
/* minor_status */
/* input_name_buffer */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* minor_status */
/* cred_handle */
/* name */
/* lifetime */
/* cred_usage */
/* mechanisms */
/* minor_status */
/* context_handle */
/* initiator_name */
/* acceptor_name */
/* lifetime_rec */
/* mech_type */
/* ret_flags */
/* locally_initiated */
/* open */
/* New V2 entry points */
/* minor_status */
/* context_handle */
/* qop_req */
/* message_buffer */
/* message_token */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* message_buffer */
/* message_token */
/* qop_state */
/* minor_status */
/* context_handle */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_message_buffer */
/* conf_state */
/* output_message_buffer */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* output_message_buffer */
/* conf_state */
/* qop_state */
/* minor_status */
/* context_handle */
/* conf_state */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* req_output_size */
/* max_input_size */
/* minor_status */
/* input_name */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* desired_name_type */
/* output_name */
/* minor_status */
/* cred_handle */
/* mech_type */
/* name */
/* initiator_lifetime */
/* acceptor_lifetime */
/* cred_usage */
/* minor_status */
/* context_handle */
/* interprocess_token */
/* minor_status */
/* interprocess_token */
/* context_handle */
/* LEAN_CLIENT */
/* minor_status */
/* oid */
/* minor_status */
/* oid */
/* minor_status */
/* mechanism */
/* name_types */
/* minor_status */
/* input_name */
/* mech_type */
/* output_name */
/* minor_status */
/* input_name */
/* exported_name */
/* minor_status */
/* input_name */
/* dest_name */
/* minor_status */
/* cred */
/* minor_status */
/* impersonator_cred_handle */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* cred_handle */
/* context */
/* naming_exts.c */
/* s4u_gss_glue.c */
/*
 * These take unglued krb5-mech-specific contexts.
 */
/* _GSS_STATIC_LINK */
/* For error message handling.  */
/* Returns a shared string, not a private copy!  */
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_delete_error_info(mut p: *mut libc::c_void) {
    gsserrmap_destroy(p as *mut crate::error_map_h::gsserrmap__head);
    crate::stdlib::free(p);
}
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 2000, 2008 by the Massachusetts Institute of Technology.
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
 *
 */
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
/* work around sunos braindamage */
/* The include of gssapi_krb5.h will dtrt with the above #defines in
 * effect.
 */
/* for debugging */
/* * constants **/
/* Incorrect krb5 mech OID emitted by MS. */
/* IAKERB variant */
/* * CFX flags **/
/* These are to be stored in little-endian order, i.e., des-mac is
stored as 02 00.  */
/* SGN_ALG_DES_MAC_MD5           = 0x0000, */
/* SGN_ALG_MD2_5                 = 0x0001, */
/* SGN_ALG_DES_MAC               = 0x0002, */
/* SGN_ALG_3                     = 0x0003, /\* not published *\/ */
/* microsoft w2k;  */
/* SEAL_ALG_DES             = 0x0000, */
/* SEAL_ALG_1               = 0x0001, /\* not published *\/ */
/* microsoft w2k;  */
/* for 3DES */
/* for draft-ietf-krb-wg-gssapi-cfx-01 */
/* GSS_KRB5_INTEG_C_QOP_MD5       = 0x0001, */
/* GSS_KRB5_INTEG_C_QOP_DES_MD5   = 0x0002, */
/* GSS_KRB5_INTEG_C_QOP_DES_MAC   = 0x0003, */
/* GSS_KRB5_CONF_C_QOP_DES        = 0x0100, */
/* * internal types **/
/* immutable */
/* immutable */
/* immutable */
/* protects ad_context only for now */
/* protect against simultaneous accesses */
/* name/type of credential */
/* keytab (accept) data */
/* ccache (init) data */
/* limit negotiated enctypes to this list */
/* nonzero if initiating, zero if accepting */
/* XXX tested but never actually set */
/* One of two potential keys to use with RFC 4121
 * packets; this key must always be set. */
/* RFC 1964 encryption key; seq xored with a constant
 * for DES, seq for other RFC 1964 enctypes  */
/* RFC 1964 sequencing key */
/* XXX these used to be signed.  the old spec is inspecific, and
the new spec specifies unsigned.  I don't believe that the change
affects the wire encoding. */
/* Protocol spec revision for sending packets
0 => RFC 1964 with 3DES and RC4 enhancements
1 => RFC 4121
No others defined so far.  It is always permitted to receive
tokens in RFC 4121 format.  If enc is non-null, receiving RFC
1964 tokens is permitted.*/
/* for "main" subkey */
/* CFX only */
/* did we get rcache from creds? */
/* LEAN_CLIENT */
/* * helper functions **/
/* Encrypt length bytes at ptr in place, with the given key and usage.  If
 * iv is not NULL, use it as the cipher state. */
/* AEAD */
/* for conf len */
/* * declarations of internal name mechanism functions **/
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* cred_handle */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* exts */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/*exts */
/* LEAN_CLIENT */
/* minor_status */
/* context_handle */
/* desired_object */
/* data_set */
/* minor_status */
/* context_handle */
/* desired_object */
/* value */
/* minor_status */
/* context_handle */
/* token_buffer */
/* minor_status */
/* context_handle */
/* output_token */
/* minor_status */
/* context_handle */
/* time_rec */
/* */
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_display_status(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut status_value: crate::gssapi_h::OM_uint32,
    mut status_type: i32,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut message_context: *mut crate::gssapi_h::OM_uint32,
    mut status_string: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    (*status_string).length = 0usize;
    (*status_string).value = 0 as *mut libc::c_void;
    if !mech_type.is_null()
        && !((*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length == (*mech_type).length
            && crate::stdlib::memcmp(
                (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).elements,
                (*mech_type).elements,
                (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length as usize,
            ) == 0i32)
        && !((*crate::src::krb5::gssapi_krb5::gss_mech_krb5_old).length == (*mech_type).length
            && crate::stdlib::memcmp(
                (*crate::src::krb5::gssapi_krb5::gss_mech_krb5_old).elements,
                (*mech_type).elements,
                (*crate::src::krb5::gssapi_krb5::gss_mech_krb5_old).length as usize,
            ) == 0i32)
        && !((*crate::src::krb5::gssapi_krb5::gss_mech_iakerb).length == (*mech_type).length
            && crate::stdlib::memcmp(
                (*crate::src::krb5::gssapi_krb5::gss_mech_iakerb).elements,
                (*mech_type).elements,
                (*crate::src::krb5::gssapi_krb5::gss_mech_iakerb).length as usize,
            ) == 0i32)
    {
        *minor_status = 0u32;
        return (1u32) << 16i32;
    }
    if status_type == 1i32 {
        return crate::src::generic::disp_major_status::gssint_g_display_major_status(
            minor_status,
            status_value,
            message_context,
            status_string,
        );
    } else if status_type == 2i32 {
        crate::src::krb5::gssapi_krb5::gss_krb5int_initialize_library();
        if *message_context != 0 {
            *minor_status = -(2045022971 as isize) as crate::gssapi_h::OM_uint32;
            return (13u32) << 16i32;
        }
        /* If this fails, there's not much we can do...  */
        if crate::src::generic::util_buffer::gssint_g_make_string_buffer(
            krb5_gss_get_error_message(status_value),
            status_string,
        ) == 0
        {
            *minor_status = 12u32;
            return (13u32) << 16i32;
        }
        *minor_status = 0u32;
        return 0u32;
    } else {
        *minor_status = 0u32;
        return (5u32) << 16i32;
    };
}
