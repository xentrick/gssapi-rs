use ::libc;

pub mod k5_thread_h {

    #[inline]

    pub unsafe extern "C" fn k5_mutex_init(mut m: *mut crate::k5_thread_h::k5_mutex_t) -> i32 {
        return crate::k5_thread_h::k5_os_mutex_init(m);
    }
    #[inline]

    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut crate::k5_thread_h::k5_mutex_t) {
        let mut r = crate::k5_thread_h::k5_os_mutex_lock(m);
        if r != 0i32 {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"k5_mutex_lock: Received error %d (%s)\n\x00" as *const u8 as *const i8,
                r,
                crate::stdlib::strerror(r),
            );
        }
        if r == 0i32 {
        } else {
            crate::stdlib::__assert_fail(
                b"r == 0\x00" as *const u8 as *const i8,
                b"../../../include/k5-thread.h\x00" as *const u8 as *const i8,
                376u32,
                (*::std::mem::transmute::<&[u8; 33], &[i8; 33]>(
                    b"void k5_mutex_lock(k5_mutex_t *)\x00",
                ))
                .as_ptr(),
            );
        };
    }
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

    use crate::src::krb5::naming_exts::byteswap_h::__bswap_16;
    use crate::src::krb5::naming_exts::byteswap_h::__bswap_32;

    /* K5_PLATFORM_H */
}

