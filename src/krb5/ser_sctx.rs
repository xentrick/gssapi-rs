use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::__uint8_t;

pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::pthread_mutex_t;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_authdata_context;
pub use crate::k5_int_h::_krb5_authdata_context_module;
pub use crate::k5_int_h::_krb5_context;
pub use crate::k5_int_h::_krb5_key_data;
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::_krb5int_access;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::derived_key;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_externalize_auth_context;
pub use crate::k5_int_h::k5_externalize_authdata;
pub use crate::k5_int_h::k5_externalize_authdata_context;
pub use crate::k5_int_h::k5_externalize_context;
pub use crate::k5_int_h::k5_externalize_keyblock;
pub use crate::k5_int_h::k5_externalize_principal;
pub use crate::k5_int_h::k5_internalize_auth_context;
pub use crate::k5_int_h::k5_internalize_authdata;
pub use crate::k5_int_h::k5_internalize_authdata_context;
pub use crate::k5_int_h::k5_internalize_context;
pub use crate::k5_int_h::k5_internalize_keyblock;
pub use crate::k5_int_h::k5_internalize_principal;
pub use crate::k5_int_h::k5_size_auth_context;
pub use crate::k5_int_h::k5_size_authdata;
pub use crate::k5_int_h::k5_size_authdata_context;
pub use crate::k5_int_h::k5_size_context;
pub use crate::k5_int_h::k5_size_keyblock;
pub use crate::k5_int_h::k5_size_principal;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_key_st;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
pub use crate::k5_int_h::krb5_ser_pack_bytes;
pub use crate::k5_int_h::krb5_ser_pack_int32;
pub use crate::k5_int_h::krb5_ser_unpack_bytes;
pub use crate::k5_int_h::krb5_ser_unpack_int32;
pub use crate::k5_int_h::krb5int_access;
pub use crate::k5_int_h::krb5int_accessor;
pub use crate::k5_int_h::ldap_seqof_key_data;
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
pub use crate::krb5_h::_krb5_checksum;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::_krb5_enc_data;
pub use crate::krb5_h::_krb5_enc_tkt_part;
pub use crate::krb5_h::_krb5_kdc_req;
pub use crate::krb5_h::_krb5_keyblock;
pub use crate::krb5_h::_krb5_pa_data;
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
pub use crate::krb5_h::krb5_checksum;
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
pub use crate::krb5_h::krb5_free_keyblock;
pub use crate::krb5_h::krb5_free_principal;
pub use crate::krb5_h::krb5_int16;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_k_create_key;
pub use crate::krb5_h::krb5_k_free_key;
pub use crate::krb5_h::krb5_kdc_req;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_msgtype;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_pa_data;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_preauthtype;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::krb5_h::krb5_ui_2;
pub use crate::profile_h::profile_t;
pub use crate::stdlib::uint16_t;
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
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::k5_int_pkinit_h::_krb5_algorithm_identifier;
pub use crate::k5_int_pkinit_h::_krb5_auth_pack;
pub use crate::k5_int_pkinit_h::_krb5_dh_rep_info;
pub use crate::k5_int_pkinit_h::_krb5_external_principal_identifier;
pub use crate::k5_int_pkinit_h::_krb5_kdc_dh_key_info;
pub use crate::k5_int_pkinit_h::_krb5_pa_pk_as_rep;
pub use crate::k5_int_pkinit_h::_krb5_pa_pk_as_req;
pub use crate::k5_int_pkinit_h::_krb5_pk_authenticator;
pub use crate::k5_int_pkinit_h::_krb5_reply_key_pack;
pub use crate::k5_int_pkinit_h::_krb5_subject_pk_info;
pub use crate::k5_int_pkinit_h::choice_pa_pk_as_rep_UNKNOWN;
pub use crate::k5_int_pkinit_h::choice_pa_pk_as_rep_dhInfo;
pub use crate::k5_int_pkinit_h::choice_pa_pk_as_rep_encKeyPack;
pub use crate::k5_int_pkinit_h::krb5_algorithm_identifier;
pub use crate::k5_int_pkinit_h::krb5_auth_pack;
pub use crate::k5_int_pkinit_h::krb5_dh_rep_info;
pub use crate::k5_int_pkinit_h::krb5_external_principal_identifier;
pub use crate::k5_int_pkinit_h::krb5_kdc_dh_key_info;
pub use crate::k5_int_pkinit_h::krb5_pa_pk_as_rep;
pub use crate::k5_int_pkinit_h::krb5_pa_pk_as_rep_choices;
pub use crate::k5_int_pkinit_h::krb5_pa_pk_as_rep_selection;
pub use crate::k5_int_pkinit_h::krb5_pa_pk_as_req;
pub use crate::k5_int_pkinit_h::krb5_pk_authenticator;
pub use crate::k5_int_pkinit_h::krb5_reply_key_pack;
pub use crate::k5_int_pkinit_h::krb5_subject_pk_info;
pub use crate::src::generic::util_seqstate::gssint_g_seqstate_externalize;
pub use crate::src::generic::util_seqstate::gssint_g_seqstate_free;
pub use crate::src::generic::util_seqstate::gssint_g_seqstate_internalize;
pub use crate::src::generic::util_seqstate::gssint_g_seqstate_size;
pub use crate::src::krb5::naming_exts::kg_init_name;
pub use crate::src::krb5::naming_exts::kg_release_name;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/ser_sctx.c - [De]serialization of security context */
/*
 * Copyright 1995, 2004, 2008 by the Massachusetts Institute of Technology.
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
 * This module contains routines to [de]serialize
 *      krb5_gss_enc_desc and krb5_gss_ctx_id_t.
 * XXX This whole serialization abstraction is unnecessary in a
 * non-messaging environment, which krb5 is.  Someday, this should
 * all get redone without the extra level of indirection. I've done
 * some of this work here, since adding new serializers is an internal
 * krb5 interface, and I won't use those.  There is some more
 * deobfuscation (no longer anonymizing pointers, mostly) which could
 * still be done. --marc
 */

