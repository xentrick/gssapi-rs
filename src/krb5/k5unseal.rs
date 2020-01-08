use ::libc;

pub mod k5_platform_h {

    #[inline]

    pub unsafe extern "C" fn store_32_be(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_6)).i = __bswap_32(val);
    }
    #[inline]

    pub unsafe extern "C" fn load_16_be(mut cvp: *const libc::c_void) -> u16 {
        let mut p = cvp as *const u8;
        return __bswap_16((*(p as *const crate::k5_platform_h::C2RustUnnamed_5)).i);
    }

    use crate::src::krb5::k5unseal::byteswap_h::__bswap_16;
    use crate::src::krb5::k5unseal::byteswap_h::__bswap_32;

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
    #[inline]

    pub unsafe extern "C" fn __bswap_16(
        mut __bsx: crate::stdlib::__uint16_t,
    ) -> crate::stdlib::__uint16_t {
        return (__bsx as i32 >> 8i32 & 0xffi32 | (__bsx as i32 & 0xffi32) << 8i32)
            as crate::stdlib::__uint16_t;
    }
}

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
pub use crate::k5_platform_h::k5_bcmp;
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
pub use crate::krb5_h::krb5_c_checksum_length;
pub use crate::krb5_h::krb5_checksum;
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_const_pointer;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
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
pub use crate::krb5_h::krb5_k_make_checksum;
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
pub use crate::src::krb5::k5unseal::k5_platform_h::load_16_be;
pub use crate::src::krb5::k5unseal::k5_platform_h::store_32_be;
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
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_int32;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::src::generic::util_seqstate::gssint_g_seqstate_check;
pub use crate::src::generic::util_token::gssint_g_verify_token_header;
pub use crate::src::krb5::disp_status::krb5_gss_save_error_info;
pub use crate::src::krb5::k5sealv3::gss_krb5int_unseal_token_v3;
pub use crate::src::krb5::util_crypt::kg_arcfour_docrypt;
pub use crate::src::krb5::util_crypt::kg_confounder_size;
pub use crate::src::krb5::util_crypt::kg_decrypt;
pub use crate::src::krb5::util_seqnum::kg_get_seq_num;

pub use crate::src::krb5::k5unseal::byteswap_h::__bswap_16;
pub use crate::src::krb5::k5unseal::byteswap_h::__bswap_32;
pub use crate::src::krb5::k5unseal::gssapi_alloc_h::gssalloc_free;
pub use crate::src::krb5::k5unseal::gssapi_alloc_h::gssalloc_malloc;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2001, 2007 by the Massachusetts Institute of Technology.
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
/* message_buffer is an input if SIGN, output if SEAL, and ignored if DEL_CTX
conf_state is only valid if SEAL. */

