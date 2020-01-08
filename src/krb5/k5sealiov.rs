use ::libc;

pub mod k5_platform_h {

    #[inline]

    pub unsafe extern "C" fn store_32_be(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_6)).i = __bswap_32(val);
    }
    #[inline]

    pub unsafe extern "C" fn store_16_le(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_5)).i = val as crate::stdlib::uint16_t;
    }
    use crate::src::krb5::k5sealiov::byteswap_h::__bswap_32;

    /* K5_PLATFORM_H */
}

pub mod byteswap_h {
    #[inline]

    pub unsafe extern "C" fn __bswap_32(
        mut __bsx: crate::stdlib::__uint32_t,
    ) -> crate::stdlib::__uint32_t {
        return (__bsx & 0xff000000u32) >> 24i32
            | (__bsx & 0xff0000u32) >> 8i32
            | (__bsx & 0xff00u32) << 8i32
            | (__bsx & 0xffu32) << 24i32;
    }
}
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__uint16_t;
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
pub use crate::k5_platform_h::C2RustUnnamed_5;
pub use crate::k5_platform_h::C2RustUnnamed_6;
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
pub use crate::krb5_h::krb5_c_block_size;
pub use crate::krb5_h::krb5_c_checksum_length;
pub use crate::krb5_h::krb5_c_crypto_length;
pub use crate::krb5_h::krb5_c_padding_length;
pub use crate::krb5_h::krb5_checksum;
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_cryptotype;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_checksum_contents;
pub use crate::krb5_h::krb5_free_keyblock;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_k_key_keyblock;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keyusage;
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
pub use crate::krb5_h::krb5_ui_4;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::k5sealiov::k5_platform_h::store_16_le;
pub use crate::src::krb5::k5sealiov::k5_platform_h::store_32_be;
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
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapiP_krb5_h::seal_alg;
pub use crate::gssapiP_krb5_h::sgn_alg;
pub use crate::gssapiP_krb5_h::SEAL_ALG_DES3KD;
pub use crate::gssapiP_krb5_h::SEAL_ALG_MICROSOFT_RC4;
pub use crate::gssapiP_krb5_h::SEAL_ALG_NONE;
pub use crate::gssapiP_krb5_h::SGN_ALG_HMAC_MD5;
pub use crate::gssapiP_krb5_h::SGN_ALG_HMAC_SHA1_DES3_KD;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
pub use crate::gssapi_ext_h::gss_iov_buffer_t;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::src::generic::util_token::gssint_g_make_token_header;
pub use crate::src::generic::util_token::gssint_g_token_size;
pub use crate::src::krb5::disp_status::krb5_gss_save_error_info;
pub use crate::src::krb5::k5sealiov::byteswap_h::__bswap_32;
pub use crate::src::krb5::k5sealv3iov::gss_krb5int_make_seal_token_v3_iov;
pub use crate::src::krb5::util_cksum::kg_make_checksum_iov_v1;
pub use crate::src::krb5::util_crypt::kg_allocate_iov;
pub use crate::src::krb5::util_crypt::kg_arcfour_docrypt_iov;
pub use crate::src::krb5::util_crypt::kg_confounder_size;
pub use crate::src::krb5::util_crypt::kg_encrypt_iov;
pub use crate::src::krb5::util_crypt::kg_integ_only_iov;
pub use crate::src::krb5::util_crypt::kg_iov_msglen;
pub use crate::src::krb5::util_crypt::kg_locate_header_iov;
pub use crate::src::krb5::util_crypt::kg_locate_iov;
pub use crate::src::krb5::util_crypt::kg_make_confounder;
pub use crate::src::krb5::util_crypt::kg_release_iov;
pub use crate::src::krb5::util_seqnum::kg_make_seq_num;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/k5sealiov.c */
/*
 * Copyright 2008, 2009 by the Massachusetts Institute of Technology.
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

unsafe extern "C" fn make_seal_token_v1_iov(
    mut context: crate::krb5_h::krb5_context,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut conf_req_flag: i32,
    mut conf_state: *mut i32,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut code = 0i32;
    let mut header = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut padding = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut trailer = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut md5cksum = crate::krb5_h::krb5_checksum {
        magic: 0,
        checksum_type: 0,
        length: 0,
        contents: 0 as *mut crate::krb5_h::krb5_octet,
    };
    let mut cksum = crate::krb5_h::krb5_checksum {
        magic: 0,
        checksum_type: 0,
        length: 0,
        contents: 0 as *mut crate::krb5_h::krb5_octet,
    };
    let mut k5_headerlen = 0usize;
    let mut k5_trailerlen = 0usize;
    let mut data_length = 0usize;
    let mut assoc_data_length = 0usize;
    let mut tmsglen = 0usize;
    let mut tlen: crate::stddef_h::size_t = 0;
    let mut ptr = 0 as *mut u8;
    let mut sign_usage = 23i32;
    cksum.length = 0u32;
    md5cksum.length = cksum.length;
    cksum.contents = 0 as *mut crate::krb5_h::krb5_octet;
    md5cksum.contents = cksum.contents;
    header = crate::src::krb5::util_crypt::kg_locate_header_iov(iov, iov_count, toktype);
    if header.is_null() {
        return 22i32;
    }
    padding = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 9u32);
    if padding.is_null() && toktype == 0x201i32 && (*ctx).gss_flags & 0x1000u32 == 0u32 {
        return 22i32;
    }
    trailer = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 7u32);
    if !trailer.is_null() {
        (*trailer).buffer.length = 0usize
    }
    /* Determine confounder length */
    if toktype == 0x201i32 || conf_req_flag != 0 {
        k5_headerlen = crate::src::krb5::util_crypt::kg_confounder_size(
            context,
            (*(*ctx).enc).keyblock.enctype,
        ) as crate::stddef_h::size_t
    }
    /* Check padding length */
    if toktype == 0x201i32 {
        let mut k5_padlen =
            if (*ctx).sealalg == crate::gssapiP_krb5_h::SEAL_ALG_MICROSOFT_RC4 as i32 {
                1i32
            } else {
                8i32
            } as crate::stddef_h::size_t; /* one byte to indicate one byte of padding */
        let mut gss_padlen: crate::stddef_h::size_t = 0;
        let mut conf_data_length: crate::stddef_h::size_t = 0;
        crate::src::krb5::util_crypt::kg_iov_msglen(
            iov,
            iov_count,
            &mut data_length,
            &mut assoc_data_length,
        );
        conf_data_length = k5_headerlen
            .wrapping_add(data_length)
            .wrapping_sub(assoc_data_length);
        if k5_padlen == 1usize {
            gss_padlen = 1usize
        } else {
            gss_padlen = k5_padlen.wrapping_sub(conf_data_length.wrapping_rem(k5_padlen))
        }
        if (*ctx).gss_flags & 0x1000u32 != 0 {
            /* DCE will pad the actual data itself; padding buffer optional and will be zeroed */
            gss_padlen = 0usize;
            if conf_data_length.wrapping_rem(k5_padlen) != 0 {
                code = -(1765328194 as isize) as crate::krb5_h::krb5_error_code
            }
        } else if (*padding).type_0 & 0x10000u32 != 0 {
            code = crate::src::krb5::util_crypt::kg_allocate_iov(padding, gss_padlen)
        } else if (*padding).buffer.length < gss_padlen {
            code = -(1765328194 as isize) as crate::krb5_h::krb5_error_code
        }
        if code != 0i32 {
            current_block = 2491959576112690396;
        } else {
            /* Initialize padding buffer to pad itself */
            if !padding.is_null() {
                (*padding).buffer.length = gss_padlen; /* confounder length */
                crate::stdlib::memset((*padding).buffer.value, gss_padlen as i32, gss_padlen);
            }
            if (*ctx).gss_flags & 0x1000u32 != 0 {
                tmsglen = k5_headerlen
            } else {
                tmsglen = conf_data_length.wrapping_add((*padding).buffer.length)
            }
            current_block = 3222590281903869779;
        }
    } else {
        current_block = 3222590281903869779;
    }
    match current_block {
        3222590281903869779 => {
            /* Determine token size */
            tlen = crate::src::generic::util_token::gssint_g_token_size(
                (*ctx).mech_used,
                (14usize)
                    .wrapping_add((*ctx).cksum_size)
                    .wrapping_add(tmsglen) as u32,
            ) as crate::stddef_h::size_t;
            k5_headerlen = (k5_headerlen).wrapping_add(tlen.wrapping_sub(tmsglen));
            if (*header).type_0 & 0x10000u32 != 0 {
                code = crate::src::krb5::util_crypt::kg_allocate_iov(header, k5_headerlen)
            } else if (*header).buffer.length < k5_headerlen {
                code = -(1765328194 as isize) as crate::krb5_h::krb5_error_code
            }
            if !(code != 0i32) {
                (*header).buffer.length = k5_headerlen;
                ptr = (*header).buffer.value as *mut u8;
                crate::src::generic::util_token::gssint_g_make_token_header(
                    (*ctx).mech_used,
                    (14usize)
                        .wrapping_add((*ctx).cksum_size)
                        .wrapping_add(tmsglen) as u32,
                    &mut ptr,
                    toktype,
                );
                /* 0..1 SIGN_ALG */
                store_16_le(
                    (*ctx).signalg as u32,
                    &mut *ptr.offset(0isize) as *mut u8 as *mut libc::c_void,
                );
                /* 2..3 SEAL_ALG or Filler */
                if toktype == 0x201i32 && conf_req_flag != 0 {
                    store_16_le(
                        (*ctx).sealalg as u32,
                        &mut *ptr.offset(2isize) as *mut u8 as *mut libc::c_void,
                    );
                } else {
                    /* No seal */
                    *ptr.offset(2isize) = 0xffu8;
                    *ptr.offset(3isize) = 0xffu8
                }
                /* 4..5 Filler */
                *ptr.offset(4isize) = 0xffu8;
                *ptr.offset(5isize) = 0xffu8;
                /* pad the plaintext, encrypt if needed, and stick it in the token */
                /* initialize the checksum */
                match (*ctx).signalg {
                    4 => md5cksum.checksum_type = 0xci32,
                    17 => {
                        md5cksum.checksum_type = -(138i32);
                        if toktype != 0x201i32 {
                            sign_usage = 15i32
                        }
                    }
                    _ => {
                        crate::stdlib::abort();
                    }
                }
                code = crate::krb5_h::krb5_c_checksum_length(
                    context,
                    md5cksum.checksum_type,
                    &mut k5_trailerlen,
                );
                if !(code != 0i32) {
                    md5cksum.length = k5_trailerlen as u32;
                    if k5_headerlen != 0usize && toktype == 0x201i32 {
                        code = crate::src::krb5::util_crypt::kg_make_confounder(
                            context,
                            (*(*ctx).enc).keyblock.enctype,
                            ptr.offset(14isize).offset((*ctx).cksum_size as isize),
                        );
                        if code != 0i32 {
                            current_block = 2491959576112690396;
                        } else {
                            current_block = 16029476503615101993;
                        }
                    } else {
                        current_block = 16029476503615101993;
                    }
                    match current_block {
                        2491959576112690396 => {}
                        _ => {
                            /* compute the checksum */
                            code = crate::src::krb5::util_cksum::kg_make_checksum_iov_v1(
                                context,
                                md5cksum.checksum_type,
                                (*ctx).cksum_size,
                                (*ctx).seq,
                                (*ctx).enc,
                                sign_usage,
                                iov,
                                iov_count,
                                toktype,
                                &mut md5cksum,
                            );
                            if !(code != 0i32) {
                                match (*ctx).signalg {
                                    4 => {
                                        if md5cksum.length as usize == (*ctx).cksum_size {
                                        } else {
                                            crate::stdlib::__assert_fail(b"md5cksum.length == ctx->cksum_size\x00"
                                                              as *const u8 as
                                                              *const i8,
                                                          b"k5sealiov.c\x00"
                                                              as *const u8 as
                                                              *const i8,
                                                          182u32,
                                                          (*::std::mem::transmute::<&[u8; 121],
                                                                                    &[i8; 121]>(b"krb5_error_code make_seal_token_v1_iov(krb5_context, krb5_gss_ctx_id_rec *, int, int *, gss_iov_buffer_desc *, int, int)\x00")).as_ptr());
                                        }
                                        crate::stdlib::memcpy(
                                            ptr.offset(14isize) as *mut libc::c_void,
                                            md5cksum.contents as *const libc::c_void,
                                            md5cksum.length as usize,
                                        );
                                    }
                                    17 => {
                                        crate::stdlib::memcpy(
                                            ptr.offset(14isize) as *mut libc::c_void,
                                            md5cksum.contents as *const libc::c_void,
                                            (*ctx).cksum_size,
                                        );
                                    }
                                    _ => {}
                                }
                                /* create the seq_num */
                                code = crate::src::krb5::util_seqnum::kg_make_seq_num(
                                    context,
                                    (*ctx).seq,
                                    if (*ctx).initiate() as i32 != 0 {
                                        0i32
                                    } else {
                                        0xffi32
                                    },
                                    (*ctx).seq_send as crate::gssapi_h::OM_uint32,
                                    ptr.offset(14isize),
                                    ptr.offset(6isize),
                                );
                                if !(code != 0i32) {
                                    if conf_req_flag != 0 {
                                        if (*ctx).sealalg
                                            == crate::gssapiP_krb5_h::SEAL_ALG_MICROSOFT_RC4 as i32
                                        {
                                            let mut bigend_seqnum: [u8; 4] = [0; 4];
                                            let mut enc_key =
                                                0 as *mut crate::krb5_h::krb5_keyblock;
                                            let mut i: crate::stddef_h::size_t = 0;
                                            store_32_be(
                                                (*ctx).seq_send as u32,
                                                bigend_seqnum.as_mut_ptr() as *mut libc::c_void,
                                            );
                                            code = crate::krb5_h::krb5_k_key_keyblock(
                                                context,
                                                (*ctx).enc,
                                                &mut enc_key,
                                            );
                                            if code != 0i32 {
                                                current_block = 2491959576112690396;
                                            } else {
                                                if (*enc_key).length == 16u32 {
                                                } else {
                                                    crate::stdlib::__assert_fail(b"enc_key->length == 16\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const i8,
                                                                  b"k5sealiov.c\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const i8,
                                                                  208u32,
                                                                  (*::std::mem::transmute::<&[u8; 121],
                                                                                            &[i8; 121]>(b"krb5_error_code make_seal_token_v1_iov(krb5_context, krb5_gss_ctx_id_rec *, int, int *, gss_iov_buffer_desc *, int, int)\x00")).as_ptr());
                                                }
                                                i = 0usize;
                                                while i < (*enc_key).length as usize {
                                                    let ref mut fresh0 = *((*enc_key).contents
                                                        as *mut i8)
                                                        .offset(i as isize);
                                                    *fresh0 = (*fresh0 as i32 ^ 0xf0i32) as i8;
                                                    i = i.wrapping_add(1)
                                                }
                                                code =
                                                    crate::src::krb5::util_crypt::kg_arcfour_docrypt_iov(context,
                                                                           enc_key,
                                                                           0i32,
                                                                           bigend_seqnum.as_mut_ptr(),
                                                                           4usize,
                                                                           iov,
                                                                           iov_count);
                                                crate::krb5_h::krb5_free_keyblock(context, enc_key);
                                                current_block = 17958840340921835115;
                                            }
                                        } else {
                                            code = crate::src::krb5::util_crypt::kg_encrypt_iov(
                                                context,
                                                (*ctx).proto,
                                                ((*ctx).gss_flags & 0x1000u32 != 0u32) as i32,
                                                0usize,
                                                0usize,
                                                (*ctx).enc,
                                                22i32,
                                                0 as *mut libc::c_void,
                                                iov,
                                                iov_count,
                                            );
                                            current_block = 17958840340921835115;
                                        }
                                        match current_block {
                                            2491959576112690396 => {}
                                            _ => {
                                                if code != 0i32 {
                                                    current_block = 2491959576112690396;
                                                } else {
                                                    current_block = 6040267449472925966;
                                                }
                                            }
                                        }
                                    } else {
                                        current_block = 6040267449472925966;
                                    }
                                    match current_block {
                                        2491959576112690396 => {}
                                        _ => {
                                            (*ctx).seq_send = (*ctx).seq_send.wrapping_add(1);
                                            (*ctx).seq_send &= 0xffffffff as usize;
                                            code = 0i32;
                                            if !conf_state.is_null() {
                                                *conf_state = conf_req_flag
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
        _ => {}
    }
    if code != 0i32 {
        crate::src::krb5::util_crypt::kg_release_iov(iov, iov_count);
    }
    crate::krb5_h::krb5_free_checksum_contents(context, &mut md5cksum);
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn kg_seal_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut conf_state: *mut i32,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    if qop_req != 0u32 {
        *minor_status = -(2045022968 as isize) as crate::gssapi_h::OM_uint32;
        return (14u32) << 16i32;
    }
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    if (*ctx).terminated() as i32 != 0 || (*ctx).established() == 0 {
        *minor_status = 39756039u32;
        return (8u32) << 16i32;
    }
    if conf_req_flag != 0 && crate::src::krb5::util_crypt::kg_integ_only_iov(iov, iov_count) != 0 {
        /* may be more sensible to return an error here */
        conf_req_flag = 0i32
    }
    context = (*ctx).k5_context;
    match (*ctx).proto {
        0 => {
            code = make_seal_token_v1_iov(
                context,
                ctx,
                conf_req_flag,
                conf_state,
                iov,
                iov_count,
                toktype,
            )
        }
        1 => {
            code = crate::src::krb5::k5sealv3iov::gss_krb5int_make_seal_token_v3_iov(
                context,
                ctx,
                conf_req_flag,
                conf_state,
                iov,
                iov_count,
                toktype,
            )
        }
        _ => code = -(2045022968 as isize) as crate::krb5_h::krb5_error_code,
    }
    if code != 0i32 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
        return (13u32) << 16i32;
    }
    *minor_status = 0u32;
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn kg_seal_iov_length(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut conf_state: *mut i32,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut header = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut trailer = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut padding = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut data_length: crate::stddef_h::size_t = 0;
    let mut assoc_data_length: crate::stddef_h::size_t = 0;
    let mut gss_headerlen: crate::stddef_h::size_t = 0;
    let mut gss_padlen: crate::stddef_h::size_t = 0;
    let mut gss_trailerlen: crate::stddef_h::size_t = 0;
    let mut k5_headerlen = 0u32;
    let mut k5_trailerlen = 0u32;
    let mut k5_padlen = 0u32;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut dce_or_mic: i32 = 0;
    if qop_req != 0u32 {
        *minor_status = -(2045022968 as isize) as crate::gssapi_h::OM_uint32;
        return (14u32) << 16i32;
    }
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    if (*ctx).established() == 0 {
        *minor_status = 39756039u32;
        return (8u32) << 16i32;
    }
    header = crate::src::krb5::util_crypt::kg_locate_header_iov(iov, iov_count, toktype);
    if header.is_null() {
        *minor_status = 22u32;
        return (13u32) << 16i32;
    }
    (*header).buffer.value = 0 as *mut libc::c_void;
    (*header).buffer.length = 0usize;
    trailer = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 7u32);
    if !trailer.is_null() {
        (*trailer).buffer.value = 0 as *mut libc::c_void;
        (*trailer).buffer.length = 0usize
    }
    /* MIC tokens and DCE-style wrap tokens have similar length considerations:
     * no padding, and the framing surrounds the header only, not the data. */
    dce_or_mic = ((*ctx).gss_flags & 0x1000u32 != 0u32 || toktype == 0x101i32) as i32;
    /* For CFX, EC is used instead of padding, and is placed in header or trailer */
    padding = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 9u32); /* Header */
    if padding.is_null() {
        if conf_req_flag != 0 && (*ctx).proto == 0i32 && dce_or_mic == 0 {
            *minor_status = 22u32; /* Kerb-Header */
            return (13u32) << 16i32;
        }
    } else {
        (*padding).buffer.value = 0 as *mut libc::c_void; /* Kerb-Trailer */
        (*padding).buffer.length = 0usize
    }
    crate::src::krb5::util_crypt::kg_iov_msglen(
        iov,
        iov_count,
        &mut data_length,
        &mut assoc_data_length,
    );
    if conf_req_flag != 0 && crate::src::krb5::util_crypt::kg_integ_only_iov(iov, iov_count) != 0 {
        conf_req_flag = 0i32
    }
    context = (*ctx).k5_context;
    gss_trailerlen = 0usize;
    gss_padlen = gss_trailerlen;
    gss_headerlen = gss_padlen;
    if (*ctx).proto == 1i32 {
        let mut key = 0 as *mut crate::k5_int_h::krb5_key_st;
        let mut enctype: crate::krb5_h::krb5_enctype = 0;
        let mut ec: crate::stddef_h::size_t = 0;
        key = if (*ctx).have_acceptor_subkey() as i32 != 0 {
            (*ctx).acceptor_subkey
        } else {
            (*ctx).subkey
        };
        enctype = (*key).keyblock.enctype;
        code = crate::krb5_h::krb5_c_crypto_length(
            context,
            enctype,
            if conf_req_flag != 0 { 5i32 } else { 6i32 },
            &mut k5_trailerlen,
        );
        if code != 0i32 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            return (13u32) << 16i32;
        }
        if conf_req_flag != 0 {
            code = crate::krb5_h::krb5_c_crypto_length(context, enctype, 1i32, &mut k5_headerlen);
            if code != 0i32 {
                *minor_status = code as crate::gssapi_h::OM_uint32;
                return (13u32) << 16i32;
            }
        }
        gss_headerlen = 16usize;
        if conf_req_flag != 0 {
            gss_headerlen = (gss_headerlen).wrapping_add(k5_headerlen as usize);
            gss_trailerlen = (16u32).wrapping_add(k5_trailerlen) as crate::stddef_h::size_t;
            code = crate::krb5_h::krb5_c_padding_length(
                context,
                enctype,
                data_length
                    .wrapping_sub(assoc_data_length)
                    .wrapping_add(16usize),
                &mut k5_padlen,
            );
            if code != 0i32 {
                *minor_status = code as crate::gssapi_h::OM_uint32;
                return (13u32) << 16i32;
            }
            if k5_padlen == 0u32 && dce_or_mic != 0 {
                /* Windows rejects AEAD tokens with non-zero EC */
                code = crate::krb5_h::krb5_c_block_size(context, enctype, &mut ec);
                if code != 0i32 {
                    *minor_status = code as crate::gssapi_h::OM_uint32;
                    return (13u32) << 16i32;
                }
            } else {
                ec = k5_padlen as crate::stddef_h::size_t
            }
            gss_trailerlen = (gss_trailerlen).wrapping_add(ec)
        } else {
            gss_trailerlen = k5_trailerlen as crate::stddef_h::size_t
            /* Kerb-Checksum */
        }
    } else if dce_or_mic == 0 {
        k5_padlen = if (*ctx).sealalg == crate::gssapiP_krb5_h::SEAL_ALG_MICROSOFT_RC4 as i32 {
            1i32
        } else {
            8i32
        } as u32;
        if k5_padlen == 1u32 {
            gss_padlen = 1usize
        } else {
            gss_padlen = (k5_padlen as usize).wrapping_sub(
                data_length
                    .wrapping_sub(assoc_data_length)
                    .wrapping_rem(k5_padlen as usize),
            )
        }
    }
    data_length = (data_length).wrapping_add(gss_padlen);
    if (*ctx).proto == 0i32 {
        /* Header | Checksum | Confounder | Data | Pad */
        let mut data_size: crate::stddef_h::size_t = 0;
        k5_headerlen = crate::src::krb5::util_crypt::kg_confounder_size(
            context,
            (*(*ctx).enc).keyblock.enctype,
        ) as u32;
        data_size = (14usize)
            .wrapping_add((*ctx).cksum_size)
            .wrapping_add(k5_headerlen as usize);
        if dce_or_mic == 0 {
            data_size = (data_size).wrapping_add(data_length)
        }
        gss_headerlen = crate::src::generic::util_token::gssint_g_token_size(
            (*ctx).mech_used,
            data_size as u32,
        ) as crate::stddef_h::size_t;
        /* g_token_size() will include data_size as well as the overhead, so
         * subtract data_length just to get the overhead (ie. token size) */
        if dce_or_mic == 0 {
            gss_headerlen = (gss_headerlen).wrapping_sub(data_length)
        }
    }
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if trailer.is_null() {
        gss_headerlen = (gss_headerlen).wrapping_add(gss_trailerlen)
    } else {
        (*trailer).buffer.length = gss_trailerlen
    }
    if gss_padlen == 0usize || !padding.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"gss_padlen == 0 || padding != NULL\x00" as *const u8
                          as *const i8,
                      b"k5sealiov.c\x00" as *const u8 as *const i8,
                      460u32,
                      (*::std::mem::transmute::<&[u8; 112],
                                                &[i8; 112]>(b"OM_uint32 kg_seal_iov_length(OM_uint32 *, gss_ctx_id_t, int, gss_qop_t, int *, gss_iov_buffer_desc *, int, int)\x00")).as_ptr());
    }
    if !padding.is_null() {
        (*padding).buffer.length = gss_padlen
    }
    (*header).buffer.length = gss_headerlen;
    if !conf_state.is_null() {
        *conf_state = conf_req_flag
    }
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_wrap_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut conf_state: *mut i32,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    major_status = kg_seal_iov(
        minor_status,
        context_handle,
        conf_req_flag,
        qop_req,
        conf_state,
        iov,
        iov_count,
        0x201i32,
    );
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_wrap_iov_length(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut conf_state: *mut i32,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    major_status = kg_seal_iov_length(
        minor_status,
        context_handle,
        conf_req_flag,
        qop_req,
        conf_state,
        iov,
        iov_count,
        0x201i32,
    );
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_get_mic_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    major_status = kg_seal_iov(
        minor_status,
        context_handle,
        0i32,
        qop_req,
        0 as *mut i32,
        iov,
        iov_count,
        0x101i32,
    );
    return major_status;
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
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_get_mic_iov_length(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    major_status = kg_seal_iov_length(
        minor_status,
        context_handle,
        0i32,
        qop_req,
        0 as *mut i32,
        iov,
        iov_count,
        0x101i32,
    );
    return major_status;
}