pub mod k5_int_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /*
     * Copyright (C) 1989,1990,1991,1992,1993,1994,1995,2000,2001,
     * 2003,2006,2007,2008,2009 by the Massachusetts Institute of Technology,
     * Cambridge, MA, USA.  All Rights Reserved.
     *
     * This software is being provided to you, the LICENSEE, by the
     * Massachusetts Institute of Technology (M.I.T.) under the following
     * license.  By obtaining, using and/or copying this software, you agree
     * that you have read, understood, and will comply with these terms and
     * conditions:
     *
     * Export of this software from the United States of America may
     * require a specific license from the United States Government.
     * It is the responsibility of any person or organization contemplating
     * export to obtain such a license before exporting.
     *
     * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
     * this software and its documentation for any purpose and without fee or
     * royalty is hereby granted, provided that you agree to comply with the
     * following copyright notice and statements, including the disclaimer, and
     * that the same appear on ALL copies of the software and documentation,
     * including modifications that you make for internal use or for
     * distribution:
     *
     * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
     * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
     * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
     * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
     * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
     * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
     *
     * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
     * be used in advertising or publicity pertaining to distribution of the
     * software.  Title to copyright in this software and any associated
     * documentation shall at all times remain with M.I.T., and USER agrees to
     * preserve same.
     *
     * Furthermore if you modify this software you must label
     * your software as modified software and not distribute it in such a
     * fashion that it might be confused with the original M.I.T. software.
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
    /*
     * This prototype for k5-int.h (Krb5 internals include file)
     * includes the user-visible definitions from krb5.h and then
     * includes other definitions that are not user-visible but are
     * required for compiling Kerberos internal routines.
     *
     * John Gilmore, Cygnus Support, Sat Jan 21 22:45:52 PST 1995
     */
    /* KRB5_GENERAL__ */
    /*
     * Begin "k5-config.h"
     */
    /*
     * Machine-type definitions: PC Clone 386 running Microloss Windows
     */
    /* From autoconf.h */
    /* HAVE_SYS_TYPES_H */
    /* HAVE_SYS_TYPES_H */
    /* KRB5_SYSTYPES__ */
    /* one day */
    /* one week */
    /* Thu Jan  1 00:00:00 2038 UTC */
    /*
     * Windows requires a different api interface to each function. Here
     * just define it as NULL.
     */
    /* #define KRB5_OLD_CRYPTO is done in krb5.h */
    /* KRB5_CONFIG__ */
    /*
     * End "k5-config.h"
     */
    /*
     * After loading the configuration definitions, load the Kerberos definitions.
     */
    /* Get mutex support; currently used only for the replay cache.  */
    /* Get error info support.  */
    /* Get string buffer support. */
    /* Define tracing macros. */
    /* Profile variables.  Constants are named KRB5_CONF_STRING, where STRING
     * matches the variable name.  Keep these alphabetized. */
    /* Cache configuration variables */
    /* Error codes used in KRB_ERROR protocol messages.
    Return values of library routines are based on a different error table
    (which allows non-ambiguous error codes between subsystems) */
    /* KDC errors */
    /* No error */
    /* Client's entry in DB expired */
    /* Server's entry in DB expired */
    /* Requested pvno not supported */
    /* C's key encrypted in old master */
    /* S's key encrypted in old master */
    /* Client not found in Kerberos DB */
    /* Server not found in Kerberos DB */
    /* Multiple entries in Kerberos DB */
    /* The C or S has a null key */
    /* Tkt ineligible for postdating */
    /* Requested starttime > endtime */
    /* KDC policy rejects request */
    /* KDC can't do requested opt. */
    /* No support for encryption type */
    /* No support for checksum type */
    /* No support for padata type */
    /* No support for transited type */
    /* C's creds have been revoked */
    /* S's creds have been revoked */
    /* TGT has been revoked */
    /* C not yet valid */
    /* S not yet valid */
    /* Password has expired */
    /* Preauthentication failed */
    /* Additional preauthentication */
    /* required */
    /* Requested server and */
    /* ticket don't match*/
    /* Server principal valid for */
    /*   user2user only */
    /* KDC policy rejected transited */
    /*   path */
    /* A service is not
     * available that is
     * required to process the
     * request */
    /* Application errors */
    /* Decrypt integrity check failed */
    /* Ticket expired */
    /* Ticket not yet valid */
    /* Request is a replay */
    /* The ticket isn't for us */
    /* Ticket/authenticator don't match */
    /* Clock skew too great */
    /* Incorrect net address */
    /* Protocol version mismatch */
    /* Invalid message type */
    /* Message stream modified */
    /* Message out of order */
    /* Key version is not available */
    /* Service key not available */
    /* Mutual authentication failed */
    /* Incorrect message direction */
    /* Alternative authentication */
    /* method required */
    /* Incorrect sequence numnber */
    /* in message */
    /* Inappropriate type of */
    /* checksum in message */
    /* Policy rejects transited path */
    /* Response too big for UDP, */
    /*   retry with TCP */
    /* other errors */
    /* Generic error (description */
    /* in e-text) */
    /* Field is too long for impl. */
    /* PKINIT server-reported errors */
    /* client cert not trusted */
    /* client signature verify failed */
    /* invalid Diffie-Hellman parameters */
    /* client cert not verifiable to */
    /* trusted root cert */
    /* client cert had invalid signature */
    /* client cert was revoked */
    /* client cert revoked, reason unknown */
    /* mismatch between client cert and */
    /* principal name */
    /* bad extended key use */
    /* bad digest algorithm in client cert */
    /* missing paChecksum in PA-PK-AS-REQ */
    /* bad digest algorithm in SignedData */
    /* The IAKERB proxy could
    not find a KDC */
    /* The KDC did not respond
    to the IAKERB proxy */
    /* RFC 6113 */
    /* RFC 6113 */
    /* err table base max offset for protocol err codes */
    /*
     * A null-terminated array of this structure is returned by the KDC as
     * the data part of the ETYPE_INFO preauth type.  It informs the
     * client which encryption types are supported.
     * The  same data structure is used by both etype-info and etype-info2
     * but s2kparams must be null when encoding etype-info.
     */
    /*
     *  This is essentially -1 without sign extension which can screw up
     *  comparisons on 64 bit machines. If the length is this value, then
     *  the salt data is not present. This is to distinguish between not
     *  being set and being of 0 length.
     */
    /* RFC 4537 */
    /* sam_type values -- informational only */
    /*  Enigma Logic */
    /*  Digital Pathways */
    /*  S/key where  KDC has key 0 */
    /*  Traditional S/Key */
    /*  Security Dynamics */
    /*  CRYPTOCard */
    /* XXX need to figure out who has which numbers assigned */
    /*  ActivCard decimal mode */
    /*  ActivCard hex mode */
    /*  Digital Pathways hex mode */
    /* experimental */
    /* testing */
    /* special */
    /* Array of checksums */
    /* information */
    /* KRB5_SAM_* values */
    /* informational */
    /* KRB5_SAM_* values */
    /* copied */
    /* krb5_enc_sam_response_enc */
    /*
     * Keep the pkinit definitions in a separate file so that the plugin
     * only has to include k5-int-pkinit.h rather than k5-int.h
     */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* Plain text of an encrypted PA-FX-COOKIE value produced by the KDC. */
    /* In PAC options, indicates Resource-Based Constrained Delegation support. */
    /* struct stat, stat() */
    /* MAXPATHLEN */
    /* prototypes for file-related
    syscalls; flags for open &
    friends */
    /* libos.spec */
    /* Internal structure of an opaque key identifier */
    /*
     * Cache of data private to the cipher implementation, which we
     * don't want to have to recompute for every operation.  This may
     * include key schedules, iteration counts, etc.
     *
     * The cipher implementation is responsible for setting this up
     * whenever needed, and the enc_provider key_cleanup method must
     * then be provided to dispose of it.
     */

    #[inline]

    pub unsafe extern "C" fn k5memdup0(
        mut in_0: *const libc::c_void,
        mut len: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        let mut ptr = k5alloc(len.wrapping_add(1usize), code);
        if !ptr.is_null() && len > 0usize {
            crate::stdlib::memcpy(ptr, in_0, len);
        }
        return ptr;
    }
    #[inline]

    pub unsafe extern "C" fn k5alloc(
        mut size: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        return k5calloc(1usize, size, code);
    }
    #[inline]

    pub unsafe extern "C" fn k5calloc(
        mut nmemb: crate::stddef_h::size_t,
        mut size: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        let mut ptr = 0 as *mut libc::c_void;
        ptr = crate::stdlib::calloc(
            if nmemb != 0 { nmemb } else { 1usize },
            if size != 0 { size } else { 1usize },
        );
        *code = if ptr.is_null() { 12i32 } else { 0i32 };
        return ptr;
    }
    #[inline]

    pub unsafe extern "C" fn empty_data() -> crate::krb5_h::krb5_data {
        return make_data(0 as *mut libc::c_void, 0u32);
    }
    #[inline]

    pub unsafe extern "C" fn make_data(
        mut data: *mut libc::c_void,
        mut len: u32,
    ) -> crate::krb5_h::krb5_data {
        let mut d = crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        };
        d.magic = -(1760647422 as isize) as crate::krb5_h::krb5_magic;
        d.data = data as *mut i8;
        d.length = len;
        return d;
    }

    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}