unsafe extern "C" fn kg_oid_externalize(
    mut oid: crate::gssapi_h::gss_OID,
    mut buffer: *mut *mut crate::krb5_h::krb5_octet,
    mut lenremain: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut err: crate::krb5_h::krb5_error_code = 0;
    err = crate::k5_int_h::krb5_ser_pack_int32(
        -(1760647369 as isize) as crate::krb5_h::krb5_int32,
        buffer,
        lenremain,
    );
    if err != 0 {
        return err;
    }
    err = crate::k5_int_h::krb5_ser_pack_int32(
        (*oid).length as crate::krb5_h::krb5_int32,
        buffer,
        lenremain,
    );
    if err != 0 {
        return err;
    }
    err = crate::k5_int_h::krb5_ser_pack_bytes(
        (*oid).elements as *mut crate::krb5_h::krb5_octet,
        (*oid).length as crate::stddef_h::size_t,
        buffer,
        lenremain,
    );
    if err != 0 {
        return err;
    }
    err = crate::k5_int_h::krb5_ser_pack_int32(
        -(1760647369 as isize) as crate::krb5_h::krb5_int32,
        buffer,
        lenremain,
    );
    return err;
}

unsafe extern "C" fn kg_oid_internalize(
    mut argp: *mut crate::gssapi_h::gss_OID,
    mut buffer: *mut *mut crate::krb5_h::krb5_octet,
    mut lenremain: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut oid = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut ibuf: crate::krb5_h::krb5_int32 = 0;
    let mut bp = 0 as *mut crate::krb5_h::krb5_octet;
    let mut remain: crate::stddef_h::size_t = 0;
    bp = *buffer;
    remain = *lenremain;
    /* Read in and check our magic number */
    if crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain) != 0 {
        return 22i32;
    }
    if ibuf as isize != -(1760647369 as isize) {
        return 22i32;
    }
    oid = crate::stdlib::malloc(::std::mem::size_of::<crate::gssapi_h::gss_OID_desc>())
        as crate::gssapi_h::gss_OID;
    if oid.is_null() {
        return 12i32;
    }
    if crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain) != 0 {
        crate::stdlib::free(oid as *mut libc::c_void);
        return 22i32;
    }
    (*oid).length = ibuf as crate::gssapi_h::OM_uint32;
    (*oid).elements = crate::stdlib::malloc(ibuf as crate::stddef_h::size_t);
    if (*oid).elements.is_null() {
        crate::stdlib::free(oid as *mut libc::c_void);
        return 12i32;
    }
    if crate::k5_int_h::krb5_ser_unpack_bytes(
        (*oid).elements as *mut crate::krb5_h::krb5_octet,
        (*oid).length as crate::stddef_h::size_t,
        &mut bp,
        &mut remain,
    ) != 0
    {
        crate::stdlib::free((*oid).elements);
        crate::stdlib::free(oid as *mut libc::c_void);
        return 22i32;
    }
    /* Read in and check our trailing magic number */
    if crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain) != 0 {
        crate::stdlib::free((*oid).elements); /* For the header and trailer */
        crate::stdlib::free(oid as *mut libc::c_void);
        return 22i32;
    }
    if ibuf as isize != -(1760647369 as isize) {
        crate::stdlib::free((*oid).elements);
        crate::stdlib::free(oid as *mut libc::c_void);
        return 22i32;
    }
    *buffer = bp;
    *lenremain = remain;
    *argp = oid;
    return 0i32;
}

unsafe extern "C" fn kg_oid_size(
    mut oid: crate::gssapi_h::gss_OID,
    mut sizep: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut kret: crate::krb5_h::krb5_error_code = 0;
    let mut required: crate::stddef_h::size_t = 0;
    kret = 22i32;
    if !oid.is_null() {
        required = (2usize).wrapping_mul(::std::mem::size_of::<crate::krb5_h::krb5_int32>());
        required = (required).wrapping_add(::std::mem::size_of::<crate::krb5_h::krb5_int32>());
        required = (required).wrapping_add((*oid).length as usize);
        kret = 0i32;
        *sizep = (*sizep).wrapping_add(required)
    }
    return kret;
}

