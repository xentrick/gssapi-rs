use ::libc;

pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* To the extent possible under law, Painless Security, LLC has waived
     * all copyright and related or neighboring rights to GSS-API Memory
     * Management Header. This work is published from: United States.
     */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]

    pub unsafe extern "C" fn gssalloc_free(mut value: *mut libc::c_void) {
        crate::stdlib::free(value);
    }
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
    /* not _WIN32 or DEBUG_GSSALLOC */
    #[inline]

    pub unsafe extern "C" fn gssalloc_strdup(mut str: *const i8) -> *mut i8 {
        let mut size = crate::stdlib::strlen(str).wrapping_add(1usize);
        let mut copy = gssalloc_malloc(size) as *mut i8;
        if !copy.is_null() {
            crate::stdlib::memcpy(copy as *mut libc::c_void, str as *const libc::c_void, size);
            *copy.offset(size.wrapping_sub(1usize) as isize) = '\u{0}' as i8
        }
        return copy;
    }
}

/* _GSSAPIP_GENERIC_H_ */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::ssize_t;

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
pub use crate::k5_thread_h::krb5int_key_register;
pub use crate::k5_thread_h::krb5int_setspecific;
pub use crate::k5_thread_h::K5_KEY_COM_ERR;
pub use crate::k5_thread_h::K5_KEY_GSS_KRB5_CCACHE_NAME;
pub use crate::k5_thread_h::K5_KEY_GSS_KRB5_ERROR_MESSAGE;
pub use crate::k5_thread_h::K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME;
pub use crate::k5_thread_h::K5_KEY_GSS_SPNEGO_STATUS;
pub use crate::k5_thread_h::K5_KEY_MAX;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::_krb5_keyblock;
pub use crate::krb5_h::_krb5_trace_info;
pub use crate::krb5_h::_profile_t;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::profile_h::profile_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;

pub use crate::com_err_h::errcode_t;
pub use crate::com_err_h::error_message;
pub use crate::gssapiP_negoex_h::auth_scheme;
pub use crate::gssapiP_negoex_h::conversation_id;
pub use crate::gssapiP_negoex_h::negoex_auth_mech;
pub use crate::gssapiP_negoex_h::C2RustUnnamed_1;
pub use crate::gssapiP_spnego_h::negoex_mech_list;
pub use crate::gssapiP_spnego_h::send_token_flag;
pub use crate::gssapiP_spnego_h::spnego_ctx_st;
pub use crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
pub use crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
pub use crate::gssapiP_spnego_h::spnego_token_t;
pub use crate::gssapiP_spnego_h::C2RustUnnamed_3;
pub use crate::gssapiP_spnego_h::CHECK_MIC;
pub use crate::gssapiP_spnego_h::CONT_TOKEN_SEND;
pub use crate::gssapiP_spnego_h::ERROR_TOKEN_SEND;
pub use crate::gssapiP_spnego_h::INIT_TOKEN_SEND;
pub use crate::gssapiP_spnego_h::NO_TOKEN_SEND;
pub use crate::gssapi_ext_h::gss_any;
pub use crate::gssapi_ext_h::gss_any_t;
pub use crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub use crate::gssapi_ext_h::gss_buffer_set_t;
pub use crate::gssapi_ext_h::gss_const_key_value_set_t;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
pub use crate::gssapi_ext_h::gss_key_value_element_desc;
pub use crate::gssapi_ext_h::gss_key_value_element_struct;
pub use crate::gssapi_ext_h::gss_key_value_set_desc;
pub use crate::gssapi_ext_h::gss_key_value_set_struct;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_channel_bindings_struct;
pub use crate::gssapi_h::gss_channel_bindings_t;
pub use crate::gssapi_h::gss_const_OID;
pub use crate::gssapi_h::gss_const_OID_set;
pub use crate::gssapi_h::gss_const_buffer_t;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::k5_buf_h::k5buf;
pub use crate::k5_buf_h::k5buftype;
pub use crate::k5_buf_h::K5BUF_DYNAMIC;
pub use crate::k5_buf_h::K5BUF_DYNAMIC_ZAP;
pub use crate::k5_buf_h::K5BUF_ERROR;
pub use crate::k5_buf_h::K5BUF_FIXED;
pub use crate::mglueP_h::gss_config;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_mech_config;
pub use crate::mglueP_h::gss_mech_info;
pub use crate::mglueP_h::gss_mechanism;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_DEPRECATED;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_ITOK_FRAMED;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_MECH_NEGO;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_NEGOEX_AND_SPNEGO;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_NOT_DFLT_MECH;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_HOSTBASED_SERVICE;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_MACHINE_UID_NAME;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_STRING_UID_NAME;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_USER_NAME;
pub use crate::src::mechglue::g_accept_sec_context::gss_accept_sec_context;
pub use crate::src::mechglue::g_acquire_cred::gss_acquire_cred_from;
pub use crate::src::mechglue::g_acquire_cred_imp_name::gss_acquire_cred_impersonate_name;
pub use crate::src::mechglue::g_acquire_cred_with_pw::gss_acquire_cred_with_password;
pub use crate::src::mechglue::g_buffer_set::gss_release_buffer_set;
pub use crate::src::mechglue::g_compare_name::gss_compare_name;
pub use crate::src::mechglue::g_complete_auth_token::gss_complete_auth_token;
pub use crate::src::mechglue::g_context_time::gss_context_time;
pub use crate::src::mechglue::g_del_name_attr::gss_delete_name_attribute;
pub use crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context;
pub use crate::src::mechglue::g_dsp_name::gss_display_name;
pub use crate::src::mechglue::g_dsp_name_ext::gss_display_name_ext;
pub use crate::src::mechglue::g_dsp_status::gss_display_status;
pub use crate::src::mechglue::g_dup_name::gss_duplicate_name;
pub use crate::src::mechglue::g_exp_sec_context::gss_export_sec_context;
pub use crate::src::mechglue::g_export_cred::gss_export_cred;
pub use crate::src::mechglue::g_export_name_comp::gss_export_name_composite;
pub use crate::src::mechglue::g_get_name_attr::gss_get_name_attribute;
pub use crate::src::mechglue::g_glue::gssint_der_length_size;
pub use crate::src::mechglue::g_glue::gssint_get_der_length;
pub use crate::src::mechglue::g_glue::gssint_get_mech_type;
pub use crate::src::mechglue::g_glue::gssint_put_der_length;
pub use crate::src::mechglue::g_imp_cred::gss_import_cred;
pub use crate::src::mechglue::g_imp_name::gss_import_name;
pub use crate::src::mechglue::g_imp_sec_context::gss_import_sec_context;
pub use crate::src::mechglue::g_init_sec_context::gss_init_sec_context;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_register_mechinfo;
pub use crate::src::mechglue::g_inq_context::gss_inquire_context;
pub use crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid;
pub use crate::src::mechglue::g_inq_cred::gss_inquire_cred;
pub use crate::src::mechglue::g_inq_cred::gss_inquire_cred_by_mech;
pub use crate::src::mechglue::g_inq_cred_oid::gss_inquire_cred_by_oid;
pub use crate::src::mechglue::g_inq_name::gss_inquire_name;
pub use crate::src::mechglue::g_map_name_to_any::gss_map_name_to_any;
pub use crate::src::mechglue::g_mechattr::gss_indicate_mechs_by_attrs;
pub use crate::src::mechglue::g_mechattr::gss_inquire_attrs_for_mech;
pub use crate::src::mechglue::g_negoex::gssspi_query_mechanism_info;
pub use crate::src::mechglue::g_oid_ops::gss_add_oid_set_member;
pub use crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set;
pub use crate::src::mechglue::g_oid_ops::gss_oid_equal;
pub use crate::src::mechglue::g_oid_ops::gss_test_oid_set_member;
pub use crate::src::mechglue::g_oid_ops::gssint_copy_oid_set;
pub use crate::src::mechglue::g_prf::gss_pseudo_random;
pub use crate::src::mechglue::g_process_context::gss_process_context_token;
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;
pub use crate::src::mechglue::g_rel_cred::gss_release_cred;
pub use crate::src::mechglue::g_rel_name::gss_release_name;
pub use crate::src::mechglue::g_rel_name_mapping::gss_release_any_name_mapping;
pub use crate::src::mechglue::g_rel_oid_set::gss_release_oid_set;
pub use crate::src::mechglue::g_seal::gss_wrap;
pub use crate::src::mechglue::g_seal::gss_wrap_size_limit;
pub use crate::src::mechglue::g_set_context_option::gss_set_sec_context_option;
pub use crate::src::mechglue::g_set_cred_option::gss_set_cred_option;
pub use crate::src::mechglue::g_set_name_attr::gss_set_name_attribute;
pub use crate::src::mechglue::g_set_neg_mechs::gss_set_neg_mechs;
pub use crate::src::mechglue::g_sign::gss_get_mic;
pub use crate::src::mechglue::g_unseal::gss_unwrap;
pub use crate::src::mechglue::g_unwrap_aead::gss_unwrap_aead;
pub use crate::src::mechglue::g_unwrap_iov::gss_unwrap_iov;
pub use crate::src::mechglue::g_unwrap_iov::gss_verify_mic_iov;
pub use crate::src::mechglue::g_verify::gss_verify_mic;
pub use crate::src::mechglue::g_wrap_aead::gss_wrap_aead;
pub use crate::src::mechglue::g_wrap_iov::gss_get_mic_iov;
pub use crate::src::mechglue::g_wrap_iov::gss_get_mic_iov_length;
pub use crate::src::mechglue::g_wrap_iov::gss_wrap_iov;
pub use crate::src::mechglue::g_wrap_iov::gss_wrap_iov_length;
pub use crate::src::spnego::negoex_ctx::negoex_accept;
pub use crate::src::spnego::negoex_ctx::negoex_init;
pub use crate::src::spnego::negoex_util::negoex_add_auth_mech;
pub use crate::src::spnego::negoex_util::negoex_release_context;

pub use crate::src::spnego::spnego_mech::gssapi_alloc_h::gssalloc_free;
pub use crate::src::spnego::spnego_mech::gssapi_alloc_h::gssalloc_malloc;
pub use crate::src::spnego::spnego_mech::gssapi_alloc_h::gssalloc_strdup;
/*
 * Copyright (C) 2006,2008 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 *
 * A module that implements the spnego security mechanism.
 * It is used to negotiate the security mechanism between
 * peers using the GSS-API.  SPNEGO is specified in RFC 4178.
 *
 */
/*
 * Copyright (c) 2006-2008, Novell, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *   * The copyright holder's name is not used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
/* #pragma ident	"@(#)spnego_mech.c	1.7	04/09/28 SMI" */

pub type gss_OID_const = *const crate::gssapi_h::gss_OID_desc;
/*  LEAN_CLIENT */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_12 {
    pub status: crate::gssapi_h::OM_uint32,
    pub msg: *const i8,
}
/* SPNEGO oid structure */

static mut spnego_oids: [crate::gssapi_h::gss_OID_desc; 1] = [{
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 6u32,
        elements: b"+\x06\x01\x05\x05\x02\x00" as *const u8 as *mut libc::c_void,
    };
    init
}];
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_mech_spnego: *const crate::gssapi_h::gss_OID_desc =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers

static mut spnego_oidsets: [crate::gssapi_h::gss_OID_set_desc; 1] =
    [crate::gssapi_h::gss_OID_set_desc {
        count: 0,
        elements: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
    }; 1];
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_mech_set_spnego: *const crate::gssapi_h::gss_OID_set_desc =
    0 as *const crate::gssapi_h::gss_OID_set_desc;

static mut negoex_mech: crate::gssapi_h::gss_OID_desc = {
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 10u32,
        elements: b"+\x06\x01\x04\x01\x827\x02\x02\x1e\x00" as *const u8 as *mut libc::c_void,
    };
    init
};
/*
 * The Mech OID for SPNEGO:
 * { iso(1) org(3) dod(6) internet(1) security(5)
 *  mechanism(5) spnego(2) }
 */

