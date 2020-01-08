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

    use crate::src::mechglue::g_glue::byteswap_h::__bswap_16;
    use crate::src::mechglue::g_glue::byteswap_h::__bswap_32;

    /* K5_PLATFORM_H */
}

pub mod byteswap_h {
    #[inline]

    pub unsafe extern "C" fn __bswap_16(
        mut __bsx: crate::stdlib::__uint16_t,
    ) -> crate::stdlib::__uint16_t {
        return (__bsx as i32 >> 8i32 & 0xffi32 | (__bsx as i32 & 0xffi32) << 8i32)
            as crate::stdlib::__uint16_t;
    }
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

pub mod gssapi_alloc_h {
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
}

/* _GSSAPIP_GENERIC_H_ */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::ssize_t;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;

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
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_channel_bindings_struct;
pub use crate::gssapi_h::gss_channel_bindings_t;
pub use crate::gssapi_h::gss_const_OID;
pub use crate::gssapi_h::gss_const_buffer_t;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::k5_platform_h::C2RustUnnamed_5;
pub use crate::k5_platform_h::C2RustUnnamed_6;
pub use crate::mglueP_h::gss_config;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_mechanism;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::mglueP_h::gss_union_cred_t;
pub use crate::mglueP_h::gss_union_ctx_id_struct;
pub use crate::mglueP_h::gss_union_ctx_id_t;
pub use crate::mglueP_h::gss_union_name_desc;
pub use crate::mglueP_h::gss_union_name_t;

pub use crate::src::mechglue::g_buffer_set::gss_release_buffer_set;
pub use crate::src::mechglue::g_glue::byteswap_h::__bswap_16;
pub use crate::src::mechglue::g_glue::byteswap_h::__bswap_32;
pub use crate::src::mechglue::g_glue::gssapi_alloc_h::gssalloc_malloc;
pub use crate::src::mechglue::g_glue::k5_platform_h::store_16_be;
pub use crate::src::mechglue::g_glue::k5_platform_h::store_32_be;
pub use crate::src::mechglue::g_initialize::gss_release_oid;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;

/*
 * This file contains the support routines for the glue layer.
 */
/*
 * get_der_length: Givin a pointer to a buffer that contains a DER encoded
 * length, decode the length updating the buffer to point to the character
 * after the DER encoding. The parameter bytes will point to the number of
 * bytes that made up the DER encoding of the length originally pointed to
 * by the buffer. Note we return -1 on error.
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_get_der_length(
    mut buf: *mut *mut u8,
    mut buf_len: u32,
    mut bytes: *mut u32,
) -> i32 {
    /* p points to the beginning of the buffer */
    let mut p = *buf;
    let mut length: i32 = 0;
    let mut new_length: i32 = 0;
    let mut octets: u32 = 0;
    if buf_len < 1u32 {
        return -(1i32);
    }
    /* We should have at least one byte */
    *bytes = 1u32;
    /*
     * If the High order bit is not set then the length is just the value
     * of *p.
     */
    if (*p as i32) < 128i32 {
        *buf = p.offset(1isize);
        return *p as i32; /* Advance the buffer */
        /* return the length */
    }
    /*
     * if the High order bit is set, then the low order bits represent
     * the number of bytes that contain the DER encoding of the length.
     */
    let fresh0 = p;
    p = p.offset(1);
    octets = (*fresh0 as i32 & 0x7fi32) as u32;
    *bytes = (*bytes).wrapping_add(octets);
    /* See if the supplied buffer contains enough bytes for the length. */
    if octets > buf_len.wrapping_sub(1u32) {
        return -(1i32);
    }
    /*
     * Calculate a multibyte length. The length is encoded as an
     * unsigned integer base 256.
     */
    length = 0i32;
    while octets != 0 {
        let fresh1 = p;
        p = p.offset(1);
        new_length = (length << 8i32) + *fresh1 as i32;
        if new_length < length {
            /* overflow */
            return -(1i32);
        } /* Advance the buffer */
        length = new_length;
        octets = octets.wrapping_sub(1)
    }
    *buf = p;
    return length;
}
/*
 * der_length_size: Return the number of bytes to encode a given length.
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_der_length_size(mut len: u32) -> u32 {
    let mut i: i32 = 0;
    if len < 128u32 {
        return 1u32;
    }
    i = 0i32;
    while len != 0 {
        len >>= 8i32;
        i += 1
    }
    return (i + 1i32) as u32;
}
/* minor_status */
/* oid set */
/* new oid set */
/* name_type */
/* minor_status */
/* name_type */
/* mech */
/*
 * Sun extensions to GSS-API v2
 */
