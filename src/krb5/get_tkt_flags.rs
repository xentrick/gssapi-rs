use ::libc;

/* _GSSAPIP_KRB5_H_ */
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
pub use crate::src::generic::util_buffer_set::generic_gss_add_buffer_set_member;
/*
 * These take unglued krb5-mech-specific contexts.
 */
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
/*
 * $Id$
 */
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_get_tkt_flags(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    _desired_object: crate::gssapi_h::gss_OID,
    mut data_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut rep = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    rep.value = &mut (*ctx).krb_flags as *mut crate::krb5_h::krb5_flags as *mut libc::c_void;
    rep.length = ::std::mem::size_of::<crate::krb5_h::krb5_flags>();
    return crate::src::generic::util_buffer_set::generic_gss_add_buffer_set_member(
        minor_status,
        &mut rep,
        data_set,
    );
}
