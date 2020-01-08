pub mod k5_thread_h {

    #[inline]

    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut crate::k5_thread_h::k5_mutex_t) {
        let mut r = crate::k5_thread_h::k5_os_mutex_unlock(m);
        if r != 0i32 {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"k5_mutex_unlock: Received error %d (%s)\n\x00" as *const u8 as *const i8,
                r,
                crate::stdlib::strerror(r),
            );
        }
        if r == 0i32 {
        } else {
            crate::stdlib::__assert_fail(
                b"r == 0\x00" as *const u8 as *const i8,
                b"../../../include/k5-thread.h\x00" as *const u8 as *const i8,
                388u32,
                (*::std::mem::transmute::<&[u8; 35], &[i8; 35]>(
                    b"void k5_mutex_unlock(k5_mutex_t *)\x00",
                ))
                .as_ptr(),
            );
        };
    }

    /* multiple inclusion? */
    /* In time, many of the definitions above should move into the support
    library, and this file should be greatly simplified.  For type
    definitions, that'll take some work, since other data structures
    incorporate mutexes directly, and our mutex type is dependent on
    configuration options and system attributes.  For most functions,
    though, it should be relatively easy.

    For now, plugins should use the exported functions, and not the
    above macros, and use krb5int_mutex_alloc for allocations.  */
}

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::pthread_mutex_t;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_authdata_context;
pub use crate::k5_int_h::_krb5_authdata_context_module;
pub use crate::k5_int_h::_krb5_context;
pub use crate::k5_int_h::_krb5_kt;
pub use crate::k5_int_h::_krb5_kt_ops;
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
pub use crate::k5_int_h::krb5int_cc_default;
pub use crate::k5_int_h::localauth_module_handle;
pub use crate::k5_int_h::plugin_interface;
pub use crate::k5_int_h::plugin_mapping;
pub use crate::k5_int_h::CANONHOST_FALLBACK;
pub use crate::k5_int_h::CANONHOST_FALSE;
pub use crate::k5_int_h::CANONHOST_TRUE;
pub use crate::k5_plugin_h::plugin_dir_handle;
pub use crate::k5_plugin_h::plugin_file_handle;
pub use crate::k5_thread_h::k5_mutex_t;
pub use crate::k5_thread_h::k5_os_mutex;
pub use crate::k5_thread_h::k5_os_mutex_unlock;
pub use crate::krb5_h::_krb5_address;
pub use crate::krb5_h::_krb5_ap_req;
pub use crate::krb5_h::_krb5_auth_context;
pub use crate::krb5_h::_krb5_authdata;
pub use crate::krb5_h::_krb5_ccache;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::_krb5_enc_data;
pub use crate::krb5_h::_krb5_enc_tkt_part;
pub use crate::krb5_h::_krb5_keyblock;
pub use crate::krb5_h::_krb5_ticket;
pub use crate::krb5_h::_krb5_ticket_times;
pub use crate::krb5_h::_krb5_trace_info;
pub use crate::krb5_h::_krb5_transited;
pub use crate::krb5_h::_profile_t;
pub use crate::krb5_h::krb5_address;
pub use crate::krb5_h::krb5_addrtype;
pub use crate::krb5_h::krb5_ap_req;
pub use crate::krb5_h::krb5_auth_context;
pub use crate::krb5_h::krb5_authdata;
pub use crate::krb5_h::krb5_authdatatype;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_cc_close;
pub use crate::krb5_h::krb5_cc_copy_creds;
pub use crate::krb5_h::krb5_cc_initialize;
pub use crate::krb5_h::krb5_cc_resolve;
pub use crate::krb5_h::krb5_ccache;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keytab;
pub use crate::krb5_h::krb5_keytab_entry;
pub use crate::krb5_h::krb5_keytab_entry_st;
pub use crate::krb5_h::krb5_kt_cursor;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_pointer;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_rc_st;
pub use crate::krb5_h::krb5_rcache;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::store_cred::k5_thread_h::k5_mutex_unlock;