pub mod gssapiP_krb5_h {

    #[inline]

    pub unsafe extern "C" fn data_to_gss(
        mut input_k5data: *mut crate::krb5_h::krb5_data,
        mut output_buffer: crate::gssapi_h::gss_buffer_t,
    ) -> crate::krb5_h::krb5_error_code {
        let mut code = 0i32;
        (*output_buffer).length = (*input_k5data).length as crate::stddef_h::size_t;
        (*output_buffer).value = (*input_k5data).data as *mut libc::c_void;
        *input_k5data = empty_data();
        return code;
    }

    use crate::src::krb5::naming_exts::k5_int_h::empty_data;

    /* _GSSAPIP_KRB5_H_ */
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
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
    #[inline]

    pub unsafe extern "C" fn gssalloc_calloc(
        mut count: crate::stddef_h::size_t,
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::calloc(count, size);
    }
}

/* _GSSAPI_KRB5_H_ */

/* __cplusplus */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::pthread_mutex_t;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::uint16_t;
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
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_authdata_context_copy;
pub use crate::k5_int_h::krb5_authdata_context_free;
pub use crate::k5_int_h::krb5_authdata_context_init;
pub use crate::k5_int_h::krb5_authdata_delete_attribute;
pub use crate::k5_int_h::krb5_authdata_export_attributes;
pub use crate::k5_int_h::krb5_authdata_export_internal;
pub use crate::k5_int_h::krb5_authdata_free_internal;
pub use crate::k5_int_h::krb5_authdata_get_attribute;
pub use crate::k5_int_h::krb5_authdata_get_attribute_types;
pub use crate::k5_int_h::krb5_authdata_set_attribute;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
pub use crate::k5_int_h::krb5int_free_data_list;
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
pub use crate::k5_thread_h::k5_os_mutex_destroy;
pub use crate::k5_thread_h::k5_os_mutex_init;
pub use crate::k5_thread_h::k5_os_mutex_lock;
pub use crate::k5_thread_h::k5_os_mutex_unlock;
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
pub use crate::krb5_h::krb5_build_principal;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_copy_principal;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_free_data;
pub use crate::krb5_h::krb5_free_principal;
pub use crate::krb5_h::krb5_free_unparsed_name;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_compare;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::krb5_h::krb5_unparse_name;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::naming_exts::k5_int_h::empty_data;
pub use crate::src::krb5::naming_exts::k5_int_h::k5alloc;
pub use crate::src::krb5::naming_exts::k5_int_h::k5calloc;
pub use crate::src::krb5::naming_exts::k5_int_h::k5memdup0;
pub use crate::src::krb5::naming_exts::k5_int_h::make_data;
pub use crate::src::krb5::naming_exts::k5_platform_h::store_16_be;
pub use crate::src::krb5::naming_exts::k5_platform_h::store_32_be;
pub use crate::src::krb5::naming_exts::k5_thread_h::k5_mutex_init;
pub use crate::src::krb5::naming_exts::k5_thread_h::k5_mutex_lock;
pub use crate::src::krb5::naming_exts::k5_thread_h::k5_mutex_unlock;

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
pub use crate::gssapiP_krb5_h::_krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapi_ext_h::gss_any;
pub use crate::gssapi_ext_h::gss_any_t;
pub use crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub use crate::gssapi_ext_h::gss_buffer_set_t;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::krb5::init_sec_context::krb5_gss_init_context;
pub use crate::src::krb5::naming_exts::gssapiP_krb5_h::data_to_gss;
pub use crate::src::mechglue::g_buffer_set::gss_create_empty_buffer_set;
pub use crate::src::mechglue::g_buffer_set::gss_release_buffer_set;