unsafe extern "C" fn kg_seqstate_externalize(
    mut arg: crate::gssapiP_generic_h::g_seqnum_state,
    mut buffer: *mut *mut crate::krb5_h::krb5_octet,
    mut lenremain: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut err: crate::krb5_h::krb5_error_code = 0;
    err = crate::k5_int_h::krb5_ser_pack_int32(
        -(1760647368 as isize) as crate::krb5_h::krb5_int32,
        buffer,
        lenremain,
    );
    if err == 0i32 {
        err = crate::src::generic::util_seqstate::gssint_g_seqstate_externalize(
            arg, buffer, lenremain,
        ) as crate::krb5_h::krb5_error_code
    }
    if err == 0i32 {
        err = crate::k5_int_h::krb5_ser_pack_int32(
            -(1760647368 as isize) as crate::krb5_h::krb5_int32,
            buffer,
            lenremain,
        )
    }
    return err;
}

unsafe extern "C" fn kg_seqstate_internalize(
    mut argp: *mut crate::gssapiP_generic_h::g_seqnum_state,
    mut buffer: *mut *mut crate::krb5_h::krb5_octet,
    mut lenremain: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut ibuf: crate::krb5_h::krb5_int32 = 0;
    let mut bp = 0 as *mut crate::krb5_h::krb5_octet;
    let mut remain: crate::stddef_h::size_t = 0;
    let mut err: crate::krb5_h::krb5_error_code = 0;
    bp = *buffer;
    remain = *lenremain;
    /* Read in and check our magic number */
    if crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain) != 0 {
        return 22i32;
    }
    if ibuf as isize != -(1760647368 as isize) {
        return 22i32;
    }
    err = crate::src::generic::util_seqstate::gssint_g_seqstate_internalize(
        argp,
        &mut bp,
        &mut remain,
    ) as crate::krb5_h::krb5_error_code;
    if err != 0 {
        return err;
    }
    /* Read in and check our trailing magic number */
    if crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain) != 0 {
        crate::src::generic::util_seqstate::gssint_g_seqstate_free(*argp); /* For the header and trailer */
        return 22i32;
    }
    if ibuf as isize != -(1760647368 as isize) {
        crate::src::generic::util_seqstate::gssint_g_seqstate_free(*argp);
        return 22i32;
    }
    *buffer = bp;
    *lenremain = remain;
    return 0i32;
}

unsafe extern "C" fn kg_seqstate_size(
    mut arg: crate::gssapiP_generic_h::g_seqnum_state,
    mut sizep: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut kret: crate::krb5_h::krb5_error_code = 0;
    let mut required: crate::stddef_h::size_t = 0;
    kret = 22i32;
    if !arg.is_null() {
        required = (2usize).wrapping_mul(::std::mem::size_of::<crate::krb5_h::krb5_int32>());
        crate::src::generic::util_seqstate::gssint_g_seqstate_size(arg, &mut required);
        kret = 0i32;
        *sizep = (*sizep).wrapping_add(required)
    }
    return kret;
}
/*
 * Determine the size required for this krb5_gss_ctx_id_rec.
 */
#[no_mangle]