/* buf */
/* buf_len */
/* bytes */
/* len */
/*
 * put_der_length: Encode the supplied length into the buffer pointed to
 * by buf. max_length represents the maximum length of the buffer pointed
 * to by buff. We will advance buf to point to the character after the newly
 * DER encoded length. We return 0 on success or -l it the length cannot
 * be encoded in max_len characters.
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_put_der_length(
    mut length: u32,
    mut buf: *mut *mut u8,
    mut max_len: u32,
) -> i32 {
    let mut s = 0 as *mut u8;
    let mut p = 0 as *mut u8;
    let mut buf_len = 0u32;
    let mut i: i32 = 0;
    let mut first: i32 = 0;
    /* Oops */
    if buf.is_null() || max_len < 1u32 {
        return -(1i32);
    }
    s = *buf;
    /* Single byte is the length */
    if length < 128u32 {
        let fresh2 = s;
        s = s.offset(1);
        *fresh2 = length as u8;
        *buf = s;
        return 0i32;
    }
    /* First byte contains the number of octets */
    p = s.offset(1isize);
    /* Running total of the DER encoding length */
    buf_len = 0u32;
    /*
     * Encode MSB first. We do the encoding by setting a shift
     * factor to MSO_BIT (24 for 32 bit words) and then shifting the length
     * by the factor. We then encode the resulting low order byte.
     * We subtract 8 from the shift factor and repeat to ecnode the next
     * byte. We stop when the shift factor is zero or we've run out of
     * buffer to encode into.
     */
    first = 0i32;
    i = (8usize).wrapping_mul((::std::mem::size_of::<i32>()).wrapping_sub(1usize)) as i32;
    while i >= 0i32 && buf_len <= max_len {
        let mut v: u32 = 0;
        v = length >> i & 0xffu32;
        if v != 0 || first != 0 {
            buf_len = buf_len.wrapping_add(1u32);
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = v as u8;
            first = 1i32
        }
        i -= 8i32
    }
    if i >= 0i32 {
        /* buffer overflow */
        return -(1i32);
    }
    /*
     * We go back now and set the first byte to be the length with
     * the high order bit set.
     */
    *s = (buf_len | 0x80u32) as u8;
    *buf = p;
    return 0i32;
}
/*
 *  glue routine for get_mech_type
 *
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_get_mech_type_oid(
    mut OID: crate::gssapi_h::gss_OID,
    mut token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut buffer_ptr = 0 as *mut u8;
    let mut buflen: crate::stddef_h::size_t = 0;
    let mut lenbytes: crate::stddef_h::size_t = 0;
    let mut length: crate::stddef_h::size_t = 0;
    let mut oidlen: crate::stddef_h::size_t = 0;
    /*
     * This routine reads the prefix of "token" in order to determine
     * its mechanism type. It assumes the encoding suggested in
     * Appendix B of RFC 1508. This format starts out as follows :
     *
     * tag for APPLICATION 0, Sequence[constructed, definite length]
     * length of remainder of token
     * tag of OBJECT IDENTIFIER
     * length of mechanism OID
     * encoding of mechanism OID
     * <the rest of the token>
     *
     * Numerically, this looks like :
     *
     * 0x60
     * <length> - could be multiple bytes
     * 0x06
     * <length> - assume only one byte, hence OID length < 127
     * <mech OID bytes>
     *
     * The routine fills in the OID value and returns an error as necessary.
     */
    if OID.is_null() {
        return (2u32) << 24i32;
    }
    if token.is_null() || (*token).value.is_null() {
        return (9u32) << 16i32;
    }
    /* Skip past the APP/Sequnce byte and the token length */
    buffer_ptr = (*token).value as *mut u8;
    buflen = (*token).length;
    if buflen < 2usize || {
        let fresh4 = buffer_ptr;
        buffer_ptr = buffer_ptr.offset(1);
        (*fresh4 as i32) != 0x60i32
    } {
        return (9u32) << 16i32;
    }
    let fresh5 = buffer_ptr;
    buffer_ptr = buffer_ptr.offset(1);
    length = *fresh5 as crate::stddef_h::size_t;
    buflen = (buflen).wrapping_sub(2usize);
    /* check if token length is null */
    if length == 0usize {
        return (9u32) << 16i32;
    }
    if length & 0x80usize != 0 {
        lenbytes = length & 0x7fusize;
        if lenbytes > 4usize || lenbytes > buflen {
            return (9u32) << 16i32;
        }
        buffer_ptr = buffer_ptr.offset(lenbytes as isize);
        buflen = (buflen).wrapping_sub(lenbytes)
    }
    if buflen < 2usize || {
        let fresh6 = buffer_ptr;
        buffer_ptr = buffer_ptr.offset(1);
        (*fresh6 as i32) != 0x6i32
    } {
        return (9u32) << 16i32;
    }
    let fresh7 = buffer_ptr;
    buffer_ptr = buffer_ptr.offset(1);
    oidlen = *fresh7 as crate::stddef_h::size_t;
    buflen = (buflen).wrapping_sub(2usize);
    if oidlen > 0x7fusize || oidlen > buflen {
        return (9u32) << 16i32;
    }
    (*OID).length = oidlen as crate::gssapi_h::OM_uint32;
    (*OID).elements = buffer_ptr as *mut libc::c_void;
    return 0u32;
}
/*
 * The following mechanisms do not always identify themselves
 * per the GSS-API specification, when interoperating with MS
 * peers. We include the OIDs here so we do not have to ilnk
 * with the mechanism.
 */