pub use crate::authdata_plugin_h::authdata_client_copy_proc;
pub use crate::authdata_plugin_h::authdata_client_delete_attribute_proc;
pub use crate::authdata_plugin_h::authdata_client_export_authdata_proc;
pub use crate::authdata_plugin_h::authdata_client_export_internal_proc;
pub use crate::authdata_plugin_h::authdata_client_externalize_proc;
pub use crate::authdata_plugin_h::authdata_client_free_internal_proc;
pub use crate::authdata_plugin_h::authdata_client_get_attribute_proc;
pub use crate::authdata_plugin_h::authdata_client_get_attribute_types_proc;
pub use crate::authdata_plugin_h::authdata_client_import_authdata_proc;
pub use crate::authdata_plugin_h::authdata_client_internalize_proc;
pub use crate::authdata_plugin_h::authdata_client_plugin_fini_proc;
pub use crate::authdata_plugin_h::authdata_client_plugin_flags_proc;
pub use crate::authdata_plugin_h::authdata_client_plugin_init_proc;
pub use crate::authdata_plugin_h::authdata_client_request_fini_proc;
pub use crate::authdata_plugin_h::authdata_client_request_init_proc;
pub use crate::authdata_plugin_h::authdata_client_set_attribute_proc;
pub use crate::authdata_plugin_h::authdata_client_size_proc;
pub use crate::authdata_plugin_h::authdata_client_verify_proc;
pub use crate::authdata_plugin_h::krb5plugin_authdata_client_ftable_v0;
pub use crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapi_ext_h::gss_const_key_value_set_t;
pub use crate::gssapi_ext_h::gss_key_value_element_desc;
pub use crate::gssapi_ext_h::gss_key_value_element_struct;
pub use crate::gssapi_ext_h::gss_key_value_set_desc;
pub use crate::gssapi_ext_h::gss_key_value_set_struct;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::krb5::acquire_cred::krb5_gss_acquire_cred_from;
pub use crate::src::krb5::cred_store::kg_value_from_cred_store;
pub use crate::src::krb5::init_sec_context::krb5_gss_init_context;
pub use crate::src::krb5::inq_cred::krb5_gss_inquire_cred;
pub use crate::src::krb5::rel_cred::krb5_gss_release_cred;
pub use crate::src::krb5::val_cred::krb5_gss_validate_cred_1;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/store_cred.c */
/*
 * Copyright 2009 by the Massachusetts Institute of Technology.
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

unsafe extern "C" fn has_unexpired_creds(
    mut kcred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    desired_mech: crate::gssapi_h::gss_OID,
    mut default_cred: i32,
    mut cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
) -> i32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut cred_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut desired_mechs = crate::gssapi_h::gss_OID_set_desc {
        count: 0,
        elements: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
    };
    let mut tmp_cred = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut time_rec: crate::gssapi_h::OM_uint32 = 0;
    desired_mechs.count = 1usize;
    desired_mechs.elements = desired_mech;
    if default_cred != 0 {
        cred_name = 0 as crate::gssapi_h::gss_name_t
    } else {
        cred_name = (*kcred).name as crate::gssapi_h::gss_name_t
    }
    major_status = crate::src::krb5::acquire_cred::krb5_gss_acquire_cred_from(
        &mut minor,
        cred_name,
        0u32,
        &mut desired_mechs,
        1i32,
        cred_store,
        &mut tmp_cred,
        0 as *mut crate::gssapi_h::gss_OID_set,
        &mut time_rec,
    );
    crate::src::krb5::rel_cred::krb5_gss_release_cred(&mut minor, &mut tmp_cred);
    return (major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 || time_rec != 0)
        as i32;
}

unsafe extern "C" fn copy_initiator_creds(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_cred_handle: crate::gssapi_h::gss_cred_id_t,
    desired_mech: crate::gssapi_h::gss_OID,
    mut overwrite_cred: crate::gssapi_h::OM_uint32,
    mut default_cred: crate::gssapi_h::OM_uint32,
    mut cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kcred = 0 as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    let mut context = 0 as crate::krb5_h::krb5_context;
    let mut ccache = 0 as crate::krb5_h::krb5_ccache;
    let mut ccache_name = 0 as *const i8;
    *minor_status = 0u32;
    if default_cred == 0 && cred_store.is_null() {
        *minor_status = -(2045022957 as isize) as crate::gssapi_h::OM_uint32;
        major_status = (13u32) << 16i32
    } else {
        code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
        if code != 0i32 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            major_status = (13u32) << 16i32
        } else {
            major_status = crate::src::krb5::val_cred::krb5_gss_validate_cred_1(
                minor_status,
                input_cred_handle,
                context,
            );
            if !(major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                kcred = input_cred_handle as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
                if (*kcred).ccache.is_null() {
                    *minor_status = 39756032u32;
                    major_status = (10u32) << 16i32
                } else if overwrite_cred == 0
                    && has_unexpired_creds(kcred, desired_mech, default_cred as i32, cred_store)
                        != 0
                {
                    major_status = (17u32) << 16i32
                } else {
                    major_status = crate::src::krb5::cred_store::kg_value_from_cred_store(
                        cred_store,
                        b"ccache\x00" as *const u8 as *const i8,
                        &mut ccache_name,
                    );
                    if !(major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                        if !ccache_name.is_null() {
                            code =
                                crate::krb5_h::krb5_cc_resolve(context, ccache_name, &mut ccache);
                            if code != 0i32 {
                                *minor_status = code as crate::gssapi_h::OM_uint32;
                                major_status = (13u32) << 16i32;
                                current_block = 5850311850572335389;
                            } else {
                                code = crate::krb5_h::krb5_cc_initialize(
                                    context,
                                    ccache,
                                    (*(*kcred).name).princ,
                                );
                                if code != 0i32 {
                                    *minor_status = code as crate::gssapi_h::OM_uint32;
                                    major_status = (13u32) << 16i32;
                                    current_block = 5850311850572335389;
                                } else {
                                    current_block = 18377268871191777778;
                                }
                            }
                        } else {
                            current_block = 18377268871191777778;
                        }
                        match current_block {
                            5850311850572335389 => {}
                            _ => {
                                if ccache.is_null() {
                                    if default_cred == 0 {
                                        *minor_status =
                                            -(2045022957 as isize) as crate::gssapi_h::OM_uint32;
                                        major_status = (13u32) << 16i32;
                                        current_block = 5850311850572335389;
                                    } else {
                                        code = crate::k5_int_h::krb5int_cc_default(
                                            context,
                                            &mut ccache,
                                        );
                                        if code != 0i32 {
                                            *minor_status = code as crate::gssapi_h::OM_uint32;
                                            major_status = (13u32) << 16i32;
                                            current_block = 5850311850572335389;
                                        } else {
                                            current_block = 12199444798915819164;
                                        }
                                    }
                                } else {
                                    current_block = 12199444798915819164;
                                }
                                match current_block {
                                    5850311850572335389 => {}
                                    _ => {
                                        code = crate::krb5_h::krb5_cc_copy_creds(
                                            context,
                                            (*kcred).ccache,
                                            ccache,
                                        );
                                        if code != 0i32 {
                                            *minor_status = code as crate::gssapi_h::OM_uint32;
                                            major_status = (13u32) << 16i32
                                        } else {
                                            *minor_status = 0u32;
                                            major_status = 0u32
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
    if !kcred.is_null() {
        k5_mutex_unlock(&mut (*kcred).lock);
    }
    if !ccache.is_null() {
        crate::krb5_h::krb5_cc_close(context, ccache);
    }
    crate::krb5_h::krb5_free_context(context);
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_store_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    desired_mech: crate::gssapi_h::gss_OID,
    mut overwrite_cred: crate::gssapi_h::OM_uint32,
    mut default_cred: crate::gssapi_h::OM_uint32,
    mut elements_stored: *mut crate::gssapi_h::gss_OID_set,
    mut cred_usage_stored: *mut crate::gssapi_h::gss_cred_usage_t,
) -> crate::gssapi_h::OM_uint32 {
    return krb5_gss_store_cred_into(
        minor_status,
        input_cred_handle,
        cred_usage,
        desired_mech,
        overwrite_cred,
        default_cred,
        0 as crate::gssapi_ext_h::gss_const_key_value_set_t,
        elements_stored,
        cred_usage_stored,
    );
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
/* minor_status */
/* status_value */
/* status_type */
/* mech_type */
/* message_context */
/* status_string */
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
/* Prefix concatenated with Kerberos encryption type */
/* IAKERB */
/*
 * Transfer contents of a krb5_data to a gss_buffer and invalidate the source
 * On unix, this is a simple pointer copy
 * On windows, memory is reallocated and copied.
 */