unsafe extern "C" fn kg_unseal_v1(
    mut context: crate::krb5_h::krb5_context,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut ptr: *mut u8,
    mut bodysize: i32,
    mut message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut toktype: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut conflen = 0i32;
    let mut signalg: i32 = 0;
    let mut sealalg: i32 = 0;
    let mut bad_pad = 0i32;
    let mut token = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut md5cksum = crate::krb5_h::krb5_checksum {
        magic: 0,
        checksum_type: 0,
        length: 0,
        contents: 0 as *mut crate::krb5_h::krb5_octet,
    };
    let mut plaind = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut data_ptr = 0 as *mut i8;
    let mut plain = 0 as *mut u8;
    let mut cksum_len = 0u32;
    let mut plainlen: crate::stddef_h::size_t = 0;
    let mut direction: i32 = 0;
    let mut seqnum: crate::krb5_h::krb5_ui_4 = 0;
    let mut retval: crate::gssapi_h::OM_uint32 = 0;
    let mut sumlen: crate::stddef_h::size_t = 0;
    let mut padlen: crate::stddef_h::size_t = 0;
    let mut sign_usage = 23i32;
    if toktype == 0x201i32 {
        (*message_buffer).length = 0usize;
        (*message_buffer).value = 0 as *mut libc::c_void
    }
    /* Sanity checks */
    if (*ctx).seq.is_null() {
        /* ctx was established using a newer enctype, and cannot process RFC
         * 1964 tokens. */
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    if bodysize < 22i32
        || *ptr.offset(4isize) as i32 != 0xffi32
        || *ptr.offset(5isize) as i32 != 0xffi32
    {
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    signalg = *ptr.offset(0isize) as i32 + ((*ptr.offset(1isize) as i32) << 8i32);
    sealalg = *ptr.offset(2isize) as i32 + ((*ptr.offset(3isize) as i32) << 8i32);
    if toktype != 0x201i32 && sealalg != 0xffffi32 {
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    /* in the current spec, there is only one valid seal algorithm per
    key type, so a simple comparison is ok */
    if toktype == 0x201i32 && !(sealalg == 0xffffi32 || sealalg == (*ctx).sealalg) {
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    /* there are several mappings of seal algorithms to sign algorithms,
    but few enough that we can try them all. */
    if (*ctx).sealalg == crate::gssapiP_krb5_h::SEAL_ALG_NONE as i32 && signalg > 1i32
        || (*ctx).sealalg == crate::gssapiP_krb5_h::SEAL_ALG_DES3KD as i32
            && signalg != crate::gssapiP_krb5_h::SGN_ALG_HMAC_SHA1_DES3_KD as i32
        || (*ctx).sealalg == crate::gssapiP_krb5_h::SEAL_ALG_MICROSOFT_RC4 as i32
            && signalg != crate::gssapiP_krb5_h::SGN_ALG_HMAC_MD5 as i32
    {
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    match signalg {
        17 => {
            cksum_len = 8u32;
            if toktype != 0x201i32 {
                sign_usage = 15i32
            }
        }
        4 => cksum_len = 20u32,
        _ => {
            *minor_status = 0u32;
            return (9u32) << 16i32;
        }
    }
    if (bodysize as crate::stddef_h::size_t) < (14u32).wrapping_add(cksum_len) as usize {
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    /* get the token parameters */
    code = crate::src::krb5::util_seqnum::kg_get_seq_num(
        context,
        (*ctx).seq,
        ptr.offset(14isize),
        ptr.offset(6isize),
        &mut direction,
        &mut seqnum,
    );
    if code != 0 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (6u32) << 16i32;
    }
    /* decode the message, if SEAL */
    if toktype == 0x201i32 {
        let mut tmsglen = (bodysize as u32).wrapping_sub((14u32).wrapping_add(cksum_len))
            as crate::stddef_h::size_t;
        if sealalg != 0xffffi32 {
            plain = crate::stdlib::malloc(tmsglen) as *mut u8;
            if plain.is_null() {
                *minor_status = 12u32;
                return (13u32) << 16i32;
            }
            if (*ctx).sealalg == crate::gssapiP_krb5_h::SEAL_ALG_MICROSOFT_RC4 as i32 {
                let mut bigend_seqnum: [u8; 4] = [0; 4];
                let mut enc_key = 0 as *mut crate::krb5_h::krb5_keyblock;
                let mut i: i32 = 0;
                store_32_be(seqnum, bigend_seqnum.as_mut_ptr() as *mut libc::c_void);
                code = crate::krb5_h::krb5_k_key_keyblock(context, (*ctx).enc, &mut enc_key);
                if code != 0 {
                    crate::stdlib::free(plain as *mut libc::c_void);
                    *minor_status = code as crate::gssapi_h::OM_uint32;
                    return (13u32) << 16i32;
                }
                if (*enc_key).length == 16u32 {
                } else {
                    crate::stdlib::__assert_fail(
                        b"enc_key->length == 16\x00" as *const u8 as *const i8,
                        b"k5unseal.c\x00" as *const u8 as *const i8,
                        191u32,
                        (*::std::mem::transmute::<&[u8; 25], &[i8; 25]>(
                            b"OM_uint32 kg_unseal_v1()\x00",
                        ))
                        .as_ptr(),
                    );
                }
                i = 0i32;
                while i <= 15i32 {
                    let ref mut fresh0 = *((*enc_key).contents as *mut i8).offset(i as isize);
                    *fresh0 = (*fresh0 as i32 ^ 0xf0i32) as i8;
                    i += 1
                }
                code = crate::src::krb5::util_crypt::kg_arcfour_docrypt(
                    enc_key,
                    0i32,
                    &mut *bigend_seqnum.as_mut_ptr().offset(0isize),
                    4usize,
                    ptr.offset(14isize).offset(cksum_len as isize),
                    tmsglen,
                    plain,
                );
                crate::krb5_h::krb5_free_keyblock(context, enc_key);
            } else {
                code = crate::src::krb5::util_crypt::kg_decrypt(
                    context,
                    (*ctx).enc,
                    22i32,
                    0 as *mut libc::c_void,
                    ptr.offset(14isize).offset(cksum_len as isize)
                        as crate::krb5_h::krb5_const_pointer,
                    plain as crate::krb5_h::krb5_pointer,
                    tmsglen as u32,
                )
            }
            if code != 0 {
                crate::stdlib::free(plain as *mut libc::c_void);
                *minor_status = code as crate::gssapi_h::OM_uint32;
                return (13u32) << 16i32;
            }
        } else {
            plain = ptr.offset(14isize).offset(cksum_len as isize)
        }
        plainlen = tmsglen;
        conflen = crate::src::krb5::util_crypt::kg_confounder_size(
            context,
            (*(*ctx).enc).keyblock.enctype,
        );
        if tmsglen < conflen as crate::stddef_h::size_t {
            if sealalg != 0xffffi32 {
                crate::stdlib::free(plain as *mut libc::c_void);
            }
            *minor_status = 0u32;
            return (9u32) << 16i32;
        }
        padlen = *plain.offset(tmsglen.wrapping_sub(1usize) as isize) as crate::stddef_h::size_t;
        if tmsglen.wrapping_sub(conflen as usize) < padlen {
            /* Don't error out yet, to avoid padding oracle attacks.  We will
             * treat this as a checksum failure later on. */
            padlen = 0usize;
            bad_pad = 1i32
        }
        token.length = tmsglen.wrapping_sub(conflen as usize).wrapping_sub(padlen);
        if token.length != 0 {
            token.value = gssalloc_malloc(token.length);
            if token.value.is_null() {
                if sealalg != 0xffffi32 {
                    crate::stdlib::free(plain as *mut libc::c_void);
                }
                *minor_status = 12u32;
                return (13u32) << 16i32;
            }
            crate::stdlib::memcpy(
                token.value,
                plain.offset(conflen as isize) as *const libc::c_void,
                token.length,
            );
        } else {
            token.value = 0 as *mut libc::c_void
        }
    } else if toktype == 0x101i32 {
        token = *message_buffer;
        plain = token.value as *mut u8;
        plainlen = token.length
    } else {
        token.length = 0usize;
        token.value = 0 as *mut libc::c_void;
        plain = token.value as *mut u8;
        plainlen = token.length
    }
    /* compute the checksum of the message */
    /* initialize the the cksum */
    match signalg {
        17 => md5cksum.checksum_type = -(138i32),
        4 => md5cksum.checksum_type = 0xci32,
        _ => {
            crate::stdlib::abort();
        }
    }
    code = crate::krb5_h::krb5_c_checksum_length(context, md5cksum.checksum_type, &mut sumlen);
    if code != 0 {
        return code as crate::gssapi_h::OM_uint32;
    }
    md5cksum.length = sumlen as u32;
    match signalg {
        4 | 17 => {
            /* compute the checksum of the message */
            /* 8 = bytes of token body to be checksummed according to spec */
            data_ptr = crate::stdlib::malloc((8usize).wrapping_add(plainlen)) as *mut i8;
            if data_ptr.is_null() {
                if sealalg != 0xffffi32 {
                    crate::stdlib::free(plain as *mut libc::c_void);
                }
                if toktype == 0x201i32 {
                    gssalloc_free(token.value);
                }
                *minor_status = 12u32;
                return (13u32) << 16i32;
            }
            crate::stdlib::memcpy(
                data_ptr as *mut libc::c_void,
                ptr.offset(-(2isize)) as *const libc::c_void,
                8usize,
            );
            crate::stdlib::memcpy(
                data_ptr.offset(8isize) as *mut libc::c_void,
                plain as *const libc::c_void,
                plainlen,
            );
            plaind.length = (8usize).wrapping_add(plainlen) as u32;
            plaind.data = data_ptr;
            code = crate::krb5_h::krb5_k_make_checksum(
                context,
                md5cksum.checksum_type,
                (*ctx).seq,
                sign_usage,
                &mut plaind,
                &mut md5cksum,
            );
            crate::stdlib::free(data_ptr as *mut libc::c_void);
            if code != 0 {
                if toktype == 0x201i32 {
                    gssalloc_free(token.value);
                }
                *minor_status = code as crate::gssapi_h::OM_uint32;
                return (13u32) << 16i32;
            }
            code = crate::k5_platform_h::k5_bcmp(
                md5cksum.contents as *const libc::c_void,
                ptr.offset(14isize) as *const libc::c_void,
                cksum_len as crate::stddef_h::size_t,
            )
        }
        _ => {
            *minor_status = 0u32;
            return (9u32) << 16i32;
        }
    }
    crate::krb5_h::krb5_free_checksum_contents(context, &mut md5cksum);
    if sealalg != 0xffffi32 {
        crate::stdlib::free(plain as *mut libc::c_void);
    }
    /* compare the computed checksum against the transmitted checksum */
    if code != 0 || bad_pad != 0 {
        if toktype == 0x201i32 {
            gssalloc_free(token.value);
        }
        *minor_status = 0u32;
        return (6u32) << 16i32;
    }
    /* It got through unscathed.  Make sure the context is unexpired. */
    if toktype == 0x201i32 {
        *message_buffer = token
    }
    if !conf_state.is_null() {
        *conf_state = (sealalg != 0xffffi32) as i32
    }
    if !qop_state.is_null() {
        *qop_state = 0u32
    }
    /* do sequencing checks */
    if (*ctx).initiate() as i32 != 0 && direction != 0xffi32
        || (*ctx).initiate() == 0 && direction != 0i32
    {
        if toktype == 0x201i32 {
            gssalloc_free(token.value);
            (*message_buffer).value = 0 as *mut libc::c_void;
            (*message_buffer).length = 0usize
        }
        *minor_status = -(2045022963 as isize) as crate::gssapi_h::OM_uint32;
        return (6u32) << 16i32;
    }
    retval = crate::src::generic::util_seqstate::gssint_g_seqstate_check(
        (*ctx).seqstate,
        seqnum as crate::stdlib::uint64_t,
    );
    /* success or ordering violation */
    *minor_status = 0u32;
    return retval;
}
/* message_buffer is an input if SIGN, output if SEAL, and ignored if DEL_CTX
conf_state is only valid if SEAL. */
#[no_mangle]

pub unsafe extern "C" fn kg_unseal(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut input_token_buffer: crate::gssapi_h::gss_buffer_t,
    mut message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut toktype: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut ptr = 0 as *mut u8;
    let mut bodysize: u32 = 0;
    let mut err: i32 = 0;
    let mut toktype2: i32 = 0;
    let mut vfyflags = 0i32;
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    if (*ctx).terminated() as i32 != 0 || (*ctx).established() == 0 {
        *minor_status = 39756039u32;
        return (8u32) << 16i32;
    }
    /* parse the token, leave the data in message_buffer, setting conf_state */
    /* verify the header */
    ptr = (*input_token_buffer).value as *mut u8;
    err = crate::src::generic::util_token::gssint_g_verify_token_header(
        (*ctx).mech_used,
        &mut bodysize,
        &mut ptr,
        -(1i32),
        (*input_token_buffer).length as u32,
        vfyflags,
    );
    if err != 0 {
        *minor_status = err as crate::gssapi_h::OM_uint32;
        return (9u32) << 16i32;
    }
    if bodysize < 2u32 {
        *minor_status = -(2045022964 as isize) as crate::gssapi_h::OM_uint32;
        return (9u32) << 16i32;
    }
    toktype2 = load_16_be(ptr as *const libc::c_void) as i32;
    ptr = ptr.offset(2isize);
    bodysize = bodysize.wrapping_sub(2u32);
    match toktype2 {
        1028 | 1284 | 1029 => {
            ret = crate::src::krb5::k5sealv3::gss_krb5int_unseal_token_v3(
                &mut (*ctx).k5_context,
                minor_status,
                ctx,
                ptr,
                bodysize,
                message_buffer,
                conf_state,
                qop_state,
                toktype,
            )
        }
        257 | 513 | 258 => {
            ret = kg_unseal_v1(
                (*ctx).k5_context,
                minor_status,
                ctx,
                ptr,
                bodysize as i32,
                message_buffer,
                conf_state,
                qop_state,
                toktype,
            )
        }
        _ => {
            *minor_status = -(2045022964 as isize) as crate::gssapi_h::OM_uint32;
            ret = (9u32) << 16i32
        }
    }
    if ret != 0u32 {
        crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, (*ctx).k5_context);
    }
    return ret;
}
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
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_unwrap(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut rstat: crate::gssapi_h::OM_uint32 = 0;
    rstat = kg_unseal(
        minor_status,
        context_handle,
        input_message_buffer,
        output_message_buffer,
        conf_state,
        qop_state,
        0x201i32,
    );
    return rstat;
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
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_verify_mic(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut message_buffer: crate::gssapi_h::gss_buffer_t,
    mut token_buffer: crate::gssapi_h::gss_buffer_t,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut rstat: crate::gssapi_h::OM_uint32 = 0;
    rstat = kg_unseal(
        minor_status,
        context_handle,
        token_buffer,
        message_buffer,
        0 as *mut i32,
        qop_state,
        0x101i32,
    );
    return rstat;
}