static mut gss_ntlm_mechanism_oid_desc: crate::gssapi_h::gss_OID_desc = {
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 10u32,
        elements: b"+\x06\x01\x04\x01\x827\x02\x02\n\x00" as *const u8 as *mut libc::c_void,
    };
    init
};

static mut gss_spnego_mechanism_oid_desc: crate::gssapi_h::gss_OID_desc = {
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 6u32,
        elements: b"+\x06\x01\x05\x05\x02\x00" as *const u8 as *mut libc::c_void,
    };
    init
};

static mut gss_krb5_mechanism_oid_desc: crate::gssapi_h::gss_OID_desc = {
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 9u32,
        elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x00" as *const u8 as *mut libc::c_void,
    };
    init
};
#[no_mangle]

pub unsafe extern "C" fn gssint_get_mech_type(
    mut OID: crate::gssapi_h::gss_OID,
    mut token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Check for interoperability exceptions */
    if (*token).length >= ::std::mem::size_of::<[i8; 8]>()
        && crate::stdlib::memcmp(
            (*token).value,
            b"NTLMSSP\x00" as *const u8 as *const libc::c_void,
            ::std::mem::size_of::<[i8; 8]>(),
        ) == 0i32
    {
        *OID = gss_ntlm_mechanism_oid_desc
    } else if (*token).length != 0usize
        && *((*token).value as *mut i8).offset(0isize) as i32 == 0x6ei32
    {
        /* Could be a raw AP-REQ (check for APPLICATION tag) */
        *OID = gss_krb5_mechanism_oid_desc
    } else if (*token).length == 0usize {
        *OID = gss_spnego_mechanism_oid_desc
    } else {
        return gssint_get_mech_type_oid(OID, token);
    }
    return 0u32;
}