pub use crate::src::krb5::naming_exts::byteswap_h::__bswap_16;
pub use crate::src::krb5::naming_exts::byteswap_h::__bswap_32;
pub use crate::src::krb5::naming_exts::gssapi_alloc_h::gssalloc_calloc;
pub use crate::src::krb5::naming_exts::gssapi_alloc_h::gssalloc_malloc;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/naming_exts.c */
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
#[no_mangle]

pub unsafe extern "C" fn kg_init_name(
    mut context: crate::krb5_h::krb5_context,
    mut principal: crate::krb5_h::krb5_principal,
    mut service: *mut i8,
    mut host: *mut i8,
    mut ad_context: crate::k5_int_h::krb5_authdata_context,
    mut flags: crate::krb5_h::krb5_flags,
    mut ret_name: *mut crate::gssapiP_krb5_h::krb5_gss_name_t,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut name = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    *ret_name = 0 as crate::gssapiP_krb5_h::krb5_gss_name_t;
    if !principal.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"principal != NULL\x00" as *const u8 as
                          *const i8,
                      b"naming_exts.c\x00" as *const u8 as
                          *const i8,
                      40u32,
                      (*::std::mem::transmute::<&[u8; 129],
                                                &[i8; 129]>(b"krb5_error_code kg_init_name(krb5_context, krb5_principal, char *, char *, krb5_authdata_context, krb5_flags, krb5_gss_name_t *)\x00")).as_ptr());
    }
    if principal.is_null() {
        return 22i32;
    }
    name = crate::stdlib::malloc(::std::mem::size_of::<
        crate::gssapiP_krb5_h::krb5_gss_name_rec,
    >()) as crate::gssapiP_krb5_h::krb5_gss_name_t;
    if name.is_null() {
        return 12i32;
    }
    crate::stdlib::memset(
        name as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_name_rec>(),
    );
    code = k5_mutex_init(&mut (*name).lock);
    if !(code != 0i32) {
        if flags & 0x1i32 == 0i32 {
            code = crate::krb5_h::krb5_copy_principal(
                context,
                principal as crate::krb5_h::krb5_const_principal,
                &mut (*name).princ,
            );
            if code != 0i32 {
                current_block = 4931485695408934665;
            } else {
                if !ad_context.is_null() {
                    code = crate::k5_int_h::krb5_authdata_context_copy(
                        context,
                        ad_context,
                        &mut (*name).ad_context,
                    );
                    if code != 0i32 {
                        current_block = 4931485695408934665;
                    } else {
                        current_block = 1054647088692577877;
                    }
                } else {
                    current_block = 1054647088692577877;
                }
                match current_block {
                    4931485695408934665 => {}
                    _ => {
                        code = 12i32;
                        if !service.is_null() {
                            (*name).service = crate::stdlib::strdup(service);
                            if (*name).service.is_null() {
                                current_block = 4931485695408934665;
                            } else {
                                current_block = 26972500619410423;
                            }
                        } else {
                            current_block = 26972500619410423;
                        }
                        match current_block {
                            4931485695408934665 => {}
                            _ => {
                                if !host.is_null() {
                                    (*name).host = crate::stdlib::strdup(host);
                                    if (*name).host.is_null() {
                                        current_block = 4931485695408934665;
                                    } else {
                                        current_block = 11194104282611034094;
                                    }
                                } else {
                                    current_block = 11194104282611034094;
                                }
                                match current_block {
                                    4931485695408934665 => {}
                                    _ => {
                                        code = 0i32;
                                        current_block = 4488286894823169796;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            (*name).princ = principal;
            (*name).service = service;
            (*name).host = host;
            (*name).ad_context = ad_context;
            current_block = 4488286894823169796;
        }
        match current_block {
            4931485695408934665 => {}
            _ => *ret_name = name,
        }
    }
    if code != 0i32 {
        kg_release_name(context, &mut name);
    }
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn kg_release_name(
    mut context: crate::krb5_h::krb5_context,
    mut name: *mut crate::gssapiP_krb5_h::krb5_gss_name_t,
) -> crate::krb5_h::krb5_error_code {
    if !(*name).is_null() {
        crate::krb5_h::krb5_free_principal(context, (**name).princ);
        crate::stdlib::free((**name).service as *mut libc::c_void);
        crate::stdlib::free((**name).host as *mut libc::c_void);
        crate::k5_int_h::krb5_authdata_context_free(context, (**name).ad_context);
        crate::k5_thread_h::k5_os_mutex_destroy(&mut (**name).lock);
        crate::stdlib::free(*name as *mut libc::c_void);
        *name = 0 as crate::gssapiP_krb5_h::krb5_gss_name_t
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn kg_duplicate_name(
    mut context: crate::krb5_h::krb5_context,
    src: crate::gssapiP_krb5_h::krb5_gss_name_t,
    mut dst: *mut crate::gssapiP_krb5_h::krb5_gss_name_t,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    k5_mutex_lock(&mut (*src).lock);
    code = kg_init_name(
        context,
        (*src).princ,
        (*src).service,
        (*src).host,
        (*src).ad_context,
        0i32,
        dst,
    );
    k5_mutex_unlock(&mut (*src).lock);
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn kg_compare_name(
    mut context: crate::krb5_h::krb5_context,
    mut name1: crate::gssapiP_krb5_h::krb5_gss_name_t,
    mut name2: crate::gssapiP_krb5_h::krb5_gss_name_t,
) -> crate::krb5_h::krb5_boolean {
    return crate::krb5_h::krb5_principal_compare(
        context,
        (*name1).princ as crate::krb5_h::krb5_const_principal,
        (*name2).princ as crate::krb5_h::krb5_const_principal,
    );
}
/* Determine the principal to use for an acceptor name, which is different from
 * name->princ for host-based names. */
#[no_mangle]

pub unsafe extern "C" fn kg_acceptor_princ(
    mut context: crate::krb5_h::krb5_context,
    mut name: crate::gssapiP_krb5_h::krb5_gss_name_t,
    mut princ_out: *mut crate::krb5_h::krb5_principal,
) -> crate::krb5_h::krb5_boolean {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut host = 0 as *const i8;
    let mut tmp = 0 as *mut i8;
    *princ_out = 0 as crate::krb5_h::krb5_principal;
    if name.is_null() {
        return 0u32;
    }
    /* If it's not a host-based name, just copy name->princ. */
    if (*name).service.is_null() {
        return crate::krb5_h::krb5_copy_principal(
            context,
            (*name).princ as crate::krb5_h::krb5_const_principal,
            princ_out,
        ) as crate::krb5_h::krb5_boolean;
    } /* No host was given; use an empty string. */
    if !(*name).host.is_null() && (*(*name).princ).length == 2i32 {
        let mut d: *const crate::krb5_h::krb5_data =
            &mut *(*(*name).princ).data.offset(1isize) as *mut crate::krb5_h::krb5_data;
        tmp = k5memdup0(
            (*d).data as *const libc::c_void,
            (*d).length as crate::stddef_h::size_t,
            &mut code,
        ) as *mut i8;
        if tmp.is_null() {
            return 12u32;
        }
        host = tmp
    /* If a host was given, we have to use the canonicalized form of it (as
     * given by krb5_sname_to_principal) for backward compatibility. */
    } else {
        host = b"\x00" as *const u8 as *const i8
    }
    code = crate::krb5_h::krb5_build_principal(
        context,
        princ_out,
        0u32,
        b"\x00" as *const u8 as *const i8,
        (*name).service,
        host,
        0 as *mut i8,
    );
    if !(*princ_out).is_null() {
        (**princ_out).type_0 = 3i32
    }
    crate::stdlib::free(tmp as *mut libc::c_void);
    return code as crate::krb5_h::krb5_boolean;
}

unsafe extern "C" fn kg_map_name_error(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut code: crate::krb5_h::krb5_error_code,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    match code {
        0 => major_status = 0u32,
        2 | 1 => major_status = (16u32) << 16i32,
        _ => major_status = (13u32) << 16i32,
    }
    *minor_status = code as crate::gssapi_h::OM_uint32;
    return major_status;
}
/* Owns data on success */

unsafe extern "C" fn data_list_to_buffer_set(
    mut context: crate::krb5_h::krb5_context,
    mut data: *mut crate::krb5_h::krb5_data,
    mut buffer_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::krb5_h::krb5_error_code {
    let mut set = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    let mut minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut i: i32 = 0;
    let mut code = 0i32;
    if !data.is_null() {
        if !buffer_set.is_null() {
            if crate::src::mechglue::g_buffer_set::gss_create_empty_buffer_set(
                &mut minor_status,
                &mut set,
            ) & ((0o377u32) << 24i32 | (0o377u32) << 16i32)
                != 0
            {
                if minor_status != 0u32 {
                } else {
                    crate::stdlib::__assert_fail(b"minor_status != 0\x00" as *const u8 as
                                      *const i8,
                                  b"naming_exts.c\x00" as *const u8 as
                                      *const i8,
                                  215u32,
                                  (*::std::mem::transmute::<&[u8; 87],
                                                            &[i8; 87]>(b"krb5_error_code data_list_to_buffer_set(krb5_context, krb5_data *, gss_buffer_set_t *)\x00")).as_ptr());
                }
                code = minor_status as crate::krb5_h::krb5_error_code
            } else {
                i = 0i32;
                while !(*data.offset(i as isize)).data.is_null() {
                    i += 1
                }
                (*set).count = i as crate::stddef_h::size_t;
                (*set).elements = gssalloc_calloc(
                    i as crate::stddef_h::size_t,
                    ::std::mem::size_of::<crate::gssapi_h::gss_buffer_desc>(),
                ) as *mut crate::gssapi_h::gss_buffer_desc;
                if (*set).elements.is_null() {
                    crate::src::mechglue::g_buffer_set::gss_release_buffer_set(
                        &mut minor_status,
                        &mut set,
                    );
                    code = 12i32
                } else {
                    /*
                     * Copy last element first so data remains properly
                     * NULL-terminated in case of allocation failure
                     * in data_to_gss() on windows.
                     */
                    i = (*set).count.wrapping_sub(1usize) as i32;
                    while i >= 0i32 {
                        if data_to_gss(
                            &mut *data.offset(i as isize),
                            &mut *(*set).elements.offset(i as isize),
                        ) != 0
                        {
                            crate::src::mechglue::g_buffer_set::gss_release_buffer_set(
                                &mut minor_status,
                                &mut set,
                            );
                            code = 12i32;
                            break;
                        } else {
                            i -= 1
                        }
                    }
                }
            }
        }
    }
    crate::k5_int_h::krb5int_free_data_list(context, data);
    if !buffer_set.is_null() {
        *buffer_set = set
    }
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_inquire_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut _name_is_MN: *mut i32,
    mut _MN_mech: *mut crate::gssapi_h::gss_OID,
    mut attrs: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kname = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut kattrs = 0 as *mut crate::krb5_h::krb5_data;
    *minor_status = 0u32;
    if !attrs.is_null() {
        *attrs = 0 as crate::gssapi_ext_h::gss_buffer_set_t
    }
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0i32 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    kname = name as crate::gssapiP_krb5_h::krb5_gss_name_t;
    k5_mutex_lock(&mut (*kname).lock);
    if (*kname).ad_context.is_null() {
        code = crate::k5_int_h::krb5_authdata_context_init(context, &mut (*kname).ad_context);
        if code != 0i32 {
            current_block = 6168349222852989109;
        } else {
            current_block = 3512920355445576850;
        }
    } else {
        current_block = 3512920355445576850;
    }
    match current_block {
        3512920355445576850 => {
            code = crate::k5_int_h::krb5_authdata_get_attribute_types(
                context,
                (*kname).ad_context,
                &mut kattrs,
            );
            if !(code != 0i32) {
                code = data_list_to_buffer_set(context, kattrs, attrs);
                kattrs = 0 as *mut crate::krb5_h::krb5_data;
                (code) != 0i32;
            }
        }
        _ => {}
    }
    k5_mutex_unlock(&mut (*kname).lock);
    crate::k5_int_h::krb5int_free_data_list(context, kattrs);
    crate::krb5_h::krb5_free_context(context);
    return kg_map_name_error(minor_status, code);
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_get_name_attribute(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut attr: crate::gssapi_h::gss_buffer_t,
    mut authenticated: *mut i32,
    mut complete: *mut i32,
    mut value: crate::gssapi_h::gss_buffer_t,
    mut display_value: crate::gssapi_h::gss_buffer_t,
    mut more: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kname = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut kattr = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut kauthenticated: crate::krb5_h::krb5_boolean = 0;
    let mut kcomplete: crate::krb5_h::krb5_boolean = 0;
    let mut kvalue = empty_data();
    let mut kdisplay_value = empty_data();
    *minor_status = 0u32;
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0i32 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    kname = name as crate::gssapiP_krb5_h::krb5_gss_name_t;
    k5_mutex_lock(&mut (*kname).lock);
    if (*kname).ad_context.is_null() {
        code = crate::k5_int_h::krb5_authdata_context_init(context, &mut (*kname).ad_context);
        if code != 0i32 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            k5_mutex_unlock(&mut (*kname).lock);
            crate::krb5_h::krb5_free_context(context);
            return (16u32) << 16i32;
        }
    }
    kattr.data = (*attr).value as *mut i8;
    kattr.length = (*attr).length as u32;
    kauthenticated = 0u32;
    kcomplete = 0u32;
    code = crate::k5_int_h::krb5_authdata_get_attribute(
        context,
        (*kname).ad_context,
        &mut kattr,
        &mut kauthenticated,
        &mut kcomplete,
        &mut kvalue,
        &mut kdisplay_value,
        more,
    );
    if code == 0i32 {
        if !value.is_null() {
            code = data_to_gss(&mut kvalue, value)
        }
        if !authenticated.is_null() {
            *authenticated = kauthenticated as i32
        }
        if !complete.is_null() {
            *complete = kcomplete as i32
        }
        if !display_value.is_null() && code == 0i32 {
            code = data_to_gss(&mut kdisplay_value, display_value)
        }
    }
    crate::stdlib::free(kdisplay_value.data as *mut libc::c_void);
    crate::stdlib::free(kvalue.data as *mut libc::c_void);
    k5_mutex_unlock(&mut (*kname).lock);
    crate::krb5_h::krb5_free_context(context);
    return kg_map_name_error(minor_status, code);
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_set_name_attribute(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut complete: i32,
    mut attr: crate::gssapi_h::gss_buffer_t,
    mut value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kname = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut kattr = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut kvalue = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    *minor_status = 0u32;
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0i32 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    kname = name as crate::gssapiP_krb5_h::krb5_gss_name_t;
    k5_mutex_lock(&mut (*kname).lock);
    if (*kname).ad_context.is_null() {
        code = crate::k5_int_h::krb5_authdata_context_init(context, &mut (*kname).ad_context);
        if code != 0i32 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            k5_mutex_unlock(&mut (*kname).lock);
            crate::krb5_h::krb5_free_context(context);
            return (16u32) << 16i32;
        }
    }
    kattr.data = (*attr).value as *mut i8;
    kattr.length = (*attr).length as u32;
    kvalue.data = (*value).value as *mut i8;
    kvalue.length = (*value).length as u32;
    code = crate::k5_int_h::krb5_authdata_set_attribute(
        context,
        (*kname).ad_context,
        complete as crate::krb5_h::krb5_boolean,
        &mut kattr,
        &mut kvalue,
    );
    k5_mutex_unlock(&mut (*kname).lock);
    crate::krb5_h::krb5_free_context(context);
    return kg_map_name_error(minor_status, code);
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_delete_name_attribute(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut attr: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kname = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut kattr = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    *minor_status = 0u32;
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0i32 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    kname = name as crate::gssapiP_krb5_h::krb5_gss_name_t;
    k5_mutex_lock(&mut (*kname).lock);
    if (*kname).ad_context.is_null() {
        code = crate::k5_int_h::krb5_authdata_context_init(context, &mut (*kname).ad_context);
        if code != 0i32 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            k5_mutex_unlock(&mut (*kname).lock);
            crate::krb5_h::krb5_free_context(context);
            return (16u32) << 16i32;
        }
    }
    kattr.data = (*attr).value as *mut i8;
    kattr.length = (*attr).length as u32;
    code =
        crate::k5_int_h::krb5_authdata_delete_attribute(context, (*kname).ad_context, &mut kattr);
    k5_mutex_unlock(&mut (*kname).lock);
    crate::krb5_h::krb5_free_context(context);
    return kg_map_name_error(minor_status, code);
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_map_name_to_any(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut authenticated: i32,
    mut type_id: crate::gssapi_h::gss_buffer_t,
    mut output: *mut crate::gssapi_ext_h::gss_any_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kname = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut kmodule = 0 as *mut i8;
    *minor_status = 0u32;
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0i32 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    kname = name as crate::gssapiP_krb5_h::krb5_gss_name_t;
    k5_mutex_lock(&mut (*kname).lock);
    if (*kname).ad_context.is_null() {
        code = crate::k5_int_h::krb5_authdata_context_init(context, &mut (*kname).ad_context);
        if code != 0i32 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            k5_mutex_unlock(&mut (*kname).lock);
            crate::krb5_h::krb5_free_context(context);
            return (16u32) << 16i32;
        }
    }
    kmodule = (*type_id).value as *mut i8;
    if *kmodule.offset((*type_id).length as isize) as i32 != '\u{0}' as i32 {
        k5_mutex_unlock(&mut (*kname).lock);
        crate::krb5_h::krb5_free_context(context);
        return (16u32) << 16i32;
    }
    code = crate::k5_int_h::krb5_authdata_export_internal(
        context,
        (*kname).ad_context,
        authenticated as crate::krb5_h::krb5_boolean,
        kmodule,
        output as *mut *mut libc::c_void,
    );
    k5_mutex_unlock(&mut (*kname).lock);
    crate::krb5_h::krb5_free_context(context);
    return kg_map_name_error(minor_status, code);
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_release_any_name_mapping(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut type_id: crate::gssapi_h::gss_buffer_t,
    mut input: *mut crate::gssapi_ext_h::gss_any_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kname = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut kmodule = 0 as *mut i8;
    *minor_status = 0u32;
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0i32 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    kname = name as crate::gssapiP_krb5_h::krb5_gss_name_t;
    k5_mutex_lock(&mut (*kname).lock);
    if (*kname).ad_context.is_null() {
        code = crate::k5_int_h::krb5_authdata_context_init(context, &mut (*kname).ad_context);
        if code != 0i32 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            k5_mutex_unlock(&mut (*kname).lock);
            crate::krb5_h::krb5_free_context(context);
            return (16u32) << 16i32;
        }
    }
    kmodule = (*type_id).value as *mut i8;
    if *kmodule.offset((*type_id).length as isize) as i32 != '\u{0}' as i32 {
        k5_mutex_unlock(&mut (*kname).lock);
        crate::krb5_h::krb5_free_context(context);
        return (16u32) << 16i32;
    }
    code = crate::k5_int_h::krb5_authdata_free_internal(
        context,
        (*kname).ad_context,
        kmodule,
        *input as *mut libc::c_void,
    );
    if code == 0i32 {
        *input = 0 as crate::gssapi_ext_h::gss_any_t
    }
    k5_mutex_unlock(&mut (*kname).lock);
    crate::krb5_h::krb5_free_context(context);
    return kg_map_name_error(minor_status, code);
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
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_export_name_composite(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: crate::gssapi_h::gss_name_t,
    mut exp_composite_name: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut kname = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut attrs = 0 as *mut crate::krb5_h::krb5_data;
    let mut princstr = 0 as *mut i8;
    let mut cp = 0 as *mut u8;
    let mut princlen: crate::stddef_h::size_t = 0;
    *minor_status = 0u32;
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0i32 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    kname = name as crate::gssapiP_krb5_h::krb5_gss_name_t;
    k5_mutex_lock(&mut (*kname).lock);
    code = crate::krb5_h::krb5_unparse_name(
        context,
        (*kname).princ as crate::krb5_h::krb5_const_principal,
        &mut princstr,
    );
    if !(code != 0i32) {
        princlen = crate::stdlib::strlen(princstr);
        if !(*kname).ad_context.is_null() {
            code = crate::k5_int_h::krb5_authdata_export_attributes(
                context,
                (*kname).ad_context,
                0x2fi32,
                &mut attrs,
            );
            if code != 0i32 {
                current_block = 7857450428241827620;
            } else {
                current_block = 5143058163439228106;
            }
        } else {
            current_block = 5143058163439228106;
        }
        match current_block {
            7857450428241827620 => {}
            _ => {
                /* 04 02 OID Name AuthData */
                (*exp_composite_name).length = ((10u32)
                    .wrapping_add((*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length)
                    as usize)
                    .wrapping_add(princlen); /* length of encoded attributes */
                (*exp_composite_name).length = ((*exp_composite_name).length).wrapping_add(4usize);
                if !attrs.is_null() {
                    (*exp_composite_name).length =
                        ((*exp_composite_name).length).wrapping_add((*attrs).length as usize)
                }
                (*exp_composite_name).value = gssalloc_malloc((*exp_composite_name).length);
                if (*exp_composite_name).value.is_null() {
                    code = 12i32
                } else {
                    cp = (*exp_composite_name).value as *mut u8;
                    /* Note: we assume the OID will be less than 128 bytes... */
                    let fresh0 = cp;
                    cp = cp.offset(1);
                    *fresh0 = 0x4u8;
                    let fresh1 = cp;
                    cp = cp.offset(1);
                    *fresh1 = 0x2u8;
                    store_16_be(
                        (*crate::src::krb5::gssapi_krb5::gss_mech_krb5)
                            .length
                            .wrapping_add(2u32),
                        cp as *mut libc::c_void,
                    );
                    cp = cp.offset(2isize);
                    let fresh2 = cp;
                    cp = cp.offset(1);
                    *fresh2 = 0x6u8;
                    let fresh3 = cp;
                    cp = cp.offset(1);
                    *fresh3 =
                        ((*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length & 0xffu32) as u8;
                    crate::stdlib::memcpy(
                        cp as *mut libc::c_void,
                        (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).elements,
                        (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length as usize,
                    );
                    cp = cp.offset((*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length as isize);
                    store_32_be(princlen as u32, cp as *mut libc::c_void);
                    cp = cp.offset(4isize);
                    crate::stdlib::memcpy(
                        cp as *mut libc::c_void,
                        princstr as *const libc::c_void,
                        princlen,
                    );
                    cp = cp.offset(princlen as isize);
                    store_32_be(
                        if !attrs.is_null() {
                            (*attrs).length
                        } else {
                            0u32
                        },
                        cp as *mut libc::c_void,
                    );
                    cp = cp.offset(4isize);
                    if !attrs.is_null() {
                        crate::stdlib::memcpy(
                            cp as *mut libc::c_void,
                            (*attrs).data as *const libc::c_void,
                            (*attrs).length as usize,
                        );
                        cp = cp.offset((*attrs).length as isize)
                    }
                }
            }
        }
    }
    crate::krb5_h::krb5_free_unparsed_name(context, princstr);
    crate::krb5_h::krb5_free_data(context, attrs);
    k5_mutex_unlock(&mut (*kname).lock);
    crate::krb5_h::krb5_free_context(context);
    return kg_map_name_error(minor_status, code);
}
