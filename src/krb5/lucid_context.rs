use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::pthread_mutex_t;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_authdata_context;
pub use crate::k5_int_h::_krb5_authdata_context_module;
pub use crate::k5_int_h::_krb5_context;
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::derived_key;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_key_st;
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
pub use crate::k5_thread_h::k5_mutex_t;
pub use crate::k5_thread_h::k5_os_mutex;
pub use crate::krb5_h::_krb5_address;
pub use crate::krb5_h::_krb5_ap_req;
pub use crate::krb5_h::_krb5_auth_context;
pub use crate::krb5_h::_krb5_authdata;
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
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_pointer;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::profile_h::profile_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::uint8_t;

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
pub use crate::gssapiP_generic_h::g_seqnum_state;
pub use crate::gssapiP_generic_h::g_seqnum_state_st;
pub use crate::gssapiP_krb5_h::_krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_id_t;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub use crate::gssapi_ext_h::gss_buffer_set_t;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::gssapi_krb5_h::gss_krb5_cfx_keydata;
pub use crate::gssapi_krb5_h::gss_krb5_cfx_keydata_t;
pub use crate::gssapi_krb5_h::gss_krb5_lucid_context_v1;
pub use crate::gssapi_krb5_h::gss_krb5_lucid_context_v1_t;
pub use crate::gssapi_krb5_h::gss_krb5_lucid_context_version;
pub use crate::gssapi_krb5_h::gss_krb5_lucid_context_version_t;
pub use crate::gssapi_krb5_h::gss_krb5_lucid_key;
pub use crate::gssapi_krb5_h::gss_krb5_lucid_key_t;
pub use crate::gssapi_krb5_h::gss_krb5_rfc1964_keydata;
pub use crate::gssapi_krb5_h::gss_krb5_rfc1964_keydata_t;
pub use crate::src::generic::oid_ops::generic_gss_oid_decompose;
pub use crate::src::generic::util_buffer_set::generic_gss_add_buffer_set_member;

/*
 * Exported routines
 */
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_export_lucid_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    mut data_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut kret = 0i32;
    let mut retval: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = context_handle as crate::gssapiP_krb5_h::krb5_gss_ctx_id_t;
    let mut lctx = 0 as *mut libc::c_void;
    let mut version = 0i32;
    let mut rep = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    /* Assume failure */
    retval = (13u32) << 16i32;
    *minor_status = 0u32;
    *data_set = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    if (*ctx).terminated() as i32 != 0 || (*ctx).established() == 0 {
        *minor_status = 39756039u32;
        return (8u32) << 16i32;
    }
    retval = crate::src::generic::oid_ops::generic_gss_oid_decompose(
        minor_status,
        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x06\x00" as *const u8 as *const i8,
        11usize,
        desired_object,
        &mut version,
    );
    if retval & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return retval;
    }
    /* Externalize a structure of the right version */
    match version {
        1 => kret = make_external_lucid_ctx_v1(ctx, version, &mut lctx),
        _ => kret = 39756046i32,
    }
    if !(kret != 0) {
        rep.value = &mut lctx as *mut *mut libc::c_void as *mut libc::c_void;
        rep.length = ::std::mem::size_of::<*mut libc::c_void>();
        retval = crate::src::generic::util_buffer_set::generic_gss_add_buffer_set_member(
            minor_status,
            &mut rep,
            data_set,
        );
        (retval & ((0o377u32) << 24i32 | (0o377u32) << 16i32)) != 0;
    }
    if *minor_status == 0u32 {
        *minor_status = kret as crate::gssapi_h::OM_uint32
    }
    return retval;
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
/*
 * Frees the storage associated with an
 * exported lucid context structure.
 */
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_free_lucid_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    _desired_mech: crate::gssapi_h::gss_OID,
    _desired_object: crate::gssapi_h::gss_OID,
    mut value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut retval: crate::gssapi_h::OM_uint32 = 0;
    let mut kret = 0i32;
    let mut version: i32 = 0;
    let mut kctx = 0 as *mut libc::c_void;
    /* Assume failure */
    retval = (13u32) << 16i32;
    *minor_status = 0u32;
    kctx = (*value).value;
    if kctx.is_null() {
        kret = 22i32
    } else {
        /* Determine version and call correct free routine */
        version =
            (*(kctx as *mut crate::gssapi_krb5_h::gss_krb5_lucid_context_version_t)).version as i32;
        match version {
            1 => {
                free_external_lucid_ctx_v1(
                    kctx as *mut crate::gssapi_krb5_h::gss_krb5_lucid_context_v1_t,
                );
            }
            _ => kret = 22i32,
        }
        if !(kret != 0) {
            /* Success! */
            *minor_status = 0u32;
            retval = 0u32;
            return retval;
        }
    }
    if *minor_status == 0u32 {
        *minor_status = kret as crate::gssapi_h::OM_uint32
    }
    return retval;
}
/*
 * Local routines
 */