static mut spnego_mechanism: crate::mglueP_h::gss_config = {
    {
        let mut init = crate::mglueP_h::gss_config {
            mech_type: {
                let mut init = crate::gssapi_h::gss_OID_desc_struct {
                    length: 6u32,
                    elements: b"+\x06\x01\x05\x05\x02\x00" as *const u8 as *mut libc::c_void,
                };
                init
            },
            context: 0 as *mut libc::c_void,
            gss_acquire_cred: Some(
                spnego_gss_acquire_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID_set,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_release_cred: Some(
                spnego_gss_release_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_init_sec_context: Some(
                spnego_gss_init_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_channel_bindings_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_accept_sec_context: Some(
                spnego_gss_accept_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_channel_bindings_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_process_context_token: None,
            gss_delete_sec_context: Some(
                spnego_gss_delete_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_context_time: Some(
                spnego_gss_context_time
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_get_mic: Some(
                spnego_gss_get_mic
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_qop_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_verify_mic: Some(
                spnego_gss_verify_mic
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_qop_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_wrap: Some(
                spnego_gss_wrap
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_unwrap: Some(
                spnego_gss_unwrap
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_h::gss_qop_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_display_status: Some(
                spnego_gss_display_status
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::OM_uint32,
                        _: i32,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_indicate_mechs: None,
            gss_compare_name: Some(
                spnego_gss_compare_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_name_t,
                        _: *mut i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_display_name: Some(
                spnego_gss_display_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_OID,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_import_name: Some(
                spnego_gss_import_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::gss_name_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_release_name: Some(
                spnego_gss_release_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_name_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_cred: Some(
                spnego_gss_inquire_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut i32,
                        _: *mut crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_add_cred: None,
            gss_export_sec_context: Some(
                spnego_gss_export_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_import_sec_context: Some(
                spnego_gss_import_sec_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_cred_by_mech: None,
            gss_inquire_names_for_mech: Some(
                spnego_gss_inquire_names_for_mech
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_context: Some(
                spnego_gss_inquire_context
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut i32,
                        _: *mut i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_internal_release_oid: None,
            gss_wrap_size_limit: Some(
                spnego_gss_wrap_size_limit
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_localname: None,
            gssspi_authorize_localname: None,
            gss_export_name: None,
            gss_duplicate_name: Some(
                spnego_gss_duplicate_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: *mut crate::gssapi_h::gss_name_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_store_cred: None,
            gss_inquire_sec_context_by_oid: Some(
                spnego_gss_inquire_sec_context_by_oid
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_cred_by_oid: Some(
                spnego_gss_inquire_cred_by_oid
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_set_sec_context_option: Some(
                spnego_gss_set_sec_context_option
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_set_cred_option: Some(
                spnego_gss_set_cred_option
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_mech_invoke: None,
            gss_wrap_aead: Some(
                spnego_gss_wrap_aead
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_unwrap_aead: Some(
                spnego_gss_unwrap_aead
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_h::gss_qop_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_wrap_iov: Some(
                spnego_gss_wrap_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_unwrap_iov: Some(
                spnego_gss_unwrap_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_wrap_iov_length: Some(
                spnego_gss_wrap_iov_length
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_complete_auth_token: Some(
                spnego_gss_complete_auth_token
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_acquire_cred_impersonate_name: Some(
                spnego_gss_acquire_cred_impersonate_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID_set,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_add_cred_impersonate_name: None,
            gss_display_name_ext: Some(
                spnego_gss_display_name_ext
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_name: Some(
                spnego_gss_inquire_name
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: *mut i32,
                        _: *mut crate::gssapi_h::gss_OID,
                        _: *mut crate::gssapi_ext_h::gss_buffer_set_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_get_name_attribute: Some(
                spnego_gss_get_name_attribute
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                        _: *mut i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_set_name_attribute: Some(
                spnego_gss_set_name_attribute
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_delete_name_attribute: Some(
                spnego_gss_delete_name_attribute
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_export_name_composite: Some(
                spnego_gss_export_name_composite
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_map_name_to_any: Some(
                spnego_gss_map_name_to_any
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_ext_h::gss_any_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_release_any_name_mapping: Some(
                spnego_gss_release_any_name_mapping
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_ext_h::gss_any_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_pseudo_random: Some(
                spnego_gss_pseudo_random
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: i32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::stdlib::ssize_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_set_neg_mechs: Some(
                spnego_gss_set_neg_mechs
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_saslname_for_mech: Some(
                spnego_gss_inquire_saslname_for_mech
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_mech_for_saslname: Some(
                spnego_gss_inquire_mech_for_saslname
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_OID,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_inquire_attrs_for_mech: Some(
                spnego_gss_inquire_attrs_for_mech
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_const_OID,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::gss_OID_set,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_acquire_cred_from: Some(
                spnego_gss_acquire_cred_from
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID_set,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: crate::gssapi_ext_h::gss_const_key_value_set_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_store_cred_into: None,
            gssspi_acquire_cred_with_password: Some(
                spnego_gss_acquire_cred_with_password
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_name_t,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_OID_set,
                        _: crate::gssapi_h::gss_cred_usage_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                        _: *mut crate::gssapi_h::gss_OID_set,
                        _: *mut crate::gssapi_h::OM_uint32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_export_cred: Some(
                spnego_gss_export_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_cred_id_t,
                        _: crate::gssapi_h::gss_buffer_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_import_cred: Some(
                spnego_gss_import_cred
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_buffer_t,
                        _: *mut crate::gssapi_h::gss_cred_id_t,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_import_sec_context_by_mech: None,
            gssspi_import_name_by_mech: None,
            gssspi_import_cred_by_mech: None,
            gss_get_mic_iov: Some(
                spnego_gss_get_mic_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_verify_mic_iov: Some(
                spnego_gss_verify_mic_iov
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: *mut crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gss_get_mic_iov_length: Some(
                spnego_gss_get_mic_iov_length
                    as unsafe extern "C" fn(
                        _: *mut crate::gssapi_h::OM_uint32,
                        _: crate::gssapi_h::gss_ctx_id_t,
                        _: crate::gssapi_h::gss_qop_t,
                        _: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
                        _: i32,
                    ) -> crate::gssapi_h::OM_uint32,
            ),
            gssspi_query_meta_data: None,
            gssspi_exchange_meta_data: None,
            gssspi_query_mechanism_info: None,
        };
        init
    }
};

unsafe extern "C" fn gss_spnegomechglue_init() -> i32 {
    let mut mech_spnego = crate::mglueP_h::gss_mech_config {
        kmodName: 0 as *mut i8,
        uLibName: 0 as *mut i8,
        mechNameStr: 0 as *mut i8,
        optionStr: 0 as *mut i8,
        dl_handle: 0 as *mut libc::c_void,
        mech_type: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
        mech: 0 as *mut crate::mglueP_h::gss_config,
        priority: 0,
        freeMech: 0,
        is_interposer: 0,
        int_mech_type: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
        int_mech: 0 as *mut crate::mglueP_h::gss_config,
        next: 0 as *mut crate::mglueP_h::gss_mech_config,
    };
    crate::stdlib::memset(
        &mut mech_spnego as *mut crate::mglueP_h::gss_mech_config as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::mglueP_h::gss_mech_config>(),
    );
    mech_spnego.mech = &mut spnego_mechanism;
    mech_spnego.mechNameStr = b"spnego\x00" as *const u8 as *mut i8;
    mech_spnego.mech_type = 0 as crate::gssapi_h::gss_OID;
    return crate::src::mechglue::g_initialize::gssint_register_mechinfo(&mut mech_spnego);
}
/* _GSS_STATIC_LINK */
#[no_mangle]

pub unsafe extern "C" fn gss_spnegoint_lib_init() -> i32 {
    let mut err: i32 = 0;
    err = crate::k5_thread_h::krb5int_key_register(
        crate::k5_thread_h::K5_KEY_GSS_SPNEGO_STATUS,
        None,
    );
    if err != 0 {
        return err;
    }
    return gss_spnegomechglue_init();
}
#[no_mangle]

pub unsafe extern "C" fn gss_spnegoint_lib_fini() {}

unsafe extern "C" fn create_spnego_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mcred: crate::gssapi_h::gss_cred_id_t,
    mut cred_out: *mut crate::gssapiP_spnego_h::spnego_gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut spcred = 0 as *mut crate::gssapiP_spnego_h::C2RustUnnamed_3;
    *cred_out = 0 as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    spcred = crate::stdlib::calloc(
        1usize,
        ::std::mem::size_of::<crate::gssapiP_spnego_h::C2RustUnnamed_3>(),
    ) as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    if spcred.is_null() {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    }
    (*spcred).mcred = mcred;
    *cred_out = spcred;
    return 0u32;
}
/* SPNEGO oid declarations */
/* DEBUG */
/*
 * declarations of internal name mechanism functions
 */
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_acquire_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut desired_name: crate::gssapi_h::gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    return spnego_gss_acquire_cred_from(
        minor_status,
        desired_name,
        time_req,
        desired_mechs,
        cred_usage,
        0 as crate::gssapi_ext_h::gss_const_key_value_set_t,
        output_cred_handle,
        actual_mechs,
        time_rec,
    );
}
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_acquire_cred_from(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    desired_name: crate::gssapi_h::gss_name_t,
    mut _time_req: crate::gssapi_h::OM_uint32,
    _desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut amechs = 0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    let mut mcred = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut spcred = 0 as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    if !actual_mechs.is_null() {
        *actual_mechs = 0 as crate::gssapi_h::gss_OID_set
    }
    if !time_rec.is_null() {
        *time_rec = 0u32
    }
    /* We will obtain a mechglue credential and wrap it in a
     * spnego_gss_cred_id_rec structure.  Allocate the wrapper. */
    status = create_spnego_cred(minor_status, mcred, &mut spcred);
    if status != 0u32 {
        return status;
    }
    /*
     * Always use get_available_mechs to collect a list of
     * mechs for which creds are available.
     */
    status = get_available_mechs(
        minor_status,
        desired_name,
        cred_usage,
        cred_store,
        &mut mcred,
        &mut amechs,
        time_rec,
    );
    if !actual_mechs.is_null() && !amechs.is_null() {
        crate::src::mechglue::g_oid_ops::gssint_copy_oid_set(
            &mut tmpmin,
            amechs as *const crate::gssapi_h::gss_OID_set_desc,
            actual_mechs,
        );
    }
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpmin, &mut amechs);
    if status == 0u32 {
        (*spcred).mcred = mcred;
        *output_cred_handle = spcred as crate::gssapi_h::gss_cred_id_t
    } else {
        crate::stdlib::free(spcred as *mut libc::c_void);
        *output_cred_handle = 0 as crate::gssapi_h::gss_cred_id_t
    }
    return status;
}
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_release_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut spcred = 0 as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    if minor_status.is_null() || cred_handle.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if (*cred_handle).is_null() {
        return 0u32;
    }
    spcred = *cred_handle as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    *cred_handle = 0 as crate::gssapi_h::gss_cred_id_t;
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(
        minor_status,
        &mut (*spcred).neg_mechs,
    );
    crate::src::mechglue::g_rel_cred::gss_release_cred(minor_status, &mut (*spcred).mcred);
    crate::stdlib::free(spcred as *mut libc::c_void);
    return 0u32;
}

unsafe extern "C" fn create_spnego_ctx(
    mut initiate: i32,
) -> crate::gssapiP_spnego_h::spnego_gss_ctx_id_t {
    let mut spnego_ctx = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    spnego_ctx =
        crate::stdlib::malloc(::std::mem::size_of::<crate::gssapiP_spnego_h::spnego_ctx_st>())
            as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if spnego_ctx.is_null() {
        return 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    }
    (*spnego_ctx).magic_num = 0xfedu32;
    (*spnego_ctx).ctx_handle = 0 as crate::gssapi_h::gss_ctx_id_t;
    (*spnego_ctx).mech_set = 0 as crate::gssapi_h::gss_OID_set;
    (*spnego_ctx).internal_mech = 0 as crate::gssapi_h::gss_OID;
    (*spnego_ctx).DER_mechTypes.length = 0usize;
    (*spnego_ctx).DER_mechTypes.value = 0 as *mut libc::c_void;
    (*spnego_ctx).mic_reqd = 0i32;
    (*spnego_ctx).mic_sent = 0i32;
    (*spnego_ctx).mic_rcvd = 0i32;
    (*spnego_ctx).mech_complete = 0i32;
    (*spnego_ctx).nego_done = 0i32;
    (*spnego_ctx).opened = 0i32;
    (*spnego_ctx).initiate = initiate;
    (*spnego_ctx).internal_name = 0 as crate::gssapi_h::gss_name_t;
    (*spnego_ctx).actual_mech = 0 as crate::gssapi_h::gss_OID;
    (*spnego_ctx).deleg_cred = 0 as crate::gssapi_h::gss_cred_id_t;
    (*spnego_ctx).negoex_step = 0i32;
    crate::stdlib::memset(
        &mut (*spnego_ctx).negoex_transcript as *mut crate::k5_buf_h::k5buf as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_buf_h::k5buf>(),
    );
    (*spnego_ctx).negoex_seqnum = 0u32;
    (*spnego_ctx).negoex_mechs.tqh_first = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    (*spnego_ctx).negoex_mechs.tqh_last = &mut (*spnego_ctx).negoex_mechs.tqh_first;
    (*spnego_ctx).kctx = 0 as crate::krb5_h::krb5_context;
    crate::stdlib::memset(
        (*spnego_ctx).negoex_conv_id.as_mut_ptr() as *mut libc::c_void,
        0i32,
        16usize,
    );
    return spnego_ctx;
}
/* iso(1) org(3) dod(6) internet(1) private(4) enterprises(1) samba(7165)
 * gssntlmssp(655) controls(1) spnego_req_mechlistMIC(2) */

static mut spnego_req_mechlistMIC_oid: crate::gssapi_h::gss_OID_desc = {
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 11u32,
        elements: b"+\x06\x01\x04\x01\xb7}\x85\x0f\x01\x02\x00" as *const u8 as *mut libc::c_void,
    };
    init
};
/*
 * Return nonzero if the mechanism has reason to believe that a mechlistMIC
 * exchange will be required.  Microsoft servers erroneously require SPNEGO
 * mechlistMIC if they see an internal MIC within an NTLMSSP Authenticate
 * message, even if NTLMSSP was the preferred mechanism.
 */

unsafe extern "C" fn mech_requires_mechlistMIC(
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
) -> i32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = (*sc).ctx_handle;
    let mut oid = &spnego_req_mechlistMIC_oid as *const crate::gssapi_h::gss_OID_desc
        as crate::gssapi_h::gss_OID;
    let mut bufs = 0 as *mut crate::gssapi_ext_h::gss_buffer_set_desc_struct;
    let mut result: i32 = 0;
    major = crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid(
        &mut minor, ctx, oid, &mut bufs,
    );
    if major != 0u32 {
        return 0i32;
    }
    /* Report true if the mech returns a single buffer containing a single
     * byte with value 1. */
    result = (!bufs.is_null()
        && (*bufs).count == 1usize
        && (*(*bufs).elements.offset(0isize)).length == 1usize
        && crate::stdlib::memcmp(
            (*(*bufs).elements.offset(0isize)).value,
            b"\x01\x00" as *const u8 as *const libc::c_void,
            1usize,
        ) == 0i32) as i32;
    crate::src::mechglue::g_buffer_set::gss_release_buffer_set(&mut minor, &mut bufs);
    return result;
}
/* iso(1) org(3) dod(6) internet(1) private(4) enterprises(1) Microsoft(311)
 * security(2) mechanisms(2) NTLM(10) */

static mut gss_mech_ntlmssp_oid: crate::gssapi_h::gss_OID_desc = {
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 10u32,
        elements: b"+\x06\x01\x04\x01\x827\x02\x02\n\x00" as *const u8 as *mut libc::c_void,
    };
    init
};
/* iso(1) org(3) dod(6) internet(1) private(4) enterprises(1) samba(7165)
 * gssntlmssp(655) controls(1) ntlmssp_reset_crypto(3) */

static mut ntlmssp_reset_crypto_oid: crate::gssapi_h::gss_OID_desc = {
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 11u32,
        elements: b"+\x06\x01\x04\x01\xb7}\x85\x0f\x01\x03\x00" as *const u8 as *mut libc::c_void,
    };
    init
};
/*
 * MS-SPNG section 3.3.5.1 warns that the NTLM mechanism requires special
 * handling of the crypto state to interop with Windows.  If the mechanism for
 * sc is SPNEGO, invoke a mechanism-specific operation on the context to reset
 * the RC4 state after producing or verifying a MIC.  Ignore a result of
 * GSS_S_UNAVAILABLE for compatibility with older versions of the mechanism
 * that do not support this functionality.
 */

unsafe extern "C" fn ntlmssp_reset_crypto_state(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut verify: crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut value = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    if !((*(*sc).internal_mech).length == gss_mech_ntlmssp_oid.length
        && crate::stdlib::memcmp(
            (*(*sc).internal_mech).elements,
            gss_mech_ntlmssp_oid.elements,
            (*(*sc).internal_mech).length as usize,
        ) == 0i32)
    {
        return 0u32;
    }
    value.length = ::std::mem::size_of::<crate::gssapi_h::OM_uint32>();
    value.value = &mut verify as *mut crate::gssapi_h::OM_uint32 as *mut libc::c_void;
    major = crate::src::mechglue::g_set_context_option::gss_set_sec_context_option(
        &mut minor,
        &mut (*sc).ctx_handle,
        &ntlmssp_reset_crypto_oid as *const crate::gssapi_h::gss_OID_desc
            as crate::gssapi_h::gss_OID,
        &mut value,
    );
    if major == (16u32) << 16i32 {
        return 0u32;
    }
    *minor_status = minor;
    return major;
}
/*
 * Both initiator and acceptor call here to verify and/or create mechListMIC,
 * and to consistency-check the MIC state.  handle_mic is invoked only if the
 * negotiated mech has completed and supports MICs.
 */

unsafe extern "C" fn handle_mic(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mic_in: crate::gssapi_h::gss_buffer_t,
    mut send_mechtok: i32,
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut mic_out: *mut crate::gssapi_h::gss_buffer_t,
    mut negState: *mut crate::gssapi_h::OM_uint32,
    mut tokflag: *mut crate::gssapiP_spnego_h::send_token_flag,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    ret = (13u32) << 16i32;
    *mic_out = 0 as crate::gssapi_h::gss_buffer_t;
    if !mic_in.is_null() {
        if (*sc).mic_rcvd != 0 {
            /* Reject MIC if we've already received a MIC. */
            *negState = 2u32;
            *tokflag = crate::gssapiP_spnego_h::ERROR_TOKEN_SEND;
            return (9u32) << 16i32;
        }
    } else if (*sc).mic_reqd != 0 && send_mechtok == 0 {
        /*
         * If the peer sends the final mechanism token, it
         * must send the MIC with that token if the
         * negotiation requires MICs.
         */
        *negState = 2u32;
        *tokflag = crate::gssapiP_spnego_h::ERROR_TOKEN_SEND;
        return (9u32) << 16i32;
    }
    ret = process_mic(minor_status, mic_in, sc, mic_out, negState, tokflag);
    if ret != 0u32 {
        return ret;
    }
    if (*sc).mic_reqd != 0 {
        if (*sc).mic_sent != 0 || (*sc).mic_rcvd != 0 {
        } else {
            crate::stdlib::__assert_fail(b"sc->mic_sent || sc->mic_rcvd\x00" as *const u8 as
                              *const i8,
                          b"spnego_mech.c\x00" as *const u8 as
                              *const i8,
                          591u32,
                          (*::std::mem::transmute::<&[u8; 122],
                                                    &[i8; 122]>(b"OM_uint32 handle_mic(OM_uint32 *, gss_buffer_t, int, spnego_gss_ctx_id_t, gss_buffer_t *, OM_uint32 *, send_token_flag *)\x00")).as_ptr());
        }
    }
    if (*sc).mic_sent != 0 && (*sc).mic_rcvd != 0 {
        ret = 0u32;
        *negState = 0u32;
        if (*mic_out).is_null() {
            /*
             * We sent a MIC on the previous pass; we
             * shouldn't be sending a mechanism token.
             */
            if send_mechtok == 0 {
            } else {
                crate::stdlib::__assert_fail(b"!send_mechtok\x00" as *const u8 as
                                  *const i8,
                              b"spnego_mech.c\x00" as *const u8 as
                                  *const i8,
                              601u32,
                              (*::std::mem::transmute::<&[u8; 122],
                                                        &[i8; 122]>(b"OM_uint32 handle_mic(OM_uint32 *, gss_buffer_t, int, spnego_gss_ctx_id_t, gss_buffer_t *, OM_uint32 *, send_token_flag *)\x00")).as_ptr());
            }
            *tokflag = crate::gssapiP_spnego_h::NO_TOKEN_SEND
        } else {
            *tokflag = crate::gssapiP_spnego_h::CONT_TOKEN_SEND
        }
    } else if (*sc).mic_reqd != 0 {
        *negState = 1u32;
        ret = ((1i32) << 0i32 + 0i32) as crate::gssapi_h::OM_uint32
    } else if *negState == 0u32 {
        ret = 0u32
    } else {
        ret = ((1i32) << 0i32 + 0i32) as crate::gssapi_h::OM_uint32
    }
    return ret;
}
/*
 * Perform the actual verification and/or generation of mechListMIC.
 */

unsafe extern "C" fn process_mic(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mic_in: crate::gssapi_h::gss_buffer_t,
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut mic_out: *mut crate::gssapi_h::gss_buffer_t,
    mut negState: *mut crate::gssapi_h::OM_uint32,
    mut tokflag: *mut crate::gssapiP_spnego_h::send_token_flag,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut qop_state: crate::gssapi_h::gss_qop_t = 0;
    let mut tmpmic = {
        let mut init = crate::gssapi_h::gss_buffer_desc_struct {
            length: 0usize,
            value: 0 as *mut libc::c_void,
        };
        init
    };
    ret = (13u32) << 16i32;
    if !mic_in.is_null() {
        ret = crate::src::mechglue::g_verify::gss_verify_mic(
            minor_status,
            (*sc).ctx_handle,
            &mut (*sc).DER_mechTypes,
            mic_in,
            &mut qop_state,
        );
        if ret == 0u32 {
            ret = ntlmssp_reset_crypto_state(minor_status, sc, 1u32)
        }
        if ret != 0u32 {
            *negState = 2u32;
            *tokflag = crate::gssapiP_spnego_h::ERROR_TOKEN_SEND;
            return ret;
        }
        /* If we got a MIC, we must send a MIC. */
        (*sc).mic_reqd = 1i32;
        (*sc).mic_rcvd = 1i32
    }
    if (*sc).mic_reqd != 0 && (*sc).mic_sent == 0 {
        ret = crate::src::mechglue::g_sign::gss_get_mic(
            minor_status,
            (*sc).ctx_handle,
            0u32,
            &mut (*sc).DER_mechTypes,
            &mut tmpmic,
        );
        if ret == 0u32 {
            ret = ntlmssp_reset_crypto_state(minor_status, sc, 0u32)
        }
        if ret != 0u32 {
            crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, &mut tmpmic);
            *tokflag = crate::gssapiP_spnego_h::NO_TOKEN_SEND;
            return ret;
        }
        *mic_out = crate::stdlib::malloc(::std::mem::size_of::<crate::gssapi_h::gss_buffer_desc>())
            as crate::gssapi_h::gss_buffer_t;
        if (*mic_out).is_null() {
            crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, &mut tmpmic);
            *tokflag = crate::gssapiP_spnego_h::NO_TOKEN_SEND;
            return (13u32) << 16i32;
        }
        **mic_out = tmpmic;
        (*sc).mic_sent = 1i32
    }
    return 0u32;
}
/* Create a new SPNEGO context handle for the initial call to
 * spnego_gss_init_sec_context().  */

unsafe extern "C" fn init_ctx_new(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut spcred: crate::gssapiP_spnego_h::spnego_gss_cred_id_t,
    mut tokflag: *mut crate::gssapiP_spnego_h::send_token_flag,
    mut sc_out: *mut crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    *sc_out = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    sc = create_spnego_ctx(1i32);
    if sc.is_null() {
        return (13u32) << 16i32;
    }
    /* determine negotiation mech set */
    ret = get_negotiable_mechs(minor_status, sc, spcred, 1i32);
    if !(ret != 0u32) {
        /* Set an initial internal mech to make the first context token. */
        (*sc).internal_mech = &mut *(*(*sc).mech_set).elements.offset(0isize)
            as *mut crate::gssapi_h::gss_OID_desc_struct;
        if put_mech_set((*sc).mech_set, &mut (*sc).DER_mechTypes) < 0i32 {
            ret = (13u32) << 16i32
        } else {
            (*sc).ctx_handle = 0 as crate::gssapi_h::gss_ctx_id_t;
            *sc_out = sc;
            sc = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
            *tokflag = crate::gssapiP_spnego_h::INIT_TOKEN_SEND;
            ret = 0u32
        }
    }
    release_spnego_ctx(&mut sc);
    return ret;
}
/*
 * Called by second and later calls to spnego_gss_init_sec_context()
 * to decode reply and update state.
 */

unsafe extern "C" fn init_ctx_cont(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut buf: crate::gssapi_h::gss_buffer_t,
    mut responseToken: *mut crate::gssapi_h::gss_buffer_t,
    mut mechListMIC: *mut crate::gssapi_h::gss_buffer_t,
    mut acc_negState: *mut crate::gssapi_h::OM_uint32,
    mut tokflag: *mut crate::gssapiP_spnego_h::send_token_flag,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut ptr = 0 as *mut u8;
    let mut supportedMech = 0 as crate::gssapi_h::gss_OID;
    *acc_negState = 0xffffffffu32;
    *tokflag = crate::gssapiP_spnego_h::ERROR_TOKEN_SEND;
    ptr = (*buf).value as *mut u8;
    ret = get_negTokenResp(
        minor_status,
        ptr,
        (*buf).length as u32,
        acc_negState,
        &mut supportedMech,
        responseToken,
        mechListMIC,
    );
    if !(ret != 0u32) {
        /* Bail out now on a reject with no error token.  If we have an error
         * token, keep going and get a better error status from the mech. */
        if *acc_negState == 2u32 && (*responseToken).is_null() {
            if (*sc).nego_done == 0 {
                /* RFC 4178 says to return GSS_S_BAD_MECH on a
                 * mechanism negotiation failure. */
                *minor_status = 0x20000004u32;
                *minor_status =
                    crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
                ret = (1u32) << 16i32
            } else {
                ret = (13u32) << 16i32
            }
            *tokflag = crate::gssapiP_spnego_h::NO_TOKEN_SEND
        } else if (*sc).nego_done == 0 {
            ret = init_ctx_nego(
                minor_status,
                sc,
                *acc_negState,
                supportedMech,
                responseToken,
                mechListMIC,
                tokflag,
            )
        } else if (*sc).mech_complete == 0 && (*responseToken).is_null()
            || (*sc).mech_complete != 0 && !(*responseToken).is_null()
        {
            /*
             * nego_done is false for the first call to init_ctx_cont()
             */
            /* Missing or spurious token from acceptor. */
            ret = (9u32) << 16i32
        } else if (*sc).mech_complete == 0 || (*sc).mic_reqd != 0 && (*sc).ctx_flags & 32u32 != 0 {
            /* Not obviously done; we may decide we're done later in
             * init_ctx_call_init or handle_mic. */
            *tokflag = crate::gssapiP_spnego_h::CONT_TOKEN_SEND;
            ret = 0u32
        } else {
            /* mech finished on last pass and no MIC required, so done. */
            *tokflag = crate::gssapiP_spnego_h::NO_TOKEN_SEND;
            ret = 0u32
        }
    }
    if !supportedMech.is_null() {
        crate::src::generic::oid_ops::generic_gss_release_oid(&mut tmpmin, &mut supportedMech);
    }
    return ret;
}
/*
 * Consistency checking and mechanism negotiation handling for second
 * call of spnego_gss_init_sec_context().  Call init_ctx_reselect() to
 * update internal state if acceptor has counter-proposed.
 */

unsafe extern "C" fn init_ctx_nego(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut acc_negState: crate::gssapi_h::OM_uint32,
    mut supportedMech: crate::gssapi_h::gss_OID,
    mut responseToken: *mut crate::gssapi_h::gss_buffer_t,
    mut mechListMIC: *mut crate::gssapi_h::gss_buffer_t,
    mut tokflag: *mut crate::gssapiP_spnego_h::send_token_flag,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    *tokflag = crate::gssapiP_spnego_h::ERROR_TOKEN_SEND;
    ret = (9u32) << 16i32;
    /*
     * According to RFC 4178, both supportedMech and negState must be
     * present in the first acceptor token.  However, some Java
     * implementations include only a responseToken in the first
     * NegTokenResp.  In this case we can use sc->internal_mech as the
     * negotiated mechanism.  (We do not currently look at acc_negState
     * when continuing with the optimistic mechanism.)
     */
    if supportedMech.is_null() {
        supportedMech = (*sc).internal_mech
    }
    /*
     * If the mechanism we sent is not the mechanism returned from
     * the server, we need to handle the server's counter
     * proposal.  There is a bug in SAMBA servers that always send
     * the old Kerberos mech OID, even though we sent the new one.
     * So we will treat all the Kerberos mech OIDS as the same.
     */
    if !(is_kerb_mech(supportedMech) != 0 && is_kerb_mech((*sc).internal_mech) != 0)
        && !((*supportedMech).length == (*(*sc).internal_mech).length
            && crate::stdlib::memcmp(
                (*supportedMech).elements,
                (*(*sc).internal_mech).elements,
                (*supportedMech).length as usize,
            ) == 0i32)
    {
        ret = init_ctx_reselect(
            minor_status,
            sc,
            acc_negState,
            supportedMech,
            responseToken,
            mechListMIC,
            tokflag,
        )
    } else if (*responseToken).is_null() {
        if (*sc).mech_complete != 0 {
            /*
             * Mech completed on first call to its
             * init_sec_context().  Acceptor sends no mech
             * token.
             */
            *tokflag = crate::gssapiP_spnego_h::NO_TOKEN_SEND;
            ret = 0u32
        } else {
            /*
             * Reject missing mech token when optimistic
             * mech selected.
             */
            *minor_status = 0x20000005u32;
            *minor_status =
                crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
            ret = (9u32) << 16i32
        }
    } else if (**responseToken).length == 0usize && (*sc).mech_complete != 0 {
        /* Handle old IIS servers returning empty token instead of
         * null tokens in the non-mutual auth case. */
        *tokflag = crate::gssapiP_spnego_h::NO_TOKEN_SEND;
        ret = 0u32
    } else if (*sc).mech_complete != 0 {
        /* Reject spurious mech token. */
        ret = (9u32) << 16i32
    } else {
        *tokflag = crate::gssapiP_spnego_h::CONT_TOKEN_SEND;
        ret = 0u32
    }
    (*sc).nego_done = 1i32;
    return ret;
}
/*
 * Handle acceptor's counter-proposal of an alternative mechanism.
 */

unsafe extern "C" fn init_ctx_reselect(
    mut _minor_status: *mut crate::gssapi_h::OM_uint32,
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut acc_negState: crate::gssapi_h::OM_uint32,
    mut supportedMech: crate::gssapi_h::gss_OID,
    mut _responseToken: *mut crate::gssapi_h::gss_buffer_t,
    mut _mechListMIC: *mut crate::gssapi_h::gss_buffer_t,
    mut tokflag: *mut crate::gssapiP_spnego_h::send_token_flag,
) -> crate::gssapi_h::OM_uint32 {
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut i: crate::stddef_h::size_t = 0;
    crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context(
        &mut tmpmin,
        &mut (*sc).ctx_handle,
        0 as crate::gssapi_h::gss_buffer_t,
    );
    /* Find supportedMech in sc->mech_set. */
    i = 0usize;
    while i < (*(*sc).mech_set).count {
        if (*supportedMech).length == (*(*(*sc).mech_set).elements.offset(i as isize)).length
            && crate::stdlib::memcmp(
                (*supportedMech).elements,
                (*(*(*sc).mech_set).elements.offset(i as isize)).elements,
                (*supportedMech).length as usize,
            ) == 0i32
        {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i == (*(*sc).mech_set).count {
        return (9u32) << 16i32;
    }
    (*sc).internal_mech = &mut *(*(*sc).mech_set).elements.offset(i as isize)
        as *mut crate::gssapi_h::gss_OID_desc_struct;
    /*
     * A server conforming to RFC4178 MUST set REQUEST_MIC here, but
     * Windows Server 2003 and earlier implement (roughly) RFC2478 instead,
     * and send ACCEPT_INCOMPLETE.  Tolerate that only if we are falling
     * back to NTLMSSP.
     */
    if acc_negState == 1u32 {
        if !((*supportedMech).length == gss_mech_ntlmssp_oid.length
            && crate::stdlib::memcmp(
                (*supportedMech).elements,
                gss_mech_ntlmssp_oid.elements,
                (*supportedMech).length as usize,
            ) == 0i32)
        {
            return (9u32) << 16i32;
        }
    } else if acc_negState != 3u32 {
        return (9u32) << 16i32;
    }
    (*sc).mech_complete = 0i32;
    (*sc).mic_reqd = (acc_negState == 3u32) as i32;
    *tokflag = crate::gssapiP_spnego_h::CONT_TOKEN_SEND;
    return 0u32;
}
/*
 * Wrap call to mechanism gss_init_sec_context() and update state
 * accordingly.
 */

unsafe extern "C" fn init_ctx_call_init(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut spcred: crate::gssapiP_spnego_h::spnego_gss_cred_id_t,
    mut acc_negState: crate::gssapi_h::OM_uint32,
    mut target_name: crate::gssapi_h::gss_name_t,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut mechtok_in: crate::gssapi_h::gss_buffer_t,
    mut mechtok_out: crate::gssapi_h::gss_buffer_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut send_token: *mut crate::gssapiP_spnego_h::send_token_flag,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut mech_req_flags: crate::gssapi_h::OM_uint32 = 0;
    let mut mcred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    mcred = if spcred.is_null() {
        0 as crate::gssapi_h::gss_cred_id_t
    } else {
        (*spcred).mcred
    };
    mech_req_flags = req_flags;
    if spcred.is_null() || (*spcred).no_ask_integ == 0 {
        mech_req_flags |= 32u32
    }
    if crate::src::mechglue::g_oid_ops::gss_oid_equal(
        (*sc).internal_mech as crate::gssapi_h::gss_const_OID,
        &mut negoex_mech as *mut crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_const_OID,
    ) != 0
    {
        ret = crate::src::spnego::negoex_ctx::negoex_init(
            minor_status,
            sc,
            mcred,
            target_name,
            mech_req_flags,
            time_req,
            mechtok_in,
            mechtok_out,
            time_rec,
        )
    } else {
        ret = crate::src::mechglue::g_init_sec_context::gss_init_sec_context(
            minor_status,
            mcred,
            &mut (*sc).ctx_handle,
            target_name,
            (*sc).internal_mech,
            mech_req_flags,
            time_req,
            0 as crate::gssapi_h::gss_channel_bindings_t,
            mechtok_in,
            &mut (*sc).actual_mech,
            mechtok_out,
            &mut (*sc).ctx_flags,
            time_rec,
        )
    }
    /* Bail out if the acceptor gave us an error token but the mech didn't
     * see it as an error. */
    if acc_negState == 2u32 && ret & ((0o377u32) << 24i32 | (0o377u32) << 16i32) == 0 {
        ret = (9u32) << 16i32
    } else {
        if ret == 0u32 {
            (*sc).mech_complete = 1i32;
            /*
             * Microsoft SPNEGO implementations expect an even number of
             * token exchanges.  So if we're sending a final token, ask for
             * a zero-length token back from the server.  Also ask for a
             * token back if this is the first token or if a MIC exchange
             * is required.
             */
            if *send_token == crate::gssapiP_spnego_h::CONT_TOKEN_SEND
                && (*mechtok_out).length == 0usize
                && ((*sc).mic_reqd == 0 || (*sc).ctx_flags & 32u32 == 0)
            {
                *send_token = crate::gssapiP_spnego_h::NO_TOKEN_SEND
            }
            return 0u32;
        }
        if ret == ((1i32) << 0i32 + 0i32) as u32 {
            return 0u32;
        }
        if *send_token != crate::gssapiP_spnego_h::INIT_TOKEN_SEND {
            *send_token = crate::gssapiP_spnego_h::ERROR_TOKEN_SEND;
            return ret;
        }
        /*
         * Since this is the first token, we can fall back to later mechanisms
         * in the list.  Since the mechanism list is expected to be short, we
         * can do this with recursion.  If all mechanisms produce errors, the
         * caller should get the error from the first mech in the list.
         */
        gssalloc_free((*(*(*sc).mech_set).elements).elements);
        (*(*sc).mech_set).count = (*(*sc).mech_set).count.wrapping_sub(1);
        crate::stdlib::memmove(
            (*(*sc).mech_set).elements as *mut libc::c_void,
            (*(*sc).mech_set).elements.offset(1isize) as *const libc::c_void,
            (*(*sc).mech_set)
                .count
                .wrapping_mul(::std::mem::size_of::<crate::gssapi_h::gss_OID_desc_struct>()),
        );
        if !((*(*sc).mech_set).count == 0usize) {
            crate::src::mechglue::g_rel_buffer::gss_release_buffer(
                &mut tmpmin,
                &mut (*sc).DER_mechTypes,
            );
            if !(put_mech_set((*sc).mech_set, &mut (*sc).DER_mechTypes) < 0i32) {
                crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context(
                    &mut tmpmin,
                    &mut (*sc).ctx_handle,
                    0 as crate::gssapi_h::gss_buffer_t,
                );
                tmpret = init_ctx_call_init(
                    &mut tmpmin,
                    sc,
                    spcred,
                    acc_negState,
                    target_name,
                    req_flags,
                    time_req,
                    mechtok_in,
                    mechtok_out,
                    time_rec,
                    send_token,
                );
                if !(tmpret != 0u32 && tmpret != ((1i32) << 0i32 + 0i32) as u32) {
                    *minor_status = tmpmin;
                    return tmpret;
                }
            }
        }
    }
    /* Don't output token on error from first call. */
    *send_token = crate::gssapiP_spnego_h::NO_TOKEN_SEND;
    return ret;
}
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_init_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut claimant_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut target_name: crate::gssapi_h::gss_name_t,
    mut _mech_type: crate::gssapi_h::gss_OID,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut _input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut actual_mech: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut send_token = crate::gssapiP_spnego_h::NO_TOKEN_SEND;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut negState = 0xffffffffu32;
    let mut acc_negState: crate::gssapi_h::OM_uint32 = 0;
    let mut mechtok_in = 0 as *mut crate::gssapi_h::gss_buffer_desc_struct;
    let mut mechListMIC_in = 0 as *mut crate::gssapi_h::gss_buffer_desc_struct;
    let mut mechListMIC_out = 0 as *mut crate::gssapi_h::gss_buffer_desc_struct;
    let mut mechtok_out = {
        let mut init = crate::gssapi_h::gss_buffer_desc_struct {
            length: 0usize,
            value: 0 as *mut libc::c_void,
        };
        init
    };
    let mut spcred = 0 as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    let mut spnego_ctx = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    mechListMIC_in = 0 as crate::gssapi_h::gss_buffer_t;
    mechListMIC_out = mechListMIC_in;
    mechtok_in = mechListMIC_out;
    /*
     * This function works in three steps:
     *
     *   1. Perform mechanism negotiation.
     *   2. Invoke the negotiated or optimistic mech's gss_init_sec_context
     *      function and examine the results.
     *   3. Process or generate MICs if necessary.
     *
     * The three steps share responsibility for determining when the
     * exchange is complete.  If the selected mech completed in a previous
     * call and no MIC exchange is expected, then step 1 will decide.  If
     * the selected mech completes in this call and no MIC exchange is
     * expected, then step 2 will decide.  If a MIC exchange is expected,
     * then step 3 will decide.  If an error occurs in any step, the
     * exchange will be aborted, possibly with an error token.
     *
     * negState determines the state of the negotiation, and is
     * communicated to the acceptor if a continuing token is sent.
     * send_token is used to indicate what type of token, if any, should be
     * generated.
     */
    /* Validate arguments. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !output_token.is_null() {
        (*output_token).length = 0usize;
        (*output_token).value = 0 as *mut libc::c_void
    }
    if minor_status.is_null() || output_token.is_null() || context_handle.is_null() {
        return (2u32) << 24i32;
    }
    if !actual_mech.is_null() {
        *actual_mech = 0 as crate::gssapi_h::gss_OID
    }
    if !time_rec.is_null() {
        *time_rec = 0u32
    }
    /* Step 1: perform mechanism negotiation. */
    spcred = claimant_cred_handle as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    spnego_ctx = *context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if spnego_ctx.is_null() {
        ret = init_ctx_new(minor_status, spcred, &mut send_token, &mut spnego_ctx);
        if ret != 0u32 {
            current_block = 16927928461156034833;
        } else {
            *context_handle = spnego_ctx as crate::gssapi_h::gss_ctx_id_t;
            acc_negState = 0xffffffffu32;
            current_block = 9828876828309294594;
        }
    } else {
        ret = init_ctx_cont(
            minor_status,
            spnego_ctx,
            input_token,
            &mut mechtok_in,
            &mut mechListMIC_in,
            &mut acc_negState,
            &mut send_token,
        );
        if ret != 0u32 {
            current_block = 16927928461156034833;
        } else {
            current_block = 9828876828309294594;
        }
    }
    match current_block {
        9828876828309294594 =>
        /* Step 2: invoke the selected or optimistic mechanism's
         * gss_init_sec_context function, if it didn't complete previously. */
        {
            if (*spnego_ctx).mech_complete == 0 {
                ret = init_ctx_call_init(
                    minor_status,
                    spnego_ctx,
                    spcred,
                    acc_negState,
                    target_name,
                    req_flags,
                    time_req,
                    mechtok_in,
                    &mut mechtok_out,
                    time_rec,
                    &mut send_token,
                );
                if ret != 0u32 {
                    current_block = 16927928461156034833;
                } else {
                    /* Give the mechanism a chance to force a mechlistMIC. */
                    if mech_requires_mechlistMIC(spnego_ctx) != 0 {
                        (*spnego_ctx).mic_reqd = 1i32
                    }
                    current_block = 8693738493027456495;
                }
            } else {
                current_block = 8693738493027456495;
            }
            match current_block {
                16927928461156034833 => {}
                _ => {
                    /* Step 3: process or generate the MIC, if the negotiated mech is
                     * complete and supports MICs.  Also decide the outgoing negState. */
                    negState = 1u32;
                    if (*spnego_ctx).mech_complete != 0 && (*spnego_ctx).ctx_flags & 32u32 != 0 {
                        ret = handle_mic(
                            minor_status,
                            mechListMIC_in,
                            (mechtok_out.length != 0usize) as i32,
                            spnego_ctx,
                            &mut mechListMIC_out,
                            &mut negState,
                            &mut send_token,
                        );
                        if ret != 0u32 && ret != ((1i32) << 0i32 + 0i32) as u32 {
                            current_block = 16927928461156034833;
                        } else {
                            current_block = 7333393191927787629;
                        }
                    } else {
                        current_block = 7333393191927787629;
                    }
                    match current_block {
                        16927928461156034833 => {}
                        _ => {
                            if !ret_flags.is_null() {
                                *ret_flags = (*spnego_ctx).ctx_flags & !(128i32) as u32
                            }
                            ret = if send_token == crate::gssapiP_spnego_h::NO_TOKEN_SEND
                                || negState == 0u32
                            {
                                0i32
                            } else {
                                (1i32) << 0i32 + 0i32
                            } as crate::gssapi_h::OM_uint32
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if send_token == crate::gssapiP_spnego_h::INIT_TOKEN_SEND {
        if make_spnego_tokenInit_msg(
            spnego_ctx,
            0i32,
            mechListMIC_out,
            req_flags,
            &mut mechtok_out,
            send_token,
            output_token,
        ) < 0i32
        {
            ret = (13u32) << 16i32
        }
    } else if send_token != crate::gssapiP_spnego_h::NO_TOKEN_SEND {
        if send_token == crate::gssapiP_spnego_h::ERROR_TOKEN_SEND {
            negState = 2u32
        }
        if make_spnego_tokenTarg_msg(
            negState,
            0 as crate::gssapi_h::gss_OID,
            &mut mechtok_out,
            mechListMIC_out,
            send_token,
            output_token,
        ) < 0i32
        {
            ret = (13u32) << 16i32
        }
    }
    crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, &mut mechtok_out);
    if ret == 0u32 {
        (*spnego_ctx).opened = 1i32;
        if !actual_mech.is_null() {
            *actual_mech = (*spnego_ctx).actual_mech
        }
        /* Get an updated lifetime if we didn't call into the mech. */
        if !time_rec.is_null() && *time_rec == 0u32 {
            crate::src::mechglue::g_context_time::gss_context_time(
                &mut tmpmin,
                (*spnego_ctx).ctx_handle,
                time_rec,
            );
        }
    } else if ret != ((1i32) << 0i32 + 0i32) as u32 {
        if !spnego_ctx.is_null() {
            crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context(
                &mut tmpmin,
                &mut (*spnego_ctx).ctx_handle,
                0 as crate::gssapi_h::gss_buffer_t,
            );
            release_spnego_ctx(&mut spnego_ctx);
        }
        *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t
    }
    if !mechtok_in.is_null() {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, mechtok_in);
        crate::stdlib::free(mechtok_in as *mut libc::c_void);
    }
    if !mechListMIC_in.is_null() {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, mechListMIC_in);
        crate::stdlib::free(mechListMIC_in as *mut libc::c_void);
    }
    if !mechListMIC_out.is_null() {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, mechListMIC_out);
        crate::stdlib::free(mechListMIC_out as *mut libc::c_void);
    }
    return ret;
}
/* init_sec_context */
/* We don't want to import KRB5 headers here */

static mut gss_mech_krb5_oid: crate::gssapi_h::gss_OID_desc = {
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 9u32,
        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x00" as *const u8 as *mut libc::c_void,
    };
    init
};

static mut gss_mech_krb5_wrong_oid: crate::gssapi_h::gss_OID_desc = {
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 9u32,
        elements: b"*\x86H\x82\xf7\x12\x01\x02\x02\x00" as *const u8 as *mut libc::c_void,
    };
    init
};
/*
 * verify that the input token length is not 0. If it is, just return.
 * If the token length is greater than 0, der encode as a sequence
 * and place in buf_out, advancing buf_out.
 */

unsafe extern "C" fn put_neg_hints(
    mut buf_out: *mut *mut u8,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut buflen: u32,
) -> i32 {
    let mut ret: i32 = 0;
    /* if token length is 0, we do not want to send */
    if (*input_token).length == 0usize {
        return 0i32;
    }
    if (*input_token).length > buflen as usize {
        return -(1i32);
    }
    let fresh0 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh0 = 0x30u8;
    ret = crate::src::mechglue::g_glue::gssint_put_der_length(
        (*input_token).length as u32,
        buf_out,
        (*input_token).length as u32,
    );
    if ret != 0 {
        return ret;
    }
    crate::stdlib::memcpy(
        *buf_out as *mut libc::c_void,
        (*input_token).value,
        (*input_token).length,
    );
    *buf_out = (*buf_out).offset((*input_token).length as isize);
    return 0i32;
}
/*
 * NegHints ::= SEQUENCE {
 *    hintName       [0]  GeneralString      OPTIONAL,
 *    hintAddress    [1]  OCTET STRING       OPTIONAL
 * }
 */
/* Encode the dummy hintname string (as specified in [MS-SPNG]) into a
 * DER-encoded [0] tagged GeneralString, and place the result in *outbuf. */

unsafe extern "C" fn make_NegHints(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut outbuf: *mut crate::gssapi_h::gss_buffer_t,
) -> i32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut tlen = 0u32;
    let mut hintNameSize = 0u32;
    let mut ptr = 0 as *mut u8;
    let mut t = 0 as *mut u8;
    let mut hintname = b"not_defined_in_RFC4178@please_ignore\x00" as *const u8 as *const i8;
    let hintname_len = crate::stdlib::strlen(hintname);
    *outbuf = 0 as crate::gssapi_h::gss_buffer_t;
    major_status = (13u32) << 16i32;
    /* Length of DER encoded GeneralString */
    tlen = ((1u32).wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
        hintname_len as u32,
    )) as usize)
        .wrapping_add(hintname_len) as u32;
    hintNameSize = tlen;
    /* Length of DER encoded hintName */
    tlen = tlen.wrapping_add((1u32).wrapping_add(
        crate::src::mechglue::g_glue::gssint_der_length_size(hintNameSize),
    )); /* hintName identifier */
    t = gssalloc_malloc(tlen as crate::stddef_h::size_t) as *mut u8; /* don't free */
    if t.is_null() {
        *minor_status = 12u32
    } else {
        ptr = t;
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        *fresh1 = (0xa0i32 | 0i32) as u8;
        if !(crate::src::mechglue::g_glue::gssint_put_der_length(
            hintNameSize,
            &mut ptr,
            tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
        ) != 0)
        {
            let fresh2 = ptr;
            ptr = ptr.offset(1);
            *fresh2 = 0x1bu8;
            if !(crate::src::mechglue::g_glue::gssint_put_der_length(
                hintname_len as u32,
                &mut ptr,
                tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
            ) != 0)
            {
                crate::stdlib::memcpy(
                    ptr as *mut libc::c_void,
                    hintname as *const libc::c_void,
                    hintname_len,
                );
                ptr = ptr.offset(hintname_len as isize);
                *outbuf = crate::stdlib::malloc(::std::mem::size_of::<
                    crate::gssapi_h::gss_buffer_desc,
                >()) as crate::gssapi_h::gss_buffer_t;
                if (*outbuf).is_null() {
                    *minor_status = 12u32
                } else {
                    (**outbuf).value = t as *mut libc::c_void;
                    (**outbuf).length = ptr.wrapping_offset_from(t) as crate::stddef_h::size_t;
                    t = 0 as *mut u8;
                    *minor_status = 0u32;
                    major_status = 0u32
                }
            }
        }
    }
    if !t.is_null() {
        crate::stdlib::free(t as *mut libc::c_void);
    }
    return major_status as i32;
}
/*
 * Create a new SPNEGO context handle for the initial call to
 * spnego_gss_accept_sec_context() when the request is empty.  For empty
 * requests, we implement the Microsoft NegHints extension to SPNEGO for
 * compatibility with some versions of Samba.  See:
 * https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-spng/8e71cf53-e867-4b79-b5b5-38c92be3d472
 */

unsafe extern "C" fn acc_ctx_hints(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut spcred: crate::gssapiP_spnego_h::spnego_gss_cred_id_t,
    mut mechListMIC: *mut crate::gssapi_h::gss_buffer_t,
    mut negState: *mut crate::gssapi_h::OM_uint32,
    mut return_token: *mut crate::gssapiP_spnego_h::send_token_flag,
    mut sc_out: *mut crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    *mechListMIC = 0 as crate::gssapi_h::gss_buffer_t;
    *return_token = crate::gssapiP_spnego_h::NO_TOKEN_SEND;
    *negState = 2u32;
    *minor_status = 0u32;
    *sc_out = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    ret = make_NegHints(minor_status, mechListMIC) as crate::gssapi_h::OM_uint32;
    if !(ret != 0u32) {
        sc = create_spnego_ctx(0i32);
        if sc.is_null() {
            ret = (13u32) << 16i32
        } else {
            ret = get_negotiable_mechs(minor_status, sc, spcred, 2i32);
            if !(ret != 0u32) {
                if put_mech_set((*sc).mech_set, &mut (*sc).DER_mechTypes) < 0i32 {
                    ret = (13u32) << 16i32
                } else {
                    (*sc).internal_mech = 0 as crate::gssapi_h::gss_OID;
                    *negState = 1u32;
                    *return_token = crate::gssapiP_spnego_h::INIT_TOKEN_SEND;
                    (*sc).firstpass = 1i32;
                    *sc_out = sc;
                    sc = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
                    ret = 0u32
                }
            }
        }
    }
    release_spnego_ctx(&mut sc);
    return ret;
}
/*
 * Create a new SPNEGO context handle for the initial call to
 * spnego_gss_accept_sec_context().  Set negState to REJECT if the token is
 * defective, else ACCEPT_INCOMPLETE or REQUEST_MIC, depending on whether
 * the initiator's preferred mechanism is supported.
 */

unsafe extern "C" fn acc_ctx_new(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut buf: crate::gssapi_h::gss_buffer_t,
    mut spcred: crate::gssapiP_spnego_h::spnego_gss_cred_id_t,
    mut mechToken: *mut crate::gssapi_h::gss_buffer_t,
    mut mechListMIC: *mut crate::gssapi_h::gss_buffer_t,
    mut negState: *mut crate::gssapi_h::OM_uint32,
    mut return_token: *mut crate::gssapiP_spnego_h::send_token_flag,
    mut sc_out: *mut crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut req_flags: crate::gssapi_h::OM_uint32 = 0;
    let mut mechTypes = 0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    let mut der_mechTypes = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut mech_wanted = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut sc = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    ret = (9u32) << 16i32;
    der_mechTypes.length = 0usize;
    der_mechTypes.value = 0 as *mut libc::c_void;
    *mechListMIC = 0 as crate::gssapi_h::gss_buffer_t;
    *mechToken = *mechListMIC;
    mechTypes = 0 as crate::gssapi_h::gss_OID_set;
    *return_token = crate::gssapiP_spnego_h::ERROR_TOKEN_SEND;
    *negState = 2u32;
    *minor_status = 0u32;
    ret = get_negTokenInit(
        minor_status,
        buf,
        &mut der_mechTypes,
        &mut mechTypes,
        &mut req_flags,
        mechToken,
        mechListMIC,
    );
    if !(ret != 0u32) {
        sc = create_spnego_ctx(0i32);
        if sc.is_null() {
            ret = (13u32) << 16i32;
            *return_token = crate::gssapiP_spnego_h::NO_TOKEN_SEND
        } else {
            ret = get_negotiable_mechs(minor_status, sc, spcred, 1i32);
            if ret != 0u32 {
                *return_token = crate::gssapiP_spnego_h::NO_TOKEN_SEND
            } else {
                /*
                 * Select the best match between the list of mechs
                 * that the initiator requested and the list that
                 * the acceptor will support.
                 */
                mech_wanted = negotiate_mech(sc, mechTypes, negState);
                if *negState == 2u32 {
                    ret = (1u32) << 16i32
                } else {
                    (*sc).internal_mech = mech_wanted;
                    (*sc).DER_mechTypes = der_mechTypes;
                    der_mechTypes.length = 0usize;
                    der_mechTypes.value = 0 as *mut libc::c_void;
                    if *negState == 3u32 {
                        (*sc).mic_reqd = 1i32
                    }
                    *return_token = crate::gssapiP_spnego_h::INIT_TOKEN_SEND;
                    (*sc).firstpass = 1i32;
                    *sc_out = sc;
                    sc = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
                    ret = 0u32
                }
            }
        }
    }
    release_spnego_ctx(&mut sc);
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpmin, &mut mechTypes);
    if der_mechTypes.length != 0usize {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, &mut der_mechTypes);
    }
    return ret;
}

unsafe extern "C" fn acc_ctx_cont(
    mut minstat: *mut crate::gssapi_h::OM_uint32,
    mut buf: crate::gssapi_h::gss_buffer_t,
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut responseToken: *mut crate::gssapi_h::gss_buffer_t,
    mut mechListMIC: *mut crate::gssapi_h::gss_buffer_t,
    mut negState: *mut crate::gssapi_h::OM_uint32,
    mut return_token: *mut crate::gssapiP_spnego_h::send_token_flag,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut supportedMech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut len: u32 = 0;
    let mut ptr = 0 as *mut u8;
    let mut bufstart = 0 as *mut u8;
    ret = (9u32) << 16i32;
    *negState = 2u32;
    *minstat = 0u32;
    supportedMech = 0 as crate::gssapi_h::gss_OID;
    *return_token = crate::gssapiP_spnego_h::ERROR_TOKEN_SEND;
    *mechListMIC = 0 as crate::gssapi_h::gss_buffer_t;
    *responseToken = *mechListMIC;
    bufstart = (*buf).value as *mut u8;
    ptr = bufstart;
    if (*buf)
        .length
        .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize)
        == 0usize
        || (*buf)
            .length
            .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize)
            > 2147483647i32 as usize
    {
        return (9u32) << 16i32;
    }
    /*
     * Attempt to work with old Sun SPNEGO.
     */
    if *ptr as i32 == 0x60i32 {
        ret = g_verify_token_header(
            gss_mech_spnego,
            &mut len,
            &mut ptr,
            0i32,
            (*buf)
                .length
                .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize) as u32,
        ) as crate::gssapi_h::OM_uint32;
        if ret != 0 {
            *minstat = ret;
            return (9u32) << 16i32;
        }
    }
    if *ptr as i32 != 0xa0i32 | 0x1i32 {
        return (9u32) << 16i32;
    }
    ret = get_negTokenResp(
        minstat,
        ptr,
        (*buf)
            .length
            .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize) as u32,
        negState,
        &mut supportedMech,
        responseToken,
        mechListMIC,
    );
    if !(ret != 0u32) {
        if (*responseToken).is_null() && (*mechListMIC).is_null() {
            ret = (9u32) << 16i32
        } else if !supportedMech.is_null() {
            ret = (9u32) << 16i32
        } else {
            (*sc).firstpass = 0i32;
            *negState = 1u32;
            *return_token = crate::gssapiP_spnego_h::CONT_TOKEN_SEND
        }
    }
    if !supportedMech.is_null() {
        crate::src::generic::oid_ops::generic_gss_release_oid(&mut tmpmin, &mut supportedMech);
    }
    return ret;
}
/*
 * Verify that mech OID is either exactly the same as the negotiated
 * mech OID, or is a mech OID supported by the negotiated mech.  MS
 * implementations can list a most preferred mech using an incorrect
 * krb5 OID while emitting a krb5 initiator mech token having the
 * correct krb5 mech OID.
 */

unsafe extern "C" fn acc_ctx_vfy_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut mechoid: crate::gssapi_h::gss_OID,
    mut negState: *mut crate::gssapi_h::OM_uint32,
    mut tokflag: *mut crate::gssapiP_spnego_h::send_token_flag,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as crate::mglueP_h::gss_mechanism;
    let mut mech_set = 0 as crate::gssapi_h::gss_OID_set;
    let mut present = 0i32;
    if (*(*sc).internal_mech).length == (*mechoid).length
        && crate::stdlib::memcmp(
            (*(*sc).internal_mech).elements,
            (*mechoid).elements,
            (*(*sc).internal_mech).length as usize,
        ) == 0i32
    {
        return 0u32;
    }
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        (*sc).internal_mech as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() || (*mech).gss_indicate_mechs.is_none() {
        *minor_status = 0x20000004u32;
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
        *negState = 2u32;
        *tokflag = crate::gssapiP_spnego_h::ERROR_TOKEN_SEND;
        return (1u32) << 16i32;
    }
    ret = (*mech)
        .gss_indicate_mechs
        .expect("non-null function pointer")(minor_status, &mut mech_set);
    if ret != 0u32 {
        *tokflag = crate::gssapiP_spnego_h::NO_TOKEN_SEND;
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        )
    } else {
        ret = crate::src::mechglue::g_oid_ops::gss_test_oid_set_member(
            minor_status,
            mechoid,
            mech_set,
            &mut present,
        );
        if !(ret != 0u32) {
            if present == 0 {
                *minor_status = 0x20000004u32;
                *minor_status =
                    crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
                *negState = 2u32;
                *tokflag = crate::gssapiP_spnego_h::ERROR_TOKEN_SEND;
                ret = (1u32) << 16i32
            }
        }
    }
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpmin, &mut mech_set);
    return ret;
}
/*
 * Wrap call to gss_accept_sec_context() and update state
 * accordingly.
 */

unsafe extern "C" fn acc_ctx_call_acc(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut spcred: crate::gssapiP_spnego_h::spnego_gss_cred_id_t,
    mut mechtok_in: crate::gssapi_h::gss_buffer_t,
    mut mechtok_out: crate::gssapi_h::gss_buffer_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut negState: *mut crate::gssapi_h::OM_uint32,
    mut tokflag: *mut crate::gssapiP_spnego_h::send_token_flag,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut mechoid = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    let mut mcred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut negoex = crate::src::mechglue::g_oid_ops::gss_oid_equal(
        (*sc).internal_mech as crate::gssapi_h::gss_const_OID,
        &mut negoex_mech as *mut crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_const_OID,
    );
    if (*sc).ctx_handle.is_null() && negoex == 0 {
        /*
         * mechoid is an alias; don't free it.
         */
        ret = crate::src::mechglue::g_glue::gssint_get_mech_type(&mut mechoid, mechtok_in);
        if ret != 0u32 {
            *tokflag = crate::gssapiP_spnego_h::NO_TOKEN_SEND;
            return ret;
        }
        ret = acc_ctx_vfy_oid(minor_status, sc, &mut mechoid, negState, tokflag);
        if ret != 0u32 {
            return ret;
        }
    }
    mcred = if spcred.is_null() {
        0 as crate::gssapi_h::gss_cred_id_t
    } else {
        (*spcred).mcred
    };
    crate::src::mechglue::g_rel_name::gss_release_name(&mut tmpmin, &mut (*sc).internal_name);
    crate::src::mechglue::g_rel_cred::gss_release_cred(&mut tmpmin, &mut (*sc).deleg_cred);
    if negoex != 0 {
        ret = crate::src::spnego::negoex_ctx::negoex_accept(
            minor_status,
            sc,
            mcred,
            mechtok_in,
            mechtok_out,
            time_rec,
        )
    } else {
        ret = crate::src::mechglue::g_accept_sec_context::gss_accept_sec_context(
            minor_status,
            &mut (*sc).ctx_handle,
            mcred,
            mechtok_in,
            0 as crate::gssapi_h::gss_channel_bindings_t,
            &mut (*sc).internal_name,
            &mut (*sc).actual_mech,
            mechtok_out,
            &mut (*sc).ctx_flags,
            time_rec,
            &mut (*sc).deleg_cred,
        )
    }
    if ret == 0u32 {
        (*sc).mech_complete = 1i32;
        if (*sc).mic_reqd == 0 || (*sc).ctx_flags & 32u32 == 0 {
            /* No MIC exchange required, so we're done. */
            *negState = 0u32;
            ret = 0u32
        } else {
            /* handle_mic will decide if we're done. */
            ret = ((1i32) << 0i32 + 0i32) as crate::gssapi_h::OM_uint32
        }
    } else if ret != ((1i32) << 0i32 + 0i32) as u32 {
        *negState = 2u32;
        *tokflag = crate::gssapiP_spnego_h::ERROR_TOKEN_SEND
    }
    return ret;
}
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_accept_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut verifier_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut _input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut src_name: *mut crate::gssapi_h::gss_name_t,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut delegated_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut negState: crate::gssapi_h::OM_uint32 = 0;
    let mut return_token = crate::gssapiP_spnego_h::NO_TOKEN_SEND;
    let mut mechtok_in = 0 as *mut crate::gssapi_h::gss_buffer_desc_struct;
    let mut mic_in = 0 as *mut crate::gssapi_h::gss_buffer_desc_struct;
    let mut mic_out = 0 as *mut crate::gssapi_h::gss_buffer_desc_struct;
    let mut mechtok_out = {
        let mut init = crate::gssapi_h::gss_buffer_desc_struct {
            length: 0usize,
            value: 0 as *mut libc::c_void,
        };
        init
    };
    let mut sc = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    let mut spcred = 0 as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    let mut sendTokenInit = 0i32;
    let mut tmpret: i32 = 0;
    mic_out = 0 as crate::gssapi_h::gss_buffer_t;
    mic_in = mic_out;
    mechtok_in = mic_in;
    /*
     * This function works in three steps:
     *
     *   1. Perform mechanism negotiation.
     *   2. Invoke the negotiated mech's gss_accept_sec_context function
     *      and examine the results.
     *   3. Process or generate MICs if necessary.
     *
     * Step one determines whether the negotiation requires a MIC exchange,
     * while steps two and three share responsibility for determining when
     * the exchange is complete.  If the selected mech completes in this
     * call and no MIC exchange is expected, then step 2 will decide.  If a
     * MIC exchange is expected, then step 3 will decide.  If an error
     * occurs in any step, the exchange will be aborted, possibly with an
     * error token.
     *
     * negState determines the state of the negotiation, and is
     * communicated to the acceptor if a continuing token is sent.
     * return_token is used to indicate what type of token, if any, should
     * be generated.
     */
    /* Validate arguments. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !output_token.is_null() {
        (*output_token).length = 0usize;
        (*output_token).value = 0 as *mut libc::c_void
    }
    if !src_name.is_null() {
        *src_name = 0 as crate::gssapi_h::gss_name_t
    }
    if !mech_type.is_null() {
        *mech_type = 0 as crate::gssapi_h::gss_OID
    }
    if !time_rec.is_null() {
        *time_rec = 0u32
    }
    if !ret_flags.is_null() {
        *ret_flags = 0u32
    }
    if !delegated_cred_handle.is_null() {
        *delegated_cred_handle = 0 as crate::gssapi_h::gss_cred_id_t
    }
    if minor_status.is_null() || output_token.is_null() || context_handle.is_null() {
        return (2u32) << 24i32;
    }
    if input_token.is_null() {
        return (1u32) << 24i32;
    }
    /* Step 1: Perform mechanism negotiation. */
    sc = *context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    spcred = verifier_cred_handle as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    if sc.is_null() && (*input_token).length == 0usize {
        /* Process a request for NegHints. */
        ret = acc_ctx_hints(
            minor_status,
            spcred,
            &mut mic_out,
            &mut negState,
            &mut return_token,
            &mut sc,
        );
        if ret != 0u32 {
            current_block = 15072255976349127597;
        } else {
            *context_handle = sc as crate::gssapi_h::gss_ctx_id_t;
            sendTokenInit = 1i32;
            ret = ((1i32) << 0i32 + 0i32) as crate::gssapi_h::OM_uint32;
            current_block = 3222590281903869779;
        }
    } else if sc.is_null() || (*sc).internal_mech.is_null() {
        if !sc.is_null() {
            /* Discard the context from the NegHints request. */
            release_spnego_ctx(&mut sc);
            *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t
        }
        /* Process an initial token; can set negState to
         * REQUEST_MIC. */
        ret = acc_ctx_new(
            minor_status,
            input_token,
            spcred,
            &mut mechtok_in,
            &mut mic_in,
            &mut negState,
            &mut return_token,
            &mut sc,
        );
        if ret != 0u32 {
            current_block = 15072255976349127597;
        } else {
            *context_handle = sc as crate::gssapi_h::gss_ctx_id_t;
            ret = ((1i32) << 0i32 + 0i32) as crate::gssapi_h::OM_uint32;
            current_block = 3222590281903869779;
        }
    } else {
        /* Process a response token.  Can set negState to
         * ACCEPT_INCOMPLETE. */
        ret = acc_ctx_cont(
            minor_status,
            input_token,
            sc,
            &mut mechtok_in,
            &mut mic_in,
            &mut negState,
            &mut return_token,
        );
        if ret != 0u32 {
            current_block = 15072255976349127597;
        } else {
            ret = ((1i32) << 0i32 + 0i32) as crate::gssapi_h::OM_uint32;
            current_block = 3222590281903869779;
        }
    }
    match current_block {
        3222590281903869779 => {
            /* Step 2: invoke the negotiated mechanism's gss_accept_sec_context
             * function. */
            /*
             * Handle mechtok_in and mic_in only if they are
             * present in input_token.  If neither is present, whether
             * this is an error depends on whether this is the first
             * round-trip.  RET is set to a default value according to
             * whether it is the first round-trip.
             */
            if negState != 3u32 && !mechtok_in.is_null() {
                ret = acc_ctx_call_acc(
                    minor_status,
                    sc,
                    spcred,
                    mechtok_in,
                    &mut mechtok_out,
                    time_rec,
                    &mut negState,
                    &mut return_token,
                )
            }
            /* Step 3: process or generate the MIC, if the negotiated mech is
             * complete and supports MICs. */
            if !(ret != 0u32 && ret != ((1i32) << 0i32 + 0i32) as u32)
                && (*sc).mech_complete != 0
                && (*sc).ctx_flags & 32u32 != 0
            {
                ret = handle_mic(
                    minor_status,
                    mic_in,
                    (mechtok_out.length != 0usize) as i32,
                    sc,
                    &mut mic_out,
                    &mut negState,
                    &mut return_token,
                )
            }
            if !(ret != 0u32 && ret != ((1i32) << 0i32 + 0i32) as u32) && !ret_flags.is_null() {
                *ret_flags = (*sc).ctx_flags & !(128i32) as u32
            }
        }
        _ => {}
    }
    if return_token == crate::gssapiP_spnego_h::INIT_TOKEN_SEND && sendTokenInit != 0 {
        if !sc.is_null() {
        } else {
            crate::stdlib::__assert_fail(b"sc != NULL\x00" as *const u8 as
                              *const i8,
                          b"spnego_mech.c\x00" as *const u8 as
                              *const i8,
                          1757u32,
                          (*::std::mem::transmute::<&[u8; 204],
                                                    &[i8; 204]>(b"OM_uint32 spnego_gss_accept_sec_context(OM_uint32 *, gss_ctx_id_t *, gss_cred_id_t, gss_buffer_t, gss_channel_bindings_t, gss_name_t *, gss_OID *, gss_buffer_t, OM_uint32 *, OM_uint32 *, gss_cred_id_t *)\x00")).as_ptr());
        }
        tmpret = make_spnego_tokenInit_msg(
            sc,
            1i32,
            mic_out,
            0u32,
            0 as crate::gssapi_h::gss_buffer_t,
            return_token,
            output_token,
        );
        if tmpret < 0i32 {
            ret = (13u32) << 16i32
        }
    } else if return_token != crate::gssapiP_spnego_h::NO_TOKEN_SEND
        && return_token != crate::gssapiP_spnego_h::CHECK_MIC
    {
        tmpret = make_spnego_tokenTarg_msg(
            negState,
            if !sc.is_null() {
                (*sc).internal_mech
            } else {
                0 as crate::gssapi_h::gss_OID
            },
            &mut mechtok_out,
            mic_out,
            return_token,
            output_token,
        );
        if tmpret < 0i32 {
            ret = (13u32) << 16i32
        }
    }
    if ret == 0u32 {
        (*sc).opened = 1i32;
        if !(*sc).internal_name.is_null() && !src_name.is_null() {
            *src_name = (*sc).internal_name;
            (*sc).internal_name = 0 as crate::gssapi_h::gss_name_t
        }
        if !mech_type.is_null() {
            *mech_type = (*sc).actual_mech
        }
        /* Get an updated lifetime if we didn't call into the mech. */
        if !time_rec.is_null() && *time_rec == 0u32 {
            crate::src::mechglue::g_context_time::gss_context_time(
                &mut tmpmin,
                (*sc).ctx_handle,
                time_rec,
            );
        }
        if !delegated_cred_handle.is_null() {
            *delegated_cred_handle = (*sc).deleg_cred;
            (*sc).deleg_cred = 0 as crate::gssapi_h::gss_cred_id_t
        }
    } else if ret != ((1i32) << 0i32 + 0i32) as u32 {
        if !sc.is_null() {
            crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context(
                &mut tmpmin,
                &mut (*sc).ctx_handle,
                0 as crate::gssapi_h::gss_buffer_t,
            );
            release_spnego_ctx(&mut sc);
        }
        *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t
    }
    crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, &mut mechtok_out);
    if !mechtok_in.is_null() {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, mechtok_in);
        crate::stdlib::free(mechtok_in as *mut libc::c_void);
    }
    if !mic_in.is_null() {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, mic_in);
        crate::stdlib::free(mic_in as *mut libc::c_void);
    }
    if !mic_out.is_null() {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, mic_out);
        crate::stdlib::free(mic_out as *mut libc::c_void);
    }
    return ret;
}

static mut msg_table: [C2RustUnnamed_12; 19] = [
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000001u32,
            msg: b"SPNEGO cannot find mechanisms to negotiate\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000002u32,
            msg: b"SPNEGO failed to acquire creds\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000003u32,
            msg: b"SPNEGO acceptor did not select a mechanism\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000004u32,
            msg: b"SPNEGO failed to negotiate a mechanism\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000005u32,
            msg: b"SPNEGO acceptor did not return a valid token\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000006u32,
            msg: b"Invalid NegoEx signature\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000007u32,
            msg: b"Invalid NegoEx message type\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000008u32,
            msg: b"Invalid NegoEx message size\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000009u32,
            msg: b"Invalid NegoEx conversation ID\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000010u32,
            msg: b"NegoEx authentication scheme not found\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000011u32,
            msg: b"Missing NegoEx negotiate message\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000012u32,
            msg: b"Missing NegoEx authentication protocol request message\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000013u32,
            msg: b"No mutually supported NegoEx authentication schemes\x00" as *const u8
                as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000014u32,
            msg: b"No NegoEx verify key\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000015u32,
            msg: b"Unknown NegoEx checksum scheme\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000016u32,
            msg: b"Invalid NegoEx checksum\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000017u32,
            msg: b"Unsupported critical NegoEx extension\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000018u32,
            msg: b"Unsupported NegoEx version\x00" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_12 {
            status: 0x20000019u32,
            msg: b"NegoEx message out of sequence\x00" as *const u8 as *const i8,
        };
        init
    },
];
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_display_status(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut status_value: crate::gssapi_h::OM_uint32,
    mut status_type: i32,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut message_context: *mut crate::gssapi_h::OM_uint32,
    mut status_string: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut maj = 0u32;
    let mut msg = 0 as *const i8;
    let mut i: crate::stddef_h::size_t = 0;
    let mut ret: i32 = 0;
    *message_context = 0u32;
    i = 0usize;
    while i
        < (::std::mem::size_of::<[C2RustUnnamed_12; 19]>())
            .wrapping_div(::std::mem::size_of::<C2RustUnnamed_12>())
    {
        if status_value == msg_table[i].status {
            msg = crate::stdlib::dgettext(
                b"mit-krb5\x00" as *const u8 as *const i8,
                msg_table[i].msg,
            );
            *status_string = make_err_msg(msg);
            return 0u32;
        }
        i = i.wrapping_add(1)
    }
    /* Not one of our minor codes; might be from a mech.  Call back
     * to gss_display_status, but first check for recursion. */
    if !crate::k5_thread_h::krb5int_getspecific(crate::k5_thread_h::K5_KEY_GSS_SPNEGO_STATUS)
        .is_null()
    {
        /* Perhaps we returned a com_err code like ENOMEM. */
        let mut err = crate::com_err_h::error_message(status_value as crate::com_err_h::errcode_t);
        *status_string = make_err_msg(err);
        return 0u32;
    }
    /* Set a non-null pointer value; doesn't matter which one. */
    ret = crate::k5_thread_h::krb5int_setspecific(
        crate::k5_thread_h::K5_KEY_GSS_SPNEGO_STATUS,
        &mut ret as *mut i32 as *mut libc::c_void,
    );
    if ret != 0i32 {
        *minor_status = ret as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    maj = crate::src::mechglue::g_dsp_status::gss_display_status(
        minor_status,
        status_value,
        status_type,
        mech_type,
        message_context,
        status_string,
    );
    /* This is unlikely to fail; not much we can do if it does. */
    crate::k5_thread_h::krb5int_setspecific(
        crate::k5_thread_h::K5_KEY_GSS_SPNEGO_STATUS,
        0 as *mut libc::c_void,
    );
    return maj;
}
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_import_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_name_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_name_type: crate::gssapi_h::gss_OID,
    mut output_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    status = crate::src::mechglue::g_imp_name::gss_import_name(
        minor_status,
        input_name_buffer,
        input_name_type,
        output_name,
    );
    return status;
}
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_release_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    status = crate::src::mechglue::g_rel_name::gss_release_name(minor_status, input_name);
    return status;
}
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_duplicate_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    input_name: crate::gssapi_h::gss_name_t,
    mut output_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    status =
        crate::src::mechglue::g_dup_name::gss_duplicate_name(minor_status, input_name, output_name);
    return status;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_inquire_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut name: *mut crate::gssapi_h::gss_name_t,
    mut lifetime: *mut crate::gssapi_h::OM_uint32,
    mut cred_usage: *mut i32,
    mut mechanisms: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut spcred = 0 as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    let mut creds = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut tmp_minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut initiator_lifetime: crate::gssapi_h::OM_uint32 = 0;
    let mut acceptor_lifetime: crate::gssapi_h::OM_uint32 = 0;
    /*
     * To avoid infinite recursion, if GSS_C_NO_CREDENTIAL is
     * supplied we call gss_inquire_cred_by_mech() on the
     * first non-SPNEGO mechanism.
     */
    spcred = cred_handle as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    if spcred.is_null() {
        status = get_available_mechs(
            minor_status,
            0 as crate::gssapi_h::gss_name_t,
            0i32,
            0 as crate::gssapi_ext_h::gss_const_key_value_set_t,
            &mut creds,
            mechanisms,
            0 as *mut crate::gssapi_h::OM_uint32,
        );
        if status != 0u32 {
            return status;
        }
        if (**mechanisms).count == 0usize {
            crate::src::mechglue::g_rel_cred::gss_release_cred(&mut tmp_minor_status, &mut creds);
            crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(
                &mut tmp_minor_status,
                mechanisms,
            );
            return (10u32) << 16i32;
        }
        if !(**mechanisms).elements.is_null() {
        } else {
            crate::stdlib::__assert_fail(b"(*mechanisms)->elements != NULL\x00" as *const u8
                              as *const i8,
                          b"spnego_mech.c\x00" as *const u8 as
                              *const i8,
                          2004u32,
                          (*::std::mem::transmute::<&[u8; 111],
                                                    &[i8; 111]>(b"OM_uint32 spnego_gss_inquire_cred(OM_uint32 *, gss_cred_id_t, gss_name_t *, OM_uint32 *, int *, gss_OID_set *)\x00")).as_ptr());
        }
        status = crate::src::mechglue::g_inq_cred::gss_inquire_cred_by_mech(
            minor_status,
            creds,
            &mut *(**mechanisms).elements.offset(0isize),
            name,
            &mut initiator_lifetime,
            &mut acceptor_lifetime,
            cred_usage,
        );
        if status != 0u32 {
            crate::src::mechglue::g_rel_cred::gss_release_cred(&mut tmp_minor_status, &mut creds);
            return status;
        }
        if !lifetime.is_null() {
            *lifetime = if *cred_usage == 2i32 {
                acceptor_lifetime
            } else {
                initiator_lifetime
            }
        }
        crate::src::mechglue::g_rel_cred::gss_release_cred(&mut tmp_minor_status, &mut creds);
    } else {
        status = crate::src::mechglue::g_inq_cred::gss_inquire_cred(
            minor_status,
            (*spcred).mcred,
            name,
            lifetime,
            cred_usage,
            mechanisms,
        )
    }
    return status;
}
/* LEAN_CLIENT */
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_compare_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    name1: crate::gssapi_h::gss_name_t,
    name2: crate::gssapi_h::gss_name_t,
    mut name_equal: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut status = 0u32;
    status = crate::src::mechglue::g_compare_name::gss_compare_name(
        minor_status,
        name1,
        name2,
        name_equal,
    );
    return status;
}
/*ARGSUSED*/
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_display_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_name: crate::gssapi_h::gss_name_t,
    mut output_name_buffer: crate::gssapi_h::gss_buffer_t,
    mut output_name_type: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    let mut status = 0u32;
    status = crate::src::mechglue::g_dsp_name::gss_display_name(
        minor_status,
        input_name,
        output_name_buffer,
        output_name_type,
    );
    return status;
}
/*ARGSUSED*/
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_inquire_names_for_mech(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mechanism: crate::gssapi_h::gss_OID,
    mut name_types: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    /*
     * We only know how to handle our own mechanism.
     */
    if !mechanism.is_null()
        && !((*gss_mech_spnego).length == (*mechanism).length
            && crate::stdlib::memcmp(
                (*gss_mech_spnego).elements,
                (*mechanism).elements,
                (*gss_mech_spnego).length as usize,
            ) == 0i32)
    {
        *minor_status = 0u32;
        return (13u32) << 16i32;
    }
    major = crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set(minor_status, name_types);
    if major == 0u32 {
        /* Now add our members. */
        major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
            minor_status,
            crate::src::generic::gssapi_generic::GSS_C_NT_USER_NAME,
            name_types,
        );
        if major == 0u32
            && {
                major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                    minor_status,
                    crate::src::generic::gssapi_generic::GSS_C_NT_MACHINE_UID_NAME,
                    name_types,
                );
                (major) == 0u32
            }
            && {
                major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                    minor_status,
                    crate::src::generic::gssapi_generic::GSS_C_NT_STRING_UID_NAME,
                    name_types,
                );
                (major) == 0u32
            }
        {
            major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                minor_status,
                crate::src::generic::gssapi_generic::GSS_C_NT_HOSTBASED_SERVICE,
                name_types,
            )
        }
        if major != 0u32 {
            crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut minor, name_types);
        }
    }
    return major;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_unwrap(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_unseal::gss_unwrap(
        minor_status,
        (*sc).ctx_handle,
        input_message_buffer,
        output_message_buffer,
        conf_state,
        qop_state,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_wrap(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_seal::gss_wrap(
        minor_status,
        (*sc).ctx_handle,
        conf_req_flag,
        qop_req,
        input_message_buffer,
        conf_state,
        output_message_buffer,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_process_context_token(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    token_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    /* SPNEGO doesn't have its own context tokens. */
    if (*sc).opened == 0 {
        return (9u32) << 16i32;
    }
    ret = crate::src::mechglue::g_process_context::gss_process_context_token(
        minor_status,
        (*sc).ctx_handle,
        token_buffer,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_delete_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut output_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret = 0u32;
    let mut ctx = context_handle as *mut crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    *minor_status = 0u32;
    if context_handle.is_null() {
        return (13u32) << 16i32;
    }
    if (*ctx).is_null() {
        return 0u32;
    }
    crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context(
        minor_status,
        &mut (**ctx).ctx_handle,
        output_token,
    );
    release_spnego_ctx(ctx);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_context_time(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_context_time::gss_context_time(
        minor_status,
        (*sc).ctx_handle,
        time_rec,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_export_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut interprocess_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = *(context_handle as *mut crate::gssapiP_spnego_h::spnego_gss_ctx_id_t);
    /* We don't currently support exporting partially established
     * contexts. */
    if (*sc).opened == 0 {
        return (16u32) << 16i32;
    }
    ret = crate::src::mechglue::g_exp_sec_context::gss_export_sec_context(
        minor_status,
        &mut (*sc).ctx_handle,
        interprocess_token,
    );
    if (*sc).ctx_handle.is_null() {
        release_spnego_ctx(&mut sc);
        *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_import_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    interprocess_token: crate::gssapi_h::gss_buffer_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut mctx = 0 as *mut crate::gssapi_h::gss_ctx_id_struct;
    let mut sc = 0 as *mut crate::gssapiP_spnego_h::spnego_ctx_st;
    let mut initiate: i32 = 0;
    let mut opened: i32 = 0;
    ret = crate::src::mechglue::g_imp_sec_context::gss_import_sec_context(
        minor_status,
        interprocess_token,
        &mut mctx,
    );
    if ret != 0u32 {
        return ret;
    }
    ret = crate::src::mechglue::g_inq_context::gss_inquire_context(
        &mut tmpmin,
        mctx,
        0 as *mut crate::gssapi_h::gss_name_t,
        0 as *mut crate::gssapi_h::gss_name_t,
        0 as *mut crate::gssapi_h::OM_uint32,
        0 as *mut crate::gssapi_h::gss_OID,
        0 as *mut crate::gssapi_h::OM_uint32,
        &mut initiate,
        &mut opened,
    );
    if ret != 0u32 || opened == 0 {
        /* We don't currently support importing partially established
         * contexts. */
        crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context(
            &mut tmpmin,
            &mut mctx,
            0 as crate::gssapi_h::gss_buffer_t,
        );
        return (13u32) << 16i32;
    }
    sc = create_spnego_ctx(initiate);
    if sc.is_null() {
        crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context(
            &mut tmpmin,
            &mut mctx,
            0 as crate::gssapi_h::gss_buffer_t,
        );
        return (13u32) << 16i32;
    }
    (*sc).ctx_handle = mctx;
    (*sc).opened = 1i32;
    *context_handle = sc as crate::gssapi_h::gss_ctx_id_t;
    return 0u32;
}
/* LEAN_CLIENT */
/* LEAN_CLIENT */
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_inquire_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut src_name: *mut crate::gssapi_h::gss_name_t,
    mut targ_name: *mut crate::gssapi_h::gss_name_t,
    mut lifetime_rec: *mut crate::gssapi_h::OM_uint32,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
    mut ctx_flags: *mut crate::gssapi_h::OM_uint32,
    mut locally_initiated: *mut i32,
    mut opened: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret = 0u32;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if !src_name.is_null() {
        *src_name = 0 as crate::gssapi_h::gss_name_t
    }
    if !targ_name.is_null() {
        *targ_name = 0 as crate::gssapi_h::gss_name_t
    }
    if !lifetime_rec.is_null() {
        *lifetime_rec = 0u32
    }
    if !mech_type.is_null() {
        *mech_type = gss_mech_spnego as crate::gssapi_h::gss_OID
    }
    if !ctx_flags.is_null() {
        *ctx_flags = 0u32
    }
    if !locally_initiated.is_null() {
        *locally_initiated = (*sc).initiate
    }
    if !opened.is_null() {
        *opened = (*sc).opened
    }
    if !(*sc).ctx_handle.is_null() {
        ret = crate::src::mechglue::g_inq_context::gss_inquire_context(
            minor_status,
            (*sc).ctx_handle,
            src_name,
            targ_name,
            lifetime_rec,
            mech_type,
            ctx_flags,
            0 as *mut i32,
            0 as *mut i32,
        )
    }
    if (*sc).opened == 0 {
        /*
         * We are still doing SPNEGO negotiation, so report SPNEGO as
         * the OID.  After negotiation is complete we will report the
         * underlying mechanism OID.
         */
        if !mech_type.is_null() {
            *mech_type = gss_mech_spnego as crate::gssapi_h::gss_OID
        }
        /*
         * Remove flags we don't support with partially-established
         * contexts.  (Change this to keep GSS_C_TRANS_FLAG if we add
         * support for exporting partial SPNEGO contexts.)
         */
        if !ctx_flags.is_null() {
            *ctx_flags &= !(128i32) as u32;
            *ctx_flags &= !(256i32) as u32
        }
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_wrap_size_limit(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut req_output_size: crate::gssapi_h::OM_uint32,
    mut max_input_size: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_seal::gss_wrap_size_limit(
        minor_status,
        (*sc).ctx_handle,
        conf_req_flag,
        qop_req,
        req_output_size,
        max_input_size,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_get_mic(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    message_buffer: crate::gssapi_h::gss_buffer_t,
    mut message_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_sign::gss_get_mic(
        minor_status,
        (*sc).ctx_handle,
        qop_req,
        message_buffer,
        message_token,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_verify_mic(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    msg_buffer: crate::gssapi_h::gss_buffer_t,
    token_buffer: crate::gssapi_h::gss_buffer_t,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_verify::gss_verify_mic(
        minor_status,
        (*sc).ctx_handle,
        msg_buffer,
        token_buffer,
        qop_state,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_inquire_sec_context_by_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    mut data_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    /* There are no SPNEGO-specific OIDs for this function. */
    if (*sc).ctx_handle.is_null() {
        return (16u32) << 16i32;
    }
    ret = crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid(
        minor_status,
        (*sc).ctx_handle,
        desired_object,
        data_set,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_inquire_cred_by_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    cred_handle: crate::gssapi_h::gss_cred_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    mut data_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut spcred = cred_handle as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    let mut mcred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    mcred = if spcred.is_null() {
        0 as crate::gssapi_h::gss_cred_id_t
    } else {
        (*spcred).mcred
    };
    ret = crate::src::mechglue::g_inq_cred_oid::gss_inquire_cred_by_oid(
        minor_status,
        mcred,
        desired_object,
        data_set,
    );
    return ret;
}

static mut no_ci_flags_oid: [crate::gssapi_h::gss_OID_desc; 1] = [{
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 6u32,
        elements: b"*\x85p+\r\x1d\x00" as *const u8 as *mut libc::c_void,
    };
    init
}];
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_set_cred_option(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmp_minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut spcred = *cred_handle as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    let mut mcred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    mcred = if spcred.is_null() {
        0 as crate::gssapi_h::gss_cred_id_t
    } else {
        (*spcred).mcred
    };
    ret = crate::src::mechglue::g_set_cred_option::gss_set_cred_option(
        minor_status,
        &mut mcred,
        desired_object,
        value,
    );
    if ret == 0u32 && spcred.is_null() {
        /*
         * If the mechanism allocated a new credential handle, then
         * we need to wrap it up in an SPNEGO credential handle.
         */
        ret = create_spnego_cred(minor_status, mcred, &mut spcred);
        if ret != 0u32 {
            crate::src::mechglue::g_rel_cred::gss_release_cred(&mut tmp_minor_status, &mut mcred);
            return ret;
        }
        *cred_handle = spcred as crate::gssapi_h::gss_cred_id_t
    }
    if ret != 0u32 {
        return ret;
    }
    /* Recognize KRB5_NO_CI_FLAGS_X_OID and avoid asking for integrity. */
    if (*desired_object).length == (*no_ci_flags_oid.as_ptr()).length
        && crate::stdlib::memcmp(
            (*desired_object).elements,
            (*no_ci_flags_oid.as_ptr()).elements,
            (*desired_object).length as usize,
        ) == 0i32
    {
        (*spcred).no_ask_integ = 1i32
    }
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_set_sec_context_option(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = *context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    /* There are no SPNEGO-specific OIDs for this function, and we cannot
     * construct an empty SPNEGO context with it. */
    if sc.is_null() || (*sc).ctx_handle.is_null() {
        return (16u32) << 16i32;
    }
    ret = crate::src::mechglue::g_set_context_option::gss_set_sec_context_option(
        minor_status,
        &mut (*sc).ctx_handle,
        desired_object,
        value,
    );
    return ret;
}
/* _GSS_STATIC_LINK */
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_wrap_aead(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut input_assoc_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_payload_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_wrap_aead::gss_wrap_aead(
        minor_status,
        (*sc).ctx_handle,
        conf_req_flag,
        qop_req,
        input_assoc_buffer,
        input_payload_buffer,
        conf_state,
        output_message_buffer,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_unwrap_aead(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_assoc_buffer: crate::gssapi_h::gss_buffer_t,
    mut output_payload_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_unwrap_aead::gss_unwrap_aead(
        minor_status,
        (*sc).ctx_handle,
        input_message_buffer,
        input_assoc_buffer,
        output_payload_buffer,
        conf_state,
        qop_state,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_wrap_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut conf_state: *mut i32,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_wrap_iov::gss_wrap_iov(
        minor_status,
        (*sc).ctx_handle,
        conf_req_flag,
        qop_req,
        conf_state,
        iov,
        iov_count,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_unwrap_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_unwrap_iov::gss_unwrap_iov(
        minor_status,
        (*sc).ctx_handle,
        conf_state,
        qop_state,
        iov,
        iov_count,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_wrap_iov_length(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut conf_state: *mut i32,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_wrap_iov::gss_wrap_iov_length(
        minor_status,
        (*sc).ctx_handle,
        conf_req_flag,
        qop_req,
        conf_state,
        iov,
        iov_count,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_complete_auth_token(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (16u32) << 16i32;
    }
    ret = crate::src::mechglue::g_complete_auth_token::gss_complete_auth_token(
        minor_status,
        (*sc).ctx_handle,
        input_message_buffer,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_acquire_cred_impersonate_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    impersonator_cred_handle: crate::gssapi_h::gss_cred_id_t,
    desired_name: crate::gssapi_h::gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut _desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut amechs = 0 as crate::gssapi_h::gss_OID_set;
    let mut imp_spcred = 0 as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    let mut out_spcred = 0 as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    let mut imp_mcred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut out_mcred = 0 as crate::gssapi_h::gss_cred_id_t;
    if !actual_mechs.is_null() {
        *actual_mechs = 0 as crate::gssapi_h::gss_OID_set
    }
    if !time_rec.is_null() {
        *time_rec = 0u32
    }
    imp_spcred = impersonator_cred_handle as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    imp_mcred = if !imp_spcred.is_null() {
        (*imp_spcred).mcred
    } else {
        0 as crate::gssapi_h::gss_cred_id_t
    };
    status = crate::src::mechglue::g_inq_cred::gss_inquire_cred(
        minor_status,
        imp_mcred,
        0 as *mut crate::gssapi_h::gss_name_t,
        0 as *mut crate::gssapi_h::OM_uint32,
        0 as *mut crate::gssapi_h::gss_cred_usage_t,
        &mut amechs,
    );
    if status != 0u32 {
        return status;
    }
    status = crate::src::mechglue::g_acquire_cred_imp_name::gss_acquire_cred_impersonate_name(
        minor_status,
        imp_mcred,
        desired_name,
        time_req,
        amechs,
        cred_usage,
        &mut out_mcred,
        actual_mechs,
        time_rec,
    );
    if !(status != 0u32) {
        status = create_spnego_cred(minor_status, out_mcred, &mut out_spcred);
        if !(status != 0u32) {
            out_mcred = 0 as crate::gssapi_h::gss_cred_id_t;
            *output_cred_handle = out_spcred as crate::gssapi_h::gss_cred_id_t
        }
    }
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpmin, &mut amechs);
    crate::src::mechglue::g_rel_cred::gss_release_cred(&mut tmpmin, &mut out_mcred);
    return status;
}
/* time_rec */
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_acquire_cred_with_password(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    desired_name: crate::gssapi_h::gss_name_t,
    password: crate::gssapi_h::gss_buffer_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    _desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut amechs = 0 as crate::gssapi_h::gss_OID_set;
    let mut mcred = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut spcred = 0 as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    if !actual_mechs.is_null() {
        *actual_mechs = 0 as crate::gssapi_h::gss_OID_set
    }
    if !time_rec.is_null() {
        *time_rec = 0u32
    }
    status = get_available_mechs(
        minor_status,
        desired_name,
        cred_usage,
        0 as crate::gssapi_ext_h::gss_const_key_value_set_t,
        0 as *mut crate::gssapi_h::gss_cred_id_t,
        &mut amechs,
        0 as *mut crate::gssapi_h::OM_uint32,
    );
    if !(status != 0u32) {
        status = crate::src::mechglue::g_acquire_cred_with_pw::gss_acquire_cred_with_password(
            minor_status,
            desired_name,
            password,
            time_req,
            amechs,
            cred_usage,
            &mut mcred,
            actual_mechs,
            time_rec,
        );
        if !(status != 0u32) {
            status = create_spnego_cred(minor_status, mcred, &mut spcred);
            if !(status != 0u32) {
                mcred = 0 as crate::gssapi_h::gss_cred_id_t;
                *output_cred_handle = spcred as crate::gssapi_h::gss_cred_id_t
            }
        }
    }
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpmin, &mut amechs);
    crate::src::mechglue::g_rel_cred::gss_release_cred(&mut tmpmin, &mut mcred);
    return status;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_display_name_ext(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut display_as_name_type: crate::gssapi_h::gss_OID,
    mut display_name: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    ret = crate::src::mechglue::g_dsp_name_ext::gss_display_name_ext(
        minor_status,
        name,
        display_as_name_type,
        display_name,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_inquire_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut name_is_MN: *mut i32,
    mut MN_mech: *mut crate::gssapi_h::gss_OID,
    mut attrs: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    ret = crate::src::mechglue::g_inq_name::gss_inquire_name(
        minor_status,
        name,
        name_is_MN,
        MN_mech,
        attrs,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_get_name_attribute(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut attr: crate::gssapi_h::gss_buffer_t,
    mut authenticated: *mut i32,
    mut complete: *mut i32,
    mut value: crate::gssapi_h::gss_buffer_t,
    mut display_value: crate::gssapi_h::gss_buffer_t,
    mut more: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    ret = crate::src::mechglue::g_get_name_attr::gss_get_name_attribute(
        minor_status,
        name,
        attr,
        authenticated,
        complete,
        value,
        display_value,
        more,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_set_name_attribute(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut complete: i32,
    mut attr: crate::gssapi_h::gss_buffer_t,
    mut value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    ret = crate::src::mechglue::g_set_name_attr::gss_set_name_attribute(
        minor_status,
        name,
        complete,
        attr,
        value,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_delete_name_attribute(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut attr: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    ret =
        crate::src::mechglue::g_del_name_attr::gss_delete_name_attribute(minor_status, name, attr);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_export_name_composite(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut exp_composite_name: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    ret = crate::src::mechglue::g_export_name_comp::gss_export_name_composite(
        minor_status,
        name,
        exp_composite_name,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_map_name_to_any(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut authenticated: i32,
    mut type_id: crate::gssapi_h::gss_buffer_t,
    mut output: *mut crate::gssapi_ext_h::gss_any_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    ret = crate::src::mechglue::g_map_name_to_any::gss_map_name_to_any(
        minor_status,
        name,
        authenticated,
        type_id,
        output,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_release_any_name_mapping(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut type_id: crate::gssapi_h::gss_buffer_t,
    mut input: *mut crate::gssapi_ext_h::gss_any_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    ret = crate::src::mechglue::g_rel_name_mapping::gss_release_any_name_mapping(
        minor_status,
        name,
        type_id,
        input,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_pseudo_random(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context: crate::gssapi_h::gss_ctx_id_t,
    mut prf_key: i32,
    prf_in: crate::gssapi_h::gss_buffer_t,
    mut desired_output_len: crate::stdlib::ssize_t,
    mut prf_out: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut sc = context as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    ret = crate::src::mechglue::g_prf::gss_pseudo_random(
        minor_status,
        (*sc).ctx_handle,
        prf_key,
        prf_in,
        desired_output_len,
        prf_out,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_set_neg_mechs(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: crate::gssapi_h::gss_cred_id_t,
    mech_list: crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut spcred = cred_handle as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    /* Store mech_list in spcred for use in negotiation logic. */
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(
        minor_status,
        &mut (*spcred).neg_mechs,
    );
    ret = crate::src::generic::oid_ops::generic_gss_copy_oid_set(
        minor_status,
        mech_list as *const crate::gssapi_h::gss_OID_set_desc,
        &mut (*spcred).neg_mechs,
    );
    if ret == 0u32 {
        crate::src::mechglue::g_set_neg_mechs::gss_set_neg_mechs(
            minor_status,
            (*spcred).mcred,
            (*spcred).neg_mechs,
        );
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_inquire_mech_for_saslname(
    mut _minor_status: *mut crate::gssapi_h::OM_uint32,
    sasl_mech_name: crate::gssapi_h::gss_buffer_t,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    if (*sasl_mech_name).length == (::std::mem::size_of::<[i8; 7]>()).wrapping_sub(1usize)
        && crate::stdlib::memcmp(
            (*sasl_mech_name).value,
            b"SPNEGO\x00" as *const u8 as *const libc::c_void,
            (::std::mem::size_of::<[i8; 7]>()).wrapping_sub(1usize),
        ) == 0i32
    {
        if !mech_type.is_null() {
            *mech_type = gss_mech_spnego as crate::gssapi_h::gss_OID
        }
        return 0u32;
    }
    return (1u32) << 16i32;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_inquire_saslname_for_mech(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    desired_mech: crate::gssapi_h::gss_OID,
    mut sasl_mech_name: crate::gssapi_h::gss_buffer_t,
    mut mech_name: crate::gssapi_h::gss_buffer_t,
    mut mech_description: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    *minor_status = 0u32;
    if !((*desired_mech).length == (*gss_mech_spnego).length
        && crate::stdlib::memcmp(
            (*desired_mech).elements,
            (*gss_mech_spnego).elements,
            (*desired_mech).length as usize,
        ) == 0i32)
    {
        return (1u32) << 16i32;
    }
    if crate::src::generic::util_buffer::gssint_g_make_string_buffer(
        b"SPNEGO\x00" as *const u8 as *const i8,
        sasl_mech_name,
    ) == 0
        || crate::src::generic::util_buffer::gssint_g_make_string_buffer(
            b"spnego\x00" as *const u8 as *const i8,
            mech_name,
        ) == 0
        || crate::src::generic::util_buffer::gssint_g_make_string_buffer(
            b"Simple and Protected GSS-API Negotiation Mechanism\x00" as *const u8 as *const i8,
            mech_description,
        ) == 0
    {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    } else {
        return 0u32;
    };
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_inquire_attrs_for_mech(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut _mech: crate::gssapi_h::gss_const_OID,
    mut mech_attrs: *mut crate::gssapi_h::gss_OID_set,
    mut _known_mech_attrs: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    /* known_mech_attrs is handled by mechglue */
    *minor_status = 0u32;
    if mech_attrs.is_null() {
        return 0u32;
    }
    major = crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set(minor_status, mech_attrs);
    if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
        major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
            minor_status,
            crate::src::generic::gssapi_generic::GSS_C_MA_MECH_NEGO as crate::gssapi_h::gss_OID,
            mech_attrs,
        );
        if !(major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
            major = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                minor_status,
                crate::src::generic::gssapi_generic::GSS_C_MA_ITOK_FRAMED
                    as crate::gssapi_h::gss_OID,
                mech_attrs,
            );
            (major & ((0o377u32) << 24i32 | (0o377u32) << 16i32)) != 0;
        }
    }
    if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpMinor, mech_attrs);
    }
    return major;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_export_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut spcred = cred_handle as crate::gssapiP_spnego_h::spnego_gss_cred_id_t;
    return crate::src::mechglue::g_export_cred::gss_export_cred(
        minor_status,
        (*spcred).mcred,
        token,
    );
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_import_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut token: crate::gssapi_h::gss_buffer_t,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut spcred = 0 as *mut crate::gssapiP_spnego_h::C2RustUnnamed_3;
    let mut mcred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    ret = crate::src::mechglue::g_imp_cred::gss_import_cred(minor_status, token, &mut mcred);
    if ret & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return ret;
    }
    ret = create_spnego_cred(minor_status, mcred, &mut spcred);
    if ret & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return ret;
    }
    *cred_handle = spcred as crate::gssapi_h::gss_cred_id_t;
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_get_mic_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::mechglue::g_wrap_iov::gss_get_mic_iov(
        minor_status,
        (*sc).ctx_handle,
        qop_req,
        iov,
        iov_count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_verify_mic_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::mechglue::g_unwrap_iov::gss_verify_mic_iov(
        minor_status,
        (*sc).ctx_handle,
        qop_state,
        iov,
        iov_count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn spnego_gss_get_mic_iov_length(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut sc = context_handle as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
    if (*sc).ctx_handle.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::mechglue::g_wrap_iov::gss_get_mic_iov_length(
        minor_status,
        (*sc).ctx_handle,
        qop_req,
        iov,
        iov_count,
    );
}
/*
 * We will release everything but the ctx_handle so that it
 * can be passed back to init/accept context. This routine should
 * not be called until after the ctx_handle memory is assigned to
 * the supplied context handle from init/accept context.
 */

unsafe extern "C" fn release_spnego_ctx(
    mut ctx: *mut crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
) {
    let mut context = 0 as *mut crate::gssapiP_spnego_h::spnego_ctx_st;
    let mut minor_stat: crate::gssapi_h::OM_uint32 = 0;
    context = *ctx;
    if !context.is_null() {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(
            &mut minor_stat,
            &mut (*context).DER_mechTypes,
        );
        crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(
            &mut minor_stat,
            &mut (*context).mech_set,
        );
        crate::src::mechglue::g_rel_name::gss_release_name(
            &mut minor_stat,
            &mut (*context).internal_name,
        );
        crate::src::mechglue::g_rel_cred::gss_release_cred(
            &mut minor_stat,
            &mut (*context).deleg_cred,
        );
        crate::src::spnego::negoex_util::negoex_release_context(context);
        crate::stdlib::free(context as *mut libc::c_void);
        *ctx = 0 as crate::gssapiP_spnego_h::spnego_gss_ctx_id_t
    };
}
/*
 * Can't use gss_indicate_mechs by itself to get available mechs for
 * SPNEGO because it will also return the SPNEGO mech and we do not
 * want to consider SPNEGO as an available security mech for
 * negotiation. For this reason, get_available_mechs will return
 * all available, non-deprecated mechs except SPNEGO and NegoEx-
 * only mechanisms.
 *
 * Note that gss_acquire_cred_from(GSS_C_NO_OID_SET) will filter
 * out hidden (GSS_C_MA_NOT_INDICATED) mechanisms such as NegoEx, so
 * calling gss_indicate_mechs_by_attrs() also works around that.
 *
 * If a ptr to a creds list is given, this function will attempt
 * to acquire creds for the creds given and trim the list of
 * returned mechanisms to only those for which creds are valid.
 *
 */

unsafe extern "C" fn get_available_mechs(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut usage: crate::gssapi_h::gss_cred_usage_t,
    mut cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
    mut creds: *mut crate::gssapi_h::gss_cred_id_t,
    mut rmechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status = 0u32; /* Exclude ourselves */
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut mechs = 0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    let mut goodmechs = 0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    let mut except_attrs = crate::gssapi_h::gss_OID_set_desc {
        count: 0,
        elements: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
    };
    let mut attr_oids: [crate::gssapi_h::gss_OID_desc; 3] = [crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    }; 3];
    *rmechs = 0 as crate::gssapi_h::gss_OID_set;
    attr_oids[0usize] = *crate::src::generic::gssapi_generic::GSS_C_MA_DEPRECATED;
    attr_oids[1usize] = *crate::src::generic::gssapi_generic::GSS_C_MA_NOT_DFLT_MECH;
    attr_oids[2usize] = *crate::src::generic::gssapi_generic::GSS_C_MA_MECH_NEGO;
    except_attrs.count = (::std::mem::size_of::<[crate::gssapi_h::gss_OID_desc; 3]>())
        .wrapping_div(::std::mem::size_of::<crate::gssapi_h::gss_OID_desc>());
    except_attrs.elements = attr_oids.as_mut_ptr();
    major_status = crate::src::mechglue::g_mechattr::gss_indicate_mechs_by_attrs(
        minor_status,
        0 as crate::gssapi_h::gss_const_OID_set,
        &mut except_attrs as *mut crate::gssapi_h::gss_OID_set_desc
            as crate::gssapi_h::gss_const_OID_set,
        0 as crate::gssapi_h::gss_const_OID_set,
        &mut mechs,
    );
    /*
     * If the caller wanted a list of creds returned,
     * trim the list of mechanisms down to only those
     * for which the creds are valid.
     */
    if (*mechs).count > 0usize && major_status == 0u32 && !creds.is_null() {
        major_status = crate::src::mechglue::g_acquire_cred::gss_acquire_cred_from(
            minor_status,
            name,
            0xffffffffu32,
            mechs,
            usage,
            cred_store,
            creds,
            &mut goodmechs,
            time_rec,
        );
        /*
         * Drop the old list in favor of the new
         * "trimmed" list.
         */
        if major_status == 0u32 {
            crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpmin, &mut mechs);
            mechs = goodmechs
        }
    }
    if (*mechs).count > 0usize && major_status == 0u32 {
        *rmechs = mechs
    } else {
        crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpmin, &mut mechs);
        *minor_status = 0x20000001u32;
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
        if major_status == 0u32 {
            major_status = (13u32) << 16i32
        }
    }
    return major_status;
}
/* Return true if mech asserts the GSS_C_MA_NEGOEX_AND_SPNEGO attribute. */

unsafe extern "C" fn negoex_and_spnego(mut mech: crate::gssapi_h::gss_OID) -> i32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut attrs = 0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    let mut present: i32 = 0;
    ret = crate::src::mechglue::g_mechattr::gss_inquire_attrs_for_mech(
        &mut minor,
        mech as crate::gssapi_h::gss_const_OID,
        &mut attrs,
        0 as *mut crate::gssapi_h::gss_OID_set,
    );
    if ret != 0u32 || attrs.is_null() {
        return 0i32;
    }
    crate::src::generic::oid_ops::generic_gss_test_oid_set_member(
        &mut minor,
        crate::src::generic::gssapi_generic::GSS_C_MA_NEGOEX_AND_SPNEGO,
        attrs,
        &mut present,
    );
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut minor, &mut attrs);
    return present;
}
/*
 * Fill sc->mech_set with the SPNEGO-negotiable mechanism OIDs, and
 * sc->negoex_mechs with an entry for each NegoEx-negotiable mechanism.  Take
 * into account the mech set provided with gss_set_neg_mechs() if it exists.
 */

unsafe extern "C" fn get_negotiable_mechs(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut sc: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut spcred: crate::gssapiP_spnego_h::spnego_gss_cred_id_t,
    mut usage: crate::gssapi_h::gss_cred_usage_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut creds = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut cred_mechs = 0 as crate::gssapi_h::gss_OID_set;
    let mut mechs = 0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    let mut i: u32 = 0;
    let mut present: i32 = 0;
    let mut added_negoex = 0i32;
    let mut scheme: crate::gssapiP_negoex_h::auth_scheme = [0; 16];
    if !spcred.is_null() {
        /* Get the list of mechs in the mechglue cred. */
        ret = crate::src::mechglue::g_inq_cred::gss_inquire_cred(
            minor_status,
            (*spcred).mcred,
            0 as *mut crate::gssapi_h::gss_name_t,
            0 as *mut crate::gssapi_h::OM_uint32,
            0 as *mut crate::gssapi_h::gss_cred_usage_t,
            &mut cred_mechs,
        );
        if ret != 0u32 {
            return ret;
        }
    } else {
        /* Start with the list of available mechs. */
        ret = get_available_mechs(
            minor_status,
            0 as crate::gssapi_h::gss_name_t,
            usage,
            0 as crate::gssapi_ext_h::gss_const_key_value_set_t,
            &mut creds,
            &mut cred_mechs,
            0 as *mut crate::gssapi_h::OM_uint32,
        );
        if ret != 0u32 {
            return ret;
        }
        crate::src::mechglue::g_rel_cred::gss_release_cred(&mut tmpmin, &mut creds);
    }
    /* If gss_set_neg_mechs() was called, use that to determine the
     * iteration order.  Otherwise iterate over the credential mechs. */
    mechs = if !spcred.is_null() && !(*spcred).neg_mechs.is_null() {
        (*spcred).neg_mechs
    } else {
        cred_mechs
    };
    ret = crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set(
        minor_status,
        &mut (*sc).mech_set,
    );
    if !(ret != 0u32) {
        i = 0u32;
        loop {
            if !((i as usize) < (*mechs).count) {
                current_block = 17500079516916021833;
                break;
            }
            if mechs != cred_mechs {
                /* Intersect neg_mechs with cred_mechs. */
                crate::src::mechglue::g_oid_ops::gss_test_oid_set_member(
                    &mut tmpmin,
                    &mut *(*mechs).elements.offset(i as isize),
                    cred_mechs,
                    &mut present,
                );
                if present == 0 {
                    current_block = 13056961889198038528;
                } else {
                    current_block = 26972500619410423;
                }
            } else {
                current_block = 26972500619410423;
            }
            match current_block {
                26972500619410423 => {
                    /* Query the auth scheme to see if this is a NegoEx mech. */
                    ret = crate::src::mechglue::g_negoex::gssspi_query_mechanism_info(
                        &mut tmpmin,
                        &mut *(*mechs).elements.offset(i as isize)
                            as *mut crate::gssapi_h::gss_OID_desc_struct
                            as crate::gssapi_h::gss_const_OID,
                        scheme.as_mut_ptr(),
                    );
                    if ret == 0u32 {
                        /* Add an entry for this mech to the NegoEx list. */
                        ret = crate::src::spnego::negoex_util::negoex_add_auth_mech(
                            minor_status,
                            sc,
                            &mut *(*mechs).elements.offset(i as isize)
                                as *mut crate::gssapi_h::gss_OID_desc_struct
                                as crate::gssapi_h::gss_const_OID,
                            scheme.as_mut_ptr(),
                        );
                        if ret != 0u32 {
                            current_block = 4395529121510314304;
                            break;
                        }
                        /* Add the NegoEx OID to the SPNEGO list at the
                         * position of the first NegoEx mechanism. */
                        if added_negoex == 0 {
                            ret = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                                minor_status,
                                &mut negoex_mech,
                                &mut (*sc).mech_set,
                            );
                            if ret != 0u32 {
                                current_block = 4395529121510314304;
                                break;
                            }
                            added_negoex = 1i32
                        }
                        /* Skip this mech in the SPNEGO list unless it asks for
                         * direct SPNEGO negotiation. */
                        if negoex_and_spnego(&mut *(*mechs).elements.offset(i as isize)) == 0 {
                            current_block = 13056961889198038528;
                        } else {
                            current_block = 17788412896529399552;
                        }
                    } else {
                        current_block = 17788412896529399552;
                    }
                    match current_block {
                        13056961889198038528 => {}
                        _ => {
                            /* Add this mech to the SPNEGO list. */
                            ret = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                                minor_status,
                                &mut *(*mechs).elements.offset(i as isize),
                                &mut (*sc).mech_set,
                            );
                            if ret != 0u32 {
                                current_block = 4395529121510314304;
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
            i = i.wrapping_add(1)
        }
        match current_block {
            4395529121510314304 => {}
            _ => *minor_status = 0u32,
        }
    }
    if ret != 0u32 || (*(*sc).mech_set).count == 0usize {
        *minor_status = 0x20000001u32;
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
        ret = (13u32) << 16i32
    }
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpmin, &mut cred_mechs);
    return ret;
}
/* following are token creation and reading routines */
/*
 * If buff_in is not pointing to a MECH_OID, then return NULL and do not
 * advance the buffer, otherwise, decode the mech_oid from the buffer and
 * place in gss_OID.
 */

unsafe extern "C" fn get_mech_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut buff_in: *mut *mut u8,
    mut length: crate::stddef_h::size_t,
) -> crate::gssapi_h::gss_OID {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut toid = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    let mut mech_out = 0 as crate::gssapi_h::gss_OID;
    let mut start = 0 as *mut u8;
    let mut end = 0 as *mut u8;
    if length < 1usize || **buff_in as i32 != 0x6i32 {
        return 0 as crate::gssapi_h::gss_OID;
    }
    start = *buff_in;
    end = start.offset(length as isize);
    *buff_in = (*buff_in).offset(1);
    let fresh3 = *buff_in;
    *buff_in = (*buff_in).offset(1);
    toid.length = *fresh3 as crate::gssapi_h::OM_uint32;
    if (*buff_in).offset(toid.length as isize) > end {
        return 0 as crate::gssapi_h::gss_OID;
    }
    toid.elements = *buff_in as *mut libc::c_void;
    *buff_in = (*buff_in).offset(toid.length as isize);
    status =
        crate::src::generic::oid_ops::generic_gss_copy_oid(minor_status, &mut toid, &mut mech_out);
    if status != 0u32 {
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
        mech_out = 0 as crate::gssapi_h::gss_OID
    }
    return mech_out;
}
/*
 * der encode the given mechanism oid into buf_out, advancing the
 * buffer pointer.
 */

unsafe extern "C" fn put_mech_oid(
    mut buf_out: *mut *mut u8,
    mut mech: gss_OID_const,
    mut buflen: u32,
) -> i32 {
    if buflen < (*mech).length.wrapping_add(2u32) {
        return -(1i32);
    }
    let fresh4 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh4 = 0x6u8;
    let fresh5 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh5 = (*mech).length as u8;
    crate::stdlib::memcpy(
        *buf_out as *mut libc::c_void,
        (*mech).elements,
        (*mech).length as usize,
    );
    *buf_out = (*buf_out).offset((*mech).length as isize);
    return 0i32;
}
/*
 * verify that buff_in points to an octet string, if it does not,
 * return NULL and don't advance the pointer. If it is an octet string
 * decode buff_in into a gss_buffer_t and return it, advancing the
 * buffer pointer.
 */

unsafe extern "C" fn get_input_token(
    mut buff_in: *mut *mut u8,
    mut buff_length: u32,
) -> crate::gssapi_h::gss_buffer_t {
    let mut input_token = 0 as *mut crate::gssapi_h::gss_buffer_desc_struct;
    let mut len: u32 = 0;
    if g_get_tag_and_length(buff_in, 0x4i32, buff_length, &mut len) < 0i32 {
        return 0 as crate::gssapi_h::gss_buffer_t;
    }
    input_token = crate::stdlib::malloc(::std::mem::size_of::<crate::gssapi_h::gss_buffer_desc>())
        as crate::gssapi_h::gss_buffer_t;
    if input_token.is_null() {
        return 0 as crate::gssapi_h::gss_buffer_t;
    }
    (*input_token).length = len as crate::stddef_h::size_t;
    if (*input_token).length > 0usize {
        (*input_token).value = gssalloc_malloc((*input_token).length);
        if (*input_token).value.is_null() {
            crate::stdlib::free(input_token as *mut libc::c_void);
            return 0 as crate::gssapi_h::gss_buffer_t;
        }
        crate::stdlib::memcpy(
            (*input_token).value,
            *buff_in as *const libc::c_void,
            (*input_token).length,
        );
    } else {
        (*input_token).value = 0 as *mut libc::c_void
    }
    *buff_in = (*buff_in).offset((*input_token).length as isize);
    return input_token;
}
/*
 * verify that the input token length is not 0. If it is, just return.
 * If the token length is greater than 0, der encode as an octet string
 * and place in buf_out, advancing buf_out.
 */

unsafe extern "C" fn put_input_token(
    mut buf_out: *mut *mut u8,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut buflen: u32,
) -> i32 {
    let mut ret: i32 = 0;
    /* if token length is 0, we do not want to send */
    if (*input_token).length == 0usize {
        return 0i32;
    }
    if (*input_token).length > buflen as usize {
        return -(1i32);
    }
    let fresh6 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh6 = 0x4u8;
    ret = crate::src::mechglue::g_glue::gssint_put_der_length(
        (*input_token).length as u32,
        buf_out,
        (*input_token).length as u32,
    );
    if ret != 0 {
        return ret;
    }
    crate::stdlib::memcpy(
        *buf_out as *mut libc::c_void,
        (*input_token).value,
        (*input_token).length,
    );
    *buf_out = (*buf_out).offset((*input_token).length as isize);
    return 0i32;
}
/*
 * verify that buff_in points to a sequence of der encoding. The mech
 * set is the only sequence of encoded object in the token, so if it is
 * a sequence of encoding, decode the mechset into a gss_OID_set and
 * return it, advancing the buffer pointer.
 */

unsafe extern "C" fn get_mech_set(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut buff_in: *mut *mut u8,
    mut buff_length: u32,
) -> crate::gssapi_h::gss_OID_set {
    let mut returned_mechSet = 0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut length: i32 = 0;
    let mut bytes: u32 = 0;
    let mut set_length: crate::gssapi_h::OM_uint32 = 0;
    let mut start = 0 as *mut u8;
    let mut i: i32 = 0;
    if **buff_in as i32 != 0x30i32 {
        return 0 as crate::gssapi_h::gss_OID_set;
    }
    start = *buff_in;
    *buff_in = (*buff_in).offset(1);
    length = crate::src::mechglue::g_glue::gssint_get_der_length(buff_in, buff_length, &mut bytes);
    if length < 0i32 || buff_length.wrapping_sub(bytes) < length as u32 {
        return 0 as crate::gssapi_h::gss_OID_set;
    }
    major_status = crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set(
        minor_status,
        &mut returned_mechSet,
    );
    if major_status != 0u32 {
        return 0 as crate::gssapi_h::gss_OID_set;
    }
    set_length = 0u32;
    i = 0i32;
    while set_length < length as u32 {
        let mut temp = get_mech_oid(
            minor_status,
            buff_in,
            (buff_length as isize - (*buff_in).wrapping_offset_from(start))
                as crate::stddef_h::size_t,
        );
        if temp.is_null() {
            break;
        }
        major_status = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
            minor_status,
            temp,
            &mut returned_mechSet,
        );
        if major_status == 0u32 {
            set_length = (set_length).wrapping_add(
                (*(*returned_mechSet).elements.offset(i as isize))
                    .length
                    .wrapping_add(2u32),
            );
            if crate::src::generic::oid_ops::generic_gss_release_oid(minor_status, &mut temp) != 0 {
                *minor_status =
                    crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
            }
        }
        i += 1
    }
    return returned_mechSet;
}
/*
 * Encode mechSet into buf.
 */

unsafe extern "C" fn put_mech_set(
    mut mechSet: crate::gssapi_h::gss_OID_set,
    mut buf: crate::gssapi_h::gss_buffer_t,
) -> i32 {
    let mut ptr = 0 as *mut u8;
    let mut i: u32 = 0;
    let mut tlen: u32 = 0;
    let mut ilen: u32 = 0;
    ilen = 0u32;
    tlen = ilen;
    i = 0u32;
    while (i as usize) < (*mechSet).count {
        /*
         * 0x06 [DER LEN] [OID]
         */
        ilen = ilen.wrapping_add(
            (1u32)
                .wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
                    (*(*mechSet).elements.offset(i as isize)).length,
                ))
                .wrapping_add((*(*mechSet).elements.offset(i as isize)).length),
        );
        i = i.wrapping_add(1)
    }
    /*
     * 0x30 [DER LEN]
     */
    tlen = (1u32)
        .wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(ilen))
        .wrapping_add(ilen);
    ptr = gssalloc_malloc(tlen as crate::stddef_h::size_t) as *mut u8;
    if ptr.is_null() {
        return -(1i32);
    }
    (*buf).value = ptr as *mut libc::c_void;
    (*buf).length = tlen as crate::stddef_h::size_t;
    let fresh7 = ptr;
    ptr = ptr.offset(1);
    *fresh7 = 0x30u8;
    if crate::src::mechglue::g_glue::gssint_put_der_length(
        ilen,
        &mut ptr,
        (*buf)
            .length
            .wrapping_sub(((*buf).value as *mut u8).wrapping_offset_from(ptr) as usize)
            as u32,
    ) < 0i32
    {
        return -(1i32);
    }
    i = 0u32;
    while (i as usize) < (*mechSet).count {
        if put_mech_oid(
            &mut ptr,
            &mut *(*mechSet).elements.offset(i as isize)
                as *mut crate::gssapi_h::gss_OID_desc_struct as gss_OID_const,
            (*buf)
                .length
                .wrapping_sub(((*buf).value as *mut u8).wrapping_offset_from(ptr) as usize)
                as u32,
        ) < 0i32
        {
            return -(1i32);
        }
        i = i.wrapping_add(1)
    }
    return 0i32;
}
/*
 * Verify that buff_in is pointing to a BIT_STRING with the correct
 * length and padding for the req_flags. If it is, decode req_flags
 * and return them, otherwise, return NULL.
 */

unsafe extern "C" fn get_req_flags(
    mut buff_in: *mut *mut u8,
    mut bodysize: crate::gssapi_h::OM_uint32,
    mut req_flags: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut len: u32 = 0;
    if **buff_in as i32 != 0xa0i32 | 0x1i32 {
        return 0u32;
    }
    if g_get_tag_and_length(buff_in, 0xa0i32 | 0x1i32, bodysize, &mut len) < 0i32 {
        return (9u32) << 16i32;
    }
    let fresh8 = *buff_in;
    *buff_in = (*buff_in).offset(1);
    if *fresh8 as i32 != 0x3i32 {
        return (9u32) << 16i32;
    }
    let fresh9 = *buff_in;
    *buff_in = (*buff_in).offset(1);
    if *fresh9 as i32 != 0x2i32 {
        return (9u32) << 16i32;
    }
    let fresh10 = *buff_in;
    *buff_in = (*buff_in).offset(1);
    if *fresh10 as i32 != 0x1i32 {
        return (9u32) << 16i32;
    }
    let fresh11 = *buff_in;
    *buff_in = (*buff_in).offset(1);
    *req_flags = (*fresh11 as i32 >> 1i32) as crate::gssapi_h::OM_uint32;
    return 0u32;
}

unsafe extern "C" fn get_negTokenInit(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut buf: crate::gssapi_h::gss_buffer_t,
    mut der_mechSet: crate::gssapi_h::gss_buffer_t,
    mut mechSet: *mut crate::gssapi_h::gss_OID_set,
    mut req_flags: *mut crate::gssapi_h::OM_uint32,
    mut mechtok: *mut crate::gssapi_h::gss_buffer_t,
    mut mechListMIC: *mut crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut err: crate::gssapi_h::OM_uint32 = 0;
    let mut ptr = 0 as *mut u8;
    let mut bufstart = 0 as *mut u8;
    let mut len: u32 = 0;
    let mut tmpbuf = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    *minor_status = 0u32;
    (*der_mechSet).length = 0usize;
    (*der_mechSet).value = 0 as *mut libc::c_void;
    *mechSet = 0 as crate::gssapi_h::gss_OID_set;
    *req_flags = 0u32;
    *mechListMIC = 0 as crate::gssapi_h::gss_buffer_t;
    *mechtok = *mechListMIC;
    bufstart = (*buf).value as *mut u8;
    ptr = bufstart;
    if (*buf)
        .length
        .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize)
        > 2147483647i32 as usize
    {
        return (13u32) << 16i32;
    }
    err = g_verify_token_header(
        gss_mech_spnego,
        &mut len,
        &mut ptr,
        0i32,
        (*buf)
            .length
            .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize) as u32,
    ) as crate::gssapi_h::OM_uint32;
    if err != 0 {
        *minor_status = err;
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
        return (13u32) << 16i32;
    }
    *minor_status = g_verify_neg_token_init(
        &mut ptr,
        (*buf)
            .length
            .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize) as u32,
    ) as crate::gssapi_h::OM_uint32;
    if *minor_status != 0 {
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
        return (13u32) << 16i32;
    }
    /* alias into input_token */
    tmpbuf.value = ptr as *mut libc::c_void;
    tmpbuf.length = (*buf)
        .length
        .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize);
    *mechSet = get_mech_set(
        minor_status,
        &mut ptr,
        (*buf)
            .length
            .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize) as u32,
    );
    if (*mechSet).is_null() {
        return (13u32) << 16i32;
    }
    tmpbuf.length = ptr.wrapping_offset_from(tmpbuf.value as *mut u8) as crate::stddef_h::size_t;
    (*der_mechSet).value = gssalloc_malloc(tmpbuf.length);
    if (*der_mechSet).value.is_null() {
        return (13u32) << 16i32;
    }
    crate::stdlib::memcpy((*der_mechSet).value, tmpbuf.value, tmpbuf.length);
    (*der_mechSet).length = tmpbuf.length;
    err = get_req_flags(
        &mut ptr,
        (*buf)
            .length
            .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize)
            as crate::gssapi_h::OM_uint32,
        req_flags,
    );
    if err != 0u32 {
        return err;
    }
    if g_get_tag_and_length(
        &mut ptr,
        0xa0i32 | 0x2i32,
        (*buf)
            .length
            .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize) as u32,
        &mut len,
    ) >= 0i32
    {
        *mechtok = get_input_token(&mut ptr, len);
        if (*mechtok).is_null() {
            return (13u32) << 16i32;
        }
    }
    if g_get_tag_and_length(
        &mut ptr,
        0xa0i32 | 0x3i32,
        (*buf)
            .length
            .wrapping_sub(ptr.wrapping_offset_from(bufstart) as usize) as u32,
        &mut len,
    ) >= 0i32
    {
        *mechListMIC = get_input_token(&mut ptr, len);
        if (*mechListMIC).is_null() {
            return (13u32) << 16i32;
        }
    }
    return 0u32;
}

unsafe extern "C" fn get_negTokenResp(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut buf: *mut u8,
    mut buflen: u32,
    mut negState: *mut crate::gssapi_h::OM_uint32,
    mut supportedMech: *mut crate::gssapi_h::gss_OID,
    mut responseToken: *mut crate::gssapi_h::gss_buffer_t,
    mut mechListMIC: *mut crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ptr = 0 as *mut u8;
    let mut bufstart = 0 as *mut u8;
    let mut len: u32 = 0;
    let mut tmplen: i32 = 0;
    let mut tag: u32 = 0;
    let mut bytes: u32 = 0;
    *negState = 0xffffffffu32;
    *supportedMech = 0 as crate::gssapi_h::gss_OID;
    *mechListMIC = 0 as crate::gssapi_h::gss_buffer_t;
    *responseToken = *mechListMIC;
    bufstart = buf;
    ptr = bufstart;
    if g_get_tag_and_length(
        &mut ptr,
        0xa0i32 | 0x1i32,
        (buflen as isize - ptr.wrapping_offset_from(bufstart)) as u32,
        &mut len,
    ) < 0i32
    {
        return (9u32) << 16i32;
    }
    let fresh12 = ptr;
    ptr = ptr.offset(1);
    if *fresh12 as i32 == 0x30i32 {
        tmplen = crate::src::mechglue::g_glue::gssint_get_der_length(
            &mut ptr,
            (buflen as isize - ptr.wrapping_offset_from(bufstart)) as u32,
            &mut bytes,
        );
        if tmplen < 0i32 || (buflen as isize - ptr.wrapping_offset_from(bufstart)) < tmplen as isize
        {
            return (9u32) << 16i32;
        }
    }
    if (buflen as isize - ptr.wrapping_offset_from(bufstart)) < 1isize {
        tag = 0u32
    } else {
        let fresh13 = ptr;
        ptr = ptr.offset(1);
        tag = *fresh13 as u32
    }
    if tag == 0xa0u32 {
        tmplen = crate::src::mechglue::g_glue::gssint_get_der_length(
            &mut ptr,
            (buflen as isize - ptr.wrapping_offset_from(bufstart)) as u32,
            &mut bytes,
        );
        if tmplen < 0i32 || (buflen as isize - ptr.wrapping_offset_from(bufstart)) < tmplen as isize
        {
            return (9u32) << 16i32;
        }
        if g_get_tag_and_length(
            &mut ptr,
            0xai32,
            (buflen as isize - ptr.wrapping_offset_from(bufstart)) as u32,
            &mut len,
        ) < 0i32
        {
            return (9u32) << 16i32;
        }
        if len != 1u32 {
            return (9u32) << 16i32;
        }
        if (buflen as isize - ptr.wrapping_offset_from(bufstart)) < 1isize {
            return (9u32) << 16i32;
        }
        let fresh14 = ptr;
        ptr = ptr.offset(1);
        *negState = *fresh14 as crate::gssapi_h::OM_uint32;
        if (buflen as isize - ptr.wrapping_offset_from(bufstart)) < 1isize {
            tag = 0u32
        } else {
            let fresh15 = ptr;
            ptr = ptr.offset(1);
            tag = *fresh15 as u32
        }
    }
    if tag == (0xa0i32 | 0x1i32) as u32 {
        tmplen = crate::src::mechglue::g_glue::gssint_get_der_length(
            &mut ptr,
            (buflen as isize - ptr.wrapping_offset_from(bufstart)) as u32,
            &mut bytes,
        );
        if tmplen < 0i32 || (buflen as isize - ptr.wrapping_offset_from(bufstart)) < tmplen as isize
        {
            return (9u32) << 16i32;
        }
        *supportedMech = get_mech_oid(
            minor_status,
            &mut ptr,
            (buflen as isize - ptr.wrapping_offset_from(bufstart)) as crate::stddef_h::size_t,
        );
        if (*supportedMech).is_null() {
            return (9u32) << 16i32;
        }
        if (buflen as isize - ptr.wrapping_offset_from(bufstart)) < 1isize {
            tag = 0u32
        } else {
            let fresh16 = ptr;
            ptr = ptr.offset(1);
            tag = *fresh16 as u32
        }
    }
    if tag == (0xa0i32 | 0x2i32) as u32 {
        tmplen = crate::src::mechglue::g_glue::gssint_get_der_length(
            &mut ptr,
            (buflen as isize - ptr.wrapping_offset_from(bufstart)) as u32,
            &mut bytes,
        );
        if tmplen < 0i32 || (buflen as isize - ptr.wrapping_offset_from(bufstart)) < tmplen as isize
        {
            return (9u32) << 16i32;
        }
        *responseToken = get_input_token(
            &mut ptr,
            (buflen as isize - ptr.wrapping_offset_from(bufstart)) as u32,
        );
        if (*responseToken).is_null() {
            return (9u32) << 16i32;
        }
        if (buflen as isize - ptr.wrapping_offset_from(bufstart)) < 1isize {
            tag = 0u32
        } else {
            let fresh17 = ptr;
            ptr = ptr.offset(1);
            tag = *fresh17 as u32
        }
    }
    if tag == (0xa0i32 | 0x3i32) as u32 {
        tmplen = crate::src::mechglue::g_glue::gssint_get_der_length(
            &mut ptr,
            (buflen as isize - ptr.wrapping_offset_from(bufstart)) as u32,
            &mut bytes,
        );
        if tmplen < 0i32 || (buflen as isize - ptr.wrapping_offset_from(bufstart)) < tmplen as isize
        {
            return (9u32) << 16i32;
        }
        *mechListMIC = get_input_token(
            &mut ptr,
            (buflen as isize - ptr.wrapping_offset_from(bufstart)) as u32,
        );
        if (*mechListMIC).is_null() {
            return (9u32) << 16i32;
        }
        /* Handle Windows 2000 duplicate response token */
        if !(*responseToken).is_null()
            && (**responseToken).length == (**mechListMIC).length
            && crate::stdlib::memcmp(
                (**responseToken).value,
                (**mechListMIC).value,
                (**responseToken).length,
            ) == 0
        {
            let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
            crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, *mechListMIC);
            crate::stdlib::free(*mechListMIC as *mut libc::c_void);
            *mechListMIC = 0 as crate::gssapi_h::gss_buffer_t
        }
    }
    return 0u32;
}
/*
 * der encode the passed negResults as an ENUMERATED type and
 * place it in buf_out, advancing the buffer.
 */

unsafe extern "C" fn put_negResult(
    mut buf_out: *mut *mut u8,
    mut negResult: crate::gssapi_h::OM_uint32,
    mut buflen: u32,
) -> i32 {
    if buflen < 3u32 {
        return -(1i32);
    }
    let fresh18 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh18 = 0xau8;
    let fresh19 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh19 = 1u8;
    let fresh20 = *buf_out;
    *buf_out = (*buf_out).offset(1);
    *fresh20 = negResult as u8;
    return 0i32;
}
/*
 * This routine compares the recieved mechset to the mechset that
 * this server can support. It looks sequentially through the mechset
 * and the first one that matches what the server can support is
 * chosen as the negotiated mechanism. If one is found, negResult
 * is set to ACCEPT_INCOMPLETE if it's the first mech, REQUEST_MIC if
 * it's not the first mech, otherwise we return NULL and negResult
 * is set to REJECT. The returned pointer is an alias into
 * received->elements and should not be freed.
 *
 * NOTE: There is currently no way to specify a preference order of
 * mechanisms supported by the acceptor.
 */

unsafe extern "C" fn negotiate_mech(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut received: crate::gssapi_h::gss_OID_set,
    mut negResult: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::gss_OID {
    let mut i: crate::stddef_h::size_t = 0;
    let mut j: crate::stddef_h::size_t = 0;
    let mut wrong_krb5_oid: i32 = 0;
    i = 0usize;
    while i < (*received).count {
        let mut mech_oid: crate::gssapi_h::gss_OID = &mut *(*received).elements.offset(i as isize)
            as *mut crate::gssapi_h::gss_OID_desc_struct;
        /* Accept wrong mechanism OID from MS clients */
        wrong_krb5_oid = 0i32;
        if (*mech_oid).length == gss_mech_krb5_wrong_oid.length
            && crate::stdlib::memcmp(
                (*mech_oid).elements,
                gss_mech_krb5_wrong_oid.elements,
                (*mech_oid).length as usize,
            ) == 0i32
        {
            mech_oid = &gss_mech_krb5_oid as *const crate::gssapi_h::gss_OID_desc
                as crate::gssapi_h::gss_OID;
            wrong_krb5_oid = 1i32
        }
        j = 0usize;
        while j < (*(*ctx).mech_set).count {
            if (*mech_oid).length == (*(*(*ctx).mech_set).elements.offset(j as isize)).length
                && crate::stdlib::memcmp(
                    (*mech_oid).elements,
                    (*(*(*ctx).mech_set).elements.offset(j as isize)).elements,
                    (*mech_oid).length as usize,
                ) == 0i32
            {
                *negResult = if i == 0usize { 1i32 } else { 3i32 } as crate::gssapi_h::OM_uint32;
                return if wrong_krb5_oid != 0 {
                    &gss_mech_krb5_wrong_oid as *const crate::gssapi_h::gss_OID_desc
                        as crate::gssapi_h::gss_OID
                } else {
                    &mut *(*(*ctx).mech_set).elements.offset(j as isize)
                        as *mut crate::gssapi_h::gss_OID_desc_struct
                };
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    *negResult = 2u32;
    return 0 as crate::gssapi_h::gss_OID;
}
/* private routines for spnego_mechanism */
/*
 * the next two routines make a token buffer suitable for
 * spnego_gss_display_status. These currently take the string
 * in name and place it in the token. Eventually, if
 * spnego_gss_display_status returns valid error messages,
 * these routines will be changes to return the error string.
 */

unsafe extern "C" fn make_spnego_token(
    mut name: *const i8,
) -> crate::gssapiP_spnego_h::spnego_token_t {
    return gssalloc_strdup(name) as crate::gssapiP_spnego_h::spnego_token_t;
}

unsafe extern "C" fn make_err_msg(mut name: *const i8) -> crate::gssapi_h::gss_buffer_desc {
    let mut buffer = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    if name.is_null() {
        buffer.length = 0usize;
        buffer.value = 0 as *mut libc::c_void
    } else {
        buffer.length = crate::stdlib::strlen(name).wrapping_add(1usize);
        buffer.value = make_spnego_token(name)
    }
    return buffer;
}
/*
 * Create the client side spnego token passed back to gss_init_sec_context
 * and eventually up to the application program and over to the server.
 *
 * Use DER rules, definite length method per RFC 2478
 */

unsafe extern "C" fn make_spnego_tokenInit_msg(
    mut spnego_ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut negHintsCompat: i32,
    mut mechListMIC: crate::gssapi_h::gss_buffer_t,
    mut _req_flags: crate::gssapi_h::OM_uint32,
    mut data: crate::gssapi_h::gss_buffer_t,
    mut _sendtoken: crate::gssapiP_spnego_h::send_token_flag,
    mut outbuf: crate::gssapi_h::gss_buffer_t,
) -> i32 {
    let mut current_block: u64;
    let mut ret = 0i32;
    let mut tlen: u32 = 0;
    let mut dataLen = 0u32;
    let mut negTokenInitSize = 0u32;
    let mut negTokenInitSeqSize = 0u32;
    let mut negTokenInitContSize = 0u32;
    let mut rspTokenSize = 0u32;
    let mut mechListTokenSize = 0u32;
    let mut micTokenSize = 0u32;
    let mut t = 0 as *mut u8;
    let mut ptr = 0 as *mut u8;
    if outbuf.is_null() {
        return -(1i32);
    }
    (*outbuf).length = 0usize;
    (*outbuf).value = 0 as *mut libc::c_void;
    /* calculate the data length */
    /*
     * 0xa0 [DER LEN] [mechTypes]
     */
    mechListTokenSize = ((1u32).wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
        (*spnego_ctx).DER_mechTypes.length as u32,
    )) as usize)
        .wrapping_add((*spnego_ctx).DER_mechTypes.length) as u32;
    dataLen = dataLen.wrapping_add(mechListTokenSize);
    /*
     * If a token from gss_init_sec_context exists,
     * add the length of the token + the ASN.1 overhead
     */
    if !data.is_null() {
        /*
         * Encoded in final output as:
         * 0xa2 [DER LEN] 0x04 [DER LEN] [DATA]
         * -----s--------|--------s2----------
         */
        rspTokenSize = ((1u32).wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
            (*data).length as u32,
        )) as usize)
            .wrapping_add((*data).length) as u32;
        dataLen = dataLen.wrapping_add(
            (1u32)
                .wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
                    rspTokenSize,
                ))
                .wrapping_add(rspTokenSize),
        )
    }
    if !mechListMIC.is_null() {
        /*
         * Encoded in final output as:
         * 0xa3 [DER LEN] 0x04 [DER LEN] [DATA]
         *	--s--     -----tlen------------
         */
        micTokenSize = ((1u32).wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
            (*mechListMIC).length as u32,
        )) as usize)
            .wrapping_add((*mechListMIC).length) as u32;
        dataLen = dataLen.wrapping_add(
            (1u32)
                .wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
                    micTokenSize,
                ))
                .wrapping_add(micTokenSize),
        )
    }
    /*
     * Add size of DER encoding
     * [ SEQUENCE { MechTypeList | ReqFLags | Token | mechListMIC } ]
     *   0x30 [DER_LEN] [data]
     *
     */
    negTokenInitContSize = dataLen;
    negTokenInitSeqSize = (1u32)
        .wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
            dataLen,
        ))
        .wrapping_add(dataLen);
    dataLen = negTokenInitSeqSize;
    /*
     * negTokenInitSize indicates the bytes needed to
     * hold the ASN.1 encoding of the entire NegTokenInit
     * SEQUENCE.
     * 0xa0 [DER_LEN] + data
     *
     */
    negTokenInitSize = (1u32)
        .wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
            negTokenInitSeqSize,
        ))
        .wrapping_add(negTokenInitSeqSize);
    tlen = g_token_size(gss_mech_spnego, negTokenInitSize) as u32;
    t = gssalloc_malloc(tlen as crate::stddef_h::size_t) as *mut u8;
    if t.is_null() {
        return -(1i32);
    }
    ptr = t;
    /* create the message */
    ret = g_make_token_header(gss_mech_spnego, negTokenInitSize, &mut ptr, tlen); /* NegotiationToken identifier */
    if !(ret != 0) {
        let fresh21 = ptr; /* MechTypeList identifier */
        ptr = ptr.offset(1);
        *fresh21 = 0xa0u8;
        ret = crate::src::mechglue::g_glue::gssint_put_der_length(
            negTokenInitSeqSize,
            &mut ptr,
            tlen,
        );
        if !(ret != 0) {
            let fresh22 = ptr;
            ptr = ptr.offset(1);
            *fresh22 = 0x30u8;
            ret = crate::src::mechglue::g_glue::gssint_put_der_length(
                negTokenInitContSize,
                &mut ptr,
                tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
            );
            if !(ret != 0) {
                let fresh23 = ptr;
                ptr = ptr.offset(1);
                *fresh23 = (0xa0i32 | 0i32) as u8;
                ret = crate::src::mechglue::g_glue::gssint_put_der_length(
                    (*spnego_ctx).DER_mechTypes.length as u32,
                    &mut ptr,
                    tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                );
                if !(ret != 0) {
                    /* We already encoded the MechSetList */
                    crate::stdlib::memcpy(
                        ptr as *mut libc::c_void,
                        (*spnego_ctx).DER_mechTypes.value,
                        (*spnego_ctx).DER_mechTypes.length,
                    );
                    ptr = ptr.offset((*spnego_ctx).DER_mechTypes.length as isize);
                    if !data.is_null() {
                        let fresh24 = ptr;
                        ptr = ptr.offset(1);
                        *fresh24 = (0xa0i32 | 0x2i32) as u8;
                        ret = crate::src::mechglue::g_glue::gssint_put_der_length(
                            rspTokenSize,
                            &mut ptr,
                            tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                        );
                        if ret != 0 {
                            current_block = 4996352559381616237;
                        } else {
                            ret = put_input_token(
                                &mut ptr,
                                data,
                                tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                            );
                            if ret != 0 {
                                current_block = 4996352559381616237;
                            } else {
                                current_block = 980989089337379490;
                            }
                        }
                    } else {
                        current_block = 980989089337379490;
                    }
                    match current_block {
                        4996352559381616237 => {}
                        _ => {
                            if !mechListMIC.is_null() {
                                let fresh25 = ptr;
                                ptr = ptr.offset(1);
                                *fresh25 = (0xa0i32 | 0x3i32) as u8;
                                ret = crate::src::mechglue::g_glue::gssint_put_der_length(
                                    micTokenSize,
                                    &mut ptr,
                                    tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                                );
                                if !(ret != 0) {
                                    if negHintsCompat != 0 {
                                        ret = put_neg_hints(
                                            &mut ptr,
                                            mechListMIC,
                                            tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                                        );
                                        (ret) != 0;
                                    } else {
                                        ret = put_input_token(
                                            &mut ptr,
                                            mechListMIC,
                                            tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                                        );
                                        (ret) != 0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ret != 0i32 {
        if !t.is_null() {
            crate::stdlib::free(t as *mut libc::c_void);
        }
        t = 0 as *mut u8;
        tlen = 0u32
    }
    (*outbuf).length = tlen as crate::stddef_h::size_t;
    (*outbuf).value = t as *mut libc::c_void;
    return ret;
}
/*
 * create the server side spnego token passed back to
 * gss_accept_sec_context and eventually up to the application program
 * and over to the client.
 */

unsafe extern "C" fn make_spnego_tokenTarg_msg(
    mut status: crate::gssapi_h::OM_uint32,
    mut mech_wanted: crate::gssapi_h::gss_OID,
    mut data: crate::gssapi_h::gss_buffer_t,
    mut mechListMIC: crate::gssapi_h::gss_buffer_t,
    mut sendtoken: crate::gssapiP_spnego_h::send_token_flag,
    mut outbuf: crate::gssapi_h::gss_buffer_t,
) -> i32 {
    let mut current_block: u64;
    let mut tlen = 0u32;
    let mut ret = 0u32;
    let mut NegTokenTargSize = 0u32;
    let mut NegTokenSize = 0u32;
    let mut rspTokenSize = 0u32;
    let mut micTokenSize = 0u32;
    let mut dataLen = 0u32;
    let mut t = 0 as *mut u8;
    let mut ptr = 0 as *mut u8;
    if outbuf.is_null() {
        return ((9u32) << 16i32) as i32;
    }
    if sendtoken == crate::gssapiP_spnego_h::INIT_TOKEN_SEND && mech_wanted.is_null() {
        return ((9u32) << 16i32) as i32;
    }
    (*outbuf).length = 0usize;
    (*outbuf).value = 0 as *mut libc::c_void;
    /*
     * ASN.1 encoding of the negResult
     * ENUMERATED type is 3 bytes
     *  ENUMERATED TAG, Length, Value,
     * Plus 2 bytes for the CONTEXT id and length.
     */
    dataLen = 5u32;
    /*
     * calculate data length
     *
     * If this is the initial token, include length of
     * mech_type and the negotiation result fields.
     */
    if sendtoken == crate::gssapiP_spnego_h::INIT_TOKEN_SEND {
        let mut mechlistTokenSize: i32 = 0;
        /*
         * 1 byte for the CONTEXT ID(0xa0),
         * 1 byte for the OID ID(0x06)
         * 1 byte for OID Length field
         * Plus the rest... (OID Length, OID value)
         */
        mechlistTokenSize = (3u32).wrapping_add((*mech_wanted).length).wrapping_add(
            crate::src::mechglue::g_glue::gssint_der_length_size((*mech_wanted).length),
        ) as i32;
        dataLen = dataLen.wrapping_add(mechlistTokenSize as u32)
    }
    if !data.is_null() && (*data).length > 0usize {
        /* Length of the inner token */
        rspTokenSize = ((1u32).wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
            (*data).length as u32,
        )) as usize)
            .wrapping_add((*data).length) as u32;
        dataLen = dataLen.wrapping_add(rspTokenSize);
        /* Length of the outer token */
        dataLen = dataLen.wrapping_add((1u32).wrapping_add(
            crate::src::mechglue::g_glue::gssint_der_length_size(rspTokenSize),
        ))
    }
    if !mechListMIC.is_null() {
        /* Length of the inner token */
        micTokenSize = ((1u32).wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
            (*mechListMIC).length as u32,
        )) as usize)
            .wrapping_add((*mechListMIC).length) as u32;
        dataLen = dataLen.wrapping_add(micTokenSize);
        /* Length of the outer token */
        dataLen = dataLen.wrapping_add((1u32).wrapping_add(
            crate::src::mechglue::g_glue::gssint_der_length_size(micTokenSize),
        ))
    }
    /*
     * Add size of DER encoded:
     * NegTokenTarg [ SEQUENCE ] of
     *    NegResult[0] ENUMERATED {
     *	accept_completed(0),
     *	accept_incomplete(1),
     *	reject(2) }
     *    supportedMech [1] MechType OPTIONAL,
     *    responseToken [2] OCTET STRING OPTIONAL,
     *    mechListMIC   [3] OCTET STRING OPTIONAL
     *
     * size = data->length + MechListMic + SupportedMech len +
     *	Result Length + ASN.1 overhead
     */
    NegTokenTargSize = dataLen;
    dataLen = dataLen.wrapping_add((1u32).wrapping_add(
        crate::src::mechglue::g_glue::gssint_der_length_size(NegTokenTargSize),
    ));
    /*
     * NegotiationToken [ CHOICE ]{
     *    negTokenInit  [0]	 NegTokenInit,
     *    negTokenTarg  [1]	 NegTokenTarg }
     */
    NegTokenSize = dataLen;
    dataLen = dataLen.wrapping_add((1u32).wrapping_add(
        crate::src::mechglue::g_glue::gssint_der_length_size(NegTokenSize),
    ));
    tlen = dataLen;
    t = gssalloc_malloc(tlen as crate::stddef_h::size_t) as *mut u8;
    if t.is_null() {
        ret = (9u32) << 16i32
    } else {
        ptr = t;
        /*
         * Indicate that we are sending CHOICE 1
         * (NegTokenTarg)
         */
        let fresh26 = ptr;
        ptr = ptr.offset(1);
        *fresh26 = (0xa0i32 | 0x1i32) as u8;
        if crate::src::mechglue::g_glue::gssint_put_der_length(NegTokenSize, &mut ptr, dataLen)
            < 0i32
        {
            ret = (9u32) << 16i32
        } else {
            let fresh27 = ptr;
            ptr = ptr.offset(1);
            *fresh27 = 0x30u8;
            if crate::src::mechglue::g_glue::gssint_put_der_length(
                NegTokenTargSize,
                &mut ptr,
                tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
            ) < 0i32
            {
                ret = (9u32) << 16i32
            } else {
                /*
                 * First field of the NegTokenTarg SEQUENCE
                 * is the ENUMERATED NegResult.
                 */
                let fresh28 = ptr;
                ptr = ptr.offset(1);
                *fresh28 = 0xa0u8;
                if crate::src::mechglue::g_glue::gssint_put_der_length(
                    3u32,
                    &mut ptr,
                    tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                ) < 0i32
                {
                    ret = (9u32) << 16i32
                } else if put_negResult(
                    &mut ptr,
                    status,
                    tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                ) < 0i32
                {
                    ret = (9u32) << 16i32
                } else {
                    if sendtoken == crate::gssapiP_spnego_h::INIT_TOKEN_SEND {
                        /*
                         * Next, is the Supported MechType
                         */
                        let fresh29 = ptr;
                        ptr = ptr.offset(1);
                        *fresh29 = (0xa0i32 | 0x1i32) as u8;
                        if crate::src::mechglue::g_glue::gssint_put_der_length(
                            (*mech_wanted).length.wrapping_add(2u32),
                            &mut ptr,
                            tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                        ) < 0i32
                        {
                            ret = (9u32) << 16i32;
                            current_block = 8037724726832295043;
                        } else if put_mech_oid(
                            &mut ptr,
                            mech_wanted as gss_OID_const,
                            tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                        ) < 0i32
                        {
                            ret = (9u32) << 16i32;
                            current_block = 8037724726832295043;
                        } else {
                            current_block = 2516253395664191498;
                        }
                    } else {
                        current_block = 2516253395664191498;
                    }
                    match current_block {
                        8037724726832295043 => {}
                        _ => {
                            if !data.is_null() && (*data).length > 0usize {
                                let fresh30 = ptr;
                                ptr = ptr.offset(1);
                                *fresh30 = (0xa0i32 | 0x2i32) as u8;
                                if crate::src::mechglue::g_glue::gssint_put_der_length(
                                    rspTokenSize,
                                    &mut ptr,
                                    tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                                ) < 0i32
                                {
                                    ret = (9u32) << 16i32;
                                    current_block = 8037724726832295043;
                                } else if put_input_token(
                                    &mut ptr,
                                    data,
                                    tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                                ) < 0i32
                                {
                                    ret = (9u32) << 16i32;
                                    current_block = 8037724726832295043;
                                } else {
                                    current_block = 9512719473022792396;
                                }
                            } else {
                                current_block = 9512719473022792396;
                            }
                            match current_block {
                                8037724726832295043 => {}
                                _ => {
                                    if !mechListMIC.is_null() {
                                        let fresh31 = ptr;
                                        ptr = ptr.offset(1);
                                        *fresh31 = (0xa0i32 | 0x3i32) as u8;
                                        if crate::src::mechglue::g_glue::gssint_put_der_length(
                                            micTokenSize,
                                            &mut ptr,
                                            tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                                        ) < 0i32
                                        {
                                            ret = (9u32) << 16i32;
                                            current_block = 8037724726832295043;
                                        } else if put_input_token(
                                            &mut ptr,
                                            mechListMIC,
                                            tlen.wrapping_sub(ptr.wrapping_offset_from(t) as u32),
                                        ) < 0i32
                                        {
                                            ret = (9u32) << 16i32;
                                            current_block = 8037724726832295043;
                                        } else {
                                            current_block = 13660591889533726445;
                                        }
                                    } else {
                                        current_block = 13660591889533726445;
                                    }
                                    match current_block {
                                        8037724726832295043 => {}
                                        _ => ret = 0u32,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ret != 0u32 {
        if !t.is_null() {
            crate::stdlib::free(t as *mut libc::c_void);
        }
    } else {
        (*outbuf).length = ptr.wrapping_offset_from(t) as crate::stddef_h::size_t;
        (*outbuf).value = t as *mut libc::c_void
    }
    return ret as i32;
}
/* determine size of token */

unsafe extern "C" fn g_token_size(mut mech: gss_OID_const, mut body_size: u32) -> i32 {
    let mut hdrsize: i32 = 0;
    /*
     * Initialize the header size to the
     * MECH_OID byte + the bytes needed to indicate the
     * length of the OID + the OID itself.
     *
     * 0x06 [MECHLENFIELD] MECHDATA
     */
    hdrsize = (1u32)
        .wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
            (*mech).length,
        ))
        .wrapping_add((*mech).length) as i32;
    /*
     * Now add the bytes needed for the initial header
     * token bytes:
     * 0x60 + [DER_LEN] + HDRSIZE
     */
    hdrsize = (hdrsize as u32).wrapping_add((1u32).wrapping_add(
        crate::src::mechglue::g_glue::gssint_der_length_size(
            body_size.wrapping_add(hdrsize as u32),
        ),
    )) as i32;
    return (hdrsize as u32).wrapping_add(body_size) as i32;
}
/*
 * generate token header.
 *
 * Use DER Definite Length method per RFC2478
 * Use of indefinite length encoding will not be compatible
 * with Microsoft or others that actually follow the spec.
 */

unsafe extern "C" fn g_make_token_header(
    mut mech: gss_OID_const,
    mut body_size: u32,
    mut buf: *mut *mut u8,
    mut totallen: u32,
) -> i32 {
    let mut ret = 0i32;
    let mut hdrsize: u32 = 0;
    let mut p = *buf;
    hdrsize = (1u32)
        .wrapping_add(crate::src::mechglue::g_glue::gssint_der_length_size(
            (*mech).length,
        ))
        .wrapping_add((*mech).length);
    let fresh32 = *buf;
    *buf = (*buf).offset(1);
    *fresh32 = 0x60u8;
    ret = crate::src::mechglue::g_glue::gssint_put_der_length(
        hdrsize.wrapping_add(body_size),
        buf,
        totallen,
    );
    if ret != 0 {
        return ret;
    }
    let fresh33 = *buf;
    *buf = (*buf).offset(1);
    *fresh33 = 0x6u8;
    ret = crate::src::mechglue::g_glue::gssint_put_der_length(
        (*mech).length,
        buf,
        totallen.wrapping_sub(p.wrapping_offset_from(*buf) as u32),
    );
    if ret != 0 {
        return ret;
    }
    crate::stdlib::memcpy(
        *buf as *mut libc::c_void,
        (*mech).elements,
        (*mech).length as usize,
    );
    *buf = (*buf).offset((*mech).length as isize);
    return 0i32;
}
/*
 * NOTE: This checks that the length returned by
 * gssint_get_der_length() is not greater than the number of octets
 * remaining, even though gssint_get_der_length() already checks, in
 * theory.
 */

unsafe extern "C" fn g_get_tag_and_length(
    mut buf: *mut *mut u8,
    mut tag: i32,
    mut buflen: u32,
    mut outlen: *mut u32,
) -> i32 {
    let mut ptr = *buf; /* pessimists, assume failure ! */
    let mut ret = -(1i32);
    let mut encoded_len: u32 = 0;
    let mut tmplen = 0i32;
    *outlen = 0u32;
    if buflen > 1u32 && *ptr as i32 == tag {
        ptr = ptr.offset(1);
        tmplen = crate::src::mechglue::g_glue::gssint_get_der_length(
            &mut ptr,
            buflen.wrapping_sub(1u32),
            &mut encoded_len,
        );
        if tmplen < 0i32 {
            ret = -(1i32)
        } else if tmplen as isize > buflen as isize - ptr.wrapping_offset_from(*buf) {
            ret = -(1i32)
        } else {
            ret = 0i32
        }
    }
    *outlen = tmplen as u32;
    *buf = ptr;
    return ret;
}

unsafe extern "C" fn g_verify_neg_token_init(mut buf_in: *mut *mut u8, mut cur_size: u32) -> i32 {
    let mut buf = *buf_in;
    let mut endptr = buf.offset(cur_size as isize);
    let mut seqsize: i32 = 0;
    let mut ret = 0i32;
    let mut bytes: u32 = 0;
    /*
     * Verify this is a NegotiationToken type token
     * - check for a0(context specific identifier)
     * - get length and verify that enoughd ata exists
     */
    if g_get_tag_and_length(&mut buf, 0xa0i32, cur_size, &mut bytes) < 0i32 {
        return -(2045022964 as isize) as i32;
    } /* should indicate bytes remaining */
    cur_size = bytes;
    /*
     * Verify the next piece, it should identify this as
     * a strucure of type NegTokenInit.
     */
    let fresh34 = buf;
    buf = buf.offset(1);
    if *fresh34 as i32 == 0x30i32 {
        seqsize =
            crate::src::mechglue::g_glue::gssint_get_der_length(&mut buf, cur_size, &mut bytes);
        if seqsize < 0i32 {
            return -(2045022964 as isize) as i32;
        }
        /*
         * Make sure we have the entire buffer as described
         */
        if seqsize as isize > endptr.wrapping_offset_from(buf) {
            return -(2045022964 as isize) as i32;
        }
    } else {
        return -(2045022964 as isize) as i32;
    } /* should indicate bytes remaining */
    cur_size = seqsize as u32;
    /*
     * Verify that the first blob is a sequence of mechTypes
     */
    let fresh35 = buf;
    buf = buf.offset(1);
    if *fresh35 as i32 == 0xa0i32 {
        seqsize =
            crate::src::mechglue::g_glue::gssint_get_der_length(&mut buf, cur_size, &mut bytes);
        if seqsize < 0i32 {
            return -(2045022964 as isize) as i32;
        }
        /*
         * Make sure we have the entire buffer as described
         */
        if seqsize as isize > endptr.wrapping_offset_from(buf) {
            return -(2045022964 as isize) as i32;
        }
    } else {
        return -(2045022964 as isize) as i32;
    }
    /*
     * At this point, *buf should be at the beginning of the
     * DER encoded list of mech types that are to be negotiated.
     */
    *buf_in = buf;
    return ret;
}
/* verify token header. */

unsafe extern "C" fn g_verify_token_header(
    mut mech: gss_OID_const,
    mut body_size: *mut u32,
    mut buf_in: *mut *mut u8,
    mut _tok_type: i32,
    mut toksize: u32,
) -> i32 {
    let mut buf = *buf_in;
    let mut seqsize: i32 = 0;
    let mut toid = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    let mut ret = 0i32;
    let mut bytes: u32 = 0;
    let fresh36 = toksize;
    toksize = toksize.wrapping_sub(1);
    if fresh36 < 1u32 {
        return -(2045022964 as isize) as i32;
    }
    let fresh37 = buf;
    buf = buf.offset(1);
    if *fresh37 as i32 != 0x60i32 {
        return -(2045022964 as isize) as i32;
    }
    seqsize = crate::src::mechglue::g_glue::gssint_get_der_length(&mut buf, toksize, &mut bytes);
    if seqsize < 0i32 {
        return -(2045022964 as isize) as i32;
    }
    if (seqsize as u32).wrapping_add(bytes) != toksize {
        return -(2045022964 as isize) as i32;
    }
    let fresh38 = toksize;
    toksize = toksize.wrapping_sub(1);
    if fresh38 < 1u32 {
        return -(2045022964 as isize) as i32;
    }
    let fresh39 = buf;
    buf = buf.offset(1);
    if *fresh39 as i32 != 0x6i32 {
        return -(2045022964 as isize) as i32;
    }
    let fresh40 = toksize;
    toksize = toksize.wrapping_sub(1);
    if fresh40 < 1u32 {
        return -(2045022964 as isize) as i32;
    }
    let fresh41 = buf;
    buf = buf.offset(1);
    toid.length = *fresh41 as crate::gssapi_h::OM_uint32;
    if toksize < toid.length {
        return -(2045022964 as isize) as i32;
    } else {
        toksize = toksize.wrapping_sub(toid.length)
    }
    toid.elements = buf as *mut libc::c_void;
    buf = buf.offset(toid.length as isize);
    if !(toid.length == (*mech).length
        && crate::stdlib::memcmp(toid.elements, (*mech).elements, toid.length as usize) == 0i32)
    {
        ret = -(2045022965 as isize) as i32
    }
    /*
     * G_WRONG_MECH is not returned immediately because it's more important
     * to return G_BAD_TOK_HEADER if the token header is in fact bad
     */
    if toksize < 2u32 {
        return -(2045022964 as isize) as i32;
    } else {
        toksize = toksize.wrapping_sub(2u32)
    }
    if ret == 0 {
        *buf_in = buf;
        *body_size = toksize
    }
    return ret;
}
/*
 * Return non-zero if the oid is one of the kerberos mech oids,
 * otherwise return zero.
 *
 * N.B. There are 3 oids that represent the kerberos mech:
 * RFC-specified GSS_MECH_KRB5_OID,
 * Old pre-RFC   GSS_MECH_KRB5_OLD_OID,
 * Incorrect MS  GSS_MECH_KRB5_WRONG_OID
 */

unsafe extern "C" fn is_kerb_mech(mut oid: crate::gssapi_h::gss_OID) -> i32 {
    let mut answer = 0i32;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    extern "C" {
        #[no_mangle]
        pub static gss_mech_set_krb5_both: *const crate::gssapi_h::gss_OID_set_desc;
    }
    crate::src::mechglue::g_oid_ops::gss_test_oid_set_member(
        &mut minor,
        oid,
        gss_mech_set_krb5_both as crate::gssapi_h::gss_OID_set,
        &mut answer,
    );
    return answer;
}
unsafe extern "C" fn run_static_initializers() {
    gss_mech_spnego = spnego_oids.as_ptr().offset(0isize);
    spnego_oidsets = [{
        let mut init = crate::gssapi_h::gss_OID_set_desc_struct {
            count: 1usize,
            elements: (spnego_oids.as_ptr() as crate::gssapi_h::gss_OID).offset(0isize),
        };
        init
    }];
    gss_mech_set_spnego = spnego_oidsets.as_ptr().offset(0isize)
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