pub unsafe extern "C" fn kg_ctx_size(
    mut kcontext: crate::krb5_h::krb5_context,
    mut ctx: crate::gssapiP_krb5_h::krb5_gss_ctx_id_t,
    mut sizep: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut kret: crate::krb5_h::krb5_error_code = 0;
    let mut required: crate::stddef_h::size_t = 0;
    /*
     * krb5_gss_ctx_id_rec requires:
     *  krb5_int32      for KG_CONTEXT
     *  krb5_int32      for initiate.
     *  krb5_int32      for established.
     *  krb5_int32      for have_acceptor_subkey.
     *  krb5_int32      for seed_init.
     *  krb5_int32      for gss_flags.
     *  sizeof(seed)    for seed
     *  ...             for here
     *  ...             for there
     *  ...             for subkey
     *  krb5_int32      for signalg.
     *  krb5_int32      for cksum_size.
     *  krb5_int32      for sealalg.
     *  ...             for enc
     *  ...             for seq
     *  krb5_int32      for authtime.
     *  krb5_int32      for starttime.
     *  krb5_int32      for endtime.
     *  krb5_int32      for renew_till.
     *  krb5_int32      for flags.
     *  int64_t         for seq_send.
     *  int64_t         for seq_recv.
     *  ...             for seqstate
     *  ...             for auth_context
     *  ...             for mech_used
     *  krb5_int32      for proto
     *  krb5_int32      for cksumtype
     *  ...             for acceptor_subkey
     *  krb5_int32      for acceptor_key_cksumtype
     *  krb5_int32      for cred_rcache
     *  krb5_int32      for number of elements in authdata array
     *  ...             for authdata array
     *  krb5_int32      for trailer.
     */
    kret = 22i32;
    if !ctx.is_null() {
        required = (21usize).wrapping_mul(::std::mem::size_of::<crate::krb5_h::krb5_int32>());
        required = (required)
            .wrapping_add((2usize).wrapping_mul(::std::mem::size_of::<crate::stdlib::int64_t>()));
        required = (required).wrapping_add(::std::mem::size_of::<[u8; 16]>());
        kret = 0i32;
        if kret == 0 && !(*ctx).here.is_null() {
            kret = crate::k5_int_h::k5_size_principal((*(*ctx).here).princ, &mut required)
        }
        if kret == 0 && !(*ctx).there.is_null() {
            kret = crate::k5_int_h::k5_size_principal((*(*ctx).there).princ, &mut required)
        }
        if kret == 0 && !(*ctx).subkey.is_null() {
            kret = crate::k5_int_h::k5_size_keyblock(&mut (*(*ctx).subkey).keyblock, &mut required)
        }
        if kret == 0 && !(*ctx).enc.is_null() {
            kret = crate::k5_int_h::k5_size_keyblock(&mut (*(*ctx).enc).keyblock, &mut required)
        }
        if kret == 0 && !(*ctx).seq.is_null() {
            kret = crate::k5_int_h::k5_size_keyblock(&mut (*(*ctx).seq).keyblock, &mut required)
        }
        if kret == 0 {
            kret = kg_oid_size((*ctx).mech_used, &mut required)
        }
        if kret == 0 && !(*ctx).seqstate.is_null() {
            kret = kg_seqstate_size((*ctx).seqstate, &mut required)
        }
        if kret == 0 {
            kret = crate::k5_int_h::k5_size_context((*ctx).k5_context, &mut required)
        }
        if kret == 0 {
            kret = crate::k5_int_h::k5_size_auth_context((*ctx).auth_context, &mut required)
        }
        if kret == 0 && !(*ctx).acceptor_subkey.is_null() {
            kret = crate::k5_int_h::k5_size_keyblock(
                &mut (*(*ctx).acceptor_subkey).keyblock,
                &mut required,
            )
        }
        if kret == 0 && !(*ctx).authdata.is_null() {
            let mut i: crate::krb5_h::krb5_int32 = 0;
            i = 0i32;
            while kret == 0 && !(*(*ctx).authdata.offset(i as isize)).is_null() {
                kret = crate::k5_int_h::k5_size_authdata(
                    *(*ctx).authdata.offset(i as isize),
                    &mut required,
                );
                i += 1
            }
        }
        if kret == 0 {
            let mut initiator_name = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
            initiator_name = if (*ctx).initiate() as i32 != 0 {
                (*ctx).here
            } else {
                (*ctx).there
            };
            if !initiator_name.is_null() && !(*initiator_name).ad_context.is_null() {
                kret = crate::k5_int_h::k5_size_authdata_context(
                    kcontext,
                    (*initiator_name).ad_context,
                    &mut required,
                )
            }
        }
        *sizep = (*sizep).wrapping_add(required)
    }
    return kret;
}
/*
 * Externalize this krb5_gss_ctx_id_ret.
 */
#[no_mangle]

