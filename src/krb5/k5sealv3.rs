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

    pub unsafe extern "C" fn store_64_be(
        mut val: crate::stdlib::uint64_t,
        mut vp: *mut libc::c_void,
    ) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_4)).i = __bswap_64(val);
    }
    #[inline]

    pub unsafe extern "C" fn load_16_be(mut cvp: *const libc::c_void) -> u16 {
        let mut p = cvp as *const u8;
        return __bswap_16((*(p as *const crate::k5_platform_h::C2RustUnnamed_5)).i);
    }
    #[inline]

    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void) -> u32 {
        let mut p = cvp as *const u8;
        return __bswap_32((*(p as *const crate::k5_platform_h::C2RustUnnamed_6)).i);
    }
    #[inline]

    pub unsafe extern "C" fn load_64_be(mut cvp: *const libc::c_void) -> crate::stdlib::uint64_t {
        let mut p = cvp as *const u8;
        return __bswap_64((*(p as *const crate::k5_platform_h::C2RustUnnamed_4)).i);
    }

    use crate::src::krb5::k5sealv3::byteswap_h::__bswap_16;
    use crate::src::krb5::k5sealv3::byteswap_h::__bswap_32;
    use crate::src::krb5::k5sealv3::byteswap_h::__bswap_64;

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

    pub unsafe extern "C" fn alloc_data(
        mut data: *mut crate::krb5_h::krb5_data,
        mut len: u32,
    ) -> crate::krb5_h::krb5_error_code {
        let mut ptr =
            crate::stdlib::calloc(if len > 0u32 { len } else { 1u32 } as usize, 1usize) as *mut i8;
        if ptr.is_null() {
            return 12i32;
        }
        (*data).magic = -(1760647422 as isize) as crate::krb5_h::krb5_magic;
        (*data).data = ptr;
        (*data).length = len;
        return 0i32;
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

pub mod byteswap_h {
    #[inline]

    pub unsafe extern "C" fn __bswap_64(
        mut __bsx: crate::stdlib::__uint64_t,
    ) -> crate::stdlib::__uint64_t {
        return ((__bsx as u64 & 0xff00000000000000u64) >> 56i32
            | (__bsx as u64 & 0xff000000000000u64) >> 40i32
            | (__bsx as u64 & 0xff0000000000u64) >> 24i32
            | (__bsx as u64 & 0xff00000000u64) >> 8i32
            | (__bsx as u64 & 0xff000000u64) << 8i32
            | (__bsx as u64 & 0xff0000u64) << 24i32
            | (__bsx as u64 & 0xff00u64) << 40i32
            | (__bsx as u64 & 0xffu64) << 56i32) as crate::stdlib::__uint64_t;
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
pub use crate::k5_platform_h::C2RustUnnamed_4;
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
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_encrypt_size;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_checksum_contents;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_k_decrypt;
pub use crate::krb5_h::krb5_k_encrypt;
pub use crate::krb5_h::krb5_k_make_checksum;
pub use crate::krb5_h::krb5_k_verify_checksum;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keyusage;
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
pub use crate::src::krb5::k5sealv3::k5_int_h::alloc_data;
pub use crate::src::krb5::k5sealv3::k5_int_h::empty_data;
pub use crate::src::krb5::k5sealv3::k5_int_h::make_data;
pub use crate::src::krb5::k5sealv3::k5_platform_h::load_16_be;
pub use crate::src::krb5::k5sealv3::k5_platform_h::load_32_be;
pub use crate::src::krb5::k5sealv3::k5_platform_h::load_64_be;
pub use crate::src::krb5::k5sealv3::k5_platform_h::store_16_be;
pub use crate::src::krb5::k5sealv3::k5_platform_h::store_64_be;
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
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::src::generic::util_seqstate::gssint_g_seqstate_check;
pub use crate::src::krb5::disp_status::krb5_gss_save_error_info;

pub use crate::src::krb5::k5sealv3::byteswap_h::__bswap_16;
pub use crate::src::krb5::k5sealv3::byteswap_h::__bswap_32;
pub use crate::src::krb5::k5sealv3::byteswap_h::__bswap_64;
pub use crate::src::krb5::k5sealv3::gssapi_alloc_h::gssalloc_free;
pub use crate::src::krb5::k5sealv3::gssapi_alloc_h::gssalloc_malloc;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/k5sealv3.c */
/*
 * Copyright 2003,2004,2007 by the Massachusetts Institute of Technology.
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
/* draft-ietf-krb-wg-gssapi-cfx-05 */
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_rotate_left(
    mut ptr: *mut libc::c_void,
    mut bufsiz: crate::stddef_h::size_t,
    mut rc: crate::stddef_h::size_t,
) -> i32 {
    /* Optimize for receiving.  After some debugging is done, the MIT
    implementation won't do any rotates on sending, and while
    debugging, they'll be randomly chosen.

    Return 1 for success, 0 for failure (ENOMEM).  */
    let mut tbuf = 0 as *mut libc::c_void;
    if bufsiz == 0usize {
        return 1i32;
    }
    rc = rc.wrapping_rem(bufsiz);
    if rc == 0usize {
        return 1i32;
    }
    tbuf = crate::stdlib::malloc(rc);
    if tbuf.is_null() {
        return 0i32;
    }
    crate::stdlib::memcpy(tbuf, ptr, rc);
    crate::stdlib::memmove(
        ptr,
        (ptr as *mut i8).offset(rc as isize) as *const libc::c_void,
        bufsiz.wrapping_sub(rc),
    );
    crate::stdlib::memcpy(
        (ptr as *mut i8)
            .offset(bufsiz as isize)
            .offset(-(rc as isize)) as *mut libc::c_void,
        tbuf,
        rc,
    );
    crate::stdlib::free(tbuf);
    return 1i32;
}

static mut empty_message: crate::gssapi_h::gss_buffer_desc = {
    let mut init = crate::gssapi_h::gss_buffer_desc_struct {
        length: 0usize,
        value: 0 as *mut libc::c_void,
    };
    init
};
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_make_seal_token_v3(
    mut context: crate::krb5_h::krb5_context,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut message: *const crate::gssapi_h::gss_buffer_desc,
    mut token: crate::gssapi_h::gss_buffer_t,
    mut conf_req_flag: i32,
    mut toktype: i32,
) -> crate::krb5_h::krb5_error_code {
    let mut plain_0: crate::krb5_h::krb5_data = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut cksumsize: crate::stddef_h::size_t = 0;
    let mut current_block: u64;
    let mut bufsize = 16usize;
    let mut outbuf = 0 as *mut u8;
    let mut err: crate::krb5_h::krb5_error_code = 0;
    let mut key_usage: i32 = 0;
    let mut acceptor_flag: u8 = 0;
    let mut message2 = message;
    let mut ec: crate::stddef_h::size_t = 0;
    let mut tok_id: u16 = 0;
    let mut sum = crate::krb5_h::krb5_checksum {
        magic: 0,
        checksum_type: 0,
        length: 0,
        contents: 0 as *mut crate::krb5_h::krb5_octet,
    };
    let mut key = 0 as *mut crate::k5_int_h::krb5_key_st;
    let mut cksumtype: crate::krb5_h::krb5_cksumtype = 0;
    acceptor_flag = if (*ctx).initiate() as i32 != 0 {
        0i32
    } else {
        0x1i32
    } as u8;
    key_usage = if toktype == 0x201i32 {
        if (*ctx).initiate() as i32 != 0 {
            24i32
        } else {
            22i32
        }
    } else if (*ctx).initiate() as i32 != 0 {
        25i32
    } else {
        23i32
    };
    if (*ctx).have_acceptor_subkey() != 0 {
        key = (*ctx).acceptor_subkey;
        cksumtype = (*ctx).acceptor_subkey_cksumtype
    } else {
        key = (*ctx).subkey;
        cksumtype = (*ctx).cksumtype
    }
    if !key.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"key != NULL\x00" as *const u8 as *const i8,
                      b"k5sealv3.c\x00" as *const u8 as *const i8,
                      97u32,
                      (*::std::mem::transmute::<&[u8; 133],
                                                &[i8; 133]>(b"krb5_error_code gss_krb5int_make_seal_token_v3(krb5_context, krb5_gss_ctx_id_rec *, const gss_buffer_desc *, gss_buffer_t, int, int)\x00")).as_ptr());
    }
    if toktype == 0x201i32 && conf_req_flag != 0 {
        let mut plain = crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        };
        let mut cipher = crate::krb5_h::krb5_enc_data {
            magic: 0,
            enctype: 0,
            kvno: 0,
            ciphertext: crate::krb5_h::krb5_data {
                magic: 0,
                length: 0,
                data: 0 as *mut i8,
            },
        };
        let mut ec_max: crate::stddef_h::size_t = 0;
        let mut encrypt_size: crate::stddef_h::size_t = 0;
        /* 300: Adds some slop.  */
        if (18446744073709551615 as usize).wrapping_sub(300usize) < (*message).length {
            return 12i32;
        }
        ec_max = (18446744073709551615 as usize)
            .wrapping_sub((*message).length)
            .wrapping_sub(300usize);
        if ec_max > 0xffffusize {
            ec_max = 0xffffusize
        }
        ec = 0usize;
        err = alloc_data(
            &mut plain,
            (*message).length.wrapping_add(16usize).wrapping_add(ec) as u32,
        );
        if err != 0 {
            return err;
        }
        /* Get size of ciphertext.  */
        encrypt_size = crate::krb5_h::krb5_encrypt_size(
            plain.length as crate::stddef_h::size_t,
            (*key).keyblock.enctype,
        );
        if encrypt_size > (18446744073709551615 as usize).wrapping_div(2usize) {
            err = 12i32;
            current_block = 2945127981008348964;
        } else {
            bufsize = (16usize).wrapping_add(encrypt_size);
            /* Allocate space for header plus encrypted data.  */
            outbuf = gssalloc_malloc(bufsize) as *mut u8;
            if outbuf.is_null() {
                crate::stdlib::free(plain.data as *mut libc::c_void);
                return 12i32;
            }
            /* TOK_ID */
            store_16_be(0x504u32, outbuf as *mut libc::c_void);
            /* flags */
            *outbuf.offset(2isize) = (acceptor_flag as i32
                | 0x2i32
                | (if (*ctx).have_acceptor_subkey() as i32 != 0 {
                    0x4i32
                } else {
                    0i32
                })) as u8;
            /* filler */
            *outbuf.offset(3isize) = 0xffu8;
            /* EC */
            store_16_be(ec as u32, outbuf.offset(4isize) as *mut libc::c_void);
            /* RRC */
            store_16_be(0u32, outbuf.offset(6isize) as *mut libc::c_void);
            store_64_be((*ctx).seq_send, outbuf.offset(8isize) as *mut libc::c_void);
            crate::stdlib::memcpy(
                plain.data as *mut libc::c_void,
                (*message).value,
                (*message).length,
            );
            if ec != 0usize {
                crate::stdlib::memset(
                    plain.data.offset((*message).length as isize) as *mut libc::c_void,
                    'x' as i32,
                    ec,
                );
            }
            crate::stdlib::memcpy(
                plain
                    .data
                    .offset((*message).length as isize)
                    .offset(ec as isize) as *mut libc::c_void,
                outbuf as *const libc::c_void,
                16usize,
            );
            cipher.ciphertext.data = (outbuf as *mut i8).offset(16isize);
            cipher.ciphertext.length = bufsize.wrapping_sub(16usize) as u32;
            cipher.enctype = (*key).keyblock.enctype;
            err = crate::krb5_h::krb5_k_encrypt(
                context,
                key,
                key_usage,
                0 as *const crate::krb5_h::krb5_data,
                &mut plain,
                &mut cipher,
            );
            crate::stdlib::explicit_bzero(
                plain.data as *mut libc::c_void,
                plain.length as crate::stddef_h::size_t,
            );
            crate::stdlib::free(plain.data as *mut libc::c_void);
            plain.data = 0 as *mut i8;
            if err != 0 {
                current_block = 2945127981008348964;
            } else {
                /* Now that we know we're returning a valid token....  */
                (*ctx).seq_send = (*ctx).seq_send.wrapping_add(1);
                current_block = 5706507068631705000;
            }
        }
    } else {
        if toktype == 0x201i32 && conf_req_flag == 0 {
            plain_0 = crate::krb5_h::krb5_data {
                magic: 0,
                length: 0,
                data: 0 as *mut i8,
            };
            cksumsize = 0;
            /* Here, message is the application-supplied data; message2 is
            what goes into the output token.  They may be the same, or
            message2 may be empty (for MIC).  */
            tok_id = 0x504u16
        } else if toktype == 0x101i32 {
            tok_id = 0x404u16;
            message2 = &empty_message
        } else if toktype == 0x102i32 {
            tok_id = 0x405u16;
            message2 = &empty_message;
            message = message2
        } else {
            crate::stdlib::abort();
        }
        err = alloc_data(&mut plain_0, (*message).length.wrapping_add(16usize) as u32);
        if err != 0 {
            return err;
        }
        err = crate::krb5_h::krb5_c_checksum_length(context, cksumtype, &mut cksumsize);
        if err != 0 {
            current_block = 2945127981008348964;
        } else {
            if cksumsize <= 0xffffusize {
            } else {
                crate::stdlib::__assert_fail(b"cksumsize <= 0xffff\x00" as *const u8 as
                                  *const i8,
                              b"k5sealv3.c\x00" as *const u8 as
                                  *const i8,
                              202u32,
                              (*::std::mem::transmute::<&[u8; 133],
                                                        &[i8; 133]>(b"krb5_error_code gss_krb5int_make_seal_token_v3(krb5_context, krb5_gss_ctx_id_rec *, const gss_buffer_desc *, gss_buffer_t, int, int)\x00")).as_ptr());
            }
            bufsize = (16usize)
                .wrapping_add((*message2).length)
                .wrapping_add(cksumsize);
            outbuf = gssalloc_malloc(bufsize) as *mut u8;
            if outbuf.is_null() {
                crate::stdlib::free(plain_0.data as *mut libc::c_void);
                plain_0.data = 0 as *mut i8;
                err = 12i32;
                current_block = 2945127981008348964;
            } else {
                /* TOK_ID */
                store_16_be(tok_id as u32, outbuf as *mut libc::c_void);
                /* flags */
                *outbuf.offset(2isize) = (acceptor_flag as i32
                    | (if (*ctx).have_acceptor_subkey() as i32 != 0 {
                        0x4i32
                    } else {
                        0i32
                    })) as u8;
                /* filler */
                *outbuf.offset(3isize) = 0xffu8;
                if toktype == 0x201i32 {
                    /* Use 0 for checksum calculation, substitute
                    checksum length later.  */
                    /* EC */
                    store_16_be(0u32, outbuf.offset(4isize) as *mut libc::c_void);
                    /* RRC */
                    store_16_be(0u32, outbuf.offset(6isize) as *mut libc::c_void);
                } else {
                    /* MIC and DEL store 0xFF in EC and RRC.  */
                    store_16_be(0xffffu32, outbuf.offset(4isize) as *mut libc::c_void);
                    store_16_be(0xffffu32, outbuf.offset(6isize) as *mut libc::c_void);
                }
                store_64_be((*ctx).seq_send, outbuf.offset(8isize) as *mut libc::c_void);
                crate::stdlib::memcpy(
                    plain_0.data as *mut libc::c_void,
                    (*message).value,
                    (*message).length,
                );
                crate::stdlib::memcpy(
                    plain_0.data.offset((*message).length as isize) as *mut libc::c_void,
                    outbuf as *const libc::c_void,
                    16usize,
                );
                /* Fill in the output token -- data contents, if any, and
                space for the checksum.  */
                if (*message2).length != 0 {
                    crate::stdlib::memcpy(
                        outbuf.offset(16isize) as *mut libc::c_void,
                        (*message2).value,
                        (*message2).length,
                    );
                }
                sum.contents = outbuf.offset(16isize).offset((*message2).length as isize);
                sum.length = cksumsize as u32;
                err = crate::krb5_h::krb5_k_make_checksum(
                    context,
                    cksumtype,
                    key,
                    key_usage,
                    &mut plain_0,
                    &mut sum,
                );
                crate::stdlib::explicit_bzero(
                    plain_0.data as *mut libc::c_void,
                    plain_0.length as crate::stddef_h::size_t,
                );
                crate::stdlib::free(plain_0.data as *mut libc::c_void);
                plain_0.data = 0 as *mut i8;
                if err != 0 {
                    crate::stdlib::explicit_bzero(outbuf as *mut libc::c_void, bufsize);
                    current_block = 2945127981008348964;
                } else {
                    if sum.length as usize != cksumsize {
                        crate::stdlib::abort();
                    }
                    crate::stdlib::memcpy(
                        outbuf.offset(16isize).offset((*message2).length as isize)
                            as *mut libc::c_void,
                        sum.contents as *const libc::c_void,
                        cksumsize,
                    );
                    crate::krb5_h::krb5_free_checksum_contents(context, &mut sum);
                    sum.contents = 0 as *mut crate::krb5_h::krb5_octet;
                    /* Now that we know we're actually generating the token...  */
                    (*ctx).seq_send = (*ctx).seq_send.wrapping_add(1);
                    if toktype == 0x201i32 {
                        /* Fix up EC field.  */
                        store_16_be(cksumsize as u32, outbuf.offset(4isize) as *mut libc::c_void);
                    } else {
                        store_16_be(0xffffu32, outbuf.offset(6isize) as *mut libc::c_void);
                    }
                    current_block = 5706507068631705000;
                }
            }
        }
    }
    match current_block {
        2945127981008348964 => {
            gssalloc_free(outbuf as *mut libc::c_void);
            (*token).value = 0 as *mut libc::c_void;
            (*token).length = 0usize;
            return err;
        }
        _ => {
            (*token).value = outbuf as *mut libc::c_void;
            (*token).length = bufsize;
            return 0i32;
        }
    };
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
/* message_buffer is an input if SIGN, output if SEAL, and ignored if DEL_CTX
conf_state is only valid if SEAL. */
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_unseal_token_v3(
    mut contextptr: *mut crate::krb5_h::krb5_context,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut ptr: *mut u8,
    mut bodysize: u32,
    mut message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut toktype: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut context = *contextptr;
    let mut plain = empty_data();
    let mut seqnum: crate::stdlib::uint64_t = 0;
    let mut ec: crate::stddef_h::size_t = 0;
    let mut rrc: crate::stddef_h::size_t = 0;
    let mut key_usage: i32 = 0;
    let mut acceptor_flag: u8 = 0;
    let mut sum = crate::krb5_h::krb5_checksum {
        magic: 0,
        checksum_type: 0,
        length: 0,
        contents: 0 as *mut crate::krb5_h::krb5_octet,
    };
    let mut err: crate::krb5_h::krb5_error_code = 0;
    let mut valid: crate::krb5_h::krb5_boolean = 0;
    let mut key = 0 as *mut crate::k5_int_h::krb5_key_st;
    let mut cksumtype: crate::krb5_h::krb5_cksumtype = 0;
    if !qop_state.is_null() {
        *qop_state = 0u32
    }
    acceptor_flag = if (*ctx).initiate() as i32 != 0 {
        0x1i32
    } else {
        0i32
    } as u8;
    key_usage = if toktype == 0x201i32 {
        if (*ctx).initiate() == 0 {
            24i32
        } else {
            22i32
        }
    } else if (*ctx).initiate() == 0 {
        25i32
    } else {
        23i32
    };
    /* Oops.  I wrote this code assuming ptr would be at the start of
    the token header.  */
    ptr = ptr.offset(-(2isize));
    bodysize = bodysize.wrapping_add(2u32);
    if !(bodysize < 16u32) {
        if *ptr.offset(2isize) as i32 & 0x1i32 != acceptor_flag as i32 {
            *minor_status = -(2045022963 as isize) as crate::gssapi_h::OM_uint32;
            return (6u32) << 16i32;
        }
        /* Two things to note here.

        First, we can't really enforce the use of the acceptor's subkey,
        if we're the acceptor; the initiator may have sent messages
        before getting the subkey.  We could probably enforce it if
        we're the initiator.

        Second, if someone tweaks the code to not set the flag telling
        the krb5 library to generate a new subkey in the AP-REP
        message, the MIT library may include a subkey anyways --
        namely, a copy of the AP-REQ subkey, if it was provided.  So
        the initiator may think we wanted a subkey, and set the flag,
        even though we weren't trying to set the subkey.  The "other"
        key, the one not asserted by the acceptor, will have the same
        value in that case, though, so we can just ignore the flag.  */
        if (*ctx).have_acceptor_subkey() as i32 != 0 && *ptr.offset(2isize) as i32 & 0x4i32 != 0 {
            key = (*ctx).acceptor_subkey;
            cksumtype = (*ctx).acceptor_subkey_cksumtype
        } else {
            key = (*ctx).subkey;
            cksumtype = (*ctx).cksumtype
        }
        if !key.is_null() {
        } else {
            crate::stdlib::__assert_fail(b"key != NULL\x00" as *const u8 as
                              *const i8,
                          b"k5sealv3.c\x00" as *const u8 as
                              *const i8,
                          369u32,
                          (*::std::mem::transmute::<&[u8; 160],
                                                    &[i8; 160]>(b"OM_uint32 gss_krb5int_unseal_token_v3(krb5_context *, OM_uint32 *, krb5_gss_ctx_id_rec *, unsigned char *, unsigned int, gss_buffer_t, int *, gss_qop_t *, int)\x00")).as_ptr());
        }
        if toktype == 0x201i32 {
            if load_16_be(ptr as *const libc::c_void) as i32 != 0x504i32 {
                current_block = 12585931968640856392;
            } else if *ptr.offset(3isize) as i32 != 0xffi32 {
                current_block = 12585931968640856392;
            } else {
                ec = load_16_be(ptr.offset(4isize) as *const libc::c_void)
                    as crate::stddef_h::size_t;
                rrc = load_16_be(ptr.offset(6isize) as *const libc::c_void)
                    as crate::stddef_h::size_t;
                seqnum = load_64_be(ptr.offset(8isize) as *const libc::c_void);
                if gss_krb5int_rotate_left(
                    ptr.offset(16isize) as *mut libc::c_void,
                    bodysize.wrapping_sub(16u32) as crate::stddef_h::size_t,
                    rrc,
                ) == 0
                {
                    current_block = 1062717902307394780;
                } else {
                    if *ptr.offset(2isize) as i32 & 0x2i32 != 0 {
                        /* confidentiality */
                        let mut cipher = crate::krb5_h::krb5_enc_data {
                            magic: 0,
                            enctype: 0,
                            kvno: 0,
                            ciphertext: crate::krb5_h::krb5_data {
                                magic: 0,
                                length: 0,
                                data: 0 as *mut i8,
                            },
                        };
                        let mut althdr = 0 as *mut u8;
                        if !conf_state.is_null() {
                            *conf_state = 1i32
                        }
                        /* Do we have no decrypt_size function?

                        For all current cryptosystems, the ciphertext size will
                        be larger than the plaintext size.  */
                        cipher.enctype = (*key).keyblock.enctype;
                        cipher.ciphertext.length = bodysize.wrapping_sub(16u32);
                        cipher.ciphertext.data = (ptr as *mut i8).offset(16isize);
                        plain.length = bodysize.wrapping_sub(16u32);
                        plain.data =
                            gssalloc_malloc(plain.length as crate::stddef_h::size_t) as *mut i8;
                        if plain.data.is_null() {
                            current_block = 1062717902307394780;
                        } else {
                            err = crate::krb5_h::krb5_k_decrypt(
                                context,
                                key,
                                key_usage,
                                0 as *const crate::krb5_h::krb5_data,
                                &mut cipher,
                                &mut plain,
                            );
                            if err != 0 {
                                gssalloc_free(plain.data as *mut libc::c_void);
                                current_block = 41667159751421923;
                            } else {
                                /* Don't use bodysize here!  Use the fact that
                                cipher.ciphertext.length has been adjusted to the
                                correct length.  */
                                althdr = (plain.data as *mut u8)
                                    .offset(plain.length as isize)
                                    .offset(-(16isize));
                                if load_16_be(althdr as *const libc::c_void) as i32 != 0x504i32
                                    || *althdr.offset(2isize) as i32 != *ptr.offset(2isize) as i32
                                    || *althdr.offset(3isize) as i32 != *ptr.offset(3isize) as i32
                                    || crate::stdlib::memcmp(
                                        althdr.offset(8isize) as *const libc::c_void,
                                        ptr.offset(8isize) as *const libc::c_void,
                                        8usize,
                                    ) != 0
                                {
                                    crate::stdlib::free(plain.data as *mut libc::c_void);
                                    current_block = 12585931968640856392;
                                } else {
                                    (*message_buffer).value = plain.data as *mut libc::c_void;
                                    (*message_buffer).length = (plain.length as usize)
                                        .wrapping_sub(ec)
                                        .wrapping_sub(16usize);
                                    if (*message_buffer).length == 0usize {
                                        gssalloc_free((*message_buffer).value);
                                        (*message_buffer).value = 0 as *mut libc::c_void
                                    }
                                    current_block = 17019156190352891614;
                                }
                            }
                        }
                    } else {
                        let mut cksumsize: crate::stddef_h::size_t = 0;
                        err = crate::krb5_h::krb5_c_checksum_length(
                            context,
                            cksumtype,
                            &mut cksumsize,
                        );
                        if err != 0 {
                            current_block = 41667159751421923;
                        } else {
                            /* no confidentiality */
                            if !conf_state.is_null() {
                                *conf_state = 0i32
                            }
                            if ec.wrapping_add(16usize) < ec {
                                current_block = 12585931968640856392;
                            } else if ec.wrapping_add(16usize) > bodysize as usize {
                                current_block = 12585931968640856392;
                            } else {
                                /* We have: header | msg | cksum.
                                We need cksum(msg | header).
                                Rotate the first two.  */
                                store_16_be(0u32, ptr.offset(4isize) as *mut libc::c_void);
                                store_16_be(0u32, ptr.offset(6isize) as *mut libc::c_void);
                                plain = make_data(
                                    ptr as *mut libc::c_void,
                                    (bodysize as usize).wrapping_sub(ec) as u32,
                                );
                                if gss_krb5int_rotate_left(
                                    ptr as *mut libc::c_void,
                                    (bodysize as usize).wrapping_sub(ec),
                                    16usize,
                                ) == 0
                                {
                                    current_block = 1062717902307394780;
                                } else {
                                    sum.length = ec as u32;
                                    if sum.length as usize != cksumsize {
                                        *minor_status = 0u32;
                                        return (6u32) << 16i32;
                                    }
                                    sum.contents =
                                        ptr.offset(bodysize as isize).offset(-(ec as isize));
                                    sum.checksum_type = cksumtype;
                                    err = crate::krb5_h::krb5_k_verify_checksum(
                                        context, key, key_usage, &mut plain, &mut sum, &mut valid,
                                    );
                                    if err != 0 {
                                        current_block = 41667159751421923;
                                    } else {
                                        if valid == 0 {
                                            *minor_status = 0u32;
                                            return (6u32) << 16i32;
                                        }
                                        (*message_buffer).length = plain.length.wrapping_sub(16u32)
                                            as crate::stddef_h::size_t;
                                        (*message_buffer).value =
                                            gssalloc_malloc((*message_buffer).length);
                                        if (*message_buffer).value.is_null() {
                                            current_block = 1062717902307394780;
                                        } else {
                                            crate::stdlib::memcpy(
                                                (*message_buffer).value,
                                                plain.data as *const libc::c_void,
                                                (*message_buffer).length,
                                            );
                                            current_block = 17019156190352891614;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        1062717902307394780 => {}
                        41667159751421923 => {}
                        12585931968640856392 => {}
                        _ => {
                            err = crate::src::generic::util_seqstate::gssint_g_seqstate_check(
                                (*ctx).seqstate,
                                seqnum,
                            ) as crate::krb5_h::krb5_error_code;
                            *minor_status = 0u32;
                            return err as crate::gssapi_h::OM_uint32;
                        }
                    }
                }
            }
        } else {
            if toktype == 0x101i32 {
                /* wrap token, no confidentiality */
                if load_16_be(ptr as *const libc::c_void) as i32 != 0x404i32 {
                    current_block = 12585931968640856392;
                } else {
                    current_block = 11237477937609663097;
                }
            } else if toktype == 0x102i32 {
                if load_16_be(ptr as *const libc::c_void) as i32 != 0x405i32 {
                    current_block = 12585931968640856392;
                } else {
                    message_buffer = &empty_message as *const crate::gssapi_h::gss_buffer_desc
                        as crate::gssapi_h::gss_buffer_t;
                    current_block = 11237477937609663097;
                }
            } else {
                current_block = 12585931968640856392;
            }
            match current_block {
                12585931968640856392 => {}
                _ => {
                    if *ptr.offset(3isize) as i32 != 0xffi32 {
                        current_block = 12585931968640856392;
                    } else if load_32_be(ptr.offset(4isize) as *const libc::c_void) as isize
                        != 0xffffffff as isize
                    {
                        current_block = 12585931968640856392;
                    } else {
                        seqnum = load_64_be(ptr.offset(8isize) as *const libc::c_void);
                        plain.length = (*message_buffer).length.wrapping_add(16usize) as u32;
                        plain.data = crate::stdlib::malloc(plain.length as usize) as *mut i8;
                        if plain.data.is_null() {
                            current_block = 1062717902307394780;
                        } else {
                            if (*message_buffer).length != 0 {
                                crate::stdlib::memcpy(
                                    plain.data as *mut libc::c_void,
                                    (*message_buffer).value,
                                    (*message_buffer).length,
                                );
                            }
                            crate::stdlib::memcpy(
                                plain.data.offset((*message_buffer).length as isize)
                                    as *mut libc::c_void,
                                ptr as *const libc::c_void,
                                16usize,
                            );
                            sum.length = bodysize.wrapping_sub(16u32);
                            sum.contents = ptr.offset(16isize);
                            sum.checksum_type = cksumtype;
                            err = crate::krb5_h::krb5_k_verify_checksum(
                                context, key, key_usage, &mut plain, &mut sum, &mut valid,
                            );
                            crate::stdlib::free(plain.data as *mut libc::c_void);
                            plain.data = 0 as *mut i8;
                            if err != 0 {
                                current_block = 41667159751421923;
                            } else {
                                if valid == 0 {
                                    *minor_status = 0u32;
                                    return (6u32) << 16i32;
                                }
                                err = crate::src::generic::util_seqstate::gssint_g_seqstate_check(
                                    (*ctx).seqstate,
                                    seqnum,
                                )
                                    as crate::krb5_h::krb5_error_code;
                                *minor_status = 0u32;
                                return err as crate::gssapi_h::OM_uint32;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            12585931968640856392 => {}
            _ => {
                match current_block {
                    41667159751421923 => {
                        *minor_status = err as crate::gssapi_h::OM_uint32;
                        crate::src::krb5::disp_status::krb5_gss_save_error_info(
                            *minor_status,
                            context,
                        );
                        return (6u32) << 16i32;
                        /* XXX */
                    }
                    _ => {
                        *minor_status = 12u32;
                        return (13u32) << 16i32;
                    }
                }
            }
        }
    }
    /* overflow check */
    *minor_status = 0u32;
    return (9u32) << 16i32;
}
