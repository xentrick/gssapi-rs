use ::libc;

pub mod k5_platform_h {

    /* Do it in macro form so we get the file/line of the invocation if
    the assertion fails.  */
    /* forward declaration for use in initializer */
    /* so ';' following macro use won't get error */
    /* This should be called in finalization only, so we shouldn't have
    multiple active threads mucking around in our library at this
    point.  So ignore the once_t object and just look at the flag.

    XXX Could we have problems with memory coherence between processors
    if we don't invoke mutex/once routines?  Probably not, the
    application code should already be coordinating things such that
    the library code is not in use by this point, and memory
    synchronization will be needed there.  */
    /* If we're using gcc, if the C++ support works, the compiler should
    build executables and shared libraries that support the use of
    static constructors and destructors.  The C compiler supports a
    function attribute that makes use of the same facility as C++.

    XXX How do we know if the C++ support actually works?  */
    /* Read and write integer values as (unaligned) octet strings in
    specific byte orders.  Add per-platform optimizations as
    needed.  */
    /* Check for BIG/LITTLE_ENDIAN macros.  If exactly one is defined, use
    it.  If both are defined, then BYTE_ORDER should be defined and
    match one of them.  Try those symbols, then try again with an
    underscore prefix.  */
    /* Optimize for GCC on platforms with known byte orders.

    GCC's packed structures can be written to with any alignment; the
    compiler will use byte operations, unaligned-word operations, or
    normal memory ops as appropriate for the architecture.

    This assumes the availability of uint##_t types, which should work
    on most of our platforms except Windows, where we're not using
    GCC.  */
    /* To do: Define SWAP16, SWAP32, SWAP64 macros to byte-swap values
    with the indicated numbers of bits.

    Linux: byteswap.h, bswap_16 etc.
    Solaris 10: none
    macOS: machine/endian.h or byte_order.h, NXSwap{Short,Int,LongLong}
    NetBSD: sys/bswap.h, bswap16 etc.  */
    /* Note that on Windows at least this file can be included from C++
    source, so casts *from* void* are required.  */
    #[inline]

    pub unsafe extern "C" fn store_16_be(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_5)).i =
            __bswap_16(val as crate::stdlib::__uint16_t);
    }
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

    use crate::src::krb5::k5unsealiov::byteswap_h::__bswap_16;
    use crate::src::krb5::k5unsealiov::byteswap_h::__bswap_32;

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
pub use crate::krb5_h::krb5_c_crypto_length;
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
pub use crate::src::krb5::k5unsealiov::k5_platform_h::load_16_be;
pub use crate::src::krb5::k5unsealiov::k5_platform_h::store_16_be;
pub use crate::src::krb5::k5unsealiov::k5_platform_h::store_32_be;
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
pub use crate::gssapi_h::gss_int32;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::src::generic::util_seqstate::gssint_g_seqstate_check;
pub use crate::src::generic::util_token::gssint_g_verify_token_header;
pub use crate::src::krb5::disp_status::krb5_gss_save_error_info;
pub use crate::src::krb5::k5sealv3::gss_krb5int_rotate_left;
pub use crate::src::krb5::k5sealv3iov::gss_krb5int_unseal_v3_iov;
pub use crate::src::krb5::k5unsealiov::byteswap_h::__bswap_16;
pub use crate::src::krb5::k5unsealiov::byteswap_h::__bswap_32;
pub use crate::src::krb5::util_cksum::kg_make_checksum_iov_v1;
pub use crate::src::krb5::util_crypt::kg_allocate_iov;
pub use crate::src::krb5::util_crypt::kg_arcfour_docrypt_iov;
pub use crate::src::krb5::util_crypt::kg_confounder_size;
pub use crate::src::krb5::util_crypt::kg_decrypt_iov;
pub use crate::src::krb5::util_crypt::kg_fixup_padding_iov;
pub use crate::src::krb5::util_crypt::kg_iov_msglen;
pub use crate::src::krb5::util_crypt::kg_locate_header_iov;
pub use crate::src::krb5::util_crypt::kg_locate_iov;
pub use crate::src::krb5::util_crypt::kg_release_iov;
pub use crate::src::krb5::util_seqnum::kg_get_seq_num;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/k5unsealiov.c */
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