unsafe extern "C" fn import_internal_attributes(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut dmech: crate::mglueP_h::gss_mechanism,
    mut sname: crate::mglueP_h::gss_union_name_t,
    mut dname: crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    let mut smech = 0 as *mut crate::mglueP_h::gss_config;
    let mut attrs = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    let mut i: crate::stddef_h::size_t = 0;
    if (*sname).mech_name.is_null() {
        return (16u32) << 16i32;
    }
    smech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        (*sname).mech_type as crate::gssapi_h::gss_const_OID,
    );
    if smech.is_null() {
        return (1u32) << 16i32;
    }
    if (*smech).gss_inquire_name.is_none() || (*smech).gss_get_name_attribute.is_none() {
        return (16u32) << 16i32;
    }
    if (*dmech).gss_set_name_attribute.is_none() {
        return (16u32) << 16i32;
    }
    major = (*smech)
        .gss_inquire_name
        .expect("non-null function pointer")(
        minor,
        (*sname).mech_name,
        0 as *mut i32,
        0 as *mut crate::gssapi_h::gss_OID,
        &mut attrs,
    );
    if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 || attrs.is_null() {
        crate::src::mechglue::g_buffer_set::gss_release_buffer_set(&mut tmpMinor, &mut attrs);
        return major;
    }
    i = 0usize;
    while i < (*attrs).count {
        let mut more = -(1i32);
        while more != 0i32 {
            let mut value = crate::gssapi_h::gss_buffer_desc {
                length: 0,
                value: 0 as *mut libc::c_void,
            };
            let mut display_value = crate::gssapi_h::gss_buffer_desc {
                length: 0,
                value: 0 as *mut libc::c_void,
            };
            let mut authenticated: i32 = 0;
            let mut complete: i32 = 0;
            major = (*smech)
                .gss_get_name_attribute
                .expect("non-null function pointer")(
                minor,
                (*sname).mech_name,
                &mut *(*attrs).elements.offset(i as isize),
                &mut authenticated,
                &mut complete,
                &mut value,
                &mut display_value,
                &mut more,
            );
            if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                continue;
            }
            if authenticated != 0 {
                (*dmech)
                    .gss_set_name_attribute
                    .expect("non-null function pointer")(
                    minor,
                    dname,
                    complete,
                    &mut *(*attrs).elements.offset(i as isize),
                    &mut value,
                );
            }
            crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpMinor, &mut value);
            crate::src::mechglue::g_rel_buffer::gss_release_buffer(
                &mut tmpMinor,
                &mut display_value,
            );
        }
        i = i.wrapping_add(1)
    }
    crate::src::mechglue::g_buffer_set::gss_release_buffer_set(&mut tmpMinor, &mut attrs);
    return 0u32;
}
/*
 *  Internal routines to get and release an internal mechanism name
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_import_internal_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut union_name: crate::mglueP_h::gss_union_name_t,
    mut internal_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut public_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        mech_type as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    /*
     * If we are importing a name for the same mechanism, and the
     * mechanism implements gss_duplicate_name, then use that.
     */
    if !(*union_name).mech_type.is_null()
        && !(*union_name).mech_name.is_null()
        && ((*(*union_name).mech_type).length == (*mech_type).length
            && crate::stdlib::memcmp(
                (*(*union_name).mech_type).elements,
                (*mech_type).elements,
                (*(*union_name).mech_type).length as usize,
            ) == 0i32)
        && (*mech).gss_duplicate_name.is_some()
    {
        status = (*mech)
            .gss_duplicate_name
            .expect("non-null function pointer")(
            minor_status,
            (*union_name).mech_name,
            internal_name,
        );
        if status != (16u32) << 16i32 {
            if status != 0u32 {
                *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                    *minor_status,
                    &mut (*mech).mech_type,
                )
            }
            return status;
        }
    }
    if (*mech).gssspi_import_name_by_mech.is_some() {
        public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
            mech_type as crate::gssapi_h::gss_const_OID,
        );
        status = (*mech)
            .gssspi_import_name_by_mech
            .expect("non-null function pointer")(
            minor_status,
            public_mech,
            (*union_name).external_name,
            (*union_name).name_type,
            internal_name,
        )
    } else if (*mech).gss_import_name.is_some() {
        status = (*mech).gss_import_name.expect("non-null function pointer")(
            minor_status,
            (*union_name).external_name,
            (*union_name).name_type,
            internal_name,
        )
    } else {
        return (16u32) << 16i32;
    }
    if status == 0u32 {
        /* Attempt to round-trip attributes */
        import_internal_attributes(&mut tmpMinor, mech, union_name, *internal_name);
    } else {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        )
    }
    return status;
}
#[no_mangle]