unsafe extern "C" fn make_external_lucid_ctx_v1(
    mut gctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut _version: i32,
    mut out_ptr: *mut *mut libc::c_void,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut lctx = 0 as *mut crate::gssapi_krb5_h::gss_krb5_lucid_context_v1_t;
    let mut bufsize =
        ::std::mem::size_of::<crate::gssapi_krb5_h::gss_krb5_lucid_context_v1_t>() as u32;
    let mut retval: crate::krb5_h::krb5_error_code = 0;
    /* Allocate the structure */
    lctx = crate::stdlib::malloc(bufsize as usize)
        as *mut crate::gssapi_krb5_h::gss_krb5_lucid_context_v1_t;
    if lctx.is_null() {
        retval = 12i32
    } else {
        crate::stdlib::memset(lctx as *mut libc::c_void, 0i32, bufsize as usize);
        (*lctx).version = 1u32;
        (*lctx).initiate = if (*gctx).initiate() as i32 != 0 {
            1i32
        } else {
            0i32
        } as crate::gssapi_h::OM_uint32;
        (*lctx).endtime = (*gctx).krb_times.endtime as crate::gssapi_h::OM_uint32;
        (*lctx).send_seq = (*gctx).seq_send;
        (*lctx).recv_seq = (*gctx).seq_recv;
        (*lctx).protocol = (*gctx).proto as crate::gssapi_h::OM_uint32;
        /* gctx->proto == 0 ==> rfc1964-style key information
        gctx->proto == 1 ==> cfx-style (draft-ietf-krb-wg-gssapi-cfx-07) keys */
        if (*gctx).proto == 0i32 {
            (*lctx).rfc1964_kd.sign_alg = (*gctx).signalg as crate::gssapi_h::OM_uint32;
            (*lctx).rfc1964_kd.seal_alg = (*gctx).sealalg as crate::gssapi_h::OM_uint32;
            /* Copy key */
            retval = copy_keyblock_to_lucid_key(
                &mut (*(*gctx).seq).keyblock,
                &mut (*lctx).rfc1964_kd.ctx_key,
            );
            if retval != 0 {
                current_block = 5031962668085496365;
            } else {
                current_block = 1109700713171191020;
            }
        } else if (*gctx).proto == 1i32 {
            /* Copy keys */
            /* (subkey is always present, either a copy of the kerberos
            session key or a subkey) */
            retval = copy_keyblock_to_lucid_key(
                &mut (*(*gctx).subkey).keyblock,
                &mut (*lctx).cfx_kd.ctx_key,
            );
            if retval != 0 {
                current_block = 5031962668085496365;
            } else if (*gctx).have_acceptor_subkey() != 0 {
                retval = copy_keyblock_to_lucid_key(
                    &mut (*(*gctx).acceptor_subkey).keyblock,
                    &mut (*lctx).cfx_kd.acceptor_subkey,
                );
                if retval != 0 {
                    current_block = 5031962668085496365;
                } else {
                    (*lctx).cfx_kd.have_acceptor_subkey = 1u32;
                    current_block = 1109700713171191020;
                }
            } else {
                current_block = 1109700713171191020;
            }
        } else {
            crate::stdlib::free(lctx as *mut libc::c_void);
            return 22i32;
            /* XXX better error code? */
        }
        match current_block {
            5031962668085496365 => {}
            _ => {
                /* Success! */
                *out_ptr = lctx as *mut libc::c_void;
                return 0i32;
            }
        }
    }
    if !lctx.is_null() {
        free_external_lucid_ctx_v1(lctx);
    }
    return retval;
}
/* Copy the contents of a krb5_keyblock to a gss_krb5_lucid_key_t structure */

unsafe extern "C" fn copy_keyblock_to_lucid_key(
    mut k5key: *mut crate::krb5_h::krb5_keyblock,
    mut lkey: *mut crate::gssapi_krb5_h::gss_krb5_lucid_key_t,
) -> crate::krb5_h::krb5_error_code {
    if k5key.is_null() || (*k5key).contents.is_null() || (*k5key).length == 0u32 {
        return 22i32;
    }
    crate::stdlib::memset(
        lkey as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::gssapi_krb5_h::gss_krb5_lucid_key_t>(),
    );
    /* Allocate storage for the key data */
    (*lkey).data = crate::stdlib::malloc((*k5key).length as usize);
    if (*lkey).data.is_null() {
        return 12i32;
    }
    crate::stdlib::memcpy(
        (*lkey).data,
        (*k5key).contents as *const libc::c_void,
        (*k5key).length as usize,
    );
    (*lkey).length = (*k5key).length;
    (*lkey).type_0 = (*k5key).enctype as crate::gssapi_h::OM_uint32;
    return 0i32;
}
/* Free any storage associated with a gss_krb5_lucid_key_t structure */

unsafe extern "C" fn free_lucid_key_data(mut key: *mut crate::gssapi_krb5_h::gss_krb5_lucid_key_t) {
    if !key.is_null() {
        if !(*key).data.is_null() && (*key).length != 0 {
            crate::stdlib::explicit_bzero((*key).data, (*key).length as crate::stddef_h::size_t);
            crate::stdlib::free((*key).data);
            crate::stdlib::explicit_bzero(
                key as *mut libc::c_void,
                ::std::mem::size_of::<crate::gssapi_krb5_h::gss_krb5_lucid_key_t>(),
            );
        }
    };
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/lucid_context.c */
/*
 * Copyright 2004, 2008 by the Massachusetts Institute of Technology.
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
/* Externalize a "lucid" security context from a krb5_gss_ctx_id_rec
 * structure. */
/*
 * Local routine prototypes
 */
/* Free any storage associated with a gss_krb5_lucid_context_v1 structure */

unsafe extern "C" fn free_external_lucid_ctx_v1(
    mut ctx: *mut crate::gssapi_krb5_h::gss_krb5_lucid_context_v1_t,
) {
    if !ctx.is_null() {
        if (*ctx).protocol == 0u32 {
            free_lucid_key_data(&mut (*ctx).rfc1964_kd.ctx_key);
        }
        if (*ctx).protocol == 1u32 {
            free_lucid_key_data(&mut (*ctx).cfx_kd.ctx_key);
            if (*ctx).cfx_kd.have_acceptor_subkey != 0 {
                free_lucid_key_data(&mut (*ctx).cfx_kd.acceptor_subkey);
            }
        }
        crate::stdlib::free(ctx as *mut libc::c_void);
        ctx = 0 as *mut crate::gssapi_krb5_h::gss_krb5_lucid_context_v1_t
    };
}