/* Credential store extensions */
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* cred_store */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_store_cred_into(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    desired_mech: crate::gssapi_h::gss_OID,
    mut overwrite_cred: crate::gssapi_h::OM_uint32,
    mut default_cred: crate::gssapi_h::OM_uint32,
    mut cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
    mut elements_stored: *mut crate::gssapi_h::gss_OID_set,
    mut cred_usage_stored: *mut crate::gssapi_h::gss_cred_usage_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut actual_usage: crate::gssapi_h::gss_cred_usage_t = 0;
    let mut lifetime: crate::gssapi_h::OM_uint32 = 0;
    if input_cred_handle.is_null() {
        return (7u32) << 16i32;
    }
    major_status = (13u32) << 16i32;
    if cred_usage == 2i32 {
        *minor_status = -(2045022958 as isize) as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    } else {
        if cred_usage != 1i32 && cred_usage != 0i32 {
            *minor_status = -(2045022969 as isize) as crate::gssapi_h::OM_uint32;
            return (13u32) << 16i32;
        }
    }
    major_status = crate::src::krb5::inq_cred::krb5_gss_inquire_cred(
        minor_status,
        input_cred_handle,
        0 as *mut crate::gssapi_h::gss_name_t,
        &mut lifetime,
        &mut actual_usage,
        elements_stored,
    );
    if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return major_status;
    }
    if lifetime == 0u32 {
        return (11u32) << 16i32;
    }
    if actual_usage != 1i32 && actual_usage != 0i32 {
        *minor_status = -(2045022958 as isize) as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    major_status = copy_initiator_creds(
        minor_status,
        input_cred_handle,
        desired_mech,
        overwrite_cred,
        default_cred,
        cred_store,
    );
    if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return major_status;
    }
    if !cred_usage_stored.is_null() {
        *cred_usage_stored = 1i32
    }
    return 0u32;
}