pub unsafe extern "C" fn kg_ctx_externalize(
    mut kcontext: crate::krb5_h::krb5_context,
    mut ctx: crate::gssapiP_krb5_h::krb5_gss_ctx_id_t,
    mut buffer: *mut *mut crate::krb5_h::krb5_octet,
    mut lenremain: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut kret: crate::krb5_h::krb5_error_code = 0;
    let mut required: crate::stddef_h::size_t = 0;
    let mut bp = 0 as *mut crate::krb5_h::krb5_octet;
    let mut remain: crate::stddef_h::size_t = 0;
    let mut kaccess = crate::k5_int_h::krb5int_access {
        auth_con_get_subkey_enctype: None,
        mandatory_cksumtype: None,
        ser_pack_int64: None,
        ser_unpack_int64: None,
        asn1_ldap_encode_sequence_of_keys: None,
        asn1_ldap_decode_sequence_of_keys: None,
        encode_krb5_auth_pack: None,
        encode_krb5_kdc_dh_key_info: None,
        encode_krb5_pa_pk_as_rep: None,
        encode_krb5_pa_pk_as_req: None,
        encode_krb5_reply_key_pack: None,
        encode_krb5_td_dh_parameters: None,
        encode_krb5_td_trusted_certifiers: None,
        decode_krb5_auth_pack: None,
        decode_krb5_pa_pk_as_req: None,
        decode_krb5_pa_pk_as_rep: None,
        decode_krb5_kdc_dh_key_info: None,
        decode_krb5_principal_name: None,
        decode_krb5_reply_key_pack: None,
        decode_krb5_td_dh_parameters: None,
        decode_krb5_td_trusted_certifiers: None,
        encode_krb5_kdc_req_body: None,
        free_kdc_req: None,
        set_prompt_types: None,
    };
    kret = crate::k5_int_h::krb5int_accessor(
        &mut kaccess,
        ((::std::mem::size_of::<crate::k5_int_h::krb5int_access>() & 0xffffusize
            | ((23i32) << 16i32) as usize) as u32
            & 0xffffffffu32) as crate::krb5_h::krb5_int32,
    );
    if kret != 0 {
        return kret;
    }
    required = 0usize;
    bp = *buffer;
    remain = *lenremain;
    kret = 22i32;
    if !ctx.is_null() {
        kret = 12i32;
        if kg_ctx_size(kcontext, ctx, &mut required) == 0 && required <= remain {
            /* Our identifier */
            crate::k5_int_h::krb5_ser_pack_int32(39756040i32, &mut bp, &mut remain);
            /* Now static data */
            crate::k5_int_h::krb5_ser_pack_int32(
                (*ctx).initiate() as crate::krb5_h::krb5_int32,
                &mut bp,
                &mut remain,
            );
            crate::k5_int_h::krb5_ser_pack_int32(
                (*ctx).established() as crate::krb5_h::krb5_int32,
                &mut bp,
                &mut remain,
            );
            crate::k5_int_h::krb5_ser_pack_int32(
                (*ctx).have_acceptor_subkey() as crate::krb5_h::krb5_int32,
                &mut bp,
                &mut remain,
            );
            crate::k5_int_h::krb5_ser_pack_int32(
                (*ctx).seed_init() as crate::krb5_h::krb5_int32,
                &mut bp,
                &mut remain,
            );
            crate::k5_int_h::krb5_ser_pack_int32(
                (*ctx).gss_flags as crate::krb5_h::krb5_int32,
                &mut bp,
                &mut remain,
            );
            crate::k5_int_h::krb5_ser_pack_bytes(
                (*ctx).seed.as_mut_ptr(),
                ::std::mem::size_of::<[u8; 16]>(),
                &mut bp,
                &mut remain,
            );
            crate::k5_int_h::krb5_ser_pack_int32((*ctx).signalg, &mut bp, &mut remain);
            crate::k5_int_h::krb5_ser_pack_int32(
                (*ctx).cksum_size as crate::krb5_h::krb5_int32,
                &mut bp,
                &mut remain,
            );
            crate::k5_int_h::krb5_ser_pack_int32((*ctx).sealalg, &mut bp, &mut remain);
            crate::k5_int_h::krb5_ser_pack_int32((*ctx).krb_times.authtime, &mut bp, &mut remain);
            crate::k5_int_h::krb5_ser_pack_int32((*ctx).krb_times.starttime, &mut bp, &mut remain);
            crate::k5_int_h::krb5_ser_pack_int32((*ctx).krb_times.endtime, &mut bp, &mut remain);
            crate::k5_int_h::krb5_ser_pack_int32((*ctx).krb_times.renew_till, &mut bp, &mut remain);
            crate::k5_int_h::krb5_ser_pack_int32((*ctx).krb_flags, &mut bp, &mut remain);
            Some(kaccess.ser_pack_int64.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*ctx).seq_send as crate::stdlib::int64_t,
                &mut bp,
                &mut remain,
            );
            Some(kaccess.ser_pack_int64.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*ctx).seq_recv as crate::stdlib::int64_t,
                &mut bp,
                &mut remain,
            );
            /* Now dynamic data */
            kret = 0i32;
            if kret == 0 && !(*ctx).mech_used.is_null() {
                kret = kg_oid_externalize((*ctx).mech_used, &mut bp, &mut remain)
            }
            if kret == 0 && !(*ctx).here.is_null() {
                kret = crate::k5_int_h::k5_externalize_principal(
                    (*(*ctx).here).princ,
                    &mut bp,
                    &mut remain,
                )
            }
            if kret == 0 && !(*ctx).there.is_null() {
                kret = crate::k5_int_h::k5_externalize_principal(
                    (*(*ctx).there).princ,
                    &mut bp,
                    &mut remain,
                )
            }
            if kret == 0 && !(*ctx).subkey.is_null() {
                kret = crate::k5_int_h::k5_externalize_keyblock(
                    &mut (*(*ctx).subkey).keyblock,
                    &mut bp,
                    &mut remain,
                )
            }
            if kret == 0 && !(*ctx).enc.is_null() {
                kret = crate::k5_int_h::k5_externalize_keyblock(
                    &mut (*(*ctx).enc).keyblock,
                    &mut bp,
                    &mut remain,
                )
            }
            if kret == 0 && !(*ctx).seq.is_null() {
                kret = crate::k5_int_h::k5_externalize_keyblock(
                    &mut (*(*ctx).seq).keyblock,
                    &mut bp,
                    &mut remain,
                )
            }
            if kret == 0 && !(*ctx).seqstate.is_null() {
                kret = kg_seqstate_externalize((*ctx).seqstate, &mut bp, &mut remain)
            }
            if kret == 0 {
                kret =
                    crate::k5_int_h::k5_externalize_context((*ctx).k5_context, &mut bp, &mut remain)
            }
            if kret == 0 {
                kret = crate::k5_int_h::k5_externalize_auth_context(
                    (*ctx).auth_context,
                    &mut bp,
                    &mut remain,
                )
            }
            if kret == 0 {
                kret = crate::k5_int_h::krb5_ser_pack_int32((*ctx).proto, &mut bp, &mut remain)
            }
            if kret == 0 {
                kret = crate::k5_int_h::krb5_ser_pack_int32((*ctx).cksumtype, &mut bp, &mut remain)
            }
            if kret == 0 && !(*ctx).acceptor_subkey.is_null() {
                kret = crate::k5_int_h::k5_externalize_keyblock(
                    &mut (*(*ctx).acceptor_subkey).keyblock,
                    &mut bp,
                    &mut remain,
                )
            }
            if kret == 0 {
                kret = crate::k5_int_h::krb5_ser_pack_int32(
                    (*ctx).acceptor_subkey_cksumtype,
                    &mut bp,
                    &mut remain,
                )
            }
            if kret == 0 {
                kret =
                    crate::k5_int_h::krb5_ser_pack_int32((*ctx).cred_rcache, &mut bp, &mut remain)
            }
            if kret == 0 {
                let mut i = 0i32;
                if !(*ctx).authdata.is_null() {
                    while !(*(*ctx).authdata.offset(i as isize)).is_null() {
                        i += 1
                    }
                }
                /* authdata count */
                kret = crate::k5_int_h::krb5_ser_pack_int32(i, &mut bp, &mut remain);
                if kret == 0 && !(*ctx).authdata.is_null() {
                    /* authdata */
                    i = 0i32;
                    while kret == 0 && !(*(*ctx).authdata.offset(i as isize)).is_null() {
                        kret = crate::k5_int_h::k5_externalize_authdata(
                            *(*ctx).authdata.offset(i as isize),
                            &mut bp,
                            &mut remain,
                        );
                        i += 1
                    }
                }
            }
            /* authdata context */
            if kret == 0 {
                let mut initiator_name = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
                initiator_name = if (*ctx).initiate() as i32 != 0 {
                    (*ctx).here
                } else {
                    (*ctx).there
                };
                if !initiator_name.is_null() && !(*initiator_name).ad_context.is_null() {
                    kret = crate::k5_int_h::k5_externalize_authdata_context(
                        kcontext,
                        (*initiator_name).ad_context,
                        &mut bp,
                        &mut remain,
                    )
                }
            }
            /* trailer */
            if kret == 0 {
                kret = crate::k5_int_h::krb5_ser_pack_int32(39756040i32, &mut bp, &mut remain)
            }
            if kret == 0 {
                *buffer = bp;
                *lenremain = remain
            }
        }
    }
    return kret;
}
/* Internalize a keyblock and convert it to a key. */