pub unsafe extern "C" fn gssint_export_internal_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mech_type: crate::gssapi_h::gss_OID,
    internal_name: crate::gssapi_h::gss_name_t,
    mut name_buf: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut dispName = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut nameOid = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut buf = 0 as *mut u8;
    let tokId: [u8; 3] = *::std::mem::transmute::<&[u8; 3], &[u8; 3]>(b"\x04\x01\x00");
    let tokIdLen = 2u32;
    let mechOidLenLen = 2i32;
    let mechOidTagLen = 1i32;
    let nameLenLen = 4i32;
    let mut mechOidDERLen = 0i32;
    let mut mechOidLen = 0i32;
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        mech_type as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gss_export_name.is_some() {
        status = (*mech).gss_export_name.expect("non-null function pointer")(
            minor_status,
            internal_name,
            name_buf,
        );
        if status != 0u32 {
            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor_status,
                &mut (*mech).mech_type,
            )
        }
        return status;
    }
    /*
     * if we are here it is because the mechanism does not provide
     * a gss_export_name so we will use our implementation.  We
     * do required that the mechanism define a gss_display_name.
     */
    if (*mech).gss_display_name.is_none() {
        return (16u32) << 16i32;
    }
    /*
     * NOTE: RFC2743 (section 3.2) governs the format of the outer
     *	 wrapper of exported names; the mechanisms' specs govern
     *	 the format of the inner portion of the exported name
     *	 and, for some (e.g., RFC1964, the Kerberos V mech), a
     *	 generic default as implemented here will do.
     *
     * The outer wrapper of an exported MN is: 2-octet tok Id
     * (0x0401) + 2-octet network-byte order mech OID length + mech
     * oid (in DER format, including DER tag and DER length) +
     * 4-octet network-byte order length of inner portion + inner
     * portion.
     *
     * For the Kerberos V mechanism the inner portion of an exported
     * MN is the display name string and ignores the name type OID
     * altogether.  And we hope this will be so for any future
     * mechanisms also, so that factoring name export/import out of
     * the mech and into libgss pays off.
     */
    status = (*mech).gss_display_name.expect("non-null function pointer")(
        minor_status,
        internal_name,
        &mut dispName,
        &mut nameOid,
    );
    if status != 0u32 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        );
        return status;
    }
    /* determine the size of the buffer needed */
    mechOidDERLen = gssint_der_length_size((*mech_type).length) as i32;
    (*name_buf).length = (tokIdLen
        .wrapping_add(mechOidLenLen as u32)
        .wrapping_add(mechOidTagLen as u32)
        .wrapping_add(mechOidDERLen as u32)
        .wrapping_add((*mech_type).length)
        .wrapping_add(nameLenLen as u32) as usize)
        .wrapping_add(dispName.length);
    (*name_buf).value = gssalloc_malloc((*name_buf).length);
    if (*name_buf).value == 0 as *mut libc::c_void {
        (*name_buf).length = 0usize;
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut status, &mut dispName);
        return (13u32) << 16i32;
    }
    /* now create the name ..... */
    buf = (*name_buf).value as *mut u8;
    crate::stdlib::memset((*name_buf).value, 0i32, (*name_buf).length);
    crate::stdlib::memcpy(
        buf as *mut libc::c_void,
        tokId.as_ptr() as *const libc::c_void,
        tokIdLen as usize,
    );
    buf = buf.offset(tokIdLen as isize);
    /* spec allows only 2 bytes for the mech oid length */
    mechOidLen = ((mechOidDERLen + mechOidTagLen) as u32).wrapping_add((*mech_type).length) as i32;
    store_16_be(mechOidLen as u32, buf as *mut libc::c_void);
    buf = buf.offset(2isize);
    /*
     * DER Encoding of mech OID contains OID Tag (0x06), length and
     * mech OID value
     */
    let fresh8 = buf;
    buf = buf.offset(1);
    *fresh8 = 0x6u8;
    if gssint_put_der_length(
        (*mech_type).length,
        &mut buf,
        (*name_buf)
            .length
            .wrapping_sub(tokIdLen as usize)
            .wrapping_sub(2usize) as u32,
    ) != 0i32
    {
        (*name_buf).length = 0usize;
        crate::stdlib::free((*name_buf).value);
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut status, &mut dispName);
        return (13u32) << 16i32;
    }
    crate::stdlib::memcpy(
        buf as *mut libc::c_void,
        (*mech_type).elements,
        (*mech_type).length as usize,
    );
    buf = buf.offset((*mech_type).length as isize);
    /* spec designates the next 4 bytes for the name length */
    store_32_be(dispName.length as u32, buf as *mut libc::c_void);
    buf = buf.offset(4isize);
    /* for the final ingredient - add the name from gss_display_name */
    crate::stdlib::memcpy(buf as *mut libc::c_void, dispName.value, dispName.length);
    /* release the buffer obtained from gss_display_name */
    crate::src::mechglue::g_rel_buffer::gss_release_buffer(minor_status, &mut dispName);
    return 0u32;
}
/*  gssint_export_internal_name */
#[no_mangle]

pub unsafe extern "C" fn gssint_display_internal_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut internal_name: crate::gssapi_h::gss_name_t,
    mut external_name: crate::gssapi_h::gss_buffer_t,
    mut name_type: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        mech_type as crate::gssapi_h::gss_const_OID,
    );
    if !mech.is_null() {
        if (*mech).gss_display_name.is_some() {
            status = (*mech).gss_display_name.expect("non-null function pointer")(
                minor_status,
                internal_name,
                external_name,
                name_type,
            );
            if status != 0u32 {
                *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                    *minor_status,
                    &mut (*mech).mech_type,
                )
            }
        } else {
            status = (16u32) << 16i32
        }
        return status;
    }
    return (1u32) << 16i32;
}
#[no_mangle]

pub unsafe extern "C" fn gssint_release_internal_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut internal_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        mech_type as crate::gssapi_h::gss_const_OID,
    );
    if !mech.is_null() {
        if (*mech).gss_release_name.is_some() {
            status = (*mech).gss_release_name.expect("non-null function pointer")(
                minor_status,
                internal_name,
            );
            if status != 0u32 {
                *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                    *minor_status,
                    &mut (*mech).mech_type,
                )
            }
        } else {
            status = (16u32) << 16i32
        }
        return status;
    }
    return (1u32) << 16i32;
}
#[no_mangle]

