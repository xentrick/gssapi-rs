use ::libc;

pub mod k5_platform_h {

    #[inline]

    pub unsafe extern "C" fn store_32_be(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_6)).i = __bswap_32(val);
    }
    #[inline]

    pub unsafe extern "C" fn store_32_le(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_6)).i = val;
    }

    use crate::src::krb5::util_cksum::byteswap_h::__bswap_32;
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

/* _GSSAPIP_KRB5_H_ */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_context;
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::derived_key;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_key_st;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
pub use crate::k5_int_h::localauth_module_handle;
pub use crate::k5_int_h::plugin_interface;
pub use crate::k5_int_h::plugin_mapping;
pub use crate::k5_int_h::CANONHOST_FALLBACK;
pub use crate::k5_int_h::CANONHOST_FALSE;
pub use crate::k5_int_h::CANONHOST_TRUE;
pub use crate::k5_platform_h::C2RustUnnamed_6;
pub use crate::k5_plugin_h::plugin_dir_handle;
pub use crate::k5_plugin_h::plugin_file_handle;
pub use crate::krb5_h::_krb5_checksum;
pub use crate::krb5_h::_krb5_crypto_iov;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::_krb5_keyblock;
pub use crate::krb5_h::_krb5_trace_info;
pub use crate::krb5_h::_profile_t;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_c_checksum_length;
pub use crate::krb5_h::krb5_c_crypto_length;
pub use crate::krb5_h::krb5_c_make_checksum;
pub use crate::krb5_h::krb5_checksum;
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_crypto_iov;
pub use crate::krb5_h::krb5_cryptotype;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_checksum_contents;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_k_make_checksum_iov;
pub use crate::krb5_h::krb5_k_verify_checksum_iov;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keyusage;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::util_cksum::k5_platform_h::store_32_be;
pub use crate::src::krb5::util_cksum::k5_platform_h::store_32_le;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;

pub use crate::gssapi_ext_h::gss_iov_buffer_desc;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
pub use crate::gssapi_ext_h::gss_iov_buffer_t;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_channel_bindings_struct;
pub use crate::gssapi_h::gss_channel_bindings_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::src::krb5::util_cksum::byteswap_h::__bswap_32;

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
/* Checksumming the channel bindings always uses plain MD5.  */
#[no_mangle]