unsafe extern "C" fn intern_key(
    mut key: *mut crate::krb5_h::krb5_key,
    mut bp: *mut *mut crate::krb5_h::krb5_octet,
    mut sp: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut keyblock = 0 as *mut crate::krb5_h::krb5_keyblock;
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    ret = crate::k5_int_h::k5_internalize_keyblock(&mut keyblock, bp, sp);
    if ret != 0i32 {
        return ret;
    }
    ret = crate::krb5_h::krb5_k_create_key(0 as crate::krb5_h::krb5_context, keyblock, key);
    crate::krb5_h::krb5_free_keyblock(0 as crate::krb5_h::krb5_context, keyblock);
    return ret;
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
/*
 * Internalize this krb5_gss_ctx_id_t.
 */
#[no_mangle]

pub unsafe extern "C" fn kg_ctx_internalize(
    mut kcontext: crate::krb5_h::krb5_context,
    mut argp: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_t,
    mut buffer: *mut *mut crate::krb5_h::krb5_octet,
    mut lenremain: *mut crate::stddef_h::size_t,
) -> crate::krb5_h::krb5_error_code {
    let mut kret: crate::krb5_h::krb5_error_code = 0;
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut ibuf: crate::krb5_h::krb5_int32 = 0;
    let mut bp = 0 as *mut crate::krb5_h::krb5_octet;
    let mut remain: crate::stddef_h::size_t = 0;
    let mut kaccess = crate::k5_int_h::krb5int_access {
        auth_con_get_subkey_enctype: None,
        mandatory_cksumtype: None,
        ser_pack_int64: None,
        ser_unpack_int64: None,
        asn1_ldap_encode_sequence_of_keys: None,
        asn1_ldap_decode_sequence_of_keys: None,
        encode_krb5_auth_pack: None,
        encode_krb5_kdc_dh_key_info: None,
        encode_krb5_pa_pk_as_rep: None,
        encode_krb5_pa_pk_as_req: None,
        encode_krb5_reply_key_pack: None,
        encode_krb5_td_dh_parameters: None,
        encode_krb5_td_trusted_certifiers: None,
        decode_krb5_auth_pack: None,
        decode_krb5_pa_pk_as_req: None,
        decode_krb5_pa_pk_as_rep: None,
        decode_krb5_kdc_dh_key_info: None,
        decode_krb5_principal_name: None,
        decode_krb5_reply_key_pack: None,
        decode_krb5_td_dh_parameters: None,
        decode_krb5_td_trusted_certifiers: None,
        encode_krb5_kdc_req_body: None,
        free_kdc_req: None,
        set_prompt_types: None,
    };
    let mut princ = 0 as *mut crate::krb5_h::krb5_principal_data;
    kret = crate::k5_int_h::krb5int_accessor(
        &mut kaccess,
        ((::std::mem::size_of::<crate::k5_int_h::krb5int_access>() & 0xffffusize
            | ((23i32) << 16i32) as usize) as u32
            & 0xffffffffu32) as crate::krb5_h::krb5_int32,
    );
    if kret != 0 {
        return kret;
    }
    bp = *buffer;
    remain = *lenremain;
    kret = 22i32;
    princ = 0 as crate::krb5_h::krb5_principal;
    /* Read our magic number */
    if crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain) != 0 {
        ibuf = 0i32
    }
    if ibuf as isize == 39756040 as isize {
        kret = 12i32;
        /* Get a context */
        if remain
            >= (17usize)
                .wrapping_mul(::std::mem::size_of::<crate::krb5_h::krb5_int32>())
                .wrapping_add(
                    (2usize).wrapping_mul(::std::mem::size_of::<crate::stdlib::int64_t>()),
                )
                .wrapping_add(::std::mem::size_of::<[u8; 16]>())
            && {
                ctx = crate::stdlib::malloc(::std::mem::size_of::<
                    crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
                >()) as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
                !ctx.is_null()
            }
        {
            crate::stdlib::memset(
                ctx as *mut libc::c_void,
                0i32,
                ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec>(),
            );
            (*ctx).magic = ibuf;
            /* Get static data */
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).set_initiate(ibuf as u32);
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).set_established(ibuf as u32);
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).set_have_acceptor_subkey(ibuf as u32);
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).set_seed_init(ibuf as u32);
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).gss_flags = ibuf as crate::gssapi_h::OM_uint32;
            crate::k5_int_h::krb5_ser_unpack_bytes(
                (*ctx).seed.as_mut_ptr(),
                ::std::mem::size_of::<[u8; 16]>(),
                &mut bp,
                &mut remain,
            );
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).signalg = ibuf;
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).cksum_size = ibuf as crate::stddef_h::size_t;
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).sealalg = ibuf;
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).krb_times.authtime = ibuf;
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).krb_times.starttime = ibuf;
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).krb_times.endtime = ibuf;
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).krb_times.renew_till = ibuf;
            crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
            (*ctx).krb_flags = ibuf;
            Some(kaccess.ser_unpack_int64.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                &mut (*ctx).seq_send as *mut crate::stdlib::uint64_t as *mut crate::stdlib::int64_t,
                &mut bp,
                &mut remain,
            );
            kret = Some(kaccess.ser_unpack_int64.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                &mut (*ctx).seq_recv as *mut crate::stdlib::uint64_t as *mut crate::stdlib::int64_t,
                &mut bp,
                &mut remain,
            );
            if kret != 0 {
                crate::stdlib::free(ctx as *mut libc::c_void);
                return kret;
            }
            let mut tmp = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
            kret = kg_oid_internalize(&mut tmp, &mut bp, &mut remain);
            if kret == 0i32 {
                (*ctx).mech_used = tmp
            } else if kret == 22i32 {
                kret = 0i32
            }
            /* Now get substructure data */
            kret = crate::k5_int_h::k5_internalize_principal(&mut princ, &mut bp, &mut remain);
            if kret == 0i32 {
                kret = crate::src::krb5::naming_exts::kg_init_name(
                    kcontext,
                    princ,
                    0 as *mut i8,
                    0 as *mut i8,
                    0 as crate::k5_int_h::krb5_authdata_context,
                    0x1i32,
                    &mut (*ctx).here,
                );
                if kret != 0 {
                    crate::krb5_h::krb5_free_principal(kcontext, princ);
                }
            } else if kret == 22i32 {
                kret = 0i32
            }
            if kret == 0 {
                kret = crate::k5_int_h::k5_internalize_principal(&mut princ, &mut bp, &mut remain);
                if kret == 0i32 {
                    kret = crate::src::krb5::naming_exts::kg_init_name(
                        kcontext,
                        princ,
                        0 as *mut i8,
                        0 as *mut i8,
                        0 as crate::k5_int_h::krb5_authdata_context,
                        0x1i32,
                        &mut (*ctx).there,
                    );
                    if kret != 0 {
                        crate::krb5_h::krb5_free_principal(kcontext, princ);
                    }
                } else if kret == 22i32 {
                    kret = 0i32
                }
            }
            if kret == 0 && {
                kret = intern_key(&mut (*ctx).subkey, &mut bp, &mut remain);
                (kret) != 0
            } {
                if kret == 22i32 {
                    kret = 0i32
                }
            }
            if kret == 0 && {
                kret = intern_key(&mut (*ctx).enc, &mut bp, &mut remain);
                (kret) != 0
            } {
                if kret == 22i32 {
                    kret = 0i32
                }
            }
            if kret == 0 && {
                kret = intern_key(&mut (*ctx).seq, &mut bp, &mut remain);
                (kret) != 0
            } {
                if kret == 22i32 {
                    kret = 0i32
                }
            }
            if kret == 0 {
                kret = kg_seqstate_internalize(&mut (*ctx).seqstate, &mut bp, &mut remain);
                if kret == 22i32 {
                    kret = 0i32
                }
            }
            if kret == 0 {
                kret = crate::k5_int_h::k5_internalize_context(
                    &mut (*ctx).k5_context,
                    &mut bp,
                    &mut remain,
                )
            }
            if kret == 0 {
                kret = crate::k5_int_h::k5_internalize_auth_context(
                    &mut (*ctx).auth_context,
                    &mut bp,
                    &mut remain,
                )
            }
            if kret == 0 {
                kret = crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain)
            }
            (*ctx).proto = ibuf;
            if kret == 0 {
                kret = crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain)
            }
            (*ctx).cksumtype = ibuf;
            if kret == 0 && {
                kret = intern_key(&mut (*ctx).acceptor_subkey, &mut bp, &mut remain);
                (kret) != 0
            } {
                if kret == 22i32 {
                    kret = 0i32
                }
            }
            if kret == 0 {
                kret = crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain)
            }
            (*ctx).acceptor_subkey_cksumtype = ibuf;
            if kret == 0 {
                kret = crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain)
            }
            (*ctx).cred_rcache = ibuf;
            /* authdata */
            if kret == 0 {
                kret = crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain)
            }
            if kret == 0 {
                let mut nadata = ibuf;
                let mut i: crate::krb5_h::krb5_int32 = 0;
                if nadata > 0i32 {
                    (*ctx).authdata = crate::stdlib::calloc(
                        (nadata as crate::stddef_h::size_t).wrapping_add(1usize),
                        ::std::mem::size_of::<*mut crate::krb5_h::krb5_authdata>(),
                    )
                        as *mut *mut crate::krb5_h::krb5_authdata;
                    if (*ctx).authdata.is_null() {
                        kret = 12i32
                    } else {
                        i = 0i32;
                        while kret == 0 && i < nadata {
                            kret = crate::k5_int_h::k5_internalize_authdata(
                                &mut *(*ctx).authdata.offset(i as isize),
                                &mut bp,
                                &mut remain,
                            );
                            i += 1
                        }
                    }
                }
            }
            /* authdata context */
            if kret == 0 {
                let mut initiator_name = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
                initiator_name = if (*ctx).initiate() as i32 != 0 {
                    (*ctx).here
                } else {
                    (*ctx).there
                };
                if initiator_name.is_null() {
                    kret = 22i32
                } else {
                    kret = crate::k5_int_h::k5_internalize_authdata_context(
                        kcontext,
                        &mut (*initiator_name).ad_context,
                        &mut bp,
                        &mut remain,
                    );
                    if kret == 22i32 {
                        kret = 0i32
                    }
                }
            }
            /* Get trailer */
            if kret == 0 {
                kret = crate::k5_int_h::krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain)
            }
            if kret == 0 && ibuf as isize != 39756040 as isize {
                kret = 22i32
            }
            if kret == 0 {
                *buffer = bp;
                *lenremain = remain;
                *argp = ctx
            } else {
                if !(*ctx).seq.is_null() {
                    crate::krb5_h::krb5_k_free_key(kcontext, (*ctx).seq);
                }
                if !(*ctx).enc.is_null() {
                    crate::krb5_h::krb5_k_free_key(kcontext, (*ctx).enc);
                }
                if !(*ctx).subkey.is_null() {
                    crate::krb5_h::krb5_k_free_key(kcontext, (*ctx).subkey);
                }
                if !(*ctx).there.is_null() {
                    crate::src::krb5::naming_exts::kg_release_name(kcontext, &mut (*ctx).there);
                }
                if !(*ctx).here.is_null() {
                    crate::src::krb5::naming_exts::kg_release_name(kcontext, &mut (*ctx).here);
                }
                crate::stdlib::free(ctx as *mut libc::c_void);
            }
        }
    }
    return kret;
}