pub unsafe extern "C" fn gssint_delete_internal_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut internal_ctx: *mut crate::gssapi_h::gss_ctx_id_t,
    mut output_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        mech_type as crate::gssapi_h::gss_const_OID,
    );
    if !mech.is_null() {
        if (*mech).gss_delete_sec_context.is_some() {
            status = (*mech)
                .gss_delete_sec_context
                .expect("non-null function pointer")(
                minor_status, internal_ctx, output_token
            )
        } else {
            status = (16u32) << 16i32
        }
        return status;
    }
    return (1u32) << 16i32;
}
/*
 * This function converts an internal gssapi name to a union gssapi
 * name.  Note that internal_name should be considered "consumed" by
 * this call, whether or not we return an error.
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_convert_name_to_union_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech: crate::mglueP_h::gss_mechanism,
    mut internal_name: crate::gssapi_h::gss_name_t,
    mut external_name: *mut crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmp: crate::gssapi_h::OM_uint32 = 0;
    let mut union_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    union_name = crate::stdlib::malloc(::std::mem::size_of::<crate::mglueP_h::gss_union_name_desc>())
        as crate::mglueP_h::gss_union_name_t;
    if union_name.is_null() {
        major_status = (13u32) << 16i32;
        *minor_status = 12u32;
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
    } else {
        (*union_name).mech_type = 0 as crate::gssapi_h::gss_OID;
        (*union_name).mech_name = internal_name;
        (*union_name).name_type = 0 as crate::gssapi_h::gss_OID;
        (*union_name).external_name = 0 as crate::gssapi_h::gss_buffer_t;
        major_status = crate::src::generic::oid_ops::generic_gss_copy_oid(
            minor_status,
            &mut (*mech).mech_type,
            &mut (*union_name).mech_type,
        );
        if major_status != 0u32 {
            *minor_status =
                crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
        } else {
            (*union_name).external_name = crate::stdlib::malloc(::std::mem::size_of::<
                crate::gssapi_h::gss_buffer_desc,
            >()) as crate::gssapi_h::gss_buffer_t;
            if (*union_name).external_name.is_null() {
                major_status = (13u32) << 16i32
            } else {
                (*(*union_name).external_name).length = 0usize;
                (*(*union_name).external_name).value = 0 as *mut libc::c_void;
                major_status = (*mech).gss_display_name.expect("non-null function pointer")(
                    minor_status,
                    internal_name,
                    (*union_name).external_name,
                    &mut (*union_name).name_type,
                );
                if major_status != 0u32 {
                    *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                        *minor_status,
                        &mut (*mech).mech_type,
                    )
                } else {
                    (*union_name).loopback = union_name;
                    *external_name = union_name;
                    return 0u32;
                }
            }
        }
    }
    if !union_name.is_null() {
        if !(*union_name).external_name.is_null() {
            if !(*(*union_name).external_name).value.is_null() {
                crate::stdlib::free((*(*union_name).external_name).value);
            }
            crate::stdlib::free((*union_name).external_name as *mut libc::c_void);
        }
        if !(*union_name).name_type.is_null() {
            crate::src::mechglue::g_initialize::gss_release_oid(
                &mut tmp,
                &mut (*union_name).name_type,
            );
        }
        if !(*union_name).mech_type.is_null() {
            crate::src::mechglue::g_initialize::gss_release_oid(
                &mut tmp,
                &mut (*union_name).mech_type,
            );
        }
        crate::stdlib::free(union_name as *mut libc::c_void);
    }
    /*
     * do as the top comment says - since we are now owners of
     * internal_name, we must clean it up
     */
    if !internal_name.is_null() {
        gssint_release_internal_name(&mut tmp, &mut (*mech).mech_type, &mut internal_name);
    }
    return major_status;
}
/*
 * Glue routine for returning the mechanism-specific credential from a
 * external union credential.
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_get_mechanism_cred(
    mut union_cred: crate::mglueP_h::gss_union_cred_t,
    mut mech_type: crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::gss_cred_id_t {
    let mut i: i32 = 0;
    if union_cred.is_null() {
        return 0 as crate::gssapi_h::gss_cred_id_t;
    }
    i = 0i32;
    while i < (*union_cred).count {
        if (*mech_type).length == (*(*union_cred).mechs_array.offset(i as isize)).length
            && crate::stdlib::memcmp(
                (*mech_type).elements,
                (*(*union_cred).mechs_array.offset(i as isize)).elements,
                (*mech_type).length as usize,
            ) == 0i32
        {
            return *(*union_cred).cred_array.offset(i as isize);
        }
        i += 1
    }
    return 0 as crate::gssapi_h::gss_cred_id_t;
}
/*
 * Routine to create and copy the gss_buffer_desc structure.
 * Both space for the structure and the data is allocated.
 */
#[no_mangle]