pub unsafe extern "C" fn kg_checksum_channel_bindings(
    mut context: crate::krb5_h::krb5_context,
    mut cb: crate::gssapi_h::gss_channel_bindings_t,
    mut cksum: *mut crate::krb5_h::krb5_checksum,
) -> crate::krb5_h::krb5_error_code {
    let mut len: crate::stddef_h::size_t = 0;
    let mut buf = 0 as *mut i8;
    let mut ptr = 0 as *mut i8;
    let mut sumlen: crate::stddef_h::size_t = 0;
    let mut plaind = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut temp = 0 as *mut libc::c_void;
    /* initialize the the cksum */
    code = crate::krb5_h::krb5_c_checksum_length(context, 0x7i32, &mut sumlen);
    if code != 0 {
        return code;
    }
    (*cksum).checksum_type = 0x7i32;
    (*cksum).length = sumlen as u32;
    (*cksum).magic = -(1760647420 as isize) as crate::krb5_h::krb5_magic;
    /* generate a buffer full of zeros if no cb specified */
    if cb.is_null() {
        (*cksum).contents =
            crate::stdlib::malloc((*cksum).length as usize) as *mut crate::krb5_h::krb5_octet;
        if (*cksum).contents.is_null() {
            return 12i32;
        }
        crate::stdlib::memset(
            (*cksum).contents as *mut libc::c_void,
            '\u{0}' as i32,
            (*cksum).length as usize,
        );
        return 0i32;
    }
    /* create the buffer to checksum into */
    len = (::std::mem::size_of::<crate::krb5_h::krb5_int32>())
        .wrapping_mul(5usize)
        .wrapping_add((*cb).initiator_address.length)
        .wrapping_add((*cb).acceptor_address.length)
        .wrapping_add((*cb).application_data.length);
    buf = crate::stdlib::malloc(len) as *mut i8;
    if buf.is_null() {
        return 12i32;
    }
    /* helper macros.  This code currently depends on a long being 32
    bits, and htonl dtrt. */
    ptr = buf;
    store_32_le((*cb).initiator_addrtype, ptr as *mut libc::c_void);
    ptr = ptr.offset(4isize);
    store_32_le(
        (*cb).initiator_address.length as u32,
        ptr as *mut libc::c_void,
    );
    ptr = ptr.offset(4isize);
    crate::stdlib::memcpy(
        ptr as *mut libc::c_void,
        (*cb).initiator_address.value,
        (*cb).initiator_address.length,
    );
    ptr = ptr.offset((*cb).initiator_address.length as isize);
    store_32_le((*cb).acceptor_addrtype, ptr as *mut libc::c_void);
    ptr = ptr.offset(4isize);
    store_32_le(
        (*cb).acceptor_address.length as u32,
        ptr as *mut libc::c_void,
    );
    ptr = ptr.offset(4isize);
    crate::stdlib::memcpy(
        ptr as *mut libc::c_void,
        (*cb).acceptor_address.value,
        (*cb).acceptor_address.length,
    );
    ptr = ptr.offset((*cb).acceptor_address.length as isize);
    store_32_le(
        (*cb).application_data.length as u32,
        ptr as *mut libc::c_void,
    );
    ptr = ptr.offset(4isize);
    crate::stdlib::memcpy(
        ptr as *mut libc::c_void,
        (*cb).application_data.value,
        (*cb).application_data.length,
    );
    ptr = ptr.offset((*cb).application_data.length as isize);
    /* checksum the data */
    plaind.length = len as u32;
    plaind.data = buf;
    code = crate::krb5_h::krb5_c_make_checksum(
        context,
        0x7i32,
        0 as *const crate::krb5_h::krb5_keyblock,
        0i32,
        &mut plaind,
        cksum,
    );
    if !(code != 0) {
        temp = crate::stdlib::malloc((*cksum).length as usize);
        if temp.is_null() {
            crate::krb5_h::krb5_free_checksum_contents(context, cksum);
            code = 12i32
        } else {
            crate::stdlib::memcpy(
                temp,
                (*cksum).contents as *const libc::c_void,
                (*cksum).length as usize,
            );
            crate::krb5_h::krb5_free_checksum_contents(context, cksum);
            (*cksum).contents = temp as *mut crate::krb5_h::krb5_octet
        }
    }
    /* success */
    if !buf.is_null() {
        crate::stdlib::free(buf as *mut libc::c_void);
    }
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn kg_make_checksum_iov_v1(
    mut context: crate::krb5_h::krb5_context,
    mut type_0: crate::krb5_h::krb5_cksumtype,
    mut cksum_len: crate::stddef_h::size_t,
    mut seq: crate::krb5_h::krb5_key,
    mut enc: crate::krb5_h::krb5_key,
    mut sign_usage: crate::krb5_h::krb5_keyusage,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
    mut checksum: *mut crate::krb5_h::krb5_checksum,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut header = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc;
    let mut kiov = 0 as *mut crate::krb5_h::krb5_crypto_iov;
    let mut i = 0i32;
    let mut j: i32 = 0;
    let mut conf_len = 0usize;
    let mut token_header_len: crate::stddef_h::size_t = 0;
    header = crate::src::krb5::util_crypt::kg_locate_header_iov(iov, iov_count, toktype);
    if !header.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"header != NULL\x00" as *const u8 as
                          *const i8,
                      b"util_cksum.c\x00" as *const u8 as *const i8,
                      130u32,
                      (*::std::mem::transmute::<&[u8; 163],
                                                &[i8; 163]>(b"krb5_error_code kg_make_checksum_iov_v1(krb5_context, krb5_cksumtype, size_t, krb5_key, krb5_key, krb5_keyusage, gss_iov_buffer_desc *, int, int, krb5_checksum *)\x00")).as_ptr());
    }
    kiov = crate::stdlib::calloc(
        (iov_count + 3i32) as usize,
        ::std::mem::size_of::<crate::krb5_h::krb5_crypto_iov>(),
    ) as *mut crate::krb5_h::krb5_crypto_iov;
    if kiov.is_null() {
        return 12i32;
    }
    /* Checksum over ( Header | Confounder | Data | Pad ) */
    if toktype == 0x201i32 {
        conf_len =
            crate::src::krb5::util_crypt::kg_confounder_size(context, (*enc).keyblock.enctype)
                as crate::stddef_h::size_t
    }
    /* Checksum output */
    (*kiov.offset(i as isize)).flags = 6i32;
    (*kiov.offset(i as isize)).data.length = (*checksum).length;
    let ref mut fresh0 = (*kiov.offset(i as isize)).data.data;
    *fresh0 = crate::stdlib::malloc((*checksum).length as usize) as *mut i8;
    if (*kiov.offset(i as isize)).data.data.is_null() {
        crate::stdlib::free(kiov as *mut libc::c_void);
        return 12i32;
    }
    i += 1;
    /* Header | SND_SEQ | SGN_CKSUM | Confounder */
    token_header_len = (16usize).wrapping_add(cksum_len).wrapping_add(conf_len);
    /* Header (calculate from end because of variable length ASN.1 header) */
    (*kiov.offset(i as isize)).flags = 3i32;
    (*kiov.offset(i as isize)).data.length = 8u32;
    let ref mut fresh1 = (*kiov.offset(i as isize)).data.data;
    *fresh1 = ((*header).buffer.value as *mut i8)
        .offset((*header).buffer.length as isize)
        .offset(-(token_header_len as isize));
    i += 1;
    /* Confounder */
    if toktype == 0x201i32 {
        (*kiov.offset(i as isize)).flags = 2i32;
        (*kiov.offset(i as isize)).data.length = conf_len as u32;
        let ref mut fresh2 = (*kiov.offset(i as isize)).data.data;
        *fresh2 = ((*header).buffer.value as *mut i8)
            .offset((*header).buffer.length as isize)
            .offset(-(conf_len as isize));
        i += 1
    }
    j = 0i32;
    while j < iov_count {
        (*kiov.offset(i as isize)).flags =
            crate::src::krb5::util_crypt::kg_translate_flag_iov((*iov.offset(j as isize)).type_0);
        (*kiov.offset(i as isize)).data.length = (*iov.offset(j as isize)).buffer.length as u32;
        let ref mut fresh3 = (*kiov.offset(i as isize)).data.data;
        *fresh3 = (*iov.offset(j as isize)).buffer.value as *mut i8;
        i += 1;
        j += 1
    }
    code = crate::krb5_h::krb5_k_make_checksum_iov(
        context,
        type_0,
        seq,
        sign_usage,
        kiov,
        i as crate::stddef_h::size_t,
    );
    if code == 0i32 {
        (*checksum).length = (*kiov.offset(0isize)).data.length;
        (*checksum).contents = (*kiov.offset(0isize)).data.data as *mut u8
    } else {
        crate::stdlib::free((*kiov.offset(0isize)).data.data as *mut libc::c_void);
    }
    crate::stdlib::free(kiov as *mut libc::c_void);
    return code;
}