unsafe extern "C" fn kg_unseal_v1_iov(
    mut context: crate::krb5_h::krb5_context,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut token_wrapper_len: crate::stddef_h::size_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut toktype: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut code: crate::gssapi_h::OM_uint32 = 0;
    let mut header = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut trailer = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut ptr = 0 as *mut u8;
    let mut sealalg: i32 = 0;
    let mut signalg: i32 = 0;
    let mut md5cksum = crate::krb5_h::krb5_checksum {
        magic: 0,
        checksum_type: 0,
        length: 0,
        contents: 0 as *mut crate::krb5_h::krb5_octet,
    };
    let mut cksum_len = 0usize;
    let mut conflen = 0usize;
    let mut direction: i32 = 0;
    let mut seqnum: crate::krb5_h::krb5_ui_4 = 0;
    let mut retval: crate::gssapi_h::OM_uint32 = 0;
    let mut sumlen: crate::stddef_h::size_t = 0;
    let mut sign_usage = 23i32;
    md5cksum.length = 0u32;
    md5cksum.contents = 0 as *mut crate::krb5_h::krb5_octet;
    header = crate::src::krb5::util_crypt::kg_locate_header_iov(iov, iov_count, toktype);
    if !header.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"header != NULL\x00" as *const u8 as
                          *const i8,
                      b"k5unsealiov.c\x00" as *const u8 as
                          *const i8,
                      60u32,
                      (*::std::mem::transmute::<&[u8; 138],
                                                &[i8; 138]>(b"OM_uint32 kg_unseal_v1_iov(krb5_context, OM_uint32 *, krb5_gss_ctx_id_rec *, gss_iov_buffer_desc *, int, size_t, int *, gss_qop_t *, int)\x00")).as_ptr());
    }
    trailer = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 7u32);
    if !trailer.is_null() && (*trailer).buffer.length != 0usize {
        *minor_status = -(1765328194 as isize) as crate::gssapi_h::OM_uint32;
        return (9u32) << 16i32;
    }
    if (*ctx).seq.is_null() {
        /* ctx was established using a newer enctype, and cannot process RFC
         * 1964 tokens. */
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    if (*header).buffer.length < token_wrapper_len.wrapping_add(22usize) {
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    ptr = ((*header).buffer.value as *mut u8).offset(token_wrapper_len as isize);
    signalg = *ptr.offset(0isize) as i32;
    signalg |= (*ptr.offset(1isize) as i32) << 8i32;
    sealalg = *ptr.offset(2isize) as i32;
    sealalg |= (*ptr.offset(3isize) as i32) << 8i32;
    if *ptr.offset(4isize) as i32 != 0xffi32 || *ptr.offset(5isize) as i32 != 0xffi32 {
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    if toktype != 0x201i32 && sealalg != 0xffffi32 {
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    if toktype == 0x201i32 && !(sealalg == 0xffffi32 || sealalg == (*ctx).sealalg) {
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
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
            cksum_len = 8usize;
            if toktype != 0x201i32 {
                sign_usage = 15i32
            }
        }
        4 => cksum_len = 20usize,
        _ => {
            *minor_status = 0u32;
            return (9u32) << 16i32;
        }
    }
    /* get the token parameters */
    code = crate::src::krb5::util_seqnum::kg_get_seq_num(
        context,
        (*ctx).seq,
        ptr.offset(14isize),
        ptr.offset(6isize),
        &mut direction,
        &mut seqnum,
    ) as crate::gssapi_h::OM_uint32;
    if code != 0u32 {
        *minor_status = code;
        return (6u32) << 16i32;
    }
    /* decode the message, if SEAL */
    if toktype == 0x201i32 {
        if sealalg != 0xffffi32 {
            if (*ctx).sealalg == crate::gssapiP_krb5_h::SEAL_ALG_MICROSOFT_RC4 as i32 {
                let mut bigend_seqnum: [u8; 4] = [0; 4];
                let mut enc_key = 0 as *mut crate::krb5_h::krb5_keyblock;
                let mut i: crate::stddef_h::size_t = 0;
                store_32_be(seqnum, bigend_seqnum.as_mut_ptr() as *mut libc::c_void);
                code = crate::krb5_h::krb5_k_key_keyblock(context, (*ctx).enc, &mut enc_key)
                    as crate::gssapi_h::OM_uint32;
                if code != 0u32 {
                    retval = (13u32) << 16i32;
                    current_block = 7397202668667826222;
                } else {
                    if (*enc_key).length == 16u32 {
                    } else {
                        crate::stdlib::__assert_fail(b"enc_key->length == 16\x00" as
                                          *const u8 as *const i8,
                                      b"k5unsealiov.c\x00" as *const u8 as
                                          *const i8,
                                      151u32,
                                      (*::std::mem::transmute::<&[u8; 138],
                                                                &[i8; 138]>(b"OM_uint32 kg_unseal_v1_iov(krb5_context, OM_uint32 *, krb5_gss_ctx_id_rec *, gss_iov_buffer_desc *, int, size_t, int *, gss_qop_t *, int)\x00")).as_ptr());
                    }
                    i = 0usize;
                    while i < (*enc_key).length as usize {
                        let ref mut fresh0 = *((*enc_key).contents as *mut i8).offset(i as isize);
                        *fresh0 = (*fresh0 as i32 ^ 0xf0i32) as i8;
                        i = i.wrapping_add(1)
                    }
                    code = crate::src::krb5::util_crypt::kg_arcfour_docrypt_iov(
                        context,
                        enc_key,
                        0i32,
                        &mut *bigend_seqnum.as_mut_ptr().offset(0isize),
                        4usize,
                        iov,
                        iov_count,
                    ) as crate::gssapi_h::OM_uint32;
                    crate::krb5_h::krb5_free_keyblock(context, enc_key);
                    current_block = 2631791190359682872;
                }
            } else {
                code = crate::src::krb5::util_crypt::kg_decrypt_iov(
                    context,
                    0i32,
                    ((*ctx).gss_flags & 0x1000u32 != 0u32) as i32,
                    0usize,
                    0usize,
                    (*ctx).enc,
                    22i32,
                    0 as *mut libc::c_void,
                    iov,
                    iov_count,
                ) as crate::gssapi_h::OM_uint32;
                current_block = 2631791190359682872;
            }
            match current_block {
                7397202668667826222 => {}
                _ => {
                    if code != 0u32 {
                        retval = (13u32) << 16i32;
                        current_block = 7397202668667826222;
                    } else {
                        current_block = 10512632378975961025;
                    }
                }
            }
        } else {
            current_block = 10512632378975961025;
        }
        match current_block {
            7397202668667826222 => {}
            _ => {
                conflen = crate::src::krb5::util_crypt::kg_confounder_size(
                    context,
                    (*(*ctx).enc).keyblock.enctype,
                ) as crate::stddef_h::size_t;
                current_block = 12961834331865314435;
            }
        }
    } else {
        current_block = 12961834331865314435;
    }
    match current_block {
        12961834331865314435 => {
            if (*header).buffer.length
                != token_wrapper_len
                    .wrapping_add(14usize)
                    .wrapping_add(cksum_len)
                    .wrapping_add(conflen)
            {
                retval = (9u32) << 16i32
            } else {
                /* compute the checksum of the message */
                /* initialize the checksum */
                match signalg {
                    17 => md5cksum.checksum_type = -(138i32),
                    4 => md5cksum.checksum_type = 0xci32,
                    _ => {
                        crate::stdlib::abort();
                    }
                }
                code = crate::krb5_h::krb5_c_checksum_length(
                    context,
                    md5cksum.checksum_type,
                    &mut sumlen,
                ) as crate::gssapi_h::OM_uint32;
                if code != 0u32 {
                    retval = (13u32) << 16i32
                } else {
                    md5cksum.length = sumlen as u32;
                    /* compute the checksum of the message */
                    code = crate::src::krb5::util_cksum::kg_make_checksum_iov_v1(
                        context,
                        md5cksum.checksum_type,
                        cksum_len,
                        (*ctx).seq,
                        (*ctx).enc,
                        sign_usage,
                        iov,
                        iov_count,
                        toktype,
                        &mut md5cksum,
                    ) as crate::gssapi_h::OM_uint32;
                    if code != 0u32 {
                        retval = (13u32) << 16i32
                    } else {
                        match signalg {
                            4 | 17 => {
                                code = crate::k5_platform_h::k5_bcmp(
                                    md5cksum.contents as *const libc::c_void,
                                    ptr.offset(14isize) as *const libc::c_void,
                                    cksum_len,
                                )
                                    as crate::gssapi_h::OM_uint32;
                                if code != 0u32 {
                                    code = 0u32;
                                    retval = (6u32) << 16i32
                                } else {
                                    /*
                                     * For GSS_C_DCE_STYLE, the caller manages the padding, because the
                                     * pad length is in the RPC PDU. The value of the padding may be
                                     * uninitialized. For normal GSS, the last bytes of the decrypted
                                     * data contain the pad length. kg_fixup_padding_iov() will find
                                     * this and fixup the last data IOV appropriately.
                                     */
                                    if toktype == 0x201i32 && (*ctx).gss_flags & 0x1000u32 == 0u32 {
                                        retval = crate::src::krb5::util_crypt::kg_fixup_padding_iov(
                                            &mut code, iov, iov_count,
                                        );
                                        if retval != 0u32 {
                                            current_block = 7397202668667826222;
                                        } else {
                                            current_block = 5700653730392116747;
                                        }
                                    } else {
                                        current_block = 5700653730392116747;
                                    }
                                    match current_block {
                                        7397202668667826222 => {}
                                        _ => {
                                            if !conf_state.is_null() {
                                                *conf_state = (sealalg != 0xffffi32) as i32
                                            }
                                            if !qop_state.is_null() {
                                                *qop_state = 0u32
                                            }
                                            if (*ctx).initiate() as i32 != 0 && direction != 0xffi32
                                                || (*ctx).initiate() == 0 && direction != 0i32
                                            {
                                                *minor_status = -(2045022963 as isize)
                                                    as crate::gssapi_h::OM_uint32;
                                                retval = (6u32) << 16i32
                                            } else {
                                                code = 0u32;
                                                retval =
                                                    crate::src::generic::util_seqstate::gssint_g_seqstate_check((*ctx).seqstate,
                                                                            seqnum
                                                                                as
                                                                                crate::stdlib::uint64_t)
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                code = 0u32;
                                retval = (9u32) << 16i32
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    crate::krb5_h::krb5_free_checksum_contents(context, &mut md5cksum);
    *minor_status = code;
    return retval;
}
/*
 * Caller must provide TOKEN | DATA | PADDING | TRAILER, except
 * for DCE in which case it can just provide TOKEN | DATA (must
 * guarantee that DATA is padded)
 */

unsafe extern "C" fn kg_unseal_iov_token(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut context = (*ctx).k5_context;
    let mut ptr = 0 as *mut u8;
    let mut header = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut padding = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut trailer = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut input_length: crate::stddef_h::size_t = 0;
    let mut bodysize: u32 = 0;
    let mut toktype2: i32 = 0;
    header = crate::src::krb5::util_crypt::kg_locate_header_iov(iov, iov_count, toktype);
    if header.is_null() {
        *minor_status = 22u32;
        return (13u32) << 16i32;
    }
    padding = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 9u32);
    trailer = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 7u32);
    ptr = (*header).buffer.value as *mut u8;
    input_length = (*header).buffer.length;
    if (*ctx).gss_flags & 0x1000u32 == 0u32 && toktype == 0x201i32 {
        let mut data_length: crate::stddef_h::size_t = 0;
        let mut assoc_data_length: crate::stddef_h::size_t = 0;
        crate::src::krb5::util_crypt::kg_iov_msglen(
            iov,
            iov_count,
            &mut data_length,
            &mut assoc_data_length,
        );
        input_length = (input_length).wrapping_add(data_length.wrapping_sub(assoc_data_length));
        if !padding.is_null() {
            input_length = (input_length).wrapping_add((*padding).buffer.length)
        }
        if !trailer.is_null() {
            input_length = (input_length).wrapping_add((*trailer).buffer.length)
        }
    }
    code = crate::src::generic::util_token::gssint_g_verify_token_header(
        (*ctx).mech_used,
        &mut bodysize,
        &mut ptr,
        -(1i32),
        input_length as u32,
        0i32,
    );
    if code != 0i32 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
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
            code = crate::src::krb5::k5sealv3iov::gss_krb5int_unseal_v3_iov(
                context,
                minor_status,
                ctx,
                iov,
                iov_count,
                conf_state,
                qop_state,
                toktype,
            ) as crate::krb5_h::krb5_error_code
        }
        257 | 513 | 258 => {
            code = kg_unseal_v1_iov(
                context,
                minor_status,
                ctx,
                iov,
                iov_count,
                ptr.wrapping_offset_from((*header).buffer.value as *mut u8)
                    as crate::stddef_h::size_t,
                conf_state,
                qop_state,
                toktype,
            ) as crate::krb5_h::krb5_error_code
        }
        _ => {
            *minor_status = -(2045022964 as isize) as crate::gssapi_h::OM_uint32;
            code = ((9u32) << 16i32) as crate::krb5_h::krb5_error_code
        }
    }
    if code != 0i32 {
        crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
    }
    return code as crate::gssapi_h::OM_uint32;
}
/*
 * Split a STREAM | SIGN_DATA | DATA into
 *         HEADER | SIGN_DATA | DATA | PADDING | TRAILER
 */

unsafe extern "C" fn kg_unseal_stream_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut ptr = 0 as *mut u8;
    let mut bodysize: u32 = 0;
    let mut code = 0u32;
    let mut major_status = (13u32) << 16i32;
    let mut context = (*ctx).k5_context;
    let mut conf_req_flag: i32 = 0;
    let mut toktype2: i32 = 0;
    let mut i = 0i32;
    let mut j: i32 = 0;
    let mut tiov = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc;
    let mut stream = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut data = 0 as crate::gssapi_ext_h::gss_iov_buffer_t;
    let mut theader = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut tdata = 0 as crate::gssapi_ext_h::gss_iov_buffer_t;
    let mut tpadding = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut ttrailer = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    if toktype == 0x201i32 {
    } else {
        crate::stdlib::__assert_fail(b"toktype == KG_TOK_WRAP_MSG\x00" as *const u8 as
                          *const i8,
                      b"k5unsealiov.c\x00" as *const u8 as
                          *const i8,
                      386u32,
                      (*::std::mem::transmute::<&[u8; 120],
                                                &[i8; 120]>(b"OM_uint32 kg_unseal_stream_iov(OM_uint32 *, krb5_gss_ctx_id_rec *, int *, gss_qop_t *, gss_iov_buffer_desc *, int, int)\x00")).as_ptr());
    }
    if toktype != 0x201i32 || (*ctx).gss_flags & 0x1000u32 != 0 {
        code = 22u32
    } else {
        stream = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 10u32);
        if !stream.is_null() {
        } else {
            crate::stdlib::__assert_fail(b"stream != NULL\x00" as *const u8 as
                              *const i8,
                          b"k5unsealiov.c\x00" as *const u8 as
                              *const i8,
                          394u32,
                          (*::std::mem::transmute::<&[u8; 120],
                                                    &[i8; 120]>(b"OM_uint32 kg_unseal_stream_iov(OM_uint32 *, krb5_gss_ctx_id_rec *, int *, gss_qop_t *, gss_iov_buffer_desc *, int, int)\x00")).as_ptr());
        }
        ptr = (*stream).buffer.value as *mut u8;
        code = crate::src::generic::util_token::gssint_g_verify_token_header(
            (*ctx).mech_used,
            &mut bodysize,
            &mut ptr,
            -(1i32),
            (*stream).buffer.length as u32,
            0i32,
        ) as crate::gssapi_h::OM_uint32;
        if code != 0u32 {
            major_status = (9u32) << 16i32
        } else {
            if bodysize < 2u32 {
                *minor_status = -(2045022964 as isize) as crate::gssapi_h::OM_uint32;
                return (9u32) << 16i32;
            }
            toktype2 = load_16_be(ptr as *const libc::c_void) as i32;
            ptr = ptr.offset(2isize);
            bodysize = bodysize.wrapping_sub(2u32);
            tiov = crate::stdlib::calloc(
                (iov_count as crate::stddef_h::size_t).wrapping_add(2usize),
                ::std::mem::size_of::<crate::gssapi_ext_h::gss_iov_buffer_desc>(),
            ) as *mut crate::gssapi_ext_h::gss_iov_buffer_desc;
            if tiov.is_null() {
                code = 12u32
            } else {
                /* HEADER */
                let fresh1 = i;
                i = i + 1;
                theader = &mut *tiov.offset(fresh1 as isize)
                    as *mut crate::gssapi_ext_h::gss_iov_buffer_desc;
                (*theader).type_0 = 2u32;
                (*theader).buffer.value = (*stream).buffer.value;
                (*theader).buffer.length = ptr
                    .wrapping_offset_from((*stream).buffer.value as *mut u8)
                    as crate::stddef_h::size_t;
                if bodysize < 14u32
                    || (*stream).buffer.length
                        != (*theader).buffer.length.wrapping_add(bodysize as usize)
                {
                    major_status = (9u32) << 16i32
                } else {
                    (*theader).buffer.length = ((*theader).buffer.length).wrapping_add(14usize);
                    /* n[SIGN_DATA] | DATA | m[SIGN_DATA] */
                    j = 0i32;
                    loop {
                        if !(j < iov_count) {
                            current_block = 10891380440665537214;
                            break;
                        }
                        let mut type_0 = (*iov.offset(j as isize)).type_0 & !(0xffff0000u32);
                        if type_0 == 1u32 {
                            if !data.is_null() {
                                /* only a single DATA buffer can appear */
                                code = 22u32;
                                current_block = 8417685536221577686;
                                break;
                            } else {
                                data = &mut *iov.offset(j as isize)
                                    as *mut crate::gssapi_ext_h::gss_iov_buffer_desc;
                                tdata = &mut *tiov.offset(i as isize)
                                    as *mut crate::gssapi_ext_h::gss_iov_buffer_desc
                            }
                        }
                        if type_0 == 1u32 || type_0 == 11u32 {
                            let fresh2 = i;
                            i = i + 1;
                            *tiov.offset(fresh2 as isize) = *iov.offset(j as isize)
                        }
                        j += 1
                    }
                    match current_block {
                        8417685536221577686 => {}
                        _ => {
                            if data.is_null() {
                                /* a single DATA buffer must be present */
                                code = 22u32
                            } else {
                                /* PADDING | TRAILER */
                                let fresh3 = i;
                                i = i + 1;
                                tpadding = &mut *tiov.offset(fresh3 as isize)
                                    as *mut crate::gssapi_ext_h::gss_iov_buffer_desc;
                                (*tpadding).type_0 = 9u32;
                                (*tpadding).buffer.length = 0usize;
                                (*tpadding).buffer.value = 0 as *mut libc::c_void;
                                let fresh4 = i;
                                i = i + 1;
                                ttrailer = &mut *tiov.offset(fresh4 as isize)
                                    as *mut crate::gssapi_ext_h::gss_iov_buffer_desc;
                                (*ttrailer).type_0 = 7u32;
                                match toktype2 {
                                    1028 | 1284 | 1029 => {
                                        let mut ec: crate::stddef_h::size_t = 0;
                                        let mut rrc: crate::stddef_h::size_t = 0;
                                        let mut enctype: crate::krb5_h::krb5_enctype = 0;
                                        let mut k5_headerlen = 0u32;
                                        let mut k5_trailerlen = 0u32;
                                        if (*ctx).have_acceptor_subkey() != 0 {
                                            enctype = (*(*ctx).acceptor_subkey).keyblock.enctype
                                        } else {
                                            enctype = (*(*ctx).subkey).keyblock.enctype
                                        }
                                        conf_req_flag =
                                            (*ptr.offset(0isize) as i32 & 0x2i32 != 0i32) as i32;
                                        ec = if conf_req_flag != 0 {
                                            load_16_be(ptr.offset(2isize) as *const libc::c_void)
                                                as i32
                                        } else {
                                            0i32
                                        }
                                            as crate::stddef_h::size_t;
                                        rrc = load_16_be(ptr.offset(4isize) as *const libc::c_void)
                                            as crate::stddef_h::size_t;
                                        if rrc != 0usize {
                                            if crate::src::krb5::k5sealv3::gss_krb5int_rotate_left(
                                                ((*stream).buffer.value as *mut u8).offset(16isize)
                                                    as *mut libc::c_void,
                                                (*stream).buffer.length.wrapping_sub(16usize),
                                                rrc,
                                            ) == 0
                                            {
                                                code = 12u32;
                                                current_block = 8417685536221577686;
                                            } else {
                                                store_16_be(
                                                    0u32,
                                                    ptr.offset(4isize) as *mut libc::c_void,
                                                );
                                                current_block = 1623252117315916725;
                                            }
                                        /* set RRC to zero */
                                        } else {
                                            current_block = 1623252117315916725;
                                        }
                                        match current_block {
                                            8417685536221577686 => {}
                                            _ => {
                                                if conf_req_flag != 0 {
                                                    code = crate::krb5_h::krb5_c_crypto_length(
                                                        context,
                                                        enctype,
                                                        1i32,
                                                        &mut k5_headerlen,
                                                    )
                                                        as crate::gssapi_h::OM_uint32;
                                                    if code != 0u32 {
                                                        current_block = 8417685536221577686;
                                                    } else {
                                                        (*theader).buffer.length = ((*theader)
                                                            .buffer
                                                            .length)
                                                            .wrapping_add(k5_headerlen as usize);
                                                        current_block = 8869332144787829186;
                                                    }
                                                /* length validated later */
                                                } else {
                                                    current_block = 8869332144787829186;
                                                }
                                                match current_block {
                                                    8417685536221577686 => {}
                                                    _ => {
                                                        /* no PADDING for CFX, EC is used instead */
                                                        code = crate::krb5_h::krb5_c_crypto_length(
                                                            context,
                                                            enctype,
                                                            if conf_req_flag != 0 {
                                                                5i32
                                                            } else {
                                                                6i32
                                                            },
                                                            &mut k5_trailerlen,
                                                        )
                                                            as crate::gssapi_h::OM_uint32;
                                                        if code != 0u32 {
                                                            current_block = 8417685536221577686;
                                                        } else {
                                                            (*ttrailer).buffer.length = ec
                                                                .wrapping_add(
                                                                    (if conf_req_flag != 0 {
                                                                        16i32
                                                                    } else {
                                                                        0i32
                                                                    })
                                                                        as usize,
                                                                )
                                                                .wrapping_add(
                                                                    k5_trailerlen as usize,
                                                                );
                                                            (*ttrailer).buffer.value =
                                                                ((*stream).buffer.value as *mut u8)
                                                                    .offset(
                                                                        (*stream).buffer.length
                                                                            as isize,
                                                                    )
                                                                    .offset(
                                                                        -((*ttrailer).buffer.length
                                                                            as isize),
                                                                    )
                                                                    as *mut libc::c_void;
                                                            current_block = 17485376261910781866;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    257 | 513 | 258 => {
                                        (*theader).buffer.length = ((*theader).buffer.length)
                                            .wrapping_add((*ctx).cksum_size.wrapping_add(
                                                crate::src::krb5::util_crypt::kg_confounder_size(
                                                    context,
                                                    (*(*ctx).enc).keyblock.enctype,
                                                )
                                                    as usize,
                                            ));
                                        /*
                                         * we can't set the padding accurately until decryption;
                                         * kg_fixup_padding_iov() will take care of this
                                         */
                                        (*tpadding).buffer.length = 1usize;
                                        (*tpadding).buffer.value = ((*stream).buffer.value
                                            as *mut u8)
                                            .offset((*stream).buffer.length as isize)
                                            .offset(-(1isize))
                                            as *mut libc::c_void;
                                        /* no TRAILER for pre-CFX */
                                        (*ttrailer).buffer.length = 0usize;
                                        (*ttrailer).buffer.value = 0 as *mut libc::c_void;
                                        current_block = 17485376261910781866;
                                    }
                                    _ => {
                                        code = -(2045022964 as isize) as crate::gssapi_h::OM_uint32;
                                        major_status = (9u32) << 16i32;
                                        current_block = 8417685536221577686;
                                    }
                                }
                                match current_block {
                                    8417685536221577686 => {}
                                    _ =>
                                    /* IOV: -----------0-------------+---1---+--2--+----------------3--------------*/
                                    /* Old: GSS-Header | Conf        | Data  | Pad |                               */
                                    /* CFX: GSS-Header | Kerb-Header | Data  |     | EC | E(Header) | Kerb-Trailer */
                                    /* GSS: -------GSS-HEADER--------+-DATA--+-PAD-+----------GSS-TRAILER----------*/
                                    /* validate lengths */
                                    {
                                        if (*stream).buffer.length
                                            < (*theader)
                                                .buffer
                                                .length
                                                .wrapping_add((*tpadding).buffer.length)
                                                .wrapping_add((*ttrailer).buffer.length)
                                        {
                                            code = -(1765328194 as isize)
                                                as crate::gssapi_h::OM_uint32;
                                            major_status = (9u32) << 16i32
                                        } else {
                                            /* setup data */
                                            (*tdata).buffer.length = (*stream)
                                                .buffer
                                                .length
                                                .wrapping_sub((*ttrailer).buffer.length)
                                                .wrapping_sub((*tpadding).buffer.length)
                                                .wrapping_sub((*theader).buffer.length);
                                            if !data.is_null() {
                                            } else {
                                                crate::stdlib::__assert_fail(b"data != NULL\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const i8,
                                                              b"k5unsealiov.c\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const i8,
                                                              557u32,
                                                              (*::std::mem::transmute::<&[u8; 120],
                                                                                        &[i8; 120]>(b"OM_uint32 kg_unseal_stream_iov(OM_uint32 *, krb5_gss_ctx_id_rec *, int *, gss_qop_t *, gss_iov_buffer_desc *, int, int)\x00")).as_ptr());
                                            }
                                            if (*data).type_0 & 0x10000u32 != 0 {
                                                code = crate::src::krb5::util_crypt::kg_allocate_iov(
                                                    tdata,
                                                    (*tdata).buffer.length,
                                                )
                                                    as crate::gssapi_h::OM_uint32;
                                                if code != 0u32 {
                                                    current_block = 8417685536221577686;
                                                } else {
                                                    crate::stdlib::memcpy(
                                                        (*tdata).buffer.value,
                                                        ((*stream).buffer.value as *mut u8).offset(
                                                            (*theader).buffer.length as isize,
                                                        )
                                                            as *const libc::c_void,
                                                        (*tdata).buffer.length,
                                                    );
                                                    current_block = 14541395414537699361;
                                                }
                                            } else {
                                                (*tdata).buffer.value = ((*stream).buffer.value
                                                    as *mut u8)
                                                    .offset((*theader).buffer.length as isize)
                                                    as *mut libc::c_void;
                                                current_block = 14541395414537699361;
                                            }
                                            match current_block {
                                                8417685536221577686 => {}
                                                _ => {
                                                    if i <= iov_count + 2i32 {
                                                    } else {
                                                        crate::stdlib::__assert_fail(b"i <= iov_count + 2\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const i8,
                                                                      b"k5unsealiov.c\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const i8,
                                                                      568u32,
                                                                      (*::std::mem::transmute::<&[u8; 120],
                                                                                                &[i8; 120]>(b"OM_uint32 kg_unseal_stream_iov(OM_uint32 *, krb5_gss_ctx_id_rec *, int *, gss_qop_t *, gss_iov_buffer_desc *, int, int)\x00")).as_ptr());
                                                    }
                                                    major_status = kg_unseal_iov_token(
                                                        &mut code, ctx, conf_state, qop_state,
                                                        tiov, i, toktype,
                                                    );
                                                    if major_status == 0u32 {
                                                        *data = *tdata
                                                    } else {
                                                        crate::src::krb5::util_crypt::kg_release_iov(tdata,
                                                                       1i32);
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
    if !tiov.is_null() {
        crate::stdlib::free(tiov as *mut libc::c_void);
    }
    *minor_status = code;
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn kg_unseal_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut code: crate::gssapi_h::OM_uint32 = 0;
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    if (*ctx).terminated() as i32 != 0 || (*ctx).established() == 0 {
        *minor_status = 39756039u32;
        return (8u32) << 16i32;
    }
    if !crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 10u32).is_null() {
        code = kg_unseal_stream_iov(
            minor_status,
            ctx,
            conf_state,
            qop_state,
            iov,
            iov_count,
            toktype,
        )
    } else {
        code = kg_unseal_iov_token(
            minor_status,
            ctx,
            conf_state,
            qop_state,
            iov,
            iov_count,
            toktype,
        )
    }
    return code;
}
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
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_unwrap_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    major_status = kg_unseal_iov(
        minor_status,
        context_handle,
        conf_state,
        qop_state,
        iov,
        iov_count,
        0x201i32,
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
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_verify_mic_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    major_status = kg_unseal_iov(
        minor_status,
        context_handle,
        0 as *mut i32,
        qop_state,
        iov,
        iov_count,
        0x101i32,
    );
    return major_status;
}