pub unsafe extern "C" fn gssint_create_copy_buffer(
    srcBuf: crate::gssapi_h::gss_buffer_t,
    mut destBuf: *mut crate::gssapi_h::gss_buffer_t,
    mut addNullChar: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut aBuf = 0 as *mut crate::gssapi_h::gss_buffer_desc_struct;
    let mut len: u32 = 0;
    if destBuf.is_null() {
        return (2u32) << 24i32;
    }
    *destBuf = 0 as crate::gssapi_h::gss_buffer_t;
    aBuf = crate::stdlib::malloc(::std::mem::size_of::<crate::gssapi_h::gss_buffer_desc>())
        as crate::gssapi_h::gss_buffer_t;
    if aBuf.is_null() {
        return (13u32) << 16i32;
    }
    if addNullChar != 0 {
        len = (*srcBuf).length.wrapping_add(1usize) as u32
    } else {
        len = (*srcBuf).length as u32
    }
    (*aBuf).value = gssalloc_malloc(len as crate::stddef_h::size_t);
    if (*aBuf).value.is_null() {
        crate::stdlib::free(aBuf as *mut libc::c_void);
        return (13u32) << 16i32;
    }
    crate::stdlib::memcpy((*aBuf).value, (*srcBuf).value, (*srcBuf).length);
    (*aBuf).length = (*srcBuf).length;
    *destBuf = aBuf;
    /* optionally add a NULL character */
    if addNullChar != 0 {
        *((*aBuf).value as *mut i8).offset((*aBuf).length as isize) = '\u{0}' as i8
    }
    return 0u32;
}
/*
 * Rudimentary pointer validation macro to check whether the
 * "loopback" field of an opaque struct points back to itself.  This
 * field also catches some programming errors where an opaque pointer
 * is passed to a function expecting the address of the opaque
 * pointer.
 */
/* *******************************************************/
/* The Mechanism Dispatch Table -- a mechanism needs to */
/* define one of these and provide a function to return */
/* it to initialize the GSSAPI library		  */
/*
 * This is the definition of the mechs_array struct, which is used to
 * define the mechs array table. This table is used to indirectly
 * access mechanism specific versions of the gssapi routines through
 * the routines in the glue module (gssd_mech_glue.c)
 *
 * This contants all of the functions defined in gssapi.h except for
 * gss_release_buffer() and gss_release_oid_set(), which I am
 * assuming, for now, to be equal across mechanisms.
 */
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
/* token_buffer */
/* minor_status */
/* context_handle */
/* output_token */
/* minor_status */
/* context_handle */
/* time_rec */
/* minor_status */
/* context_handle */
/* qop_req */
/* message_buffer */
/* message_token */
/* minor_status */
/* context_handle */
/* message_buffer */
/* token_buffer */
/* qop_state */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_message_buffer */
/* conf_state */
/* output_message_buffer */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* output_message_buffer */
/* conf_state */
/* qop_state */
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
/* input_cred_handle */
/* desired_name */
/* desired_mech */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
/* minor_status */
/* context_handle */
/* interprocess_token */
/* minor_status */
/* interprocess_token */
/* context_handle */
/* minor_status */
/* cred_handle */
/* mech_type */
/* name */
/* initiator_lifetime */
/* acceptor_lifetime */
/* cred_usage */
/* minor_status */
/* mechanism */
/* name_types */
/* minor_status */
/* context_handle */
/* src_name */
/* targ_name */
/* lifetime_rec */
/* mech_type */
/* ctx_flags */
/* locally_initiated */
/* open */
/* minor_status */
/* OID */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* req_output_size */
/* max_input_size */
/* minor */
/* name */
/* mech_type */
/* localname */
/* minor_status */
/* pname */
/* local user */
/* local nametype */
/* */
/* minor_status */
/* input_name */
/* exported_name */
/* */
/* minor_status */
/* input_name */
/* output_name */
/* */
/* minor_status */
/* input_cred */
/* cred_usage */
/* desired_mech */
/* overwrite_cred */
/* default_cred */
/* elements_stored */
/* cred_usage_stored */
/* */
/* GGF extensions */
/* minor_status */
/* context_handle */
/* OID */
/* data_set */
/* minor_status */
/* cred_handle */
/* OID */
/* data_set */
/* minor_status */
/* context_handle */
/* OID */
/* value */
/* minor_status */
/* cred_handle */
/* OID */
/* value */
/* minor_status */
/* mech OID */
/* OID */
/* value */
/* AEAD extensions */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_assoc_buffer */
/* input_payload_buffer */
/* conf_state */
/* output_message_buffer */
/* */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* input_assoc_buffer */
/* output_payload_buffer */
/* conf_state */
/* qop_state */
/* */
/* SSPI extensions */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* */
/* minor_status */
/* context_handle */
/* conf_state */
/* qop_state */
/* iov */
/* iov_count */
/* */
/* minor_status */
/* context_handle */
/* conf_req_flag*/
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* New for 1.8 */
/* minor_status */
/* impersonator_cred_handle */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* */
/* minor_status */
/* input_cred_handle */
/* impersonator_cred_handle */
/* desired_name */
/* desired_mech */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
/* */
/* minor_status */
/* name */
/* display_as_name_type */
/* display_name */
/* */
/* minor_status */
/* name */
/* name_is_MN */
/* MN_mech */
/* attrs */
/* */
/* minor_status */
/* name */
/* attr */
/* authenticated */
/* complete */
/* value */
/* display_value */
/* more */
/* */
/* minor_status */
/* name */
/* complete */
/* attr */
/* value */
/* */
/* minor_status */
/* name */
/* attr */
/* */
/* minor_status */
/* name */
/* exp_composite_name */
/* */
/* minor_status */
/* name */
/* authenticated */
/* type_id */
/* output */
/* */
/* minor_status */
/* name */
/* type_id */
/* input */
/* */
/* minor_status */
/* context */
/* prf_key */
/* prf_in */
/* desired_output_len */
/* prf_out */
/* */
/* minor_status */
/* cred_handle */
/* mech_set */
/* */
/* minor_status */
/* desired_mech */
/* sasl_mech_name */
/* mech_name */
/* mech_description */
/* */
/* minor_status */
/* sasl_mech_name */
/* mech_type */
/* */
/* minor_status */
/* mech */
/* mech_attrs */
/* known_mech_attrs */
/* */
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
/* */
/* minor_status */
/* input_cred_handle */
/* input_usage */
/* desired_mech */
/* overwrite_cred */
/* default_cred */
/* cred_store */
/* elements_stored */
/* cred_usage_stored */
/* */
/* minor_status */
/* desired_name */
/* password */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* */
/* minor_status */
/* cred_handle */
/* token */
/* */
/* minor_status */
/* token */
/* cred_handle */
/* */
/* minor_status */
/* desired_mech */
/* interprocess_token */
/* context_handle */
/* */
/* minor_status */
/* mech_type */
/* input_name_buffer */
/* input_name_type */
/* output_name */
/* */
/* minor_status */
/* mech_type */
/* token */
/* cred_handle */
/* */
/* get_mic_iov extensions, added in 1.12 */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* NegoEx extensions added in 1.18 */
/* minor_status */
/* mech_oid */
/* cred_handle */
/* context_handle */
/* targ_name */
/* req_flags */
/* meta_data */
/* */
/* minor_status */
/* mech_oid */
/* cred_handle */
/* context_handle */
/* targ_name */
/* req_flags */
/* meta_data */
/* */
/* minor_status */
/* mech_oid */
/* auth_scheme */
/* */
/*
 * In the user space we use a wrapper structure to encompass the
 * mechanism entry points.  The wrapper contain the mechanism
 * entry points and other data which is only relevant to the gss-api
 * layer.  In the kernel we use only the gss_config strucutre because
 * the kernal does not cantain any of the extra gss-api specific data.
 */