unsafe extern "C" fn checksum_iov_v3(
    mut context: crate::krb5_h::krb5_context,
    mut type_0: crate::krb5_h::krb5_cksumtype,
    mut rrc: crate::stddef_h::size_t,
    mut key: crate::krb5_h::krb5_key,
    mut sign_usage: crate::krb5_h::krb5_keyusage,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
    mut verify: crate::krb5_h::krb5_boolean,
    mut valid: *mut crate::krb5_h::krb5_boolean,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut header = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc;
    let mut trailer = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc;
    let mut kiov = 0 as *mut crate::krb5_h::krb5_crypto_iov;
    let mut kiov_count: crate::stddef_h::size_t = 0;
    let mut i = 0i32;
    let mut j: i32 = 0;
    let mut k5_checksumlen: u32 = 0;
    if verify != 0 {
        *valid = 0u32
    }
    code = crate::krb5_h::krb5_c_crypto_length(
        context,
        (*key).keyblock.enctype,
        6i32,
        &mut k5_checksumlen,
    );
    if code != 0i32 {
        return code;
    }
    header = crate::src::krb5::util_crypt::kg_locate_header_iov(iov, iov_count, toktype);
    if !header.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"header != NULL\x00" as *const u8 as
                          *const i8,
                      b"util_cksum.c\x00" as *const u8 as *const i8,
                      214u32,
                      (*::std::mem::transmute::<&[u8; 158],
                                                &[i8; 158]>(b"krb5_error_code checksum_iov_v3(krb5_context, krb5_cksumtype, size_t, krb5_key, krb5_keyusage, gss_iov_buffer_desc *, int, int, krb5_boolean, krb5_boolean *)\x00")).as_ptr());
    }
    trailer = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 7u32);
    if rrc != 0usize || !trailer.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"rrc != 0 || trailer != NULL\x00" as *const u8 as
                          *const i8,
                      b"util_cksum.c\x00" as *const u8 as *const i8,
                      217u32,
                      (*::std::mem::transmute::<&[u8; 158],
                                                &[i8; 158]>(b"krb5_error_code checksum_iov_v3(krb5_context, krb5_cksumtype, size_t, krb5_key, krb5_keyusage, gss_iov_buffer_desc *, int, int, krb5_boolean, krb5_boolean *)\x00")).as_ptr());
    }
    if trailer.is_null() {
        if rrc != k5_checksumlen as usize {
            return -(1765328194 as isize) as crate::krb5_h::krb5_error_code;
        }
        if (*header).buffer.length != (16u32).wrapping_add(k5_checksumlen) as usize {
            return -(1765328194 as isize) as crate::krb5_h::krb5_error_code;
        }
    } else if (*trailer).buffer.length != k5_checksumlen as usize {
        return -(1765328194 as isize) as crate::krb5_h::krb5_error_code;
    }
    kiov_count = (2i32 + iov_count) as crate::stddef_h::size_t;
    kiov = crate::stdlib::malloc(
        kiov_count.wrapping_mul(::std::mem::size_of::<crate::krb5_h::krb5_crypto_iov>()),
    ) as *mut crate::krb5_h::krb5_crypto_iov;
    if kiov.is_null() {
        return 12i32;
    }
    /* Checksum over ( Data | Header ) */
    /* Data */
    j = 0i32;
    while j < iov_count {
        (*kiov.offset(i as isize)).flags =
            crate::src::krb5::util_crypt::kg_translate_flag_iov((*iov.offset(j as isize)).type_0);
        (*kiov.offset(i as isize)).data.length = (*iov.offset(j as isize)).buffer.length as u32;
        let ref mut fresh4 = (*kiov.offset(i as isize)).data.data;
        *fresh4 = (*iov.offset(j as isize)).buffer.value as *mut i8;
        i += 1;
        j += 1
    }
    /* Header */
    (*kiov.offset(i as isize)).flags = 3i32;
    (*kiov.offset(i as isize)).data.length = 16u32;
    let ref mut fresh5 = (*kiov.offset(i as isize)).data.data;
    *fresh5 = (*header).buffer.value as *mut i8;
    i += 1;
    /* Checksum */
    (*kiov.offset(i as isize)).flags = 6i32;
    if trailer.is_null() {
        (*kiov.offset(i as isize)).data.length =
            (*header).buffer.length.wrapping_sub(16usize) as u32;
        let ref mut fresh6 = (*kiov.offset(i as isize)).data.data;
        *fresh6 = ((*header).buffer.value as *mut i8).offset(16isize)
    } else {
        (*kiov.offset(i as isize)).data.length = (*trailer).buffer.length as u32;
        let ref mut fresh7 = (*kiov.offset(i as isize)).data.data;
        *fresh7 = (*trailer).buffer.value as *mut i8
    }
    i += 1;
    if verify != 0 {
        code = crate::krb5_h::krb5_k_verify_checksum_iov(
            context, type_0, key, sign_usage, kiov, kiov_count, valid,
        )
    } else {
        code = crate::krb5_h::krb5_k_make_checksum_iov(
            context, type_0, key, sign_usage, kiov, kiov_count,
        )
    }
    crate::stdlib::free(kiov as *mut libc::c_void);
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn kg_make_checksum_iov_v3(
    mut context: crate::krb5_h::krb5_context,
    mut type_0: crate::krb5_h::krb5_cksumtype,
    mut rrc: crate::stddef_h::size_t,
    mut key: crate::krb5_h::krb5_key,
    mut sign_usage: crate::krb5_h::krb5_keyusage,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
) -> crate::krb5_h::krb5_error_code {
    return checksum_iov_v3(
        context,
        type_0,
        rrc,
        key,
        sign_usage,
        iov,
        iov_count,
        toktype,
        0u32,
        0 as *mut crate::krb5_h::krb5_boolean,
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
#[no_mangle]

pub unsafe extern "C" fn kg_verify_checksum_iov_v3(
    mut context: crate::krb5_h::krb5_context,
    mut type_0: crate::krb5_h::krb5_cksumtype,
    mut rrc: crate::stddef_h::size_t,
    mut key: crate::krb5_h::krb5_key,
    mut sign_usage: crate::krb5_h::krb5_keyusage,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
    mut valid: *mut crate::krb5_h::krb5_boolean,
) -> crate::krb5_h::krb5_error_code {
    return checksum_iov_v3(
        context, type_0, rrc, key, sign_usage, iov, iov_count, toktype, 1u32, valid,
    );
}