/* kernel module name */
/* user library name */
/* mechanism string name */
/* optional mech parameters */
/* RTLD object handle for the mech */
/* mechanism oid */
/* mechanism initialization struct */
/* mechanism preference order */
/* free mech table */
/* interposer mechanism flag */
/* points to the interposer OID */
/* points to the interposer mech */
/* next element in the list */
/* *******************************************************/
/* Internal mechglue routines */
/* minor_status */
/* mech */
/* internal_name */
/* external_name */
/* union_cred */
/* mech_type */
/* src buffer */
/* destination buffer */
/* NULL terminate buffer ? */
/* ****** gssint_create_copy_buffer  ****** */
#[no_mangle]

pub unsafe extern "C" fn gssint_create_union_context(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut mech_oid: crate::gssapi_h::gss_const_OID,
    mut ctx_out: *mut crate::mglueP_h::gss_union_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = 0 as *mut crate::mglueP_h::gss_union_ctx_id_struct;
    *ctx_out = 0 as crate::mglueP_h::gss_union_ctx_id_t;
    ctx = crate::stdlib::calloc(
        1usize,
        ::std::mem::size_of::<crate::mglueP_h::gss_union_ctx_id_struct>(),
    ) as crate::mglueP_h::gss_union_ctx_id_t;
    if ctx.is_null() {
        *minor = 12u32;
        return (13u32) << 16i32;
    }
    status =
        crate::src::generic::oid_ops::generic_gss_copy_oid(minor, mech_oid, &mut (*ctx).mech_type);
    if status != 0u32 {
        crate::stdlib::free(ctx as *mut libc::c_void);
        return status;
    }
    (*ctx).loopback = ctx;
    (*ctx).internal_ctx_id = 0 as crate::gssapi_h::gss_ctx_id_t;
    *ctx_out = ctx;
    return 0u32;
}
